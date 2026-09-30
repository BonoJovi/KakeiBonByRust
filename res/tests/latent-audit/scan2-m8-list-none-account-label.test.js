// latent-audit scan2-M8: the transaction list shows the stored NONE account name ("指定なし") instead of the i18n "unspecified" label
/**
 * Transaction list screen (res/js/transaction-management.js).
 *
 * scan2-M8  `TRANSACTION_LIST_BASE` returns ACCOUNT_NAME for the NONE account,
 *           which is stored as '指定なし' (or the JA template name). The row
 *           renderer prints `from_account_name` / `to_account_name` as is, so
 *           the English UI shows "Bank → 指定なし". The transaction form labels
 *           NONE with `common.unspecified`.
 *
 * Expected: a NONE side is rendered with i18n `common.unspecified`, never the
 * stored account name.
 *
 * Scope: this pins the JS half of M8 only (fix direction: map
 * `account_code === 'NONE'` in the JS). The category1 half (base
 * CATEGORY1_NAME '支出' without a CATEGORY1_I18N join) is a backend query issue
 * that `TransactionService::get_transactions` cannot express yet: it takes
 * no language, so a Rust test for it needs the fixed signature.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage,
} from '../pages/_page-harness.js';

const TRANSACTION = {
    transaction_id: 1,
    user_id: 2,
    shop_id: null,
    transaction_date: '2026-09-01 10:00:00',
    category1_code: 'EXPENSE',
    from_account_code: 'BANK',
    to_account_code: 'NONE',
    total_amount: 5000,
    tax_rounding_type: 0,
    memo_id: null,
    is_scheduled: 0,
    category1_name: 'Expense',
    from_account_name: 'Main Bank',
    to_account_name: '指定なし', // ACCOUNTS.ACCOUNT_NAME of the NONE row
    memo_text: null,
};

mockPageModules(jest, {
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
                    transactions: [TRANSACTION],
                    total_count: 1,
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
await import('../../js/transaction-management.js');
await bootPage();

describe('scan2-M8 transaction list NONE account label', () => {
    test('renders the NONE side with common.unspecified, not the stored name', () => {
        // The list header row uses the same class; keep the data rows only.
        const cells = Array.from(document.querySelectorAll('.transaction-account'))
            .filter((el) => !el.querySelector('[data-i18n]'));
        // Precondition: the row was rendered.
        expect(cells).toHaveLength(1);
        const text = cells[0].textContent;
        expect(text).toContain('Main Bank');
        expect(text).not.toContain('指定なし');
        expect(text).toContain('common.unspecified');
    });
});
