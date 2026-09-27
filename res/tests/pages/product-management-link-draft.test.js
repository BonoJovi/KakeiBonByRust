/**
 * Product master screen (res/js/product-management.js) — regression tests
 * for latent-audit L17 (detail → product-master jump).
 *
 * L17 Bug: after adding a product during the detail → product-master jump,
 *     linkNewProductToDraft() looks the product up by name and, when no
 *     candidate matches exactly, falls back to `candidates[0]` — a different
 *     product gets linked to the detail draft and the typed item name is
 *     replaced with that product's name.
 *     Expected: without an exact name match the draft's product link and
 *     item name are left untouched (no silent pick of candidates[0]); an
 *     exact match is still linked.
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

describe('product master screen — detail draft link (regression, latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
        sessionStorage.clear();
    });

    test('[L17] should leave the detail draft alone when no product name matches exactly', async () => {
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
        expect(draft).toEqual(originalDraft);
    });

    test('[L17] should link the detail draft to the product whose name matches exactly', async () => {
        sessionStorage.setItem(DETAIL_DRAFT_KEY, JSON.stringify({
            transaction_id: '10',
            detail_id: null,
            item_name: 'Soy Sauce',
            selected_product_id: null,
        }));
        // The exact match sorts after a partial one.
        searchResults = [
            { product_id: 50, product_name: 'Soy Sauce Light', manufacturer_name: null },
            { product_id: 100, product_name: 'Soy Sauce', manufacturer_name: null },
        ];

        document.getElementById('add-product-btn').click();
        await flush(5);
        document.getElementById('product-name').value = 'Soy Sauce';
        submitProductForm();
        await flush(10);

        const draft = JSON.parse(sessionStorage.getItem(DETAIL_DRAFT_KEY));
        expect(draft.selected_product_id).toBe(100);
        expect(draft.item_name).toBe('Soy Sauce');
    });
});
