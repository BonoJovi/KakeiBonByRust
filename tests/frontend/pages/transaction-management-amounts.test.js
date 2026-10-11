/**
 * Transaction list and edit window (res/transaction-management.html,
 * handled by res/js/transaction-management.js) — how the total is shown
 * and read.
 *
 * The list shows each total as "¥" + the ja-JP grouping ("¥1,234,567").
 * The edit window's Total field (an <input type="number" min="0" step="1">)
 * shows the saved total as plain digits and sends it back unchanged.
 *
 * When Save is clicked, the field's own checks stop a total that is not a
 * whole number of 0 or more: "1,000" (the field empties it), "1099.5" and
 * "-5" are not sent. A total that passes those checks but is not plain
 * digits within the safe integer range ("1e3", "9007199254740993") is
 * refused by parseAmountStrict with common.error_amount_not_integer under
 * the field. parseAmountStrict itself is tested in
 * ../parse-amount-strict.test.js.
 *
 * These replace the last part of a former test file that tested copies of
 * these rules written inside the test. The real page module is booted
 * against res/transaction-management.html via ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const SAVED = {
    transaction_date: '2026-09-01 10:30:00',
    shop_id: null,
    category1_code: 'EXPENSE',
    from_account_code: 'CASH',
    to_account_code: 'NONE',
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: null,
    is_scheduled: 0,
};

const HEADERS = [
    { ...SAVED, transaction_id: 1, total_amount: 1234567 },
    { ...SAVED, transaction_id: 2, total_amount: 999 },
    { ...SAVED, transaction_id: 3, total_amount: 0 },
];

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
                    transactions: HEADERS.map((h) => ({ ...h, category1_name: 'Expense' })),
                    total_count: HEADERS.length,
                    page: args.page,
                    per_page: 50,
                    total_pages: 1,
                };
            case 'get_accounts':
                return [{ account_code: 'CASH', account_name: 'Cash', is_disabled: 0 }];
            case 'get_shops':
                return [];
            case 'get_transaction_header':
                return HEADERS.find((h) => h.transaction_id === args.transactionId);
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
const totalField = () => field('total-amount');
const totalError = () => totalField().parentElement.querySelector('.validation-error');
const listedAmounts = () => Array.from(document.querySelectorAll('#transaction-list .transaction-amount'))
    .map((el) => el.textContent);

async function closeWindow() {
    field('cancel-transaction-btn')?.click();
    await flush();
}

// Open the index-th transaction of the list.
async function openEdit(index) {
    await closeWindow();
    const editBtns = Array.from(document.querySelectorAll('#transaction-list .transaction-item button'))
        .filter((b) => b.getAttribute('data-i18n') === 'common.edit');
    editBtns[index].click();
    await flush(10);
    invoke.mockClear();
}

async function clickSave() {
    field('transaction-form').querySelector('button[type="submit"]').click();
    await flush(10);
}

const sentUpdates = () => callsOf(invoke, 'update_transaction_header');

describe('transaction list — totals', () => {
    test('should show the total with ¥ and commas when it is 1000 or more', () => {
        expect(listedAmounts()[0]).toBe('¥1,234,567');
    });

    test('should show the total with ¥ and no comma when it is below 1000', () => {
        expect(listedAmounts()[1]).toBe('¥999');
    });

    test('should show ¥0 when the total is 0', () => {
        expect(listedAmounts()[2]).toBe('¥0');
    });
});

describe('transaction edit window — Total field', () => {
    test('should show the saved total as plain digits when a header is opened', async () => {
        await openEdit(0);
        expect(totalField().value).toBe('1234567');
    });

    test('should send the saved total back when a header is saved without changes', async () => {
        await openEdit(0);
        await clickSave();
        expect(sentUpdates()).toHaveLength(1);
        expect(sentUpdates()[0].totalAmount).toBe(1234567);
    });

    test('should send the typed total as a number when it is plain digits', async () => {
        await openEdit(0);
        totalField().value = '1000';
        await clickSave();
        expect(sentUpdates()[0].totalAmount).toBe(1000);
    });

    test.each([
        ['has a comma', '1,000'],
        ['has a decimal point', '1099.5'],
        ['is negative', '-5'],
    ])('should not send the header when the total %s', async (_label, typed) => {
        await openEdit(0);
        totalField().value = typed;
        await clickSave();
        expect(sentUpdates()).toHaveLength(0);
        expect(field('transaction-modal').classList.contains('hidden')).toBe(false);
    });

    test.each([
        ['is in scientific notation', '1e3'],
        ['is past the safe integer range', '9007199254740993'],
    ])('should show the whole-number message under the total and not send when it %s', async (_label, typed) => {
        await openEdit(0);
        totalField().value = typed;
        await clickSave();
        expect(sentUpdates()).toHaveLength(0);
        expect(totalError().textContent).toBe('common.error_amount_not_integer');
    });

    test('should clear the whole-number message when the total is fixed and saved again', async () => {
        await openEdit(0);
        totalField().value = '1e3';
        await clickSave();
        totalField().value = '1000';
        await clickSave();
        expect(sentUpdates()).toHaveLength(1);
        expect(totalError()?.textContent ?? '').toBe('');
    });
});
