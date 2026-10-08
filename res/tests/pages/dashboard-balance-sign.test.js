// Dashboard amounts keep their sign and are not abbreviated (latent-audit scan2-A2)
/**
 * Dashboard (res/js/dashboard.js).
 *
 * scan2-A2  The trend chart's Balance series (income - expense) is negative
 *           in a deficit month, but its tooltip went through formatAmount(),
 *           which was '¥' + Math.abs(amount) -> "¥30,000" for -30,000. The
 *           axis ticks printed "¥-30,000" for negatives and "¥30K" / "¥1.5M"
 *           for positives, and the account balances printed "¥-1,234".
 *
 * Expected: every dashboard amount uses the aggregation screens' L12 form,
 * sign before the currency symbol ("-¥30,000"), and the axis ticks show the
 * full amount instead of K / M abbreviations (owner decision 2026-10-07: the
 * abbreviations read poorly aloud and are unclear to many users).
 *
 * The real dashboard page module is booted against res/dashboard.html;
 * chart.js is replaced by a recording fake.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage } from './_page-harness.js';

const charts = [];
class FakeChart {
    constructor(ctx, config) {
        this.ctx = ctx;
        this.config = config;
        charts.push(this);
    }
    destroy() {
        this.destroyed = true;
    }
}
// chart.js is only resolvable through the page's import map. Jest 29 checks
// ESM virtual mocks against the CJS virtual-mock table, so register both.
jest.mock('chart.js', () => ({ Chart: FakeChart }), { virtual: true });
jest.unstable_mockModule('chart.js', () => ({ Chart: FakeChart }), { virtual: true });

// Every month: income 200,000, expense 230,000 -> balance -30,000.
const category1Rows = [
    { group_key: 'INCOME', group_name: 'Income', total_amount: 200000, count: 1, avg_amount: 200000 },
    { group_key: 'EXPENSE', group_name: 'Expense', total_amount: -230000, count: 1, avg_amount: -230000 },
];

// Selected month's expense breakdown, drawn by the pie and the top-10 bar chart.
const category2Rows = [
    { group_key: 'EXPENSE/FOOD', group_name: 'Food', total_amount: -30000, count: 1, avg_amount: -30000 },
];

mockPageModules(jest, {
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_monthly_aggregation':
                return args.groupBy === 'category1' ? category1Rows : category2Rows;
            case 'get_user_period_settings':
                return { month_period_start_day: 1, year_period_start_month: 1, year_period_start_day: 1, month_period_holiday_shift: 0 };
            case 'get_monthly_period_bounds':
                return { start: '2026-09-01', end: '2026-09-30' };
            case 'get_account_balances_as_of':
                return [
                    { account_code: 'BANK', account_name: 'Bank', balance: 1500000, is_disabled: 0 },
                    { account_code: 'CARD', account_name: 'Card', balance: -1234, is_disabled: 0 },
                ];
            case 'get_language_names':
                return [];
            default:
                return null;
        }
    },
});

loadPageBody('dashboard.html');
await import('../../js/dashboard.js');
await bootPage();

describe('dashboard amount format (latent scan2-A2)', () => {
    test('should show the minus sign when the balance is a -30,000 deficit (scan2-A2)', () => {
        const line = charts.filter((c) => c.config.type === 'line').pop();
        expect(line).toBeDefined();
        const balance = line.config.data.datasets.find((ds) => ds.label === 'dashboard.balance');
        expect(balance.data.every((v) => v === -30000)).toBe(true); // sanity

        const label = line.config.options.plugins.tooltip.callbacks.label({
            dataset: balance,
            raw: balance.data[0],
        });
        expect(label).toMatch(/-¥30,000/);
    });

    test('should show the full signed amount, without K / M, when the axis ticks are drawn (scan2-A2)', () => {
        const line = charts.filter((c) => c.config.type === 'line').pop();
        const bar = charts.filter((c) => c.config.type === 'bar').pop();
        expect(bar).toBeDefined();
        const lineTick = line.config.options.scales.y.ticks.callback;
        const barTick = bar.config.options.scales.x.ticks.callback;

        expect(lineTick(-30000)).toBe('-¥30,000');
        expect(lineTick(1500000)).toBe('¥1,500,000');
        expect(lineTick(0)).toBe('¥0');
        expect(barTick(30000)).toBe('¥30,000');
    });

    test('should put the minus sign before ¥ when an account balance is negative (scan2-A2)', () => {
        const cells = [...document.querySelectorAll('#account-balances-tbody .balance-col')]
            .map((td) => td.textContent);
        expect(cells).toEqual(['¥1,500,000', '-¥1,234']);
    });
});
