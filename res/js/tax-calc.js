/**
 * Pure-JS mirror of `services::transaction::calculate_recommended_total` in
 * the Rust backend. Used by the header edit modal so changes to the rounding
 * type / tax-included type can be reflected in the total field immediately,
 * without round-tripping the backend, while staying bit-for-bit consistent
 * with the value the backend would compute.
 *
 * Both implementations follow the same three rules:
 *
 * 1. A detail row counts as "already tax-included" when `tax_rate === 0` or
 *    `amount === amount_including_tax`. Those rows are summed verbatim.
 * 2. Other rows are pre-tax: per tax rate, sum first, gross up by
 *    `(100 + rate) / 100` second, then round once according to the chosen
 *    mode. Rounding once per rate (not once per detail) is what eliminates
 *    the v1.x accumulation error.
 * 3. The integer slices are summed with no further rounding.
 *
 * @param {Array<{amount: number, amount_including_tax: number|null, tax_rate: number}>} details
 *   Detail rows in the same shape `TRANSACTIONS_DETAIL` exposes (camelCase /
 *   snake_case both work — the function reads `tax_rate` and falls back to
 *   `taxRate`).
 * @param {number} taxRoundingType - 0=floor, 1=half-away-from-zero, 2=ceil.
 *   Must match the constants in `consts.js` and `consts.rs`.
 * @returns {number} The recommended `TOTAL_AMOUNT` as an integer yen value.
 */
export function calculateRecommendedTotal(details, taxRoundingType) {
    const TAX_ROUND_DOWN = 0;
    const TAX_ROUND_HALF_UP = 1;
    const TAX_ROUND_UP = 2;

    // (alreadyIncluded, pretax) sums keyed by tax rate
    const byRate = new Map();

    for (const d of details) {
        const amount = d.amount;
        const including = d.amount_including_tax ?? d.amountIncludingTax ?? null;
        const rate = d.tax_rate ?? d.taxRate;

        const isAlreadyIncluded = rate === 0
            || (including !== null && including === amount);

        const entry = byRate.get(rate) || { already: 0, pretax: 0 };
        if (isAlreadyIncluded) {
            entry.already += amount;
        } else {
            entry.pretax += amount;
        }
        byRate.set(rate, entry);
    }

    let total = 0;
    for (const [rate, { already, pretax }] of byRate) {
        const grossed = pretax * (100 + rate);
        let pretaxGrossed;
        switch (taxRoundingType) {
            case TAX_ROUND_DOWN:
                pretaxGrossed = Math.floor(grossed / 100);
                break;
            case TAX_ROUND_HALF_UP:
                // grossed + 50 then integer-divide by 100 = half-away-from-zero
                // for positive integers, matching SQLite ROUND() and Rust's
                // (grossed + 50) / 100 used by calculate_recommended_total.
                pretaxGrossed = Math.floor((grossed + 50) / 100);
                break;
            case TAX_ROUND_UP:
                pretaxGrossed = Math.floor((grossed + 99) / 100);
                break;
            default:
                pretaxGrossed = Math.floor(grossed / 100);
        }
        total += already + pretaxGrossed;
    }

    return total;
}
