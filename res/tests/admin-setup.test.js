/**
 * Admin Setup Validation Tests
 * 
 * Tests for password validation in the admin user registration form
 */

import { validatePassword } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';

// Run all standard password validation tests
runAllPasswordTests(validatePassword, 'Admin Setup Password Validation');

// Additional admin-specific edge case tests
describe('Admin Setup Specific Edge Cases', () => {
    test('should handle password with leading/trailing spaces if matching and long enough', () => {
        const result = validatePassword(' passwordpassword ', ' passwordpassword ');
        expect(result.valid).toBe(true);
        expect(result.message).toBe('');
    });

    test('should accept very long password', () => {
        const longPassword = 'a'.repeat(1000);
        const result = validatePassword(longPassword, longPassword);
        expect(result.valid).toBe(true);
        expect(result.message).toBe('');
    });

    test('should accept password with emojis', () => {
        const result = validatePassword('password🔐🔐🔐🔐123', 'password🔐🔐🔐🔐123');
        expect(result.valid).toBe(true);
        expect(result.message).toBe('');
    });

    test('should handle password with newlines (not trimmed)', () => {
        const result = validatePassword('\npasswordpassword\n', 'passwordpassword');
        expect(result.valid).toBe(false);
        expect(result.message).toBe('Passwords do not match!');
    });

    test('should handle zero-width space', () => {
        const zeroWidthSpace = '\u200B';
        const result = validatePassword(zeroWidthSpace, zeroWidthSpace);
        expect(result.valid).toBe(false);
        expect(result.message).toBe('Password must be at least 16 characters long!');
    });

    test('should reject short numeric password', () => {
        const result = validatePassword('123456', '123456');
        expect(result.valid).toBe(false);
        expect(result.message).toBe('Password must be at least 16 characters long!');
    });
});
