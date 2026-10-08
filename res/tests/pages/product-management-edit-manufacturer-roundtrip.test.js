// Product EDIT -> manufacturer master -> back reopens the same product in edit mode (latent-audit scan2-M2)
/**
 * Product master screen (res/js/product-management.js).
 *
 * scan2-M2  The "Open in manufacturer master" button is shown in the product
 *           modal in both modes. Its draft (`buildProductDraftFromForm`) has no
 *           product id / mode, and `?restore_product=1` always restores with
 *           `openModal('add')`. Coming back from the manufacturer master, an
 *           edit of product 5 becomes an "Add product" form, so Save calls
 *           add_product (duplicate error, or a second product row).
 *
 * Expected (either fix direction from the bug list):
 *   - the jump button is not offered in edit mode, or
 *   - the round trip reopens the modal in EDIT mode for the same product,
 *     and Save updates product 5 instead of adding a new one.
 *
 * The round trip is driven through the real page module: boot once, edit
 * product 5 and click the jump button (the draft goes to sessionStorage), then
 * fire DOMContentLoaded again with ?restore_product=1, as the page does when
 * the user comes back.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf, isHiddenOrAbsent,
} from './_page-harness.js';

const MANUFACTURER = { manufacturer_id: 1, manufacturer_name: 'DairyCo', is_disabled: 0 };
const MILK = {
    product_id: 5,
    product_name: 'Milk',
    manufacturer_id: 1,
    manufacturer_name: 'DairyCo',
    memo: 'fresh',
    display_order: 5,
    is_disabled: 0,
};

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_manufacturers':
                return [MANUFACTURER];
            case 'get_products':
                return [MILK];
            case 'add_product':
                return 100;
            case 'update_product':
                return null;
            default:
                return null;
        }
    },
});

window.confirm = () => true;
window.alert = () => {};

loadPageBody('product-management.html');
await import('../../js/product-management.js');
await bootPage();

describe('scan2-M2 product edit -> manufacturer master round trip', () => {
    test('should come back in edit mode for the same product when returning from the manufacturer master (or not offer the jump in edit mode) (scan2-M2)', async () => {
        const editBtn = document.querySelector('#product-list .btn-edit, .btn-edit');
        expect(editBtn).not.toBeNull();
        editBtn.click();
        await flush(5);
        // Precondition: the modal is in edit mode for product 5.
        expect(document.getElementById('modal-title').getAttribute('data-i18n')).toBe('product_mgmt.edit');
        expect(document.getElementById('product-name').value).toBe('Milk');

        const jumpBtn = document.getElementById('open-manufacturer-master-btn');
        if (isHiddenOrAbsent(jumpBtn)) {
            // Fix direction B: the jump is not offered while editing.
            return;
        }

        jumpBtn.click(); // persists the draft; jsdom ignores the navigation
        await flush(5);

        // Back from the manufacturer master: the page loads with ?restore_product=1.
        window.history.replaceState(null, '', '/?restore_product=1');
        await bootPage();

        expect(document.getElementById('modal-title').getAttribute('data-i18n')).toBe('product_mgmt.edit');
        expect(document.getElementById('product-name').value).toBe('Milk');

        invoke.mockClear();
        document.getElementById('product-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        expect(callsOf(invoke, 'add_product')).toHaveLength(0);
        const updates = callsOf(invoke, 'update_product');
        expect(updates.length).toBeGreaterThan(0);
        expect(updates[0].productId).toBe(5);
    });
});
