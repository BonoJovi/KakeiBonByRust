/**
 * Common Password Validation Test Suites
 * 
 * Reusable test suites for password validation across different forms
 */

import { validatePassword } from './validation-helpers.js';

/**
 * Test suite for empty password validation
 */
export function testEmptyPasswordValidation(validationFn) {
    describe('Empty Password Validation', () => {
        test('should reject the password when it is an empty string', () => {
            const result = validationFn('', 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject the password when it has only spaces', () => {
            const result = validationFn('   ', 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject the password when it has only tabs', () => {
            const result = validationFn('\t\t', 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject the password when it has only mixed whitespace', () => {
            const result = validationFn(' \t \n ', 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject the password when it is null', () => {
            const result = validationFn(null, 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject the password when it is undefined', () => {
            const result = validationFn(undefined, 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });
    });
}

/**
 * Test suite for password length validation
 */
export function testPasswordLengthValidation(validationFn) {
    describe('Password Length Validation', () => {
        test('should reject the password when it is shorter than 16 characters', () => {
            const result = validationFn('short', 'short');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });

        test('should reject the password when it has a single character', () => {
            const result = validationFn('a', 'a');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });

        test('should reject the password when it has exactly 15 characters', () => {
            const result = validationFn('123456789012345', '123456789012345');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });

        test('should accept the password when it has exactly 16 characters', () => {
            const result = validationFn('1234567890123456', '1234567890123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the password when it has more than 16 characters', () => {
            const result = validationFn('thisIsAVerySecurePassword', 'thisIsAVerySecurePassword');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the password when it is very long (100 characters)', () => {
            const longPassword = 'a'.repeat(100);
            const result = validationFn(longPassword, longPassword);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
    });
}

/**
 * Test suite for password matching validation
 */
export function testPasswordMatchValidation(validationFn) {
    describe('Password Matching Validation', () => {
        test('should reject the passwords when they do not match', () => {
            const result = validationFn('passwordpassword', 'differentpassword');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });

        test('should reject when password is correct but confirmation is empty', () => {
            const result = validationFn('passwordpassword', '');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });

        test('should reject when password is correct but confirmation is null', () => {
            const result = validationFn('passwordpassword', null);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });

        test('should reject the passwords when they differ only in letter case', () => {
            const result = validationFn('Passwordpassword', 'passwordpassword');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });

        test('should reject when passwords differ by one character', () => {
            const result = validationFn('1234567890123456', '1234567890123457');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });

        test('should accept when both passwords match exactly', () => {
            const result = validationFn('1234567890123456', '1234567890123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
    });
}

/**
 * Test suite for valid password scenarios
 */
export function testValidPasswordScenarios(validationFn) {
    describe('Valid Password Scenarios', () => {
        test('should accept the passwords when they match and have 16 characters', () => {
            const result = validationFn('1234567890123456', '1234567890123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept password with spaces when matching and long enough', () => {
            const result = validationFn('pass word 1234567', 'pass word 1234567');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the password when it has special characters', () => {
            const result = validationFn('p@ss!w0rd#123456', 'p@ss!w0rd#123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the password when it has unicode characters', () => {
            const password = 'パスワード12345678901234';
            const result = validationFn(password, password);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the password when it mixes letters, digits and symbols', () => {
            const password = 'C0mpl3x!P@ssw0rd#2024$%^&*()';
            const result = validationFn(password, password);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the password when it has only digits', () => {
            const result = validationFn('1234567890123456', '1234567890123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the password when it has only letters', () => {
            const result = validationFn('abcdefghijklmnop', 'abcdefghijklmnop');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept the password when it has only special characters', () => {
            const result = validationFn('!@#$%^&*()_+-={}', '!@#$%^&*()_+-={}');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
    });
}

/**
 * Run all password validation tests
 */
export function runAllPasswordTests(validationFn, suiteName = 'Password Validation') {
    describe(suiteName, () => {
        testEmptyPasswordValidation(validationFn);
        testPasswordLengthValidation(validationFn);
        testPasswordMatchValidation(validationFn);
        testValidPasswordScenarios(validationFn);
    });
}
