# Developer Guide

## Table of Contents
1. [Project Overview](#project-overview)
2. [Development Environment Setup](#development-environment-setup)
3. [Project Structure](#project-structure)
4. [Architecture](#architecture)
5. [Coding Conventions](#coding-conventions)
6. [Constants Management Best Practices](#constants-management-best-practices)
7. [Database Connection Patterns](#database-connection-patterns)
8. [Test and Production Separation](#test-and-production-separation)
9. [Build and Test](#build-and-test)
10. [Debugging Methods](#debugging-methods)
11. [Troubleshooting](#troubleshooting)

---

## Project Overview

**KakeiBon** is a household budget application built with Rust + Tauri v2 + SQLite.

### Technology Stack
- **Frontend**: Vanilla JavaScript (ES6 Modules), HTML5, CSS3
- **Backend**: Rust 1.77.2+, Tauri v2.8.5
- **Database**: SQLite 3
- **Key Libraries**:
  - `rusqlite`: SQLite database operations
  - `serde/serde_json`: JSON serialization
  - `argon2`: Password hashing
  - `aes-gcm`: Data encryption
  - `chrono`: Date/time handling

### Key Features
- User management (admin and regular users)
- Multilingual support (Japanese and English)
- Customizable font sizes
- Hierarchical category management (major, middle, minor)
- Secure password management

---

## Development Environment Setup

### Required Tools
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (for Tauri frontend dependencies)
# Recommended: v18 or higher

# Install Tauri CLI
cargo install tauri-cli

# SQLite3 (for debugging)
sudo apt install sqlite3  # Ubuntu/Debian
brew install sqlite3      # macOS
```

### Clone and Setup Project
```bash
git clone <repository-url>
cd KakeiBonByRust

# Install dependencies
cargo build

# Run in development mode
cargo tauri dev
```

---

## Project Structure

```
KakeiBonByRust/
├── src/                    # Frontend source
│   ├── index.html          # Main HTML
│   ├── js/                 # JavaScript modules
│   │   ├── main.js
│   │   ├── i18n.js        # Internationalization
│   │   ├── user_mgmt.js   # User management
│   │   └── category.js    # Category management
│   └── css/               # Stylesheets
│
├── src-tauri/             # Backend source (Rust)
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   ├── lib.rs         # Tauri command registration
│   │   ├── consts.rs      # Constants definition
│   │   ├── commands/      # Tauri command implementation
│   │   │   ├── mod.rs
│   │   │   ├── i18n.rs
│   │   │   ├── category.rs
│   │   │   └── settings.rs
│   │   ├── db/            # Database access layer
│   │   │   ├── mod.rs
│   │   │   ├── i18n.rs
│   │   │   └── category.rs
│   │   └── models/        # Data models
│   │       ├── mod.rs
│   │       └── category.rs
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
│
├── res/                   # Resource files
│   └── sql/
│       └── dbaccess.sql   # Database initialization SQL
│
├── docs/                  # Documentation
│   ├── ja/               # Japanese documentation
│   └── en/               # English documentation
│
└── TODO.md               # Task management
```

---

## Architecture

### Layer Structure

```
┌─────────────────────────────────────┐
│      Frontend (JavaScript)          │
│  - UI rendering                     │
│  - User interactions                │
│  - Tauri API invocation             │
└──────────────┬──────────────────────┘
               │ invoke()
               ▼
┌─────────────────────────────────────┐
│    Tauri Commands (lib.rs)          │
│  - commands::i18n::*                │
│  - commands::category::*            │
│  - commands::settings::*            │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│    Database Access Layer (db/)      │
│  - db::i18n::get_all_translations() │
│  - db::category::*                  │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│       SQLite Database               │
│  $HOME/.kakeibon/KakeiBonDB.sqlite3 │
└─────────────────────────────────────┘
```

### Data Flow Example: Language Change

```
1. User clicks language menu (Frontend)
   ↓
2. invoke('set_language', {language: 'en'}) (Frontend)
   ↓
3. commands::i18n::set_language() (Tauri Command)
   ↓
4. Save settings to KakeiBon.json (Backend)
   ↓
5. Return success message (Backend → Frontend)
   ↓
6. invoke('get_translations', {language: 'en'}) (Frontend)
   ↓
7. db::i18n::get_all_translations() (Database Access)
   ↓
8. Return HashMap<String, String> (Backend → Frontend)
   ↓
9. Update UI (Frontend)
```

---

## Coding Conventions

### Rust Coding Conventions

#### Naming Rules
```rust
// Constants: UPPER_SNAKE_CASE
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";

// Functions: snake_case
pub fn get_translations(language: String) -> Result<HashMap<String, String>, String> {
    // ...
}

// Structs: PascalCase
pub struct Category1 {
    pub user_id: i64,
    pub category1_code: String,
    // ...
}

// Variables: snake_case
let db_path = get_db_path();
```

#### Error Handling
```rust
// Use Result type
pub fn get_connection() -> Result<Connection, rusqlite::Error> {
    Connection::open(get_db_path())
}

// Convert errors to strings with map_err
#[tauri::command]
pub fn get_translations(language: String) -> Result<HashMap<String, String>, String> {
    get_all_translations(&language)
        .map_err(|e| format!("Failed to get translations: {}", e))
}
```

#### Documentation Comments
```rust
/// Get list of major categories for a user ID
///
/// # Arguments
/// * `user_id` - User ID
///
/// # Returns
/// List of Category1 or error
pub fn get_category1_list(user_id: i64) -> Result<Vec<Category1>, rusqlite::Error> {
    // Implementation
}
```

### JavaScript Coding Conventions

#### Module Structure
```javascript
// Use ES6 Modules
import { invoke } from '@tauri-apps/api/core';

// Prevent global namespace pollution
const CategoryManager = {
    init() { /* ... */ },
    loadCategories() { /* ... */ }
};

export default CategoryManager;
```

#### Asynchronous Processing
```javascript
// Use async/await
async function loadTranslations(language) {
    try {
        const translations = await invoke('get_translations', { language });
        return translations;
    } catch (error) {
        console.error('Failed to load translations:', error);
        throw error;
    }
}
```

#### Naming Rules
```javascript
// Classes: PascalCase
class I18n { }

// Functions: camelCase
function loadCategories() { }

// Constants: UPPER_SNAKE_CASE
const DEFAULT_LANGUAGE = 'ja';
```

---

## Constants Management Best Practices

### Centralized Constants Management (`src-tauri/src/consts.rs`)

All constants used throughout the project are centrally managed in `consts.rs`.

```rust
// src-tauri/src/consts.rs

// User role constants
pub const ROLE_ADMIN: i64 = 0;
pub const ROLE_USER: i64 = 1;
pub const ROLE_VISIT: i64 = 999;

// Database constants
pub const DB_DIR_NAME: &str = ".kakeibon";
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";
pub const SQL_INIT_FILE_PATH: &str = "res/sql/dbaccess.sql";

// Language constants
pub const LANG_ENGLISH: &str = "en";
pub const LANG_JAPANESE: &str = "ja";
pub const LANG_DEFAULT: &str = LANG_JAPANESE;

// Font size constants
pub const FONT_SIZE_SMALL: &str = "small";
pub const FONT_SIZE_MEDIUM: &str = "medium";
pub const FONT_SIZE_LARGE: &str = "large";
pub const FONT_SIZE_DEFAULT: &str = FONT_SIZE_MEDIUM;
```

### Usage Example

```rust
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME, LANG_DEFAULT};

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}

fn get_default_language() -> &'static str {
    LANG_DEFAULT
}
```

### Checklist for Adding Constants

- [ ] Add constant to `consts.rs`
- [ ] Use proper naming convention (UPPER_SNAKE_CASE)
- [ ] Specify type explicitly (`&str`, `i64`, etc.)
- [ ] Update related documentation
- [ ] Replace existing hardcoded values

---

## Database Connection Patterns

### Basic Pattern

```rust
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME};

/// Get database path
fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}

/// Get database connection
pub fn get_connection() -> Result<Connection> {
    Connection::open(get_db_path())
}
```

### Transaction Processing

```rust
pub fn add_category_with_children(
    user_id: i64,
    category: Category1,
    children: Vec<Category2>
) -> Result<(), rusqlite::Error> {
    let mut conn = get_connection()?;
    let tx = conn.transaction()?;

    // Add parent category
    tx.execute(
        "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, ...) VALUES (?1, ?2, ...)",
        params![user_id, category.code],
    )?;

    // Add child categories
    for child in children {
        tx.execute(
            "INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, ...) VALUES (?1, ?2, ?3, ...)",
            params![user_id, category.code, child.code],
        )?;
    }

    tx.commit()?;
    Ok(())
}
```

### Using Prepared Statements

```rust
pub fn get_categories_by_user(user_id: i64) -> Result<Vec<Category1>, rusqlite::Error> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT USER_ID, CATEGORY1_CODE, CATEGORY1_NAME, DISPLAY_ORDER 
         FROM CATEGORY1 
         WHERE USER_ID = ?1 
         ORDER BY DISPLAY_ORDER"
    )?;

    let category_iter = stmt.query_map([user_id], |row| {
        Ok(Category1 {
            user_id: row.get(0)?,
            category1_code: row.get(1)?,
            category1_name: row.get(2)?,
            display_order: row.get(3)?,
        })
    })?;

    let mut categories = Vec::new();
    for category in category_iter {
        categories.push(category?);
    }
    Ok(categories)
}
```

### Database Initialization

```rust
use std::fs;

pub fn initialize_database() -> Result<(), String> {
    let db_path = get_db_path();
    
    // Create database directory
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create db directory: {}", e))?;
    }

    // Initialize only if database doesn't exist
    if !db_path.exists() {
        let conn = Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // Read and execute SQL file
        let sql = fs::read_to_string("res/sql/dbaccess.sql")
            .map_err(|e| format!("Failed to read SQL file: {}", e))?;

        conn.execute_batch(&sql)
            .map_err(|e| format!("Failed to execute SQL: {}", e))?;
    }

    Ok(())
}
```

---

## Test and Production Separation

### Test Database Connection

```rust
#[cfg(test)]
fn get_test_connection() -> Result<Connection> {
    // Use in-memory database
    let conn = Connection::open_in_memory()?;
    
    // Create test tables
    conn.execute(
        "CREATE TABLE CATEGORY1 (
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE VARCHAR(64) NOT NULL,
            DISPLAY_ORDER INTEGER NOT NULL,
            CATEGORY1_NAME VARCHAR(128) NOT NULL,
            PRIMARY KEY(USER_ID, CATEGORY1_CODE)
        )",
        [],
    )?;
    
    Ok(conn)
}
```

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_category() {
        // Arrange
        let conn = get_test_connection().unwrap();
        
        // Act
        let result = add_category1(&conn, 1, "EXPENSE", "Expense", 1);
        
        // Assert
        assert!(result.is_ok());
        
        // Verify
        let categories = get_category1_list(&conn, 1).unwrap();
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].category1_code, "EXPENSE");
    }

    #[test]
    fn test_transaction_rollback() {
        let conn = get_test_connection().unwrap();
        
        // Verify rollback on transaction failure
        let result = add_invalid_category(&conn);
        assert!(result.is_err());
        
        let categories = get_category1_list(&conn, 1).unwrap();
        assert_eq!(categories.len(), 0); // Verify no data was added
    }
}
```

### Environment-based Branching

```rust
fn get_db_path() -> PathBuf {
    if cfg!(test) {
        // Use temporary directory in test environment
        PathBuf::from("/tmp/kakeibon_test.db")
    } else {
        // Production environment
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
    }
}
```

---

## Build and Test

### Run in Development Mode

```bash
# Start Tauri app in development mode
cargo tauri dev

# Build backend only
cd src-tauri
cargo build
```

### Run Tests

```bash
# Run all tests
cargo test

# Run tests for specific module only
cargo test db::category

# Show detailed test output
cargo test -- --nocapture

# Disable parallel execution (for debugging)
cargo test -- --test-threads=1
```

### Production Build

```bash
# Release build
cargo tauri build

# Build artifacts are generated in
# target/release/bundle/
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting only (used in CI)
cargo fmt -- --check
```

### Lint Check

```bash
# Check code quality with Clippy
cargo clippy

# Strict mode
cargo clippy -- -D warnings
```

---

## Debugging Methods

### Logging

```rust
use log::{info, warn, error, debug};

pub fn some_function() {
    debug!("Debug message");
    info!("Info message");
    warn!("Warning message");
    error!("Error message");
}
```

### Debugging from Frontend

```javascript
// Console logging
console.log('Debug info:', data);
console.error('Error:', error);

// Catch Tauri command errors
try {
    const result = await invoke('some_command', { param: value });
    console.log('Success:', result);
} catch (error) {
    console.error('Command failed:', error);
}
```

### Database Debugging

```bash
# Connect directly to SQLite database
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3

# List tables
.tables

# Check schema
.schema CATEGORY1

# Check data
SELECT * FROM I18N_RESOURCES WHERE LANG_CODE = 'ja' LIMIT 10;

# Exit
.quit
```

### Debugging with Breakpoints

```rust
// Use dbg! macro
let result = dbg!(some_function());

// Crash with panic! to investigate
panic!("Debug point: value = {:?}", value);
```

---

## Troubleshooting

### Common Issues and Solutions

#### 1. Database Connection Error

**Issue**: `Failed to open database file`

**Cause**: Database file or directory doesn't exist

**Solution**:
```bash
# Create directory
mkdir -p ~/.kakeibon

# Initialize database
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < res/sql/dbaccess.sql
```

#### 2. Build Error

**Issue**: `error: linker 'cc' not found`

**Cause**: C compiler is not installed

**Solution**:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# macOS
xcode-select --install
```

#### 3. Test Failures

**Issue**: Tests fail intermittently

**Cause**: Race conditions due to parallel execution

**Solution**:
```bash
# Run in single thread
cargo test -- --test-threads=1
```

#### 4. Unable to Call Tauri Command

**Issue**: `Command not found`

**Cause**: Command is not registered in `lib.rs`

**Solution**:
```rust
// src-tauri/src/lib.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::i18n::get_translations,
            commands::category::get_category_tree,
            // ← Add new command here
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Debugging Tips

1. **Read error messages carefully**: Rust error messages are detailed and helpful
2. **Use logging**: Output logs appropriately with the `log` crate
3. **Test in small pieces**: Isolate and identify problems
4. **Refer to documentation**: Check Tauri and rusqlite documentation
5. **Ask the community**: Ask questions on GitHub Issues or Discord

---

## References

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [rusqlite Documentation](https://docs.rs/rusqlite/latest/rusqlite/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)

---

Last Updated: 2025-10-28
