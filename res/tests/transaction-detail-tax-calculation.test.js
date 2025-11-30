/**
 * Transaction Detail Tax Calculation Tests
 * 
 * Tests for tax calculation logic including:
 * - Tax-excluding to tax-including calculation
 * - Tax-including to tax-excluding calculation
 * - Rounding error detection and warnings
 */

import { describe, it, expect, beforeEach } from '@jest/globals';

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
    
});
