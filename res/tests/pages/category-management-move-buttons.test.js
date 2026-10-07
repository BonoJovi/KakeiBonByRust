/**
 * Category management screen (res/js/category-management.js) — scan2-M5.
 *
 * Hidden categories are listed after the visible ones, but the ↑/↓ buttons
 * counted them as siblings. The last visible category kept its ↓ button
 * enabled, and clicking it only swapped with a hidden row, so nothing
 * visibly moved.
 *
 * Expected: the first / last *visible* sibling has its ↑ / ↓ disabled, on
 * CATEGORY2 and CATEGORY3 rows alike. Hidden rows have no ↑/↓ buttons.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush,
} from './_page-harness.js';

const cat3 = (code, name, isDisabled = 0) => ({
    category1_code: 'EXPENSE',
    category2_code: 'C2_E_1',
    category3_code: code,
    category3_name: name,
    category3_name_i18n: name,
    display_order: 1,
    is_disabled: isDisabled,
});

const cat2 = (code, name, isDisabled = 0, children = []) => ({
    category2: {
        category1_code: 'EXPENSE',
        category2_code: code,
        category2_name: name,
        category2_name_i18n: name,
        display_order: 1,
        is_disabled: isDisabled,
    },
    children,
});

// get_category_tree_all_with_lang lists hidden rows after the visible ones.
const TREE = [{
    category1: { category1_code: 'EXPENSE', category1_name: '支出', category1_name_i18n: 'Expense' },
    children: [
        cat2('C2_E_1', 'Food', 0, [cat3('C3_E_1_1', 'Rice'), cat3('C3_E_1_2', 'Bread'), cat3('C3_E_1_3', 'Noodles', 1)]),
        cat2('C2_E_2', 'Daily'),
        cat2('C2_E_3', 'Dining', 1),
    ],
}];

mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => (cmd === 'get_category_tree_all_with_lang' ? TREE : null),
});

loadPageBody('category-management.html');
await import('../../js/category-management.js');
await bootPage();

async function expand(selector) {
    const icon = document.querySelector(`${selector} > .category-header .expand-icon`);
    icon.click();
    await flush(5);
}

const button = (level, code, action) => document.querySelector(
    `.category-level-${level}[data-category-code="${code}"] [data-action="${action}"]`
);

describe('scan2-M5 category ↑/↓ buttons ignore hidden siblings', () => {
    beforeAll(async () => {
        await expand('.category-level-1[data-category-code="EXPENSE"]');
        await expand('.category-level-2[data-category-code="C2_E_1"]');
    });

    test('[scan2-M5] the last visible CATEGORY2 cannot move down past hidden ones', () => {
        expect(button(2, 'C2_E_1', 'move-up').disabled).toBe(true);
        expect(button(2, 'C2_E_1', 'move-down').disabled).toBe(false);
        expect(button(2, 'C2_E_2', 'move-up').disabled).toBe(false);
        expect(button(2, 'C2_E_2', 'move-down').disabled).toBe(true);
        expect(button(2, 'C2_E_3', 'move-down')).toBeNull(); // hidden row: no ↑/↓
    });

    test('[scan2-M5] the last visible CATEGORY3 cannot move down past hidden ones', () => {
        expect(button(3, 'C3_E_1_1', 'move-up').disabled).toBe(true);
        expect(button(3, 'C3_E_1_1', 'move-down').disabled).toBe(false);
        expect(button(3, 'C3_E_1_2', 'move-up').disabled).toBe(false);
        expect(button(3, 'C3_E_1_2', 'move-down').disabled).toBe(true);
        expect(button(3, 'C3_E_1_3', 'move-down')).toBeNull();
    });
});
