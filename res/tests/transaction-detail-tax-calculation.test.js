/**
 * Transaction Detail Tax Calculation Tests
 * 
 * Tests for tax calculation logic including:
 * - Tax-excluding to tax-including calculation
 * - Tax-including to tax-excluding calculation
 * - Rounding error detection and warnings
 */

import { describe, it, expect, beforeEach } from '@jest/globals';
import {
    applyTaxRounding,
    calculateFromIncluding,
    calculateFromExcluding,
} from '../js/detail-tax-calc.js';

describe('Transaction Detail Tax Calculation Tests', () => {
    
    describe('Tax-excluding to Tax-including Calculation', () => {
        
        it('should calculate tax-including amount correctly with 10% tax rate', () => {
            const excludingTax = 1000;
            const taxRate = 10;
            
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            
            expect(taxAmount).toBe(100);
            expect(includingTax).toBe(1100);
        });
        
        it('should calculate tax-including amount correctly with 8% tax rate', () => {
            const excludingTax = 1000;
            const taxRate = 8;
            
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            
            expect(taxAmount).toBe(80);
            expect(includingTax).toBe(1080);
        });
        
        it('should handle rounding down correctly (floor)', () => {
            const excludingTax = 333;
            const taxRate = 10;
            
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            
            expect(taxAmount).toBe(33); // 33.3 -> 33
            expect(includingTax).toBe(366);
        });
        
        it('should calculate zero tax for 0% tax rate', () => {
            const excludingTax = 1000;
            const taxRate = 0;
            
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            
            expect(taxAmount).toBe(0);
            expect(includingTax).toBe(1000);
        });
        
    });
    
    describe('Tax-including to Tax-excluding Calculation', () => {
        
        it('should calculate tax-excluding amount correctly with 10% tax rate', () => {
            const includingTax = 1100;
            const taxRate = 10;
            
            const excludingTax = Math.floor(includingTax / (1 + taxRate / 100));
            const taxAmount = includingTax - excludingTax;
            
            // Due to floating point precision: 1100 / 1.1 = 999.999...
            expect(excludingTax).toBe(999);
            expect(taxAmount).toBe(101);
        });
        
        it('should calculate tax-excluding amount correctly with 8% tax rate', () => {
            const includingTax = 1080;
            const taxRate = 8;
            
            const excludingTax = Math.floor(includingTax / (1 + taxRate / 100));
            const taxAmount = includingTax - excludingTax;
            
            // Due to floating point precision: 1080 / 1.08 = 999.999...
            expect(excludingTax).toBe(999);
            expect(taxAmount).toBe(81);
        });
        
        it('should handle rounding down correctly (floor)', () => {
            const includingTax = 366;
            const taxRate = 10;
            
            const excludingTax = Math.floor(includingTax / (1 + taxRate / 100));
            const taxAmount = includingTax - excludingTax;
            
            expect(excludingTax).toBe(332); // 332.727... -> 332
            expect(taxAmount).toBe(34);
        });
        
    });
    
    describe('Rounding Error Detection', () => {
        
        it('should detect rounding error when recalculating from tax-excluding', () => {
            // User inputs tax-including: 366
            const userInputIncluding = 366;
            const taxRate = 10;
            
            // Calculate tax-excluding
            const calculatedExcluding = Math.floor(userInputIncluding / (1 + taxRate / 100));
            // 366 / 1.1 = 332.727... -> 332
            
            // Recalculate tax-including from calculated tax-excluding
            const taxAmount = Math.floor(calculatedExcluding * taxRate / 100);
            const recalculatedIncluding = calculatedExcluding + taxAmount;
            // 332 + 33 = 365
            
            // Should detect 1 yen difference
            expect(calculatedExcluding).toBe(332);
            expect(recalculatedIncluding).toBe(365);
            expect(userInputIncluding).not.toBe(recalculatedIncluding);
            expect(Math.abs(userInputIncluding - recalculatedIncluding)).toBe(1);
        });
        
        it('should not show warning when calculation is accurate', () => {
            // Use amount that works perfectly with floor: 330 / 1.1 = 300
            const userInputIncluding = 330;
            const taxRate = 10;
            
            // Calculate tax-excluding
            const calculatedExcluding = Math.floor(userInputIncluding / (1 + taxRate / 100));
            
            // Recalculate tax-including
            const taxAmount = Math.floor(calculatedExcluding * taxRate / 100);
            const recalculatedIncluding = calculatedExcluding + taxAmount;
            
            // Should match exactly (330 / 1.1 = 300, 300 * 0.1 = 30, 300 + 30 = 330)
            expect(calculatedExcluding).toBe(300);
            expect(recalculatedIncluding).toBe(330);
            expect(userInputIncluding).toBe(recalculatedIncluding);
        });
        
        it('should detect rounding error with 8% tax rate', () => {
            // User inputs tax-including: 325
            const userInputIncluding = 325;
            const taxRate = 8;
            
            // Calculate tax-excluding
            const calculatedExcluding = Math.floor(userInputIncluding / (1 + taxRate / 100));
            // 325 / 1.08 = 300.925... -> 300
            
            // Recalculate tax-including
            const taxAmount = Math.floor(calculatedExcluding * taxRate / 100);
            const recalculatedIncluding = calculatedExcluding + taxAmount;
            // 300 + 24 = 324
            
            // Should detect 1 yen difference
            expect(calculatedExcluding).toBe(300);
            expect(recalculatedIncluding).toBe(324);
            expect(userInputIncluding).not.toBe(recalculatedIncluding);
            expect(Math.abs(userInputIncluding - recalculatedIncluding)).toBe(1);
        });
        
    });
    
    describe('Edge Cases', () => {
        
        it('should handle large amounts correctly', () => {
            const excludingTax = 999999999; // About 1 billion yen
            const taxRate = 10;
            
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            
            expect(taxAmount).toBe(99999999);
            expect(includingTax).toBe(1099999998);
        });
        
        it('should handle small amounts correctly', () => {
            const excludingTax = 1;
            const taxRate = 10;
            
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            
            expect(taxAmount).toBe(0); // 0.1 -> 0
            expect(includingTax).toBe(1);
        });
        
        it('should handle amounts that result in exact division', () => {
            // Use numbers that work perfectly with floor: 300 -> 330 -> 300
            const excludingTax = 300;
            const taxRate = 10;
            
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            // 300 + 30 = 330
            
            // Reverse calculation should match
            const reversedExcluding = Math.floor(includingTax / (1 + taxRate / 100));
            // 330 / 1.1 = 300
            expect(reversedExcluding).toBe(excludingTax);
        });
        
        it('should handle amounts with fractional tax results', () => {
            const excludingTax = 777;
            const taxRate = 10;
            
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            
            expect(taxAmount).toBe(77); // 77.7 -> 77
            expect(includingTax).toBe(854);
            
            // Verify reverse calculation detects error
            const reversedExcluding = Math.floor(includingTax / (1 + taxRate / 100));
            expect(reversedExcluding).toBe(776); // Not 777
        });
        
    });
    
    describe('Multiple Tax Rates', () => {
        
        const testCases = [
            { rate: 5, excluding: 1000, expectedTax: 50, expectedIncluding: 1050 },
            { rate: 8, excluding: 1000, expectedTax: 80, expectedIncluding: 1080 },
            { rate: 10, excluding: 1000, expectedTax: 100, expectedIncluding: 1100 },
        ];
        
        testCases.forEach(({ rate, excluding, expectedTax, expectedIncluding }) => {
            it(`should calculate correctly with ${rate}% tax rate`, () => {
                const taxAmount = Math.floor(excluding * rate / 100);
                const includingTax = excluding + taxAmount;

                expect(taxAmount).toBe(expectedTax);
                expect(includingTax).toBe(expectedIncluding);
            });
        });

    });

    // ========================================================================
    // Fable-5 review #8 — three-value self-consistency pins.
    //
    // The historic `calcFromIncluding` shape assigned `tax = included -
    // excluded` and then warned (but did NOT correct) when it disagreed
    // with the authoritative `round(excluded * rate)`. That left THREE
    // inconsistent numbers on the DB row (AMOUNT / TAX_AMOUNT /
    // AMOUNT_INCLUDING_TAX) and the aggregation pipeline produced a
    // FOURTH one downstream. The new pure helper always derives `tax`
    // from the authoritative formula and returns `includedCorrected =
    // excluded + tax`; the caller rewrites the tax-included input to
    // that corrected value, so the DB never sees the inconsistent
    // triple.
    // ========================================================================
    describe('calculateFromIncluding — three-value self-consistency (Fable-5 #8)', () => {

        it('preserves the typed input under FLOOR when a base±1 candidate reproduces it', () => {
            // The canonical Fable-5 #8 scenario: user types 101 with 10 % / FLOOR.
            // Pre-fix: `tax = included - excluded` left the DB with three
            // inconsistent numbers (91, 10, 101). CodeRabbit on #129 pointed
            // out that just picking base=91 and correcting the input down to
            // 100 is unnecessary — base+1=92 gives 92 + floor(92 * 0.10) = 101,
            // reproducing the typed input exactly. The helper now scans
            // `[base, base+1, base-1]` and prefers the candidate that matches
            // the typed input, so this common shape stays at 101 円.
            const { excluded, tax, includedCorrected } =
                calculateFromIncluding(/*includedInput*/ 101, /*rate*/ 10, /*floor*/ 0);

            expect(excluded).toBe(92);
            expect(tax).toBe(9);
            expect(includedCorrected).toBe(101);
            expect(excluded + tax).toBe(includedCorrected);
        });

        it('preserves the typed input under CEIL by picking base-1 (91, not the ceil base 92)', () => {
            // Same typed 101, but with CEIL. base = ceil(101/1.1) = 92, and
            // 92 + ceil(92*0.1) = 102 (does NOT match). base-1 = 91 with
            // ceil(91*0.1) = 10 yields 101 — pick that so the input stays.
            const { excluded, tax, includedCorrected } =
                calculateFromIncluding(101, 10, /*ceil*/ 2);

            expect(excluded).toBe(91);
            expect(tax).toBe(10);
            expect(includedCorrected).toBe(101);
        });

        it('leaves the tax-included input untouched when the split is already exact', () => {
            // 330 = 300 + 30 under FLOOR + 10 %; base wins immediately.
            const { excluded, tax, includedCorrected } =
                calculateFromIncluding(330, 10, 0);

            expect(excluded).toBe(300);
            expect(tax).toBe(30);
            expect(includedCorrected).toBe(330);
        });

        it('produces consistent numbers under half-up rounding when the base already matches', () => {
            // 325 / 1.08 ≈ 300.925 → half-up 301, tax = round(301 * 0.08) = 24,
            // 301 + 24 = 325 — base itself matches, no candidate scan needed.
            const { excluded, tax, includedCorrected } =
                calculateFromIncluding(325, 8, /*half-up*/ 1);

            expect(excluded).toBe(301);
            expect(tax).toBe(24);
            expect(includedCorrected).toBe(325);
        });

        it('falls back to the base pair when even base±1 cannot reproduce the input', () => {
            // 1000 円 at 10 % / FLOOR: base=909, 909+90=999 ≠ 1000; base+1=910,
            // 910+91=1001 ≠ 1000; base-1=908, 908+90=998 ≠ 1000. No candidate
            // fits, so the helper falls back to (base, tax_of_base,
            // base+tax_of_base) and the caller rewrites the displayed
            // tax-included value to 999.
            const { excluded, tax, includedCorrected } =
                calculateFromIncluding(1000, 10, 0);

            expect(excluded).toBe(909);
            expect(tax).toBe(90);
            expect(includedCorrected).toBe(999);
            expect(excluded + tax).toBe(includedCorrected);
        });

        it('returns zeros for zero input', () => {
            expect(calculateFromIncluding(0, 10, 0))
                .toEqual({ excluded: 0, tax: 0, includedCorrected: 0 });
        });

        it('treats a 0 % rate as identity (no tax carved out)', () => {
            expect(calculateFromIncluding(500, 0, 0))
                .toEqual({ excluded: 500, tax: 0, includedCorrected: 500 });
        });
    });

    describe('calculateFromExcluding — helper covers the excluded-input side', () => {

        it('applies floor rounding to fractional tax', () => {
            // 333 * 0.10 = 33.3 → floor 33, included = 366.
            expect(calculateFromExcluding(333, 10, 0))
                .toEqual({ tax: 33, included: 366 });
        });

        it('returns zeros for zero input', () => {
            expect(calculateFromExcluding(0, 10, 0)).toEqual({ tax: 0, included: 0 });
        });
    });

    describe('applyTaxRounding — three modes plus a defensive default', () => {
        // Sanity for the low-level helper used by both branches above.
        // These tests exercise positive inputs only — the transaction
        // save path validates `amount >= 0` before any of this runs, so
        // the negative-input semantics of `Math.round` (which rounds
        // toward +Infinity for .5, not away from zero) are outside the
        // helper's input contract. Documented explicitly here after
        // CodeRabbit on #129 flagged the pre-fix wording as ambiguous.
        it('floor is the default for unknown rounding types', () => {
            expect(applyTaxRounding(9.9, /*unknown*/ 99)).toBe(9);
        });
        it('half-up on a positive .5 rounds up to the next integer', () => {
            expect(applyTaxRounding(9.5, 1)).toBe(10);
        });
        it('ceil bumps a fractional part up', () => {
            expect(applyTaxRounding(9.01, 2)).toBe(10);
        });
    });

});
