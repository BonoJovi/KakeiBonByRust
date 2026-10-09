/**
 * Transaction Detail Tax Calculation Tests
 * 
 * Tests for the tax helpers in res/js/detail-tax-calc.js:
 * - calculateFromExcluding: tax-excluded amount to tax and tax-included amount
 * - calculateFromIncluding: tax-included amount to tax-excluded amount and tax
 * - applyTaxRounding: the three rounding types (0 = floor, 1 = half-up, 2 = ceil)
 */

import { describe, it, expect, beforeEach } from '@jest/globals';
import {
    applyTaxRounding,
    calculateFromIncluding,
    calculateFromExcluding,
} from '../../res/js/detail-tax-calc.js';

describe('Transaction Detail Tax Calculation Tests', () => {
    
    // ========================================================================
    // Tax rates, limits and rounding types on both sides. These call the
    // real helpers in res/js/detail-tax-calc.js (they replace tests that
    // re-did the arithmetic inline, some with a formula the app no longer
    // uses).
    // ========================================================================
    describe('calculateFromExcluding — tax rates, limits and rounding types', () => {

        it.each([
            { excluded: 1000, rate: 10, tax: 100, included: 1100 },
            { excluded: 1000, rate: 8, tax: 80, included: 1080 },
            { excluded: 1000, rate: 5, tax: 50, included: 1050 },
            { excluded: 1000, rate: 0, tax: 0, included: 1000 },
            { excluded: 1000, rate: 100, tax: 1000, included: 2000 },
        ])('should add $tax yen of tax when $excluded yen is taxed at $rate%', ({ excluded, rate, tax, included }) => {
            expect(calculateFromExcluding(excluded, rate, 0)).toEqual({ tax, included });
        });

        it('should round the tax to zero when the amount is 1 yen', () => {
            expect(calculateFromExcluding(1, 10, 0)).toEqual({ tax: 0, included: 1 });
        });

        it('should keep every digit when the amount is the maximum of 999,999,999 yen', () => {
            expect(calculateFromExcluding(999999999, 10, 0))
                .toEqual({ tax: 99999999, included: 1099999998 });
        });

        it.each([
            { mode: 1, tax: 33, included: 366 },
            { mode: 2, tax: 34, included: 367 },
        ])('should round 33.3 yen of tax to $tax when the rounding type is $mode', ({ mode, tax, included }) => {
            expect(calculateFromExcluding(333, 10, mode)).toEqual({ tax, included });
        });
    });

    describe('calculateFromIncluding — exact splits and limits', () => {

        it.each([
            { included: 1100, rate: 10, mode: 0, excluded: 1000, tax: 100 },
            { included: 1080, rate: 8, mode: 0, excluded: 1000, tax: 80 },
            { included: 366, rate: 10, mode: 0, excluded: 333, tax: 33 },
            { included: 366, rate: 10, mode: 2, excluded: 332, tax: 34 },
            { included: 325, rate: 8, mode: 0, excluded: 301, tax: 24 },
            { included: 325, rate: 8, mode: 2, excluded: 301, tax: 24 },
        ])('should split $included yen at $rate% into $excluded + $tax when the rounding type is $mode', ({ included, rate, mode, excluded, tax }) => {
            expect(calculateFromIncluding(included, rate, mode)).toEqual({ excluded, tax });
        });

        it('should keep 1 yen as the price with no tax when 1 yen is typed', () => {
            expect(calculateFromIncluding(1, 10, 0)).toEqual({ excluded: 1, tax: 0 });
        });

        it('should keep every digit when the tax-included amount is at the maximum', () => {
            expect(calculateFromIncluding(1099999998, 10, 0))
                .toEqual({ excluded: 999999999, tax: 99999999 });
        });

        it('should get the original amount back when an amount goes to tax-included and back under floor', () => {
            const { included } = calculateFromExcluding(777, 10, 0);
            expect(included).toBe(854);
            expect(calculateFromIncluding(included, 10, 0)).toEqual({ excluded: 777, tax: 77 });
        });

        it('should pick another split that still adds up to the typed amount when the rounding type is half-up', () => {
            // 777 + floor(77.7) = 854, but under half-up 777 + 78 = 855, so
            // 854 splits as 776 + round(77.6) = 776 + 78.
            expect(calculateFromIncluding(854, 10, 1)).toEqual({ excluded: 776, tax: 78 });
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
    // FOURTH one downstream. The pure helper now always returns a split
    // with `excluded + tax == typed amount`, preferring one that matches
    // the authoritative `round(excluded * rate)`; for prices that have
    // none, the tax is carved out of the typed amount instead of forcing
    // the price to a neighbour (latent-scan2 T2).
    // ========================================================================
    describe('calculateFromIncluding — three-value self-consistency (Fable-5 #8)', () => {

        it('should keep the typed input under FLOOR when a base±1 candidate reproduces it', () => {
            // The canonical Fable-5 #8 scenario: user types 101 with 10 % / FLOOR.
            // Pre-fix: `tax = included - excluded` left the DB with three
            // inconsistent numbers (91, 10, 101). CodeRabbit on #129 pointed
            // out that just picking base=91 and correcting the input down to
            // 100 is unnecessary — base+1=92 gives 92 + floor(92 * 0.10) = 101,
            // reproducing the typed input exactly. The helper now scans
            // `[base, base+1, base-1]` and prefers the candidate that matches
            // the typed input, so this common shape stays at 101 円.
            const { excluded, tax } =
                calculateFromIncluding(/*includedInput*/ 101, /*rate*/ 10, /*floor*/ 0);

            expect(excluded).toBe(92);
            expect(tax).toBe(9);
            expect(excluded + tax).toBe(101);
        });

        it('should keep the typed input under CEIL by picking base-1 (91, not the ceil base 92) when base-1 reproduces it', () => {
            // Same typed 101, but with CEIL. base = ceil(101/1.1) = 92, and
            // 92 + ceil(92*0.1) = 102 (does NOT match). base-1 = 91 with
            // ceil(91*0.1) = 10 yields 101 — pick that so the input stays.
            const { excluded, tax } =
                calculateFromIncluding(101, 10, /*ceil*/ 2);

            expect(excluded).toBe(91);
            expect(tax).toBe(10);
        });

        it('should leave the tax-included input untouched when the split is already exact', () => {
            // 330 = 300 + 30 under FLOOR + 10 %; base wins immediately.
            const { excluded, tax } =
                calculateFromIncluding(330, 10, 0);

            expect(excluded).toBe(300);
            expect(tax).toBe(30);
        });

        it('should produce consistent numbers under half-up rounding when the base already matches', () => {
            // 325 / 1.08 ≈ 300.925 → half-up 301, tax = round(301 * 0.08) = 24,
            // 301 + 24 = 325 — base itself matches, no candidate scan needed.
            const { excluded, tax } =
                calculateFromIncluding(325, 8, /*half-up*/ 1);

            expect(excluded).toBe(301);
            expect(tax).toBe(24);
        });

        it('should keep the typed price and carve the tax out of it when no base reproduces it (scan2-T2)', () => {
            // 1000 円 at 10 % / FLOOR: base=909, 909+90=999 ≠ 1000; base+1=910,
            // 910+91=1001 ≠ 1000; base-1=908, 908+90=998 ≠ 1000. No candidate
            // fits the tax-excluded formula, so the typed price is kept and
            // tax = 1000 - 909 = 91 (owner decision 2026-10-01).
            const { excluded, tax } = calculateFromIncluding(1000, 10, 0);

            expect(excluded).toBe(909);
            expect(tax).toBe(91);
            expect(excluded + tax).toBe(1000);
        });

        it('should keep every typed price from 1 to 10,000 when any rounding mode is used (scan2-T2)', () => {
            for (const rate of [8, 10]) {
                for (const rounding of [0, 1, 2]) {
                    for (let included = 1; included <= 10000; included++) {
                        const { excluded, tax } = calculateFromIncluding(included, rate, rounding);
                        if (excluded + tax !== included || excluded < 0 || tax < 0) {
                            throw new Error(`${included} at ${rate} % (rounding ${rounding}) -> ${excluded} + ${tax}`);
                        }
                    }
                }
            }
        });

        it('should return zeros when the input is zero', () => {
            expect(calculateFromIncluding(0, 10, 0))
                .toEqual({ excluded: 0, tax: 0 });
        });

        it('should carve out no tax when the rate is 0 %', () => {
            expect(calculateFromIncluding(500, 0, 0))
                .toEqual({ excluded: 500, tax: 0 });
        });
    });

    describe('calculateFromExcluding — helper covers the excluded-input side', () => {

        it('should round the tax down when it has a fraction', () => {
            // 333 * 0.10 = 33.3 → floor 33, included = 366.
            expect(calculateFromExcluding(333, 10, 0))
                .toEqual({ tax: 33, included: 366 });
        });

        it('should return zeros when the input is zero', () => {
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
        it('should round down when the rounding type is floor', () => {
            expect(applyTaxRounding(9.9, 0)).toBe(9);
        });
        it('should round down when the rounding type is unknown', () => {
            expect(applyTaxRounding(9.9, /*unknown*/ 99)).toBe(9);
        });
        it('should round up to the next integer when half-up gets a positive .5', () => {
            expect(applyTaxRounding(9.5, 1)).toBe(10);
        });
        it('should round up when ceil gets a fractional part', () => {
            expect(applyTaxRounding(9.01, 2)).toBe(10);
        });
    });

});
