/**
 * User Addition Validation Tests
 * 
 * Tests for user addition form validation including username and password validation
 */

import { validateUserAddition } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';
import { testUsernameValidation, testCombinedValidation } from './username-validation-tests.js';

// Wrapper function to run password tests with username
const passwordValidationWithUsername = (password, passwordConfirm) => {
    return validateUserAddition('testuser', password, passwordConfirm);
};

describe('User Addition Validation', () => {
    // Test username-specific validation
    testUsernameValidation(validateUserAddition);
    
    // Test password validation (reusing common tests)
    runAllPasswordTests(passwordValidationWithUsername, 'Password Validation (with valid username)');
    
    // Test combined username and password validation
    testCombinedValidation(validateUserAddition);
});
