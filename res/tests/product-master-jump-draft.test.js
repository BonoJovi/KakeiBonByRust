/**
 * Product master jump — draft persistence tests (v2.6.0 follow-up)
 *
 * When the user clicks "Open in product master" from inside the transaction
 * detail modal, the in-flight form state is serialized to sessionStorage so
 * it can be restored when the user returns via "Back to detail entry". These
 * tests cover the pure serialize / consume / clear contract — the real
 * functions are exported from res/js/transaction-detail-management.js but
 * importing that module pulls in DOM + Tauri invoke, so we mirror the
 * contract here exactly (matches the pattern used by product-autocomplete.test.js).
 */

import { describe, it, expect, beforeEach } from '@jest/globals';

const DETAIL_DRAFT_KEY = 'kakeibon.detail_draft.v1';

// Tiny in-memory sessionStorage stand-in so the tests don't need jsdom.
function makeFakeSessionStorage() {
    const store = new Map();
    return {
        getItem: (k) => (store.has(k) ? store.get(k) : null),
        setItem: (k, v) => { store.set(k, String(v)); },
        removeItem: (k) => { store.delete(k); },
        clear: () => { store.clear(); },
        _store: store,
    };
}

function persistDraft(storage, draft) {
    storage.setItem(DETAIL_DRAFT_KEY, JSON.stringify(draft));
}

function consumeDraft(storage) {
    const raw = storage.getItem(DETAIL_DRAFT_KEY);
    if (!raw) return null;
    try {
        return JSON.parse(raw);
    } catch (e) {
        storage.removeItem(DETAIL_DRAFT_KEY);
        return null;
    }
}

function clearDraft(storage) {
    storage.removeItem(DETAIL_DRAFT_KEY);
}

// Mirrors product-management.js linkNewProductToDraft(): updates the persisted
// draft so the user comes back with the new master entry selected.
function linkNewProductToDraft(storage, productName, candidates) {
    const raw = storage.getItem(DETAIL_DRAFT_KEY);
    if (!raw) return false;
    if (!candidates || candidates.length === 0) return false;
    const match = candidates.find(c => c.product_name === productName) || candidates[0];
    const draft = JSON.parse(raw);
    draft.selected_product_id = match.product_id;
    draft.item_name = match.product_name;
    storage.setItem(DETAIL_DRAFT_KEY, JSON.stringify(draft));
    return true;
}

function sampleDraft(overrides = {}) {
    return {
        transaction_id: '17',
        detail_id: null,
        item_name: 'サバ缶',
        category2_code: 'FOOD',
        category3_code: 'CAN',
        tax_rate: '8',
        amount_excluding_tax: '180',
        amount_including_tax: '194',
        tax_amount: '14',
        memo: '昼食',
        selected_product_id: null,
        ...overrides,
    };
}

describe('Detail draft persistence (v2.6.0 master jump round-trip)', () => {
    let storage;

    beforeEach(() => {
        storage = makeFakeSessionStorage();
    });

    describe('persist / consume / clear', () => {
        it('should return the same payload when a draft is persisted and then consumed', () => {
            const draft = sampleDraft();
            persistDraft(storage, draft);
            expect(consumeDraft(storage)).toEqual(draft);
        });

        it('should return null from consume when nothing is stored', () => {
            expect(consumeDraft(storage)).toBeNull();
        });

        it('should return null and clear storage when the stored draft is malformed JSON', () => {
            storage.setItem(DETAIL_DRAFT_KEY, '{not json');
            expect(consumeDraft(storage)).toBeNull();
            // Subsequent consume must also see empty storage.
            expect(storage.getItem(DETAIL_DRAFT_KEY)).toBeNull();
        });

        it('should remove the persisted entry when clearDraft is called', () => {
            persistDraft(storage, sampleDraft());
            clearDraft(storage);
            expect(consumeDraft(storage)).toBeNull();
        });

        it('should overwrite the earlier draft when persist is called again', () => {
            persistDraft(storage, sampleDraft({ item_name: 'first' }));
            persistDraft(storage, sampleDraft({ item_name: 'second' }));
            expect(consumeDraft(storage).item_name).toBe('second');
        });

        it('should keep detail_id and selected_product_id when an edit-mode draft makes a round trip', () => {
            const draft = sampleDraft({
                detail_id: '42',
                selected_product_id: 7,
            });
            persistDraft(storage, draft);
            const out = consumeDraft(storage);
            expect(out.detail_id).toBe('42');
            expect(out.selected_product_id).toBe(7);
        });
    });

    describe('linkNewProductToDraft (post-add canonicalization)', () => {
        it('should do nothing when there is no persisted draft', () => {
            const updated = linkNewProductToDraft(storage, 'サバ缶', [
                { product_id: 5, product_name: 'サバ缶' },
            ]);
            expect(updated).toBe(false);
            expect(consumeDraft(storage)).toBeNull();
        });

        it('should do nothing when the search returned no candidates', () => {
            persistDraft(storage, sampleDraft());
            const updated = linkNewProductToDraft(storage, 'サバ缶', []);
            expect(updated).toBe(false);
            // Draft must remain untouched so the user can retry.
            expect(consumeDraft(storage).selected_product_id).toBeNull();
        });

        it('should put the product into the draft when a candidate matches the name exactly', () => {
            persistDraft(storage, sampleDraft({ item_name: 'サバ缶' }));
            const updated = linkNewProductToDraft(storage, 'サバ缶', [
                { product_id: 10, product_name: 'サバ缶詰' },
                { product_id: 11, product_name: 'サバ缶' },
            ]);
            expect(updated).toBe(true);
            const out = consumeDraft(storage);
            expect(out.selected_product_id).toBe(11);
            expect(out.item_name).toBe('サバ缶');
        });

        it('should fall back to the first candidate when no candidate matches the name exactly', () => {
            persistDraft(storage, sampleDraft({ item_name: 'サバ' }));
            const updated = linkNewProductToDraft(storage, 'サバ', [
                { product_id: 20, product_name: 'サバ缶' },
                { product_id: 21, product_name: 'サバ味噌' },
            ]);
            expect(updated).toBe(true);
            const out = consumeDraft(storage);
            // First candidate is the fallback — canonicalizes item_name too.
            expect(out.selected_product_id).toBe(20);
            expect(out.item_name).toBe('サバ缶');
        });

        it('should keep all non-product fields when the draft is linked to a product', () => {
            const draft = sampleDraft({
                detail_id: '99',
                amount_excluding_tax: '500',
                memo: 'メモテスト',
            });
            persistDraft(storage, draft);
            linkNewProductToDraft(storage, 'サバ缶', [
                { product_id: 5, product_name: 'サバ缶' },
            ]);
            const out = consumeDraft(storage);
            expect(out.detail_id).toBe('99');
            expect(out.amount_excluding_tax).toBe('500');
            expect(out.memo).toBe('メモテスト');
            expect(out.transaction_id).toBe('17');
        });
    });
});
