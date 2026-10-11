/**
 * Transaction Edit Tests
 *
 * Tests for the transaction edit functionality in the transaction management screen.
 * This test suite validates:
 * - Amount formatting
 *
 * Loading a saved header and the values sent when saving are tested on the
 * real page in ./pages/transaction-management-edit-roundtrip.test.js; the
 * account fields and the checks on Save in
 * ./pages/transaction-management-edit-accounts.test.js; the Shop field in
 * ./pages/transaction-management-edit-shop.test.js; which header the window
 * works on and the messages when loading or saving fails in
 * ./pages/transaction-management-edit-modal-errors.test.js.
 */

describe('Transaction Edit - Amount Formatting', () => {
    // Format amount with comma separators for display
    function formatAmountForDisplay(amount) {
        if (amount === null || amount === undefined) {
            return '';
        }
        return amount.toLocaleString('en-US');
    }

    // Parse amount from display format (remove commas)
    function parseAmountFromDisplay(displayValue) {
        if (!displayValue) {
            return null;
        }
        const cleaned = displayValue.replace(/,/g, '');
        const parsed = parseInt(cleaned, 10);
        return isNaN(parsed) ? null : parsed;
    }

    describe('Format for display', () => {
        test('should add a comma when the amount is 1000', () => {
            expect(formatAmountForDisplay(1000)).toBe('1,000');
        });

        test('should add commas when the amount is 1000000', () => {
            expect(formatAmountForDisplay(1000000)).toBe('1,000,000');
        });

        test('should add no comma when the amount is below 1000', () => {
            expect(formatAmountForDisplay(999)).toBe('999');
        });

        test('should show 0 when the amount is zero', () => {
            expect(formatAmountForDisplay(0)).toBe('0');
        });

        test('should show an empty string when the amount is null', () => {
            expect(formatAmountForDisplay(null)).toBe('');
        });

        test('should show an empty string when the amount is undefined', () => {
            expect(formatAmountForDisplay(undefined)).toBe('');
        });

        test('should keep the minus sign when the amount is negative', () => {
            expect(formatAmountForDisplay(-1000)).toBe('-1,000');
        });
    });

    describe('Parse from display', () => {
        test('should parse the amount when it has a comma', () => {
            expect(parseAmountFromDisplay('1,000')).toBe(1000);
        });

        test('should parse the amount when it has several commas', () => {
            expect(parseAmountFromDisplay('1,000,000')).toBe(1000000);
        });

        test('should parse the amount when it has no commas', () => {
            expect(parseAmountFromDisplay('999')).toBe(999);
        });

        test('should parse 0 when the text is zero', () => {
            expect(parseAmountFromDisplay('0')).toBe(0);
        });

        test('should return null when the text is empty', () => {
            expect(parseAmountFromDisplay('')).toBeNull();
        });

        test('should return null when the text is null', () => {
            expect(parseAmountFromDisplay(null)).toBeNull();
        });

        test('should parse a negative amount when the text has a minus sign', () => {
            expect(parseAmountFromDisplay('-1,000')).toBe(-1000);
        });

        test('should return null when the text is not a number', () => {
            expect(parseAmountFromDisplay('abc')).toBeNull();
        });
    });

    describe('Round-trip formatting', () => {
        test('should get the same amount back when it is formatted and parsed', () => {
            const original = 1234567;
            const formatted = formatAmountForDisplay(original);
            const parsed = parseAmountFromDisplay(formatted);
            expect(parsed).toBe(original);
        });

        test('should get each amount back when several amounts are formatted and parsed', () => {
            const testCases = [0, 100, 1000, 10000, 100000, 1000000];
            testCases.forEach(amount => {
                const formatted = formatAmountForDisplay(amount);
                const parsed = parseAmountFromDisplay(formatted);
                expect(parsed).toBe(amount);
            });
        });
    });
});
