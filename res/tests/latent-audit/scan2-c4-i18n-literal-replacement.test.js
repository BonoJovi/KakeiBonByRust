// latent-audit scan2-C4: i18n.t() and the recurring-rule delete confirmation use user text as a replacement string ($&, $1, {1} are interpreted)
/**
 * i18n.t() substitutes params with String.prototype.replace(regex, value), so
 * `$&`, `$'`, `$1` ... in a user name are interpreted as replacement patterns
 * (a user named "A$&B" is welcomed as "A{name}B"). recurring-rule.js builds
 * the delete confirmation with chained tmpl.replace('{0}', name)
 * .replace('{1}', count): a rule name with `$'` or `{1}` garbles the message
 * (the count lands inside the name and the real `{1}` stays literal).
 * Expected: user text is inserted literally.
 */

import { jest } from '@jest/globals';
import {
    mockPageModules, loadPageBody, bootPage, flush,
} from '../pages/_page-harness.js';

// The real I18n instance, loaded before the page harness mocks i18n.js.
const { default: realI18n } = await import('../../js/i18n.js');

const TEMPLATES = {
    'login.welcome': 'Welcome, {name}!',
    'recurring_rule.delete_confirm_message':
        'Delete rule "{0}"? It currently has {1} generated occurrence(s).',
};

const RULE = {
    rule_id: 7,
    rule_name: 'x$\'y{1}z',
    period_unit: 'MONTH',
    period_interval: 1,
    holiday_shift_type: 'NONE',
    start_date: '2026-01-01',
    end_date: '2026-12-31',
    total_amount: 1000,
    occurrence_count: 12,
};

const { i18n: pageI18n } = mockPageModules(jest, {
    invoke: (cmd) => {
        switch (cmd) {
            case 'list_recurring_rules':
                return [RULE];
            case 'get_category_tree_with_lang':
            case 'get_accounts':
            case 'get_shops':
                return [];
            default:
                return null;
        }
    },
});
// The page sees the real t() with the seeded English templates.
pageI18n.t = (key, params) => realI18n.t(key, params);

describe('scan2-C4 — user text is inserted literally', () => {
    beforeAll(() => {
        realI18n.translations = { ...TEMPLATES };
    });

    test.each(['A$&B', "A$'B", 'A$`B', 'A$$B'])(
        'i18n.t() keeps the user name %s literally',
        (name) => {
            expect(realI18n.t('login.welcome', { name })).toBe(`Welcome, ${name}!`);
        }
    );

    test('recurring-rule delete confirmation keeps the rule name literally', async () => {
        loadPageBody('recurring-rule.html');
        await import('../../js/recurring-rule.js');
        await bootPage();
        await flush(10);

        const delBtn = document.querySelector('#rules-tbody button.btn-danger');
        expect(delBtn).not.toBeNull();
        delBtn.click();

        expect(document.getElementById('delete-modal-message').textContent).toBe(
            `Delete rule "${RULE.rule_name}"? It currently has 12 generated occurrence(s).`
        );
    });
});
