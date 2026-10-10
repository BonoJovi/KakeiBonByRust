/**
 * Transaction edit window (res/transaction-management.html, handled by
 * res/js/transaction-management.js) — the From/To account fields that the
 * category decides, and the checks run when Save is clicked.
 *
 * Choosing a category shows the account fields it uses, by the category
 * name: an expense shows From, an income shows To, a transfer and any other
 * category show both. A field that gets hidden is set back to NONE; no
 * category hides both and sets both to NONE.
 *
 * Save is the form's submit button, so the browser refuses a form with no
 * date, no category or no total (all `required`). The page itself also
 * refuses a blank date, refuses a transfer between the same account, and
 * sends a total of 0.
 *
 * These replace part of a former test file that tested copies of these
 * rules written inside the test. The real page module is booted against
 * res/transaction-management.html via ./_page-harness.js.
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

const { invoke, showToast } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [
                    { category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' }, children: [] },
                    { category1: { category1_code: 'INCOME', category1_name_i18n: 'Income' }, children: [] },
                    { category1: { category1_code: 'TRANSFER', category1_name_i18n: 'Transfer' }, children: [] },
                    { category1: { category1_code: 'OTHER', category1_name_i18n: 'Other' }, children: [] },
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
const shown = (id) => field(id).style.display !== 'none';

async function closeWindow() {
    field('cancel-transaction-btn')?.click();
    await flush();
}

async function openEdit() {
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

// Click the window's Save button, as the user does.
async function clickSave() {
    field('transaction-form').querySelector('button[type="submit"]').click();
    await flush(10);
}

const sentUpdate = () => callsOf(invoke, 'update_transaction_header')[0];

describe('transaction edit window — account fields by category', () => {
    test('should show only the From account when the category is an expense', async () => {
        await openEdit();
        chooseCategory('EXPENSE');
        expect(shown('from-account-group')).toBe(true);
        expect(shown('to-account-group')).toBe(false);
    });

    test('should show only the To account when the category is an income', async () => {
        await openEdit();
        chooseCategory('INCOME');
        expect(shown('from-account-group')).toBe(false);
        expect(shown('to-account-group')).toBe(true);
    });

    test('should show both accounts when the category is a transfer', async () => {
        await openEdit();
        chooseCategory('TRANSFER');
        expect(shown('from-account-group')).toBe(true);
        expect(shown('to-account-group')).toBe(true);
    });

    test('should show both accounts when the category is not an expense, income or transfer', async () => {
        await openEdit();
        chooseCategory('INCOME');
        chooseCategory('OTHER');
        expect(shown('from-account-group')).toBe(true);
        expect(shown('to-account-group')).toBe(true);
    });

    test('should hide both accounts and set them to NONE when no category is chosen', async () => {
        await openEdit();
        chooseCategory('TRANSFER');
        field('from-account').value = 'CASH';
        field('to-account').value = 'BANK';
        chooseCategory('');
        expect(shown('from-account-group')).toBe(false);
        expect(shown('to-account-group')).toBe(false);
        expect(field('from-account').value).toBe('NONE');
        expect(field('to-account').value).toBe('NONE');
    });

    test('should set the To account to NONE when a transfer is changed to an expense', async () => {
        await openEdit();
        chooseCategory('TRANSFER');
        field('from-account').value = 'CASH';
        field('to-account').value = 'BANK';
        chooseCategory('EXPENSE');
        expect(field('from-account').value).toBe('CASH');
        expect(field('to-account').value).toBe('NONE');
    });

    test('should set the From account to NONE when a transfer is changed to an income', async () => {
        await openEdit();
        chooseCategory('TRANSFER');
        field('from-account').value = 'CASH';
        field('to-account').value = 'BANK';
        chooseCategory('INCOME');
        expect(field('from-account').value).toBe('NONE');
        expect(field('to-account').value).toBe('BANK');
    });

    test('should keep both accounts when an expense is changed to a transfer', async () => {
        await openEdit();
        field('from-account').value = 'CASH';
        chooseCategory('TRANSFER');
        field('to-account').value = 'BANK';
        await clickSave();
        expect(sentUpdate()).toMatchObject({
            category1Code: 'TRANSFER', fromAccountCode: 'CASH', toAccountCode: 'BANK',
        });
    });
});

describe('transaction edit window — checks on Save', () => {
    test('should send the header when every field is filled in', async () => {
        await openEdit();
        await clickSave();
        expect(sentUpdate()).toMatchObject({
            category1Code: 'EXPENSE', fromAccountCode: 'CASH', totalAmount: 5000,
        });
    });

    test('should not send the header when the date is blank', async () => {
        await openEdit();
        field('transaction-date').value = '';
        await clickSave();
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        expect(field('transaction-modal').classList.contains('hidden')).toBe(false);
    });

    test('should not send the header when no category is chosen', async () => {
        await openEdit();
        chooseCategory('');
        await clickSave();
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
    });

    test('should not send the header when the total is blank', async () => {
        await openEdit();
        field('total-amount').value = '';
        await clickSave();
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
    });

    test('should send a total of 0 when the total is 0', async () => {
        await openEdit();
        field('total-amount').value = '0';
        await clickSave();
        expect(sentUpdate().totalAmount).toBe(0);
    });

    test('should show the same-account message and not send when a transfer has the same account twice', async () => {
        await openEdit();
        chooseCategory('TRANSFER');
        field('from-account').value = 'CASH';
        field('to-account').value = 'CASH';
        await clickSave();
        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(0);
        expect(showToast).toHaveBeenCalledWith('transaction_mgmt.transfer_same_account', { variant: 'error' });
    });
});
