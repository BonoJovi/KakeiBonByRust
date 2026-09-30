// Dashboard category bar chart: largest expenses first, and the top 10 keeps the largest (latent-audit scan2-A1)
/**
 * Expense totals from the category2 aggregation are negative (EXPENSE is
 * signed x-1 in the backend). updateCategoryBarChart sorts them with
 * `b.total_amount - a.total_amount` (least negative first) and keeps
 * slice(0, 10), so with 13 expense groups the biggest one (rent, -80,000)
 * is cut off and the smallest (-1,000) is drawn at the top.
 *
 * Expected: the bar chart lists the largest expenses first (by magnitude)
 * and its top 10 includes rent.
 *
 * The real dashboard page module is booted against res/dashboard.html via
 * ../pages/_page-harness.js; chart.js is replaced by a recording fake.
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

// Twelve small expense groups (-1,000 .. -12,000) plus rent (-80,000).
const category2Rows = Array.from({ length: 12 }, (_, i) => ({
    group_key: `EXPENSE/C${i + 1}`,
    group_name: `Small${i + 1}`,
    total_amount: -(i + 1) * 1000,
    count: 1,
    avg_amount: -(i + 1) * 1000,
}));
category2Rows.push({ group_key: 'EXPENSE/RENT', group_name: 'Rent', total_amount: -80000, count: 1, avg_amount: -80000 });

mockPageModules(jest, {
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_monthly_aggregation':
                return args.groupBy === 'category2' ? category2Rows : [];
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

describe('dashboard category bar chart (latent scan2-A1)', () => {
    test('[scan2-A1] largest expense category is drawn first and kept in the top 10', () => {
        const bar = charts.filter((c) => c.config.type === 'bar').pop();
        expect(bar).toBeDefined();
        const labels = bar.config.data.labels;
        expect(labels).toContain('Rent');
        expect(labels[0]).toBe('Rent');
    });
});
