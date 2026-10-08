/**
 * Transaction list screen (res/js/transaction-management.js).
 *
 * scan2-M7  `loadCategoriesForFilter` builds the category1/2/3 filter selects
 *           from `get_category_tree_with_lang`, which returns enabled rows
 *           only. After the user hides the CATEGORY2 外食 (and so its
 *           CATEGORY3s), past 外食 transactions can no longer be isolated in
 *           the list, although aggregation still shows their totals.
 *
 * Expected (owner decision 2026-10-01): the list filter also offers hidden
 * categories, labelled "(hidden)" via i18n (entry pickers keep excluding
 * them).
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush,
} from './_page-harness.js';

const cat3 = (code, name, isDisabled) => ({
    category3_code: code, category3_name_i18n: name, display_order: 1, is_disabled: isDisabled,
});

// Tree including hidden rows (get_category_tree_all_with_lang shape).
const TREE_ALL = [{
    category1: { category1_code: 'EXPENSE', category1_name_i18n: '支出', display_order: 1 },
    children: [
        {
            category2: { category2_code: 'C2_E_1', category2_name_i18n: '食費', display_order: 1, is_disabled: 0 },
            children: [cat3('C3_E_1_1', '米', 0)],
        },
        {
            category2: { category2_code: 'C2_E_2', category2_name_i18n: '外食', display_order: 2, is_disabled: 1 },
            children: [cat3('C3_E_2_1', 'ランチ', 1)],
        },
    ],
}];

// Enabled-only tree (get_category_tree_with_lang shape): 外食 is gone.
const TREE_ENABLED = [{
    category1: TREE_ALL[0].category1,
    children: [{
        category2: { category2_code: 'C2_E_1', category2_name_i18n: '食費', display_order: 1 },
        children: [{ category3_code: 'C3_E_1_1', category3_name_i18n: '米', display_order: 1 }],
    }],
}];

mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return TREE_ENABLED;
            case 'get_category_tree_all_with_lang':
                return TREE_ALL;
            case 'get_transactions':
                return { transactions: [], total_count: 0, page: args.page, per_page: 50, total_pages: 0 };
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

function choose(selectId, value) {
    const select = document.getElementById(selectId);
    select.value = value;
    select.dispatchEvent(new Event('change', { bubbles: true }));
}

const optionFor = (selectId, value) =>
    Array.from(document.getElementById(selectId).options).find((o) => o.value === value);

describe('transaction-list filter and hidden categories (scan2-M7)', () => {
    test('should offer a hidden CATEGORY2 and its CATEGORY3, labelled as hidden, when the list filter is shown (scan2-M7)', async () => {
        choose('filter-category1', 'EXPENSE');
        await flush(3);

        // Precondition / unchanged: the enabled sibling is offered unlabelled.
        expect(optionFor('filter-category2', 'C2_E_1')?.textContent).toBe('食費');

        const hiddenCat2 = optionFor('filter-category2', 'C2_E_2');
        expect(hiddenCat2).toBeDefined();
        expect(hiddenCat2.textContent).toContain('外食');
        expect(hiddenCat2.textContent).not.toBe('外食'); // carries the "(hidden)" label

        choose('filter-category2', 'C2_E_2');
        await flush(3);
        const hiddenCat3 = optionFor('filter-category3', 'C3_E_2_1');
        expect(hiddenCat3).toBeDefined();
        expect(hiddenCat3.textContent).toContain('ランチ');
    });
});
