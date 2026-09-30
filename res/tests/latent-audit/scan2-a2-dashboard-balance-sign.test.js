// latent-audit scan2-A2: dashboard formatAmount() takes Math.abs, so a monthly deficit on the trend chart's Balance tooltip reads as a surplus
/**
 * The trend chart's Balance series (income - expense) is negative in a
 * deficit month, but its tooltip label goes through dashboard.js
 * formatAmount(), which is '¥' + Math.abs(amount) -> "¥30,000" for -30,000.
 *
 * Expected: the Balance tooltip keeps the sign ("-¥30,000", matching the
 * aggregation screens' L12 format).
 *
 * The real dashboard page module is booted against res/dashboard.html via
 * ../pages/_page-harness.js; chart.js is replaced by a recording fake.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage } from '../pages/_page-harness.js';

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

mockPageModules(jest, {
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_monthly_aggregation':
                return args.groupBy === 'category1' ? category1Rows : [];
            case 'get_user_period_settings':
                return { month_period_start_day: 1, year_period_start_month: 1, year_period_start_day: 1, month_period_holiday_shift: 0 };
            case 'get_monthly_period_bounds':
                return { start: '2026-09-01', end: '2026-09-30' };
            case 'get_account_balances_as_of':
                return [];
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

describe('dashboard trend chart balance tooltip (latent scan2-A2)', () => {
    test('[scan2-A2] a -30,000 deficit is shown with its minus sign', () => {
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
});
