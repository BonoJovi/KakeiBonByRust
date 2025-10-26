/**
 * General User Edit Tests
 * 
 * Tests for the general user edit functionality in the user management screen.
 * This test suite validates:
 * - Username-only editing
 * - Password-only editing
 * - Username + Password editing
 * - Validation rules (reusing common validation modules)
 * 
 * Note: This uses the same test structure as admin-edit.test.js
 */

import { validateUserEdit } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';
import { testUsernameValidation } from './username-validation-tests.js';
import { runAllUserEditTests } from './user-edit-validation-tests.js';

describe('General User Edit - Complete Test Suite', () => {
    
    // Wrapper function to adapt validateUserEdit to password validation tests
    function validatePasswordOnly(password, passwordConfirm) {
        // Username is not required for password-only edit, so pass existing username
        return validateUserEdit('existinguser', password, passwordConfirm, true);
    }
    
    // Wrapper function for username validation
    function validateUsernameOnly(username) {
        // Password is not required for username-only edit
        return validateUserEdit(username, '', '', true);
    }
    
    // Test password validation using common module (26 tests)
    describe('Password Validation (Reused from common module)', () => {
        runAllPasswordTests(validatePasswordOnly, 'General User Edit Password');
    });
    
    // Test username validation using common module (13 tests)
    describe('Username Validation (Reused from common module)', () => {
        testUsernameValidation(validateUsernameOnly);
    });
    
    // Run all user edit specific tests (approximately 23 tests)
    runAllUserEditTests(validateUserEdit, 'General User Edit');
});

describe('General User Edit - Test Summary', () => {
    test('Test count summary', () => {
        // This test suite includes:
        // - 26 password tests (from password-validation-tests.js)
        // - 13 username tests (from username-validation-tests.js)
        // - 6 username-only edit tests
        // - 8 password-only edit tests
        // - 4 combined edit tests
        // - 5 edit mode vs add mode tests
        // Total: ~62 tests
        expect(true).toBe(true);
    });
});
