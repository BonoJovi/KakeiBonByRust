import { calculateRecommendedTotal } from '../js/tax-calc.js';

// Mirror of the Rust unit tests in src/services/transaction.rs. Each case
// pins down the same contract on the JS side so both implementations stay
// in lockstep. If a case here fails, either the JS port or the Rust master
// has drifted.

const TAX_ROUND_DOWN = 0;
const TAX_ROUND_HALF_UP = 1;
const TAX_ROUND_UP = 2;
const TAX_INCLUDED = 0;
const TAX_EXCLUDED = 1;

const detail = (amount, including, rate) => ({
    amount,
    amount_including_tax: including,
    tax_rate: rate
});

describe('calculateRecommendedTotal', () => {
    test('should round down by default when there is a single pre-tax detail', () => {
        // 1000 × 1.08 = 1080.
        const details = [detail(1000, 1080, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(1080);
    });

    test('should avoid per-detail rounding accumulation when tax is summed per rate', () => {
        // The marquee bug from v1.x: floor(999 × 1.08) × 2 = 2156, but
        // floor((999 + 999) × 1.08) = 2157.
        const details = [detail(999, 1078, 8), detail(999, 1078, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(2157);
    });

    test('should bucket each rate independently when the tax rates are mixed', () => {
        // 1000 × 1.08 + 2000 × 1.10 = 1080 + 2200.
        const details = [detail(1000, 1080, 8), detail(2000, 2200, 10)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(3280);
    });

    test('should gross up the amount regardless of amount_including_tax when the header is tax-excluded', () => {
        // amount is always tax-excluded: 200 × 1.08 = 216 whatever the
        // including column holds.
        for (const including of [216, null, 200]) {
            expect(calculateRecommendedTotal([detail(200, including, 8)], TAX_ROUND_DOWN)).toBe(216);
        }
    });

    test('should still gross up small rows when their tax rounds to zero (L1)', () => {
        // (5 + 10) × 1.08 = 16.2 → 16, not 5 + 10 = 15.
        const details = [detail(5, 5, 8), detail(10, 10, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN, TAX_EXCLUDED)).toBe(16);
    });

    test('should pass the amount through when tax_rate is zero', () => {
        const details = [detail(500, 500, 0), detail(100, null, 0)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(600);
    });

    test('should round half up when the rounding mode is half-up', () => {
        // 999 × 1.08 = 1078.92 → half-up → 1079.
        const details = [detail(999, null, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_HALF_UP)).toBe(1079);
    });

    test('should round up when the rounding mode is ceil', () => {
        const details = [detail(999, null, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_UP)).toBe(1079);

        // Exact value (no fractional part) should stay put.
        const exact = [detail(1000, null, 8)];
        expect(calculateRecommendedTotal(exact, TAX_ROUND_UP)).toBe(1080);
    });

    test('should sum amount_including_tax when the header is tax-included (H5)', () => {
        // 359 + 359 = 718; grossing up 666 × 1.08 would give 719.
        const details = [detail(333, 359, 8), detail(333, 359, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN, TAX_INCLUDED)).toBe(718);
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN, TAX_EXCLUDED)).toBe(719);
    });

    test('should derive missing tax-included prices when the header is tax-included', () => {
        // 1080 (stored) + 1000 × 1.10 (null) + 300 × 1.08 (0 sentinel) = 2504.
        const details = [detail(1000, 1080, 8), detail(1000, null, 10), detail(300, 0, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN, TAX_INCLUDED)).toBe(2504);
    });

    test('should return zero when the detail list is empty', () => {
        expect(calculateRecommendedTotal([], TAX_ROUND_DOWN)).toBe(0);
    });

    // Compatibility shim: callers that pass camelCase keys (`amountIncludingTax`,
    // `taxRate`) should still work, so the helper can be used directly with
    // payloads that came back from Tauri commands.
    test('should accept the details when they use camelCase property names', () => {
        const details = [
            { amount: 1000, amountIncludingTax: 1080, taxRate: 8 }
        ];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(1080);
    });
});
