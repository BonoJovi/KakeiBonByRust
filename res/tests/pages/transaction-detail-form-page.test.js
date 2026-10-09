/**
 * Transaction detail screen (res/js/transaction-detail-management.js) — the
 * detail form and the header total.
 *
 * The detail window saves the medium and minor categories only when chosen
 * (null otherwise); clearing the medium category empties the minor category
 * choices. Amounts must be whole numbers of yen ("-100" is rejected on the
 * field before saving); 0 is saved. The tax rate is one of 0 / 8 / 10 %
 * (10 % preselected). The tax is rounded by the header's rounding type
 * (0 = floor, 1 = half-up, 2 = ceil; floor when the header has none), and a
 * tax rate change recalculates from the amount the user typed last. A memo
 * over 1000 characters (counted as code points) is rejected before saving;
 * a blank memo is saved as null. The header total is shown as ¥ with
 * thousands separators.
 *
 * These replace a former test file that tested values written inside the
 * test. The real page module is booted against
 * res/transaction-detail-management.html via ./_page-harness.js. Each test
 * boots the page again (page body reloaded, DOMContentLoaded fired again),
 * so it can use its own header.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const TRANSACTION_ID = 10;

function makeHeader(overrides = {}) {
    return {
        transaction_id: TRANSACTION_ID,
        transaction_date: '2026-09-01 10:00:00',
        category1_code: 'EXPENSE',
        from_account_code: 'CASH',
        from_account_name: 'Cash',
        shop_name: 'Shop',
        total_amount: 0,
        tax_rounding_type: 0,
        ...overrides,
    };
}

const CATEGORY_TREE = [
    {
        category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
        children: [
            {
                category2: { category2_code: 'C2_E_1', category2_name_i18n: 'Food' },
                children: [
                    { category3_code: 'C3_E_1_1', category3_name_i18n: 'Veg' },
                    { category3_code: 'C3_E_1_2', category3_name_i18n: 'Meat' },
                ],
            },
        ],
    },
];

let header = makeHeader();

const { invoke } = mockPageModules(jest, {
    user: { user_id: 2, name: 'alice', role: 1 },
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_transaction_header_with_info':
                return header;
            case 'get_transaction_details':
                return [];
            case 'get_category_tree_with_lang':
            case 'get_category_tree_all_with_lang':
                return CATEGORY_TREE;
            case 'search_products_by_name':
                return [];
            case 'compute_recommended_transaction_total':
                return header.total_amount; // equal -> no recalc prompt
            default:
                return null;
        }
    },
});

window.confirm = () => false;
window.history.replaceState(null, '', `/?transaction_id=${TRANSACTION_ID}`);
loadPageBody('transaction-detail-management.html');
await import('../../js/transaction-detail-management.js');
await bootPage();

async function bootWith(headerOverrides = {}) {
    header = makeHeader(headerOverrides);
    loadPageBody('transaction-detail-management.html');
    await bootPage();
    invoke.mockClear();
}

const $ = (id) => document.getElementById(id);
const fieldError = (id) => $(id).parentElement.querySelector('.validation-error')?.textContent ?? null;
const optionValues = (id) => [...$(id).options].map((o) => o.value);

async function openAddDetail() {
    $('add-detail-btn').click();
    await flush(10);
}

async function chooseCategory2(code) {
    $('category2-code').value = code;
    $('category2-code').dispatchEvent(new Event('change'));
    await flush(10);
}

function typeInto(id, value) {
    $(id).value = value;
    $(id).dispatchEvent(new Event('input', { bubbles: true }));
}

function changeTaxRate(rate) {
    $('tax-rate').value = rate;
    $('tax-rate').dispatchEvent(new Event('change', { bubbles: true }));
}

async function submitDetail() {
    $('detail-form').dispatchEvent(new Event('submit', { cancelable: true, bubbles: true }));
    await flush(10);
}

// Open the window and fill a valid detail: item name and 1000 yen at 10 %.
async function openFilledDetail() {
    await openAddDetail();
    $('item-name').value = 'Milk';
    typeInto('amount-excluding-tax', '1000');
}

const adds = () => callsOf(invoke, 'add_transaction_detail');

describe('transaction detail screen — detail form', () => {
    beforeEach(async () => {
        await bootWith();
    });

    describe('categories', () => {
        test('should save no medium or minor category when neither is chosen', async () => {
            await openFilledDetail();
            await submitDetail();

            expect(adds()).toHaveLength(1);
            expect(adds()[0]).toMatchObject({ category1Code: 'EXPENSE', category2Code: null, category3Code: null });
        });

        test('should save no minor category when only the medium category is chosen', async () => {
            await openFilledDetail();
            await chooseCategory2('C2_E_1');
            await submitDetail();

            expect(adds()).toHaveLength(1);
            expect(adds()[0]).toMatchObject({ category2Code: 'C2_E_1', category3Code: null });
        });

        test('should clear the minor category choices when the medium category is cleared', async () => {
            await openAddDetail();
            await chooseCategory2('C2_E_1');
            expect(optionValues('category3-code')).toEqual(['', 'C3_E_1_1', 'C3_E_1_2']);

            await chooseCategory2('');

            expect(optionValues('category3-code')).toEqual(['']);
        });
    });

    describe('amounts and tax rate', () => {
        test('should reject the amount on its field before saving when the tax-excluded amount is negative', async () => {
            await openFilledDetail();
            $('amount-excluding-tax').value = '-100';
            await submitDetail();

            expect(adds()).toHaveLength(0);
            expect(fieldError('amount-excluding-tax')).toBe('common.error_amount_not_integer');
        });

        test('should save 0 yen when the amounts are 0', async () => {
            await openAddDetail();
            $('item-name').value = 'Free sample';
            typeInto('amount-excluding-tax', '0');
            $('amount-including-tax').value = '0';
            $('tax-amount').value = '0';
            await submitDetail();

            expect(adds()).toHaveLength(1);
            expect(adds()[0]).toMatchObject({ amount: 0, amountIncludingTax: 0, taxAmount: 0 });
        });

        test('should offer only 0, 8 and 10 percent with 10 selected when the detail window opens', async () => {
            await openAddDetail();

            expect(optionValues('tax-rate')).toEqual(['0', '8', '10']);
            expect($('tax-rate').value).toBe('10');
        });

        test('should send the tax rate as a number when the detail is saved', async () => {
            await openFilledDetail();
            changeTaxRate('8');
            await submitDetail();

            expect(adds()[0]).toMatchObject({ taxRate: 8, amount: 1000, taxAmount: 80, amountIncludingTax: 1080 });
        });
    });

    describe('memo', () => {
        test('should reject the memo before saving when it is longer than 1000 characters', async () => {
            await openFilledDetail();
            // Set the value directly: the character counter would cut it on input.
            $('memo').value = 'a'.repeat(1001);
            await submitDetail();

            expect(adds()).toHaveLength(0);
            expect(fieldError('memo')).toBe('validation.max_length(field=detail_mgmt.memo,max=1000,actual=1001)');
        });

        test('should save the memo when it has 1000 characters outside the BMP', async () => {
            const memo = '😀'.repeat(1000); // 2000 UTF-16 units, 1000 code points
            await openFilledDetail();
            $('memo').value = memo;
            await submitDetail();

            expect(adds()).toHaveLength(1);
            expect(adds()[0].memo).toBe(memo);
        });

        test.each([
            ['empty', ''],
            ['only spaces', '   '],
        ])('should save no memo when the memo is %s', async (_label, value) => {
            await openFilledDetail();
            $('memo').value = value;
            await submitDetail();

            expect(adds()).toHaveLength(1);
            expect(adds()[0].memo).toBeNull();
        });
    });

    describe('tax rate change', () => {
        test('should recompute the tax and the tax-included amount when the rate changes after the tax-excluded amount was typed', async () => {
            await openAddDetail();
            typeInto('amount-excluding-tax', '1000');
            expect($('amount-including-tax').value).toBe('1100');

            changeTaxRate('8');

            expect($('amount-excluding-tax').value).toBe('1000');
            expect($('tax-amount').value).toBe('80');
            expect($('amount-including-tax').value).toBe('1080');
        });

        test('should keep the typed tax-included amount when the rate changes after it was typed', async () => {
            await openAddDetail();
            typeInto('amount-including-tax', '1100');
            expect($('amount-excluding-tax').value).toBe('1000');

            changeTaxRate('8');

            // 1100 at 8 % under floor: 1019 + floor(81.52) = 1100.
            expect($('amount-including-tax').value).toBe('1100');
            expect($('amount-excluding-tax').value).toBe('1019');
            expect($('tax-amount').value).toBe('81');
        });
    });
});

describe('transaction detail screen — header rounding type and total', () => {
    test.each([
        { mode: 0, excluded: '105', tax: '10', included: '115' },
        { mode: 1, excluded: '105', tax: '11', included: '116' },
        { mode: 1, excluded: '104', tax: '10', included: '114' },
        { mode: 2, excluded: '104', tax: '11', included: '115' },
    ])('should add $tax yen of tax to $excluded yen at 10% when the header rounding type is $mode', async ({ mode, excluded, tax, included }) => {
        await bootWith({ tax_rounding_type: mode });
        await openAddDetail();
        typeInto('amount-excluding-tax', excluded);

        expect($('tax-amount').value).toBe(tax);
        expect($('amount-including-tax').value).toBe(included);
    });

    test('should round the tax down when the header has no rounding type', async () => {
        await bootWith({ tax_rounding_type: undefined });
        await openAddDetail();
        typeInto('amount-excluding-tax', '105');

        expect($('tax-amount').value).toBe('10');
        expect($('amount-including-tax').value).toBe('115');
    });

    test.each([
        { total: 1234567, shown: '¥1,234,567' },
        { total: 0, shown: '¥0' },
    ])('should show the header total as $shown when the total is $total', async ({ total, shown }) => {
        await bootWith({ total_amount: total });

        expect($('header-total-amount').textContent).toBe(shown);
    });
});
