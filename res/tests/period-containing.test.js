/**
 * period.js — findMonthlyPeriodContaining (latent-audit scan2-A3)
 *
 * A monthly period is named by its start month, so the calendar month can
 * name a period that does not contain today. The dashboard opens on the
 * period that does. Pinned: the calendar month when it contains the date,
 * the previous month when the period starts later, the next month when it
 * ended earlier (holiday shift back over the month end), year wrap-around
 * both ways, and the calendar month as the fallback when the backend fails.
 */

import { jest } from '@jest/globals';

const invoke = jest.fn();
jest.unstable_mockModule('@tauri-apps/api/core', () => ({ invoke }));

const { findMonthlyPeriodContaining } = await import('../js/period.js');

describe('findMonthlyPeriodContaining (latent scan2-A3)', () => {
    beforeEach(() => {
        invoke.mockReset();
    });

    test('[scan2-A3] should keep the calendar month when its period contains the date', async () => {
        invoke.mockResolvedValue({ start: '2026-09-01', end: '2026-09-30' });

        await expect(findMonthlyPeriodContaining(new Date(2026, 8, 10)))
            .resolves.toEqual({ year: 2026, month: 9 });
        expect(invoke).toHaveBeenCalledWith('get_monthly_period_bounds', { year: 2026, month: 9 });
    });

    test('[scan2-A3] should step back when the calendar month\'s period starts after the date', async () => {
        // Start day 25: "September" is 9/25 .. 10/24, so 9/10 is in "August".
        invoke.mockResolvedValue({ start: '2026-09-25', end: '2026-10-24' });
        await expect(findMonthlyPeriodContaining(new Date(2026, 8, 10)))
            .resolves.toEqual({ year: 2026, month: 8 });

        invoke.mockResolvedValue({ start: '2027-01-25', end: '2027-02-24' });
        await expect(findMonthlyPeriodContaining(new Date(2027, 0, 10)))
            .resolves.toEqual({ year: 2026, month: 12 });
    });

    test('[scan2-A3] should step forward when the calendar month\'s period ended before the date', async () => {
        // Start day 1, shifted back: "December" starts 11/30, "November" ends 11/29.
        invoke.mockResolvedValue({ start: '2026-11-01', end: '2026-11-29' });
        await expect(findMonthlyPeriodContaining(new Date(2026, 10, 30)))
            .resolves.toEqual({ year: 2026, month: 12 });

        invoke.mockResolvedValue({ start: '2026-12-01', end: '2026-12-30' });
        await expect(findMonthlyPeriodContaining(new Date(2026, 11, 31)))
            .resolves.toEqual({ year: 2027, month: 1 });
    });

    test('[scan2-A3] should fall back to the calendar month when the backend fails', async () => {
        invoke.mockRejectedValue(new Error('boom'));
        const warn = jest.spyOn(console, 'warn').mockImplementation(() => {});

        await expect(findMonthlyPeriodContaining(new Date(2026, 8, 10)))
            .resolves.toEqual({ year: 2026, month: 9 });

        warn.mockRestore();
    });
});
