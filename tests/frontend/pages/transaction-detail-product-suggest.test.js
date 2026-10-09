// The detail item-name field offers product suggestions as soon as it gets focus, ranked by the chosen category
/**
 * Transaction detail screen (res/js/transaction-detail-management.js).
 *
 * The item name field has a product autocomplete, but the list only appeared
 * after typing, so users did not know they could pick a registered product.
 *
 * Expected:
 * - The medium and minor categories come before the item name in the detail
 *   window, so the categories are chosen first.
 * - The item name field shows a hint (placeholder) that typing lists products.
 * - Focusing the empty item name field lists product suggestions at once.
 * - The suggestion request carries the detail's categories (category1 from the
 *   header, category2 and category3 from the window) so the backend can rank
 *   products used in that category first.
 * - Picking a suggestion closes the list and it does not open again by itself.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf, definedI18nKeys,
} from './_page-harness.js';

const TRANSACTION_ID = 10;

const HEADER = {
    transaction_id: TRANSACTION_ID,
    transaction_date: '2026-09-01 10:00:00',
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    from_account_name: 'Cash',
    to_account_code: 'NONE',
    to_account_name: '指定なし',
    shop_name: 'Shop',
    total_amount: 0,
    tax_rounding_type: 0,
};

const cat1 = { category1_code: 'EXPENSE', category1_name_i18n: 'Expense', is_disabled: 0 };
const food = { category2_code: 'C2_E_1', category2_name_i18n: 'Food', is_disabled: 0 };
const snack = { category3_code: 'C3_SNACK', category3_name_i18n: 'Snack', is_disabled: 0 };
const TREE = [{ category1: cat1, children: [{ category2: food, children: [snack] }] }];

const SUGGESTIONS = [
    { product_id: 1, product_name: 'Dried mango', manufacturer_name: null },
    { product_id: 2, product_name: 'Apple chips', manufacturer_name: 'FruitCo' },
];

// When set, the next search_products_by_name answers only when resolved.
let nextSearch = null;
const deferredSearch = () => {
    let resolve;
    const promise = new Promise((r) => { resolve = r; });
    return { promise, resolve };
};

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
                return TREE;
            case 'search_products_by_name':
                return nextSearch ? nextSearch.promise : SUGGESTIONS;
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

const itemName = () => document.getElementById('item-name');
const dropdown = () => document.getElementById('product-autocomplete-dropdown');
const dropdownOpen = () => !dropdown().classList.contains('hidden');
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function openAddDetail() {
    document.getElementById('add-detail-btn').click();
    await flush(10);
}

describe('detail item-name product suggestions', () => {
    beforeEach(() => invoke.mockClear());

    test('should put the medium and minor categories before the item name when the detail window is shown', () => {
        const ids = [...document.querySelectorAll('#detail-form input, #detail-form select')]
            .map((el) => el.id);
        expect(ids.indexOf('category2-code')).toBeLessThan(ids.indexOf('category3-code'));
        expect(ids.indexOf('category3-code')).toBeLessThan(ids.indexOf('item-name'));
    });

    test('should have a defined placeholder and tooltip hint when the item name field is shown', () => {
        // A short placeholder fits the field; the full hint is the tooltip.
        for (const attr of ['data-i18n-placeholder', 'data-i18n-title']) {
            const key = itemName().getAttribute(attr);
            expect(key).toBeTruthy();
            expect(definedI18nKeys().has(key)).toBe(true);
        }
    });

    test('should list suggestions with the chosen categories when the empty item name field gets focus', async () => {
        await openAddDetail();

        const c2 = document.getElementById('category2-code');
        c2.value = 'C2_E_1';
        c2.dispatchEvent(new Event('change'));
        await flush(5);
        const c3 = document.getElementById('category3-code');
        c3.value = 'C3_SNACK';
        c3.dispatchEvent(new Event('change'));
        await flush(5);

        itemName().value = '';
        itemName().focus();
        await flush(10);

        const calls = callsOf(invoke, 'search_products_by_name');
        expect(calls.length).toBeGreaterThan(0);
        expect(calls.at(-1)).toEqual({
            query: '',
            category1Code: 'EXPENSE',
            category2Code: 'C2_E_1',
            category3Code: 'C3_SNACK',
        });
        expect(dropdownOpen()).toBe(true);
        expect(dropdown().textContent).toContain('Dried mango');
    });

    test('should not show the list when focus has left the field before the answer', async () => {
        // Close the list from the previous test, then focus and leave at once.
        document.getElementById('category2-code').focus();
        await wait(150);
        itemName().focus();
        document.getElementById('category2-code').focus();
        await flush(10);
        expect(dropdownOpen()).toBe(false);
        itemName().focus();
        await flush(10);
    });

    test('should not show the stale answer of the focus search when the user types while it is pending', async () => {
        document.getElementById('category2-code').focus();
        await wait(150);
        itemName().value = '';
        nextSearch = deferredSearch();
        const focusSearch = nextSearch;
        itemName().focus();
        await flush(5);
        nextSearch = null;

        // The user types before the focus search answers.
        itemName().value = 'Ap';
        itemName().dispatchEvent(new Event('input'));
        focusSearch.resolve(SUGGESTIONS);
        await flush(10);
        // The empty-field answer must not be shown for "Ap".
        expect(dropdownOpen()).toBe(false);

        // The typed search (after the 180 ms debounce) shows its own answer.
        await wait(250);
        await flush(5);
        expect(dropdownOpen()).toBe(true);
        expect(callsOf(invoke, 'search_products_by_name').at(-1).query).toBe('Ap');
    });

    test('should close the list and keep it closed when a suggestion is picked', async () => {
        document.getElementById('category2-code').focus();
        await wait(150);
        itemName().value = '';
        itemName().focus();
        await flush(10);
        expect(dropdownOpen()).toBe(true);
        const item = dropdown().querySelector('.product-autocomplete-item[data-idx="0"]');
        expect(item).not.toBeNull();
        item.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, cancelable: true }));
        await flush(5);

        expect(itemName().value).toBe('Dried mango');
        expect(document.getElementById('product-id').value).toBe('1');
        expect(dropdownOpen()).toBe(false);

        // The autocomplete debounce is 180 ms; nothing may reopen the list.
        await wait(300);
        await flush(5);
        expect(dropdownOpen()).toBe(false);
    });
});
