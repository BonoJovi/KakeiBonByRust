/**
 * Product draft persistence tests (v2.6.0 master-jump 3-hop)
 *
 * When the user is mid-edit in the product modal and clicks
 * "Open in manufacturer master", the product form is serialized to
 * sessionStorage so the user can return via "Back to product entry" to a
 * pre-filled modal, with the just-registered manufacturer auto-selected.
 *
 * Mirrors the contract enforced by:
 *   - res/js/product-management.js (buildProductDraftFromForm / persist / consume / clear / restore)
 *   - res/js/manufacturer-management.js (linkNewManufacturerToProductDraft)
 *
 * Same pattern as product-master-jump-draft.test.js: pure logic, no DOM, no
 * Tauri invoke, to keep the suite fast and free of harness deps.
 */

import { describe, it, expect, beforeEach } from '@jest/globals';

const PRODUCT_DRAFT_KEY = 'kakeibon.product_draft.v1';

function makeFakeSessionStorage() {
    const store = new Map();
    return {
        getItem: (k) => (store.has(k) ? store.get(k) : null),
        setItem: (k, v) => { store.set(k, String(v)); },
        removeItem: (k) => { store.delete(k); },
        clear: () => { store.clear(); },
    };
}

function persistProductDraft(storage, draft) {
    storage.setItem(PRODUCT_DRAFT_KEY, JSON.stringify(draft));
}

function consumeProductDraft(storage) {
    const raw = storage.getItem(PRODUCT_DRAFT_KEY);
    if (!raw) return null;
    try {
        return JSON.parse(raw);
    } catch (e) {
        storage.removeItem(PRODUCT_DRAFT_KEY);
        return null;
    }
}

function clearProductDraft(storage) {
    storage.removeItem(PRODUCT_DRAFT_KEY);
}

// Mirrors manufacturer-management.js linkNewManufacturerToProductDraft:
// look up the new manufacturer by name in the reloaded list and stamp its id
// into the persisted product draft.
function linkNewManufacturerToProductDraft(storage, manufacturerName, manufacturers) {
    const raw = storage.getItem(PRODUCT_DRAFT_KEY);
    if (!raw) return false;
    const match = manufacturers.find(m => m.manufacturer_name === manufacturerName);
    if (!match) return false;
    const draft = JSON.parse(raw);
    draft.manufacturer_id = String(match.manufacturer_id);
    storage.setItem(PRODUCT_DRAFT_KEY, JSON.stringify(draft));
    return true;
}

function sampleProductDraft(overrides = {}) {
    return {
        product_name: 'テスト商品X',
        manufacturer_id: '',
        memo: 'メモ',
        is_disabled: false,
        return_to_transaction_id: '17',
        ...overrides,
    };
}

describe('Product draft persistence (v2.6.0 product → manufacturer side trip)', () => {
    let storage;

    beforeEach(() => {
        storage = makeFakeSessionStorage();
    });

    describe('persist / consume / clear', () => {
        it('persist + consume returns the same payload', () => {
            const draft = sampleProductDraft();
            persistProductDraft(storage, draft);
            expect(consumeProductDraft(storage)).toEqual(draft);
        });

        it('consume returns null on empty storage', () => {
            expect(consumeProductDraft(storage)).toBeNull();
        });

        it('consume returns null and discards malformed JSON', () => {
            storage.setItem(PRODUCT_DRAFT_KEY, 'not json{');
            expect(consumeProductDraft(storage)).toBeNull();
            expect(storage.getItem(PRODUCT_DRAFT_KEY)).toBeNull();
        });

        it('clearProductDraft removes the entry', () => {
            persistProductDraft(storage, sampleProductDraft());
            clearProductDraft(storage);
            expect(consumeProductDraft(storage)).toBeNull();
        });

        it('preserves return_to_transaction_id through the round-trip', () => {
            const draft = sampleProductDraft({ return_to_transaction_id: '42' });
            persistProductDraft(storage, draft);
            expect(consumeProductDraft(storage).return_to_transaction_id).toBe('42');
        });

        it('preserves null return_to_transaction_id (came from menu, not detail)', () => {
            const draft = sampleProductDraft({ return_to_transaction_id: null });
            persistProductDraft(storage, draft);
            expect(consumeProductDraft(storage).return_to_transaction_id).toBeNull();
        });
    });

    describe('linkNewManufacturerToProductDraft', () => {
        it('no-ops when no product draft is present', () => {
            const ok = linkNewManufacturerToProductDraft(storage, 'メーカーA', [
                { manufacturer_id: 5, manufacturer_name: 'メーカーA' },
            ]);
            expect(ok).toBe(false);
        });

        it('no-ops when the manufacturer is not in the reloaded list', () => {
            persistProductDraft(storage, sampleProductDraft());
            const ok = linkNewManufacturerToProductDraft(storage, 'メーカーA', [
                { manufacturer_id: 99, manufacturer_name: '別メーカー' },
            ]);
            expect(ok).toBe(false);
            // Draft preserved so the user can retry.
            expect(consumeProductDraft(storage).manufacturer_id).toBe('');
        });

        it('stamps the new manufacturer id (as string, matches <select>.value)', () => {
            persistProductDraft(storage, sampleProductDraft({ manufacturer_id: '' }));
            const ok = linkNewManufacturerToProductDraft(storage, 'メーカーA', [
                { manufacturer_id: 3, manufacturer_name: '別メーカー' },
                { manufacturer_id: 7, manufacturer_name: 'メーカーA' },
            ]);
            expect(ok).toBe(true);
            const out = consumeProductDraft(storage);
            expect(out.manufacturer_id).toBe('7');
        });

        it('preserves all non-manufacturer product fields', () => {
            const draft = sampleProductDraft({
                product_name: '保存テスト',
                memo: 'メモ',
                is_disabled: true,
                return_to_transaction_id: '99',
            });
            persistProductDraft(storage, draft);
            linkNewManufacturerToProductDraft(storage, 'メーカーA', [
                { manufacturer_id: 5, manufacturer_name: 'メーカーA' },
            ]);
            const out = consumeProductDraft(storage);
            expect(out.product_name).toBe('保存テスト');
            expect(out.memo).toBe('メモ');
            expect(out.is_disabled).toBe(true);
            expect(out.return_to_transaction_id).toBe('99');
        });

        it('overwrites a previously-selected manufacturer_id', () => {
            // User had picked manufacturer 2, then jumped to register a new one.
            persistProductDraft(storage, sampleProductDraft({ manufacturer_id: '2' }));
            linkNewManufacturerToProductDraft(storage, '新規メーカー', [
                { manufacturer_id: 2, manufacturer_name: '既存メーカー' },
                { manufacturer_id: 11, manufacturer_name: '新規メーカー' },
            ]);
            expect(consumeProductDraft(storage).manufacturer_id).toBe('11');
        });
    });
});
