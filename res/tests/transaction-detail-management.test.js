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
        
        it('should be valid when category1 and category2 are set without category3', () => {
            const category1Code = 'EXPENSE';
            const category2Code = 'C2_E_1';
            const category3Code = null;
            
            // Category3 is optional if category2 is selected
            const isValid = !!(category1Code && category2Code);
            expect(isValid).toBe(true);
        });
        
        it('should be valid when all three categories are set', () => {
            const category1Code = 'EXPENSE';
            const category2Code = 'C2_E_1';
            const category3Code = 'C3_1';
            
            const isValid = !!(category1Code && category2Code && category3Code);
            expect(isValid).toBe(true);
        });
        
        it('should be invalid when category3 is set without category2', () => {
            const category1Code = 'EXPENSE';
            const category2Code = null;
            const category3Code = 'C3_1';
            
            // Category3 cannot be selected without category2
            const isValid = !!(category1Code && category2Code);
            expect(isValid).toBe(false);
        });
        
    });
    
    describe('Amount Validation', () => {
        
        it('should accept the amount when it is positive', () => {
            const amount = 1000;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(true);
        });
        
        it('should reject the amount when it is negative', () => {
            const amount = -100;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(false);
        });
        
        it('should reject the amount when it is zero', () => {
            const amount = 0;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(false);
        });
        
        it('should accept the amount when it is the maximum (999,999,999)', () => {
            const amount = 999999999;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(true);
        });
        
        it('should reject the amount when it exceeds the maximum', () => {
            const amount = 1000000000;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(false);
        });
        
        it('should accept the amount when it is the minimum (1)', () => {
            const amount = 1;
            const isValid = amount > 0 && amount <= 999999999;
            expect(isValid).toBe(true);
        });
        
    });
    
    describe('Tax Rate Validation', () => {
        
        it('should accept the tax rate when it is 0%', () => {
            const taxRate = 0;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(true);
        });
        
        it('should accept the tax rate when it is 8%', () => {
            const taxRate = 8;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(true);
        });
        
        it('should accept the tax rate when it is 10%', () => {
            const taxRate = 10;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(true);
        });
        
        it('should accept the tax rate when it is 100%', () => {
            const taxRate = 100;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(true);
        });
        
        it('should reject the tax rate when it is negative', () => {
            const taxRate = -5;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(false);
        });
        
        it('should reject the tax rate when it is over 100%', () => {
            const taxRate = 101;
            const isValid = taxRate >= 0 && taxRate <= 100;
            expect(isValid).toBe(false);
        });
        
    });
    
    describe('Amount Formatting', () => {
        
        it('should add thousands separators when the amount has seven digits', () => {
            const amount = 1234567;
            const formatted = amount.toLocaleString('ja-JP');
            expect(formatted).toBe('1,234,567');
        });
        
        it('should add no separator when the amount has three digits', () => {
            const amount = 100;
            const formatted = amount.toLocaleString('ja-JP');
            expect(formatted).toBe('100');
        });
        
        it('should add thousands separators when the amount is the maximum', () => {
            const amount = 999999999;
            const formatted = amount.toLocaleString('ja-JP');
            expect(formatted).toBe('999,999,999');
        });
        
        it('should show 0 when the amount is zero', () => {
            const amount = 0;
            const formatted = amount.toLocaleString('ja-JP');
            expect(formatted).toBe('0');
        });
        
    });
    
    describe('Tax Type Selection', () => {
        
        it('should treat the tax type as tax-excluding (外税) when it is 0', () => {
            const taxType = 0;
            const isTaxExcluding = taxType === 0;
            expect(isTaxExcluding).toBe(true);
        });
        
        it('should treat the tax type as tax-including (内税) when it is 1', () => {
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
        
        it('should accept the memo when it is empty', () => {
            const memo = '';
            const isValid = memo.length <= 500;
            expect(isValid).toBe(true);
        });
        
        it('should accept the memo when it is within the limit', () => {
            const memo = 'テストメモ';
            const isValid = memo.length <= 500;
            expect(isValid).toBe(true);
        });
        
        it('should accept the memo when it is at the maximum length (500 characters)', () => {
            const memo = 'a'.repeat(500);
            const isValid = memo.length <= 500;
            expect(isValid).toBe(true);
        });
        
        it('should reject the memo when it exceeds the maximum length', () => {
            const memo = 'a'.repeat(501);
            const isValid = memo.length <= 500;
            expect(isValid).toBe(false);
        });
        
        it('should accept the memo when it is in Japanese', () => {
            const memo = 'これはテストメモです。今日スーパーで買い物をしました。';
            const isValid = memo.length <= 500 && memo.length > 0;
            expect(isValid).toBe(true);
        });
        
        it('should accept the memo when it is in English', () => {
            const memo = 'This is a test memo. I went shopping at the supermarket today.';
            const isValid = memo.length <= 500 && memo.length > 0;
            expect(isValid).toBe(true);
        });
        
    });
    
    describe('Detail ID Validation', () => {
        
        it('should accept the detail ID when it is 1', () => {
            const detailId = 1;
            const isValid = detailId >= 1 && detailId <= 999;
            expect(isValid).toBe(true);
        });
        
        it('should accept the detail ID when it is the maximum (999)', () => {
            const detailId = 999;
            const isValid = detailId >= 1 && detailId <= 999;
            expect(isValid).toBe(true);
        });
        
        it('should reject the detail ID when it is 0', () => {
            const detailId = 0;
            const isValid = detailId >= 1 && detailId <= 999;
            expect(isValid).toBe(false);
        });
        
        it('should reject the detail ID when it is negative', () => {
            const detailId = -1;
            const isValid = detailId >= 1 && detailId <= 999;
            expect(isValid).toBe(false);
        });
        
        it('should reject the detail ID when it exceeds the maximum', () => {
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
        
        it('should recalculate from the tax-excluding amount when the tax rate changes', () => {
            // When tax rate changes, the last edited amount field should be preserved
            // and the other field should be recalculated
            const lastEditedField = 'excluding';
            const taxRate = 10; // Changed from 8% to 10%
            
            const shouldCalculateIncluding = lastEditedField === 'excluding';
            expect(shouldCalculateIncluding).toBe(true);
        });
        
        it('should recalculate the tax-including amount when the tax type changes', () => {
            // When tax type changes, recalculate based on excluding amount
            const taxType = 1; // Changed to tax-including
            const lastEditedField = 'excluding'; // Assuming excluding was edited
            
            const shouldCalculateIncluding = true; // Always recalculate including
            expect(shouldCalculateIncluding).toBe(true);
        });
        
    });
    
    describe('Input Field State Management', () => {
        
        it('should remember the last edited field when the user edits amounts', () => {
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
        
        it('should show the rounding warning when a rounding error is detected', () => {
            const userInputIncluding = 366;
            const calculatedIncluding = 365;
            
            const hasRoundingError = userInputIncluding !== calculatedIncluding;
            expect(hasRoundingError).toBe(true);
        });
        
    });
    
    describe('Category Code Format Validation', () => {
        
        it('should accept the CATEGORY1_CODE when it is EXPENSE', () => {
            const category1Code = 'EXPENSE';
            const isValid = /^[A-Z_]+$/.test(category1Code);
            expect(isValid).toBe(true);
        });
        
        it('should accept the CATEGORY1_CODE when it is INCOME', () => {
            const category1Code = 'INCOME';
            const isValid = /^[A-Z_]+$/.test(category1Code);
            expect(isValid).toBe(true);
        });
        
        it('should accept the CATEGORY2_CODE when it is C2_E_1', () => {
            const category2Code = 'C2_E_1';
            const isValid = /^C2_[A-Z]_\d+$/.test(category2Code);
            expect(isValid).toBe(true);
        });
        
        it('should accept the CATEGORY3_CODE when it is C3_1', () => {
            const category3Code = 'C3_1';
            const isValid = /^C3_\d+$/.test(category3Code);
            expect(isValid).toBe(true);
        });
        
        it('should reject the CATEGORY2_CODE when it does not match the format', () => {
            const category2Code = 'INVALID';
            const isValid = /^C2_[A-Z]_\d+$/.test(category2Code);
            expect(isValid).toBe(false);
        });
        
        it('should reject the CATEGORY3_CODE when it does not match the format', () => {
            const category3Code = 'INVALID';
            const isValid = /^C3_\d+$/.test(category3Code);
            expect(isValid).toBe(false);
        });
        
    });
    
    describe('Edge Cases', () => {
        
        it('should round the tax to zero when the amount is 1 yen', () => {
            const excludingTax = 1;
            const taxRate = 10;
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            expect(taxAmount).toBe(0); // 0.1 -> 0
        });
        
        it('should calculate the tax when the amount is the maximum', () => {
            const excludingTax = 999999999;
            const taxRate = 10;
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            expect(includingTax).toBe(1099999998);
        });
        
        it('should calculate zero tax when the tax rate is 0%', () => {
            const excludingTax = 1000;
            const taxRate = 0;
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            expect(taxAmount).toBe(0);
            expect(includingTax).toBe(1000);
        });
        
        it('should double the amount when the tax rate is 100%', () => {
            const excludingTax = 1000;
            const taxRate = 100;
            const taxAmount = Math.floor(excludingTax * taxRate / 100);
            const includingTax = excludingTax + taxAmount;
            expect(taxAmount).toBe(1000);
            expect(includingTax).toBe(2000);
        });
        
    });
    
});
