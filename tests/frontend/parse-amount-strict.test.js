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

import { parseAmountStrict } from '../../res/js/parse-amount-strict.js';

describe('parseAmountStrict — accept', () => {
    test('should return the integer when the input is a plain integer string', () => {
        expect(parseAmountStrict('1099')).toBe(1099);
    });

    test('should return 0 when the input is a single zero', () => {
        expect(parseAmountStrict('0')).toBe(0);
    });

    test('should return the same integer value when the input has leading zeros', () => {
        // "0099" is still exactly digits — accept, callers rely on
        // Number() normalising the integer value.
        expect(parseAmountStrict('0099')).toBe(99);
    });

    test('should trim the input before matching when it has surrounding whitespace', () => {
        expect(parseAmountStrict('  1099  ')).toBe(1099);
    });

    test('should return the integer when it is large but within the Number range', () => {
        expect(parseAmountStrict('999999999')).toBe(999999999);
    });
});

describe('parseAmountStrict — empty inputs default to 0', () => {
    // Preserves the `|| 0` fallback every pre-fix caller relied on.
    test('should return 0 when the input is an empty string', () => {
        expect(parseAmountStrict('')).toBe(0);
    });

    test('should return 0 when the input is only whitespace', () => {
        expect(parseAmountStrict('   ')).toBe(0);
    });

    test('should return 0 when the input is null', () => {
        expect(parseAmountStrict(null)).toBe(0);
    });

    test('should return 0 when the input is undefined', () => {
        expect(parseAmountStrict(undefined)).toBe(0);
    });
});

describe('parseAmountStrict — reject (the Fable-5 #10 pin cases)', () => {
    // These are the exact shapes the bug report called out. All of
    // them must return null so the caller can show a validation
    // error instead of silently sending a corrupted amount.
    test('should reject the input when it is a decimal ("1099.5") — used to lose the half-yen', () => {
        expect(parseAmountStrict('1099.5')).toBeNull();
    });

    test('should reject the input when it is a bare "0.5"', () => {
        expect(parseAmountStrict('0.5')).toBeNull();
    });

    test('should reject the input when it has a locale comma ("1,099") — used to parse as 1', () => {
        expect(parseAmountStrict('1,099')).toBeNull();
    });

    test('should reject the input when it is in scientific notation ("1e3")', () => {
        expect(parseAmountStrict('1e3')).toBeNull();
    });

    test('should reject the input when it has trailing garbage ("1099abc")', () => {
        expect(parseAmountStrict('1099abc')).toBeNull();
    });

    test('should reject the input when it has leading garbage ("abc1099")', () => {
        expect(parseAmountStrict('abc1099')).toBeNull();
    });

    test('should reject the input when it has a negative sign ("-5") — HTML min="0" was not enforced pre-fix', () => {
        expect(parseAmountStrict('-5')).toBeNull();
    });

    test('should reject the input when it has a positive sign ("+5")', () => {
        expect(parseAmountStrict('+5')).toBeNull();
    });

    test('should reject the input when it has interior whitespace ("10 99")', () => {
        expect(parseAmountStrict('10 99')).toBeNull();
    });

    test('should reject the input when it has full-width digits ("１０９９") — Rust backend expects half-width', () => {
        expect(parseAmountStrict('１０９９')).toBeNull();
    });

    test('should reject the input when it is a bare period', () => {
        expect(parseAmountStrict('.')).toBeNull();
    });

    test('should reject the input when it has a trailing period ("1099.")', () => {
        expect(parseAmountStrict('1099.')).toBeNull();
    });

    // CodeRabbit on #133 — precision-loss cases: an all-digits input
    // whose integer value is past `Number.MAX_SAFE_INTEGER` (2^53-1)
    // coerces to the nearest representable Number and silently drops
    // the low bits. The helper must reject those.
    test('should accept the input when it is the max safe integer (2^53-1)', () => {
        expect(parseAmountStrict('9007199254740991')).toBe(9007199254740991);
    });

    test('should reject the input when it is one past the max safe integer ("9007199254740992") — first unsafe int', () => {
        // Number('9007199254740992') === 9007199254740992 which is still
        // *representable* but `Number.isSafeInteger` returns false for
        // anything >= 2^53. Reject to keep the "what the user typed is
        // what the backend receives" contract.
        expect(parseAmountStrict('9007199254740992')).toBeNull();
    });

    test('should reject the input when it is an unsafe integer that also loses precision ("9007199254740993")', () => {
        // Number('9007199254740993') === 9007199254740992 — the low bit
        // is gone. Pre-fix this returned 9007199254740992 silently.
        expect(parseAmountStrict('9007199254740993')).toBeNull();
    });
});
