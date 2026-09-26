import {
    TAX_ROUND_HALF_UP,
    TAX_ROUND_UP,
    TAX_INCLUDED,
    TAX_EXCLUDED,
} from './consts.js';

/**
 * Pure-JS mirror of `services::transaction::calculate_recommended_total_with_settings`
 * in the Rust backend. Used by the header edit modal so changes to the
 * rounding type / tax-included type can be reflected in the total field
 * immediately, without round-tripping the backend, while staying
 * bit-for-bit consistent with the value the backend would compute.
 *
 * A detail's `amount` is always tax-excluded (owner decision 2026-09-26);
 * its tax-included price is `amount_including_tax`. Both implementations
 * follow the same rules:
 *
 * - Tax-included header: sum each row's tax-included price verbatim. A row
 *   missing it (null, or the 0 empty-input sentinel on a non-zero amount)
 *   derives it as `amount + round(amount * rate / 100)` under the header
 *   rounding.
 * - Tax-excluded header: per tax rate, sum `amount` first, gross up by
 *   `(100 + rate) / 100` second, then round once according to the chosen
 *   mode. Rounding once per rate (not once per detail) is what eliminates
 *   the v1.x accumulation error. There is no "amount === amount_including_tax
 *   means already included" short-circuit: a small row whose tax rounds to
 *   0 yen would match it and skip the gross-up.
 *
 * @param {Array<{amount: number, amount_including_tax: number|null, tax_rate: number}>} details
 *   Detail rows in the same shape `TRANSACTIONS_DETAIL` exposes (camelCase /
 *   snake_case both work — the function reads `tax_rate` and falls back to
 *   `taxRate`).
 * @param {number} taxRoundingType - 0=floor, 1=half-away-from-zero, 2=ceil.
 *   Must match the constants in `consts.js` and `consts.rs`.
 * @param {number} [taxIncludedType=TAX_EXCLUDED] - 0=tax-included, 1=tax-excluded.
 * @returns {number} The recommended `TOTAL_AMOUNT` as an integer yen value.
 */
export function calculateRecommendedTotal(details, taxRoundingType, taxIncludedType = TAX_EXCLUDED) {
    if (taxIncludedType === TAX_INCLUDED) {
        let total = 0;
        for (const d of details) {
            const amount = d.amount;
            const including = d.amount_including_tax ?? d.amountIncludingTax ?? null;
            const rate = d.tax_rate ?? d.taxRate;
            const missing = including === null || (including === 0 && amount > 0);
            total += missing
                ? amount + roundHundredths(amount * rate, taxRoundingType)
                : including;
        }
        return total;
    }

    // pretax amount sums keyed by tax rate
    const byRate = new Map();
    for (const d of details) {
        const rate = d.tax_rate ?? d.taxRate;
        byRate.set(rate, (byRate.get(rate) || 0) + d.amount);
    }

    let total = 0;
    for (const [rate, pretax] of byRate) {
        total += roundHundredths(pretax * (100 + rate), taxRoundingType);
    }
    return total;
}

/**
 * Round a value in 1/100ths of a yen back to whole yen. Inputs are
 * non-negative, so `(v + 50) / 100` is half-away-from-zero (matching SQLite
 * ROUND() and Rust's `(v + 50) / 100`) and `(v + 99) / 100` is ceil.
 *
 * @param {number} hundredths
 * @param {number} taxRoundingType - 0=floor, 1=half-up, 2=ceil (else floor)
 * @returns {number}
 */
function roundHundredths(hundredths, taxRoundingType) {
    switch (taxRoundingType) {
        case TAX_ROUND_HALF_UP: return Math.floor((hundredths + 50) / 100);
        case TAX_ROUND_UP: return Math.floor((hundredths + 99) / 100);
        default: return Math.floor(hundredths / 100);
    }
}
