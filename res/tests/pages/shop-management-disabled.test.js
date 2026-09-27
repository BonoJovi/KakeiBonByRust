/**
 * Shop master screen (res/js/shop-management.js) — regression tests for
 * latent-audit M7.
 *
 * M7  A shop still used by transactions cannot be deleted, and the in-use
 *     toast says "Disable it instead" — but the shop screen had no way to
 *     disable a shop or to see disabled ones. Pinned: the edit form has a
 *     "disabled" checkbox that is sent on add / update, and the list can
 *     show disabled shops (marked with the disabled label), like the
 *     manufacturer and product screens.
 *
 * The real page module is booted against res/shop-management.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, deferred, callsOf,
} from './_page-harness.js';

const ACTIVE_SHOP = {
    shop_id: 1, user_id: 2, shop_name: 'New Mart', memo: null, display_order: 1, is_disabled: 0,
};
const DISABLED_SHOP = {
    shop_id: 7, user_id: 2, shop_name: 'Old Mart', memo: null, display_order: 2, is_disabled: 1,
};

// When set, get_shops with includeDisabled parks its response here.
let pendingIncludeDisabled = null;

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_shops':
                if (args && args.includeDisabled && pendingIncludeDisabled) {
                    return pendingIncludeDisabled.promise;
                }
                return args && args.includeDisabled ? [ACTIVE_SHOP, DISABLED_SHOP] : [ACTIVE_SHOP];
            default:
                return null;
        }
    },
});

loadPageBody('shop-management.html');
await import('../../js/shop-management.js');
await bootPage();

const rows = () => Array.from(document.querySelectorAll('#shops-tbody tr'));

function submitShopForm() {
    document.getElementById('shop-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
}

describe('shop master screen — disable (regression, latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('[M7] should list disabled shops, marked, only while "show disabled" is on', async () => {
        expect(rows()).toHaveLength(1);

        document.getElementById('toggle-disabled-btn').click();
        await flush(5);

        expect(callsOf(invoke, 'get_shops')[0]).toEqual({ includeDisabled: true });
        expect(rows()).toHaveLength(2);
        expect(rows()[1].cells[0].textContent).toBe('Old Mart[common.disabled_label]');

        document.getElementById('toggle-disabled-btn').click();
        await flush(5);
        expect(rows()).toHaveLength(1);
    });

    test('[M7] should not let a late "show disabled" response overwrite a newer list', async () => {
        pendingIncludeDisabled = deferred();
        document.getElementById('toggle-disabled-btn').click(); // on: response parked
        await flush(2);
        document.getElementById('toggle-disabled-btn').click(); // off: answers at once
        await flush(5);
        expect(rows()).toHaveLength(1);

        pendingIncludeDisabled.resolve([ACTIVE_SHOP, DISABLED_SHOP]);
        pendingIncludeDisabled = null;
        await flush(5);

        expect(rows()).toHaveLength(1);
    });

    test('[M7] should send the disabled checkbox when adding a shop', async () => {
        document.getElementById('add-shop-btn').click();
        await flush(5);
        expect(document.getElementById('shop-is-disabled').checked).toBe(false);

        document.getElementById('shop-name').value = 'Pop-up Store';
        document.getElementById('shop-is-disabled').checked = true;
        submitShopForm();
        await flush(10);

        const adds = callsOf(invoke, 'add_shop');
        expect(adds).toHaveLength(1);
        expect(adds[0]).toMatchObject({ shopName: 'Pop-up Store', isDisabled: 1 });
    });

    test('[M7] should show and send the disabled state when editing a shop', async () => {
        document.getElementById('toggle-disabled-btn').click();
        await flush(5);

        const editDisabled = rows()[1].querySelector('.btn-edit');
        editDisabled.click();
        await flush(5);
        expect(document.getElementById('shop-is-disabled').checked).toBe(true);

        // Enable it again.
        document.getElementById('shop-is-disabled').checked = false;
        submitShopForm();
        await flush(10);

        const updates = callsOf(invoke, 'update_shop');
        expect(updates).toHaveLength(1);
        expect(updates[0]).toMatchObject({ shopId: 7, shopName: 'Old Mart', isDisabled: 0 });

        document.getElementById('toggle-disabled-btn').click();
        await flush(5);
    });
});
