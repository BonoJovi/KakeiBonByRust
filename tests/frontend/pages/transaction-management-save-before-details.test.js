/**
 * T6  The Manage-details button navigates to the detail screen at once. It
 *     neither saves the header nor stores a draft (unlike Manage shops), so
 *     edited date / total / memo / scheduled are silently dropped.
 *     Expected (owner decision 2026-10-07): with unsaved changes the screen
 *     asks in an in-app dialog (#save-before-details-modal; native confirm()
 *     breaks the flow under Tauri + WebKitGTK), then saves the header through
 *     the normal save and moves on; "cancel" (or Esc) closes only the dialog
 *     and keeps the edit modal open. Without changes it moves on at once.
 *
 * Real page module booted against res/transaction-management.html.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf, deferred,
} from './_page-harness.js';

// When set, update_transaction_header answers through this promise.
let updateResult = null;

const HEADER = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:00:00',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    to_account_code: 'NONE',
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: 'old memo',
    is_scheduled: 0,
};

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [
                    { category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' }, children: [] },
                ];
            case 'get_transactions':
                return {
                    transactions: [{ ...HEADER, category1_name: 'Expense' }],
                    total_count: 1,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [{ account_code: 'CASH', account_name: 'Cash', is_disabled: 0 }];
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                return HEADER;
            case 'get_transaction_details':
                return [];
            case 'update_transaction_header':
                return updateResult;
            default:
                return null;
        }
    },
});

// The flow must not use a native dialog.
const confirmSpy = jest.fn(() => true);
window.confirm = confirmSpy;
window.alert = () => {};

// jsdom cannot navigate; it reports each location.href assignment as a
// "Not implemented: navigation" error, which tells us the page tried to move.
const consoleError = jest.spyOn(console, 'error').mockImplementation(() => {});
const navigated = () => consoleError.mock.calls.some((args) =>
    args.some((a) => String(a?.message ?? a).includes('navigation')));

loadPageBody('transaction-management.html');
await import('../../../res/js/transaction-management.js');
await bootPage();

const isOpen = (id) => !document.getElementById(id).classList.contains('hidden');
const dialogOpen = () => isOpen('save-before-details-modal');
const editModalOpen = () => isOpen('transaction-modal');

async function openEdit() {
    document.getElementById('cancel-transaction-btn')?.click();
    await flush();
    const editBtn = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .find((b) => b.getAttribute('data-i18n') === 'common.edit');
    editBtn.click();
    await flush(10);
    expect(document.getElementById('total-amount').value).toBe('5000');
    invoke.mockClear();
    confirmSpy.mockClear();
    consoleError.mockClear();
}

async function clickManageDetails() {
    document.getElementById('manage-details-btn').click();
    await flush(5);
}

describe('Manage details from the header edit modal (scan2-T6)', () => {
    afterEach(() => {
        expect(confirmSpy).not.toHaveBeenCalled();
    });

    test('should save the edited header values when leaving for the details (T6)', async () => {
        await openEdit();

        document.getElementById('transaction-date').value = '2026-09-20T18:00';
        document.getElementById('total-amount').value = '6400';
        document.getElementById('transaction-memo').value = 'new memo';
        document.getElementById('is-scheduled').checked = true;

        await clickManageDetails();
        expect(dialogOpen()).toBe(true);
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        expect(navigated()).toBe(false);

        document.getElementById('confirm-save-before-details').click();
        await flush(10);

        const savedHeader = callsOf(invoke, 'update_transaction_header').some((u) => u.totalAmount === 6400
            && u.transactionDate === '2026-09-20 18:00:00'
            && u.memo === 'new memo'
            && u.isScheduled === 1);
        expect(savedHeader).toBe(true);
        expect(dialogOpen()).toBe(false);
        expect(navigated()).toBe(true);
    });

    test('should not save twice when the button is clicked again while saving (T6)', async () => {
        await openEdit();
        document.getElementById('total-amount').value = '6400';

        const saved = deferred();
        updateResult = saved.promise;
        try {
            await clickManageDetails();
            document.getElementById('confirm-save-before-details').click();
            await flush(5);
            document.getElementById('confirm-save-before-details').click();
            await flush(5);
            expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(1);
            expect(navigated()).toBe(false); // not before the save has finished

            saved.resolve(null);
            await flush(10);
            expect(navigated()).toBe(true);
        } finally {
            updateResult = null;
        }
    });

    test('should stay in the edit window without saving when the dialog is cancelled (T6)', async () => {
        await openEdit();
        document.getElementById('total-amount').value = '6400';

        await clickManageDetails();
        document.getElementById('cancel-save-before-details').click();
        await flush(5);

        expect(dialogOpen()).toBe(false);
        expect(editModalOpen()).toBe(true);
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        expect(navigated()).toBe(false);
        expect(document.getElementById('total-amount').value).toBe('6400');
    });

    test('should close only the dialog, not the edit window behind it, when Esc is pressed (T6)', async () => {
        await openEdit();
        document.getElementById('total-amount').value = '6400';

        await clickManageDetails();
        document.activeElement.dispatchEvent(
            new KeyboardEvent('keydown', { key: 'Escape', bubbles: true })
        );
        await flush(5);

        expect(dialogOpen()).toBe(false);
        expect(editModalOpen()).toBe(true);
        expect(document.getElementById('total-amount').value).toBe('6400');
        expect(navigated()).toBe(false);
    });

    test('should move on at once, without asking or saving, when nothing was changed (T6)', async () => {
        await openEdit();

        await clickManageDetails();

        expect(dialogOpen()).toBe(false);
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        expect(navigated()).toBe(true);
    });
});
