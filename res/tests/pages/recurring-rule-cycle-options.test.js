/**
 * Recurring rule screen (res/js/recurring-rule.js) — regression tests for
 * latent-audit M14 and L13.
 *
 * M14 A monthly rule on day 29–31 silently skipped the months without that
 *     day; the backend's "day or month end" / "month end" rules were not
 *     reachable from the form. Pinned: the day-of-month mode sends
 *     DAY_OR_END, and a new "end of month" mode sends END.
 * L13 A daily rule with a holiday shift produced duplicate / out-of-range
 *     occurrences. Pinned: choosing "daily" resets the holiday shift to
 *     "no shift" and disables it; other cycles can use it again.
 *
 * The real page module is booted against res/recurring-rule.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from './_page-harness.js';

const { invoke } = mockPageModules(jest, {
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [{
                    category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
                    children: [],
                }];
            case 'get_accounts':
                return [{ account_code: 'CASH', account_name: 'Cash' }];
            case 'get_shops':
            case 'list_recurring_rules':
                return [];
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

function check(name, value) {
    const radio = document.querySelector(`input[name="${name}"][value="${value}"]`);
    radio.checked = true;
    radio.dispatchEvent(new Event('change'));
}

function setInput(id, value) {
    const el = document.getElementById(id);
    el.value = value;
    el.dispatchEvent(new Event('input'));
}

async function fillAndSubmit() {
    document.getElementById('rule-name').value = 'Rent';
    const category1 = document.getElementById('category1');
    category1.value = 'EXPENSE';
    category1.dispatchEvent(new Event('change'));
    await flush();
    document.getElementById('item-name').value = 'Rent';
    setInput('tax-rate', '0');
    setInput('amount-excluding-tax', '80000');
    document.getElementById('recurring-rule-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);
    return callsOf(invoke, 'create_recurring_rule').at(-1).request;
}

const visible = (id) => document.getElementById(id).classList.contains('visible');

describe('recurring rule form — month end and daily shift (regression, latent audit 2026-09)', () => {
    beforeEach(() => {
        invoke.mockClear();
    });

    test('should send DAY_OR_END when a day of the month is chosen (M14)', async () => {
        check('cycle-kind', 'MONTH');
        check('monthly-mode', 'DAY');
        document.getElementById('day-of-month').value = '31';

        const request = await fillAndSubmit();

        expect(request.month_day_rule_type).toBe('DAY_OR_END');
        expect(request.day_of_month).toBe(31);
    });

    test('should send END when the end-of-month mode is chosen (M14)', async () => {
        check('cycle-kind', 'MONTH');
        check('monthly-mode', 'END');
        expect(visible('day-of-month-group')).toBe(false);

        const request = await fillAndSubmit();

        expect(request.month_day_rule_type).toBe('END');
        expect(request.day_of_month).toBeNull();
    });

    test('should reset and disable the holiday shift when the rule is daily (L13)', () => {
        check('cycle-kind', 'MONTH');
        const shift = document.getElementById('holiday-shift-type');
        shift.value = '2';
        expect(shift.disabled).toBe(false);

        check('cycle-kind', 'DAY');
        expect(shift.value).toBe('0');
        expect(shift.disabled).toBe(true);

        check('cycle-kind', 'MONTH');
        expect(shift.disabled).toBe(false);
    });
});
