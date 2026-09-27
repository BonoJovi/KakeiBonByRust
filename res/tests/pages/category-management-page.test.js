/**
 * Category management screen (res/js/category-management.js) — regression
 * tests for latent-audit L19.
 *
 * L19 Moving or showing a category that no longer exists (e.g. changed in
 *     another window) surfaced a raw database error ("no rows returned") or
 *     a silent success. The backend now reports a structured `not_found`;
 *     pinned here: the screen shows category_mgmt.not_found and reloads the
 *     tree, like the hide flow already did.
 *
 * The real page module is booted against res/category-management.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const NOT_FOUND = { code: 'not_found', message: 'Category not found', entity: 'category' };

const cat2 = (code, name, isDisabled = 0) => ({
    category2: {
        category1_code: 'EXPENSE',
        category2_code: code,
        category2_name: name,
        category2_name_i18n: name,
        is_disabled: isDisabled,
    },
    children: [],
});

const TREE = [
    {
        category1: { category1_code: 'EXPENSE', category1_name: '支出', category1_name_i18n: 'Expense' },
        children: [cat2('C2_E_1', 'Food'), cat2('C2_E_2', 'Daily'), cat2('C2_E_3', 'Hidden', 1)],
    },
];

const { invoke, showToast } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_category_tree_all_with_lang':
                return TREE;
            case 'move_category2_up':
            case 'move_category2_down':
            case 'enable_category2':
                return Promise.reject(NOT_FOUND);
            default:
                return null;
        }
    },
});

loadPageBody('category-management.html');
await import('../../js/category-management.js');
await bootPage();

async function expandExpense() {
    const header = document.querySelector('.category-level-1[data-category-code="EXPENSE"] .expand-icon');
    if (header && !document.querySelector('.category-level-2')) {
        header.click();
        await flush(5);
    }
}

function actionButton(action, category2Code) {
    return document.querySelector(
        `.category-level-2 [data-action="${action}"][data-category2-code="${category2Code}"]`
    );
}

describe('category management — regression (latent audit 2026-09)', () => {
    beforeEach(async () => {
        invoke.mockClear();
        showToast.mockClear();
        await expandExpense();
    });

    test('[L19] should show the not-found message and reload the tree when moving a vanished category', async () => {
        const up = actionButton('move-up', 'C2_E_2');
        expect(up).not.toBeNull();
        up.click();
        await flush(10);

        expect(callsOf(invoke, 'move_category2_up')).toHaveLength(1);
        expect(showToast).toHaveBeenCalledWith('category_mgmt.not_found', { variant: 'error' });
        expect(callsOf(invoke, 'get_category_tree_all_with_lang').length).toBeGreaterThan(0);
    });

    test('[L19] should show the not-found message and reload the tree when showing a vanished category', async () => {
        const show = actionButton('show', 'C2_E_3');
        expect(show).not.toBeNull();
        show.click();
        await flush(10);

        expect(callsOf(invoke, 'enable_category2')).toHaveLength(1);
        expect(showToast).toHaveBeenCalledWith('category_mgmt.not_found', { variant: 'error' });
        expect(callsOf(invoke, 'get_category_tree_all_with_lang').length).toBeGreaterThan(0);
    });
});
