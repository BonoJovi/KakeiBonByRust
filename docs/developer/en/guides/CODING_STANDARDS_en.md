# Coding Standards

**Audience**: Developers & Contributors  
**Last Updated**: 2024-12-05 05:14 JST

---

## Table of Contents
1. [Basic Principles](#basic-principles)
2. [File Modification Guidelines](#file-modification-guidelines)
3. [Rust Conventions](#rust-conventions)
4. [JavaScript Conventions](#javascript-conventions)
5. [Validation Rules](#validation-rules)
6. [SQL Management](#sql-management)
7. [Database Naming Convention](#database-naming-convention)
8. [Testing Conventions](#testing-conventions)
9. [Documentation Conventions](#documentation-conventions)

---

## Basic Principles

### 1. DRY (Don't Repeat Yourself)
Manage shared logic in common modules.

```rust
// ✓ Good - Use common module
use crate::validation::validate_password;

// ✗ Bad - Duplicate logic
fn check_password(pwd: &str) -> bool {
    pwd.len() >= 16  // Same check implemented everywhere
}
```

### 2. Consistency
Use the same validation rules in backend and frontend.

### 3. Type Safety
- Leverage Rust's type system
- Avoid `any` in JavaScript (when TypeScript is introduced)

### 4. Explicit Error Handling
Always handle errors explicitly.

---

## File Modification Guidelines

### ❌ Prohibited: Complete File Rewrites
- Do not rewrite entire files
- **Exception**: Only when necessary due to the nature of the change
- **Reason**: Minimize risk of unintended changes and merge conflicts

### ✅ Recommended: Surgical Edits
- **Modify only necessary parts**
- Change specific lines or blocks only
- Preserve surrounding code and context
- **Benefits**:
  - Clear change history in version control
  - Reduced risk of bug introduction
  - Easier code review
  - Simpler merge conflict resolution

---

## Rust Conventions

### Naming Conventions
- **Functions**: `snake_case` (e.g., `verify_login`)
- **Structs**: `PascalCase` (e.g., `UserInfo`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g., `ROLE_ADMIN`)
- **Modules**: `snake_case` (e.g., `user_management`)

### Error Handling

```rust
// ✓ Good - Explicit Result type
pub async fn get_user(id: i64) -> Result<UserInfo, UserManagementError> {
    // ...
}

// ✗ Bad - unwrap in production code
pub async fn get_user(id: i64) -> UserInfo {
    db.query().await.unwrap()  // Don't do this
}
```

### Tauri Commands

```rust
// Pattern: #[tauri::command] + pub async fn + Result
#[tauri::command]
pub async fn create_user(
    user_data: UserData,
) -> Result<UserInfo, String> {
    // Implementation
}
```

### Module Structure
```
src/
├── main.rs              # Entry point
├── lib.rs               # Tauri commands export
├── consts.rs            # Constants definition
├── validation.rs        # Input validation
├── security.rs          # Password hashing
├── crypto.rs            # Encryption/decryption
├── sql_queries.rs       # SQL definitions
└── services/            # Business logic
    ├── auth.rs
    ├── user_management.rs
    └── ...
```

---

## JavaScript Conventions

### ES Modules
Always include `.js` extension in imports.

```javascript
// ✓ Good
import { validatePassword } from './validation-helpers.js';

// ✗ Bad
import { validatePassword } from './validation-helpers';
```

### Async/Await
Always use try/catch with async functions.

```javascript
// ✓ Good
async function loadData() {
    try {
        const data = await fetchData();
        return data;
    } catch (error) {
        console.error('Error:', error);
        throw error;
    }
}

// ✗ Bad
async function loadData() {
    const data = await fetchData();  // No error handling
    return data;
}
```

### Naming Conventions
- **Functions**: `camelCase` (e.g., `validatePassword`)
- **Classes**: `PascalCase` (e.g., `UserManager`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g., `ROLE_ADMIN`)

---

## Validation Rules

### Password
- **Minimum Length**: 16 characters
- Enforced in both backend (`src/validation.rs`) and frontend

### Unicode Handling
- **JavaScript**: `.length` (UTF-16 code units)
- **Rust**: `.len()` (UTF-8 bytes), character count: `.chars().count()`
- **Testing**: Use ASCII for boundary tests, Unicode for character support tests

### Common Validation
```rust
// src/validation.rs
pub const MIN_PASSWORD_LENGTH: usize = 16;

pub fn validate_password(password: &str) -> ValidationResult {
    if password.chars().count() < MIN_PASSWORD_LENGTH {
        return ValidationResult::Invalid("Password too short".to_string());
    }
    ValidationResult::Valid
}
```

```javascript
// res/js/validation-helpers.js
export const MIN_PASSWORD_LENGTH = 16;

export function validatePassword(password) {
    if (password.length < MIN_PASSWORD_LENGTH) {
        return { valid: false, error: 'Password too short' };
    }
    return { valid: true };
}
```

---

## SQL Management

### SQL Consolidation Rules

#### ❌ Prohibited
- Directly embedding SQL strings in service/command code

#### ✅ Required
- **All SQL queries**: Define as constants in `src/sql_queries.rs`
- **Naming convention**:
  - Production queries: `{SCOPE}_{ACTION}` (e.g., `CATEGORY2_UPDATE`, `USER_INSERT`)
  - Test queries: `TEST_` prefix (e.g., `TEST_CATEGORY_GET_CATEGORY2_NAME`)

#### Benefits
- Single source of truth for SQL
- Easier maintenance and testing
- Consistent parameterization prevents SQL injection
- Improved SQL review and optimization

#### Example

```rust
// src/sql_queries.rs
pub const CATEGORY2_UPDATE: &str = r#"
UPDATE CATEGORY2 
SET CATEGORY2_NAME = ?, UPDATE_DT = datetime('now') 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ?
"#;

// In service code
use crate::sql_queries::CATEGORY2_UPDATE;
conn.execute(CATEGORY2_UPDATE, params![name, user_id, cat1, cat2])?;
```

---

## Database Naming Convention

### ❌ Prohibited: Using "kakeibo.db"
- **Do not use** `kakeibo.db` as database filename
- **Correct filename**: `KakeiBonDB.sqlite3` (defined in `src/consts.rs`)
- **Storage location**: `~/.kakeibon/KakeiBonDB.sqlite3`
- **Access**: Use `./db.sh` script
- **Reason**: Inaccurate database names cause confusion and data inconsistency

### Database Access

```bash
# ✓ Correct - Use db.sh script
./db.sh "SELECT * FROM USERS;"

# ✗ Wrong - Don't specify filename manually
sqlite3 work/kakeibo.db "SELECT * FROM USERS;"
sqlite3 ~/.local/share/kakeibo/kakeibo.db "SELECT * FROM USERS;"
```

---

## Testing Conventions

### Test Pattern
```
"should [expected behavior] when [condition]"
```

### Structure (AAA)
1. **Arrange**: Prepare test data
2. **Act**: Execute test target
3. **Assert**: Verify results

### Reuse Common Modules
- Frontend: Import test suites from `*-validation-tests.js`
- Backend: Use `src/test_helpers.rs` and `src/validation_tests.rs`

### Test Count
- Rust backend: 201 tests
- JavaScript frontend: Manual testing (runner not configured)
- **Total**: 201+ tests

---

## Documentation Conventions

### Timestamps
- **User-facing documentation**: Always use Japan Standard Time (JST, UTC+9)
- **AI context documentation**: UTC or JST acceptable
- **Example**: `Last Updated: 2024-10-26 13:21 JST`

### Language
- **User-facing documentation**: Both Japanese and English required
- **AI/LLM documentation**: English recommended

### Structure
```
# Title

**Audience**: [target readers]
**Last Updated**: [datetime JST]

## Table of Contents
[...]

## Section
[...]
```

---

## Important Constants

| Constant | Value | Location |
|----------|-------|----------|
| ROLE_ADMIN | 0 | `src/consts.rs`, `res/js/consts.js` |
| ROLE_USER | 1 | `src/consts.rs`, `res/js/consts.js` |
| MIN_PASSWORD_LENGTH | 16 | `src/validation.rs` |
| DATABASE_FILENAME | KakeiBonDB.sqlite3 | `src/consts.rs` |

---

## Anti-Patterns

### ❌ Don't
- Duplicate validation logic across files
- Use `unwrap()` in production Rust code
- Ignore error handling
- Mix concerns (e.g., UI logic in validation functions)
- Hard-code strings (use i18n)
- Commit passwords or secrets

### ✅ Do
- Use common modules for shared logic
- Handle all errors explicitly
- Separate concerns (UI, logic, data)
- Use i18n for all user-facing text
- Perform surgical edits (don't rewrite entire files)
- Consolidate SQL queries in `sql_queries.rs`

---

**Related Documentation**:
- [Development Environment Setup](./DEVELOPMENT_SETUP_en.md)
- [Testing Guide](./testing-guide-en.md)
- [API Reference](../../reference/en/api/)
