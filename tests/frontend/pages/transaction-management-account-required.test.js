/**
 * Transaction edit window (res/transaction-management.html, handled by
 * res/js/transaction-management.js) — the account a category needs.
 *
 * An expense needs a From account, an income a To account and a transfer
 * both. Saving with that side left "Unspecified" (NONE) used to be accepted:
 * the amount was counted as an expense or income, but the dashboard hides
 * the NONE account, so no account balance moved. Save now refuses it with a
 * message under the empty field and keeps the window open. The backend
 * refuses it too (`account_required`), shown as a toast.
 *
 * The real page module is booted against res/transaction-management.html
 * via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const SAVED = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:30:00',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    to_account_code: 'NONE',
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: null,
    is_scheduled: 0,
};

// The error update_transaction_header throws, or null to accept the save.
let rejectUpdate = null;

const { invoke, showToast } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [
                    { category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' }, children: [] },
                    { category1: { category1_code: 'INCOME', category1_name_i18n: 'Income' }, children: [] },
                    { category1: { category1_code: 'TRANSFER', category1_name_i18n: 'Transfer' }, children: [] },
                ];
            case 'get_transactions':
                return {
                    transactions: [{ ...SAVED, category1_name: 'Expense' }],
                    total_count: 1,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [
                    { account_code: 'CASH', account_name: 'Cash', is_disabled: 0 },
                    { account_code: 'BANK', account_name: 'Bank', is_disabled: 0 },
                ];
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                return SAVED;
            case 'get_transaction_details':
                return [];
            case 'update_transaction_header':
                if (rejectUpdate) throw rejectUpdate;
                return null;
            default:
                return null;
        }
    },
});

window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../../res/js/transaction-management.js');
await bootPage();

const field = (id) => document.getElementById(id);
const windowOpen = () => !field('transaction-modal').classList.contains('hidden');

// The inline message shown under `id`, or null when there is none.
function inlineError(id) {
    const next = field(id).nextElementSibling;
    return next && next.classList.contains('validation-error') ? next.textContent : null;
}

async function closeWindow() {
    field('cancel-transaction-btn')?.click();
    await flush();
}

async function openEdit() {
    rejectUpdate = null;
    await closeWindow();
    const editBtn = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .find((b) => b.getAttribute('data-i18n') === 'common.edit');
    editBtn.click();
    await flush(10);
    invoke.mockClear();
    showToast.mockClear();
}

function chooseCategory(code) {
    field('category1').value = code;
    field('category1').dispatchEvent(new Event('change'));
}

async function clickSave() {
    field('transaction-form').querySelector('button[type="submit"]').click();
    await flush(10);
}

const updates = () => callsOf(invoke, 'update_transaction_header');

describe('transaction edit window — the account a category needs', () => {
    test('should refuse an expense and mark the From account when it is unspecified', async () => {
        await openEdit();
        field('from-account').value = 'NONE';
        await clickSave();
        expect(updates()).toHaveLength(0);
        expect(windowOpen()).toBe(true);
        expect(inlineError('from-account')).toBe('transaction_mgmt.from_account_required');
        expect(inlineError('to-account')).toBeNull();
    });

    test('should refuse an income and mark the To account when it is unspecified', async () => {
        await openEdit();
        chooseCategory('INCOME');
        await clickSave();
        expect(updates()).toHaveLength(0);
        expect(windowOpen()).toBe(true);
        expect(inlineError('to-account')).toBe('transaction_mgmt.to_account_required');
        expect(inlineError('from-account')).toBeNull();
    });

    test('should refuse a transfer and mark the From account when only the To account is chosen', async () => {
        await openEdit();
        chooseCategory('TRANSFER');
        field('from-account').value = 'NONE';
        field('to-account').value = 'BANK';
        await clickSave();
        expect(updates()).toHaveLength(0);
        expect(inlineError('from-account')).toBe('transaction_mgmt.from_account_required');
        expect(inlineError('to-account')).toBeNull();
    });

    test('should refuse a transfer and mark the To account when only the From account is chosen', async () => {
        await openEdit();
        chooseCategory('TRANSFER');
        field('from-account').value = 'CASH';
        field('to-account').value = 'NONE';
        await clickSave();
        expect(updates()).toHaveLength(0);
        expect(inlineError('to-account')).toBe('transaction_mgmt.to_account_required');
        expect(inlineError('from-account')).toBeNull();
    });

    test('should send an income when its To account is chosen', async () => {
        await openEdit();
        chooseCategory('INCOME');
        field('to-account').value = 'BANK';
        await clickSave();
        expect(updates()[0]).toMatchObject({
            category1Code: 'INCOME', fromAccountCode: 'NONE', toAccountCode: 'BANK',
        });
    });

    test('should clear the account message when the save is tried again with the account chosen', async () => {
        await openEdit();
        field('from-account').value = 'NONE';
        await clickSave();
        expect(inlineError('from-account')).toBe('transaction_mgmt.from_account_required');
        field('from-account').value = 'CASH';
        await clickSave();
        expect(inlineError('from-account')).toBeNull();
        expect(updates()).toHaveLength(1);
    });

    test('should show the account-required message and keep the window open when the backend refuses with account_required', async () => {
        await openEdit();
        rejectUpdate = { code: 'account_required', message: 'An account is required for this category' };
        await clickSave();
        expect(showToast).toHaveBeenCalledWith('transaction_mgmt.account_required', { variant: 'error' });
        expect(windowOpen()).toBe(true);
    });
});
