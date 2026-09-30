// latent-audit scan2-R8: start date after end date is only rejected by the backend, whose English message is shown verbatim
/**
 * R8  recurring-rule.js has no start <= end check; the backend rejects the
 *     request with the hard-coded `start_date must be on or before end_date`
 *     (code 'validation'), and the generic fallback shows it after the
 *     localized prefix: 「ルール作成に失敗しました： start_date must be on or
 *     before end_date」 — English text on the JA screen.
 *     Expected: the screen shows an error with no raw backend English text
 *     (bug-list fix direction: a client-side start <= end check with an i18n
 *     message, so create_recurring_rule is not called at all).
 *
 * Real page module (recurring-rule.js) booted against res/recurring-rule.html.
 * The create_recurring_rule mock rejects exactly like the Rust command does.
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

const BACKEND_MESSAGE = 'start_date must be on or before end_date';

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
            case 'create_recurring_rule':
                if (args.request.start_date > args.request.end_date) {
                    return Promise.reject({ code: 'validation', message: BACKEND_MESSAGE });
                }
                return { rule_id: 1, generated_count: 1 };
            default:
                return null;
        }
    },
});

loadPageBody('recurring-rule.html');
await import('../../js/recurring-rule.js');
await bootPage();

describe('scan2-R8 recurring rule form — start date after end date', () => {
    test('should reject start > end with a localized message, not the backend English text', async () => {
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
        document.getElementById('start-date').value = `${year}-06-02`;
        document.getElementById('end-date').value = `${year}-06-01`;
        document.getElementById('anchor-date').value = `${year}-06-02`;

        document.getElementById('recurring-rule-form').dispatchEvent(
            new Event('submit', { cancelable: true, bubbles: true })
        );
        await flush(10);

        const box = document.getElementById('result-box');
        expect(box.classList.contains('error')).toBe(true);
        expect(box.textContent).not.toContain(BACKEND_MESSAGE);
        expect(callsOf(invoke, 'create_recurring_rule')).toHaveLength(0);
    });
});
