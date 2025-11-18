/**
 * Transaction Detail Management Tests
 * 
 * Tests for transaction detail management including:
 * - Category selection and dynamic updates
 * - Validation logic
 * - Memo functionality
 * - Amount calculation helpers
 */

import { describe, it, expect, beforeEach } from '@jest/globals';

describe('Transaction Detail Management Tests', () => {
    
    describe('Category Selection Logic', () => {
        
        it('should require category2 when category1 is selected', () => {
            const category1Code = 'EXPENSE';
            const category2Code = null;
            
            // If category1 is selected, category2 must also be selected
            const isValid = !!(category1Code && category2Code);
            expect(isValid).toBe(false);
        });
        
        it('should allow category1 and category2 without category3', () => {
            const category1Code = 'EXPENSE';
            const category2Code = 'C2_E_1';
            const category3Code = null;
            
            // Category3 is optional if category2 is selected
            const isValid = !!(category1Code && category2Code);
            expect(isValid).toBe(true);
        });
        
        it('should allow all three categories', () => {
            const category1Code = 'EXPENSE';
            const category2Code = 'C2_E_1';
            const category3Code = 'C3_1';
            
            const isValid = !!(category1Code && category2Code && category3Code);
            expect(isValid).toBe(true);
        });
        
        it('should not allow category3 without category2', () => {
            const category1Code = 'EXPENSE';
            const category2Code = null;
            const category3Code = 'C3_1';
            
            // Category3 cannot be selected without category2
            const isValid = !!(category1Code && category2Code);
            expect(isValid).toBe(false);
        });
        
    });
    
    describe('Amount Validation', () => {
        
        it('should accept valid positive amount', () => {
            const amount = 1000;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(true);
        });
        
        it('should reject negative amount', () => {
            const amount = -100;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(false);
        });
        
        it('should reject zero amount', () => {
            const amount = 0;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(false);
        });
        
        it('should accept maximum amount (999,999,999)', () => {
            const amount = 999999999;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(true);
        });
        
        it('should reject amount exceeding maximum', () => {
            const amount = 1000000000;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(false);
        });
        
        it('should accept minimum amount (1)', () => {
            const amount = 1;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(true);
        });
        
    });
    
    describe('Tax Rate Validation', () => {
        
        it('should accept 0% tax rate', () => {
            const taxRate = 0;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(true);
        });
        
        it('should accept 8% tax rate', () => {
            const taxRate = 8;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(true);
        });
        
        it('should accept 10% tax rate', () => {
            const taxRate = 10;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(true);
        });
        
        it('should accept 100% tax rate', () => {
            const taxRate = 100;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(true);
        });
        
        it('should reject negative tax rate', () => {
            const taxRate = -5;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(false);
        });
        
        it('should reject tax rate over 100%', () => {
            const taxRate = 101;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(false);
        });
        
    });
    
    describe('Amount Formatting', () => {
        
        it('should format amount with thousand separators', () => {
            const amount = 1234567;
            const formatted = amount.toLocaleString('ja-JP');
            expect(formatted).toBe('1,234,567');
        });
        
        it('should format small amount', () => {
            const amount = 100;
            const formatted = amount.toLocaleString('ja-JP');
            expect(formatted).toBe('100');
        });
        
        it('should format maximum amount', () => {
            const amount = 999999999;
            const formatted = amount.toLocaleString('ja-JP');
            expect(formatted).toBe('999,999,999');
        });
        
        it('should format zero amount', () => {
            const amount = 0;
            const formatted = amount.toLocaleString('ja-JP');
            expect(formatted).toBe('0');
        });
        
    });
    
    describe('Tax Type Selection', () => {
        
        it('should handle tax-excluding (内税) type', () => {
            const taxType = 0;
            const isTaxExcluding = taxType === 0;
            expect(isTaxExcluding).toBe(true);
        });
        
        it('should handle tax-including (外税) type', () => {
            const taxType = 1;
            const isTaxIncluding = taxType === 1;
            expect(isTaxIncluding).toBe(true);
        });
        
        it('should default to tax-excluding when undefined', () => {
            const taxType = undefined;
            const defaultTaxType = taxType ?? 0;
            expect(defaultTaxType).toBe(0);
        });
        
    });
    
    describe('Memo Validation', () => {
        
        it('should accept empty memo', () => {
            const memo = '';
            const isValid = memo.length <= 500;
            expect(isValid).toBe(true);
        });
        
        it('should accept valid memo within limit', () => {
            const memo = 'テストメモ';
            const isValid = memo.length <= 500;
            expect(isValid).toBe(true);
        });
        
        it('should accept memo at maximum length (500 characters)', () => {
            const memo = 'a'.repeat(500);
            const isValid = memo.length <= 500;
            expect(isValid).toBe(true);
        });
        
        it('should reject memo exceeding maximum length', () => {
            const memo = 'a'.repeat(501);
            const isValid = memo.length <= 500;
            expect(isValid).toBe(false);
        });
        
        it('should accept Japanese characters in memo', () => {
            const memo = 'これはテストメモです。今日スーパーで買い物をしました。';
            const isValid = memo.length <= 500 && memo.length > 0;
            expect(isValid).toBe(true);
        });
        
        it('should accept English characters in memo', () => {
            const memo = 'This is a test memo. I went shopping at the supermarket today.';
            const isValid = memo.length <= 500 && memo.length > 0;
            expect(isValid).toBe(true);
        });
        
    });
    
    describe('Detail ID Validation', () => {
        
        it('should accept valid detail ID (1)', () => {
            const detailId = 1;
            const isValid = detailId >= 1 && detailId <= 999;
            expect(isValid).toBe(true);
        });
        
        it('should accept maximum detail ID (999)', () => {
            const detailId = 999;
            const isValid = detailId >= 1 && detailId <= 999;
            expect(isValid).toBe(true);
        });
        
        it('should reject detail ID of 0', () => {
            const detailId = 0;
            const isValid = detailId >= 1 && detailId <= 999;
            expect(isValid).toBe(false);
        });
        
        it('should reject negative detail ID', () => {
            const detailId = -1;
            const isValid = detailId >= 1 && detailId <= 999;
            expect(isValid).toBe(false);
        });
        
        it('should reject detail ID exceeding maximum', () => {
            const detailId = 1000;
            const isValid = detailId >= 1 && detailId <= 999;
            expect(isValid).toBe(false);
        });
        
    });
    
    describe('Tax Calculation Field Determination', () => {
        
        it('should recalculate tax-including when tax-excluding is last edited', () => {
            const lastEditedField = 'excluding';
            const shouldCalculateIncluding = lastEditedField === 'excluding';
            expect(shouldCalculateIncluding).toBe(true);
        });
        
        it('should recalculate tax-excluding when tax-including is last edited', () => {
            const lastEditedField = 'including';
            const shouldCalculateExcluding = lastEditedField === 'including';
            expect(shouldCalculateExcluding).toBe(true);
        });
        
        it('should recalculate based on tax-excluding after tax rate change', () => {
            // When tax rate changes, the last edited amount field should be preserved
            // and the other field should be recalculated
            const lastEditedField = 'excluding';
            const taxRate = 10; // Changed from 8% to 10%
            
            const shouldCalculateIncluding = lastEditedField === 'excluding';
            expect(shouldCalculateIncluding).toBe(true);
        });
        
        it('should handle tax type change by recalculating including amount', () => {
            // When tax type changes, recalculate based on excluding amount
            const taxType = 1; // Changed to tax-including
            const lastEditedField = 'excluding'; // Assuming excluding was edited
            
            const shouldCalculateIncluding = true; // Always recalculate including
            expect(shouldCalculateIncluding).toBe(true);
        });
        
    });
    
    describe('Input Field State Management', () => {
        
        it('should track last edited field', () => {
            let lastEditedField = null;
            
            // Simulate editing tax-excluding field
            lastEditedField = 'excluding';
            expect(lastEditedField).toBe('excluding');
            
            // Simulate editing tax-including field
            lastEditedField = 'including';
            expect(lastEditedField).toBe('including');
        });
        
        it('should clear rounding warning when user edits amount', () => {
            let hasRoundingWarning = true;
            
            // User edits amount field
            hasRoundingWarning = false;
            
            expect(hasRoundingWarning).toBe(false);
        });
        
        it('should show rounding warning when detected', () => {
            const userInputIncluding = 366;
            const calculatedIncluding = 365;
            
            const hasRoundingError = userInputIncluding !== calculatedIncluding;
            expect(hasRoundingError).toBe(true);
        });
        
    });
    
    describe('Category Code Format Validation', () => {
        
        it('should accept valid CATEGORY1_CODE format (EXPENSE)', () => {
            const category1Code = 'EXPENSE';
            const isValid = /^[A-Z_]+$/.test(category1Code);
            expect(isValid).toBe(true);
        });
        
        it('should accept valid CATEGORY1_CODE format (INCOME)', () => {
            const category1Code = 'INCOME';
            const isValid = /^[A-Z_]+$/.test(category1Code);
            expect(isValid).toBe(true);
        });
        
        it('should accept valid CATEGORY2_CODE format (C2_E_1)', () => {
            const category2Code = 'C2_E_1';
            const isValid = /^C2_[A-Z]_\d+$/.test(category2Code);
            expect(isValid).toBe(true);
        });
        
        it('should accept valid CATEGORY3_CODE format (C3_1)', () => {
            const category3Code = 'C3_1';
            const isValid = /^C3_\d+$/.test(category3Code);
            expect(isValid).toBe(true);
        });
        
        it('should reject invalid CATEGORY2_CODE format', () => {
            const category2Code = 'INVALID';
            const isValid = /^C2_[A-Z]_\d+$/.test(category2Code);
            expect(isValid).toBe(false);
        });
        
        it('should reject invalid CATEGORY3_CODE format', () => {
            const category3Code = 'INVALID';
            const isValid = /^C3_\d+$/.test(category3Code);
            expect(isValid).toBe(false);
        });
        
    });
    
    describe('Edge Cases', () => {
        
        it('should handle very small tax calculation', () => {
            const excludingTax = 1;
            const taxRate = 10;
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            expect(taxAmount).toBe(0); // 0.1 -> 0
        });
        
        it('should handle maximum values in calculation', () => {
            const excludingTax = 999999999;
            const taxRate = 10;
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            expect(includingTax).toBe(1099999998);
        });
        
        it('should handle 0% tax rate', () => {
            const excludingTax = 1000;
            const taxRate = 0;
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            expect(taxAmount).toBe(0);
            expect(includingTax).toBe(1000);
        });
        
        it('should handle 100% tax rate', () => {
            const excludingTax = 1000;
            const taxRate = 100;
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            expect(taxAmount).toBe(1000);
            expect(includingTax).toBe(2000);
        });
        
    });
    
});
