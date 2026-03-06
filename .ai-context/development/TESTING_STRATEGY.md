# KakeiBon - Testing Strategy

**Last Updated**: 2025-12-11 03:48 JST  
**Purpose**: Comprehensive testing approach for KakeiBon  
**Keywords**: testing strategy, テスト戦略, test coverage, テストカバレッジ, unit tests, ユニットテスト, integration tests, 統合テスト, TDD, test-driven development, テスト駆動開発, cargo test, Rust tests, Rustテスト, frontend tests, フロントエンドテスト, backend tests, バックエンドテスト, test helpers, テストヘルパー, test organization, テスト構成, AAA pattern, Arrange-Act-Assert, mock data, モックデータ, test database, テストデータベース, validation tests, バリデーションテスト, quality assurance, QA, 品質保証  
**Related**: @development/CONVENTIONS.md, @core/DESIGN_PHILOSOPHY.md, @architecture/PROJECT_STRUCTURE.md, @development/METHODOLOGY.md

---

## Test Coverage Overview

### Current Status
- **Backend Tests**: 201 tests (comprehensive coverage)
- **Frontend Tests**: Manual testing (automated framework pending)
- **Integration Tests**: Basic coverage in `tests/` directory

---

## Backend Testing (Rust)

### Test Organization

#### Inline Tests
```rust
// src/validation.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_password_validation() {
        // Test implementation
    }
}
```

#### Test Helpers
- **Location**: `src/test_helpers.rs`
- **Purpose**: Common setup, mock data, assertions
- **Usage**: Imported by all test modules

```rust
// src/test_helpers.rs
pub fn create_test_db() -> SqlitePool { /* ... */ }
pub fn create_test_admin() -> UserInfo { /* ... */ }
```

### Test Categories

#### 1. Unit Tests
**Scope**: Individual functions, pure logic

```rust
#[test]
fn test_validate_password_length() {
    assert!(validate_password("a".repeat(16)).is_ok());
    assert!(validate_password("a".repeat(15)).is_err());
}
```

#### 2. Integration Tests
**Scope**: Multiple modules working together

```rust
#[tokio::test]
async fn test_user_creation_flow() {
    let pool = create_test_db().await;
    
    // Create admin
    let admin = create_admin(&pool, "admin", "password").await.unwrap();
    
    // Admin creates user
    let user = create_user(&pool, admin.id, "user", "password").await.unwrap();
    
    // Verify user can login
    let result = verify_login(&pool, "user", "password").await.unwrap();
    assert_eq!(result.username, "user");
}
```

#### 3. Validation Tests
**Scope**: Input validation rules

- **Location**: `src/validation_tests.rs`
- **Common Suites**: Reusable test functions
- **Coverage**: All validation rules (username, password, etc.)

```rust
#[test]
fn test_password_minimum_length() {
    let valid = "a".repeat(MIN_PASSWORD_LENGTH);
    let invalid = "a".repeat(MIN_PASSWORD_LENGTH - 1);
    
    assert!(validate_password(&valid).is_ok());
    assert!(validate_password(&invalid).is_err());
}
```

#### 4. Database Tests
**Scope**: SQL operations, data integrity

```rust
#[tokio::test]
async fn test_duplicate_username_rejected() {
    let pool = create_test_db().await;
    
    create_user(&pool, "user1", "pass").await.unwrap();
    let result = create_user(&pool, "user1", "pass").await;
    
    assert!(result.is_err());
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test validation

# With output
cargo test -- --nocapture

# Single test
cargo test test_password_validation
```

---

## Frontend Testing (JavaScript)

### Current Approach: Manual Testing

**Checklist** (per screen):
- [ ] Form validation (empty fields, invalid input)
- [ ] Language switching (ja ↔ en)
- [ ] Error message display
- [ ] Success message display
- [ ] Modal open/close (ESC, backdrop click)
- [ ] Focus trap in modals
- [ ] CRUD operations
- [ ] Data persistence after refresh

### Future: Automated Testing

**Planned Framework**: Jest + JSDOM

#### Test Structure
```javascript
// res/tests/validation-helpers.test.js
import { validatePassword } from '../js/validation-helpers.js';

describe('Password Validation', () => {
    test('should accept 16+ character password', () => {
        expect(validatePassword('a'.repeat(16))).toBe(true);
    });
    
    test('should reject < 16 character password', () => {
        expect(validatePassword('a'.repeat(15))).toBe(false);
    });
});
```

#### Common Test Modules
- `res/tests/validation-helpers.js` - Shared validation functions
- `res/tests/*-validation-tests.js` - Reusable test suites
- `res/tests/test-common.js` - Test utilities

---

## Test Patterns

### AAA Pattern (Arrange-Act-Assert)

```rust
#[test]
fn test_user_role_assignment() {
    // Arrange
    let user = UserInfo {
        id: 1,
        username: "test".to_string(),
        role: ROLE_USER,
    };
    
    // Act
    let is_admin = user.role == ROLE_ADMIN;
    
    // Assert
    assert_eq!(is_admin, false);
}
```

### Test Naming Convention

**Format**: `test_<function>_<scenario>_<expected_result>`

```rust
#[test]
fn test_validate_password_too_short_returns_error() { /* ... */ }

#[test]
fn test_create_user_duplicate_username_returns_error() { /* ... */ }

#[test]
fn test_hash_password_different_inputs_different_hashes() { /* ... */ }
```

### Test Data

#### ASCII for Boundary Tests
```rust
let min_length = "a".repeat(MIN_PASSWORD_LENGTH);
let too_short = "a".repeat(MIN_PASSWORD_LENGTH - 1);
```

**Rationale**: JavaScript counts UTF-16 code units, Rust counts bytes. ASCII avoids encoding mismatches.

#### Unicode for Character Support Tests
```rust
let japanese = "日本語パスワード16文字以上";
let emoji = "[Smile][Party][Lock][Key][PC][Rocket]⭐✨[Star][Idea][Target][Trophy][Gift][Confetti][Balloon]";
```

**Rationale**: Verify system handles multi-byte characters correctly.

---

## Security Testing

### Password Security
- [ ] Passwords never logged
- [ ] Passwords never stored plaintext
- [ ] Hashing is salted
- [ ] Hashing is memory-hard (Argon2)

### SQL Injection Prevention
- [ ] All queries use prepared statements
- [ ] No string concatenation for SQL
- [ ] Input is parameterized

### Authorization Testing
- [ ] Non-admin cannot access admin features
- [ ] Users cannot access other users' data
- [ ] Session validation on every protected route

---

## Performance Testing

### Database Operations
```rust
#[tokio::test]
async fn test_bulk_transaction_insert() {
    let pool = create_test_db().await;
    
    let start = Instant::now();
    for i in 0..1000 {
        insert_transaction(&pool, /* ... */).await.unwrap();
    }
    let duration = start.elapsed();
    
    assert!(duration < Duration::from_secs(5));
}
```

### Validation Performance
- Keep validation lightweight (< 1ms per field)
- Avoid regex where simple checks suffice

---

## Test Maintenance

### When to Update Tests

1. **New Feature**: Write tests first (TDD)
2. **Bug Fix**: Add regression test before fixing
3. **Refactoring**: Tests should still pass
4. **API Change**: Update tests to match new signature

### Test Coverage Goals

| Module | Target Coverage | Current Status |
|--------|-----------------|----------------|
| Validation | 100% | ✅ Achieved |
| Authentication | >90% | ✅ Achieved |
| User Management | >90% | ✅ Achieved |
| Database | >80% | ✅ Achieved |
| Frontend | >80% | ⏳ Pending |

---

## Continuous Integration

### GitHub Actions (Future)

```yaml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run Rust tests
        run: cargo test --all-features
      - name: Run Frontend tests
        run: npm test
```

---

## Test Documentation

### Writing Test Documentation

```rust
/// Tests that admin user can create regular users.
/// 
/// # Test Case
/// 1. Create admin user
/// 2. Admin creates regular user
/// 3. Verify new user exists in database
/// 4. Verify new user has ROLE_USER role
#[tokio::test]
async fn test_admin_can_create_user() {
    // Implementation
}
```

---

## Anti-Patterns to Avoid

### ❌ Don't
1. **Test Implementation Details**: Test behavior, not internal structure
2. **Interdependent Tests**: Each test should be independent
3. **Flaky Tests**: Tests should be deterministic
4. **Slow Tests**: Keep unit tests fast (< 100ms)
5. **Skipped Tests**: Fix or remove, don't accumulate

### ✅ Do
1. **Test Behavior**: What should happen, not how
2. **Isolate Tests**: No shared state between tests
3. **Use Test Helpers**: DRY principle applies to tests
4. **Meaningful Names**: Test name describes scenario
5. **One Assertion Per Concept**: Multiple asserts OK if testing same concept

---

**Testing is not overhead - it's the safety net that enables confident refactoring and rapid development.**
