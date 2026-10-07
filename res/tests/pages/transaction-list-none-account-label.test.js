// The transaction list shows the NONE account with the localized label (latent-audit scan2-M8)
/**
 * Transaction list screen (res/js/transaction-management.js).
 *
 * scan2-M8  `TRANSACTION_LIST_BASE` returns ACCOUNT_NAME for the NONE account,
 *           which is stored as '指定なし' (or the JA template name). The row
 *           renderer printed `from_account_name` / `to_account_name` as is,
 *           so the English UI showed "Main Bank → 指定なし". The transaction
 *           form labels NONE with `common.unspecified`.
 *
 * Expected: a NONE side is rendered with i18n `common.unspecified`, never the
 * stored account name. The category1 half of M8 (CATEGORY1_I18N join) is
 * covered by the Rust test
 * latent_scan2_m8_transaction_list_category1_follows_language.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage,
} from './_page-harness.js';

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

describe('transaction list NONE account label (scan2-M8)', () => {
    test('renders the NONE side with common.unspecified, not the stored name', () => {
        // The list header row uses the same class; take the data rows only.
        const cells = Array.from(
            document.querySelectorAll('#transaction-list .transaction-item .transaction-account')
        );
        // Precondition: the row was rendered.
        expect(cells).toHaveLength(1);
        const text = cells[0].textContent;
        expect(text).toContain('Main Bank');
        expect(text).not.toContain('指定なし');
        expect(text).toContain('common.unspecified');
    });
});
