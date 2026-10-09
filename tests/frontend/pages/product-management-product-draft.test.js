/**
 * Product master screen (res/js/product-management.js) — product draft for
 * the product → manufacturer master side trip.
 *
 * "Open in manufacturer master" in the product window saves the window's
 * inputs to sessionStorage (the product draft) together with the transaction
 * the user came from (return_to_transaction_id). Coming back with
 * ?restore_product=1, the page reopens the window with the draft's values,
 * re-wires "Back to detail entry" from return_to_transaction_id, and removes
 * the draft. A malformed draft is discarded; no draft opens no window.
 *
 * These replace part of a former test file that tested a copy of this
 * logic. The real page module is booted against res/product-management.html
 * via ./_page-harness.js. Each test boots the page again at its own URL: the
 * page body is reloaded (dropping the old element listeners) and
 * DOMContentLoaded is fired again, as when the user navigates.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, isHiddenOrAbsent,
} from './_page-harness.js';

const PRODUCT_DRAFT_KEY = 'kakeibon.product_draft.v1';

const MANUFACTURERS = [
    { manufacturer_id: 3, manufacturer_name: 'OtherCo', is_disabled: 0 },
    { manufacturer_id: 7, manufacturer_name: 'MakerA', is_disabled: 0 },
];

mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_manufacturers':
                return MANUFACTURERS;
            case 'get_products':
                return [];
            default:
                return null;
        }
    },
});

window.history.replaceState(null, '', '/');
loadPageBody('product-management.html');
await import('../../../res/js/product-management.js');
await bootPage();

async function bootAt(url) {
    window.history.replaceState(null, '', url);
    loadPageBody('product-management.html');
    await bootPage();
}

const productModalOpen = () =>
    !document.getElementById('product-modal').classList.contains('hidden');

async function fillProductWindowAndJump({ name, manufacturerId, memo, disabled }) {
    document.getElementById('add-product-btn').click();
    await flush(5);
    document.getElementById('product-name').value = name;
    document.getElementById('product-manufacturer').value = manufacturerId;
    document.getElementById('product-memo').value = memo;
    document.getElementById('product-is-disabled').checked = disabled;
    document.getElementById('open-manufacturer-master-btn').click(); // jsdom ignores the navigation
    await flush(5);
}

function storedDraft() {
    return JSON.parse(sessionStorage.getItem(PRODUCT_DRAFT_KEY));
}

describe('product master screen — product draft for the manufacturer side trip', () => {
    beforeEach(() => {
        sessionStorage.clear();
        jest.spyOn(console, 'error').mockImplementation(() => {});
    });

    afterEach(() => {
        jest.restoreAllMocks();
    });

    describe('leaving for the manufacturer master', () => {
        test('should save the window inputs and the transaction to return to when the user came from a detail', async () => {
            await bootAt('/?return_to=17');
            await fillProductWindowAndJump({
                name: 'テスト商品X', manufacturerId: '7', memo: 'メモ', disabled: true,
            });

            expect(storedDraft()).toEqual({
                editing_product_id: null,
                product_name: 'テスト商品X',
                manufacturer_id: '7',
                memo: 'メモ',
                is_disabled: true,
                return_to_transaction_id: '17',
            });
        });

        test('should save a null transaction to return to when the user came from the menu', async () => {
            await bootAt('/');
            await fillProductWindowAndJump({
                name: 'テスト商品X', manufacturerId: '', memo: '', disabled: false,
            });

            expect(storedDraft().return_to_transaction_id).toBeNull();
            expect(storedDraft().manufacturer_id).toBe('');
        });

        test('should overwrite the earlier draft when the user leaves for the manufacturer master again', async () => {
            await bootAt('/');
            await fillProductWindowAndJump({ name: 'first', manufacturerId: '', memo: '', disabled: false });
            await fillProductWindowAndJump({ name: 'second', manufacturerId: '3', memo: '', disabled: false });

            expect(storedDraft().product_name).toBe('second');
            expect(storedDraft().manufacturer_id).toBe('3');
        });
    });

    describe('coming back with ?restore_product=1', () => {
        test('should restore the window inputs and remove the draft when a draft is stored', async () => {
            sessionStorage.setItem(PRODUCT_DRAFT_KEY, JSON.stringify({
                editing_product_id: null,
                product_name: '保存テスト',
                manufacturer_id: '7',
                memo: 'メモ',
                is_disabled: true,
                return_to_transaction_id: '42',
            }));

            await bootAt('/?restore_product=1');

            expect(productModalOpen()).toBe(true);
            expect(document.getElementById('product-name').value).toBe('保存テスト');
            expect(document.getElementById('product-manufacturer').value).toBe('7');
            expect(document.getElementById('product-memo').value).toBe('メモ');
            expect(document.getElementById('product-is-disabled').checked).toBe(true);
            expect(sessionStorage.getItem(PRODUCT_DRAFT_KEY)).toBeNull();
            // return_to_transaction_id re-wires "Back to detail entry".
            expect(isHiddenOrAbsent(document.getElementById('back-to-detail-btn'))).toBe(false);
        });

        test('should not offer "Back to detail entry" when the restored draft has no transaction to return to', async () => {
            sessionStorage.setItem(PRODUCT_DRAFT_KEY, JSON.stringify({
                editing_product_id: null,
                product_name: 'from menu',
                manufacturer_id: '',
                memo: '',
                is_disabled: false,
                return_to_transaction_id: null,
            }));

            await bootAt('/?restore_product=1');

            expect(productModalOpen()).toBe(true);
            expect(document.getElementById('product-name').value).toBe('from menu');
            expect(isHiddenOrAbsent(document.getElementById('back-to-detail-btn'))).toBe(true);
        });

        test('should discard the draft and open no window when the stored draft is malformed JSON', async () => {
            sessionStorage.setItem(PRODUCT_DRAFT_KEY, 'not json{');

            await bootAt('/?restore_product=1');

            expect(sessionStorage.getItem(PRODUCT_DRAFT_KEY)).toBeNull();
            expect(productModalOpen()).toBe(false);
        });

        test('should open no window when no draft is stored', async () => {
            await bootAt('/?restore_product=1');

            expect(productModalOpen()).toBe(false);
        });
    });
});
