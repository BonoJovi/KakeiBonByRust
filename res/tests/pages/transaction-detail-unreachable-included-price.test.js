// A tax-included price with no exact tax-excluded split (1000 at 10 % floor) is kept as typed (latent-audit scan2-T2)
/**
 * T2  calculateFromIncluding derives tax as round(excluded * rate), so only
 *     prices n + round(n * rate / 100) are reachable. At 10 % / floor,
 *     n = 909 gives 999 and n = 910 gives 1001, so a typed 1000 is rewritten
 *     to 999 and saved as 909 / 90 / 999.
 *     Expected (owner decision 2026-10-01): the typed tax-included amount is
 *     kept as is and tax = included - round(included * 100 / (100 + rate)),
 *     i.e. 1000 -> excluded 909, tax 91, included 1000.
 *     TAX_AMOUNT == round(AMOUNT * rate) need not hold.
 *
 * Real page module (transaction-detail-management.js + detail-tax-calc.js)
 * booted against res/transaction-detail-management.html. The value is put
 * in with one `input` event (a paste), so T1's per-keystroke rewrite plays
 * no part.
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
await import('../../js/transaction-detail-management.js');
await bootPage();

describe('latent-audit scan2 T2 — unreachable tax-included prices', () => {
    test('[T2] 1000 tax-included at 10 % (floor) is kept, with excluded 909 and tax 91', async () => {
        document.getElementById('add-detail-btn').click();
        await flush(10);
        expect(document.getElementById('detail-modal').classList.contains('hidden')).toBe(false);

        document.getElementById('item-name').value = 'Receipt item';
        document.getElementById('tax-rate').value = '10';

        const included = document.getElementById('amount-including-tax');
        included.value = '1000';
        included.dispatchEvent(new Event('input', { bubbles: true }));
        included.dispatchEvent(new Event('change', { bubbles: true }));

        expect(included.value).toBe('1000');
        expect(document.getElementById('amount-excluding-tax').value).toBe('909');
        expect(String(document.getElementById('tax-amount').value)).toBe('91');

        document.getElementById('detail-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        const adds = callsOf(invoke, 'add_transaction_detail');
        expect(adds).toHaveLength(1);
        expect(adds[0]).toMatchObject({ amountIncludingTax: 1000, amount: 909, taxAmount: 91, taxRate: 10 });
    });
});
