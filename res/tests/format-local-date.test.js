/**
 * format-local-date — timezone-safe YYYY-MM-DD formatting (Fable-5 #13)
 *
 * Pins the local-timezone date formatter that replaced
 * `new Date().toISOString().slice(0, 10)` in `recurring-rule.js`.
 * The old one-liner used UTC, so a JST user who opened the modal
 * before 09:00 JST saw yesterday in every date default.
 *
 * The Fable-5 #13 divergence pins below build their Date via
 * `Date.UTC(...)` and rely on the runtime being in a non-UTC zone
 * so local-getter and `.toISOString()` views diverge. TZ pinning
 * lives in `res/tests/jest.global-setup.cjs` (wired via the
 * `"globalSetup"` key in `res/tests/package.json`) — that script
 * runs in the Jest main process before workers fork, so each
 * worker's Node starts with `TZ=Asia/Tokyo`. Pinning it inside
 * this file is unreliable (Node caches TZ at startup) and pinning
 * it via a shell `TZ=…` prefix in the npm script breaks on Windows
 * `cmd.exe`. The globalSetup route is cross-platform and adds no
 * dev dependency.
 *
 * Pure helper — no i18n / DOM / Tauri stubs needed.
 */

import { formatLocalDate, formatUtcAsLocalDateTime } from '../js/format-local-date.js';

describe('formatLocalDate — normal cases', () => {
    test('should render YYYY-MM-DD in the local zone when the date is mid-year', () => {
        // Local constructor form so the assertion holds under any
        // timezone the tests happen to run in.
        const d = new Date(2026, 4, 15, 12, 0, 0); // 2026-05-15 local noon
        expect(formatLocalDate(d)).toBe('2026-05-15');
    });

    test('should zero-pad the month when it has a single digit', () => {
        const d = new Date(2026, 0, 15, 12, 0, 0); // Jan
        expect(formatLocalDate(d)).toBe('2026-01-15');
    });

    test('should zero-pad the day when it has a single digit', () => {
        const d = new Date(2026, 4, 3, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('2026-05-03');
    });

    test('should zero-pad both when the month and the day have a single digit', () => {
        const d = new Date(2026, 0, 5, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('2026-01-05');
    });

    test('should render month 12 when the date is in December (month index 11)', () => {
        const d = new Date(2026, 11, 31, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('2026-12-31');
    });

    test('should return that local date when the time is local midnight', () => {
        const d = new Date(2026, 5, 15, 0, 0, 0);
        expect(formatLocalDate(d)).toBe('2026-06-15');
    });

    test('should return the same local date when the time is one second before midnight', () => {
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
    test('should render 2026-05-15 (JST wall clock) when the time is UTC 21:30 on 2026-05-14', () => {
        // Local view under Asia/Tokyo: 2026-05-15 06:30.
        // Pre-fix `.toISOString().slice(0, 10)` = "2026-05-14" — WRONG.
        // Post-fix local getters = "2026-05-15" — CORRECT.
        const d = new Date(Date.UTC(2026, 4, 14, 21, 30, 0));
        expect(formatLocalDate(d)).toBe('2026-05-15');
    });

    test('should render 2026-05-16 (JST wall clock) when the time is UTC 15:30 on 2026-05-15', () => {
        // Local view under Asia/Tokyo: 2026-05-16 00:30.
        // Pre-fix `.toISOString().slice(0, 10)` = "2026-05-15" — WRONG.
        // Post-fix local getters = "2026-05-16" — CORRECT.
        const d = new Date(Date.UTC(2026, 4, 15, 15, 30, 0));
        expect(formatLocalDate(d)).toBe('2026-05-16');
    });

    // Local-constructor sanity: these hold in any TZ and pin the
    // "same wall-clock day regardless of hour" contract that
    // recurring-rule.js relies on for its `new Date()` default.
    test('should render 2026-05-15 when the time is local 06:30 on 2026-05-15', () => {
        const d = new Date(2026, 4, 15, 6, 30, 0);
        expect(formatLocalDate(d)).toBe('2026-05-15');
    });

    test('should render 2026-05-15 when the time is local 23:30 on 2026-05-15', () => {
        const d = new Date(2026, 4, 15, 23, 30, 0);
        expect(formatLocalDate(d)).toBe('2026-05-15');
    });
});

describe('formatLocalDate — boundary years', () => {
    test('should render the date when the year is 1900', () => {
        const d = new Date(1900, 0, 1, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('1900-01-01');
    });

    test('should render the date when the year is 2100', () => {
        const d = new Date(2100, 11, 31, 12, 0, 0);
        expect(formatLocalDate(d)).toBe('2100-12-31');
    });

    test('should render Feb 29 correctly when the year is a leap year', () => {
        const d = new Date(2024, 1, 29, 12, 0, 0); // 2024 is a leap year
        expect(formatLocalDate(d)).toBe('2024-02-29');
    });

    // CodeRabbit on #134 — early-AD dates must still produce a
    // valid `YYYY-MM-DD` string (usable by `input[type=date]`),
    // not a bare `1-01-02`.
    test('should pad to 4 digits ("0001-01-02") when the year is 1', () => {
        const d = new Date(1, 0, 2, 12, 0, 0);
        // Note: JS Date treats a 2-digit year (0-99) as 1900-1999
        // in the local constructor, so we build year 1 explicitly
        // with setFullYear to bypass that quirk.
        d.setFullYear(1);
        expect(formatLocalDate(d)).toBe('0001-01-02');
    });

    test('should pad to 4 digits ("0999-06-15") when the year is 999', () => {
        const d = new Date(999, 5, 15, 12, 0, 0);
        d.setFullYear(999);
        expect(formatLocalDate(d)).toBe('0999-06-15');
    });
});

// Created/updated timestamps are stored in UTC (`datetime('now')`) as
// `YYYY-MM-DD HH:MM:SS`. The User Management list printed them as stored,
// so a JST user saw times 9 hours behind. Runs under TZ=Asia/Tokyo.
describe('formatUtcAsLocalDateTime — stored UTC shown in local time', () => {
    test('should show 09:00 when the stored UTC time is midnight (JST)', () => {
        expect(formatUtcAsLocalDateTime('2026-01-01 00:00:00')).toBe('2026-01-01 09:00:00');
    });

    test('should move to the next local day when the stored UTC time is in the afternoon', () => {
        expect(formatUtcAsLocalDateTime('2026-04-22 16:43:45')).toBe('2026-04-23 01:43:45');
    });

    test('should convert the value when it uses the ISO "T" separator', () => {
        expect(formatUtcAsLocalDateTime('2026-05-26T19:28:14')).toBe('2026-05-27 04:28:14');
    });

    test('should show the value as it is when it is not a timestamp', () => {
        expect(formatUtcAsLocalDateTime('not a date')).toBe('not a date');
    });

    test('should show the value as it is when text follows the timestamp', () => {
        expect(formatUtcAsLocalDateTime('2026-01-01 00:00:00oops')).toBe('2026-01-01 00:00:00oops');
    });

    test('should return an empty string when the value is empty', () => {
        expect(formatUtcAsLocalDateTime('')).toBe('');
        expect(formatUtcAsLocalDateTime(null)).toBe('');
    });
});
