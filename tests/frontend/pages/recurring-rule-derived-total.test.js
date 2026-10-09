/**
 * Recurring rule screen (res/js/recurring-rule.js) — regression tests for
 * latent-audit M17.
 *
 * M17 The rule's total was typed separately, defaulted to 0 and was never
 *     checked against the detail, so 0-yen occurrences could be generated.
 *     The total is now derived from the (single) detail — the Rust side
 *     computes and stores it. Pinned: the total field is read-only, follows
 *     the detail and the header's tax settings, and is not sent.
 *
 * The real page module is booted against res/recurring-rule.html via
 * ./_page-harness.js.
 */
import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const CATEGORY_TREE = [
    {
        category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
        children: [
            {
                category2: { category2_code: 'C2_E_1', category2_name_i18n: 'Food' },
                children: [{ category3_code: 'C3_1', category3_name_i18n: 'Veg' }],
            },
        ],
    },
    {
        category1: { category1_code: 'TRANSFER', category1_name_i18n: 'Transfer' },
        children: [],
    },
];

// When set, create_recurring_rule rejects with this error once.
let createRejection = null;

const { invoke } = mockPageModules(jest, {
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return CATEGORY_TREE;
            case 'get_accounts':
                return [
                    { account_code: 'CASH', account_name: 'Cash' },
                    { account_code: 'BANK', account_name: 'Bank' },
                ];
            case 'get_shops':
                return [];
            case 'list_recurring_rules':
                return [];
            case 'create_recurring_rule':
                if (createRejection) {
                    const err = createRejection;
                    createRejection = null;
                    return Promise.reject(err);
                }
                return { rule_id: 1, generated_count: 12 };
            default:
                return null;
        }
    },
});

loadPageBody('recurring-rule.html');
await import('../../../res/js/recurring-rule.js');
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

describe('recurring rule form — derived total (regression, latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('should show a read-only total that follows the detail and tax settings when they change (M17)', async () => {
        const total = document.getElementById('total-amount');
        expect(total.readOnly).toBe(true);

        setSelect('tax-included-type', '1'); // tax-excluded
        setSelect('tax-rounding-type', '0'); // floor
        setInput('tax-rate', '8');
        setInput('amount-excluding-tax', '1005');
        await flush();
        expect(total.value).toBe('1085'); // floor(1005 × 1.08 = 1085.4)

        setSelect('tax-rounding-type', '2'); // ceil
        await flush();
        expect(total.value).toBe('1086');
    });

    test('should not send a typed total to create_recurring_rule when the rule is saved (M17)', async () => {
        document.getElementById('rule-name').value = 'Rent';
        setSelect('category1', 'EXPENSE');
        await flush();
        document.getElementById('item-name').value = 'Rent';
        setSelect('tax-included-type', '1');
        setInput('tax-rate', '0');
        setInput('amount-excluding-tax', '80000');
        await flush();

        document.getElementById('recurring-rule-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        const creates = callsOf(invoke, 'create_recurring_rule');
        expect(creates).toHaveLength(1);
        expect(creates[0].request).not.toHaveProperty('total_amount');
        expect(creates[0].request.detail.amount).toBe(80000);
    });
});
