/**
 * Tax Rounding Tests
 * 
 * Tests for tax rounding calculation functionality
 */

/**
 * Apply rounding based on tax rounding type
 * (Copied from transaction-detail-management.js for testing)
 * @param {number} value - Value to round
 * @param {number} roundingType - 0: floor, 1: half-up, 2: ceil
 * @returns {number} Rounded value
 */
function applyTaxRounding(value, roundingType = 0) {
    switch (roundingType) {
        case 0: // Round down (切り捨て)
            return Math.floor(value);
        case 1: // Round half-up (四捨五入)
            return Math.round(value);
        case 2: // Round up (切り上げ)
            return Math.ceil(value);
        default:
            return Math.floor(value);
    }
}

/**
 * Test suite for tax rounding calculations
 */
export const taxRoundingTests = [
    {
        name: 'should apply floor rounding (切り捨て) when roundingType is 0',
        test: () => {
            const testCases = [
                { value: 100.1, expected: 100 },
                { value: 100.5, expected: 100 },
                { value: 100.9, expected: 100 },
                { value: 100.0, expected: 100 },
                { value: 99.99, expected: 99 }
            ];
            
            for (const testCase of testCases) {
                const result = applyTaxRounding(testCase.value, 0);
                if (result !== testCase.expected) {
                    throw new Error(`Expected floor(${testCase.value}) to be ${testCase.expected}, but got ${result}`);
                }
            }
        }
    },
    {
        name: 'should apply half-up rounding (四捨五入) when roundingType is 1',
        test: () => {
            const testCases = [
                { value: 100.1, expected: 100 },
                { value: 100.4, expected: 100 },
                { value: 100.5, expected: 101 },
                { value: 100.6, expected: 101 },
                { value: 100.9, expected: 101 },
                { value: 100.0, expected: 100 },
                { value: 99.5, expected: 100 }
            ];
            
            for (const testCase of testCases) {
                const result = applyTaxRounding(testCase.value, 1);
                if (result !== testCase.expected) {
                    throw new Error(`Expected round(${testCase.value}) to be ${testCase.expected}, but got ${result}`);
                }
            }
        }
    },
    {
        name: 'should apply ceiling rounding (切り上げ) when roundingType is 2',
        test: () => {
            const testCases = [
                { value: 100.1, expected: 101 },
                { value: 100.5, expected: 101 },
                { value: 100.9, expected: 101 },
                { value: 100.0, expected: 100 },
                { value: 99.01, expected: 100 }
            ];
            
            for (const testCase of testCases) {
                const result = applyTaxRounding(testCase.value, 2);
                if (result !== testCase.expected) {
                    throw new Error(`Expected ceil(${testCase.value}) to be ${testCase.expected}, but got ${result}`);
                }
            }
        }
    },
    {
        name: 'should default to floor rounding when roundingType is invalid',
        test: () => {
            const testCases = [
                { value: 100.5, roundingType: -1, expected: 100 },
                { value: 100.5, roundingType: 3, expected: 100 },
                { value: 100.5, roundingType: 999, expected: 100 },
                { value: 100.5, roundingType: null, expected: 100 },
                { value: 100.5, roundingType: undefined, expected: 100 }
            ];
            
            for (const testCase of testCases) {
                const result = applyTaxRounding(testCase.value, testCase.roundingType);
                if (result !== testCase.expected) {
                    throw new Error(`Expected floor(${testCase.value}) with invalid roundingType ${testCase.roundingType} to be ${testCase.expected}, but got ${result}`);
                }
            }
        }
    },
    {
        name: 'should calculate tax correctly with floor rounding (切り捨て)',
        test: () => {
            // Tax calculation: amount * rate / 100
            const testCases = [
                { amount: 1000, rate: 10, expected: 100 },  // 1000 * 0.10 = 100.0
                { amount: 333, rate: 10, expected: 33 },    // 333 * 0.10 = 33.3 -> 33
                { amount: 777, rate: 8, expected: 62 },     // 777 * 0.08 = 62.16 -> 62
                { amount: 1234, rate: 10, expected: 123 },  // 1234 * 0.10 = 123.4 -> 123
            ];
            
            for (const testCase of testCases) {
                const tax = applyTaxRounding(testCase.amount * testCase.rate / 100, 0);
                if (tax !== testCase.expected) {
                    throw new Error(`Expected tax for ¥${testCase.amount} at ${testCase.rate}% (floor) to be ¥${testCase.expected}, but got ¥${tax}`);
                }
            }
        }
    },
    {
        name: 'should calculate tax correctly with half-up rounding (四捨五入)',
        test: () => {
            const testCases = [
                { amount: 1000, rate: 10, expected: 100 },  // 1000 * 0.10 = 100.0
                { amount: 333, rate: 10, expected: 33 },    // 333 * 0.10 = 33.3 -> 33
                { amount: 335, rate: 10, expected: 34 },    // 335 * 0.10 = 33.5 -> 34
                { amount: 777, rate: 8, expected: 62 },     // 777 * 0.08 = 62.16 -> 62
                { amount: 1234, rate: 10, expected: 123 },  // 1234 * 0.10 = 123.4 -> 123
                { amount: 1235, rate: 10, expected: 124 },  // 1235 * 0.10 = 123.5 -> 124
            ];
            
            for (const testCase of testCases) {
                const tax = applyTaxRounding(testCase.amount * testCase.rate / 100, 1);
                if (tax !== testCase.expected) {
                    throw new Error(`Expected tax for ¥${testCase.amount} at ${testCase.rate}% (half-up) to be ¥${testCase.expected}, but got ¥${tax}`);
                }
            }
        }
    },
    {
        name: 'should calculate tax correctly with ceiling rounding (切り上げ)',
        test: () => {
            const testCases = [
                { amount: 1000, rate: 10, expected: 100 },  // 1000 * 0.10 = 100.0
                { amount: 333, rate: 10, expected: 34 },    // 333 * 0.10 = 33.3 -> 34
                { amount: 777, rate: 8, expected: 63 },     // 777 * 0.08 = 62.16 -> 63
                { amount: 1234, rate: 10, expected: 124 },  // 1234 * 0.10 = 123.4 -> 124
            ];
            
            for (const testCase of testCases) {
                const tax = applyTaxRounding(testCase.amount * testCase.rate / 100, 2);
                if (tax !== testCase.expected) {
                    throw new Error(`Expected tax for ¥${testCase.amount} at ${testCase.rate}% (ceil) to be ¥${testCase.expected}, but got ¥${tax}`);
                }
            }
        }
    },
    {
        name: 'should handle zero and negative values correctly',
        test: () => {
            const testCases = [
                { value: 0, roundingType: 0, expected: 0 },
                { value: 0, roundingType: 1, expected: 0 },
                { value: 0, roundingType: 2, expected: 0 },
                { value: -100.5, roundingType: 0, expected: -101 },  // floor(-100.5) = -101
                { value: -100.5, roundingType: 1, expected: -100 },  // round(-100.5) = -100
                { value: -100.5, roundingType: 2, expected: -100 },  // ceil(-100.5) = -100
            ];
            
            for (const testCase of testCases) {
                const result = applyTaxRounding(testCase.value, testCase.roundingType);
                if (result !== testCase.expected) {
                    throw new Error(`Expected applyTaxRounding(${testCase.value}, ${testCase.roundingType}) to be ${testCase.expected}, but got ${result}`);
                }
            }
        }
    },
    {
        name: 'should handle reverse tax calculation with floor rounding',
        test: () => {
            // Given tax-included amount, calculate tax-excluded amount
            // Formula: excluded = floor(included / (1 + rate/100))
            const testCases = [
                { included: 1100, rate: 10, expectedExcluded: 1000 },  // 1100 / 1.1 = 1000.0
                { included: 366, rate: 10, expectedExcluded: 332 },    // 366 / 1.1 = 332.727... -> 332
                { included: 1080, rate: 8, expectedExcluded: 1000 },   // 1080 / 1.08 = 1000.0
            ];
            
            for (const testCase of testCases) {
                const excluded = applyTaxRounding(testCase.included / (1 + testCase.rate / 100), 0);
                if (excluded !== testCase.expectedExcluded) {
                    throw new Error(`Expected excluded amount from ¥${testCase.included} at ${testCase.rate}% (floor) to be ¥${testCase.expectedExcluded}, but got ¥${excluded}`);
                }
            }
        }
    },
    {
        name: 'should handle Japanese consumption tax rates correctly',
        test: () => {
            // Common Japanese tax rates: 0%, 8%, 10%
            const testCases = [
                { amount: 1000, rate: 0, expected: 0 },     // No tax
                { amount: 1000, rate: 8, expected: 80 },    // 8% tax
                { amount: 1000, rate: 10, expected: 100 },  // 10% tax (current standard)
                { amount: 500, rate: 8, expected: 40 },     // 500 * 0.08 = 40
                { amount: 500, rate: 10, expected: 50 },    // 500 * 0.10 = 50
            ];
            
            for (const testCase of testCases) {
                const tax = applyTaxRounding(testCase.amount * testCase.rate / 100, 0);
                if (tax !== testCase.expected) {
                    throw new Error(`Expected tax for ¥${testCase.amount} at ${testCase.rate}% to be ¥${testCase.expected}, but got ¥${tax}`);
                }
            }
        }
    }
];
