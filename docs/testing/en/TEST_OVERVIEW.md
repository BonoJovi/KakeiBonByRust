# Test Overview

This document explains the test strategy, execution methods, and overall test structure of the KakeiBon project.

**Last Updated**: 2025-12-06 06:45 JST  
**Total Tests**: 463+ (Rust 201 + JavaScript 262+)

---

## Table of Contents

- [Test Philosophy](#test-philosophy)
- [Test Structure](#test-structure)
- [Quick Start](#quick-start)
- [Test Indexes](#test-indexes)
- [How to Run Tests](#how-to-run-tests)
- [Adding New Tests](#adding-new-tests)
- [Test Design Principles](#test-design-principles)
- [CI/CD Integration](#cicd-integration)
- [Troubleshooting](#troubleshooting)

---

## Test Philosophy

KakeiBon adopts a test strategy that balances quality assurance and development efficiency.

### Core Principles

1. **DRY (Don't Repeat Yourself)**
   - Modularize test cases for reuse across multiple screens/features
   - Share validation logic

2. **Maintainability**
   - Treat test code with same care as production code
   - Minimize changes needed when validation rules change

3. **Consistency**
   - Apply same validation rules in frontend and backend
   - Unified error messages across all screens

4. **Indexing**
   - All test cases indexed in table format
   - Understand test status and prevent missing implementations

---

## Test Structure

### 3-Tier Test Architecture

```
┌─────────────────────────────────────────────┐
│  Frontend (JavaScript)                      │
│  - Screen-specific tests: 206               │
│  - Common test suites: 56                   │
│  - Feature tests: 118+                      │
│  - Aggregation tests: many                  │
└──────────────┬──────────────────────────────┘
               │
               │ Tauri IPC
               │
┌──────────────▼──────────────────────────────┐
│  Backend (Rust)                             │
│  - Common test suites: 23                   │
│  - Inline tests: 178                        │
│  - Database, Security, Encryption           │
│  - Service layer (Auth, User Mgmt, etc.)    │
└─────────────────────────────────────────────┘
```

---

## Quick Start

### Run all tests

From project root:

```bash
# Run Rust tests
cargo test

# Run JavaScript tests
cd res/tests
npm install  # First time only
npm test
```

### Run specific tests

```bash
# Rust: Validation tests only
cargo test validation::

# JavaScript: Login tests only
cd res/tests
npm test login.test.js
```

---

## Test Indexes

For complete test case listings, see these index documents:

### [BlueBook] [Backend Test Index](BACKEND_TEST_INDEX.md)
- **Total Tests**: 201
- All Rust test cases in table format
- Includes test function name, description, file, line number

### [GreenBook] [Frontend Test Index](FRONTEND_TEST_INDEX.md)
- **Total Tests**: 262+
- All JavaScript test cases in table format
- Includes test name, description, usage

---

## How to Run Tests

### Rust Tests

#### Run all tests

```bash
cargo test
```

Expected output:
```
running 201 tests
test validation::tests::test_empty_password ... ok
test security::tests::test_hash_password ... ok
test services::auth::tests::test_register_admin_user ... ok
...
test result: ok. 201 passed; 0 failed; 0 ignored
```

#### Run specific module

```bash
# Validation only
cargo test validation::

# Auth service only
cargo test services::auth::

# User management service only
cargo test services::user_management::
```

#### Run specific test function

```bash
cargo test test_empty_password
cargo test test_register_admin_user
```

#### Run with output

```bash
cargo test -- --nocapture
```

#### Generate coverage report

```bash
# Install cargo-tarpaulin (first time only)
cargo install cargo-tarpaulin

# Generate HTML report
cargo tarpaulin --out Html

# Open tarpaulin-report.html in browser
```

### JavaScript Tests

#### Run all tests

```bash
cd res/tests
npm test
```

Expected output:
```
PASS  ./admin-setup.test.js
PASS  ./login.test.js
PASS  ./user-deletion.test.js
...
Tests: 262 passed, 262 total
```

#### Run specific test file

```bash
npm test admin-setup.test.js
npm test login.test.js
npm test user-deletion.test.js
```

#### Run specific test case

```bash
# Filter by test name
npm test -- --testNamePattern="Empty Password"
npm test -- --testNamePattern="Username Validation"
```

#### Watch mode (monitor file changes)

```bash
npm test -- --watch
```

#### Generate coverage report

```bash
npm run test:coverage
```

#### Standalone tests (Node.js, no dependencies)

```bash
node login-test-standalone.js
node backend-validation-standalone.js
```

---

## Adding New Tests

### Adding Rust Tests

#### Pattern 1: Add to common test suite (recommended)

When adding new test cases to existing functionality:

```rust
// src/validation_tests.rs
pub fn test_new_password_rule() {
    let result = validate_password("new rule test", "new rule test");
    assert!(result.is_ok());
}
```

#### Pattern 2: Add inline test to new service

When creating a new service module:

```rust
// src/services/new_service.rs

// Service implementation
pub async fn new_function() -> Result<(), String> {
    // ...
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_new_function() {
        let result = new_function().await;
        assert!(result.is_ok());
    }
}
```

### Adding JavaScript Tests

#### Pattern 1: Add to common test suite (recommended)

When adding new test cases to existing validation:

```javascript
// res/tests/password-validation-tests.js

export function testNewPasswordRule(validationFn) {
    describe('New Password Rule', () => {
        test('should enforce new rule', () => {
            const result = validationFn('test', 'test');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('New rule error!');
        });
    });
}
```

#### Pattern 2: Add tests for new screen

When adding a new screen:

```javascript
// res/tests/new-screen.test.js

import { validatePassword } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';

// Run common password tests
runAllPasswordTests(validatePassword, 'New Screen Password Validation');

// Add screen-specific tests
describe('New Screen Specific Tests', () => {
    test('specific edge case', () => {
        const result = validatePassword('specific case', 'specific case');
        expect(result.valid).toBe(true);
    });
});
```

### Update Test Indexes

When adding new tests, always update the index documents:

1. [BACKEND_TEST_INDEX.md](BACKEND_TEST_INDEX.md) - When adding Rust tests
2. [FRONTEND_TEST_INDEX.md](FRONTEND_TEST_INDEX.md) - When adding JavaScript tests

Add to table:
- Test function name
- Description (brief)
- File name
- Line number (optional)

---

## Test Design Principles

For detailed test design philosophy, see [TEST_DESIGN.md](TEST_DESIGN.md).

### Key Principles

1. **Sharing and DRY Principle**
   - Same validation logic in common modules
   - Same test cases in common test suites
   - Screen-specific logic only in individual files

2. **Clear Test Names**
   ```javascript
   // ✓ Good
   test('should reject password shorter than 16 characters', () => { });
   
   // ✗ Bad
   test('password test', () => { });
   ```

3. **Unified Error Messages**
   ```rust
   // ✓ Good - Define as constants
   const ERROR_PASSWORD_TOO_SHORT: &str = "Password must be at least 16 characters long!";
   
   // ✗ Bad - Different messages per screen
   ```

4. **Test Independence**
   - Don't share state between tests
   - Each test can run independently
   - Avoid global variables

---

## CI/CD Integration

### GitHub Actions Example

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  rust-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run Rust tests
        run: cargo test --verbose

  javascript-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Node.js
        uses: actions/setup-node@v2
        with:
          node-version: '18'
      - name: Install dependencies
        run: |
          cd res/tests
          npm install
      - name: Run JavaScript tests
        run: |
          cd res/tests
          npm test
```

---

## Troubleshooting

### Rust Tests Failing

#### Build errors

```bash
# Clean build
cargo clean
cargo build
cargo test
```

#### Specific test failing

```bash
# Run specific test with details
cargo test test_name -- --nocapture
```

### JavaScript Tests Failing

#### ES Modules error

```
SyntaxError: Cannot use import statement outside a module
```

**Solution:**
1. Verify `"type": "module"` in `package.json`
2. Check import statements include `.js` extension
3. Add `--experimental-vm-modules` to Jest command

```bash
node --experimental-vm-modules node_modules/jest/bin/jest.js
```

#### Dependency errors

```bash
cd res/tests
rm -rf node_modules package-lock.json
npm install
npm test
```

#### Test not found

```
describe is not defined
```

**Solution:**
Specify `testEnvironment: "jsdom"` in Jest config

```json
// package.json
{
  "jest": {
    "testEnvironment": "jsdom",
    "transform": {}
  }
}
```

---

## Test Statistics

| Category | Test Count | Status |
|----------|------------|--------|
| **Rust Backend** | **201** | ✅ |
| Common test suites | 23 | ✅ |
| Inline tests | 178 | ✅ |
| **JavaScript Frontend** | **262+** | ✅ |
| Common test suites | 56 | ✅ |
| Screen-specific tests | 206 | ✅ |
| Feature tests | 118+ | ✅ |
| Aggregation tests | many | ✅ |
| **Total** | **463+** | ✅ |

---

## Contributor Guidelines

### Reviewing Test Cases

1. **Understand overall structure with indexes**
   - [BACKEND_TEST_INDEX.md](BACKEND_TEST_INDEX.md)
   - [FRONTEND_TEST_INDEX.md](FRONTEND_TEST_INDEX.md)

2. **Check test coverage**
   ```bash
   # Rust
   cargo tarpaulin --out Html
   
   # JavaScript
   cd res/tests
   npm run test:coverage
   ```

3. **Check for missing implementations**
   - Review indexes to verify tests exist for each feature
   - Always add tests when adding new features

4. **Verify test validity**
   - Test names accurately describe contents
   - Edge cases are covered
   - Error messages are consistent

### Pull Request Checklist

- [ ] All tests pass (Rust + JavaScript)
- [ ] Added tests for new features
- [ ] Updated test indexes
- [ ] Coverage hasn't decreased
- [ ] Test names are clear and consistent

---

## Related Documents

- [BlueBook] [Backend Test Index](BACKEND_TEST_INDEX.md) - Complete Rust test list
- [GreenBook] [Frontend Test Index](FRONTEND_TEST_INDEX.md) - Complete JavaScript test list
- [OrangeBook] [Test Design](TEST_DESIGN.md) - Test architecture and design philosophy
- [RedBook] [Test Results](TEST_RESULTS.md) - Latest test execution results and coverage

---

**For questions or suggestions, please create an Issue.**
