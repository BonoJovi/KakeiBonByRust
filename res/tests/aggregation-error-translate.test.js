/**
 * aggregation-common — translateAggregationError shape guard (Fable-5 #9)
 *
 * Pins the coercion that was added when the aggregation error banner
 * started rendering the literal `"[object Object]"`. The helper now
 * routes any incoming error through `formatApiError` before running
 * substring matches, so both the legacy `Err(String)` shape and the
 * post-migration `ApiError { code, message }` shape resolve to the
 * same i18n key.
 *
 * `aggregation-common.js` transitively imports `res/js/i18n.js`, which
 * pulls `@tauri-apps/api/core` — a real ESM module that only exists in
 * a Tauri build. We stub it (and the i18n singleton) with
 * `jest.unstable_mockModule`, then dynamic-import the module under
 * test — same trick as `master-crud.test.js`.
 */

import { jest } from '@jest/globals';

jest.unstable_mockModule('@tauri-apps/api/core', () => ({
    invoke: jest.fn(),
}));

// i18n stub — returns a deterministic marker per key so tests can
// assert on which branch fired. Falsy return still falls through to
// the raw errorStr fallback the helper already handled.
const translations = {
    'aggregation.error_invalid_year': 'i18n:invalid_year',
    'aggregation.error_invalid_month': 'i18n:invalid_month',
    'aggregation.error_invalid_date_range': 'i18n:invalid_date_range',
    'aggregation.error_invalid_day': 'i18n:invalid_day',
    'aggregation.error_invalid_date_format': 'i18n:invalid_date_format',
    'aggregation.error_generic': 'i18n:generic',
};
jest.unstable_mockModule('../js/i18n.js', () => ({
    default: {
        t: (key) => translations[key],
        updateUI: () => {},
        init: async () => {},
    },
}));

const { translateAggregationError } = await import('../js/aggregation-common.js');

describe('translateAggregationError — legacy string errors (Err(String))', () => {
    test('routes "Invalid year" to the year i18n key', () => {
        expect(translateAggregationError('Invalid year: 1800. Year must be between 1900 and 2100.'))
            .toBe('i18n:invalid_year');
    });

    test('routes "Invalid month" to the month i18n key', () => {
        expect(translateAggregationError('Invalid month: 13. Month must be between 1 and 12.'))
            .toBe('i18n:invalid_month');
    });

    test('routes "Invalid date range" to the date-range i18n key', () => {
        expect(translateAggregationError('Invalid date range: 2026-01-01 to 2025-12-31.'))
            .toBe('i18n:invalid_date_range');
    });

    test('routes "Invalid day" to the day i18n key', () => {
        expect(translateAggregationError('Invalid day 31 for 2026-02'))
            .toBe('i18n:invalid_day');
    });

    test('routes "Invalid date format" to the format i18n key', () => {
        expect(translateAggregationError('Invalid date format: not-a-date'))
            .toBe('i18n:invalid_date_format');
    });

    test('falls back to the raw string when nothing matches', () => {
        expect(translateAggregationError('Failed to execute aggregation query: db down'))
            .toBe('Failed to execute aggregation query: db down');
    });
});

describe('translateAggregationError — ApiError shape ({ code, message })', () => {
    test('extracts .message and routes on it (was rendering "[object Object]" pre-fix)', () => {
        const apiError = {
            code: 'validation',
            message: 'Invalid year: 1800. Year must be between 1900 and 2100.',
        };
        expect(translateAggregationError(apiError)).toBe('i18n:invalid_year');
    });

    test('unmatched ApiError message falls back to that message, never "[object Object]"', () => {
        const apiError = {
            code: 'internal',
            message: 'Failed to execute aggregation query: db down',
        };
        expect(translateAggregationError(apiError))
            .toBe('Failed to execute aggregation query: db down');
    });
});

describe('translateAggregationError — Error instances', () => {
    test('routes Error.message through the substring branches', () => {
        expect(translateAggregationError(new Error('Invalid month: 13')))
            .toBe('i18n:invalid_month');
    });
});

describe('translateAggregationError — hostile shapes', () => {
    // These pin down the actual banner text the user sees. Pre-fix the
    // helper returned the literal `"[object Object]"` for every shape
    // in this block; the point of Fable-5 #9 is that that string is
    // never surfaced. All three cases must land on the localised
    // generic fallback (`aggregation.error_generic`).
    test('object with no .message resolves to the generic i18n banner (was "[object Object]")', () => {
        expect(translateAggregationError({ weird: true })).toBe('i18n:generic');
    });

    test('null resolves to the generic i18n banner', () => {
        expect(translateAggregationError(null)).toBe('i18n:generic');
    });

    test('undefined resolves to the generic i18n banner', () => {
        expect(translateAggregationError(undefined)).toBe('i18n:generic');
    });

    test('empty-string ApiError message resolves to the generic banner too', () => {
        expect(translateAggregationError({ code: 'internal', message: '' })).toBe('i18n:generic');
    });
});
