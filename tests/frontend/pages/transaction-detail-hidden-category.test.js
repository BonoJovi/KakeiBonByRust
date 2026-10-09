// Editing a detail whose category2/3 is hidden keeps the category (latent-audit scan2-T3)
/**
 * T3  The detail modal's category2/3 selects are filled from
 *     get_category_tree_with_lang, which lists enabled categories only
 *     (IS_DISABLED = 0), and nothing adds an option for the hidden value an
 *     existing row holds. `select.value = 'C2_…'` then matches no option, the
 *     select reads '', and the save sends category2Code / category3Code null.
 *     Expected: saving an edited detail keeps its (hidden) category2 and
 *     category3, as the M5 / M7 fixes do for shops, accounts and makers.
 *
 * Real page module booted against res/transaction-detail-management.html.
 * get_category_tree_all_with_lang answers with the hidden entries too
 * (is_disabled = 1), so a fix may use either that command or the row's own
 * category names.
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
    total_amount: 2200,
    tax_rounding_type: 0,
};

const baseDetail = {
    transaction_id: TRANSACTION_ID,
    category1_code: 'EXPENSE',
    amount: 1000,
    tax_rate: 10,
    tax_amount: 100,
    amount_including_tax: 1100,
    product_id: null,
    memo_text: null,
};

// category2 "Eating out" is hidden (and so is its only category3).
const HIDDEN_CAT2_DETAIL = {
    ...baseDetail,
    detail_id: 1,
    category2_code: 'C2_E_HIDDEN',
    category2_name: 'Eating out',
    category3_code: 'C3_H_1',
    category3_name: 'Lunch',
    item_name: 'Old lunch',
};

// category2 "Food" is enabled but its category3 "Snacks" is hidden.
const HIDDEN_CAT3_DETAIL = {
    ...baseDetail,
    detail_id: 2,
    category2_code: 'C2_E_1',
    category2_name: 'Food',
    category3_code: 'C3_HIDDEN',
    category3_name: 'Snacks',
    item_name: 'Old snack',
};

const cat1 = { category1_code: 'EXPENSE', category1_name_i18n: 'Expense', is_disabled: 0 };
const food = { category2_code: 'C2_E_1', category2_name_i18n: 'Food', is_disabled: 0 };
const veg = { category3_code: 'C3_1', category3_name_i18n: 'Veg', is_disabled: 0 };
const snacks = { category3_code: 'C3_HIDDEN', category3_name_i18n: 'Snacks', is_disabled: 1 };
const eatingOut = { category2_code: 'C2_E_HIDDEN', category2_name_i18n: 'Eating out', is_disabled: 1 };
const lunch = { category3_code: 'C3_H_1', category3_name_i18n: 'Lunch', is_disabled: 1 };

// What the backend returns: enabled-only vs. everything.
const ENABLED_TREE = [
    { category1: cat1, children: [{ category2: food, children: [veg] }] },
];
const ALL_TREE = [
    {
        category1: cat1,
        children: [
            { category2: food, children: [veg, snacks] },
            { category2: eatingOut, children: [lunch] },
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
                return [HIDDEN_CAT2_DETAIL, HIDDEN_CAT3_DETAIL];
            case 'get_category_tree_with_lang':
                return ENABLED_TREE;
            case 'get_category_tree_all_with_lang':
                return ALL_TREE;
            case 'search_products_by_name':
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
await import('../../../res/js/transaction-detail-management.js');
await bootPage();

async function editAndSaveMemoOnly(detailId) {
    const editBtn = document.querySelector(`.edit-detail-btn[data-detail-id="${detailId}"]`);
    expect(editBtn).not.toBeNull();
    editBtn.click();
    await flush(10);
    expect(document.getElementById('detail-id').value).toBe(String(detailId));

    // The user changes only the memo.
    document.getElementById('memo').value = 'note';
    document.getElementById('detail-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);

    const updates = callsOf(invoke, 'update_transaction_detail');
    expect(updates).toHaveLength(1);
    return updates[0];
}

describe('latent-audit scan2 T3 — editing a detail with a hidden category', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('should keep a hidden category2 (and its category3) when only the memo is edited (T3)', async () => {
        const sent = await editAndSaveMemoOnly(1);
        expect(sent.category2Code).toBe('C2_E_HIDDEN');
        expect(sent.category3Code).toBe('C3_H_1');
    });

    test('should keep a hidden category3 under an enabled category2 when only the memo is edited (T3)', async () => {
        const sent = await editAndSaveMemoOnly(2);
        expect(sent.category2Code).toBe('C2_E_1');
        expect(sent.category3Code).toBe('C3_HIDDEN');
    });
});
