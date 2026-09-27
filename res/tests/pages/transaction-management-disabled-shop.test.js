/**
 * Transaction list / header screen (res/js/transaction-management.js) —
 * regression tests for latent-audit M7 (disabled shops).
 *
 * M7  A shop that is still used by transactions can be disabled instead of
 *     deleted. The shop dropdown only offers enabled shops, so editing a
 *     transaction that names a disabled shop found no matching option: the
 *     select fell back to "Unspecified" and saving silently dropped the
 *     shop. Pinned: the disabled shop is shown (with the disabled label) and
 *     kept on save, while a new transaction is not offered it.
 *
 * The real page module is booted against res/transaction-management.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const ACTIVE_SHOP = { shop_id: 1, shop_name: 'New Mart', is_disabled: 0 };
const DISABLED_SHOP = { shop_id: 7, shop_name: 'Old Mart', is_disabled: 1 };

const HEADER = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:00:00',
    shop_id: DISABLED_SHOP.shop_id,
    category1_code: 'EXPENSE',
    from_account_code: 'NONE',
    to_account_code: 'NONE',
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: null,
    is_scheduled: 0,
};

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [{
                    category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
                    children: [],
                }];
            case 'get_transactions':
                return {
                    transactions: [{ ...HEADER, category1_name: 'Expense' }],
                    total_count: 1,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [];
            case 'get_shops':
                return args && args.includeDisabled ? [ACTIVE_SHOP, DISABLED_SHOP] : [ACTIVE_SHOP];
            case 'get_transaction_header':
                return HEADER;
            case 'get_transaction_details':
                return [];
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

const shopOptions = () => Array.from(document.getElementById('shop').options).map((o) => o.value);

function editButton() {
    return Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .find((b) => b.getAttribute('data-i18n') === 'common.edit');
}

async function closeModal() {
    document.getElementById('cancel-btn')?.click();
    await flush(5);
}

describe('transaction management screen — disabled shop (regression, latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('[M7] should keep a disabled shop selected when editing a transaction that names it', async () => {
        editButton().click();
        await flush(10);

        const shopSelect = document.getElementById('shop');
        expect(shopSelect.value).toBe('7');
        expect(shopSelect.selectedOptions[0].textContent).toBe('Old Mart common.disabled_label');

        document.getElementById('transaction-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        const headerUpdates = callsOf(invoke, 'update_transaction_header');
        expect(headerUpdates).toHaveLength(1);
        expect(headerUpdates[0].shopId).toBe(7);
    });

    test('[M7] should not offer a disabled shop for a new transaction', async () => {
        document.getElementById('add-transaction-btn').click();
        await flush(10);

        expect(shopOptions()).toEqual(['', '1']);

        await closeModal();
    });
});
