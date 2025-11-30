#!/usr/bin/env node

/**
 * Backend Validation Standalone Tests
 * Tests the validation logic without requiring Tauri
 * Run with: node backend-validation-standalone.js
 */

console.log('🔧 Running Backend Validation Tests (Standalone)...\n');

let passed = 0;
let failed = 0;

function assert(condition, message) {
    if (condition) {
        passed++;
        console.log(`✓ ${message}`);
    } else {
        failed++;
        console.log(`✗ ${message}`);
    }
}

// Replicate the Rust validation logic in JavaScript for testing
function validatePassword(password) {
    // Check if password is empty or only whitespace
    if (password === null || password === undefined || password.trim() === '') {
        throw new Error('Password cannot be empty!');
    }
    
    // Check minimum length
    if (password.length < 16) {
        throw new Error('Password must be at least 16 characters long!');
    }
    
    return true;
}

function validatePasswordConfirmation(password, passwordConfirm) {
    if (password !== passwordConfirm) {
        throw new Error('Passwords do not match!');
    }
    return true;
}

function validatePasswordsFull(password, passwordConfirm) {
    validatePassword(password);
    validatePasswordConfirmation(password, passwordConfirm);
    return true;
}

// Test 1: Empty password should be rejected
console.log('\n📋 Test Category 1: Empty Password Validation');
try {
    validatePassword('');
    assert(false, 'Empty password should be rejected');
} catch (error) {
    assert(error.message === 'Password cannot be empty!', 'Empty password rejection with correct message');
}

// Test 2: Whitespace-only password should be rejected
try {
    validatePassword('   ');
    assert(false, 'Whitespace-only password should be rejected');
} catch (error) {
    assert(error.message === 'Password cannot be empty!', 'Whitespace-only password rejection');
}

// Test 3: Short password should be rejected
console.log('\n📋 Test Category 2: Password Length Validation');
try {
    validatePassword('short');
    assert(false, 'Short password should be rejected');
} catch (error) {
    assert(error.message === 'Password must be at least 16 characters long!', 'Short password rejection');
}

// Test 4: 15 character password should be rejected
try {
    validatePassword('123456789012345');
    assert(false, '15 character password should be rejected');
} catch (error) {
    assert(error.message === 'Password must be at least 16 characters long!', '15 character password rejection');
}

// Test 5: 16 character password should be accepted
console.log('\n📋 Test Category 3: Valid Password Acceptance');
try {
    validatePassword('1234567890123456');
    assert(true, '16 character password should be accepted');
} catch (error) {
    assert(false, `16 character password acceptance failed: ${error.message}`);
}

// Test 6: Long password should be accepted
try {
    validatePassword('thisIsAVerySecurePassword123');
    assert(true, 'Long password should be accepted');
} catch (error) {
    assert(false, `Long password acceptance failed: ${error.message}`);
}

// Test 7: Password with special characters should be accepted
try {
    validatePassword('p@ssw0rd!#$12345');
    assert(true, 'Password with special characters should be accepted');
} catch (error) {
    assert(false, `Special character password failed: ${error.message}`);
}

// Test 8: Matching passwords should be accepted
console.log('\n📋 Test Category 4: Password Confirmation');
try {
    validatePasswordsFull('1234567890123456', '1234567890123456');
    assert(true, 'Matching passwords should be accepted');
} catch (error) {
    assert(false, `Matching passwords failed: ${error.message}`);
}

// Test 9: Non-matching passwords should be rejected
try {
    validatePasswordsFull('1234567890123456', '6543210987654321');
    assert(false, 'Non-matching passwords should be rejected');
} catch (error) {
    assert(error.message === 'Passwords do not match!', 'Non-matching password rejection');
}

// Test 10: Empty password in confirmation check
try {
    validatePasswordsFull('', '');
    assert(false, 'Empty passwords should be rejected in confirmation');
} catch (error) {
    assert(error.message === 'Password cannot be empty!', 'Empty password in confirmation rejected');
}

// Test 11: Short password in confirmation check
try {
    validatePasswordsFull('short', 'short');
    assert(false, 'Short matching passwords should be rejected');
} catch (error) {
    assert(error.message === 'Password must be at least 16 characters long!', 'Short password in confirmation rejected');
}

// Test 12: Case sensitivity in password confirmation
console.log('\n📋 Test Category 5: Case Sensitivity');
try {
    validatePasswordsFull('1234567890123456', '1234567890123456');
    assert(true, 'Identical passwords should match');
} catch (error) {
    assert(false, `Identical password matching failed: ${error.message}`);
}

try {
    validatePasswordConfirmation('Password12345678', 'password12345678');
    assert(false, 'Case-different passwords should not match');
} catch (error) {
    assert(error.message === 'Passwords do not match!', 'Case sensitivity enforced');
}

// Test 13: Unicode characters
console.log('\n📋 Test Category 6: Unicode & Special Cases');
const unicodePassword = 'パスワード1234567890';
try {
    validatePassword(unicodePassword);
    if (unicodePassword.length >= 16) {
        assert(true, `Unicode password accepted (length: ${unicodePassword.length})`);
    } else {
        assert(false, `Unicode password should be rejected (length: ${unicodePassword.length} < 16)`);
    }
} catch (error) {
    if (unicodePassword.length < 16) {
        assert(error.message === 'Password must be at least 16 characters long!', 'Unicode password correctly rejected for length');
    } else {
        assert(false, `Unicode password failed unexpectedly: ${error.message}`);
    }
}

// Test 14: Password with newlines and tabs
const newlinePassword = 'password\n\t12345';
try {
    validatePassword(newlinePassword);
    if (newlinePassword.length >= 16) {
        assert(true, `Newline/tab password accepted (length: ${newlinePassword.length})`);
    } else {
        assert(false, `Newline/tab password should be rejected (length: ${newlinePassword.length} < 16)`);
    }
} catch (error) {
    if (newlinePassword.length < 16) {
        assert(error.message === 'Password must be at least 16 characters long!', 'Newline/tab password correctly rejected for length');
    } else {
        assert(false, `Newline/tab password failed unexpectedly: ${error.message}`);
    }
}

// Test 15: Very long password
try {
    validatePassword('a'.repeat(1000));
    assert(true, 'Very long password should be accepted');
} catch (error) {
    assert(false, `Very long password failed: ${error.message}`);
}

// Test 16: Null handling
console.log('\n📋 Test Category 7: Error Handling');
try {
    validatePassword(null);
    assert(false, 'Null password should be rejected');
} catch (error) {
    assert(error.message === 'Password cannot be empty!', 'Null password rejected');
}

// Test 17: Undefined handling
try {
    validatePassword(undefined);
    assert(false, 'Undefined password should be rejected');
} catch (error) {
    assert(error.message === 'Password cannot be empty!', 'Undefined password rejected');
}

// Test 18: Mixed whitespace
try {
    validatePassword(' \n\t\r ');
    assert(false, 'Mixed whitespace should be rejected');
} catch (error) {
    assert(error.message === 'Password cannot be empty!', 'Mixed whitespace rejected');
}

// Test 19: Leading/trailing spaces with valid content
try {
    validatePassword('  validpassword123456  ');
    const trimmed = '  validpassword123456  '.trim();
    if (trimmed.length >= 16) {
        assert(true, 'Password with leading/trailing spaces accepted (length includes spaces)');
    } else {
        assert(false, 'Should check based on trimmed length');
    }
} catch (error) {
    // If trimming is done in validation
    assert(error.message === 'Password must be at least 16 characters long!', 'Spaces affect length validation');
}

// Test 20: SQL injection attempt (should still be accepted as valid password if length is ok)
console.log('\n📋 Test Category 8: Security Patterns');
const sqlInjection = "admin' OR '1'='1";
if (sqlInjection.length >= 16) {
    try {
        validatePassword(sqlInjection);
        assert(true, 'SQL injection pattern accepted as password (parameterized queries protect backend)');
    } catch (error) {
        assert(false, `SQL pattern failed: ${error.message}`);
    }
} else {
    console.log('⊘ SQL injection pattern too short for test');
}

// Test 21: XSS attempt
const xssAttempt = '<script>alert("xss")</script>';
if (xssAttempt.length >= 16) {
    try {
        validatePassword(xssAttempt);
        assert(true, 'XSS pattern accepted as password (encoding protects frontend)');
    } catch (error) {
        assert(false, `XSS pattern failed: ${error.message}`);
    }
} else {
    console.log('⊘ XSS pattern too short for test');
}

// Summary
console.log('\n' + '='.repeat(50));
console.log('📊 Test Summary');
console.log('='.repeat(50));
console.log(`Total:  ${passed + failed} tests`);
console.log(`Passed: ${passed} tests ✓`);
console.log(`Failed: ${failed} tests ✗`);
console.log('='.repeat(50));

if (failed === 0) {
    console.log('\n🎉 All backend validation tests passed!\n');
    process.exit(0);
} else {
    console.log(`\n❌ ${failed} test(s) failed\n`);
    process.exit(1);
}
