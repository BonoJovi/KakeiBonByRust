# Troubleshooting Guide

## Translation Resource Not Displaying Issue

### Symptoms
- Translation resource keys (e.g., `menu.admin`) are displayed as literal strings in menus and UI
- Some translations work correctly, but specific keys are not translated

### Diagnostic Procedure

#### 1. Check Database Files
First, verify that the correct database file is being used.

```bash
# Official database file location
ls -lh $HOME/.kakeibon/KakeiBonDB.sqlite3

# Check for leftover development databases in project
find . -name "*.db" -o -name "*.sqlite*" 2>/dev/null | grep -v target
```

#### 2. Check Translation Resources in Database
Inspect the database directly using SQLite.

```bash
# Check if a specific key exists
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 \
  "SELECT RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE 
   FROM I18N_RESOURCES 
   WHERE RESOURCE_KEY = 'menu.admin';"

# Check all keys starting with menu.
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 \
  "SELECT RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE 
   FROM I18N_RESOURCES 
   WHERE RESOURCE_KEY LIKE 'menu.%' 
   ORDER BY RESOURCE_KEY, LANG_CODE;"
```

#### 3. Check Database Path in Code
Verify that the code references the correct database file.

```bash
# Search for database connection code
grep -r "Connection::open" src-tauri/src/db/ --include="*.rs"

# Check database path definitions
grep -r "DB_FILE_NAME\|DB_DIR_NAME" src-tauri/src/ --include="*.rs"
```

### Common Causes and Solutions

#### Cause 1: Translation Resource Not Registered in Database

**Verification:**
```bash
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 \
  "SELECT COUNT(*) FROM I18N_RESOURCES WHERE RESOURCE_KEY = 'menu.admin';"
```

**Solution:**
Add the translation resource to the database.

```bash
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 << 'EOF'
INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, ENTRY_DT) 
VALUES 
  ((SELECT COALESCE(MAX(RESOURCE_ID), 0) + 1 FROM I18N_RESOURCES), 
   'menu.admin', 'ja', '管理', datetime('now')),
  ((SELECT COALESCE(MAX(RESOURCE_ID), 0) + 2 FROM I18N_RESOURCES), 
   'menu.admin', 'en', 'Admin', datetime('now'));
EOF
```

#### Cause 2: Database File Path Inconsistency

**Symptoms:**
- Mixed use of development database (`src-tauri/kakeibo.db`, etc.) and production database (`$HOME/.kakeibon/KakeiBonDB.sqlite3`)
- Code uses hardcoded paths

**Verification:**
```rust
// Bad example (hardcoded)
Connection::open("kakeibo.db")?
Connection::open("src-tauri/kakeibo.db")?

// Good example (using constants)
Connection::open(get_db_path())?
```

**Solution:**
1. Define constants in `src-tauri/src/consts.rs`
```rust
pub const DB_DIR_NAME: &str = ".kakeibon";
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";
```

2. Implement database path helper function
```rust
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME};

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}

pub fn get_all_translations(lang_code: &str) -> Result<HashMap<String, String>> {
    let conn = Connection::open(get_db_path())?;
    // ...
}
```

3. Remove unnecessary development databases
```bash
rm -f kakeibo.db src-tauri/kakeibo.db KakeiBonDB.sqlite3
```

#### Cause 3: Current Directory Issue

**Symptoms:**
- Using relative paths to open database, resulting in accessing different files depending on the current working directory at runtime

**Verification:**
```bash
# Check current working directory at runtime
pwd

# Check for multiple database files
find . -name "kakeibo.db" -o -name "KakeiBonDB.sqlite3"
```

**Solution:**
Use absolute paths or HOME-based paths (see `get_db_path()` function above).

### Debugging Techniques

#### 1. Add Debug Logging in Frontend

```javascript
// res/js/i18n.js
async loadTranslations() {
    console.log('[DEBUG] Loading translations for:', this.currentLanguage);
    try {
        const translations = await invoke('get_translations', { 
            language: this.currentLanguage 
        });
        console.log('[DEBUG] Received translations:', Object.keys(translations).length, 'keys');
        console.log('[DEBUG] Sample keys:', Object.keys(translations).slice(0, 10));
        
        // Check specific key
        console.log('[DEBUG] menu.admin:', translations['menu.admin']);
        
        this.translations = translations;
    } catch (error) {
        console.error('[DEBUG] Error loading translations:', error);
    }
}
```

#### 2. Add Debug Logging in Backend

```rust
// src-tauri/src/db/i18n.rs
pub fn get_all_translations(lang_code: &str) -> Result<HashMap<String, String>> {
    let db_path = get_db_path();
    eprintln!("DEBUG: Opening database at: {:?}", db_path);
    
    let conn = Connection::open(&db_path)?;
    
    // Check record count
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM I18N_RESOURCES WHERE LANG_CODE = ?1",
        [lang_code],
        |row| row.get(0)
    )?;
    eprintln!("DEBUG: Found {} translation records for {}", count, lang_code);
    
    let mut stmt = conn.prepare(
        "SELECT RESOURCE_KEY, RESOURCE_VALUE FROM I18N_RESOURCES WHERE LANG_CODE = ?1"
    )?;
    
    let rows = stmt.query_map([lang_code], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    
    let mut translations = HashMap::new();
    for row_result in rows {
        let (key, value) = row_result?;
        if key == "menu.admin" {
            eprintln!("DEBUG: Found menu.admin = {}", value);
        }
        translations.insert(key, value);
    }
    
    eprintln!("DEBUG: Loaded {} translations", translations.len());
    Ok(translations)
}
```

#### 3. File Logging (Temporary Debug Use)

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn log_to_file(message: &str) {
    let log_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("work")
        .join("debug.log");
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let _ = writeln!(file, "[{}] {}", 
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), 
            message);
    }
}
```

**Note:** Always remove debug logs after resolving the issue.

### Checklist

When an issue occurs, check in the following order:

- [ ] Official database file exists (`$HOME/.kakeibon/KakeiBonDB.sqlite3`)
- [ ] Database file is not empty (check size with `ls -lh`)
- [ ] Translation resources are registered in database (SQL check)
- [ ] Code uses correct database path
- [ ] No leftover development database files
- [ ] Test in-memory DB is not created as actual file
- [ ] Browser cache is cleared (Ctrl+Shift+R)

### Prevention Measures

#### 1. Unified Database Path
Use `get_db_path()` function for all database connections.

#### 2. Centralized Constants Management
Manage database-related constants in `src-tauri/src/consts.rs`.

#### 3. Separate Test and Production
```rust
#[cfg(test)]
fn get_test_connection() -> Result<Connection> {
    // Use in-memory DB
    Connection::open_in_memory()
}

#[cfg(not(test))]
pub fn get_connection() -> Result<Connection> {
    // Use production DB
    Connection::open(get_db_path())
}
```

#### 4. Exclude Database Files
Add to `.gitignore` to prevent committing development databases.

```gitignore
# Development databases
*.db
*.sqlite
*.sqlite3
!schema.sql

# Production database is in $HOME/.kakeibon/, no exclusion needed
```

## Related Documentation

- [I18N Implementation Guide](./I18N_IMPLEMENTATION.md)
- [Settings Management](./SETTINGS_MANAGEMENT.md)
- [Development Environment Setup](../../README_en.md)

---

Last Updated: 2025-10-28
