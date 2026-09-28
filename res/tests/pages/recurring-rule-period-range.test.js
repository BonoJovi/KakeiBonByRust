/**
 * Recurring rule screen (res/js/recurring-rule.js) — regression tests for
 * latent-audit M15 / M18.
 *
 * M15 / M18 A rule could span any years: beyond the seeded holiday years
 *     its holiday shift silently did nothing, and a year typo (e.g. 9999)
 *     generated millions of occurrences and froze the app. A rule may now
 *     only span the seeded years. The screen asks the backend for those
 *     bounds (get_recurring_period_limits) — they can lag the calendar when
 *     the app runs across New Year. Pinned: the date pickers carry the
 *     backend's bounds, an out-of-range period is stopped before
 *     create_recurring_rule with recurring_rule.period_out_of_range, and a
 *     backend recurring_period_out_of_range rejection shows the same message.
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

let BACKEND_LIMITS = {
    first: `${new Date().getFullYear() - 4}-01-01`,
    last: `${new Date().getFullYear() + 9}-12-31`,
};

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
            case 'get_recurring_period_limits':
                return BACKEND_LIMITS;
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
await import('../../js/recurring-rule.js');
await bootPage();

const year = new Date().getFullYear();
// Deliberately narrower than the local "this year − 5 .. + 10" fallback, so
// the tests prove the backend's bounds are the ones used.
const MIN = `${year - 4}-01-01`;
const MAX = `${year + 9}-12-31`;
const OUT_OF_RANGE_MESSAGE = `recurring_rule.period_out_of_range(start=${MIN},end=${MAX})`;

async function fillExpenseForm() {
    document.getElementById('rule-name').value = 'Rent';
    const category1 = document.getElementById('category1');
    category1.value = 'EXPENSE';
    category1.dispatchEvent(new Event('change'));
    await flush();
    document.getElementById('item-name').value = 'Rent';
    document.getElementById('total-amount').value = '80000';
    document.getElementById('tax-rate').value = '0';
    document.getElementById('amount-excluding-tax').value = '80000';
    document.getElementById('amount-including-tax').value = '80000';
    document.getElementById('tax-amount').value = '0';
}

async function submitForm() {
    document.getElementById('recurring-rule-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
}

describe('recurring rule form — period range (regression, latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('[M15/M18] should bound the date pickers to the seeded holiday years', () => {
        for (const id of ['start-date', 'end-date']) {
            const input = document.getElementById(id);
            expect(input.min).toBe(MIN);
            expect(input.max).toBe(MAX);
        }
    });

    test('[M15/M18] should stop an end date past the limit before create_recurring_rule', async () => {
        await fillExpenseForm();
        document.getElementById('start-date').value = `${year}-01-01`;
        document.getElementById('end-date').value = `${year + 10}-01-01`; // inside the local fallback, outside the backend bound

        await submitForm();

        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(0);
        const box = document.getElementById('result-box');
        expect(box.classList.contains('error')).toBe(true);
        expect(box.textContent).toBe(OUT_OF_RANGE_MESSAGE);
    });

    test('[M15/M18] should stop a start date before the limit before create_recurring_rule', async () => {
        await fillExpenseForm();
        document.getElementById('start-date').value = `${year - 5}-06-01`; // inside the local fallback, outside the backend bound
        document.getElementById('end-date').value = `${year}-12-31`;

        await submitForm();

        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(0);
        expect(document.getElementById('result-box').textContent).toBe(OUT_OF_RANGE_MESSAGE);
    });

    test('[M15/M18] should move the date pickers to bounds that changed since the page loaded', async () => {
        const initial = BACKEND_LIMITS;
        // e.g. the app kept running across New Year, or the first lookup fell back
        BACKEND_LIMITS = { first: `${year - 3}-01-01`, last: `${year + 8}-12-31` };
        try {
            await fillExpenseForm();
            document.getElementById('start-date').value = `${year}-01-01`;
            document.getElementById('end-date').value = `${year + 9}-06-01`;

            await submitForm();

            expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(0);
            for (const id of ['start-date', 'end-date']) {
                expect(document.getElementById(id).min).toBe(BACKEND_LIMITS.first);
                expect(document.getElementById(id).max).toBe(BACKEND_LIMITS.last);
            }
        } finally {
            BACKEND_LIMITS = initial;
            document.getElementById('end-date').value = `${year}-12-31`;
            await submitForm(); // puts the pickers back on the initial bounds
            invoke.mockClear();
        }
    });

    test('[M15/M18] should show the same message for a backend recurring_period_out_of_range rejection', async () => {
        await fillExpenseForm();
        document.getElementById('start-date').value = MIN;
        document.getElementById('end-date').value = MAX;
        createRejection = {
            code: 'recurring_period_out_of_range',
            message: `The rule period must be between ${MIN} and ${MAX}`,
        };

        await submitForm();

        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(1);
        const box = document.getElementById('result-box');
        expect(box.classList.contains('error')).toBe(true);
        expect(box.textContent).toBe(OUT_OF_RANGE_MESSAGE);
    });
});
