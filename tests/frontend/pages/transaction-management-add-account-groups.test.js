/**
 * Transaction window (res/transaction-management.html, handled by
 * res/js/transaction-management.js) — the From/To account fields when the
 * window is opened to add a transaction.
 *
 * Opening a saved header hides the account field its category does not use
 * (an income has no From account, an expense no To account). The add window
 * starts with no category, so it shows both fields, as on the first open,
 * also when an edit was opened and closed before.
 *
 * The real page module is booted against res/transaction-management.html
 * via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush,
} from './_page-harness.js';

const HEADERS = {
    1: {
        transaction_id: 1,
        transaction_date: '2026-09-01 10:30:00',
        shop_id: null,
        category1_code: 'INCOME',
        from_account_code: 'NONE',
        to_account_code: 'CASH',
        total_amount: 5000,
        tax_rounding_type: 0,
        tax_included_type: 1,
        memo: null,
        is_scheduled: 0,
    },
    2: {
        transaction_id: 2,
        transaction_date: '2026-09-02 10:30:00',
        shop_id: null,
        category1_code: 'EXPENSE',
        from_account_code: 'CASH',
        to_account_code: 'NONE',
        total_amount: 800,
        tax_rounding_type: 0,
        tax_included_type: 1,
        memo: null,
        is_scheduled: 0,
    },
};

mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [
                    { category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' }, children: [] },
                    { category1: { category1_code: 'INCOME', category1_name_i18n: 'Income' }, children: [] },
                ];
            case 'get_transactions':
                return {
                    transactions: [
                        { ...HEADERS[1], category1_name: 'Income' },
                        { ...HEADERS[2], category1_name: 'Expense' },
                    ],
                    total_count: 2,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [{ account_code: 'CASH', account_name: 'Cash', is_disabled: 0 }];
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                return HEADERS[args.transactionId];
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

// Open the listed header at `index` (0 = income, 1 = expense) for editing.
async function openEdit(index) {
    await closeWindow();
    const editBtns = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .filter((b) => b.getAttribute('data-i18n') === 'common.edit');
    editBtns[index].click();
    await flush(10);
}

async function openAdd() {
    await closeWindow();
    field('add-transaction-btn').click();
    await flush(10);
}

describe('transaction window — account fields when adding', () => {
    test('should show both account fields when the add window is opened first', async () => {
        await openAdd();
        expect(field('category1').value).toBe('');
        expect(shown('from-account-group')).toBe(true);
        expect(shown('to-account-group')).toBe(true);
    });

    test('should show both account fields when the add window is opened after editing an income', async () => {
        await openEdit(0);
        expect(shown('from-account-group')).toBe(false);
        await openAdd();
        expect(shown('from-account-group')).toBe(true);
        expect(shown('to-account-group')).toBe(true);
    });

    test('should show both account fields when the add window is opened after editing an expense', async () => {
        await openEdit(1);
        expect(shown('to-account-group')).toBe(false);
        await openAdd();
        expect(shown('from-account-group')).toBe(true);
        expect(shown('to-account-group')).toBe(true);
    });

    test('should hide the unused account field when an income is opened after the add window', async () => {
        await openAdd();
        await openEdit(0);
        expect(shown('from-account-group')).toBe(false);
        expect(shown('to-account-group')).toBe(true);
    });
});
