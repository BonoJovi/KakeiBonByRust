// The detail screen header shows the NONE account with the localized label (latent-audit scan2-M8)
/**
 * Transaction detail screen (res/js/transaction-detail-management.js).
 *
 * scan2-M8  The header block printed `from_account_name` / `to_account_name`
 *           as returned by get_transaction_header_with_info. The NONE
 *           account is stored as '指定なし', so the English UI showed
 *           "指定なし" as the account of an expense paid from an
 *           unspecified account. The transaction form labels NONE with
 *           `common.unspecified`. Found next to the transaction list half
 *           of M8 (pages/transaction-list-none-account-label.test.js).
 *
 * Expected: a NONE account in the header is rendered with i18n
 * `common.unspecified`, never the stored account name.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage,
} from './_page-harness.js';

const TRANSACTION_ID = 10;

const HEADER = {
    transaction_id: TRANSACTION_ID,
    transaction_date: '2026-09-01 10:00:00',
    category1_code: 'EXPENSE',
    from_account_code: 'NONE',
    from_account_name: '指定なし', // ACCOUNTS.ACCOUNT_NAME of the NONE row
    to_account_code: 'NONE',
    to_account_name: '指定なし',
    shop_name: 'Shop',
    total_amount: 0,
    tax_rounding_type: 0,
};

mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_transaction_header_with_info':
                return HEADER;
            case 'get_transaction_details':
                return [];
            case 'get_category_tree_with_lang':
                return [];
            case 'compute_recommended_transaction_total':
                return HEADER.total_amount; // equal -> no recalc prompt
            default:
                return null;
        }
    },
});

window.confirm = () => false;
window.history.replaceState(null, '', `/?transaction_id=${TRANSACTION_ID}`);
loadPageBody('transaction-detail-management.html');
await import('../../js/transaction-detail-management.js');
await bootPage();

describe('transaction detail header NONE account label (scan2-M8)', () => {
    test('renders a NONE account with common.unspecified, not the stored name', () => {
        const text = document.getElementById('header-account').textContent;
        expect(text).not.toContain('指定なし');
        expect(text).toBe('common.unspecified');
    });
});
