import { calculateRecommendedTotal } from '../js/tax-calc.js';

// Mirror of the Rust unit tests in src/services/transaction.rs. Each case
// pins down the same contract on the JS side so both implementations stay
// in lockstep. If a case here fails, either the JS port or the Rust master
// has drifted.

const TAX_ROUND_DOWN = 0;
const TAX_ROUND_HALF_UP = 1;
const TAX_ROUND_UP = 2;

const detail = (amount, including, rate) => ({
    amount,
    amount_including_tax: including,
    tax_rate: rate
});

describe('calculateRecommendedTotal', () => {
    test('single pre-tax detail rounds down by default', () => {
        // 1000 × 1.08 = 1080.
        const details = [detail(1000, 1080, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(1080);
    });

    test('per-rate sum avoids per-detail rounding accumulation', () => {
        // The marquee bug from v1.x: floor(999 × 1.08) × 2 = 2156, but
        // floor((999 + 999) × 1.08) = 2157.
        const details = [detail(999, 1078, 8), detail(999, 1078, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(2157);
    });

    test('mixed tax rates bucket independently', () => {
        // 1000 × 1.08 + 2000 × 1.10 = 1080 + 2200.
        const details = [detail(1000, 1080, 8), detail(2000, 2200, 10)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(3280);
    });

    test('amount equal to amount_including_tax passes through (tax-included input)', () => {
        // Must NOT gross up a row the user already typed in tax-included.
        const details = [detail(216, 216, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(216);
    });

    test('tax_rate zero passes through', () => {
        const details = [detail(500, 500, 0), detail(100, null, 0)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(600);
    });

    test('half-up rounding mode', () => {
        // 999 × 1.08 = 1078.92 → half-up → 1079.
        const details = [detail(999, null, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_HALF_UP)).toBe(1079);
    });

    test('ceil rounding mode', () => {
        const details = [detail(999, null, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_UP)).toBe(1079);

        // Exact value (no fractional part) should stay put.
        const exact = [detail(1000, null, 8)];
        expect(calculateRecommendedTotal(exact, TAX_ROUND_UP)).toBe(1080);
    });

    test('mixed already-included and pre-tax in the same rate bucket', () => {
        // pre-tax 1000 grosses to 1080; already-included 300 stays as 300; total 1380.
        const details = [detail(1000, 1080, 8), detail(300, 300, 8)];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(1380);
    });

    test('empty detail list returns zero', () => {
        expect(calculateRecommendedTotal([], TAX_ROUND_DOWN)).toBe(0);
    });

    // Compatibility shim: callers that pass camelCase keys (`amountIncludingTax`,
    // `taxRate`) should still work, so the helper can be used directly with
    // payloads that came back from Tauri commands.
    test('accepts camelCase property names', () => {
        const details = [
            { amount: 1000, amountIncludingTax: 1080, taxRate: 8 }
        ];
        expect(calculateRecommendedTotal(details, TAX_ROUND_DOWN)).toBe(1080);
    });
});
