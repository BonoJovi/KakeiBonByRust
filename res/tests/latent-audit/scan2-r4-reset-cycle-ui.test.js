// latent-audit scan2-R4: Reset leaves the cycle UI out of sync and clears the anchor/period dates; the next submit sends anchor_date ""
/**
 * R4  recurring-rule.js setupResetButton only calls form.reset() and
 *     updateDerivedTotal(). form.reset() fires no `change`, so
 *     updateCycleVisibility() does not run: after choosing Monthly and
 *     pressing Reset, the Daily radio is checked again but the Monthly
 *     fields stay visible, the 起点日 (anchor) field stays hidden, and the
 *     holiday-shift select stays enabled. The start / end / anchor dates were
 *     set from JS (no value attribute), so reset empties them; the next
 *     submit sends a DAY rule with anchor_date "" (backend: raw
 *     `Invalid anchor_date: `).
 *     Expected: after Reset the cycle UI matches the checked radio (Daily:
 *     anchor visible, Monthly fields hidden, holiday shift '0' + disabled)
 *     and the default dates are re-applied (anchor, start and end not empty).
 *
 * Real page module (recurring-rule.js) booted against res/recurring-rule.html.
 */
import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush, callsOf,
} from '../pages/_page-harness.js';

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
                return { rule_id: 1, generated_count: 1 };
            default:
                return null;
        }
    },
});

loadPageBody('recurring-rule.html');
await import('../../js/recurring-rule.js');
await bootPage();

function isVisible(id) {
    return document.getElementById(id).classList.contains('visible');
}

describe('scan2-R4 recurring rule form — Reset', () => {
    test('should bring the cycle UI and the default dates back in line after Reset', async () => {
        // Choose Monthly: Monthly fields shown, anchor hidden.
        const monthly = document.querySelector('input[name="cycle-kind"][value="MONTH"]');
        monthly.checked = true;
        monthly.dispatchEvent(new Event('change'));
        await flush();
        expect(isVisible('monthly-mode-group')).toBe(true);
        expect(isVisible('anchor-date-group')).toBe(false);

        document.getElementById('reset-btn').click();
        await flush();

        // The form is back on Daily ...
        expect(document.querySelector('input[name="cycle-kind"]:checked').value).toBe('DAY');
        // ... so the UI must show the Daily fields, not the Monthly ones.
        const state = {
            anchorVisible: isVisible('anchor-date-group'),
            monthlyModeVisible: isVisible('monthly-mode-group'),
            holidayShiftDisabled: document.getElementById('holiday-shift-type').disabled,
            anchor: document.getElementById('anchor-date').value,
            start: document.getElementById('start-date').value,
            end: document.getElementById('end-date').value,
        };
        expect(state).toEqual({
            anchorVisible: true,
            monthlyModeVisible: false,
            holidayShiftDisabled: true,
            anchor: expect.stringMatching(/^\d{4}-\d{2}-\d{2}$/),
            start: expect.stringMatching(/^\d{4}-\d{2}-\d{2}$/),
            end: expect.stringMatching(/^\d{4}-\d{2}-\d{2}$/),
        });
    });
});
