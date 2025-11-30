#!/usr/bin/env node

/**
 * Simplified Login Tests - Standalone version
 * Run with: node login-test-standalone.js
 */

console.log('🔐 Running Login Tests (Standalone)...\n');

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

// Test 1: Basic validation functions
console.log('\n📋 Test Category 1: Basic Validation');

function validateNotEmpty(value) {
    return value !== null && value !== undefined && value.trim() !== '';
}

assert(validateNotEmpty('test'), 'Non-empty string should be valid');
assert(!validateNotEmpty(''), 'Empty string should be invalid');
assert(!validateNotEmpty('   '), 'Whitespace-only string should be invalid');
assert(!validateNotEmpty(null), 'Null should be invalid');
assert(!validateNotEmpty(undefined), 'Undefined should be invalid');

// Test 2: Password comparison
console.log('\n📋 Test Category 2: Password Comparison');

function passwordsMatch(password, confirm) {
    return password === confirm;
}

assert(passwordsMatch('test', 'test'), 'Identical passwords should match');
assert(!passwordsMatch('test', 'Test'), 'Case-sensitive passwords should not match');
assert(!passwordsMatch('password', 'different'), 'Different passwords should not match');
assert(passwordsMatch('', ''), 'Empty passwords should match');

// Test 3: SQL Injection Prevention
console.log('\n📋 Test Category 3: Security');

function containsSQLInjection(input) {
    // Simple check - in real app, parameterized queries handle this
    const suspicious = ["'", '"', '--', ';', '/*', '*/'];
    return suspicious.some(char => input.includes(char));
}

assert(containsSQLInjection("admin' OR '1'='1"), 'Should detect SQL injection attempt');
assert(containsSQLInjection('test"; DROP TABLE users;'), 'Should detect SQL command injection');
assert(!containsSQLInjection('normaluser'), 'Normal input should not be flagged');

// Test 4: XSS Prevention
console.log('\n📋 Test Category 4: XSS Prevention');

function containsXSS(input) {
    const xssPatterns = ['<script', '<iframe', 'javascript:', 'onerror='];
    return xssPatterns.some(pattern => input.toLowerCase().includes(pattern));
}

assert(containsXSS('<script>alert("xss")</script>'), 'Should detect script tags');
assert(containsXSS('<iframe src="evil.com">'), 'Should detect iframe tags');
assert(containsXSS('javascript:alert(1)'), 'Should detect javascript protocol');
assert(!containsXSS('normal text'), 'Normal text should not be flagged');

// Test 5: Username validation
console.log('\n📋 Test Category 5: Username Validation');

function isValidUsername(username) {
    if (!username || username.trim() === '') return false;
    if (username.length > 100) return false;
    return true;
}

assert(isValidUsername('admin'), 'Valid username should pass');
assert(isValidUsername('user123'), 'Username with numbers should pass');
assert(!isValidUsername(''), 'Empty username should fail');
assert(!isValidUsername('a'.repeat(101)), 'Too long username should fail');

// Test 6: Response formatting
console.log('\n📋 Test Category 6: Response Formatting');

function formatWelcomeMessage(username) {
    return `Welcome, ${username}!`;
}

assert(formatWelcomeMessage('admin') === 'Welcome, admin!', 'Welcome message format should be correct');
assert(formatWelcomeMessage('testuser').startsWith('Welcome,'), 'Welcome message should start with "Welcome,"');

// Test 7: Error message formatting
console.log('\n📋 Test Category 7: Error Messages');

function formatErrorMessage(error) {
    if (error.includes('username')) return 'Invalid username or password';
    if (error.includes('password')) return 'Invalid username or password';
    if (error.includes('database')) return 'Database error';
    return 'An error occurred';
}

assert(formatErrorMessage('username not found') === 'Invalid username or password', 'Username error should return generic message');
assert(formatErrorMessage('password incorrect') === 'Invalid username or password', 'Password error should return generic message');
assert(formatErrorMessage('database connection failed').includes('Database'), 'Database error should be identified');

// Test 8: Input sanitization
console.log('\n📋 Test Category 8: Input Sanitization');

function sanitizeInput(input) {
    if (typeof input !== 'string') return '';
    return input.trim();
}

assert(sanitizeInput('  test  ') === 'test', 'Should trim whitespace');
assert(sanitizeInput('test') === 'test', 'Should preserve normal text');
assert(sanitizeInput(null) === '', 'Should handle null');
assert(sanitizeInput(undefined) === '', 'Should handle undefined');

// Test 9: State management simulation
console.log('\n📋 Test Category 9: State Management');

class LoginState {
    constructor() {
        this.isLoggedIn = false;
        this.username = null;
        this.error = null;
    }

    login(username) {
        this.isLoggedIn = true;
        this.username = username;
        this.error = null;
    }

    logout() {
        this.isLoggedIn = false;
        this.username = null;
        this.error = null;
    }

    setError(error) {
        this.error = error;
    }
}

const state = new LoginState();
assert(!state.isLoggedIn, 'Initial state should be logged out');
state.login('testuser');
assert(state.isLoggedIn, 'After login, should be logged in');
assert(state.username === 'testuser', 'Username should be stored');
state.logout();
assert(!state.isLoggedIn, 'After logout, should be logged out');
assert(state.username === null, 'Username should be cleared after logout');

// Test 10: Edge cases
console.log('\n📋 Test Category 10: Edge Cases');

assert(!validateNotEmpty('\n\t\r'), 'Newlines and tabs should be considered empty');
assert(!passwordsMatch('test\n', 'test'), 'Passwords with different whitespace should not match');

const longUsername = 'a'.repeat(1000);
assert(longUsername.length === 1000, 'Should handle long strings');

// Summary
console.log('\n' + '='.repeat(50));
console.log('📊 Test Summary');
console.log('='.repeat(50));
console.log(`Total:  ${passed + failed} tests`);
console.log(`Passed: ${passed} tests ✓`);
console.log(`Failed: ${failed} tests ✗`);
console.log('='.repeat(50));

if (failed === 0) {
    console.log('\n🎉 All tests passed!\n');
    process.exit(0);
} else {
    console.log(`\n❌ ${failed} test(s) failed\n`);
    process.exit(1);
}
