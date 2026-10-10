/**
 * Transaction list (res/js/transaction-management.js) — transactions saved
 * without the account their category needs.
 *
 * An expense needs a From account, an income a To account and a transfer
 * both. Saving one with that side "Unspecified" (NONE) is refused now, but
 * rows saved before that are still in the database: they count as an
 * expense or income while no account balance moves. The list marks the
 * missing side with `transaction_mgmt.account_missing_label` and shows
 * `transaction_mgmt.account_missing_hint` under the accounts, so the user
 * can find these rows and fix them with Edit. The unused side keeps the
 * plain `common.unspecified` label.
 *
 * The real page module is booted against res/transaction-management.html
 * via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage,
} from './_page-harness.js';

const row = (id, category1, from, to) => ({
    transaction_id: id,
    user_id: 2,
    shop_id: null,
    transaction_date: `2026-09-0${id} 10:00:00`,
    category1_code: category1,
    from_account_code: from,
    to_account_code: to,
    total_amount: 1000,
    tax_rounding_type: 0,
    memo_id: null,
    is_scheduled: 0,
    category1_name: category1,
    from_account_name: from === 'NONE' ? '指定なし' : from,
    to_account_name: to === 'NONE' ? '指定なし' : to,
    memo_text: null,
});

const TRANSACTIONS = [
    row(1, 'EXPENSE', 'NONE', 'NONE'),
    row(2, 'EXPENSE', 'CASH', 'NONE'),
    row(3, 'INCOME', 'NONE', 'NONE'),
    row(4, 'INCOME', 'NONE', 'BANK'),
    row(5, 'TRANSFER', 'NONE', 'BANK'),
    row(6, 'TRANSFER', 'CASH', 'NONE'),
];

mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [];
            case 'get_transactions':
                return {
                    transactions: TRANSACTIONS,
                    total_count: TRANSACTIONS.length,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [];
            case 'get_shops':
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

const items = () => Array.from(document.querySelectorAll('#transaction-list .transaction-item'));

// The two account labels of row `index` and its hint ('' when there is none).
function accountsOf(index) {
    const cell = items()[index].querySelector('.transaction-account');
    const [from, to] = Array.from(cell.querySelectorAll('.account-side')).map((el) => ({
        text: el.textContent,
        missing: el.classList.contains('account-missing'),
    }));
    const hint = cell.querySelector('.account-missing-hint');
    return { from, to, hint: hint ? hint.textContent : '' };
}

const MISSING = '⚠ transaction_mgmt.account_missing_label';
const HINT = 'transaction_mgmt.account_missing_hint';

describe('transaction list — transactions without the account their category needs', () => {
    test('should render every transaction when the list is loaded', () => {
        expect(items()).toHaveLength(TRANSACTIONS.length);
    });

    test('should mark the From account and show the hint when an expense has no From account', () => {
        expect(accountsOf(0)).toEqual({
            from: { text: MISSING, missing: true },
            to: { text: 'common.unspecified', missing: false },
            hint: HINT,
        });
    });

    test('should mark nothing when an expense has a From account', () => {
        expect(accountsOf(1)).toEqual({
            from: { text: 'CASH', missing: false },
            to: { text: 'common.unspecified', missing: false },
            hint: '',
        });
    });

    test('should mark the To account and show the hint when an income has no To account', () => {
        expect(accountsOf(2)).toEqual({
            from: { text: 'common.unspecified', missing: false },
            to: { text: MISSING, missing: true },
            hint: HINT,
        });
    });

    test('should mark nothing when an income has a To account', () => {
        expect(accountsOf(3)).toEqual({
            from: { text: 'common.unspecified', missing: false },
            to: { text: 'BANK', missing: false },
            hint: '',
        });
    });

    test('should mark the From account when a transfer has no From account', () => {
        expect(accountsOf(4)).toEqual({
            from: { text: MISSING, missing: true },
            to: { text: 'BANK', missing: false },
            hint: HINT,
        });
    });

    test('should mark the To account when a transfer has no To account', () => {
        expect(accountsOf(5)).toEqual({
            from: { text: 'CASH', missing: false },
            to: { text: MISSING, missing: true },
            hint: HINT,
        });
    });
});
