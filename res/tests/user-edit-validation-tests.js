/**
 * User Edit Validation Test Suite
 * 
 * This module provides reusable test suites for user edit validation.
 * Can be used for both admin user edit and general user edit screens.
 * Similar to password-validation-tests.js and username-validation-tests.js
 */

/**
 * Test suite for username-only edit validation
 * Tests editing username while leaving password empty
 */
export function testUsernameOnlyEdit(validateFunc) {
    describe('Username-Only Edit Tests', () => {
        
        test('should allow a username change when no password is entered', () => {
            const result = validateFunc('newusername', '', '', true);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
        
        test('should reject the username when it is empty and no password is entered', () => {
            const result = validateFunc('', '', '', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });
        
        test('should reject the username when it has only whitespace', () => {
            const result = validateFunc('   ', '', '', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });
        
        test('should allow the username when it has special characters', () => {
            const result = validateFunc('user@#$%', '', '', true);
            expect(result.valid).toBe(true);
        });
        
        test('should allow the username when it has unicode characters', () => {
            const result = validateFunc('ユーザー名', '', '', true);
            expect(result.valid).toBe(true);
        });
        
        test('should allow the username when it is very long (128 characters)', () => {
            const longUsername = 'a'.repeat(128);
            const result = validateFunc(longUsername, '', '', true);
            expect(result.valid).toBe(true);
        });
    });
}

/**
 * Test suite for password-only edit validation
 * Tests changing password while keeping username unchanged
 */
export function testPasswordOnlyEdit(validateFunc) {
    describe('Password-Only Edit Tests', () => {
        
        test('should allow a password change when the username is unchanged', () => {
            const result = validateFunc('existinguser', '1234567890123456', '1234567890123456', true);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
        
        test('should reject the password when it is shorter than 16 characters', () => {
            const result = validateFunc('existinguser', 'short', 'short', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });
        
        test('should reject the password when the confirmation does not match', () => {
            const result = validateFunc('existinguser', '1234567890123456', '6543210987654321', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });
        
        test('should reject the password when the confirmation is empty', () => {
            const result = validateFunc('existinguser', '1234567890123456', '', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });
        
        test('should reject the password when it is empty but the confirmation is filled', () => {
            const result = validateFunc('existinguser', '', '1234567890123456', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });
        
        test('should allow the password when it has spaces', () => {
            const password = 'my secure password 16';
            const result = validateFunc('existinguser', password, password, true);
            expect(result.valid).toBe(true);
        });
        
        test('should allow the password when it has special characters', () => {
            const password = 'p@ssw0rd!#$12345';
            const result = validateFunc('existinguser', password, password, true);
            expect(result.valid).toBe(true);
        });
        
        test('should allow the password when it has unicode characters', () => {
            const password = 'パスワード12345678901';
            const result = validateFunc('existinguser', password, password, true);
            expect(result.valid).toBe(true);
        });
    });
}

/**
 * Test suite for combined username and password edit
 * Tests changing both username and password together
 */
export function testCombinedEdit(validateFunc) {
    describe('Combined Username and Password Edit Tests', () => {
        
        test('should allow the change when both the username and the password change', () => {
            const result = validateFunc('newusername', '1234567890123456', '1234567890123456', true);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
        
        test('should reject the username when it is empty even though the password is valid', () => {
            const result = validateFunc('', '1234567890123456', '1234567890123456', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });
        
        test('should reject the change when the username is valid but the password is short', () => {
            const result = validateFunc('newusername', 'short', 'short', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });
        
        test('should reject the change when the username is valid but the passwords do not match', () => {
            const result = validateFunc('newusername', '1234567890123456', '6543210987654321', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });
    });
}

/**
 * Test suite for edit mode vs add mode behavior
 * Tests the difference between editing and adding users
 */
export function testEditModeVsAddMode(validateFunc) {
    describe('Edit Mode vs Add Mode Tests', () => {
        
        test('should allow an empty password when in edit mode', () => {
            const result = validateFunc('username', '', '', true);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
        
        test('should reject an empty password when in add mode', () => {
            const result = validateFunc('username', '', '', false);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });
        
        test('should reject a short password when one is entered in edit mode', () => {
            const result = validateFunc('username', 'short', 'short', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });
        
        test('should accept a valid password when in add mode', () => {
            const result = validateFunc('username', '1234567890123456', '1234567890123456', false);
            expect(result.valid).toBe(true);
        });
        
        test('should require the confirmation when a password is entered in edit mode', () => {
            const result = validateFunc('username', '1234567890123456', '', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });
    });
}

/**
 * Run all user edit validation tests
 * Executes all test suites for complete coverage
 */
export function runAllUserEditTests(validateFunc, contextName = 'User Edit') {
    describe(`${contextName} - All Validation Tests`, () => {
        testUsernameOnlyEdit(validateFunc);
        testPasswordOnlyEdit(validateFunc);
        testCombinedEdit(validateFunc);
        testEditModeVsAddMode(validateFunc);
    });
}
