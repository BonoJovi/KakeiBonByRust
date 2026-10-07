/**
 * R5  recurring-rule.js: the rounding / tax-type `change` handlers only run
 *     updateDerivedTotal(); the detail's tax-amount / amount-including-tax
 *     fields keep the values computed under the old rounding. 外税, floor,
 *     amount 105 @ 10 % → tax 10 / tax-included 115. Switching rounding to
 *     ceil makes the total 116, but the detail stays 10 / 115, and every
 *     generated occurrence stores that detail under a 116 header.
 *     Expected: after the rounding change the detail fields are 11 / 116,
 *     and the request sent to create_recurring_rule carries them (matching
 *     the derived total). With 内税 the typed tax-included price stays and
 *     the tax-excluded amount follows the new rounding (116: floor 106 / 10,
 *     ceil 105 / 11).
 *
 * Real page module (recurring-rule.js + detail-tax-calc.js) booted against
 * res/recurring-rule.html.
 */
import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const CATEGORY_TREE = [
    {
        category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
        children: [],
    },
];

const year = new Date().getFullYear();

const { invoke } = mockPageModules(jest, {
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return CATEGORY_TREE;
            case 'get_accounts':
                return [{ account_code: 'BANK', account_name: 'Bank' }];
            case 'get_shops':
                return [];
            case 'list_recurring_rules':
                return [];
            case 'get_recurring_period_limits':
                return { first: `${year - 5}-01-01`, last: `${year + 10}-12-31` };
            case 'create_recurring_rule':
                return { rule_id: 1, generated_count: 12 };
            default:
                return null;
        }
    },
});

loadPageBody('recurring-rule.html');
await import('../../js/recurring-rule.js');
await bootPage();

function setInput(id, value) {
    const el = document.getElementById(id);
    el.value = value;
    el.dispatchEvent(new Event('input'));
}

function setSelect(id, value) {
    const el = document.getElementById(id);
    el.value = value;
    el.dispatchEvent(new Event('change'));
}

async function submitAndGetDetail() {
    invoke.mockClear();
    document.getElementById('recurring-rule-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
    const creates = callsOf(invoke, 'create_recurring_rule');
    expect(creates).toHaveLength(1);
    const { detail } = creates[0].request;
    return {
        amount: detail.amount,
        tax_amount: detail.tax_amount,
        amount_including_tax: detail.amount_including_tax,
    };
}

describe('recurring rule form: rounding change after the amount (scan2-R5)', () => {
    test('[scan2-R5] recomputes the detail tax fields when the rounding changes (tax excluded)', async () => {
        document.getElementById('rule-name').value = 'Sub';
        setSelect('category1', 'EXPENSE');
        await flush();
        document.getElementById('item-name').value = 'Sub';
        setSelect('tax-included-type', '1'); // tax-excluded
        setSelect('tax-rounding-type', '0'); // floor
        setInput('tax-rate', '10');
        document.getElementById('tax-rate').dispatchEvent(new Event('change'));
        setInput('amount-excluding-tax', '105');
        await flush();
        expect(document.getElementById('tax-amount').value).toBe('10');
        expect(document.getElementById('amount-including-tax').value).toBe('115');

        setSelect('tax-rounding-type', '2'); // ceil
        await flush();
        expect(document.getElementById('total-amount').value).toBe('116');

        expect(await submitAndGetDetail())
            .toEqual({ amount: 105, tax_amount: 11, amount_including_tax: 116 });
    });

    test('[scan2-R5] keeps the typed tax-included price and recomputes the rest (tax included)', async () => {
        document.getElementById('rule-name').value = 'Sub';
        setSelect('category1', 'EXPENSE');
        await flush();
        document.getElementById('item-name').value = 'Sub';
        setSelect('tax-included-type', '0'); // tax-included
        setSelect('tax-rounding-type', '0'); // floor
        setInput('tax-rate', '10');
        document.getElementById('tax-rate').dispatchEvent(new Event('change'));
        setInput('amount-excluding-tax', '');
        setInput('amount-including-tax', '116');
        await flush();
        expect(document.getElementById('amount-excluding-tax').value).toBe('106');
        expect(document.getElementById('tax-amount').value).toBe('10');

        setSelect('tax-rounding-type', '2'); // ceil
        await flush();
        expect(document.getElementById('amount-including-tax').value).toBe('116');
        expect(document.getElementById('amount-excluding-tax').value).toBe('105');
        expect(document.getElementById('tax-amount').value).toBe('11');
        expect(document.getElementById('total-amount').value).toBe('116');

        expect(await submitAndGetDetail())
            .toEqual({ amount: 105, tax_amount: 11, amount_including_tax: 116 });
    });
});
