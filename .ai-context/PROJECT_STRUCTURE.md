# AI Context: KakeiBon Project Structure

**Purpose**: This document helps AI assistants quickly understand the project structure without reading every file.
**Last Updated**: 2024-10-26

---

## Project Overview

**Name**: KakeiBon  
**Type**: Desktop application (Tauri + Rust + HTML/JS)  
**Purpose**: Personal finance management (家計簿 - Kakeibo)

---

## Tech Stack

- **Backend**: Rust (Tauri framework)
- **Frontend**: Vanilla HTML/CSS/JavaScript (no framework)
- **Database**: SQLite (via sqlx)
- **Security**: Argon2 password hashing, AES-256-GCM encryption

---

## Directory Structure

```
KakeiBonByRust/
├── src/                      # Rust backend source
│   ├── main.rs               # Entry point
│   ├── lib.rs                # Tauri commands export
│   ├── db.rs                 # Database operations
│   ├── validation.rs         # Input validation
│   ├── security.rs           # Password hashing
│   ├── crypto.rs             # Encryption/decryption
│   ├── settings.rs           # Application settings
│   ├── consts.rs             # Constants (ROLE_ADMIN=0, ROLE_USER=1)
│   ├── test_helpers.rs       # Common test utilities (test only)
│   ├── validation_tests.rs   # Reusable validation test suites (test only)
│   └── services/             # Business logic modules
│       ├── auth.rs           # Authentication
│       ├── user_management.rs # User CRUD
│       ├── category.rs       # Category management
│       ├── encryption.rs     # Encryption service
│       └── i18n.rs           # Internationalization
│
├── res/                      # Frontend resources
│   ├── index.html            # Login/Admin setup screen
│   ├── user-management.html  # User management screen
│   ├── js/                   # JavaScript modules
│   │   ├── menu.js           # Admin setup logic
│   │   ├── user-management.js # User CRUD logic
│   │   ├── i18n.js           # i18n client
│   │   ├── indicators.js     # Caps Lock indicator
│   │   └── consts.js         # JS constants
│   ├── css/                  # Stylesheets
│   └── locales/              # Translation files (ja, en)
│
├── res/tests/                # Test suites
│   ├── validation-helpers.js          # Common validation logic
│   ├── password-validation-tests.js   # Password test suite
│   ├── username-validation-tests.js   # Username test suite
│   ├── admin-setup.test.js            # Admin setup tests (29 tests)
│   ├── user-addition.test.js          # User addition tests (49 tests)
│   ├── login.test.js                  # Login tests (58 tests)
│   └── [docs]                         # See TEST_INDEX.md
│
├── .ai-context/              # AI assistant context (THIS DIRECTORY)
│   ├── PROJECT_STRUCTURE.md  # This file
│   ├── KEY_FILES.md          # Important files quick reference
│   └── CONVENTIONS.md        # Coding conventions
│
├── Cargo.toml                # Rust dependencies
├── tauri.conf.json           # Tauri configuration
└── package.json              # (if exists) Node.js dependencies
```

---

## Key Modules & Their Responsibilities

### Backend (Rust)

| Module | File | Purpose |
|--------|------|---------|
| Main | `src/main.rs` | Application entry point |
| Library | `src/lib.rs` | Export all Tauri commands |
| Database | `src/db.rs` | SQLite connection, migrations, queries |
| Validation | `src/validation.rs` | Input validation (password length, etc.) |
| Security | `src/security.rs` | Argon2 password hashing |
| Crypto | `src/crypto.rs` | AES-256-GCM encryption/decryption |
| Auth | `src/services/auth.rs` | Login, authentication |
| User Mgmt | `src/services/user_management.rs` | User CRUD operations |
| Encryption | `src/services/encryption.rs` | Encryption service layer |
| i18n | `src/services/i18n.rs` | Backend i18n support |
| **Test Helpers** | `src/test_helpers.rs` | **Common test utilities and database setup** |
| **Validation Tests** | `src/validation_tests.rs` | **Reusable password validation test suites** |

### Frontend (JavaScript)

| Module | File | Purpose |
|--------|------|---------|
| Admin Setup | `res/js/menu.js` | Admin user registration |
| User Management | `res/js/user-management.js` | User CRUD UI |
| i18n Client | `res/js/i18n.js` | Frontend translation |
| Indicators | `res/js/indicators.js` | Caps Lock indicator |
| Constants | `res/js/consts.js` | ROLE_ADMIN, ROLE_USER |

### Test Modules (Jest + ES Modules)

| Module | File | Purpose |
|--------|------|---------|
| Validation Helpers | `res/tests/validation-helpers.js` | Common validation functions |
| Password Tests | `res/tests/password-validation-tests.js` | Reusable password test suite (26 tests) |
| Username Tests | `res/tests/username-validation-tests.js` | Reusable username test suite (13 tests) |
| Admin Setup Tests | `res/tests/admin-setup.test.js` | Admin registration screen tests |
| User Addition Tests | `res/tests/user-addition.test.js` | User addition screen tests |

---

## Data Flow

### 1. Admin Setup (First Launch)
```
index.html → menu.js → [Tauri] → register_admin_user() → db.rs
                                → Creates admin in USERS table
```

### 2. Login
```
index.html → menu.js → [Tauri] → verify_login() → auth.rs → db.rs
                                → Returns user info + JWT (future)
```

### 3. User Management
```
user-management.html → user-management.js → [Tauri]
  ├── list_users() → user_management.rs → db.rs
  ├── create_general_user() → user_management.rs → db.rs
  ├── update_general_user_info() → user_management.rs → db.rs
  └── delete_general_user_info() → user_management.rs → db.rs
```

---

## Database Schema (SQLite)

### USERS Table
```sql
CREATE TABLE USERS (
    USER_ID INTEGER PRIMARY KEY,
    NAME VARCHAR(128) UNIQUE NOT NULL,
    PAW VARCHAR(128) NOT NULL,        -- Argon2 hash
    ROLE INTEGER NOT NULL,             -- 1=ADMIN, 2=USER
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME
);
```

### Other Tables (Future)
- CATEGORIES: Expense categories
- TRANSACTIONS: Financial transactions
- ENCRYPTED_DATA: User-encrypted sensitive data

---

## Validation Rules

### Password (enforced in both frontend and backend)
- **Minimum length**: 16 characters
- **Empty check**: `trim()` must not be empty
- **Confirmation**: Must match confirmation field
- **Storage**: Hashed with Argon2id

### Username
- **Empty check**: `trim()` must not be empty
- **Uniqueness**: Checked in database (UNIQUE constraint)
- **Length**: No hard limit (database accepts up to 128 chars)

---

## i18n Support

### Languages
- Japanese (`ja`) - Default
- English (`en`)

### Files
- Backend: `res/locales/{lang}.json`
- Frontend: Loaded via `i18n.js`
- Keys: Dot notation (e.g., `user_mgmt.add_user`)

---

## Test Architecture

### Common Module Pattern
Tests use a DRY (Don't Repeat Yourself) pattern:

**Frontend (JavaScript):**
1. **Common validation functions** in `validation-helpers.js`
2. **Common test suites** in `*-validation-tests.js`
3. **Screen-specific tests** import and reuse common modules

**Backend (Rust):**
1. **Common test helpers** in `src/test_helpers.rs`
2. **Common validation test suites** in `src/validation_tests.rs`
3. **Module-specific tests** use common helpers and test suites

**Example (JavaScript)**:
```javascript
// user-addition.test.js
import { validateUserAddition } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';
import { testUsernameValidation } from './username-validation-tests.js';

// Reuse 26 password tests + 13 username tests
runAllPasswordTests(wrapperFn, 'User Addition Password Tests');
testUsernameValidation(validateUserAddition);
```

**Example (Rust)**:
```rust
// src/services/user_management.rs
use crate::test_helpers::database::{setup_test_db, create_test_admin};

#[tokio::test]
async fn test_register_general_user() {
    let pool = setup_test_db().await;
    create_test_admin(&pool, "admin", "password").await;
    // ... test implementation
}
```

**Benefits**:
- Validation rule change → modify 1 file → all screens updated
- New screen → 5 lines of code → full test coverage
- Consistent behavior across all screens
- Backend and frontend tests follow same pattern

---

## Build & Run

### Development
```bash
cargo tauri dev
```

### Tests
```bash
# Rust tests
cargo test

# JavaScript tests
cd res/tests
npm test

# All tests
./res/tests/run-all-tests.sh
```

**Test Count:**
- Rust backend: 94 tests
- JavaScript frontend: 199 tests
- **Total: 293 tests**
```

### Build
```bash
cargo tauri build
```

---

## Important Constants

| Constant | Value | Location |
|----------|-------|----------|
| ROLE_ADMIN | 0 | `src/consts.rs`, `res/js/consts.js` |
| ROLE_USER | 1 | `src/consts.rs`, `res/js/consts.js` |
| ROLE_VISIT | 999 | `src/consts.rs`, `res/js/consts.js` |
| MIN_PASSWORD_LENGTH | 16 | `src/validation.rs` |
| DATABASE_FILE | `KakeiBonDB.sqlite3` | `src/db.rs` |

---

## Common Tasks (AI Assistant Quick Reference)

### Adding a new screen
1. Create HTML file in `res/`
2. Create JS file in `res/js/`
3. Add Tauri commands in `src/` (if needed)
4. Create test file in `res/tests/` using common modules
5. Update test documentation

### Modifying validation rules
1. Update `res/tests/validation-helpers.js`
2. Update `res/tests/*-validation-tests.js` (if needed)
3. Update backend in `src/validation.rs`
4. Run `npm test` to verify all screens

### Adding a new language
1. Create `res/locales/{lang}.json`
2. Add to `src/services/i18n.rs`
3. Test with `invoke('set_language', { lang })`

---

## Known Issues / TODOs

- [ ] JWT/Session management not yet implemented
- [ ] Encryption key derivation uses placeholder
- [ ] Category management UI not yet created
- [ ] Transaction recording UI not yet created

---

## Related Documentation

- **User Documentation**: See `res/tests/README_NEW.md`
- **Test Design**: See `res/tests/TEST_DESIGN.md`
- **Test Cases**: See `res/tests/TEST_CASES.md`
- **Quick Start**: See `res/tests/QUICK_START.md`
