// latent-audit scan2-A3: dashboard defaults to the calendar month, which under a custom start day (25) is a future period that does not contain today
/**
 * initializeFilterDefaults() sets year/month to now.getFullYear() /
 * now.getMonth()+1, but a monthly period is named by its START month
 * (period.rs monthly_period_bounds). With MONTH_PERIOD_START_DAY = 25 and
 * today = 2026-09-10, the "September" period is 2026-09-25..2026-10-24,
 * entirely in the future; the period containing today is "August"
 * (2026-08-25..2026-09-24).
 *
 * Expected: the dashboard opens on the period that contains today
 * (2026 / 8) and loads that month.
 *
 * Only Date is faked (timers stay real so the harness flush() works).
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

jest.useFakeTimers({
    now: new Date(2026, 8, 10, 12, 0, 0), // 2026-09-10 local
    doNotFake: [
        'hrtime', 'nextTick', 'performance', 'queueMicrotask',
        'requestAnimationFrame', 'cancelAnimationFrame', 'requestIdleCallback', 'cancelIdleCallback',
        'setImmediate', 'clearImmediate', 'setInterval', 'clearInterval', 'setTimeout', 'clearTimeout',
    ],
});

const START_DAY = 25;
const pad = (n) => String(n).padStart(2, '0');
// Start-day-25 bounds (no holiday shift): (y, m) -> y-m-25 .. next month 24.
function bounds(year, month) {
    const ny = month === 12 ? year + 1 : year;
    const nm = month === 12 ? 1 : month + 1;
    return { start: `${year}-${pad(month)}-${pad(START_DAY)}`, end: `${ny}-${pad(nm)}-${pad(START_DAY - 1)}` };
}

const { invoke } = mockPageModules(jest, {
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_monthly_aggregation':
                return [];
            case 'get_user_period_settings':
                return { month_period_start_day: START_DAY, year_period_start_month: 1, year_period_start_day: 1, month_period_holiday_shift: 0 };
            case 'get_monthly_period_bounds':
                return bounds(args.year, args.month);
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

afterAll(() => {
    jest.useRealTimers();
});

describe('dashboard default period (latent scan2-A3)', () => {
    test('[scan2-A3] start day 25, today 2026-09-10 -> defaults to the August period that contains today', () => {
        expect(document.getElementById('year').value).toBe('2026');
        expect(document.getElementById('month').value).toBe('8');

        const pieLoad = invoke.mock.calls
            .filter(([c, a]) => c === 'get_monthly_aggregation' && a.groupBy === 'category2')
            .map(([, a]) => a);
        expect(pieLoad[0]).toMatchObject({ year: 2026, month: 8 });
    });
});
