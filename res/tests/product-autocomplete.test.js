/**
 * Product autocomplete (v2.6.0 master integration) — pure-logic tests
 *
 * The real autocomplete lives in res/js/transaction-detail-management.js and
 * is tightly coupled to the DOM + Tauri invoke. These tests verify the
 * state-machine contract that the autocomplete code follows:
 *
 *   1. A keystroke after a master selection demotes the row back to free-text
 *      (PRODUCT_ID = null), so we don't silently send a stale id when the
 *      user types over a previous pick.
 *   2. A fresh selection re-sets PRODUCT_ID from the candidate.
 *   3. Edit-mode restoration: opening a detail with product_id != null seeds
 *      the hidden field; opening a free-text detail leaves it null.
 *   4. Stale-fetch protection: an older fetch's results must not overwrite a
 *      newer one's candidates.
 */

import { describe, it, expect, beforeEach } from '@jest/globals';

// Mirror the state machine from transaction-detail-management.js so we can
// exercise the same transitions without importing the DOM module.
function makeAutocompleteState() {
    return {
        selectedProductId: null,
        candidates: [],
        activeIndex: -1,
        requestToken: 0,
    };
}

function onUserKeystroke(state) {
    state.selectedProductId = null;
}

function onSelectCandidate(state, candidate) {
    state.selectedProductId = candidate.product_id;
    state.candidates = [];
    state.activeIndex = -1;
}

function onOpenForEdit(state, detail) {
    state.selectedProductId = detail.product_id ?? null;
}

function onReset(state) {
    state.selectedProductId = null;
    state.candidates = [];
    state.activeIndex = -1;
    state.requestToken++;
}

function applyFetchResult(state, token, results) {
    if (token !== state.requestToken) return false;
    state.candidates = results;
    state.activeIndex = -1;
    return true;
}

describe('Product autocomplete state machine (v2.6.0)', () => {
    let state;

    beforeEach(() => {
        state = makeAutocompleteState();
    });

    describe('Selection → keystroke demotion', () => {
        it('should set product_id when a candidate is selected', () => {
            onSelectCandidate(state, { product_id: 42, product_name: 'サバ缶' });
            expect(state.selectedProductId).toBe(42);
        });

        it('should clear product_id (free-text fallback) when the user types after a selection', () => {
            onSelectCandidate(state, { product_id: 42, product_name: 'サバ缶' });
            onUserKeystroke(state);
            expect(state.selectedProductId).toBeNull();
        });

        it('should update product_id when a different candidate is selected', () => {
            onSelectCandidate(state, { product_id: 42, product_name: 'サバ缶' });
            onSelectCandidate(state, { product_id: 99, product_name: '味噌' });
            expect(state.selectedProductId).toBe(99);
        });
    });

    describe('Edit-mode restoration', () => {
        it('should set product_id when a product-linked detail is opened', () => {
            onOpenForEdit(state, { product_id: 7, item_name: 'whatever' });
            expect(state.selectedProductId).toBe(7);
        });

        it('should leave product_id null when a free-text detail is opened', () => {
            onOpenForEdit(state, { product_id: null, item_name: 'free text' });
            expect(state.selectedProductId).toBeNull();
        });

        it('should treat product_id as null when it is undefined (defensive)', () => {
            onOpenForEdit(state, { item_name: 'no product_id field at all' });
            expect(state.selectedProductId).toBeNull();
        });

        it('should clear the selection when it is reset', () => {
            onSelectCandidate(state, { product_id: 42, product_name: 'X' });
            onReset(state);
            expect(state.selectedProductId).toBeNull();
            expect(state.candidates).toEqual([]);
        });
    });

    describe('Stale-fetch protection', () => {
        it('should reject the results when they come from an older fetch token', () => {
            const oldToken = ++state.requestToken;
            // Simulate the user typing again, which bumps the token
            ++state.requestToken;

            const applied = applyFetchResult(state, oldToken, [
                { product_id: 1, product_name: 'stale' },
            ]);
            expect(applied).toBe(false);
            expect(state.candidates).toEqual([]);
        });

        it('should accept the results when they match the current token', () => {
            const currentToken = ++state.requestToken;
            const applied = applyFetchResult(state, currentToken, [
                { product_id: 1, product_name: 'fresh' },
            ]);
            expect(applied).toBe(true);
            expect(state.candidates).toEqual([{ product_id: 1, product_name: 'fresh' }]);
            expect(state.activeIndex).toBe(-1);
        });

        it('should make an in-flight fetch stale when it is reset', () => {
            const inflightToken = ++state.requestToken;
            onReset(state);
            const applied = applyFetchResult(state, inflightToken, [
                { product_id: 1, product_name: 'late' },
            ]);
            expect(applied).toBe(false);
        });
    });
});
