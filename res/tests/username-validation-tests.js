/**
 * Common Username Validation Test Suites
 * 
 * Reusable test suites for username validation in user management forms
 */

/**
 * Test suite for username validation
 * Pass a function that takes (username, password, passwordConfirm)
 */
export function testUsernameValidation(validationFn) {
    const validPassword = '1234567890123456';
    
    describe('Username Validation', () => {
        test('should reject empty username', () => {
            const result = validationFn('', validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject username with only spaces', () => {
            const result = validationFn('   ', validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject username with only tabs', () => {
            const result = validationFn('\t\t', validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject username with mixed whitespace', () => {
            const result = validationFn(' \t \n ', validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject null username', () => {
            const result = validationFn(null, validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject undefined username', () => {
            const result = validationFn(undefined, validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should accept valid username with single character', () => {
            const result = validationFn('a', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept valid username with multiple characters', () => {
            const result = validationFn('testuser', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept valid username with numbers', () => {
            const result = validationFn('user123', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept valid username with special characters', () => {
            const result = validationFn('user_test-01', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept valid username with email format', () => {
            const result = validationFn('user@example.com', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should handle username with leading spaces (after trim)', () => {
            const result = validationFn('  testuser', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should handle username with trailing spaces (after trim)', () => {
            const result = validationFn('testuser  ', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
    });
}

/**
 * Test suite for combined username and password validation scenarios
 */
export function testCombinedValidation(validationFn) {
    describe('Combined Validation Scenarios', () => {
        test('should reject when both username and password are empty', () => {
            const result = validationFn('', '', '');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should prioritize username validation over password validation', () => {
            const result = validationFn('', 'short', 'short');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should prioritize password empty validation over length validation', () => {
            const result = validationFn('testuser', '', '');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should prioritize password length validation over match validation', () => {
            const result = validationFn('testuser', 'short', 'different');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });

        test('should accept completely valid user addition', () => {
            const result = validationFn('validuser', '1234567890123456', '1234567890123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept valid user addition with complex username', () => {
            const result = validationFn('user_test@example.com', 'SecurePassword123456', 'SecurePassword123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept valid user addition with complex password', () => {
            const password = 'C0mpl3x!P@ssw0rd#2024$%^&*()';
            const result = validationFn('testuser', password, password);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
    });
}
