// The yearly aggregation screen opens on the yearly period that contains today (latent-audit scan2-A3)
/**
 * Yearly aggregation screen (res/js/aggregation-yearly.js).
 *
 * scan2-A3  initializeFilterDefaults() set the year to the calendar year,
 *           but a yearly period is named by its START year. With the year
 *           starting on 04-01 and today = 2026-02-10, the "2026" period is
 *           2026-04-01..2027-03-31, entirely in the future; the period
 *           containing today is "2025" (2025-04-01..2026-03-31).
 *
 * Expected: the screen opens on 2025.
 *
 * Only Date is faked (timers stay real so the harness flush() works).
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage } from './_page-harness.js';

jest.useFakeTimers({
    now: new Date(2026, 1, 10, 12, 0, 0), // 2026-02-10 local
    doNotFake: [
        'hrtime', 'nextTick', 'performance', 'queueMicrotask',
        'requestAnimationFrame', 'cancelAnimationFrame', 'requestIdleCallback', 'cancelIdleCallback',
        'setImmediate', 'clearImmediate', 'setInterval', 'clearInterval', 'setTimeout', 'clearTimeout',
    ],
});

mockPageModules(jest, {
    invoke: (cmd) => {
        switch (cmd) {
            case 'get_user_period_settings':
                return { month_period_start_day: 1, year_period_start_month: 4, year_period_start_day: 1, month_period_holiday_shift: 0 };
            case 'get_language_names':
                return [];
            case 'get_language':
                return 'ja';
            default:
                return null;
        }
    },
});

loadPageBody('aggregation-yearly.html');
await import('../../js/aggregation-yearly.js');
await bootPage();

afterAll(() => {
    jest.useRealTimers();
});

describe('yearly aggregation default period (scan2-A3)', () => {
    test('[scan2-A3] year starting 04-01, today 2026-02-10 -> opens on the 2025 period that contains today', () => {
        expect(document.getElementById('year').value).toBe('2025');
    });
});
