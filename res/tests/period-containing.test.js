/**
 * period.js — findMonthlyPeriodContaining / findYearlyPeriodContaining
 * (latent-audit scan2-A3)
 *
 * A monthly period is named by its start month, so the calendar month can
 * name a period that does not contain today. The dashboard opens on the
 * period that does. Pinned: the calendar month when it contains the date,
 * the previous month when the period starts later, the next month when it
 * ended earlier (holiday shift back over the month end), a period two months
 * away (CodeRabbit on #178), year wrap-around both ways, and the calendar
 * month as the fallback when the backend fails or never matches.
 */

import { jest } from '@jest/globals';

const invoke = jest.fn();
jest.unstable_mockModule('@tauri-apps/api/core', () => ({ invoke }));

const { findMonthlyPeriodContaining, findYearlyPeriodContaining } = await import('../js/period.js');

describe('findMonthlyPeriodContaining (latent scan2-A3)', () => {
    beforeEach(() => {
        invoke.mockReset();
    });

    test('should keep the calendar month when its period contains the date (scan2-A3)', async () => {
        invoke.mockResolvedValue({ start: '2026-09-01', end: '2026-09-30' });

        await expect(findMonthlyPeriodContaining(new Date(2026, 8, 10)))
            .resolves.toEqual({ year: 2026, month: 9 });
        expect(invoke).toHaveBeenCalledWith('get_monthly_period_bounds', { year: 2026, month: 9 });
    });

    // Answer get_monthly_period_bounds from a { 'year-month': bounds } table.
    const answerFrom = (bounds) =>
        invoke.mockImplementation(async (cmd, { year, month }) => bounds[`${year}-${month}`]);

    test('should step back when the calendar month\'s period starts after the date (scan2-A3)', async () => {
        // Start day 25: "September" is 9/25 .. 10/24, so 9/10 is in "August".
        answerFrom({
            '2026-9': { start: '2026-09-25', end: '2026-10-24' },
            '2026-8': { start: '2026-08-25', end: '2026-09-24' },
            '2027-1': { start: '2027-01-25', end: '2027-02-24' },
            '2026-12': { start: '2026-12-25', end: '2027-01-24' },
        });
        await expect(findMonthlyPeriodContaining(new Date(2026, 8, 10)))
            .resolves.toEqual({ year: 2026, month: 8 });
        await expect(findMonthlyPeriodContaining(new Date(2027, 0, 10)))
            .resolves.toEqual({ year: 2026, month: 12 });
    });

    test('should step forward when the calendar month\'s period ended before the date (scan2-A3)', async () => {
        // Start day 1, shifted back: "December" starts 11/30, "November" ends 11/29.
        answerFrom({
            '2026-11': { start: '2026-11-01', end: '2026-11-29' },
            '2026-12': { start: '2026-11-30', end: '2026-12-30' },
            '2027-1': { start: '2026-12-31', end: '2027-01-31' },
        });
        await expect(findMonthlyPeriodContaining(new Date(2026, 10, 30)))
            .resolves.toEqual({ year: 2026, month: 12 });
        await expect(findMonthlyPeriodContaining(new Date(2026, 11, 31)))
            .resolves.toEqual({ year: 2027, month: 1 });
    });

    test('should keep stepping when the neighbouring month does not contain the date either (scan2-A3)', async () => {
        // Start day 31, next business day: 2026-01-31 and 02-28 are Saturdays,
        // so "January" is 02-02..03-01 and "February" starts on 03-02.
        answerFrom({
            '2026-3': { start: '2026-03-31', end: '2026-04-29' },
            '2026-2': { start: '2026-03-02', end: '2026-03-30' },
            '2026-1': { start: '2026-02-02', end: '2026-03-01' },
        });

        await expect(findMonthlyPeriodContaining(new Date(2026, 2, 1)))
            .resolves.toEqual({ year: 2026, month: 1 });
        expect(invoke).toHaveBeenCalledTimes(3);
    });

    test('should give up on the calendar month when no period ever matches (scan2-A3)', async () => {
        // A backend answering the same future period for every month.
        invoke.mockResolvedValue({ start: '2099-01-01', end: '2099-01-31' });
        const warn = jest.spyOn(console, 'warn').mockImplementation(() => {});

        await expect(findMonthlyPeriodContaining(new Date(2026, 8, 10)))
            .resolves.toEqual({ year: 2026, month: 9 });
        expect(invoke.mock.calls.length).toBeLessThanOrEqual(6);

        warn.mockRestore();
    });

    test('should fall back to the calendar month when the backend fails (scan2-A3)', async () => {
        invoke.mockRejectedValue(new Error('boom'));
        const warn = jest.spyOn(console, 'warn').mockImplementation(() => {});

        await expect(findMonthlyPeriodContaining(new Date(2026, 8, 10)))
            .resolves.toEqual({ year: 2026, month: 9 });

        warn.mockRestore();
    });
});

describe('findYearlyPeriodContaining (latent scan2-A3)', () => {
    // A yearly period is named by its start year and has no holiday shift,
    // so the year containing a date is its calendar year or the one before.
    test.each([
        // [start month, start day, date, expected year]
        [1, 1, new Date(2026, 0, 1), 2026],
        [1, 1, new Date(2026, 11, 31), 2026],
        [4, 1, new Date(2026, 1, 10), 2025],
        [4, 1, new Date(2026, 2, 31), 2025],
        [4, 1, new Date(2026, 3, 1), 2026],
        [12, 31, new Date(2026, 0, 5), 2025],
        [12, 31, new Date(2026, 11, 30), 2025],
        [12, 31, new Date(2026, 11, 31), 2026],
        // Start 02-31 resolves to the month end: 2026-02-28 / 2027-02-28.
        [2, 31, new Date(2026, 1, 27), 2025],
        [2, 31, new Date(2026, 1, 28), 2026],
    ])('should place the date in the right yearly period when the year starts on %i/%i (date %p -> year %i) (scan2-A3)', (startMonth, startDay, date, expected) => {
        expect(findYearlyPeriodContaining(date, startMonth, startDay)).toBe(expected);
    });
});
