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

function applyTaxRounding(value, roundingType) {
    switch (roundingType) {
        case 0: return Math.floor(value);
        case 1: return Math.round(value);
        case 2: return Math.ceil(value);
        default: return Math.floor(value);
    }
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
        const excluded = parseFloat(amountExcludingTax.value) || 0;
        const rate = parseFloat(taxRate.value) || 0;
        onCleared();
        lastTaxInputField = 'excluding';

        const tax = applyTaxRounding(excluded * rate / 100, getRoundingType());
        const included = excluded + tax;
        taxAmount.value = tax;
        amountIncludingTax.value = included || '';
    }

    function calcFromIncluding() {
        const included = parseFloat(amountIncludingTax.value) || 0;
        const rate = parseFloat(taxRate.value) || 0;
        onCleared();
        lastTaxInputField = 'including';

        if (!included) {
            amountExcludingTax.value = '';
            taxAmount.value = 0;
            return;
        }
        if (rate === 0) {
            amountExcludingTax.value = included || '';
            taxAmount.value = 0;
            return;
        }

        const roundingType = getRoundingType();
        const excluded = applyTaxRounding(included / (1 + rate / 100), roundingType);
        const tax = included - excluded;

        const taxReverse = applyTaxRounding(excluded * rate / 100, roundingType);
        const includedReverse = excluded + taxReverse;
        if (includedReverse !== included) {
            onDiscrepancy({ userInput: included, calculated: includedReverse });
        }

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
