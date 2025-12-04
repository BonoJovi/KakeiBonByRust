# Testing Guide

**Last Updated**: 2024-12-05 05:21 JST

## Overview

This document provides guidelines for testing in the KakeiBon project. The project emphasizes comprehensive testing to ensure code quality and reliability.

---

## Testing Philosophy

### Core Principles

1. **Test Coverage**: Aim for high test coverage (currently 201+ Rust tests)
2. **Test Pyramid**: Unit tests > Integration tests > E2E tests
3. **Test Maintainability**: Keep tests simple, readable, and maintainable
4. **Continuous Testing**: Run tests frequently during development

### Test Types

| Test Type | Purpose | Location |
|-----------|---------|----------|
| Unit Tests | Test individual functions/modules | `src/**/*.rs` (inline) |
| Integration Tests | Test module interactions | `tests/` directory |
| Frontend Tests | Test UI logic and validation | `res/tests/` |

---

## Backend Testing (Rust)

### Test Structure

**Pattern**: "should [expected behavior] when [condition]"

```rust
#[test]
fn should_accept_valid_password_when_16_characters() {
    // Arrange
    let password = "a".repeat(16);
    
    // Act
    let result = validate_password(&password);
    
    // Assert
    assert!(result.is_ok());
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with coverage
cargo tarpaulin --out Html
```

### Test Helpers

Common test utilities are centralized in `src/test_helpers.rs`:

```rust
use crate::test_helpers::*;

#[test]
fn my_test() {
    let test_user = create_test_user();
    // ...
}
```

### Testing Database Operations

```rust
#[tokio::test]
async fn test_user_creation() {
    let pool = setup_test_db().await;
    
    // Test logic
    
    cleanup_test_db(&pool).await;
}
```

---

## Frontend Testing (JavaScript)

### Test Structure

Frontend tests use common validation modules and test suites:

**Common Modules**:
- `validation-helpers.js`: Shared validation functions
- `*-validation-tests.js`: Reusable test suites

**Example**:

```javascript
import { validatePassword } from './validation-helpers.js';
import { runPasswordValidationTests } from './password-validation-tests.js';

// Reuse common test suite
runPasswordValidationTests('User Management Screen', validatePassword);
```

### Running Frontend Tests

Currently, frontend tests are run manually in the browser:

1. Open `res/tests/test-runner.html`
2. Select test suite to run
3. View results in browser console

**Note**: Automated test runner is planned for future implementation.

---

## Test Coverage

### Current Coverage

- **Rust Backend**: 201+ tests
- **Frontend**: Manual testing (test runner TBD)

### Coverage Goals

| Component | Target | Status |
|-----------|--------|--------|
| Core Logic | 90%+ | ✅ Achieved |
| Database | 80%+ | ✅ Achieved |
| Security | 100% | ✅ Achieved |
| Frontend | 70%+ | 🚧 In Progress |

### Measuring Coverage

```bash
# Generate coverage report
cargo tarpaulin --out Html

# View report
xdg-open tarpaulin-report.html
```

---

## Testing Best Practices

### DO ✅

- Write tests before fixing bugs (TDD)
- Test edge cases and boundary conditions
- Use descriptive test names
- Keep tests independent
- Use test helpers for common setup
- Test error conditions

### DON'T ❌

- Use `unwrap()` in production code
- Write tests that depend on execution order
- Hard-code values that might change
- Ignore failing tests
- Skip error handling tests
- Test implementation details

---

## Common Testing Patterns

### Testing Validation

```rust
// Test boundary values
#[test]
fn should_reject_password_below_minimum_length() {
    assert!(validate_password("a".repeat(15)).is_err());
}

#[test]
fn should_accept_password_at_minimum_length() {
    assert!(validate_password("a".repeat(16)).is_ok());
}

// Test Unicode handling
#[test]
fn should_handle_unicode_characters() {
    assert!(validate_password("あ".repeat(16)).is_ok());
}
```

### Testing Error Handling

```rust
#[test]
fn should_return_error_for_duplicate_user() {
    let result = create_user("existing_user");
    
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "User already exists"
    );
}
```

### Testing Async Operations

```rust
#[tokio::test]
async fn should_fetch_user_data() {
    let pool = setup_test_db().await;
    let user = fetch_user(&pool, 1).await.unwrap();
    
    assert_eq!(user.username, "test_user");
}
```

---

## Test Organization

### File Structure

```
tests/
├── integration/
│   ├── auth_tests.rs
│   ├── user_management_tests.rs
│   └── transaction_tests.rs
└── common/
    └── mod.rs          # Shared test utilities

src/
├── validation.rs
└── validation.rs       # Inline unit tests (#[cfg(test)])

res/tests/
├── test-runner.html
├── validation-helpers.js
└── *-validation-tests.js
```

---

## Continuous Integration

### Pre-commit Checks

Run these before committing:

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Run tests
cargo test
```

### CI Pipeline (Future)

Planned GitHub Actions workflow:

1. Run `cargo fmt --check`
2. Run `cargo clippy`
3. Run `cargo test`
4. Generate coverage report
5. Run frontend tests (when automated)

---

## Debugging Tests

### Rust Tests

```bash
# Show println! output
cargo test -- --nocapture

# Run specific test with output
cargo test test_name -- --nocapture --test-threads=1
```

### Frontend Tests

Use browser DevTools:
- Set breakpoints in test code
- Use `console.log()` for debugging
- Check Network tab for API calls

---

## Related Documentation

- [Coding Standards](./CODING_STANDARDS.md)
- [Development Setup](./DEVELOPMENT_SETUP.md)
- [API Reference](../../api/API_COMMON.md)

---

**Note**: This guide will be updated as the testing infrastructure evolves.
