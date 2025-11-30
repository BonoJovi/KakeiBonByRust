use sqlx::sqlite::SqlitePool;
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME, SQL_INIT_FILE_PATH};
use crate::sql_queries;

/// Connect to a SQLite database with the given URL
pub async fn connect_db(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePool::connect(db_url).await
}

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_path = get_db_path();
        
        // Create directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| sqlx::Error::Configuration(Box::new(e)))?;
        }
        
        // Ensure the database file can be created
        let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
        let pool = connect_db(&db_url).await?;
        
        // Enable WAL mode
        sqlx::query(sql_queries::DB_PRAGMA_WAL)
            .execute(&pool)
            .await?;
        
        Ok(Database { pool })
    }
    
    pub fn db_exists() -> bool {
        let db_path = get_db_path();
        db_path.exists()
    }
    
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
    
    pub async fn initialize(&self) -> Result<(), sqlx::Error> {
        // Load SQL from file
        let sql_path = get_sql_file_path();
        let sql_content = std::fs::read_to_string(&sql_path)
            .map_err(|e| sqlx::Error::Configuration(
                format!("Failed to read SQL file at {}: {}", sql_path.display(), e).into()
            ))?;
        
        // Remove comment lines first
        let cleaned_sql: Vec<&str> = sql_content
            .lines()
            .filter(|line| !line.trim().starts_with("--") && !line.trim().is_empty())
            .collect();
        let sql_without_comments = cleaned_sql.join("\n");
        
        // Execute each SQL statement
        for statement in sql_without_comments.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed)
                    .execute(&self.pool)
                    .await?;
            }
        }
        
        Ok(())
    }
    
    /// Run migrations for transaction-related tables
    pub async fn migrate_transactions(&self) -> Result<(), sqlx::Error> {
        // Create MEMOS table
        sqlx::query(sql_queries::CREATE_MEMOS_TABLE)
            .execute(&self.pool)
            .await?;

        // Create TRANSACTIONS_HEADER table
        sqlx::query(sql_queries::CREATE_TRANSACTIONS_HEADER_TABLE)
            .execute(&self.pool)
            .await?;

        // Check if TRANSACTIONS_DETAIL table needs migration
        let needs_migration = self.check_transactions_detail_needs_migration().await?;

        if needs_migration {
            // Perform migration for existing table
            self.migrate_transactions_detail_table().await?;
        } else {
            // Create TRANSACTIONS_DETAIL table with new schema
            sqlx::query(sql_queries::CREATE_TRANSACTIONS_DETAIL_TABLE)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Check if TRANSACTIONS_DETAIL table needs migration (old schema without USER_ID)
    async fn check_transactions_detail_needs_migration(&self) -> Result<bool, sqlx::Error> {
        // Check if TRANSACTIONS_DETAIL table exists
        let table_exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='TRANSACTIONS_DETAIL'"
        )
        .fetch_one(&self.pool)
        .await?;

        if table_exists == 0 {
            // Table doesn't exist, no migration needed
            return Ok(false);
        }

        // Check if USER_ID column exists
        let has_user_id: i64 = sqlx::query_scalar(sql_queries::CHECK_TRANSACTIONS_DETAIL_HAS_USER_ID)
            .fetch_one(&self.pool)
            .await?;

        // Needs migration if table exists but USER_ID column doesn't exist
        Ok(has_user_id == 0)
    }

    /// Migrate TRANSACTIONS_DETAIL table from old schema to new schema
    async fn migrate_transactions_detail_table(&self) -> Result<(), sqlx::Error> {
        // Begin transaction
        let mut tx = self.pool.begin().await?;

        // Disable foreign key constraints temporarily
        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&mut *tx)
            .await?;

        // Create new table with updated schema
        sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_CREATE_NEW)
            .execute(&mut *tx)
            .await?;

        // Copy data from old table to new table
        sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_COPY_DATA)
            .execute(&mut *tx)
            .await?;

        // Drop old table
        sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_DROP_OLD)
            .execute(&mut *tx)
            .await?;

        // Rename new table to original name
        sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_RENAME_NEW)
            .execute(&mut *tx)
            .await?;

        // Commit transaction
        tx.commit().await?;

        // Re-enable foreign key constraints
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    
    PathBuf::from(home)
        .join(DB_DIR_NAME)
        .join(DB_FILE_NAME)
}

fn get_sql_file_path() -> PathBuf {
    PathBuf::from(SQL_INIT_FILE_PATH)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Row;

    #[tokio::test]
    async fn test_wal_mode_enabled() {
        // Create a temporary database
        let temp_dir = std::env::temp_dir();
        let test_db_path = temp_dir.join("test_wal_mode.db");

        // Clean up any existing test database
        let _ = std::fs::remove_file(&test_db_path);

        // Set up temporary database path
        std::env::set_var("HOME", temp_dir.to_str().unwrap());

        // Create database connection
        let db = Database::new().await.expect("Failed to create database");

        // Query journal mode
        let result = sqlx::query("PRAGMA journal_mode;")
            .fetch_one(db.pool())
            .await
            .expect("Failed to query journal mode");

        let journal_mode: String = result.get(0);

        // Verify WAL mode is enabled
        assert_eq!(journal_mode.to_uppercase(), "WAL", "Database should be in WAL mode");

        // Clean up
        drop(db);
    }

    #[tokio::test]
    async fn test_transactions_detail_migration() {
        use crate::sql_queries;

        // Create a temporary database
        let temp_dir = std::env::temp_dir();
        let test_db_name = format!("test_migration_{}.db", std::process::id());
        let test_db_path = temp_dir.join(&test_db_name);

        // Clean up any existing test database
        let _ = std::fs::remove_file(&test_db_path);

        // Create database connection
        let db_url = format!("sqlite://{}?mode=rwc", test_db_path.display());
        let pool = connect_db(&db_url).await.expect("Failed to connect to database");

        // Enable foreign keys
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("Failed to enable foreign keys");

        // Create required tables for testing
        // Create USERS table
        sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE)
        .execute(&pool)
        .await
        .expect("Failed to create USERS table");

        // Insert test user
        sqlx::query(sql_queries::TEST_INSERT_TEST_USER)
            .execute(&pool)
            .await
            .expect("Failed to insert test user");

        // Create CATEGORY1 table
        sqlx::query(sql_queries::TEST_CREATE_CATEGORY1_TABLE)
        .execute(&pool)
        .await
        .expect("Failed to create CATEGORY1 table");

        // Insert test category1
        sqlx::query(sql_queries::TEST_INSERT_CATEGORY1)
            .execute(&pool)
            .await
            .expect("Failed to insert test category1");

        // Create CATEGORY2 table
        sqlx::query(sql_queries::TEST_CREATE_CATEGORY2_TABLE)
            .execute(&pool)
            .await
            .expect("Failed to create CATEGORY2 table");

        // Insert test category2
        sqlx::query(sql_queries::TEST_INSERT_CATEGORY2)
            .execute(&pool)
            .await
            .expect("Failed to insert test category2");

        // Create CATEGORY3 table
        sqlx::query(sql_queries::TEST_CREATE_CATEGORY3_TABLE)
            .execute(&pool)
            .await
            .expect("Failed to create CATEGORY3 table");

        // Insert test category3
        sqlx::query(sql_queries::TEST_INSERT_CATEGORY3)
            .execute(&pool)
            .await
            .expect("Failed to insert test category3");

        // Create MEMOS table
        sqlx::query(sql_queries::CREATE_MEMOS_TABLE)
            .execute(&pool)
            .await
            .expect("Failed to create MEMOS table");

        // Create ACCOUNT_TEMPLATES table (required by ACCOUNTS)
        sqlx::query(sql_queries::TEST_ACCOUNT_CREATE_TEMPLATES_TABLE)
            .execute(&pool)
            .await
            .expect("Failed to create ACCOUNT_TEMPLATES table");

        // Insert test account template
        sqlx::query("INSERT INTO ACCOUNT_TEMPLATES (TEMPLATE_CODE, TEMPLATE_NAME_JA, TEMPLATE_NAME_EN, DISPLAY_ORDER) VALUES ('CASH', '現金', 'Cash', 1)")
            .execute(&pool)
            .await
            .expect("Failed to insert test account template");

        // Create ACCOUNTS table (required by TRANSACTIONS_HEADER foreign keys)
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_ACCOUNTS_TABLE)
            .execute(&pool)
            .await
            .expect("Failed to create ACCOUNTS table");

        // Insert test accounts
        sqlx::query("INSERT INTO ACCOUNTS (USER_ID, ACCOUNT_CODE, ACCOUNT_NAME, TEMPLATE_CODE) VALUES (1, 'NONE', 'None', 'CASH')")
            .execute(&pool)
            .await
            .expect("Failed to insert test account");

        // Create TRANSACTIONS_HEADER table
        sqlx::query(sql_queries::CREATE_TRANSACTIONS_HEADER_TABLE)
            .execute(&pool)
            .await
            .expect("Failed to create TRANSACTIONS_HEADER table");

        // Insert test transaction header
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTION_HEADER)
        .execute(&pool)
        .await
        .expect("Failed to insert test transaction header");

        // Create old schema TRANSACTIONS_DETAIL table (without USER_ID and CATEGORY1_CODE)
        sqlx::query(sql_queries::TEST_CREATE_OLD_TRANSACTIONS_DETAIL_TABLE)
        .execute(&pool)
        .await
        .expect("Failed to create old TRANSACTIONS_DETAIL table");

        // Insert test data
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTION_DETAIL)
        .execute(&pool)
        .await
        .expect("Failed to insert test transaction detail");

        // Create Database instance and run migration
        let db = Database { pool };

        // Check that migration is needed
        let needs_migration = db.check_transactions_detail_needs_migration()
            .await
            .expect("Failed to check migration status");
        assert!(needs_migration, "Migration should be needed for old schema");

        // Run migration
        db.migrate_transactions_detail_table()
            .await
            .expect("Failed to migrate TRANSACTIONS_DETAIL table");

        // Verify migration completed successfully
        let needs_migration_after = db.check_transactions_detail_needs_migration()
            .await
            .expect("Failed to check migration status after migration");
        assert!(!needs_migration_after, "Migration should not be needed after migration");

        // Verify new schema has USER_ID column
        let has_user_id: i64 = sqlx::query_scalar(sql_queries::CHECK_TRANSACTIONS_DETAIL_HAS_USER_ID)
            .fetch_one(db.pool())
            .await
            .expect("Failed to check USER_ID column");
        assert_eq!(has_user_id, 1, "USER_ID column should exist after migration");

        // Verify data was migrated correctly
        let row: (i64, i64, i64, String, String, String, String, i64) = sqlx::query_as(
            sql_queries::TEST_SELECT_MIGRATED_TRANSACTION_DETAIL
        )
        .fetch_one(db.pool())
        .await
        .expect("Failed to fetch migrated data");

        assert_eq!(row.0, 1, "DETAIL_ID should be preserved");
        assert_eq!(row.1, 1, "TRANSACTION_ID should be preserved");
        assert_eq!(row.2, 1, "USER_ID should be populated from TRANSACTIONS_HEADER");
        assert_eq!(row.3, "INCOME", "CATEGORY1_CODE should be populated from TRANSACTIONS_HEADER");
        assert_eq!(row.4, "SALARY", "CATEGORY2_CODE should be preserved");
        assert_eq!(row.5, "MONTHLY", "CATEGORY3_CODE should be preserved");
        assert_eq!(row.6, "Test Item", "ITEM_NAME should be preserved");
        assert_eq!(row.7, 1000, "AMOUNT should be preserved");

        // Clean up
        drop(db);
        let _ = std::fs::remove_file(&test_db_path);
    }
}
