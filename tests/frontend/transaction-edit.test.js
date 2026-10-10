/**
 * Transaction Edit Tests
 *
 * Tests for the transaction edit functionality in the transaction management screen.
 * This test suite validates:
 * - Edit modal opening
 * - Error handling
 *
 * Loading a saved header and the values sent when saving are tested on the
 * real page in ./pages/transaction-management-edit-roundtrip.test.js; the
 * account fields and the checks on Save in
 * ./pages/transaction-management-edit-accounts.test.js; the Shop field in
 * ./pages/transaction-management-edit-shop.test.js.
 */

describe('Transaction Edit - Modal State Management', () => {
    // Simulate transaction edit modal state
    class TransactionModalState {
        constructor() {
            this.isOpen = false;
            this.mode = null; // 'add' or 'edit'
            this.transactionId = null;
            this.transactionData = null;
        }

        open(mode, data = {}) {
            this.isOpen = true;
            this.mode = mode;
            this.transactionId = data.transactionId || null;
            this.transactionData = data;
        }

        close() {
            this.isOpen = false;
            this.mode = null;
            this.transactionId = null;
            this.transactionData = null;
        }

        isEditMode() {
            return this.mode === 'edit' && this.transactionId !== null;
        }

        isAddMode() {
            return this.mode === 'add';
        }
    }

    test('should be closed with no mode or ID when the edit window is created', () => {
        const state = new TransactionModalState();
        expect(state.isOpen).toBe(false);
        expect(state.mode).toBeNull();
        expect(state.transactionId).toBeNull();
    });

    test('should open in edit mode with the ID when a transaction ID is given', () => {
        const state = new TransactionModalState();
        state.open('edit', { transactionId: 123 });

        expect(state.isOpen).toBe(true);
        expect(state.mode).toBe('edit');
        expect(state.transactionId).toBe(123);
        expect(state.isEditMode()).toBe(true);
    });

    test('should open in add mode when no transaction ID is given', () => {
        const state = new TransactionModalState();
        state.open('add', {});

        expect(state.isOpen).toBe(true);
        expect(state.mode).toBe('add');
        expect(state.transactionId).toBeNull();
        expect(state.isAddMode()).toBe(true);
    });

    test('should clear all data when the edit window closes', () => {
        const state = new TransactionModalState();
        state.open('edit', { transactionId: 123 });
        state.close();

        expect(state.isOpen).toBe(false);
        expect(state.mode).toBeNull();
        expect(state.transactionId).toBeNull();
        expect(state.transactionData).toBeNull();
    });

    test('should hold the right ID when the edit window opens and closes several times', () => {
        const state = new TransactionModalState();

        state.open('edit', { transactionId: 1 });
        expect(state.transactionId).toBe(1);

        state.close();
        expect(state.transactionId).toBeNull();

        state.open('edit', { transactionId: 2 });
        expect(state.transactionId).toBe(2);
    });
});

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

describe('Transaction Edit - Error Handling', () => {
    // Simulate API error handling
    class TransactionEditErrorHandler {
        constructor() {
            this.lastError = null;
        }

        handleSaveError(error) {
            this.lastError = error;

            if (error.includes('not found')) {
                return 'Transaction not found. It may have been deleted.';
            } else if (error.includes('permission')) {
                return 'You do not have permission to edit this transaction.';
            } else if (error.includes('validation')) {
                return 'Validation error: Please check your input.';
            } else if (error.includes('network')) {
                return 'Network error: Please check your connection.';
            } else {
                return 'Failed to save transaction: ' + error;
            }
        }

        handleLoadError(error) {
            this.lastError = error;

            if (error.includes('not found')) {
                return 'Transaction not found.';
            } else if (error.includes('permission')) {
                return 'You do not have permission to view this transaction.';
            } else {
                return 'Failed to load transaction: ' + error;
            }
        }

        clearError() {
            this.lastError = null;
        }
    }

    test('should show the not-found message when saving finds no transaction', () => {
        const handler = new TransactionEditErrorHandler();
        const message = handler.handleSaveError('Transaction not found');
        expect(message).toBe('Transaction not found. It may have been deleted.');
    });

    test('should show the permission message when saving is not permitted', () => {
        const handler = new TransactionEditErrorHandler();
        const message = handler.handleSaveError('permission denied');
        expect(message).toBe('You do not have permission to edit this transaction.');
    });

    test('should show the validation message when saving fails validation', () => {
        const handler = new TransactionEditErrorHandler();
        const message = handler.handleSaveError('validation failed');
        expect(message).toBe('Validation error: Please check your input.');
    });

    test('should show the network message when saving hits a network error', () => {
        const handler = new TransactionEditErrorHandler();
        const message = handler.handleSaveError('network timeout');
        expect(message).toBe('Network error: Please check your connection.');
    });

    test('should show the generic message when saving fails for another reason', () => {
        const handler = new TransactionEditErrorHandler();
        const message = handler.handleSaveError('unknown error');
        expect(message).toBe('Failed to save transaction: unknown error');
    });

    test('should show the not-found message when loading finds no transaction', () => {
        const handler = new TransactionEditErrorHandler();
        const message = handler.handleLoadError('Transaction not found');
        expect(message).toBe('Transaction not found.');
    });

    test('should show the permission message when loading is not permitted', () => {
        const handler = new TransactionEditErrorHandler();
        const message = handler.handleLoadError('permission denied');
        expect(message).toBe('You do not have permission to view this transaction.');
    });

    test('should show the generic message when loading fails for another reason', () => {
        const handler = new TransactionEditErrorHandler();
        const message = handler.handleLoadError('database error');
        expect(message).toBe('Failed to load transaction: database error');
    });

    test('should keep the last error when an error is handled', () => {
        const handler = new TransactionEditErrorHandler();
        handler.handleSaveError('test error');
        expect(handler.lastError).toBe('test error');
    });

    test('should clear the last error when it is cleared', () => {
        const handler = new TransactionEditErrorHandler();
        handler.handleSaveError('test error');
        handler.clearError();
        expect(handler.lastError).toBeNull();
    });
});
