/**
 * Strict integer parser for money / count / rate fields (Fable-5 #10).
 *
 * The app's transaction-detail-management, transaction-management, and
 * recurring-rule forms all used to read money fields with
 * `parseInt(el.value) || 0`. `parseInt` is a lenient parser that
 * silently accepts:
 *   - decimals ("1099.5" → 1099 — half-yen loss on submit)
 *   - locale commas ("1,099" → 1 — 99% off the intended value)
 *   - trailing garbage ("1099abc" → 1099)
 *   - scientific notation ("1e3" → 1)
 *   - signed values ("-5" → -5, bypassing the `min="0"` HTML guard)
 *
 * `parseAmountStrict` accepts the trimmed input only when it is
 * exactly one or more decimal digits — no sign, no decimal point, no
 * separator, no exponent, no trailing text. Empty input (including
 * whitespace-only) returns 0 to preserve the `|| 0` fallback every
 * pre-fix caller relied on. Anything else returns `null`, and the
 * caller shows a field-level validation error so the user sees the
 * corruption before the invoke lands.
 *
 * The tests in `res/tests/parse-amount-strict.test.js` pin the full
 * accept/reject table.
 *
 * @param {string|null|undefined} raw - `input.value` from a money field.
 * @returns {number|null} integer on accept, null on reject (empty
 *                        input returns 0).
 */
export function parseAmountStrict(raw) {
    if (raw == null) return 0;
    const trimmed = String(raw).trim();
    if (trimmed === '') return 0;
    if (!/^\d+$/.test(trimmed)) return null;
    const value = Number(trimmed);
    // CodeRabbit on #133 — an all-digits input past 2^53-1 (e.g.
    // "9007199254740993") coerces to the nearest representable
    // Number and loses precision silently. Reject it here so the
    // user sees the validation error rather than a corrupted
    // amount landing in the backend. `Number.isSafeInteger` also
    // returns false for `Infinity`, so the check doubles as a
    // belt-and-suspenders guard even though `Infinity` can't reach
    // this line under `/^\d+$/`.
    if (!Number.isSafeInteger(value)) return null;
    return value;
}
