/**
 * parse-amount-strict — money-field parser accept/reject table (Fable-5 #10)
 *
 * Pins the pure integer parser that replaced `parseInt(value) || 0`
 * across the three money-input submit paths (transaction-detail,
 * transaction, recurring-rule). The old lenient parser silently ate
 * `"1099.5"` as 1099 (half-yen loss) and `"1,099"` as 1 (99% off).
 * These tests are the guarantee that the strict replacement rejects
 * every one of those shapes.
 *
 * Pure helper — no i18n / DOM / Tauri stubs needed.
 */

import { parseAmountStrict } from '../js/parse-amount-strict.js';

describe('parseAmountStrict — accept', () => {
    test('plain integer string returns the integer', () => {
        expect(parseAmountStrict('1099')).toBe(1099);
    });

    test('single zero returns 0', () => {
        expect(parseAmountStrict('0')).toBe(0);
    });

    test('leading zeros are preserved as the same integer value', () => {
        // "0099" is still exactly digits — accept, callers rely on
        // Number() normalising the integer value.
        expect(parseAmountStrict('0099')).toBe(99);
    });

    test('surrounding whitespace is trimmed before matching', () => {
        expect(parseAmountStrict('  1099  ')).toBe(1099);
    });

    test('large integer within Number range', () => {
        expect(parseAmountStrict('999999999')).toBe(999999999);
    });
});

describe('parseAmountStrict — empty inputs default to 0', () => {
    // Preserves the `|| 0` fallback every pre-fix caller relied on.
    test('empty string returns 0', () => {
        expect(parseAmountStrict('')).toBe(0);
    });

    test('whitespace-only string returns 0', () => {
        expect(parseAmountStrict('   ')).toBe(0);
    });

    test('null returns 0', () => {
        expect(parseAmountStrict(null)).toBe(0);
    });

    test('undefined returns 0', () => {
        expect(parseAmountStrict(undefined)).toBe(0);
    });
});

describe('parseAmountStrict — reject (the Fable-5 #10 pin cases)', () => {
    // These are the exact shapes the bug report called out. All of
    // them must return null so the caller can show a validation
    // error instead of silently sending a corrupted amount.
    test('decimal ("1099.5") rejected — used to lose the half-yen', () => {
        expect(parseAmountStrict('1099.5')).toBeNull();
    });

    test('bare "0.5" rejected', () => {
        expect(parseAmountStrict('0.5')).toBeNull();
    });

    test('locale comma ("1,099") rejected — used to parse as 1', () => {
        expect(parseAmountStrict('1,099')).toBeNull();
    });

    test('scientific notation ("1e3") rejected', () => {
        expect(parseAmountStrict('1e3')).toBeNull();
    });

    test('trailing garbage ("1099abc") rejected', () => {
        expect(parseAmountStrict('1099abc')).toBeNull();
    });

    test('leading garbage ("abc1099") rejected', () => {
        expect(parseAmountStrict('abc1099')).toBeNull();
    });

    test('negative sign ("-5") rejected — HTML min="0" was not enforced pre-fix', () => {
        expect(parseAmountStrict('-5')).toBeNull();
    });

    test('positive sign ("+5") rejected', () => {
        expect(parseAmountStrict('+5')).toBeNull();
    });

    test('interior whitespace ("10 99") rejected', () => {
        expect(parseAmountStrict('10 99')).toBeNull();
    });

    test('full-width digits ("１０９９") rejected — Rust backend expects half-width', () => {
        expect(parseAmountStrict('１０９９')).toBeNull();
    });

    test('bare period rejected', () => {
        expect(parseAmountStrict('.')).toBeNull();
    });

    test('trailing period ("1099.") rejected', () => {
        expect(parseAmountStrict('1099.')).toBeNull();
    });

    // CodeRabbit on #133 — precision-loss cases: an all-digits input
    // whose integer value is past `Number.MAX_SAFE_INTEGER` (2^53-1)
    // coerces to the nearest representable Number and silently drops
    // the low bits. The helper must reject those.
    test('max safe integer (2^53-1) accepted', () => {
        expect(parseAmountStrict('9007199254740991')).toBe(9007199254740991);
    });

    test('one past max safe integer ("9007199254740992") rejected — first unsafe int', () => {
        // Number('9007199254740992') === 9007199254740992 which is still
        // *representable* but `Number.isSafeInteger` returns false for
        // anything >= 2^53. Reject to keep the "what the user typed is
        // what the backend receives" contract.
        expect(parseAmountStrict('9007199254740992')).toBeNull();
    });

    test('unsafe integer that also loses precision ("9007199254740993") rejected', () => {
        // Number('9007199254740993') === 9007199254740992 — the low bit
        // is gone. Pre-fix this returned 9007199254740992 silently.
        expect(parseAmountStrict('9007199254740993')).toBeNull();
    });
});
