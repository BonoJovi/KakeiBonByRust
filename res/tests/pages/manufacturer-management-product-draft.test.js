/**
 * Manufacturer master screen (res/js/manufacturer-management.js) — linking a
 * new manufacturer to the product draft.
 *
 * Opened from the product window (?return_to_product=1), the page lets the
 * user add a manufacturer. After the add it reloads the manufacturer list,
 * finds the new manufacturer by name and writes its id (as a string, the
 * value of the product window's <select>) into the stored product draft, so
 * the product window comes back with it selected. Nothing else in the draft
 * changes; without a draft, or when the name is not in the reloaded list, the
 * draft is left as it is. Opened from the menu, the draft is not touched.
 *
 * These replace part of a former test file that tested a copy of this
 * logic. The real page module is booted against
 * res/manufacturer-management.html via ./_page-harness.js. Each test boots
 * the page again at its own URL (page body reloaded, DOMContentLoaded fired
 * again).
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const PRODUCT_DRAFT_KEY = 'kakeibon.product_draft.v1';

// What get_manufacturers returns (the list is reloaded after an add).
let manufacturerList = [];

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_manufacturers':
                return manufacturerList;
            case 'add_manufacturer':
                return null;
            default:
                return null;
        }
    },
});

window.history.replaceState(null, '', '/');
loadPageBody('manufacturer-management.html');
await import('../../js/manufacturer-management.js');
await bootPage();

async function bootAt(url) {
    window.history.replaceState(null, '', url);
    loadPageBody('manufacturer-management.html');
    await bootPage();
}

async function addManufacturer(name) {
    if (document.getElementById('manufacturer-modal').classList.contains('hidden')) {
        document.getElementById('add-manufacturer-btn').click();
        await flush(5);
    }
    document.getElementById('manufacturer-name').value = name;
    document.getElementById('manufacturer-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
}

function sampleProductDraft(overrides = {}) {
    return {
        editing_product_id: null,
        product_name: 'テスト商品X',
        manufacturer_id: '',
        memo: 'メモ',
        is_disabled: false,
        return_to_transaction_id: '17',
        ...overrides,
    };
}

function storeDraft(draft) {
    sessionStorage.setItem(PRODUCT_DRAFT_KEY, JSON.stringify(draft));
}

function storedDraft() {
    return JSON.parse(sessionStorage.getItem(PRODUCT_DRAFT_KEY));
}

describe('manufacturer master screen — new manufacturer linked to the product draft', () => {
    beforeEach(() => {
        sessionStorage.clear();
        invoke.mockClear();
        manufacturerList = [];
    });

    test('should not create a product draft when no draft is stored', async () => {
        await bootAt('/?return_to_product=1');
        manufacturerList = [{ manufacturer_id: 5, manufacturer_name: 'メーカーA', is_disabled: 0 }];

        await addManufacturer('メーカーA');

        expect(callsOf(invoke, 'add_manufacturer')).toHaveLength(1);
        expect(sessionStorage.getItem(PRODUCT_DRAFT_KEY)).toBeNull();
    });

    test('should leave the draft alone when the new manufacturer is not in the reloaded list', async () => {
        storeDraft(sampleProductDraft());
        await bootAt('/?return_to_product=1');
        manufacturerList = [{ manufacturer_id: 99, manufacturer_name: '別メーカー', is_disabled: 0 }];

        await addManufacturer('メーカーA');

        expect(callsOf(invoke, 'add_manufacturer')).toHaveLength(1);
        expect(storedDraft()).toEqual(sampleProductDraft());
    });

    test('should write the new manufacturer id as a string when the manufacturer is in the reloaded list', async () => {
        storeDraft(sampleProductDraft({ manufacturer_id: '' }));
        await bootAt('/?return_to_product=1');
        manufacturerList = [
            { manufacturer_id: 3, manufacturer_name: '別メーカー', is_disabled: 0 },
            { manufacturer_id: 7, manufacturer_name: 'メーカーA', is_disabled: 0 },
        ];

        await addManufacturer('メーカーA');

        expect(storedDraft().manufacturer_id).toBe('7');
    });

    test('should keep all non-manufacturer fields when the manufacturer is linked', async () => {
        const draft = sampleProductDraft({
            editing_product_id: 4,
            product_name: '保存テスト',
            memo: 'メモ',
            is_disabled: true,
            return_to_transaction_id: '99',
        });
        storeDraft(draft);
        await bootAt('/?return_to_product=1');
        manufacturerList = [{ manufacturer_id: 5, manufacturer_name: 'メーカーA', is_disabled: 0 }];

        await addManufacturer('メーカーA');

        expect(storedDraft()).toEqual({ ...draft, manufacturer_id: '5' });
    });

    test('should replace the manufacturer id when one was already selected', async () => {
        // The user had picked manufacturer 2, then left to register a new one.
        storeDraft(sampleProductDraft({ manufacturer_id: '2' }));
        await bootAt('/?return_to_product=1');
        manufacturerList = [
            { manufacturer_id: 2, manufacturer_name: '既存メーカー', is_disabled: 0 },
            { manufacturer_id: 11, manufacturer_name: '新規メーカー', is_disabled: 0 },
        ];

        await addManufacturer('新規メーカー');

        expect(storedDraft().manufacturer_id).toBe('11');
    });

    test('should leave the draft alone when the manufacturer master was opened from the menu', async () => {
        storeDraft(sampleProductDraft());
        await bootAt('/');
        manufacturerList = [{ manufacturer_id: 5, manufacturer_name: 'メーカーA', is_disabled: 0 }];

        await addManufacturer('メーカーA');

        expect(callsOf(invoke, 'add_manufacturer')).toHaveLength(1);
        expect(storedDraft()).toEqual(sampleProductDraft());
    });
});
