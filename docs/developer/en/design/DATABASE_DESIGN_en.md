# Database Design

**Last Updated**: 2025-12-05 04:04 JST  
**Target Version**: v0.1.0

---

## Overview

KakeiBon uses SQLite database to manage household accounting data. This document describes the database schema design, table structures, and indexing strategies.

---

## Database Configuration

### Basic Information
- **DBMS**: SQLite 3
- **File Location**: `$HOME/.kakeibo/kakeibo.db`
- **Character Encoding**: UTF-8
- **Foreign Key Constraints**: Must be enabled (`PRAGMA foreign_keys = ON`)

### Access Methods
```bash
# Access production DB
./db.sh

# Check schema
./db.sh .schema

# List tables
./db.sh .tables
```

---

## Table Overview

### Core Tables
| Table Name | Purpose | Notes |
|-----------|---------|-------|
| USERS | User information | Password hash, encrypted master key |
| ACCOUNTS | Account information | Per-user account management |
| TRANSACTIONS_HEADER | Transaction header | Basic transaction info |
| TRANSACTIONS_DETAIL | Transaction details | Item-level transaction details |

### Master Data Tables
| Table Name | Purpose | Hierarchy |
|-----------|---------|-----------|
| CATEGORY1 | Categories (Major) | 1st tier |
| CATEGORY2 | Categories (Medium) | 2nd tier |
| CATEGORY3 | Categories (Minor) | 3rd tier |
| SHOPS | Shop master | - |
| MANUFACTURERS | Manufacturer master | - |
| PRODUCTS | Product master | Links to manufacturers |
| ACCOUNT_TEMPLATES | Account templates | System-wide |

### Internationalization Tables
| Table Name | Purpose |
|-----------|---------|
| I18N_RESOURCES | System resource translations |
| CATEGORY1_I18N | Category 1 translations |
| CATEGORY2_I18N | Category 2 translations |
| CATEGORY3_I18N | Category 3 translations |

### Other Tables
| Table Name | Purpose |
|-----------|---------|
| MEMOS | Memo data (deduplication) |
| DATA_FIELDS | Data field definitions |

---

## Primary Table Designs

### USERS Table

**Purpose**: User authentication and encryption information management

| Column Name | Type | Constraints | Description |
|------------|------|------------|-------------|
| USER_ID | INTEGER | PK, AUTOINCREMENT | User ID |
| USERNAME | VARCHAR(255) | NOT NULL, UNIQUE | Username |
| PASSWORD_HASH | TEXT | NOT NULL | Argon2 hash value |
| ROLE | INTEGER | NOT NULL | Role (0:admin, 1:user) |
| MASTER_KEY_ENCRYPTED | BLOB | NOT NULL | Encrypted master key (AES-256-GCM) |
| MASTER_KEY_SALT | BLOB | NOT NULL | Master key salt |
| MASTER_KEY_NONCE | BLOB | NOT NULL | Master key nonce |
| IS_DISABLED | INTEGER | DEFAULT 0 | Disabled flag |
| ENTRY_DT | DATETIME | NOT NULL | Entry datetime |
| UPDATE_DT | DATETIME | | Update datetime |

**Indexes**:
- `idx_users_username`: `USERNAME` (login optimization)

---

### ACCOUNTS Table

**Purpose**: Per-user account management

| Column Name | Type | Constraints | Description |
|------------|------|------------|-------------|
| ACCOUNT_ID | INTEGER | PK, AUTOINCREMENT | Account ID |
| USER_ID | INTEGER | NOT NULL, FK → USERS | User ID |
| ACCOUNT_CODE | VARCHAR(50) | NOT NULL | Account code |
| ACCOUNT_NAME | TEXT | NOT NULL | Account name |
| TEMPLATE_CODE | VARCHAR(50) | NOT NULL, FK → ACCOUNT_TEMPLATES | Template code |
| INITIAL_BALANCE | INTEGER | DEFAULT 0 | Initial balance |
| DISPLAY_ORDER | INTEGER | | Display order |
| IS_DISABLED | INTEGER | DEFAULT 0 | Disabled flag |
| ENTRY_DT | DATETIME | NOT NULL | Entry datetime |
| UPDATE_DT | DATETIME | | Update datetime |

**Constraints**:
- `UNIQUE(USER_ID, ACCOUNT_CODE)`: No duplicate codes within user
- `ON DELETE CASCADE`: Cascade delete when user is deleted

**Indexes**:
- `idx_accounts_user`: `(USER_ID, ACCOUNT_CODE)`
- `idx_accounts_user_order`: `(USER_ID, DISPLAY_ORDER)`
- `idx_accounts_template`: `TEMPLATE_CODE`

---

### TRANSACTIONS_HEADER Table

**Purpose**: Basic transaction information (one record per transaction)

| Column Name | Type | Constraints | Description |
|------------|------|------------|-------------|
| TRANSACTION_ID | INTEGER | PK, AUTOINCREMENT | Transaction ID |
| USER_ID | INTEGER | NOT NULL, FK → USERS | User ID |
| SHOP_ID | INTEGER | FK → SHOPS | Shop ID |
| CATEGORY1_CODE | VARCHAR(50) | NOT NULL, FK → CATEGORY1 | Major category code |
| FROM_ACCOUNT_CODE | VARCHAR(50) | NOT NULL, FK → ACCOUNTS | Payment source account |
| TO_ACCOUNT_CODE | VARCHAR(50) | NOT NULL, FK → ACCOUNTS | Payment destination account |
| TRANSACTION_DATE | DATETIME | NOT NULL | Transaction datetime |
| TOTAL_AMOUNT | INTEGER | NOT NULL | Total amount |
| TAX_ROUNDING_TYPE | INTEGER | DEFAULT 0 | Tax rounding method |
| TAX_INCLUDED_TYPE | INTEGER | DEFAULT 1, NOT NULL | Tax included/excluded |
| MEMO_ID | INTEGER | FK → MEMOS | Memo ID |
| IS_DISABLED | INTEGER | DEFAULT 0 | Disabled flag |
| ENTRY_DT | DATETIME | NOT NULL | Entry datetime |
| UPDATE_DT | DATETIME | | Update datetime |

**Indexes**:
- `idx_transactions_header_user`: `(USER_ID, TRANSACTION_DATE)` – User-based date search
- `idx_transactions_header_accounts`: `(FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE)` – Account transfer search
- `idx_transactions_header_category`: `CATEGORY1_CODE` – Category search
- `idx_transactions_header_date`: `TRANSACTION_DATE` – Date range search

---

### TRANSACTIONS_DETAIL Table

**Purpose**: Transaction details (item-level)

| Column Name | Type | Constraints | Description |
|------------|------|------------|-------------|
| DETAIL_ID | INTEGER | PK, AUTOINCREMENT | Detail ID |
| TRANSACTION_ID | INTEGER | NOT NULL, FK → TRANSACTIONS_HEADER | Transaction ID |
| USER_ID | INTEGER | NOT NULL | User ID |
| CATEGORY1_CODE | VARCHAR(50) | NOT NULL | Major category code |
| CATEGORY2_CODE | VARCHAR(50) | NOT NULL | Medium category code |
| CATEGORY3_CODE | VARCHAR(50) | NOT NULL | Minor category code |
| ITEM_NAME | TEXT | NOT NULL, CHECK ≠ '' | Item name |
| AMOUNT | INTEGER | NOT NULL | Amount |
| TAX_AMOUNT | INTEGER | DEFAULT 0 | Tax amount |
| TAX_RATE | INTEGER | DEFAULT 8 | Tax rate (%) |
| MEMO_ID | INTEGER | FK → MEMOS | Memo ID |
| ENTRY_DT | DATETIME | NOT NULL | Entry datetime |
| UPDATE_DT | DATETIME | | Update datetime |

**Constraints**:
- `ON DELETE CASCADE`: Cascade delete when header is deleted
- Composite foreign key constraints ensure category consistency

**Indexes**:
- `idx_transactions_detail_transaction`: `TRANSACTION_ID` – Join to header
- `idx_transactions_detail_categories`: `(CATEGORY2_CODE, CATEGORY3_CODE)` – Category search

---

### CATEGORY1/2/3 Tables

**Purpose**: 3-tier category master data

**Common Columns**:
| Column Name | Type | Constraints | Description |
|------------|------|------------|-------------|
| USER_ID | INTEGER | PK part, NOT NULL | User ID |
| CATEGORY1_CODE | VARCHAR(64) | PK part, NOT NULL | Major category code |
| DISPLAY_ORDER | INTEGER | NOT NULL | Display order |
| CATEGORY*_NAME | VARCHAR(128) | NOT NULL | Category name |
| IS_DISABLED | INTEGER | DEFAULT 0 | Disabled flag |
| ENTRY_DT | DATETIME | NOT NULL | Entry datetime |
| UPDATE_DT | DATETIME | | Update datetime |

**Hierarchy**:
- CATEGORY2: Additional PK `CATEGORY2_CODE`, FK → CATEGORY1
- CATEGORY3: Additional PK `CATEGORY2_CODE, CATEGORY3_CODE`, FK → CATEGORY2

**Cascade Deletion**:
- CATEGORY1 deletion → CATEGORY2 deletion → CATEGORY3 deletion

**Indexes**:
- Each tier has `(USER_ID, DISPLAY_ORDER)` index
- Reference indexes to parent categories
- Disabled flag search indexes

---

### MEMOS Table

**Purpose**: Deduplication store for memo text

| Column Name | Type | Constraints | Description |
|------------|------|------------|-------------|
| MEMO_ID | INTEGER | PK, AUTOINCREMENT | Memo ID |
| USER_ID | INTEGER | NOT NULL, FK → USERS | User ID |
| MEMO_TEXT | TEXT | NOT NULL, CHECK ≠ '' | Memo body |
| ENTRY_DT | DATETIME | NOT NULL | Entry datetime |
| UPDATE_DT | DATETIME | | Update datetime |

**Design Intent**:
- Prevent duplicate storage of identical text
- Can be referenced from multiple transactions/details
- Managed per user

**Indexes**:
- `idx_memos_user`: `USER_ID`
- `idx_memos_text`: `(USER_ID, MEMO_TEXT)` – Text search

---

## Indexing Strategy

### Basic Policy
1. **Primary Keys**: Defined on all tables (auto-indexed)
2. **Foreign Keys**: Indexed for frequently referenced relationships
3. **Composite Indexes**: Combinations frequently used in WHERE clauses
4. **Display Order**: Indexes for queries including `DISPLAY_ORDER`

### Performance Optimization Points
- **User × Date**: Most frequent pattern for transaction search
- **Category Hierarchy**: Indexes on composite foreign keys
- **Text Search**: Indexes on MEMO_TEXT, SHOP_NAME, etc.

---

## Foreign Key Constraints and Cascades

### Cascade Deletion Design
| Parent Table | Child Table | Action |
|-------------|-------------|--------|
| USERS | ACCOUNTS, CATEGORY1, MEMOS | ON DELETE CASCADE |
| CATEGORY1 | CATEGORY2 | ON DELETE CASCADE |
| CATEGORY2 | CATEGORY3 | ON DELETE CASCADE |
| TRANSACTIONS_HEADER | TRANSACTIONS_DETAIL | ON DELETE CASCADE |
| MANUFACTURERS | PRODUCTS | ON DELETE SET NULL |

### Referential Integrity
- All foreign keys explicitly define `ON DELETE` actions
- Application-level constraint checks are minimized

---

## Data Type Selection

### Numeric Types
- **INTEGER**: IDs, amounts, tax rates, flags
  - Treated as 64-bit integers in SQLite
  - Amounts stored as integers in "yen" (no decimals)

### String Types
- **VARCHAR(n)**: Codes, short fixed-length strings
- **TEXT**: Variable-length text (memos, names)

### Date/Time Types
- **DATETIME**: ISO 8601 format (e.g., `2024-12-05 03:59:00`)
  - Stored as strings in SQLite
  - Default value: `datetime('now')`

### Binary Types
- **BLOB**: Encrypted data, salts, nonces

---

## Internationalization Support

### Multilingual Tables
- `I18N_RESOURCES`: System messages
- `CATEGORY*_I18N`: Category name translations

### Design Pattern
- Main tables: Default language (Japanese)
- I18N tables: Additional languages only
- `LANG_CODE`: 'ja', 'en', etc. (ISO 639-1)

---

## Security Considerations

### Encrypted Data
- `MASTER_KEY_ENCRYPTED`: AES-256-GCM encrypted
- Encrypted with password-derived key (decrypted at user login)

### Password Management
- `PASSWORD_HASH`: Argon2id hash
- Salt automatically generated (Argon2 internal)

### Access Control
- User-based data isolation (USER_ID foreign key)
- Feature restrictions by ROLE (application layer)

---

## Related Documentation

- [Database Configuration Guide](../guides/DATABASE_CONFIGURATION.md)
- [Database Migration](../guides/DATABASE_MIGRATION.md)
- [Security Design](SECURITY_DESIGN_en.md)
- [Architecture Overview](ARCHITECTURE_en.md)

---

**Last Updated**: 2025-12-05 04:04 JST
