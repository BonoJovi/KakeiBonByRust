/**
 * Recurring rule form (res/js/recurring-rule.js) — latent-scan2 R2.
 *
 * Bug: the daily anchor (起点日) defaulted to today on its own and never
 *      followed the start date, so moving the start date back silently
 *      skipped the days before today, and a past period produced no
 *      occurrences.
 * Pinned: the anchor starts out as the start date and follows it until the
 *      user edits the anchor; Reset makes it follow again.
 *
 * The real page module is booted against res/recurring-rule.html via
 * ./_page-harness.js.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage } from './_page-harness.js';

mockPageModules(jest, {
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_category_tree_with_lang':
                return [];
            case 'get_accounts':
            case 'get_shops':
            case 'list_recurring_rules':
                return [];
            default:
                return null;
        }
    },
});

loadPageBody('recurring-rule.html');
await import('../../js/recurring-rule.js');
await bootPage();

const start = () => document.getElementById('start-date');
const anchor = () => document.getElementById('anchor-date');

function setStart(value) {
    start().value = value;
    start().dispatchEvent(new Event('input'));
    start().dispatchEvent(new Event('change'));
}

describe('recurring rule form — daily anchor follows the start date (latent-scan2 R2)', () => {
    test('should start as the start date and follow it when the anchor has not been edited', () => {
        expect(anchor().value).toBe(start().value);

        setStart('2026-09-01');
        expect(anchor().value).toBe('2026-09-01');

        anchor().value = '2026-09-03';
        anchor().dispatchEvent(new Event('input'));
        setStart('2026-08-01');
        expect(anchor().value).toBe('2026-09-03');
    });

    test('should follow the start date again when the form has been reset', () => {
        document.getElementById('reset-btn').click();
        setStart('2026-07-01');
        expect(anchor().value).toBe('2026-07-01');
    });
});
