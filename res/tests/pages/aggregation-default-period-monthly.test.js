// The monthly aggregation screen opens on the monthly period that contains today (latent-audit scan2-A3)
/**
 * Monthly aggregation screen (res/js/aggregation.js).
 *
 * scan2-A3  initializeFilterDefaults() set year/month to the calendar month,
 *           but a monthly period is named by its START month. With start
 *           day 25 and today = 2026-09-10, "September" is 09-25..10-24,
 *           entirely in the future; the period containing today is "August"
 *           (08-25..09-24). Same bug as the dashboard (fixed in #178).
 *
 * Expected: the screen opens on 2026 / 8.
 *
 * Only Date is faked (timers stay real so the harness flush() works).
 */

import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage } from './_page-harness.js';

jest.useFakeTimers({
    now: new Date(2026, 8, 10, 12, 0, 0), // 2026-09-10 local
    doNotFake: [
        'hrtime', 'nextTick', 'performance', 'queueMicrotask',
        'requestAnimationFrame', 'cancelAnimationFrame', 'requestIdleCallback', 'cancelIdleCallback',
        'setImmediate', 'clearImmediate', 'setInterval', 'clearInterval', 'setTimeout', 'clearTimeout',
    ],
});

const pad = (n) => String(n).padStart(2, '0');
// Start-day-25 bounds (no holiday shift): (y, m) -> y-m-25 .. next month 24.
function bounds(year, month) {
    const ny = month === 12 ? year + 1 : year;
    const nm = month === 12 ? 1 : month + 1;
    return { start: `${year}-${pad(month)}-25`, end: `${ny}-${pad(nm)}-24` };
}

mockPageModules(jest, {
    invoke: (cmd, args) => {
        switch (cmd) {
            case 'get_monthly_period_bounds':
                return bounds(args.year, args.month);
            case 'get_language_names':
                return [];
            case 'get_language':
                return 'ja';
            default:
                return null;
        }
    },
});

loadPageBody('aggregation.html');
await import('../../js/aggregation.js');
await bootPage();

afterAll(() => {
    jest.useRealTimers();
});

describe('monthly aggregation default period (scan2-A3)', () => {
    test('should open on the August period that contains today when the start day is 25 and today is 2026-09-10 (scan2-A3)', () => {
        expect(document.getElementById('year').value).toBe('2026');
        expect(document.getElementById('month').value).toBe('8');
    });
});
