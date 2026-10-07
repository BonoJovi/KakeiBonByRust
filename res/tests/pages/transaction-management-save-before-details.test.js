/**
 * T6  The Manage-details button navigates to the detail screen at once. It
 *     neither saves the header nor stores a draft (unlike Manage shops), so
 *     edited date / total / memo / scheduled are silently dropped.
 *     Expected (owner decision 2026-10-07): with unsaved changes the screen
 *     asks, then saves the header through the normal save and moves on;
 *     "cancel" stays in the modal. Without changes it moves on at once.
 *
 * Real page module booted against res/transaction-management.html.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const HEADER = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:00:00',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: 'NONE',
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
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                return HEADER;
            case 'get_transaction_details':
                return [];
            default:
                return null;
        }
    },
});

const session = await import('../../js/session.js');

// Answers to the "save before Manage details?" confirm (and the total
// recalculation prompt after a save, which these tests never trigger).
let confirmAnswer = true;
const confirmSpy = jest.fn(() => confirmAnswer);
window.confirm = confirmSpy;
window.alert = () => {};

// jsdom cannot navigate; it reports each location.href assignment as a
// "Not implemented: navigation" error, which tells us the page tried to move.
const consoleError = jest.spyOn(console, 'error').mockImplementation(() => {});
const navigated = () => consoleError.mock.calls.some((args) =>
    args.some((a) => String(a?.message ?? a).includes('navigation')));

loadPageBody('transaction-management.html');
await import('../../js/transaction-management.js');
await bootPage();

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

describe('Manage details from the header edit modal (scan2-T6)', () => {
    test('[T6] edited header values are saved before leaving', async () => {
        confirmAnswer = true;
        await openEdit();

        document.getElementById('transaction-date').value = '2026-09-20T18:00';
        document.getElementById('total-amount').value = '6400';
        document.getElementById('transaction-memo').value = 'new memo';
        document.getElementById('is-scheduled').checked = true;

        session.setSessionModalState.mockClear();
        document.getElementById('manage-details-btn').click();
        await flush(10);

        const updates = callsOf(invoke, 'update_transaction_header');
        const drafts = session.setSessionModalState.mock.calls.map(([json]) => JSON.parse(json));

        const savedHeader = updates.some((u) => u.totalAmount === 6400
            && u.transactionDate === '2026-09-20 18:00:00'
            && u.memo === 'new memo'
            && u.isScheduled === 1);
        const keptDraft = drafts.some((d) => String(d.total_amount) === '6400'
            && d.transaction_date === '2026-09-20T18:00'
            && d.memo === 'new memo'
            && (d.is_scheduled === true || d.is_scheduled === 1 || d.is_scheduled === '1'));

        // Either route keeps the edits; the fix saves them.
        expect(savedHeader || keptDraft).toBe(true);
        expect(confirmSpy).toHaveBeenCalledWith('transaction_mgmt.save_before_details_confirm');
        expect(navigated()).toBe(true);
    });

    test('[T6] cancelling the confirm stays in the modal without saving', async () => {
        confirmAnswer = false;
        await openEdit();
        document.getElementById('total-amount').value = '6400';

        document.getElementById('manage-details-btn').click();
        await flush(10);

        expect(confirmSpy).toHaveBeenCalledTimes(1);
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        expect(navigated()).toBe(false);
        expect(document.getElementById('total-amount').value).toBe('6400');
    });

    test('[T6] without changes it moves on at once, without asking or saving', async () => {
        confirmAnswer = true;
        await openEdit();

        document.getElementById('manage-details-btn').click();
        await flush(10);

        expect(confirmSpy).not.toHaveBeenCalled();
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        expect(navigated()).toBe(true);
    });
});
