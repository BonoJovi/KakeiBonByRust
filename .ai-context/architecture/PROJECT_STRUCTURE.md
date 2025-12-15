# AI Context: KakeiBon Project Structure

**Last Updated**: 2025-12-11 03:48 JST  
**Purpose**: This document helps AI assistants quickly understand the project structure without reading every file  
**Keywords**: project structure, プロジェクト構造, directory structure, ディレクトリ構造, module organization, モジュール構成, file organization, ファイル構成, Rust modules, Rustモジュール, src directory, srcディレクトリ, res directory, resディレクトリ, frontend, フロントエンド, backend, バックエンド, Tauri, services, サービス, components, コンポーネント, locales, ロケール, i18n, tests, テスト, architecture, アーキテクチャ, codebase, コードベース  
**Related**: @architecture/TAURI_DEVELOPMENT.md, @core/QUICK_REFERENCE.md, @development/CONVENTIONS.md, @development/TESTING_STRATEGY.md

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
├── src/                      # Rust backend source (24 files)
│   ├── main.rs               # Entry point
│   ├── lib.rs                # Tauri commands export
│   ├── db.rs                 # Database operations
│   ├── validation.rs         # Input validation
│   ├── security.rs           # Password hashing
│   ├── crypto.rs             # Encryption/decryption
│   ├── settings.rs           # Application settings
│   ├── consts.rs             # Constants (ROLE_ADMIN=0, ROLE_USER=1)
│   ├── sql_queries.rs        # ALL SQL queries centralized
│   ├── test_helpers.rs       # Common test utilities (test only)
│   ├── validation_tests.rs   # Reusable validation test suites (test only)
│   └── services/             # Business logic modules (12 services)
│       ├── auth.rs           # Authentication
│       ├── user_management.rs # User CRUD
│       ├── category.rs       # Category management (3-level hierarchy)
│       ├── account.rs        # Account management (income/expense tracking)
│       ├── transaction.rs    # Transaction recording & editing
│       ├── manufacturer.rs   # Manufacturer master data
│       ├── product.rs        # Product master data
│       ├── shop.rs           # Shop master data
│       ├── aggregation.rs    # Data aggregation (daily/weekly/monthly/period/yearly)
│       ├── encryption.rs     # Encryption service
│       ├── session.rs        # Session management
│       └── i18n.rs           # Internationalization
│
├── res/                      # Frontend resources
│   ├── index.html            # Login/Admin setup screen
│   ├── user-management.html  # User management screen
│   ├── category-management.html # Category management (3-level)
│   ├── account-management.html # Account management
│   ├── transaction-management.html # Transaction list & search
│   ├── transaction-detail-management.html # Transaction detail editing
│   ├── manufacturer-management.html # Manufacturer master
│   ├── product-management.html # Product master
│   ├── shop-management.html  # Shop master
│   ├── aggregation.html      # Monthly aggregation (default)
│   ├── aggregation-daily.html # Daily aggregation
│   ├── aggregation-weekly.html # Weekly aggregation
│   ├── aggregation-period.html # Period aggregation
│   ├── aggregation-yearly.html # Yearly aggregation
│   ├── js/                   # JavaScript modules (23 files)
│   │   ├── menu.js           # Main menu & admin setup logic
│   │   ├── user-management.js # User CRUD logic
│   │   ├── category-management.js # Category CRUD
│   │   ├── account-management.js # Account CRUD
│   │   ├── transaction-management.js # Transaction list & search
│   │   ├── transaction-detail-management.js # Transaction editing
│   │   ├── manufacturer-management.js # Manufacturer CRUD
│   │   ├── product-management.js # Product CRUD
│   │   ├── shop-management.js # Shop CRUD
│   │   ├── aggregation.js    # Monthly aggregation
│   │   ├── aggregation-daily.js # Daily aggregation
│   │   ├── aggregation-weekly.js # Weekly aggregation
│   │   ├── aggregation-period.js # Period aggregation
│   │   ├── aggregation-yearly.js # Yearly aggregation
│   │   ├── aggregation-common.js # Common aggregation logic
│   │   ├── modal.js          # Common modal class
│   │   ├── modal-utils.js    # Modal helper utilities
│   │   ├── font-size.js      # Font size adjustment
│   │   ├── session.js        # Session management
│   │   ├── i18n.js           # i18n client
│   │   ├── indicators.js     # Caps Lock indicator
│   │   ├── html-files.js     # HTML file paths
│   │   └── consts.js         # JS constants
│   ├── css/                  # Stylesheets
│   └── locales/              # Translation files (ja, en)
│
├── res/tests/                # Test suites (29 test files, 488 tests)
│   ├── validation-helpers.js          # Common validation logic
│   ├── password-validation-tests.js   # Password test suite
│   ├── username-validation-tests.js   # Username test suite
│   ├── admin-setup.test.js            # Admin setup tests
│   ├── user-addition.test.js          # User addition tests
│   ├── login.test.js                  # Login tests
│   ├── category-management.test.js    # Category CRUD tests
│   ├── account-management.test.js     # Account CRUD tests
│   ├── transaction-edit.test.js       # Transaction editing tests
│   ├── manufacturer-management.test.js # Manufacturer tests
│   ├── product-management.test.js     # Product tests
│   ├── shop-management.test.js        # Shop tests
│   └── [additional test files]        # See test output for full list
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
| SQL Queries | `src/sql_queries.rs` | ALL SQL queries centralized |
| Validation | `src/validation.rs` | Input validation (password length, etc.) |
| Security | `src/security.rs` | Argon2 password hashing |
| Crypto | `src/crypto.rs` | AES-256-GCM encryption/decryption |
| Auth | `src/services/auth.rs` | Login, authentication |
| User Mgmt | `src/services/user_management.rs` | User CRUD operations |
| Category | `src/services/category.rs` | 3-level category management |
| Account | `src/services/account.rs` | Account (income/expense) management |
| Transaction | `src/services/transaction.rs` | Transaction recording & editing |
| Manufacturer | `src/services/manufacturer.rs` | Manufacturer master data |
| Product | `src/services/product.rs` | Product master data |
| Shop | `src/services/shop.rs` | Shop master data |
| Aggregation | `src/services/aggregation.rs` | Data aggregation (5 types) |
| Encryption | `src/services/encryption.rs` | Encryption service layer |
| Session | `src/services/session.rs` | Session management |
| i18n | `src/services/i18n.rs` | Backend i18n support |
| **Test Helpers** | `src/test_helpers.rs` | **Common test utilities and database setup** |
| **Validation Tests** | `src/validation_tests.rs` | **Reusable password validation test suites** |

### Frontend (JavaScript)

| Module | File | Purpose |
|--------|------|---------|
| Main Menu | `res/js/menu.js` | Main menu & admin user registration |
| User Management | `res/js/user-management.js` | User CRUD UI |
| Category Management | `res/js/category-management.js` | Category CRUD UI |
| Account Management | `res/js/account-management.js` | Account CRUD UI |
| Transaction List | `res/js/transaction-management.js` | Transaction list & search |
| Transaction Edit | `res/js/transaction-detail-management.js` | Transaction editing |
| Manufacturer | `res/js/manufacturer-management.js` | Manufacturer CRUD UI |
| Product | `res/js/product-management.js` | Product CRUD UI |
| Shop | `res/js/shop-management.js` | Shop CRUD UI |
| Aggregation | `res/js/aggregation*.js` | 5 aggregation screens |
| Modal | `res/js/modal.js` | Common modal class |
| Font Size | `res/js/font-size.js` | Font size adjustment |
| Session | `res/js/session.js` | Session management |
| i18n Client | `res/js/i18n.js` | Frontend translation |
| Indicators | `res/js/indicators.js` | Low Vision Support Indicator |
| Constants | `res/js/consts.js` | ROLE_ADMIN, ROLE_USER |

### Test Modules (Jest + ES Modules)

| Module | File | Purpose |
|--------|------|---------|
| Validation Helpers | `res/tests/validation-helpers.js` | Common validation functions |
| Password Tests | `res/tests/password-validation-tests.js` | Reusable password test suite (26 tests) |
| Username Tests | `res/tests/username-validation-tests.js` | Reusable username test suite (13 tests) |
| Admin Setup Tests | `res/tests/admin-setup.test.js` | Admin registration screen tests |
| User Tests | `res/tests/user-*.test.js` | User addition, deletion, update tests |
| Login Tests | `res/tests/login.test.js` | Login functionality tests |
| Category Tests | `res/tests/category-management.test.js` | Category CRUD tests |
| Account Tests | `res/tests/account-management.test.js` | Account CRUD tests |
| Transaction Tests | `res/tests/transaction-*.test.js` | Transaction editing tests |
| Master Data Tests | `res/tests/*-management.test.js` | Manufacturer, Product, Shop tests |

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

### 4. Transaction Recording
```
transaction-management.html → transaction-management.js → [Tauri]
  ├── list_transactions() → transaction.rs → db.rs
  ├── search_transactions() → transaction.rs → db.rs
  └── get_transaction_detail() → transaction.rs → db.rs

transaction-detail-management.html → transaction-detail-management.js → [Tauri]
  ├── create_transaction() → transaction.rs → db.rs
  └── update_transaction() → transaction.rs → db.rs
```

### 5. Category/Account/Master Data Management
```
category-management.html → category-management.js → [Tauri]
  ├── get_category_tree() → category.rs → db.rs
  ├── add_category2/3() → category.rs → db.rs
  ├── update_category2/3() → category.rs → db.rs
  └── delete_category2/3() → category.rs → db.rs
  
  Note: CATEGORY1 is fixed (EXPENSE/INCOME/TRANSFER only)
        - No add/edit/delete for CATEGORY1
        - User can only manage CATEGORY2/3 under selected CATEGORY1

account-management.html → account-management.js → [Tauri]
  └── Similar pattern for accounts

manufacturer/product/shop-management.html → [Similar patterns]
```

### 6. Aggregation (5 Types)
```
aggregation*.html → aggregation*.js → [Tauri]
  ├── aggregate_monthly() → aggregation.rs → db.rs
  ├── aggregate_daily() → aggregation.rs → db.rs
  ├── aggregate_weekly() → aggregation.rs → db.rs
  ├── aggregate_period() → aggregation.rs → db.rs
  └── aggregate_yearly() → aggregation.rs → db.rs
```

---

## Database Schema (SQLite)

### Core Tables

**USERS**: User authentication and authorization
```sql
CREATE TABLE USERS (
    USER_ID INTEGER PRIMARY KEY,
    NAME VARCHAR(128) UNIQUE NOT NULL,
    PAW VARCHAR(128) NOT NULL,        -- Argon2 hash
    ROLE INTEGER NOT NULL,             -- 0=ADMIN, 1=USER
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME
);
```

**CATEGORY1/CATEGORY2/CATEGORY3**: 3-level category hierarchy
```sql
CREATE TABLE CATEGORY1 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY1_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE)
);
-- CATEGORY1 is fixed: EXPENSE (出金), INCOME (入金), TRANSFER (振替)
-- Users cannot add/edit/delete CATEGORY1 records

CREATE TABLE CATEGORY2 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY2_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE) 
        REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE) ON DELETE CASCADE
);
-- Users can add/edit/delete CATEGORY2 under selected CATEGORY1

CREATE TABLE CATEGORY3 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY3_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) 
        REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) ON DELETE CASCADE
);
-- Users can add/edit/delete CATEGORY3 under selected CATEGORY2
```

**ACCOUNT**: Income/expense accounts
```sql
CREATE TABLE ACCOUNT (
    USER_ID INTEGER,
    ACCOUNT_CODE TEXT,
    ACCOUNT_NAME TEXT,
    CATEGORY1_CODE TEXT,
    PRIMARY KEY (USER_ID, ACCOUNT_CODE)
);
```

**TRANSACTIONS**: Financial transactions
```sql
CREATE TABLE TRANSACTIONS (
    USER_ID INTEGER,
    TRANSACTION_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    TRANSACTION_DT TEXT,              -- YYYY-MM-DD HH:MM:SS
    ACCOUNT_CODE TEXT,
    AMOUNT INTEGER,
    TAX INTEGER,
    TAX_RATE_DIV INTEGER,
    TAX_ROUNDING_DIV INTEGER,
    MEMO_ID INTEGER,
    -- Plus many other fields for details
);
```

**MEMO**: Shared memo data
```sql
CREATE TABLE MEMO (
    MEMO_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    MEMO TEXT
);
```

**Master Data Tables**:
- **MANUFACTURER**: Manufacturer master
- **PRODUCT**: Product master (linked to manufacturer)
- **SHOP**: Shop master

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

# JavaScript tests (488 tests)
cd res/tests
npm test

# All tests
./res/tests/run-all-tests.sh
```

**Test Count:**
- Rust backend: 121 tests (when passing)
- JavaScript frontend: 488 tests
- **Total: 600+ tests**
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

- [ ] Session timeout mechanism (basic session management implemented)
- [ ] Data export/import functionality
- [ ] Report generation features
- [ ] Mobile responsive design

---

## Related Documentation

- **User Documentation**: See `res/tests/README_NEW.md`
- **Test Design**: See `res/tests/TEST_DESIGN.md`
- **Test Cases**: See `res/tests/TEST_CASES.md`
- **Quick Start**: See `res/tests/QUICK_START.md`
