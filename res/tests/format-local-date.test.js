/**
 * format-local-date — timezone-safe YYYY-MM-DD formatting (Fable-5 #13)
 *
 * Pins the local-timezone date formatter that replaced
 * `new Date().toISOString().slice(0, 10)` in `recurring-rule.js`.
 * The old one-liner used UTC, so a JST user who opened the modal
 * before 09:00 JST saw yesterday in every date default.
 *
 * CodeRabbit on #134 noted that if the CI or a contributor runs
 * Jest under `TZ=UTC`, local getters and `.toISOString()` produce
 * the same date and a UTC regression would go undetected. Pin
 * `TZ=Asia/Tokyo` at the top of this file (before any Date value
 * is constructed) so the divergence tests below actually diverge.
 * Node reads `process.env.TZ` on demand for Date operations, so
 * setting it here scopes the timezone to just this test file.
 *
 * Pure helper — no i18n / DOM / Tauri stubs needed.
 */

process.env.TZ = 'Asia/Tokyo';

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
    // Under the pinned TZ=Asia/Tokyo, the following two instants
    // have DIFFERENT local-view and UTC-view dates. `formatLocalDate`
    // must return the local view. Pre-fix
    // `toISOString().slice(0, 10)` would return the UTC view — which
    // *would* fail these assertions with the TZ pin in place. That's
    // the "would-catch-a-regression" property the previous version
    // of this file was missing.
    test('UTC 21:30 on 2026-05-14 renders as 2026-05-15 (JST wall clock)', () => {
        // Local view under Asia/Tokyo: 2026-05-15 06:30.
        // Pre-fix `.toISOString().slice(0, 10)` = "2026-05-14" — WRONG.
        // Post-fix local getters = "2026-05-15" — CORRECT.
        const d = new Date(Date.UTC(2026, 4, 14, 21, 30, 0));
        expect(formatLocalDate(d)).toBe('2026-05-15');
    });

    test('UTC 15:30 on 2026-05-15 renders as 2026-05-16 (JST wall clock)', () => {
        // Local view under Asia/Tokyo: 2026-05-16 00:30.
        // Pre-fix `.toISOString().slice(0, 10)` = "2026-05-15" — WRONG.
        // Post-fix local getters = "2026-05-16" — CORRECT.
        const d = new Date(Date.UTC(2026, 4, 15, 15, 30, 0));
        expect(formatLocalDate(d)).toBe('2026-05-16');
    });

    // Local-constructor sanity: these hold in any TZ and pin the
    // "same wall-clock day regardless of hour" contract that
    // recurring-rule.js relies on for its `new Date()` default.
    test('local 06:30 on 2026-05-15 renders as 2026-05-15', () => {
        const d = new Date(2026, 4, 15, 6, 30, 0);
        expect(formatLocalDate(d)).toBe('2026-05-15');
    });

    test('local 23:30 on 2026-05-15 renders as 2026-05-15', () => {
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

    // CodeRabbit on #134 — early-AD dates must still produce a
    // valid `YYYY-MM-DD` string (usable by `input[type=date]`),
    // not a bare `1-01-02`.
    test('year 1 renders with 4-digit padding ("0001-01-02")', () => {
        const d = new Date(1, 0, 2, 12, 0, 0);
        // Note: JS Date treats a 2-digit year (0-99) as 1900-1999
        // in the local constructor, so we build year 1 explicitly
        // with setFullYear to bypass that quirk.
        d.setFullYear(1);
        expect(formatLocalDate(d)).toBe('0001-01-02');
    });

    test('year 999 renders with 4-digit padding ("0999-06-15")', () => {
        const d = new Date(999, 5, 15, 12, 0, 0);
        d.setFullYear(999);
        expect(formatLocalDate(d)).toBe('0999-06-15');
    });
});
