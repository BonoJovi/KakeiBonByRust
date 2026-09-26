/**
 * Latent audit 2026-09 — product master screen (res/js/product-management.js)
 *
 * IDs covered: M5, L17
 *
 * M5  Bug: the manufacturer <select> is populated only from
 *     get_manufacturers({ includeDisabled: false }). When a product is linked
 *     to a manufacturer that has since been disabled, the edit modal cannot
 *     select that value (select.value silently falls back to ""), and saving
 *     without changes sends manufacturerId: null → MANUFACTURER_ID is wiped.
 *     Expected: editing such a product and saving without changes keeps the
 *     original manufacturer_id.
 *
 * L17 Bug: after adding a product during the detail → product-master jump,
 *     linkNewProductToDraft() looks the product up by name and, when no
 *     candidate matches exactly, falls back to `candidates[0]` — a different
 *     product gets linked to the detail draft and the typed item name is
 *     replaced with that product's name.
 *     Expected: without an exact name match the draft's product link and
 *     item name are left untouched (no silent pick of candidates[0]).
 *
 * The real page module is booted against res/product-management.html (with
 * ?return_to=<transaction_id>, i.e. arriving from the detail modal) via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const DETAIL_DRAFT_KEY = 'kakeibon.detail_draft.v1';

const ENABLED_MANUFACTURER = { manufacturer_id: 1, manufacturer_name: 'ActiveCo', is_disabled: 0 };
const DISABLED_MANUFACTURER = { manufacturer_id: 9, manufacturer_name: 'RetiredCo', is_disabled: 1 };

const PRODUCTS = [
    {
        product_id: 3,
        product_name: 'Soy',
        manufacturer_id: 9,
        manufacturer_name: 'RetiredCo',
        memo: null,
        display_order: 1,
        is_disabled: 0,
    },
];

let searchResults = [];

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_manufacturers':
                return args && args.includeDisabled
                    ? [ENABLED_MANUFACTURER, DISABLED_MANUFACTURER]
                    : [ENABLED_MANUFACTURER];
            case 'get_products':
                return PRODUCTS;
            case 'add_product':
                return 100;
            case 'update_product':
                return null;
            case 'search_products_by_name':
                return searchResults;
            default:
                return null;
        }
    },
});

window.history.replaceState(null, '', '/?return_to=10');
loadPageBody('product-management.html');
await import('../../js/product-management.js');
await bootPage();

function submitProductForm() {
    document.getElementById('product-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
}

describe('product master screen — latent audit 2026-09', () => {
    beforeEach(() => {
        invoke.mockClear();
        sessionStorage.clear();
    });

    test('[latent M5] editing a product of a disabled manufacturer keeps manufacturer_id on save', async () => {
        const editBtn = document.querySelector('#products-tbody .btn-edit');
        expect(editBtn).not.toBeNull();
        editBtn.click();
        await flush(5);

        expect(document.getElementById('product-modal').classList.contains('hidden')).toBe(false);
        expect(document.getElementById('product-name').value).toBe('Soy');

        // Save without changes.
        submitProductForm();
        await flush(10);

        const updates = callsOf(invoke, 'update_product');
        expect(updates).toHaveLength(1);
        expect(updates[0].manufacturerId).toBe(9);
    });

    test('[latent L17] no exact name match → the detail draft is not linked to candidates[0]', async () => {
        const originalDraft = {
            transaction_id: '10',
            detail_id: null,
            item_name: 'Soy Sauce',
            selected_product_id: null,
        };
        sessionStorage.setItem(DETAIL_DRAFT_KEY, JSON.stringify(originalDraft));
        // The lookup returns only a *different* product (e.g. a partial match
        // that sorts before the new row).
        searchResults = [{ product_id: 50, product_name: 'Soy Sauce Light', manufacturer_name: null }];

        document.getElementById('add-product-btn').click();
        await flush(5);
        document.getElementById('product-name').value = 'Soy Sauce';
        submitProductForm();
        await flush(10);

        expect(callsOf(invoke, 'add_product')).toHaveLength(1);

        const draft = JSON.parse(sessionStorage.getItem(DETAIL_DRAFT_KEY));
        expect(draft.selected_product_id).not.toBe(50);
        expect(draft.item_name).toBe('Soy Sauce');
    });
});
