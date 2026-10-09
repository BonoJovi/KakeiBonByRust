// An older, slower dashboard load does not overwrite the newer month (latent-audit scan2-A4)
/**
 * Dashboard (res/js/dashboard.js).
 *
 * scan2-A4  loadDashboardData() had a request token only for the account balances
 * panel; the charts and their titles are drawn by whichever load finishes
 * last. Execute for 2026-09 (slow), then switch to 2026-03 and Execute
 * again: March is drawn first, then the September load finishes and
 * redraws September while the filter says March.
 *
 * Expected: the older load is discarded; the charts and titles stay on
 * the latest request (March).
 *
 * The real dashboard page module is booted against res/dashboard.html via
 * ../pages/_page-harness.js; chart.js is replaced by a recording fake.
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage, flush, deferred } from './_page-harness.js';

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

const row = (key, name, amount) => ({ group_key: key, group_name: name, total_amount: amount, count: 1, avg_amount: amount });
let slowSeptember = null;

mockPageModules(jest, {
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_monthly_aggregation':
                if (args.groupBy !== 'category2') return [];
                if (args.year === 2026 && args.month === 9 && slowSeptember) return slowSeptember.promise;
                if (args.year === 2026 && args.month === 3) return [row('EXPENSE/MAR', 'MarchCat', -3000)];
                return [];
            case 'get_user_period_settings':
                return { month_period_start_day: 1, year_period_start_month: 1, year_period_start_day: 1, month_period_holiday_shift: 0 };
            case 'get_monthly_period_bounds':
                return { start: `${args.year}-${String(args.month).padStart(2, '0')}-01`, end: `${args.year}-${String(args.month).padStart(2, '0')}-28` };
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
await import('../../../res/js/dashboard.js');
await bootPage();

function execute(year, month, trendMonths) {
    document.getElementById('year').value = String(year);
    document.getElementById('month').value = String(month);
    document.getElementById('trend-months').value = String(trendMonths);
    document.getElementById('execute-btn').click();
}

describe('dashboard reload staleness (latent scan2-A4)', () => {
    test('should not overwrite the newer March charts when a slower, older September load finishes later (scan2-A4)', async () => {
        slowSeptember = deferred();
        execute(2026, 9, 12);
        await flush(10);

        execute(2026, 3, 3);
        await flush(20);
        const barAfterMarch = charts.filter((c) => c.config.type === 'bar').pop();
        expect(barAfterMarch.config.data.labels).toEqual(['MarchCat']); // sanity

        slowSeptember.resolve([row('EXPENSE/SEP', 'SeptemberCat', -9000)]);
        await flush(20);

        const bar = charts.filter((c) => c.config.type === 'bar').pop();
        expect(bar.config.data.labels).toEqual(['MarchCat']);
        expect(document.querySelector('#category-bar-card h3').textContent).toContain('2026年3月');
    });
});
