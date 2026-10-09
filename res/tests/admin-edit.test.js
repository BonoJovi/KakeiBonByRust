/**
 * Admin User Edit Tests
 * 
 * Tests for the admin user edit functionality in the user management screen.
 * This test suite validates:
 * - Username-only editing
 * - Password-only editing
 * - Username + Password editing
 * - Validation rules (reusing common validation modules)
 */

import { validateUserEdit } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';
import { testUsernameValidation } from './username-validation-tests.js';
import { runAllUserEditTests } from './user-edit-validation-tests.js';

describe('Admin User Edit - Complete Test Suite', () => {
    
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
        runAllPasswordTests(validatePasswordOnly, 'Admin User Edit Password');
    });
    
    // Test username validation using common module (13 tests)
    describe('Username Validation (Reused from common module)', () => {
        testUsernameValidation(validateUsernameOnly);
    });
    
    // Run all user edit specific tests (approximately 23 tests)
    runAllUserEditTests(validateUserEdit, 'Admin User Edit');
});
