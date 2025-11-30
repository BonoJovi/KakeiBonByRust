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
        
        test('should allow valid username change without password', () => {
            const result = validateFunc('newusername', '', '', true);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
        
        test('should reject empty username even if no password change', () => {
            const result = validateFunc('', '', '', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });
        
        test('should reject whitespace-only username', () => {
            const result = validateFunc('   ', '', '', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });
        
        test('should allow username with special characters', () => {
            const result = validateFunc('user@#$%', '', '', true);
            expect(result.valid).toBe(true);
        });
        
        test('should allow username with unicode', () => {
            const result = validateFunc('ユーザー名', '', '', true);
            expect(result.valid).toBe(true);
        });
        
        test('should allow very long username', () => {
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
        
        test('should allow valid password change with unchanged username', () => {
            const result = validateFunc('existinguser', '1234567890123456', '1234567890123456', true);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
        
        test('should reject password shorter than 16 characters', () => {
            const result = validateFunc('existinguser', 'short', 'short', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });
        
        test('should reject password confirmation mismatch', () => {
            const result = validateFunc('existinguser', '1234567890123456', '6543210987654321', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });
        
        test('should reject password without confirmation', () => {
            const result = validateFunc('existinguser', '1234567890123456', '', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });
        
        test('should reject empty password with confirmation', () => {
            const result = validateFunc('existinguser', '', '1234567890123456', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });
        
        test('should allow password with spaces', () => {
            const password = 'my secure password 16';
            const result = validateFunc('existinguser', password, password, true);
            expect(result.valid).toBe(true);
        });
        
        test('should allow password with special characters', () => {
            const password = 'p@ssw0rd!#$12345';
            const result = validateFunc('existinguser', password, password, true);
            expect(result.valid).toBe(true);
        });
        
        test('should allow password with unicode', () => {
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
        
        test('should allow changing both username and password', () => {
            const result = validateFunc('newusername', '1234567890123456', '1234567890123456', true);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
        
        test('should reject empty username even with valid password', () => {
            const result = validateFunc('', '1234567890123456', '1234567890123456', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Username cannot be empty!');
        });
        
        test('should reject valid username with short password', () => {
            const result = validateFunc('newusername', 'short', 'short', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });
        
        test('should reject valid username with mismatched passwords', () => {
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
        
        test('should allow empty password in edit mode', () => {
            const result = validateFunc('username', '', '', true);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
        
        test('should reject empty password in add mode', () => {
            const result = validateFunc('username', '', '', false);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });
        
        test('should require valid password if provided in edit mode', () => {
            const result = validateFunc('username', 'short', 'short', true);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });
        
        test('should require valid password in add mode', () => {
            const result = validateFunc('username', '1234567890123456', '1234567890123456', false);
            expect(result.valid).toBe(true);
        });
        
        test('should require password confirmation in edit mode if password provided', () => {
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
