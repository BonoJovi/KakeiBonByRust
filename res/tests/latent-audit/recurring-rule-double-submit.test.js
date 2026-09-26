/**
 * Latent audit 2026-09 — recurring rule form (res/js/recurring-rule.js)
 *
 * IDs covered: M19 (recurring-rule half)
 *
 * Bug: the #recurring-rule-form submit handler has no re-entrancy guard and
 *      never disables the submit button, so a double click on "Create rule"
 *      (or Enter pressed twice) while create_recurring_rule is still in
 *      flight invokes the command twice → the rule AND all of its generated
 *      scheduled transactions are registered twice.
 * Expected: while the first create_recurring_rule call is pending, further
 *      submits are ignored — the command is invoked exactly once.
 *
 * The real page module is booted against res/recurring-rule.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, deferred, callsOf,
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
];

let createInflight = null;

const { invoke } = mockPageModules(jest, {
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return CATEGORY_TREE;
            case 'get_accounts':
                return [{ account_code: 'CASH', account_name: 'Cash' }];
            case 'get_shops':
                return [];
            case 'list_recurring_rules':
                return [];
            case 'create_recurring_rule':
                return createInflight ? createInflight.promise : { rule_id: 1, generated_count: 12 };
            default:
                return null;
        }
    },
});

loadPageBody('recurring-rule.html');
await import('../../js/recurring-rule.js');
await bootPage();

function fillValidForm() {
    document.getElementById('rule-name').value = 'Rent';
    document.getElementById('category1').value = 'EXPENSE';
    document.getElementById('category1').dispatchEvent(new Event('change'));
    document.getElementById('from-account').value = 'CASH';
    document.getElementById('item-name').value = 'Rent';
    document.getElementById('total-amount').value = '1100';
    document.getElementById('tax-rate').value = '10';
    document.getElementById('amount-excluding-tax').value = '1000';
    document.getElementById('amount-including-tax').value = '1100';
    document.getElementById('tax-amount').value = '100';
}

function submitForm() {
    document.getElementById('recurring-rule-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
}

describe('recurring rule form — latent audit 2026-09', () => {
    test('[latent M19] double submit invokes create_recurring_rule only once', async () => {
        fillValidForm();
        await flush();

        createInflight = deferred();
        submitForm();
        submitForm();
        await flush(5);

        const creates = callsOf(invoke, 'create_recurring_rule');

        createInflight.resolve({ rule_id: 1, generated_count: 12 });
        createInflight = null;
        await flush(10);

        // Sanity: the first submit passed client-side validation.
        expect(creates.length).toBeGreaterThanOrEqual(1);
        expect(creates).toHaveLength(1);
    });
});
