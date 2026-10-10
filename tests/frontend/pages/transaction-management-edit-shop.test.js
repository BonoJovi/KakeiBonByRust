/**
 * Transaction edit window (res/transaction-management.html, handled by
 * res/js/transaction-management.js) — the Shop field.
 *
 * The Shop select lists "Unspecified" (value '') first, then the enabled
 * shops from get_shops in the order given. Opening a saved header selects
 * its shop_id; a null shop_id, or one that is not in the list, shows
 * "Unspecified". Saving sends the chosen shop as an integer `shopId`, or
 * null for "Unspecified", to update_transaction_header, or to
 * save_transaction_header for a new header.
 *
 * Disabled shops are tested in ./transaction-management-disabled-shop.test.js.
 *
 * These replace part of a former test file that tested copies of these
 * rules written inside the test. The real page module is booted against
 * res/transaction-management.html via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const SAVED = {
    transaction_id: 1,
    transaction_date: '2026-09-01 10:30:00',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    to_account_code: 'NONE',
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: null,
    is_scheduled: 0,
};

const SHOPS = [
    { shop_id: 1, shop_name: 'Shop A', is_disabled: 0 },
    { shop_id: 2, shop_name: 'Shop B', is_disabled: 0 },
    { shop_id: 7, shop_name: 'Shop G', is_disabled: 0 },
];

// What get_transaction_header answers for the header being edited.
let header = SAVED;

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [
                    { category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' }, children: [] },
                ];
            case 'get_transactions':
                return {
                    transactions: [{ ...SAVED, category1_name: 'Expense' }],
                    total_count: 1,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [{ account_code: 'CASH', account_name: 'Cash', is_disabled: 0 }];
            case 'get_shops':
                return SHOPS;
            case 'get_transaction_header':
                return header;
            case 'get_transaction_details':
                return [];
            default:
                return null;
        }
    },
});

window.alert = () => {};

loadPageBody('transaction-management.html');
await import('../../../res/js/transaction-management.js');
await bootPage();

const field = (id) => document.getElementById(id);
const shopSelect = () => field('shop');
const selectedShopText = () => shopSelect().selectedOptions[0].textContent;

async function closeWindow() {
    field('cancel-transaction-btn')?.click();
    await flush();
}

// Open the saved header from the list, answering get_transaction_header
// with `saved`.
async function openEdit(saved) {
    header = { ...SAVED, ...saved };
    await closeWindow();
    const editBtn = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .find((b) => b.getAttribute('data-i18n') === 'common.edit');
    editBtn.click();
    await flush(10);
    invoke.mockClear();
}

async function openAdd() {
    await closeWindow();
    field('add-transaction-btn').click();
    await flush(10);
    invoke.mockClear();
}

function chooseShop(value) {
    shopSelect().value = value;
    shopSelect().dispatchEvent(new Event('change'));
}

async function save() {
    field('transaction-form').dispatchEvent(new Event('submit', { cancelable: true, bubbles: true }));
    await flush(10);
}

const sentUpdate = () => callsOf(invoke, 'update_transaction_header')[0];
const sentNew = () => callsOf(invoke, 'save_transaction_header')[0];

describe('transaction edit window — Shop list', () => {
    test('should list Unspecified first and then the shops when the window opens', async () => {
        await openAdd();
        const options = Array.from(shopSelect().options).map((o) => [o.value, o.textContent]);
        expect(options).toEqual([
            ['', 'common.unspecified'],
            ['1', 'Shop A'],
            ['2', 'Shop B'],
            ['7', 'Shop G'],
        ]);
    });

    test('should select Unspecified when a new header is opened', async () => {
        await openAdd();
        expect(shopSelect().value).toBe('');
        expect(selectedShopText()).toBe('common.unspecified');
    });
});

describe('transaction edit window — loading the shop of a saved header', () => {
    test('should select the saved shop when the header has a shop', async () => {
        await openEdit({ shop_id: 2 });
        expect(shopSelect().value).toBe('2');
        expect(selectedShopText()).toBe('Shop B');
    });

    test('should select Unspecified when the saved header has no shop', async () => {
        await openEdit({ shop_id: null });
        expect(shopSelect().value).toBe('');
        expect(selectedShopText()).toBe('common.unspecified');
    });

    test('should select Unspecified when the saved shop is not in the list', async () => {
        await openEdit({ shop_id: 999 });
        expect(shopSelect().value).toBe('');
        expect(selectedShopText()).toBe('common.unspecified');
    });
});

describe('transaction edit window — saving the shop', () => {
    test('should send the saved shop back when a header is saved without changes', async () => {
        await openEdit({ shop_id: 7 });
        await save();
        expect(sentUpdate().shopId).toBe(7);
    });

    test('should send the shop as an integer when a shop is chosen', async () => {
        await openEdit({ shop_id: null });
        chooseShop('7');
        await save();
        expect(sentUpdate().shopId).toBe(7);
    });

    test('should send the last chosen shop when the shop is changed twice', async () => {
        await openEdit({ shop_id: null });
        chooseShop('1');
        chooseShop('2');
        await save();
        expect(sentUpdate().shopId).toBe(2);
    });

    test('should send null when the shop is changed to Unspecified', async () => {
        await openEdit({ shop_id: 1 });
        chooseShop('');
        await save();
        expect(sentUpdate().shopId).toBeNull();
    });

    test('should send null when a new header is saved with no shop', async () => {
        await openAdd();
        field('transaction-date').value = '2026-10-01T09:00';
        field('category1').value = 'EXPENSE';
        field('category1').dispatchEvent(new Event('change'));
        field('from-account').value = 'CASH';
        field('total-amount').value = '1200';
        await save();
        expect(sentNew().shopId).toBeNull();
    });

    test('should send the shop as an integer when a new header is saved with a shop', async () => {
        await openAdd();
        field('transaction-date').value = '2026-10-01T09:00';
        field('category1').value = 'EXPENSE';
        field('category1').dispatchEvent(new Event('change'));
        field('from-account').value = 'CASH';
        field('total-amount').value = '1200';
        chooseShop('1');
        await save();
        expect(sentNew().shopId).toBe(1);
    });
});
