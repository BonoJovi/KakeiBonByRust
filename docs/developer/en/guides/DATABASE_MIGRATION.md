# Database Migration Guide

**Last Updated**: 2025-10-29 13:30 JST

## Table of Contents
1. [Overview](#overview)
2. [Database Structure](#database-structure)
3. [Initialization Process](#initialization-process)
4. [Adding Translation Resources](#adding-translation-resources)
5. [Schema Change Procedures](#schema-change-procedures)
6. [Backup and Restore](#backup-and-restore)
7. [Migration Script Management](#migration-script-management)
8. [Troubleshooting](#troubleshooting)

---

## Overview

KakeiBon uses **SQLite** database, and the database file is stored at:

```
$HOME/.kakeibon/KakeiBonDB.sqlite3
```

### Migration Strategy

KakeiBon currently adopts a **simple initialization-based approach**:

- **First Launch**: Initialize schema from `res/sql/dbaccess.sql`
- **Existing Database**: Check on application startup (no re-initialization)
- **Migration**: Manually execute SQL scripts as needed

> **Note**: Full-fledged migration tools (e.g., Refinery, sqlx migrations) are under consideration for future implementation, but are not currently implemented.

---

## Database Structure

### Main Tables

| Table Name | Description | Primary Key |
|-----------|-------------|-------------|
| `USERS` | User accounts | `USER_ID` |
| `I18N_RESOURCES` | System translation resources | `RESOURCE_ID` |
| `CATEGORY1` | Major categories (Expense/Income/Transfer) | `(USER_ID, CATEGORY1_CODE)` |
| `CATEGORY2` | Middle categories | `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE)` |
| `CATEGORY3` | Minor categories | `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE)` |
| `CATEGORY1_I18N` | Multilingual names for major categories | `(USER_ID, CATEGORY1_CODE, LANG_CODE)` |
| `CATEGORY2_I18N` | Multilingual names for middle categories | `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, LANG_CODE)` |
| `CATEGORY3_I18N` | Multilingual names for minor categories | `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, LANG_CODE)` |
| `ENCRYPTED_FIELDS` | Encrypted field management | `FIELD_ID` |

### Foreign Key Constraints

```sql
-- CATEGORY2 → CATEGORY1
FOREIGN KEY(USER_ID, CATEGORY1_CODE) 
    REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE) 
    ON DELETE CASCADE

-- CATEGORY3 → CATEGORY2
FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) 
    REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) 
    ON DELETE CASCADE
```

Cascade deletion ensures that child categories are automatically deleted when a parent category is deleted.

---

## Initialization Process

### Application Startup Flow

```rust
// src/db.rs
pub async fn new() -> Result<Self, sqlx::Error> {
    let db_path = get_db_path();  // $HOME/.kakeibon/KakeiBonDB.sqlite3
    
    // Create directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Connect to database (create if doesn't exist)
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
    let pool = SqlitePool::connect(&db_url).await?;
    
    // Enable WAL mode (performance improvement)
    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await?;
    
    Ok(Database { pool })
}

pub async fn initialize(&self) -> Result<(), sqlx::Error> {
    // Load and execute res/sql/dbaccess.sql
    let sql_path = get_sql_file_path();  // res/sql/dbaccess.sql
    let sql_content = std::fs::read_to_string(&sql_path)?;
    
    // Remove comment lines and execute
    for statement in sql_content.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() && !trimmed.starts_with("--") {
            sqlx::query(trimmed).execute(&self.pool).await?;
        }
    }
    
    Ok(())
}
```

### Initialization SQL

`res/sql/dbaccess.sql` contains:

1. Table creation (`CREATE TABLE IF NOT EXISTS`)
2. Index creation (`CREATE INDEX IF NOT EXISTS`)
3. System translation resource insertion (`INSERT OR IGNORE`)
4. Template user (USER_ID=1) category initial data

---

## Adding Translation Resources

### System Translation Resources (I18N_RESOURCES)

When adding new UI elements, add translation resources to `res/sql/dbaccess.sql`.

#### Steps

**1. Add to SQL File**

Add to the end of `res/sql/dbaccess.sql` in the following format:

```sql
-- Japanese resource
INSERT OR IGNORE INTO I18N_RESOURCES (
    RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, 
    CATEGORY, DESCRIPTION, ENTRY_DT
) VALUES (
    'new_feature.button_label',  -- Key (dot notation)
    'ja',                         -- Language code
    'ボタンのラベル',              -- Translation text
    'new_feature',                -- Category (feature name)
    'Button label for new feature',  -- Description
    datetime('now')               -- Entry date
);

-- English resource
INSERT OR IGNORE INTO I18N_RESOURCES (
    RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, 
    CATEGORY, DESCRIPTION, ENTRY_DT
) VALUES (
    'new_feature.button_label',
    'en',
    'Button Label',
    'new_feature',
    'Button label for new feature',
    datetime('now')
);
```

**2. Naming Conventions**

- **Key format**: `{category}.{subcategory}.{element}`
  - Examples: `user_mgmt.add_user`, `category_mgmt.edit_category1`
- **Category**: Group by feature
  - Examples: `user_mgmt`, `category_mgmt`, `common`, `error_messages`

**3. Apply to Existing Database**

To add new resources to an existing database:

```bash
# Create SQL file with only translation resources
cat > add_translations.sql << 'EOF'
INSERT OR IGNORE INTO I18N_RESOURCES ...
EOF

# Apply to existing database
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < add_translations.sql
```

**4. Usage in Frontend**

```javascript
// Access via res/js/i18n.js
const label = await i18n.t('new_feature.button_label');
```

### Multilingual Category Names (CATEGORY*_I18N)

When adding or changing category names, also add records to the corresponding I18N table.

```sql
-- Add multilingual name for middle category
INSERT INTO CATEGORY2_I18N (
    USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, 
    LANG_CODE, CATEGORY2_NAME_I18N, ENTRY_DT
) VALUES
    (1, 'EXPENSE', 'C2_E_FOOD', 'ja', '食費', datetime('now')),
    (1, 'EXPENSE', 'C2_E_FOOD', 'en', 'Food', datetime('now'));
```

---

## Schema Change Procedures

### Adding a New Table

**1. Create SQL Script**

```sql
-- sql/migrations/001_add_new_table.sql
CREATE TABLE IF NOT EXISTS NEW_TABLE (
    ID INTEGER PRIMARY KEY,
    NAME VARCHAR(128) NOT NULL,
    ENTRY_DT DATETIME NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_new_table_name ON NEW_TABLE(NAME);
```

**2. Add to dbaccess.sql**

Add the above SQL to `res/sql/dbaccess.sql` at an appropriate location.

**3. Apply to Existing Database**

```bash
# Verify in development environment
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < sql/migrations/001_add_new_table.sql

# Confirm table creation
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 ".schema NEW_TABLE"
```

**4. Update Rust Code**

```rust
// src/models.rs or appropriate file
#[derive(Debug, Serialize, Deserialize)]
pub struct NewTable {
    pub id: i64,
    pub name: String,
    pub entry_dt: String,
}
```

### Adding Columns

**To add new columns to an existing table:**

```sql
-- Migration for existing database
ALTER TABLE USERS ADD COLUMN EMAIL VARCHAR(256);
ALTER TABLE USERS ADD COLUMN PHONE VARCHAR(20);

-- Also update dbaccess.sql (for new installations)
CREATE TABLE IF NOT EXISTS USERS (
    USER_ID INTEGER NOT NULL,
    NAME VARCHAR(128) NOT NULL UNIQUE,
    PAW VARCHAR(128) NOT NULL,
    ROLE INTEGER NOT NULL,
    EMAIL VARCHAR(256),          -- Added
    PHONE VARCHAR(20),            -- Added
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID)
);
```

> **Note**: SQLite doesn't support adding `NOT NULL` columns with `ALTER TABLE`. In such cases, use one of the following methods:
> - Specify default value: `ALTER TABLE USERS ADD COLUMN EMAIL VARCHAR(256) NOT NULL DEFAULT '';`
> - Recreate the table (data migration required)

### Removing or Modifying Columns

SQLite doesn't support `ALTER TABLE DROP COLUMN` or `ALTER TABLE MODIFY COLUMN`.
To remove or modify columns, recreate the table with the following steps:

```sql
-- Step 1: Create table with new structure
CREATE TABLE USERS_NEW (
    USER_ID INTEGER NOT NULL,
    NAME VARCHAR(128) NOT NULL UNIQUE,
    PAW VARCHAR(128) NOT NULL,
    ROLE INTEGER NOT NULL,
    -- EMAIL column removed
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID)
);

-- Step 2: Migrate data
INSERT INTO USERS_NEW 
SELECT USER_ID, NAME, PAW, ROLE, ENTRY_DT, UPDATE_DT 
FROM USERS;

-- Step 3: Drop old table
DROP TABLE USERS;

-- Step 4: Rename new table
ALTER TABLE USERS_NEW RENAME TO USERS;

-- Step 5: Recreate indexes
CREATE INDEX IF NOT EXISTS idx_users_name ON USERS(NAME);
```

**Execute in development environment**:

```bash
# Create migration script
cat > sql/migrations/002_remove_email_column.sql << 'EOF'
-- (SQL from above)
EOF

# Create backup
cp ~/.kakeibon/KakeiBonDB.sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3.backup

# Execute migration
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < sql/migrations/002_remove_email_column.sql
```

---

## Backup and Restore

### Backup

**Method 1: File Copy (Recommended)**

```bash
# Copy database file directly
cp ~/.kakeibon/KakeiBonDB.sqlite3 ~/backup/KakeiBonDB_$(date +%Y%m%d_%H%M%S).sqlite3
```

**Method 2: SQL Dump**

```bash
# Create SQL dump (text format)
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 .dump > backup_$(date +%Y%m%d).sql
```

**Automatic Backup Script (Example)**

```bash
#!/bin/bash
# backup-kakeibo.sh

BACKUP_DIR="$HOME/backup/kakeibon"
DB_PATH="$HOME/.kakeibon/KakeiBonDB.sqlite3"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p "$BACKUP_DIR"
cp "$DB_PATH" "$BACKUP_DIR/KakeiBonDB_$TIMESTAMP.sqlite3"

# Delete backups older than 7 days
find "$BACKUP_DIR" -name "KakeiBonDB_*.sqlite3" -mtime +7 -delete

echo "Backup created: $BACKUP_DIR/KakeiBonDB_$TIMESTAMP.sqlite3"
```

**Automate with cron (daily at 2:00 AM)**

```bash
crontab -e

# Add the following line
0 2 * * * /path/to/backup-kakeibo.sh
```

### Restore

**Method 1: Restore from File**

```bash
# Stop the application
# Copy backup file
cp ~/backup/KakeiBonDB_20250129.sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3

# Delete WAL files (for consistency)
rm -f ~/.kakeibon/KakeiBonDB.sqlite3-wal
rm -f ~/.kakeibon/KakeiBonDB.sqlite3-shm

# Restart the application
```

**Method 2: Restore from SQL Dump**

```bash
# Delete existing database
rm ~/.kakeibon/KakeiBonDB.sqlite3

# Restore from dump
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < backup_20250129.sql
```

### Data Integrity Check

```bash
# Check database integrity
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA integrity_check;"

# Expected output: ok

# Check foreign key constraint integrity
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA foreign_key_check;"

# No output means OK
```

---

## Migration Script Management

### Directory Structure

```
sql/
├── README.md                           # SQL script documentation
├── phase4/                             # Phase-specific scripts
│   ├── phase4-0_init_categories.sql
│   └── phase4-0_update_category_codes.sql
└── migrations/                         # Migration scripts (future)
    ├── 001_add_new_table.sql
    └── 002_remove_email_column.sql
```

### Migration Script Naming Convention

```
{number}_{description}.sql
```

- **Number**: 3-digit sequence number (001, 002, ...)
- **Description**: Brief description in snake_case (e.g., `add_email_field`, `update_category_codes`)

### Script Execution Order

Execute migration scripts in numerical order:

```bash
# Manual execution example
for script in sql/migrations/*.sql; do
    echo "Applying $script..."
    sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < "$script"
done
```

### Phase-Specific Scripts

Phase-specific directories like `sql/phase4/` store scripts related to specific features under development.

**Usage**:
- Initial data setup in development environment
- Test data insertion
- Reference (not used for actual migrations)

**Execution Example**:

```bash
# Phase 4 category initialization
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < sql/phase4/phase4-0_init_categories.sql
```

> **Note**: After Phase 4-0, the database is already in the correct state, so these scripts are usually unnecessary.

---

## Troubleshooting

### Database is Locked

**Error**:
```
Error: database is locked
```

**Cause**: Another process is accessing the database.

**Solution**:

```bash
# 1. Completely stop the application
# 2. Check WAL files
ls -la ~/.kakeibon/KakeiBonDB.sqlite3*

# 3. Checkpoint WAL if necessary
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA wal_checkpoint(TRUNCATE);"

# 4. If still locked, check processes
lsof ~/.kakeibon/KakeiBonDB.sqlite3
```

### Foreign Key Constraint Violation

**Error**:
```
FOREIGN KEY constraint failed
```

**Cause**: Attempting to create a child record without a corresponding parent record.

**Debugging Method**:

```bash
# Check foreign key constraints
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA foreign_keys = ON;"
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA foreign_key_check;"

# Example output:
# CATEGORY2|1|CATEGORY1|0
# → Row 1 in CATEGORY2 references a non-existent CATEGORY1 record
```

**Solution**:

```bash
# Delete orphaned records
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 << 'EOF'
-- Delete CATEGORY2 records without parent
DELETE FROM CATEGORY2
WHERE NOT EXISTS (
    SELECT 1 FROM CATEGORY1 
    WHERE CATEGORY1.USER_ID = CATEGORY2.USER_ID 
    AND CATEGORY1.CATEGORY1_CODE = CATEGORY2.CATEGORY1_CODE
);
EOF
```

### Outdated Schema

**Symptom**: New columns or tables don't exist.

**Verification**:

```bash
# Check current schema
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 ".schema USERS"

# Compare with expected schema
cat res/sql/dbaccess.sql | grep -A 10 "CREATE TABLE.*USERS"
```

**Solution**:

1. **Create Backup**
   ```bash
   cp ~/.kakeibon/KakeiBonDB.sqlite3 ~/backup/
   ```

2. **Execute Migration Script**
   ```bash
   sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < sql/migrations/XXX_add_missing_columns.sql
   ```

3. **Or, Recreate Database (Warning: Data Loss!)**
   ```bash
   rm ~/.kakeibon/KakeiBonDB.sqlite3
   # Start application to re-initialize
   ```

### I18N Resource Not Found

**Symptom**: Translation keys are displayed as-is in the UI (e.g., `user_mgmt.add_user`)

**Cause**: Resource not registered in `I18N_RESOURCES` table.

**Verification**:

```bash
# Check if resource exists
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 << 'EOF'
SELECT RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE 
FROM I18N_RESOURCES 
WHERE RESOURCE_KEY = 'user_mgmt.add_user';
EOF
```

**Solution**:

```bash
# Add missing resource
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 << 'EOF'
INSERT OR IGNORE INTO I18N_RESOURCES (
    RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, 
    CATEGORY, ENTRY_DT
) VALUES 
    ('user_mgmt.add_user', 'ja', 'ユーザーを追加', 'user_mgmt', datetime('now')),
    ('user_mgmt.add_user', 'en', 'Add User', 'user_mgmt', datetime('now'));
EOF
```

---

## Related Documentation

- [Database Configuration Guide](DATABASE_CONFIGURATION.md) - Basic database configuration
- [Developer Guide](DEVELOPER_GUIDE.md) - Database connection patterns
- [Troubleshooting](TROUBLESHOOTING.md) - Common issues and solutions

---

## Future Improvements

### Migration Tool Integration

Currently, SQL scripts are managed manually, but future consideration includes integrating tools such as:

**Option 1: sqlx Migration Feature**

```bash
# Create migration
sqlx migrate add create_users_table

# Run migrations
sqlx migrate run --database-url sqlite://~/.kakeibon/KakeiBonDB.sqlite3
```

**Option 2: Refinery**

```rust
use refinery::embed_migrations;

embed_migrations!("migrations");

async fn run_migrations(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    migrations::runner().run_async(pool).await?;
    Ok(())
}
```

**Benefits**:
- Automated version control
- Rollback functionality
- Migration history tracking

---

**Last Updated**: 2025-10-29 13:30 JST
