/**
 * Transaction detail screen (res/js/transaction-detail-management.js) —
 * product link of the item-name autocomplete.
 *
 * The detail window keeps the PRODUCT_ID of a picked product (mirrored into
 * the hidden #product-id field) apart from the typed item name:
 *   1. Picking a suggestion sets PRODUCT_ID; picking another one replaces it.
 *   2. A keystroke after a pick drops PRODUCT_ID (free text), so a stale id
 *      is not saved when the user types over a previous pick.
 *   3. Editing a detail restores its PRODUCT_ID; a free-text detail (null or
 *      missing product_id) has none. Reopening the window clears the link.
 *   4. An answer to an older search is not shown: the list shows the newest
 *      search, with no item active, and reopening the window makes a
 *      pending search stale.
 *
 * These replace a former test file that tested a copy of this logic. The
 * real page module is booted against res/transaction-detail-management.html
 * via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, deferred, callsOf,
} from './_page-harness.js';

const TRANSACTION_ID = 10;

const HEADER = {
    transaction_id: TRANSACTION_ID,
    transaction_date: '2026-09-01 10:00:00',
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    from_account_name: 'Cash',
    shop_name: 'Shop',
    total_amount: 3300,
    tax_rounding_type: 0,
};

function detailRow(overrides) {
    return {
        transaction_id: TRANSACTION_ID,
        category1_code: 'EXPENSE',
        category2_code: 'C2_E_1',
        category2_name: 'Food',
        category3_code: 'C3_1',
        category3_name: 'Veg',
        amount: 1000,
        tax_rate: 10,
        tax_amount: 100,
        amount_including_tax: 1100,
        memo_text: null,
        ...overrides,
    };
}

const PRODUCT_LINKED_DETAIL = detailRow({ detail_id: 1, item_name: 'サバ缶', product_id: 7 });
const FREE_TEXT_DETAIL = detailRow({ detail_id: 2, item_name: 'free text', product_id: null });
// A row from before product links existed: no product_id field at all.
const NO_PRODUCT_FIELD_DETAIL = detailRow({ detail_id: 3, item_name: 'old row' });

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

const SUGGESTIONS = [
    { product_id: 42, product_name: 'サバ缶', manufacturer_name: null },
    { product_id: 99, product_name: '味噌', manufacturer_name: 'MisoCo' },
];

// When set, the next search_products_by_name answers only when resolved.
let nextSearch = null;

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_transaction_header_with_info':
                return HEADER;
            case 'get_transaction_details':
                return [PRODUCT_LINKED_DETAIL, FREE_TEXT_DETAIL, NO_PRODUCT_FIELD_DETAIL];
            case 'get_category_tree_with_lang':
            case 'get_category_tree_all_with_lang':
                return CATEGORY_TREE;
            case 'search_products_by_name':
                if (nextSearch) {
                    const pending = nextSearch;
                    nextSearch = null;
                    return pending.promise;
                }
                return SUGGESTIONS;
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
const productId = () => document.getElementById('product-id').value;
const dropdown = () => document.getElementById('product-autocomplete-dropdown');
const dropdownOpen = () => !dropdown().classList.contains('hidden');
const wait = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

async function openAddDetail() {
    document.getElementById('add-detail-btn').click();
    await flush(10);
}

async function openEditDetail(detailId) {
    document.querySelector(`.edit-detail-btn[data-detail-id="${detailId}"]`).click();
    await flush(10);
}

// Focus the item name (which searches at once) and wait for the list.
async function showSuggestions() {
    document.getElementById('category2-code').focus();
    await wait(150); // let the blur hide the earlier list
    itemName().focus();
    await flush(10);
}

async function pickSuggestion(index) {
    await showSuggestions();
    const item = dropdown().querySelector(`.product-autocomplete-item[data-idx="${index}"]`);
    item.dispatchEvent(new MouseEvent('mousedown', { bubbles: true, cancelable: true }));
    await flush(5);
}

function typeIntoItemName(text) {
    itemName().value = text;
    itemName().dispatchEvent(new Event('input'));
}

function submitDetailForm() {
    document.getElementById('detail-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
}

describe('transaction detail screen — product link of the item name', () => {
    beforeEach(() => {
        invoke.mockClear();
        nextSearch = null;
    });

    describe('picking and typing', () => {
        test('should set the product id when a suggestion is picked', async () => {
            await openAddDetail();
            await pickSuggestion(0);
            expect(itemName().value).toBe('サバ缶');
            expect(productId()).toBe('42');
        });

        test('should replace the product id when a different suggestion is picked', async () => {
            await openAddDetail();
            await pickSuggestion(0);
            await pickSuggestion(1);
            expect(productId()).toBe('99');
        });

        test('should save without a product id when the user types after picking a suggestion', async () => {
            await openAddDetail();
            await pickSuggestion(0);
            typeIntoItemName('サバ缶 大');
            expect(productId()).toBe('');

            document.getElementById('category2-code').value = 'C2_E_1';
            document.getElementById('category3-code').value = 'C3_1';
            document.getElementById('tax-rate').value = '10';
            document.getElementById('amount-excluding-tax').value = '1000';
            document.getElementById('amount-including-tax').value = '1100';
            document.getElementById('tax-amount').value = '100';
            submitDetailForm();
            await flush(10);

            const adds = callsOf(invoke, 'add_transaction_detail');
            expect(adds).toHaveLength(1);
            expect(adds[0].productId).toBeNull();
            expect(adds[0].itemName).toBe('サバ缶 大');
        });
    });

    describe('opening the detail window', () => {
        test('should restore the product id when a product-linked detail is opened for editing', async () => {
            await openEditDetail(1);
            expect(productId()).toBe('7');
        });

        test('should have no product id when a free-text detail is opened for editing', async () => {
            await openEditDetail(1);
            await openEditDetail(2);
            expect(productId()).toBe('');
        });

        test('should save a null product id when a detail without a product_id field is opened and saved', async () => {
            await openEditDetail(1);
            await openEditDetail(3);
            expect(productId()).toBe('');

            submitDetailForm();
            await flush(10);
            const updates = callsOf(invoke, 'update_transaction_detail');
            expect(updates).toHaveLength(1);
            expect(updates[0].productId).toBeNull();
        });

        test('should clear the product id and the list when the detail window is opened again', async () => {
            await openAddDetail();
            await pickSuggestion(0);
            await showSuggestions();
            expect(dropdownOpen()).toBe(true);

            await openAddDetail();
            expect(productId()).toBe('');
            expect(dropdownOpen()).toBe(false);
            expect(dropdown().innerHTML).toBe('');
        });
    });

    describe('search answers', () => {
        test('should show the newest answer with no item active when the search answers', async () => {
            await openAddDetail();
            await showSuggestions();
            expect(dropdownOpen()).toBe(true);
            const items = dropdown().querySelectorAll('.product-autocomplete-item');
            expect([...items].map((el) => el.textContent)).toEqual([
                expect.stringContaining('サバ缶'),
                expect.stringContaining('味噌'),
            ]);
            expect(dropdown().querySelector('.product-autocomplete-item--active')).toBeNull();

            // With no item active, Enter does not pick anything.
            itemName().dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', cancelable: true }));
            await flush(5);
            expect(productId()).toBe('');
        });

        test('should not show a pending answer when the detail window is opened again before it arrives', async () => {
            await openAddDetail();
            document.getElementById('category2-code').focus();
            await wait(150);
            const pending = deferred();
            nextSearch = pending;
            itemName().focus();
            await flush(5);

            await openAddDetail();
            // Keep the item name as the focused field, so only the reopen
            // can make the pending answer stale (the page also ignores an
            // answer once focus has left the field).
            const field = itemName();
            const spy = jest.spyOn(document, 'activeElement', 'get').mockReturnValue(field);
            try {
                pending.resolve([{ product_id: 1, product_name: 'late', manufacturer_name: null }]);
                await flush(10);
                expect(dropdownOpen()).toBe(false);
                expect(dropdown().textContent).not.toContain('late');
            } finally {
                spy.mockRestore();
            }
        });
    });
});
