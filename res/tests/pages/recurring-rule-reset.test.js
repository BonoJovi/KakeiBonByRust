// Reset brings the recurring rule form back to its defaults (latent-audit scan2-R4)
/**
 * Recurring rule screen (res/js/recurring-rule.js).
 *
 * scan2-R4  setupResetButton only called form.reset() and
 *           updateDerivedTotal(). form.reset() fires no `change`, so
 *           updateCycleVisibility() did not run: after choosing Monthly and
 *           pressing Reset, the Daily radio was checked again but the Monthly
 *           fields stayed visible, the 起点日 (anchor) field stayed hidden,
 *           and the holiday-shift select stayed enabled. The start / end /
 *           anchor dates are set from JS (no value attribute), so reset
 *           emptied them.
 *
 * Expected: after Reset the cycle UI matches the checked radio (Daily:
 * anchor visible, Monthly fields hidden, holiday shift '0' + disabled) and
 * the default dates are re-applied (anchor = start = today, end = one year
 * later).
 *
 * Real page module (recurring-rule.js) booted against res/recurring-rule.html.
 */
import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush,
} from './_page-harness.js';

const CATEGORY_TREE = [
    {
        category1: { category1_code: 'EXPENSE', category1_name_i18n: 'Expense' },
        children: [],
    },
];

const year = new Date().getFullYear();

const ymd = (d) => [
    d.getFullYear(),
    String(d.getMonth() + 1).padStart(2, '0'),
    String(d.getDate()).padStart(2, '0'),
].join('-');
const now = new Date();
const today = ymd(now);
const oneYearLater = ymd(new Date(now.getFullYear() + 1, now.getMonth(), now.getDate()));

mockPageModules(jest, {
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
            anchor: today,
            start: today,
            end: oneYearLater,
        });
    });
});
