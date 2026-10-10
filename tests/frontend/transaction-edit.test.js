/**
 * Transaction Edit Tests
 *
 * Tests for the transaction edit functionality in the transaction management screen.
 * This test suite validates:
 * - Edit modal opening
 * - Form validation
 * - Category change handling and account reset
 * - Error handling
 *
 * Loading a saved header and the values sent when saving are tested on the
 * real page in ./pages/transaction-management-edit-roundtrip.test.js.
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

describe('Transaction Edit - Category Change and Account Reset', () => {
    // Simulate category change behavior
    class AccountFieldManager {
        constructor() {
            this.fromAccountVisible = false;
            this.toAccountVisible = false;
            this.fromAccountValue = 'NONE';
            this.toAccountValue = 'NONE';
        }

        handleCategoryChange(categoryType) {
            if (!categoryType) {
                // No category selected
                this.fromAccountVisible = false;
                this.toAccountVisible = false;
                this.fromAccountValue = 'NONE';
                this.toAccountValue = 'NONE';
                return;
            }

            // Based on category type, show/hide account fields
            if (categoryType === 'EXPENSE') {
                this.fromAccountVisible = true;
                this.toAccountVisible = false;
                this.toAccountValue = 'NONE'; // Reset hidden field
            } else if (categoryType === 'INCOME') {
                this.fromAccountVisible = false;
                this.toAccountVisible = true;
                this.fromAccountValue = 'NONE'; // Reset hidden field
            } else if (categoryType === 'TRANSFER') {
                this.fromAccountVisible = true;
                this.toAccountVisible = true;
                // Don't reset - both are visible
            } else {
                // Default: show both
                this.fromAccountVisible = true;
                this.toAccountVisible = true;
            }
        }

        setFromAccount(value) {
            if (this.fromAccountVisible) {
                this.fromAccountValue = value;
            }
        }

        setToAccount(value) {
            if (this.toAccountVisible) {
                this.toAccountValue = value;
            }
        }
    }

    test('should hide both accounts when no category is selected', () => {
        const manager = new AccountFieldManager();
        manager.handleCategoryChange(null);

        expect(manager.fromAccountVisible).toBe(false);
        expect(manager.toAccountVisible).toBe(false);
        expect(manager.fromAccountValue).toBe('NONE');
        expect(manager.toAccountValue).toBe('NONE');
    });

    test('should show only the FROM account when the category is EXPENSE', () => {
        const manager = new AccountFieldManager();
        manager.handleCategoryChange('EXPENSE');

        expect(manager.fromAccountVisible).toBe(true);
        expect(manager.toAccountVisible).toBe(false);
        expect(manager.toAccountValue).toBe('NONE');
    });

    test('should show only the TO account when the category is INCOME', () => {
        const manager = new AccountFieldManager();
        manager.handleCategoryChange('INCOME');

        expect(manager.fromAccountVisible).toBe(false);
        expect(manager.toAccountVisible).toBe(true);
        expect(manager.fromAccountValue).toBe('NONE');
    });

    test('should show both accounts when the category is TRANSFER', () => {
        const manager = new AccountFieldManager();
        manager.handleCategoryChange('TRANSFER');

        expect(manager.fromAccountVisible).toBe(true);
        expect(manager.toAccountVisible).toBe(true);
    });

    test('should reset the TO account when switching from TRANSFER to EXPENSE', () => {
        const manager = new AccountFieldManager();
        manager.handleCategoryChange('TRANSFER');
        manager.setToAccount('BANK');

        expect(manager.toAccountValue).toBe('BANK');

        manager.handleCategoryChange('EXPENSE');
        expect(manager.toAccountValue).toBe('NONE');
    });

    test('should reset the FROM account when switching from TRANSFER to INCOME', () => {
        const manager = new AccountFieldManager();
        manager.handleCategoryChange('TRANSFER');
        manager.setFromAccount('CASH');

        expect(manager.fromAccountValue).toBe('CASH');

        manager.handleCategoryChange('INCOME');
        expect(manager.fromAccountValue).toBe('NONE');
    });

    test('should reset both accounts when switching to no category', () => {
        const manager = new AccountFieldManager();
        manager.handleCategoryChange('TRANSFER');
        manager.setFromAccount('CASH');
        manager.setToAccount('BANK');

        manager.handleCategoryChange(null);
        expect(manager.fromAccountValue).toBe('NONE');
        expect(manager.toAccountValue).toBe('NONE');
    });

    test('should keep the account NONE when it is set while hidden', () => {
        const manager = new AccountFieldManager();
        manager.handleCategoryChange('EXPENSE');
        manager.setToAccount('BANK');

        // TO account is not visible for EXPENSE, so value should remain NONE
        expect(manager.toAccountValue).toBe('NONE');
    });
});

describe('Transaction Edit - Form Validation', () => {
    // Simulate form validation
    function validateTransactionForm(formData) {
        const errors = [];

        if (!formData.transaction_date) {
            errors.push('Transaction date is required');
        }

        if (!formData.category1_code) {
            errors.push('Category is required');
        }

        if (formData.total_amount === undefined || formData.total_amount === null) {
            errors.push('Total amount is required');
        }

        if (formData.total_amount !== undefined && formData.total_amount !== null) {
            if (typeof formData.total_amount !== 'number') {
                errors.push('Total amount must be a number');
            }
        }

        // Account validation based on category
        if (formData.category1_code === 'EXPENSE' && !formData.from_account_code) {
            errors.push('From account is required for expense');
        }

        if (formData.category1_code === 'INCOME' && !formData.to_account_code) {
            errors.push('To account is required for income');
        }

        if (formData.category1_code === 'TRANSFER') {
            if (!formData.from_account_code) {
                errors.push('From account is required for transfer');
            }
            if (!formData.to_account_code) {
                errors.push('To account is required for transfer');
            }
        }

        return {
            valid: errors.length === 0,
            errors: errors
        };
    }

    test('should accept the form when all fields are valid', () => {
        const formData = {
            transaction_date: '2024-01-15 10:00:00',
            category1_code: 'EXPENSE',
            from_account_code: 'CASH',
            to_account_code: 'NONE',
            total_amount: 1000,
            tax_rounding_type: 0,
            memo: 'Test'
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(true);
        expect(result.errors.length).toBe(0);
    });

    test('should reject the form when the date is missing', () => {
        const formData = {
            category1_code: 'EXPENSE',
            from_account_code: 'CASH',
            total_amount: 1000
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(false);
        expect(result.errors).toContain('Transaction date is required');
    });

    test('should reject the form when the category is missing', () => {
        const formData = {
            transaction_date: '2024-01-15 10:00:00',
            total_amount: 1000
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(false);
        expect(result.errors).toContain('Category is required');
    });

    test('should reject the form when the amount is missing', () => {
        const formData = {
            transaction_date: '2024-01-15 10:00:00',
            category1_code: 'EXPENSE',
            from_account_code: 'CASH'
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(false);
        expect(result.errors).toContain('Total amount is required');
    });

    test('should reject an expense when the FROM account is missing', () => {
        const formData = {
            transaction_date: '2024-01-15 10:00:00',
            category1_code: 'EXPENSE',
            total_amount: 1000
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(false);
        expect(result.errors).toContain('From account is required for expense');
    });

    test('should reject an income when the TO account is missing', () => {
        const formData = {
            transaction_date: '2024-01-15 10:00:00',
            category1_code: 'INCOME',
            total_amount: 1000
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(false);
        expect(result.errors).toContain('To account is required for income');
    });

    test('should reject a transfer when the FROM account is missing', () => {
        const formData = {
            transaction_date: '2024-01-15 10:00:00',
            category1_code: 'TRANSFER',
            to_account_code: 'BANK',
            total_amount: 1000
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(false);
        expect(result.errors).toContain('From account is required for transfer');
    });

    test('should reject a transfer when the TO account is missing', () => {
        const formData = {
            transaction_date: '2024-01-15 10:00:00',
            category1_code: 'TRANSFER',
            from_account_code: 'CASH',
            total_amount: 1000
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(false);
        expect(result.errors).toContain('To account is required for transfer');
    });

    test('should accept the form when the amount is zero', () => {
        const formData = {
            transaction_date: '2024-01-15 10:00:00',
            category1_code: 'EXPENSE',
            from_account_code: 'CASH',
            total_amount: 0
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(true);
    });

    test('should accept the form when there is no memo', () => {
        const formData = {
            transaction_date: '2024-01-15 10:00:00',
            category1_code: 'EXPENSE',
            from_account_code: 'CASH',
            total_amount: 1000
        };
        const result = validateTransactionForm(formData);
        expect(result.valid).toBe(true);
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

describe('Transaction Edit - Shop Selection', () => {
    // Simulate shop selection handling
    class ShopSelectionHandler {
        constructor() {
            this.shops = [];
            this.selectedShopId = null;
        }

        loadShops(shopsData) {
            this.shops = shopsData;
        }

        setShopId(shopId) {
            if (shopId === '' || shopId === null || shopId === undefined) {
                this.selectedShopId = null;
            } else {
                this.selectedShopId = parseInt(shopId);
            }
        }

        getShopId() {
            return this.selectedShopId;
        }

        getShopName(shopId) {
            if (!shopId) {
                return 'Unspecified';
            }
            const shop = this.shops.find(s => s.shop_id === shopId);
            return shop ? shop.shop_name : 'Unknown';
        }
    }

    test('should have no shop ID when the shop selection starts', () => {
        const handler = new ShopSelectionHandler();
        expect(handler.selectedShopId).toBeNull();
    });

    test('should hold the shops when the shop list is loaded', () => {
        const handler = new ShopSelectionHandler();
        const shops = [
            { shop_id: 1, shop_name: 'Shop A' },
            { shop_id: 2, shop_name: 'Shop B' }
        ];
        handler.loadShops(shops);
        expect(handler.shops.length).toBe(2);
        expect(handler.shops[0].shop_name).toBe('Shop A');
    });

    test('should store the shop ID as an integer when a shop is selected', () => {
        const handler = new ShopSelectionHandler();
        handler.setShopId('5');
        expect(handler.selectedShopId).toBe(5);
        expect(typeof handler.selectedShopId).toBe('number');
    });

    test('should store null when the shop ID is an empty string', () => {
        const handler = new ShopSelectionHandler();
        handler.setShopId('');
        expect(handler.selectedShopId).toBeNull();
    });

    test('should store null when the shop ID is null', () => {
        const handler = new ShopSelectionHandler();
        handler.setShopId(null);
        expect(handler.selectedShopId).toBeNull();
    });

    test('should store null when the shop ID is undefined', () => {
        const handler = new ShopSelectionHandler();
        handler.setShopId(undefined);
        expect(handler.selectedShopId).toBeNull();
    });

    test('should return the shop name when the shop ID exists', () => {
        const handler = new ShopSelectionHandler();
        handler.loadShops([
            { shop_id: 1, shop_name: 'Shop A' },
            { shop_id: 2, shop_name: 'Shop B' }
        ]);
        expect(handler.getShopName(1)).toBe('Shop A');
        expect(handler.getShopName(2)).toBe('Shop B');
    });

    test('should return "Unspecified" when the shop ID is null', () => {
        const handler = new ShopSelectionHandler();
        expect(handler.getShopName(null)).toBe('Unspecified');
    });

    test('should return "Unknown" when the shop ID does not exist', () => {
        const handler = new ShopSelectionHandler();
        handler.loadShops([
            { shop_id: 1, shop_name: 'Shop A' }
        ]);
        expect(handler.getShopName(999)).toBe('Unknown');
    });

    test('should hold the new shop ID when switching between shops', () => {
        const handler = new ShopSelectionHandler();
        handler.setShopId('1');
        expect(handler.selectedShopId).toBe(1);

        handler.setShopId('2');
        expect(handler.selectedShopId).toBe(2);
    });

    test('should store null when the shop selection is cleared', () => {
        const handler = new ShopSelectionHandler();
        handler.setShopId('1');
        expect(handler.selectedShopId).toBe(1);

        handler.setShopId('');
        expect(handler.selectedShopId).toBeNull();
    });
});

describe('Transaction Edit - Shop Selection Integration', () => {
    // Simulate transaction with shop selection
    function createTransactionWithShop(shopId) {
        return {
            transaction_id: 1,
            transaction_date: '2024-01-15 10:00:00',
            category1_code: 'EXPENSE',
            from_account_code: 'CASH',
            to_account_code: 'NONE',
            total_amount: 1000,
            tax_rounding_type: 0,
            memo: 'Test',
            shop_id: shopId
        };
    }

    test('should include shop_id when the transaction data is built', () => {
        const transaction = createTransactionWithShop(5);
        expect(transaction.shop_id).toBe(5);
    });

    test('should allow a null shop_id when no shop is selected', () => {
        const transaction = createTransactionWithShop(null);
        expect(transaction.shop_id).toBeNull();
    });

    test('should keep shop_id when the form is loaded', () => {
        const transaction = createTransactionWithShop(3);
        const formShopId = transaction.shop_id || '';
        expect(formShopId).toBe(3);
    });

    test('should save null when the shop ID is an empty string', () => {
        const formShopId = '';
        const saveShopId = formShopId ? parseInt(formShopId) : null;
        expect(saveShopId).toBeNull();
    });

    test('should save an integer when the shop ID is a string', () => {
        const formShopId = '7';
        const saveShopId = formShopId ? parseInt(formShopId) : null;
        expect(saveShopId).toBe(7);
        expect(typeof saveShopId).toBe('number');
    });
});
