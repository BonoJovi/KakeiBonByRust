# Database Configuration Guide

## Overview

The KakeiBon application stores user data and configuration information in a SQLite database. This guide explains database file placement, path management, and configuration methods.

## Database File Location

### Production Environment

The official database file is located at:

```
$HOME/.kakeibon/KakeiBonDB.sqlite3
```

- **Path**: `$HOME/.kakeibon/KakeiBonDB.sqlite3`
- **Directory**: `$HOME/.kakeibon/`
- **Filename**: `KakeiBonDB.sqlite3`
- **Size**: Approximately 170KB (initial state)

### Test Environment

Tests use an in-memory database:

```rust
#[cfg(test)]
fn get_test_connection() -> Result<Connection> {
    Connection::open_in_memory()?
}
```

## Database Path Management

### Constants Definition

Database-related constants are centrally managed in `src-tauri/src/consts.rs`:

```rust
// src-tauri/src/consts.rs
pub const DB_DIR_NAME: &str = ".kakeibon";
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";
```

### Path Helper Function

Common helper function used for all database connections:

```rust
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME};

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}
```

**Usage Example:**

```rust
// src-tauri/src/db/i18n.rs
pub fn get_all_translations(lang_code: &str) -> Result<HashMap<String, String>> {
    let conn = Connection::open(get_db_path())?;
    // ...
}

// src-tauri/src/db/category.rs
pub fn get_connection() -> Result<Connection> {
    Connection::open(get_db_path())
}
```

## Database Connection Best Practices

### ✅ Recommended Approach

```rust
// 1. Define constants in consts.rs
pub const DB_DIR_NAME: &str = ".kakeibon";
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";

// 2. Use helper function
fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}

// 3. Use helper function for all connections
let conn = Connection::open(get_db_path())?;
```

### ❌ Approaches to Avoid

```rust
// Hardcoded paths (may differ between production and development)
Connection::open("kakeibo.db")?
Connection::open("src-tauri/kakeibo.db")?

// Relative paths (dependent on current directory)
Connection::open("./database/kakeibo.db")?
```

## Separating Test and Production

### Conditional Compilation

Use different databases for test and production environments:

```rust
#[cfg(test)]
pub fn get_connection() -> Result<Connection> {
    // Test: In-memory DB
    Connection::open_in_memory()
}

#[cfg(not(test))]
pub fn get_connection() -> Result<Connection> {
    // Production: File-based DB
    Connection::open(get_db_path())
}
```

### Test Data Setup

```rust
#[cfg(test)]
fn setup_test_db(conn: &Connection) -> Result<()> {
    // Create tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS I18N_RESOURCES (
            RESOURCE_ID INTEGER PRIMARY KEY,
            RESOURCE_KEY VARCHAR(256) NOT NULL,
            LANG_CODE VARCHAR(10) NOT NULL,
            RESOURCE_VALUE TEXT NOT NULL,
            ENTRY_DT DATETIME NOT NULL
        )",
        [],
    )?;
    
    // Insert test data
    conn.execute(
        "INSERT INTO I18N_RESOURCES VALUES (1, 'test.key', 'ja', 'テスト', datetime('now'))",
        [],
    )?;
    
    Ok(())
}

#[test]
fn test_translation() {
    let conn = get_connection().unwrap();
    setup_test_db(&conn).unwrap();
    // Run test...
}
```

## Database Initialization

### On Application Startup

Initialize `.kakeibon` directory and database file on application startup:

```rust
use std::fs;
use std::path::Path;

pub fn initialize_database() -> Result<()> {
    // Create directory if it doesn't exist
    let home = std::env::var("HOME")?;
    let db_dir = Path::new(&home).join(DB_DIR_NAME);
    
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir)?;
    }
    
    // Create database file if it doesn't exist
    let db_path = get_db_path();
    if !db_path.exists() {
        let conn = Connection::open(&db_path)?;
        create_tables(&conn)?;
        insert_initial_data(&conn)?;
    }
    
    Ok(())
}
```

## Database Schema

### Main Tables

#### USERS
```sql
CREATE TABLE USERS (
    USER_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USERNAME VARCHAR(64) NOT NULL UNIQUE,
    PASSWORD_HASH TEXT NOT NULL,
    ROLE INTEGER NOT NULL DEFAULT 1,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME
);
```

#### I18N_RESOURCES
```sql
CREATE TABLE I18N_RESOURCES (
    RESOURCE_ID INTEGER PRIMARY KEY,
    RESOURCE_KEY VARCHAR(256) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    RESOURCE_VALUE TEXT NOT NULL,
    CATEGORY VARCHAR(64),
    DESCRIPTION VARCHAR(512),
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    UNIQUE(RESOURCE_KEY, LANG_CODE)
);
```

#### CATEGORY1/2/3
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
```

## Backup and Restore

### Manual Backup

```bash
# Create backup
cp $HOME/.kakeibon/KakeiBonDB.sqlite3 \
   $HOME/.kakeibon/KakeiBonDB.sqlite3.backup.$(date +%Y%m%d_%H%M%S)

# Restore
cp $HOME/.kakeibon/KakeiBonDB.sqlite3.backup.20251028_120000 \
   $HOME/.kakeibon/KakeiBonDB.sqlite3
```

### SQL Dump Backup

```bash
# Backup (SQL dump)
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 .dump > backup.sql

# Restore
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 < backup.sql
```

## Troubleshooting

### Database File Not Found

```bash
# Check database file existence
ls -lh $HOME/.kakeibon/KakeiBonDB.sqlite3

# Check directory
ls -la $HOME/.kakeibon/
```

### Permission Errors

```bash
# Check permissions
ls -l $HOME/.kakeibon/KakeiBonDB.sqlite3

# Fix permissions
chmod 644 $HOME/.kakeibon/KakeiBonDB.sqlite3
```

### Database Corruption

```bash
# Integrity check
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA integrity_check;"

# Repair (rebuild to new DB)
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 ".dump" | \
  sqlite3 $HOME/.kakeibon/KakeiBonDB_new.sqlite3
```

## .gitignore Configuration

Exclude development database files from Git:

```gitignore
# Development databases
*.db
*.sqlite
*.sqlite3
!schema.sql

# Test databases
test_*.db

# Backup files
*.backup
*.bak

# Note: Production database is in $HOME/.kakeibon/,
# so it doesn't exist in project directory and doesn't need exclusion
```

## Related Documentation

- [Troubleshooting Guide](./TROUBLESHOOTING.md)
- [I18N Implementation Guide](./I18N_IMPLEMENTATION.md)
- [Developer Guide](./DEVELOPER_GUIDE.md)

---

Last Updated: 2025-10-28
