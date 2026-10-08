/**
 * Transaction list / header screen (res/js/transaction-management.js) —
 * regression test for latent-audit M2.
 *
 * M2  Changing a header's category1 (income / expense / transfer) left its
 *     details on the old category1, so income details ended up in the
 *     expense pie chart. The backend now refuses the change while the
 *     transaction has details (ApiError code `category1_has_details`).
 *     Pinned: the screen shows transaction_mgmt.category1_has_details and
 *     keeps the modal open, instead of a raw English error.
 *
 * The real page module is booted against res/transaction-management.html via
 * ./_page-harness.js.
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
            case 'update_transaction_header':
                return Promise.reject({
                    code: 'category1_has_details',
                    message: 'Category cannot be changed while the transaction has details',
                });
            default:
                return null;
        }
    },
});

window.confirm = () => true;
window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../js/transaction-management.js');
await bootPage();

describe('transaction management screen — category1 of a header with details (regression, latent audit 2026-09)', () => {
    test('should explain why the category cannot change and keep the window open when the transaction has details (M2)', async () => {
        const editBtn = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
            .find((b) => b.getAttribute('data-i18n') === 'common.edit');
        editBtn.click();
        await flush(10);

        const category1 = document.getElementById('category1');
        category1.value = 'INCOME';
        category1.dispatchEvent(new Event('change'));
        await flush(5);
        document.getElementById('transaction-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        expect(callsOf(invoke, 'update_transaction_header')).toHaveLength(1);
        expect(showToast).toHaveBeenCalledWith('transaction_mgmt.category1_has_details', { variant: 'error' });
        expect(document.getElementById('transaction-modal').classList.contains('hidden')).toBe(false);
    });
});
