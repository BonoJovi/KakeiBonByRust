// The recurring rule form checks the date order itself (latent-audit scan2-R8)
/**
 * Recurring rule screen (res/js/recurring-rule.js).
 *
 * scan2-R8  The form had no start <= end check; the backend rejected the
 *           request with the hard-coded `start_date must be on or before
 *           end_date` (code 'validation'), and the generic fallback showed
 *           it after the localized prefix — English text on the JA screen.
 *           The same fallback showed `anchor_date must be on or before
 *           end_date` (daily anchor after the end date) and
 *           `Invalid end_date: ` (end date cleared).
 *
 * Expected: the form stops each case itself with a localized message and
 * create_recurring_rule is not called.
 *
 * Real page module (recurring-rule.js) booted against res/recurring-rule.html.
 * The create_recurring_rule mock rejects exactly like the Rust command does.
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
    invoke: (cmd, args) => {
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
            case 'create_recurring_rule': {
                const r = args.request;
                if (!r.start_date || !r.end_date) {
                    return Promise.reject({ code: 'validation', message: `Invalid end_date: ${r.end_date}` });
                }
                if (r.start_date > r.end_date) {
                    return Promise.reject({ code: 'validation', message: 'start_date must be on or before end_date' });
                }
                if (r.anchor_date && r.anchor_date > r.end_date) {
                    return Promise.reject({ code: 'validation', message: 'anchor_date must be on or before end_date' });
                }
                return { rule_id: 1, generated_count: 1 };
            }
            default:
                return null;
        }
    },
});

loadPageBody('recurring-rule.html');
await import('../../js/recurring-rule.js');
await bootPage();

async function submitWithDates({ start, end, anchor }) {
    invoke.mockClear();
    document.getElementById('rule-name').value = 'Rent';
    const category1 = document.getElementById('category1');
    category1.value = 'EXPENSE';
    category1.dispatchEvent(new Event('change'));
    await flush();
    document.getElementById('item-name').value = 'Rent';
    document.getElementById('tax-rate').value = '0';
    document.getElementById('amount-excluding-tax').value = '80000';
    document.getElementById('amount-including-tax').value = '80000';
    document.getElementById('tax-amount').value = '0';
    document.getElementById('start-date').value = start;
    document.getElementById('end-date').value = end;
    document.getElementById('anchor-date').value = anchor;

    document.getElementById('recurring-rule-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
    return document.getElementById('result-box');
}

describe('scan2-R8 recurring rule form — date checks', () => {
    test('should show a localized message, not the backend English text, when the start date is after the end date', async () => {
        const box = await submitWithDates({
            start: `${year}-06-02`, end: `${year}-06-01`, anchor: `${year}-06-02`,
        });

        expect(box.classList.contains('error')).toBe(true);
        expect(box.textContent).toBe('recurring_rule.err_start_after_end');
        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(0);
    });

    test('should show a localized message when a daily anchor is after the end date', async () => {
        const box = await submitWithDates({
            start: `${year}-06-01`, end: `${year}-06-30`, anchor: `${year}-07-01`,
        });

        expect(box.classList.contains('error')).toBe(true);
        expect(box.textContent).toBe('recurring_rule.err_anchor_after_end');
        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(0);
    });

    test('should show a localized message when the end date is empty', async () => {
        const box = await submitWithDates({
            start: `${year}-06-01`, end: '', anchor: `${year}-06-01`,
        });

        expect(box.classList.contains('error')).toBe(true);
        expect(box.textContent).toBe('recurring_rule.err_dates_required');
        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(0);
    });

    test('should show the same message when the start date is empty', async () => {
        const box = await submitWithDates({
            start: '', end: `${year}-06-30`, anchor: `${year}-06-01`,
        });

        expect(box.classList.contains('error')).toBe(true);
        expect(box.textContent).toBe('recurring_rule.err_dates_required');
        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(0);
    });

    test('should still create a rule when the dates are in order', async () => {
        const box = await submitWithDates({
            start: `${year}-06-01`, end: `${year}-06-30`, anchor: `${year}-06-30`,
        });

        expect(box.classList.contains('success')).toBe(true);
        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(1);
    });
});
