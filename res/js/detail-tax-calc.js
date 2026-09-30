/**
 * Shared tax-calculation helpers for detail forms.
 *
 * Wires bidirectional auto-calculation between the tax-excluded amount,
 * tax-included amount, and tax amount fields, plus recalculation when the
 * tax rate changes. Used by both the normal transaction detail screen and
 * the recurring-rule template form.
 *
 * Numerical conventions match `services::transaction::*` on the Rust side:
 * - tax = round(excluded * rate / 100, roundingType)
 * - excluded = round(included * 100 / (100 + rate), roundingType)
 * - rounding types: 0 = floor, 1 = half-up, 2 = ceil
 *
 * A tax-included amount typed by the user is always kept as typed
 * (latent-scan2 T1/T2); see `calculateFromIncluding`.
 */

export function applyTaxRounding(value, roundingType) {
    switch (roundingType) {
        case 0: return Math.floor(value);
        case 1: return Math.round(value);
        case 2: return Math.ceil(value);
        default: return Math.floor(value);
    }
}

/**
 * Pure helper: split a tax-included input into (excluded, tax) so that
 * `excluded + tax` is always the typed amount.
 *
 * When an integer `excluded` exists whose `round(excluded * rate)` lands
 * exactly on the typed amount, that split is used, so the row matches the
 * tax-excluded formula as well (Fable-5 #8). About 9 % of prices at 10 %
 * (e.g. 1,000 yen under FLOOR) have no such split; for those the tax is
 * carved out of the typed amount, `tax = included - excluded`, instead of
 * forcing the price to a neighbour (latent-scan2 T2, owner decision
 * 2026-10-01). Tax-included headers total the rows' tax-included amounts
 * verbatim, so the typed price is what gets aggregated.
 *
 * @param {number} includedInput  tax-included amount typed by the user (>=0)
 * @param {number} rate           tax rate in percent (e.g. 8, 10; 0 means no tax)
 * @param {number} roundingType   0=floor, 1=half-up, 2=ceil
 * @returns {{ excluded: number, tax: number }}
 */
export function calculateFromIncluding(includedInput, rate, roundingType) {
    if (!includedInput) {
        return { excluded: 0, tax: 0 };
    }
    if (rate === 0) {
        return { excluded: includedInput, tax: 0 };
    }

    const base = applyTaxRounding(includedInput * 100 / (100 + rate), roundingType);

    // CodeRabbit on #129 — pick the `excluded` candidate that lets the
    // user keep their typed tax-included value, when one exists. The
    // two-step rounding (division then multiplication) can produce a
    // base that misses by 1 in either direction; e.g. 101 円 at 10 %
    // under FLOOR gives base=91 and 91+9=100, but base+1=92 gives
    // 92+9=101 — that second candidate matches the typed input, so
    // there's no need to auto-adjust it down to 100. The scan is
    // `[base, base+1, base-1]` — the base wins ties (matches the
    // configured rounding mode), then the "bumped up" variant, then
    // "bumped down". Anything wider than ±1 can't help: the split
    // stays within one integer of the exact real-number result.
    for (const delta of [0, 1, -1]) {
        const cand = base + delta;
        if (cand < 0) continue;
        const candTax = applyTaxRounding(cand * rate / 100, roundingType);
        if (cand + candTax === includedInput) {
            return { excluded: cand, tax: candTax };
        }
    }

    // No integer split reproduces the typed input under this rounding
    // mode: keep the typed amount and carve the tax out of it.
    return { excluded: base, tax: includedInput - base };
}

/**
 * Pure helper: derive the (tax, included) pair from a tax-excluded input.
 * Symmetric with `calculateFromIncluding`; this branch has always been
 * internally consistent because it starts from the authoritative formula.
 *
 * @param {number} excludedInput  tax-excluded amount typed by the user (>=0)
 * @param {number} rate           tax rate in percent
 * @param {number} roundingType   0=floor, 1=half-up, 2=ceil
 * @returns {{ tax: number, included: number }}
 */
export function calculateFromExcluding(excludedInput, rate, roundingType) {
    if (!excludedInput) {
        return { tax: 0, included: 0 };
    }
    const tax = applyTaxRounding(excludedInput * rate / 100, roundingType);
    return { tax, included: excludedInput + tax };
}

/**
 * Attach auto-calculation listeners.
 *
 * @param {object} elements
 * @param {HTMLSelectElement} elements.taxRate            <select> with rate %
 * @param {HTMLInputElement}  elements.amountExcludingTax <input type=number>
 * @param {HTMLInputElement}  elements.amountIncludingTax <input type=number>
 * @param {HTMLInputElement}  elements.taxAmount          <input type=number readonly>
 * @param {object} [options]
 * @param {() => number} [options.getRoundingType]        returns 0/1/2 (default: () => 0)
 * @returns {{getLastEditedField: () => ('excluding'|'including'|null),
 *            recalculate: () => void}}
 */
export function setupTaxCalculationListeners(elements, options = {}) {
    const { taxRate, amountExcludingTax, amountIncludingTax, taxAmount } = elements;
    const getRoundingType = options.getRoundingType || (() => 0);

    let lastTaxInputField = null;

    function calcFromExcluding() {
        const excludedInput = parseFloat(amountExcludingTax.value) || 0;
        const rate = parseFloat(taxRate.value) || 0;
        lastTaxInputField = 'excluding';

        const { tax, included } = calculateFromExcluding(excludedInput, rate, getRoundingType());
        taxAmount.value = tax;
        amountIncludingTax.value = included || '';
    }

    function calcFromIncluding() {
        const includedInput = parseFloat(amountIncludingTax.value) || 0;
        const rate = parseFloat(taxRate.value) || 0;
        lastTaxInputField = 'including';

        if (!includedInput) {
            amountExcludingTax.value = '';
            taxAmount.value = 0;
            return;
        }

        // The tax-included field is never written back: rewriting it on
        // every `input` event replaced the prefix being typed (e.g. "10"
        // became 9 on the way to 100), so the saved price was wrong
        // (latent-scan2 T1).
        const { excluded, tax } =
            calculateFromIncluding(includedInput, rate, getRoundingType());
        taxAmount.value = tax;
        amountExcludingTax.value = excluded || '';
    }

    function recalculateUsingLastField() {
        if (lastTaxInputField === 'including' && amountIncludingTax.value) {
            calcFromIncluding();
        } else if (lastTaxInputField === 'excluding' && amountExcludingTax.value) {
            calcFromExcluding();
        } else if (amountExcludingTax.value) {
            calcFromExcluding();
        } else if (amountIncludingTax.value) {
            calcFromIncluding();
        }
    }

    amountExcludingTax.addEventListener('input', calcFromExcluding);
    amountIncludingTax.addEventListener('input', calcFromIncluding);
    taxRate.addEventListener('change', recalculateUsingLastField);

    return {
        getLastEditedField: () => lastTaxInputField,
        recalculate: recalculateUsingLastField,
    };
}
