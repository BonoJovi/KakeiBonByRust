/**
 * Transaction list / header screen (res/js/transaction-management.js) —
 * regression tests for latent-audit M7 (disabled accounts).
 *
 * M7  An account that is still used by transactions can be disabled instead
 *     of deleted. The account dropdowns only offer enabled accounts, so
 *     editing a transaction that names a disabled account found no matching
 *     option and saving lost the account. Pinned: the disabled account is
 *     shown (with the disabled label) and kept on save, while a new
 *     transaction is not offered it.
 *
 * The real page module is booted against res/transaction-management.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const account = (code, name, isDisabled) => ({ account_code: code, account_name: name, is_disabled: isDisabled });
const NONE_ACCOUNT = account('NONE', '指定なし', 0);
const ACTIVE_ACCOUNT = account('BANK', 'Main Bank', 0);
const DISABLED_ACCOUNT = account('OLDCARD', 'Old Card', 1);

const HEADER = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:00:00',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: DISABLED_ACCOUNT.account_code,
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
                return args && args.includeDisabled
                    ? [NONE_ACCOUNT, ACTIVE_ACCOUNT, DISABLED_ACCOUNT]
                    : [NONE_ACCOUNT, ACTIVE_ACCOUNT];
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

window.confirm = () => true;
window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../../res/js/transaction-management.js');
await bootPage();

const fromAccountOptions = () =>
    Array.from(document.getElementById('from-account').options).map((o) => o.value);

function editButton() {
    return Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .find((b) => b.getAttribute('data-i18n') === 'common.edit');
}

async function closeModal() {
    document.getElementById('cancel-btn')?.click();
    await flush(5);
}

describe('transaction management screen — disabled account (regression, latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('should keep a disabled account selected when editing a transaction that names it (M7)', async () => {
        editButton().click();
        await flush(10);

        const fromSelect = document.getElementById('from-account');
        expect(fromSelect.value).toBe('OLDCARD');
        expect(fromSelect.selectedOptions[0].textContent).toBe('Old Card common.disabled_label');

        document.getElementById('transaction-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        const headerUpdates = callsOf(invoke, 'update_transaction_header');
        expect(headerUpdates).toHaveLength(1);
        expect(headerUpdates[0].fromAccountCode).toBe('OLDCARD');
    });

    test('should not offer a disabled account when the transaction is new (M7)', async () => {
        document.getElementById('add-transaction-btn').click();
        await flush(10);

        expect(fromAccountOptions()).toEqual(['NONE', 'BANK']);

        await closeModal();
    });
});
