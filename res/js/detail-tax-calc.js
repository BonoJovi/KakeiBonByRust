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
 * - excluded = round(included / (1 + rate / 100), roundingType)
 * - rounding types: 0 = floor, 1 = half-up, 2 = ceil
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
 * Pure helper: derive the (excluded, tax) pair from a tax-included input,
 * always producing a self-consistent tax-included figure `excluded + tax`.
 *
 * Fable-5 review #8 — the historic shape `tax = included - excluded` could
 * leave `tax != round(excluded * rate)` under FLOOR / CEIL rounding, so a
 * transaction detail row would carry three inconsistent numbers to the DB
 * (and the aggregation pipeline then produced a fourth one for the same
 * row). The tax value now always comes from the authoritative
 * `round(excluded * rate)` formula, and the returned `includedCorrected`
 * is `excluded + tax`; when it differs from the input, the caller has a
 * clean signal to tell the user "adjusted to N".
 *
 * @param {number} includedInput  tax-included amount typed by the user (>=0)
 * @param {number} rate           tax rate in percent (e.g. 8, 10; 0 means no tax)
 * @param {number} roundingType   0=floor, 1=half-up, 2=ceil
 * @returns {{ excluded: number, tax: number, includedCorrected: number }}
 */
export function calculateFromIncluding(includedInput, rate, roundingType) {
    if (!includedInput) {
        return { excluded: 0, tax: 0, includedCorrected: 0 };
    }
    if (rate === 0) {
        return { excluded: includedInput, tax: 0, includedCorrected: includedInput };
    }

    const excluded = applyTaxRounding(includedInput / (1 + rate / 100), roundingType);
    const tax = applyTaxRounding(excluded * rate / 100, roundingType);
    const includedCorrected = excluded + tax;
    return { excluded, tax, includedCorrected };
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
 * @param {(d: {userInput: number, calculated: number}) => void} [options.onRoundingDiscrepancy]
 *        called when round-trip tax-included reconstruction doesn't match user input
 * @param {() => void} [options.onCalculationCleared]     called when fields are cleared / no discrepancy
 * @returns {{getLastEditedField: () => ('excluding'|'including'|null),
 *            recalculate: () => void}}
 */
export function setupTaxCalculationListeners(elements, options = {}) {
    const { taxRate, amountExcludingTax, amountIncludingTax, taxAmount } = elements;
    const getRoundingType = options.getRoundingType || (() => 0);
    const onDiscrepancy = options.onRoundingDiscrepancy || (() => {});
    const onCleared = options.onCalculationCleared || (() => {});

    let lastTaxInputField = null;

    function calcFromExcluding() {
        const excludedInput = parseFloat(amountExcludingTax.value) || 0;
        const rate = parseFloat(taxRate.value) || 0;
        onCleared();
        lastTaxInputField = 'excluding';

        const { tax, included } = calculateFromExcluding(excludedInput, rate, getRoundingType());
        taxAmount.value = tax;
        amountIncludingTax.value = included || '';
    }

    function calcFromIncluding() {
        const includedInput = parseFloat(amountIncludingTax.value) || 0;
        const rate = parseFloat(taxRate.value) || 0;
        onCleared();
        lastTaxInputField = 'including';

        if (!includedInput) {
            amountExcludingTax.value = '';
            taxAmount.value = 0;
            return;
        }

        const { excluded, tax, includedCorrected } =
            calculateFromIncluding(includedInput, rate, getRoundingType());

        // Fable-5 #8 — historic shape was `tax = included - excluded`
        // followed by a warning-only comparison against
        // `round(excluded * rate)`. That could persist THREE inconsistent
        // numbers to the DB (AMOUNT / TAX_AMOUNT / AMOUNT_INCLUDING_TAX)
        // and the aggregation pipeline then produced a FOURTH number for
        // the same row. `calculateFromIncluding` now derives `tax` from
        // the authoritative formula, so the three saved numbers stay
        // self-consistent (`excluded + tax == includedCorrected`) and
        // match what `services::transaction::calculate_recommended_total`
        // recomputes on the Rust side. When the correction changes the
        // typed input we fire `onDiscrepancy` so the caller can tell the
        // user "adjusted to N".
        if (includedCorrected !== includedInput) {
            onDiscrepancy({ userInput: includedInput, calculated: includedCorrected });
        }

        taxAmount.value = tax;
        amountExcludingTax.value = excluded || '';
        // Rewrite the tax-included input so the on-screen number matches
        // what will be saved (and what the aggregation will later
        // recompute).
        amountIncludingTax.value = includedCorrected || '';
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
