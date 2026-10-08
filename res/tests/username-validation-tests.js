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
        test('should reject the username when it is empty', () => {
            const result = validationFn('', validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject the username when it has only spaces', () => {
            const result = validationFn('   ', validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject the username when it has only tabs', () => {
            const result = validationFn('\t\t', validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject the username when it has only mixed whitespace', () => {
            const result = validationFn(' \t \n ', validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject the username when it is null', () => {
            const result = validationFn(null, validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should reject the username when it is undefined', () => {
            const result = validationFn(undefined, validPassword, validPassword);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should accept the username when it has a single character', () => {
            const result = validationFn('a', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the username when it has several characters', () => {
            const result = validationFn('testuser', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the username when it has digits', () => {
            const result = validationFn('user123', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the username when it has underscores and hyphens', () => {
            const result = validationFn('user_test-01', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the username when it is in email format', () => {
            const result = validationFn('user@example.com', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the username when it has leading spaces (trimmed)', () => {
            const result = validationFn('  testuser', validPassword, validPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the username when it has trailing spaces (trimmed)', () => {
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

        test('should report the username error first when both the username and the password are invalid', () => {
            const result = validationFn('', 'short', 'short');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });

        test('should report the empty-password error when the password is empty', () => {
            const result = validationFn('testuser', '', '');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should report the length error first when the password is short and does not match', () => {
            const result = validationFn('testuser', 'short', 'different');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });

        test('should accept the user addition when every field is valid', () => {
            const result = validationFn('validuser', '1234567890123456', '1234567890123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the user addition when the username is in email format', () => {
            const result = validationFn('user_test@example.com', 'SecurePassword123456', 'SecurePassword123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the user addition when the password mixes letters, digits and symbols', () => {
            const password = 'C0mpl3x!P@ssw0rd#2024$%^&*()';
            const result = validationFn('testuser', password, password);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
    });
}
