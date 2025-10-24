/**
 * Admin Setup Validation Tests
 * 
 * Tests for password validation in the admin user registration form
 */

describe('Admin Setup Password Validation', () => {
    // Validation function (copied from menu.js logic)
    function validatePassword(password, passwordConfirm) {
        if (!password || password.trim() === '') {
            return {
                valid: false,
                message: 'Password cannot be empty!'
            };
        }
        
        if (password.length < 16) {
            return {
                valid: false,
                message: 'Password must be at least 16 characters long!'
            };
        }
        
        if (password !== passwordConfirm) {
            return {
                valid: false,
                message: 'Passwords do not match!'
            };
        }
        
        return {
            valid: true,
            message: ''
        };
    }

    describe('Empty Password Validation', () => {
        test('should reject empty string password', () => {
            const result = validatePassword('', 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject password with only spaces', () => {
            const result = validatePassword('   ', 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject password with only tabs', () => {
            const result = validatePassword('\t\t', 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject password with mixed whitespace', () => {
            const result = validatePassword(' \t \n ', 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject null password', () => {
            const result = validatePassword(null, 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });

        test('should reject undefined password', () => {
            const result = validatePassword(undefined, 'test123');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password cannot be empty!');
        });
    });

    describe('Password Length Validation', () => {
        test('should reject password shorter than 16 characters', () => {
            const result = validatePassword('short', 'short');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });

        test('should reject single character password', () => {
            const result = validatePassword('a', 'a');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });

        test('should reject password with exactly 15 characters', () => {
            const result = validatePassword('123456789012345', '123456789012345');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });

        test('should accept password with exactly 16 characters', () => {
            const result = validatePassword('1234567890123456', '1234567890123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept password with more than 16 characters', () => {
            const result = validatePassword('thisIsAVerySecurePassword', 'thisIsAVerySecurePassword');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
    });

    describe('Password Matching Validation', () => {
        test('should reject non-matching passwords', () => {
            const result = validatePassword('passwordpassword', 'differentpassword');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });

        test('should reject when password is correct but confirmation is empty', () => {
            const result = validatePassword('passwordpassword', '');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });

        test('should reject when password is correct but confirmation is null', () => {
            const result = validatePassword('passwordpassword', null);
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });

        test('should reject case-sensitive mismatch', () => {
            const result = validatePassword('Passwordpassword', 'passwordpassword');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Passwords do not match!');
        });
    });

    describe('Valid Password Validation', () => {
        test('should accept valid matching passwords with 16 characters', () => {
            const result = validatePassword('1234567890123456', '1234567890123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept password with spaces when matching and long enough', () => {
            const result = validatePassword('pass word 1234567', 'pass word 1234567');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept password with special characters', () => {
            const result = validatePassword('p@ss!w0rd#123456', 'p@ss!w0rd#123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept password with leading/trailing spaces if matching and long enough', () => {
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

        test('should accept password with unicode characters', () => {
            const result = validatePassword('パスワード12345678901', 'パスワード12345678901');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should accept password with emojis', () => {
            const result = validatePassword('password🔐🔐🔐🔐123', 'password🔐🔐🔐🔐123');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
    });

    describe('Edge Cases', () => {
        test('should handle password with newlines (trimmed)', () => {
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

        test('should handle numeric password with 16 digits', () => {
            const result = validatePassword('1234567890123456', '1234567890123456');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });

        test('should reject short numeric password', () => {
            const result = validatePassword('123456', '123456');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 16 characters long!');
        });
    });
});
