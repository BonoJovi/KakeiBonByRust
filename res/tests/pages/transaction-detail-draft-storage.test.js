/**
 * Transaction detail screen (res/js/transaction-detail-management.js) —
 * detail draft storage for the detail → product master round trip.
 *
 * When the user clicks "Open in product master" from inside the detail
 * modal, the in-flight form is saved to sessionStorage with persistDraft()
 * and read back with consumeDraft() when the user returns. These tests call
 * the real exported persistDraft / consumeDraft / clearDraft (they replace a
 * former test file that tested a copy of these functions).
 *
 * The page module is imported through ./_page-harness.js so its other
 * imports (Tauri invoke, i18n, menu bar, ...) are stubbed. The page itself
 * is not booted: these functions only use sessionStorage.
 */

import { jest } from '@jest/globals';
import { mockPageModules } from './_page-harness.js';

const DETAIL_DRAFT_KEY = 'kakeibon.detail_draft.v1';

mockPageModules(jest, { invoke: () => null });

const { persistDraft, consumeDraft, clearDraft } =
    await import('../../js/transaction-detail-management.js');

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

describe('transaction detail screen — detail draft storage', () => {
    beforeEach(() => {
        sessionStorage.clear();
        jest.spyOn(console, 'error').mockImplementation(() => {});
    });

    afterEach(() => {
        jest.restoreAllMocks();
    });

    test('should return the same payload when a draft is persisted and then consumed', () => {
        const draft = sampleDraft();
        persistDraft(draft);
        expect(consumeDraft()).toEqual(draft);
    });

    test('should return null from consume when nothing is stored', () => {
        expect(consumeDraft()).toBeNull();
    });

    test('should return null and clear storage when the stored draft is malformed JSON', () => {
        sessionStorage.setItem(DETAIL_DRAFT_KEY, '{not json');
        expect(consumeDraft()).toBeNull();
        expect(sessionStorage.getItem(DETAIL_DRAFT_KEY)).toBeNull();
    });

    test('should remove the persisted entry when clearDraft is called', () => {
        persistDraft(sampleDraft());
        clearDraft();
        expect(sessionStorage.getItem(DETAIL_DRAFT_KEY)).toBeNull();
        expect(consumeDraft()).toBeNull();
    });

    test('should overwrite the earlier draft when persist is called again', () => {
        persistDraft(sampleDraft({ item_name: 'first' }));
        persistDraft(sampleDraft({ item_name: 'second' }));
        expect(consumeDraft().item_name).toBe('second');
    });

    test('should keep detail_id and selected_product_id when an edit-mode draft makes a round trip', () => {
        persistDraft(sampleDraft({ detail_id: '42', selected_product_id: 7 }));
        const out = consumeDraft();
        expect(out.detail_id).toBe('42');
        expect(out.selected_product_id).toBe(7);
    });
});
