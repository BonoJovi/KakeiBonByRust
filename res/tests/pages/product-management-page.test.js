/**
 * Product master screen (res/js/product-management.js) — regression tests
 * promoted from the 2026-09 latent audit.
 *
 * M5  The manufacturer <select> lists enabled manufacturers only. For a
 *     product linked to a manufacturer that has since been disabled, the edit
 *     modal fell back to "none" and saving without changes wiped
 *     MANUFACTURER_ID. The edit modal now adds the product's disabled
 *     manufacturer as an option (labelled with common.disabled_label).
 *     Pinned: saving without changes keeps the original manufacturer_id.
 *
 * The real page module is booted against res/product-management.html (with
 * ?return_to=<transaction_id>) via ./_page-harness.js.
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

describe('product master screen — regression (latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
        sessionStorage.clear();
    });

    test('should keep manufacturer_id on save when the product\'s manufacturer is disabled (M5)', async () => {
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
});
