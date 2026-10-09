// Typing a tax-included amount digit by digit keeps every digit and saves the typed price (latent-audit scan2-T1)
/**
 * T1  detail-tax-calc.js calcFromIncluding runs on every `input` event and
 *     writes `includedCorrected` back into the field being typed. Under the
 *     default header rounding (floor) at 10 %, the prefix "10" cannot be
 *     produced, so it is rewritten to 9 and typing "100" ends as 90.
 *     100 itself is a valid tax-included price (91 + floor(9.1) = 100).
 *     Expected: the field keeps what the user typed ("100"), the derived
 *     fields are 91 / 9, and the saved detail carries 100 / 91 / 9.
 *
 * Real page module (transaction-detail-management.js + detail-tax-calc.js)
 * booted against res/transaction-detail-management.html.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const TRANSACTION_ID = 10;

const HEADER = {
    transaction_id: TRANSACTION_ID,
    transaction_date: '2026-09-01 10:00:00',
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    from_account_name: 'Cash',
    shop_name: 'Shop',
    total_amount: 0,
    tax_rounding_type: 0, // floor (the default)
};

const CATEGORY_TREE = [
    {
        category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
        children: [
            {
                category2: { category2_code: 'C2_E_1', category2_name_i18n: 'Food' },
                children: [{ category3_code: 'C3_1', category3_name_i18n: 'Veg' }],
            },
        ],
    },
];

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_transaction_header_with_info':
                return HEADER;
            case 'get_transaction_details':
                return [];
            case 'get_category_tree_with_lang':
            case 'get_category_tree_all_with_lang':
                return CATEGORY_TREE;
            case 'search_products_by_name':
                return [];
            case 'compute_recommended_transaction_total':
                return null;
            default:
                return null;
        }
    },
});

window.confirm = () => false;
window.history.replaceState(null, '', `/?transaction_id=${TRANSACTION_ID}`);
loadPageBody('transaction-detail-management.html');
await import('../../../res/js/transaction-detail-management.js');
await bootPage();

/** Type `text` into `input` one character at a time, like a keyboard does. */
function typeInto(input, text) {
    for (const ch of text) {
        input.value = String(input.value) + ch;
        input.dispatchEvent(new Event('input', { bubbles: true }));
    }
}

describe('latent-audit scan2 T1 — tax-included amount typed digit by digit', () => {
    test('should keep 100 and save 100 / 91 / 9 when "100" is typed tax-included at 10 % (floor) (T1)', async () => {
        document.getElementById('add-detail-btn').click();
        await flush(10);
        expect(document.getElementById('detail-modal').classList.contains('hidden')).toBe(false);

        document.getElementById('item-name').value = 'Milk';
        document.getElementById('tax-rate').value = '10';

        const included = document.getElementById('amount-including-tax');
        typeInto(included, '100');

        expect(included.value).toBe('100');
        expect(document.getElementById('amount-excluding-tax').value).toBe('91');
        expect(String(document.getElementById('tax-amount').value)).toBe('9');

        // Leaving the field (change / blur) must not alter a reachable value either.
        included.dispatchEvent(new Event('change', { bubbles: true }));
        included.dispatchEvent(new Event('blur'));

        document.getElementById('detail-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        const adds = callsOf(invoke, 'add_transaction_detail');
        expect(adds).toHaveLength(1);
        expect(adds[0]).toMatchObject({ amountIncludingTax: 100, amount: 91, taxAmount: 9, taxRate: 10 });
    });
});
