/**
 * period.js — fetchMonthlyPeriodEndDate (latent-audit L14)
 *
 * The dashboard's account balances were computed "as of" the calendar month
 * end even when the user's monthly period starts on another day (e.g. the
 * 25th), so they disagreed with the charts' period. The dashboard now asks
 * for the period's last day. Pinned: the backend's period end is used, and
 * the calendar month end is the fallback when the backend cannot answer.
 */

import { jest } from '@jest/globals';

const invoke = jest.fn();
jest.unstable_mockModule('@tauri-apps/api/core', () => ({ invoke }));

const { fetchMonthlyPeriodEndDate } = await import('../js/period.js');

describe('fetchMonthlyPeriodEndDate (latent audit L14)', () => {
    beforeEach(() => {
        invoke.mockReset();
    });

    test('should return the last day of the user\'s monthly period when the period is requested (L14)', async () => {
        // Period starting on the 25th: September runs 9/25 .. 10/24.
        invoke.mockResolvedValue({ start: '2026-09-25', end: '2026-10-24' });

        await expect(fetchMonthlyPeriodEndDate(2026, 9)).resolves.toBe('2026-10-24');
        expect(invoke).toHaveBeenCalledWith('get_monthly_period_bounds', { year: 2026, month: 9 });
    });

    test('should fall back to the calendar month end when the backend fails (L14)', async () => {
        invoke.mockRejectedValue(new Error('boom'));
        const warn = jest.spyOn(console, 'warn').mockImplementation(() => {});

        await expect(fetchMonthlyPeriodEndDate(2024, 2)).resolves.toBe('2024-02-29');
        await expect(fetchMonthlyPeriodEndDate(2026, 12)).resolves.toBe('2026-12-31');

        warn.mockRestore();
    });
});
