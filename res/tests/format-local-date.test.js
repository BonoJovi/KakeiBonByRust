/**
 * format-local-date — timezone-safe YYYY-MM-DD formatting (Fable-5 #13)
 *
 * Pins the local-timezone date formatter that replaced
 * `new Date().toISOString().slice(0, 10)` in `recurring-rule.js`.
 * The old one-liner used UTC, so a JST user who opened the modal
 * before 09:00 JST saw yesterday in every date default. The new
 * `formatLocalDate` uses the browser's local getters (getFullYear,
 * getMonth, getDate), so the tests below hold whatever the local
 * timezone happens to be — they build Date values with the local
 * constructor so they're comparable against the local-getter output.
 *
 * Pure helper — no i18n / DOM / Tauri stubs needed.
 */

import { formatLocalDate } from '../js/format-local-date.js';

describe('formatLocalDate — normal cases', () => {
    test('mid-year date renders as YYYY-MM-DD (local zone)', () => {
        // Local constructor form so the assertion holds under any
        // timezone the tests happen to run in.
        const d = new Date(2026, 4, 15, 12, 0, 0); // 2026-05-15 local noon
        expect(formatLocalDate(d)).toBe('2026-05-15');
    });

    test('single-digit month is zero-padded', () => {
        const d = new Date(2026, 0, 15, 12, 0, 0); // Jan
        expect(formatLocalDate(d)).toBe('2026-01-15');
    });

    test('single-digit day is zero-padded', () => {
        const d = new Date(2026, 4, 3, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('2026-05-03');
    });

    test('single-digit month and day both zero-padded', () => {
        const d = new Date(2026, 0, 5, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('2026-01-05');
    });

    test('December (month index 11) renders as month 12', () => {
        const d = new Date(2026, 11, 31, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('2026-12-31');
    });

    test('midnight local time still returns that local date', () => {
        const d = new Date(2026, 5, 15, 0, 0, 0);
        expect(formatLocalDate(d)).toBe('2026-06-15');
    });

    test('one second before midnight still returns the same local date', () => {
        const d = new Date(2026, 5, 15, 23, 59, 59);
        expect(formatLocalDate(d)).toBe('2026-06-15');
    });
});

describe('formatLocalDate — Fable-5 #13 pin (does not drift to UTC)', () => {
    // The old `toISOString().slice(0, 10)` would return one day
    // earlier for anything past this instant if UTC lags local, or
    // one day later if UTC leads local. `formatLocalDate` must return
    // the *local* wall-clock date every time. We build a Date whose
    // local-getter view is 2026-05-15 (chosen deliberately: any hour,
    // any zone, the answer is fixed by the local constructor) and
    // assert `formatLocalDate` returns that exact string.
    test('local 06:30 on 2026-05-15 renders as 2026-05-15 (not the UTC-shifted day)', () => {
        // In JST this instant is 21:30 UTC on 2026-05-14; the buggy
        // one-liner would have returned '2026-05-14'.
        const d = new Date(2026, 4, 15, 6, 30, 0);
        expect(formatLocalDate(d)).toBe('2026-05-15');
    });

    test('local 23:30 on 2026-05-15 renders as 2026-05-15 (not the UTC-shifted next day)', () => {
        // In a UTC-6 zone this instant is 05:30 UTC on 2026-05-16;
        // the buggy one-liner would have returned '2026-05-16'.
        const d = new Date(2026, 4, 15, 23, 30, 0);
        expect(formatLocalDate(d)).toBe('2026-05-15');
    });
});

describe('formatLocalDate — boundary years', () => {
    test('year 1900', () => {
        const d = new Date(1900, 0, 1, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('1900-01-01');
    });

    test('year 2100', () => {
        const d = new Date(2100, 11, 31, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('2100-12-31');
    });

    test('leap-year Feb 29 renders correctly', () => {
        const d = new Date(2024, 1, 29, 12, 0, 0); // 2024 is a leap year
        expect(formatLocalDate(d)).toBe('2024-02-29');
    });
});
