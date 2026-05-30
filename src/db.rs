use sqlx::sqlite::SqlitePool;
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME};
use crate::sql_queries;

// Schema is embedded at compile time. Reading via std::fs::read_to_string with a
// CWD-relative path silently works under `cargo tauri dev` (CWD = project root)
// but crashes the installed .msi/.exe at startup, because the installed app's
// CWD is the install directory and `res/sql/dbaccess.sql` is not there.
const INIT_SQL: &str = include_str!("../res/sql/dbaccess.sql");

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

        // SQLite ships with foreign_keys = OFF by default. Without this PRAGMA
        // every ON DELETE CASCADE / SET NULL we declared (RECURRING_RULES <→
        // RECURRING_RULE_DETAILS, TRANSACTIONS_HEADER <→ TRANSACTIONS_DETAIL,
        // TRANSACTIONS_HEADER.RULE_ID → RECURRING_RULES on new DBs, etc.) would
        // be silently ignored and we'd leak orphaned rows on every delete.
        sqlx::query("PRAGMA foreign_keys = ON")
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
        // Remove comment lines first
        let cleaned_sql: Vec<&str> = INIT_SQL
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

        // Add AMOUNT_INCLUDING_TAX column if it doesn't exist (for tables created before this column was added)
        self.ensure_amount_including_tax_column().await?;

        // Make CATEGORY2_CODE and CATEGORY3_CODE nullable if they have NOT NULL constraint
        self.ensure_category_nullable().await?;

        // Add IS_SCHEDULED column if it doesn't exist (for tables created before this column was added)
        self.ensure_is_scheduled_column().await?;

        // Add PRODUCT_ID column for v2.6.0 master integration
        self.ensure_product_id_column().await?;

        Ok(())
    }

    /// Add PRODUCT_ID to TRANSACTIONS_DETAIL if absent (v2.6.0 master integration).
    /// SQLite's ALTER TABLE ADD COLUMN cannot attach a FOREIGN KEY clause, so
    /// existing DBs end up without the FK declaration; new DBs created via
    /// MIGRATE_TRANSACTIONS_DETAIL_CREATE_NEW do carry it. Integrity is
    /// preserved at the application layer (search_products_by_name only returns
    /// the user's own products, and delete-product would need to clear refs).
    async fn ensure_product_id_column(&self) -> Result<(), sqlx::Error> {
        let has_column: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_table_info('TRANSACTIONS_DETAIL') WHERE name = 'PRODUCT_ID'"
        )
        .fetch_one(&self.pool)
        .await?;

        if has_column == 0 {
            sqlx::query("ALTER TABLE TRANSACTIONS_DETAIL ADD COLUMN PRODUCT_ID INTEGER")
                .execute(&self.pool)
                .await?;
        }

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_transactions_detail_product ON TRANSACTIONS_DETAIL(PRODUCT_ID)"
        )
        .execute(&self.pool)
        .await?;

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

    /// Make CATEGORY2_CODE and CATEGORY3_CODE nullable by recreating the table
    async fn ensure_category_nullable(&self) -> Result<(), sqlx::Error> {
        // Check if TRANSACTIONS_DETAIL table exists
        let table_exists: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='TRANSACTIONS_DETAIL'"
        )
        .fetch_one(&self.pool)
        .await?;

        if table_exists == 0 {
            return Ok(());
        }

        // Check if CATEGORY2_CODE has NOT NULL constraint
        let is_not_null: i64 = sqlx::query_scalar(sql_queries::CHECK_CATEGORY2_NOT_NULL)
            .fetch_one(&self.pool)
            .await?;

        if is_not_null == 0 {
            // Already nullable
            return Ok(());
        }

        // Recreate table with nullable CATEGORY2_CODE and CATEGORY3_CODE
        let mut tx = self.pool.begin().await?;

        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&mut *tx)
            .await?;

        sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_CREATE_NEW)
            .execute(&mut *tx)
            .await?;

        sqlx::query(sql_queries::MIGRATE_NULLABLE_CATEGORY_COPY_DATA)
            .execute(&mut *tx)
            .await?;

        sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_DROP_OLD)
            .execute(&mut *tx)
            .await?;

        sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_RENAME_NEW)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Ensure IS_SCHEDULED column exists in TRANSACTIONS_HEADER table
    async fn ensure_is_scheduled_column(&self) -> Result<(), sqlx::Error> {
        let has_column: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_table_info('TRANSACTIONS_HEADER') WHERE name = 'IS_SCHEDULED'"
        )
        .fetch_one(&self.pool)
        .await?;

        if has_column == 0 {
            sqlx::query("ALTER TABLE TRANSACTIONS_HEADER ADD COLUMN IS_SCHEDULED INTEGER DEFAULT 0")
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// Run migrations for v2.1.0 recurring scheduled transactions feature.
    /// - Adds RULE_ID to TRANSACTIONS_HEADER (group membership for occurrences
    ///   generated from a recurring rule; NULL means a one-off entry)
    /// - Drops the obsolete linked-list columns GROUP_HEAD/NEXT_TRANSACTION_ID
    ///   from TRANSACTIONS_HEADER and FIRST_TRANSACTION_ID from RECURRING_RULES
    ///   if a previous unreleased build of this branch added them
    /// - Adds HOLIDAY_LOCALE/WEEK_START_DAY to USERS
    /// - Creates RECURRING_RULES, RECURRING_RULE_DETAILS, HOLIDAYS_STANDARD,
    ///   HOLIDAYS_USER_CUSTOM tables
    /// - Seeds HOLIDAYS_STANDARD with Japanese statutory holidays generated by
    ///   the jpholiday crate for a sliding window around the current year
    pub async fn migrate_recurring(&self) -> Result<(), sqlx::Error> {
        self.ensure_header_rule_id_column().await?;
        self.ensure_users_recurring_columns().await?;
        self.create_recurring_tables().await?;
        self.drop_obsolete_linked_list_columns().await?;
        self.seed_japanese_holidays().await?;
        Ok(())
    }

    /// Populate HOLIDAYS_STANDARD with Japanese holidays for [today-5y, today+10y].
    /// Idempotent via INSERT OR IGNORE on the (LOCALE, HOLIDAY_DATE) UNIQUE
    /// index — running on every startup just patches in any newly-passing year.
    /// Replaces the hand-maintained 2026–2028 hard-coded list that earlier
    /// commits shipped in dbaccess.sql.
    async fn seed_japanese_holidays(&self) -> Result<(), sqlx::Error> {
        use chrono::{Datelike, Local};
        use jpholiday::jpholiday::JPHoliday;

        let jp = JPHoliday::new();
        let current_year = Local::now().year();
        let start_year = current_year - 5;
        let end_year = current_year + 10;

        for year in start_year..=end_year {
            for (date, name) in jp.year_holidays(year) {
                sqlx::query(
                    "INSERT OR IGNORE INTO HOLIDAYS_STANDARD \
                     (LOCALE, HOLIDAY_DATE, HOLIDAY_NAME) VALUES ('JP', ?, ?)"
                )
                .bind(date.format("%Y-%m-%d").to_string())
                .bind(name)
                .execute(&self.pool)
                .await?;
            }
        }
        Ok(())
    }

    /// Add RULE_ID to TRANSACTIONS_HEADER if absent. NULL = one-off entry.
    /// The matching index is created here too, because dbaccess.sql runs
    /// before the migration and would fail to reference RULE_ID.
    async fn ensure_header_rule_id_column(&self) -> Result<(), sqlx::Error> {
        let has_column: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_table_info('TRANSACTIONS_HEADER') WHERE name = 'RULE_ID'"
        )
        .fetch_one(&self.pool)
        .await?;

        if has_column == 0 {
            sqlx::query("ALTER TABLE TRANSACTIONS_HEADER ADD COLUMN RULE_ID INTEGER")
                .execute(&self.pool)
                .await?;
        }

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_transactions_header_rule ON TRANSACTIONS_HEADER(RULE_ID)"
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Drop GROUP_HEAD/NEXT_TRANSACTION_ID and RECURRING_RULES.FIRST_TRANSACTION_ID
    /// if a prior in-development build of dev-v2-recurring already added them.
    /// Released schemas never had these, so this no-ops for production users
    /// upgrading from v2.0.x. SQLite's DROP COLUMN was added in 3.35 (2021-03);
    /// our toolchain comfortably exceeds that.
    async fn drop_obsolete_linked_list_columns(&self) -> Result<(), sqlx::Error> {
        for (table, column) in [
            ("TRANSACTIONS_HEADER", "GROUP_HEAD"),
            ("TRANSACTIONS_HEADER", "NEXT_TRANSACTION_ID"),
            ("RECURRING_RULES",     "FIRST_TRANSACTION_ID"),
        ] {
            let has_column: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM pragma_table_info(?) WHERE name = ?"
            )
            .bind(table)
            .bind(column)
            .fetch_one(&self.pool)
            .await?;

            if has_column == 1 {
                let ddl = format!("ALTER TABLE {} DROP COLUMN {}", table, column);
                sqlx::query(&ddl).execute(&self.pool).await?;
            }
        }
        Ok(())
    }

    /// Add HOLIDAY_LOCALE, WEEK_START_DAY to USERS if absent.
    async fn ensure_users_recurring_columns(&self) -> Result<(), sqlx::Error> {
        for (name, ddl) in [
            ("HOLIDAY_LOCALE", "ALTER TABLE USERS ADD COLUMN HOLIDAY_LOCALE TEXT DEFAULT 'JP'"),
            ("WEEK_START_DAY", "ALTER TABLE USERS ADD COLUMN WEEK_START_DAY INTEGER DEFAULT 1"),
        ] {
            let has_column: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM pragma_table_info('USERS') WHERE name = ?"
            )
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

            if has_column == 0 {
                sqlx::query(ddl).execute(&self.pool).await?;
            }
        }
        Ok(())
    }

    /// Run migrations for v2.3.0 aggregation period customization.
    /// Adds MONTH_PERIOD_START_DAY / YEAR_PERIOD_START_MONTH / YEAR_PERIOD_START_DAY
    /// to USERS. Defaults preserve the previous calendar-month / calendar-year behavior
    /// for existing users, so no backfill is required.
    pub async fn migrate_period_customization(&self) -> Result<(), sqlx::Error> {
        self.ensure_users_period_columns().await?;
        Ok(())
    }

    /// Add MONTH_PERIOD_START_DAY, YEAR_PERIOD_START_MONTH, YEAR_PERIOD_START_DAY to USERS.
    async fn ensure_users_period_columns(&self) -> Result<(), sqlx::Error> {
        for (name, ddl) in [
            ("MONTH_PERIOD_START_DAY",  "ALTER TABLE USERS ADD COLUMN MONTH_PERIOD_START_DAY INTEGER DEFAULT 1"),
            ("YEAR_PERIOD_START_MONTH", "ALTER TABLE USERS ADD COLUMN YEAR_PERIOD_START_MONTH INTEGER DEFAULT 1"),
            ("YEAR_PERIOD_START_DAY",   "ALTER TABLE USERS ADD COLUMN YEAR_PERIOD_START_DAY INTEGER DEFAULT 1"),
        ] {
            let has_column: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM pragma_table_info('USERS') WHERE name = ?"
            )
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

            if has_column == 0 {
                sqlx::query(ddl).execute(&self.pool).await?;
            }
        }
        Ok(())
    }

    /// Run migrations for v2.4.0 monthly period start day holiday shift.
    /// Adds MONTH_PERIOD_HOLIDAY_SHIFT to USERS (0=None / 1=Prev / 2=Next).
    /// Default 0 preserves v2.3.0 calendar-date-fixed behavior for existing users.
    /// Yearly period start is intentionally not shifted (fiscal-year semantics).
    pub async fn migrate_period_holiday_shift(&self) -> Result<(), sqlx::Error> {
        self.ensure_users_period_holiday_shift_column().await?;
        Ok(())
    }

    /// Add MONTH_PERIOD_HOLIDAY_SHIFT to USERS if absent.
    async fn ensure_users_period_holiday_shift_column(&self) -> Result<(), sqlx::Error> {
        let has_column: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_table_info('USERS') WHERE name = 'MONTH_PERIOD_HOLIDAY_SHIFT'"
        )
        .fetch_one(&self.pool)
        .await?;

        if has_column == 0 {
            sqlx::query("ALTER TABLE USERS ADD COLUMN MONTH_PERIOD_HOLIDAY_SHIFT INTEGER DEFAULT 0")
                .execute(&self.pool)
                .await?;
        }
        Ok(())
    }

    /// Create new tables for v2.1.0 (idempotent via IF NOT EXISTS).
    async fn create_recurring_tables(&self) -> Result<(), sqlx::Error> {
        sqlx::query(sql_queries::CREATE_RECURRING_RULES_TABLE)
            .execute(&self.pool)
            .await?;
        sqlx::query(sql_queries::CREATE_RECURRING_RULE_DETAILS_TABLE)
            .execute(&self.pool)
            .await?;
        sqlx::query(sql_queries::CREATE_HOLIDAYS_STANDARD_TABLE)
            .execute(&self.pool)
            .await?;
        sqlx::query(sql_queries::CREATE_HOLIDAYS_USER_CUSTOM_TABLE)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Ensure AMOUNT_INCLUDING_TAX column exists in TRANSACTIONS_DETAIL table
    async fn ensure_amount_including_tax_column(&self) -> Result<(), sqlx::Error> {
        let has_column: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_table_info('TRANSACTIONS_DETAIL') WHERE name = 'AMOUNT_INCLUDING_TAX'"
        )
        .fetch_one(&self.pool)
        .await?;

        if has_column == 0 {
            sqlx::query("ALTER TABLE TRANSACTIONS_DETAIL ADD COLUMN AMOUNT_INCLUDING_TAX INTEGER")
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }
}

pub fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(home)
        .join(DB_DIR_NAME)
        .join(DB_FILE_NAME)
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

        // Create MANUFACTURERS + PRODUCTS so the v2.6.0 migration target
        // schema (which carries an FK PRODUCT_ID -> PRODUCTS) can resolve its
        // FOREIGN KEY clause when CREATE TABLE runs. SQLite still validates
        // the referenced-table identifier at CREATE time on modern versions.
        sqlx::query(sql_queries::TEST_MANUFACTURER_CREATE_TABLE)
            .execute(&pool)
            .await
            .expect("Failed to create MANUFACTURERS table");
        sqlx::query(sql_queries::TEST_PRODUCT_CREATE_TABLE)
            .execute(&pool)
            .await
            .expect("Failed to create PRODUCTS table");

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

    #[tokio::test]
    async fn test_ensure_product_id_column_idempotent() {
        // Adds PRODUCT_ID + index on first run; subsequent runs must no-op
        // without erroring out. Models the "user upgrades to v2.6.0, then
        // reopens the app another day" path where the column already exists.
        let temp_dir = std::env::temp_dir();
        let test_db_name = format!("test_ensure_product_id_{}.db", std::process::id());
        let test_db_path = temp_dir.join(&test_db_name);
        let _ = std::fs::remove_file(&test_db_path);

        let db_url = format!("sqlite://{}?mode=rwc", test_db_path.display());
        let pool = connect_db(&db_url).await.expect("Failed to connect");

        // Pre-v2.6.0 detail schema (no PRODUCT_ID column)
        sqlx::query(
            "CREATE TABLE TRANSACTIONS_DETAIL ( \
                DETAIL_ID INTEGER PRIMARY KEY AUTOINCREMENT, \
                TRANSACTION_ID INTEGER NOT NULL, \
                USER_ID INTEGER NOT NULL, \
                CATEGORY1_CODE VARCHAR(50) NOT NULL, \
                ITEM_NAME TEXT NOT NULL, \
                AMOUNT INTEGER NOT NULL \
            )"
        )
        .execute(&pool)
        .await
        .expect("Failed to create legacy detail table");

        let db = Database { pool };

        // First call: should ALTER + create index
        db.ensure_product_id_column().await.expect("first call");
        // Second call: should no-op (column + index already present)
        db.ensure_product_id_column().await.expect("second call");

        // Verify PRODUCT_ID column now exists
        let has_column: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_table_info('TRANSACTIONS_DETAIL') WHERE name = 'PRODUCT_ID'"
        )
        .fetch_one(db.pool())
        .await
        .expect("Failed to query pragma");
        assert_eq!(has_column, 1, "PRODUCT_ID column should exist after migration");

        // Verify the index landed
        let has_index: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name='idx_transactions_detail_product'"
        )
        .fetch_one(db.pool())
        .await
        .expect("Failed to query indexes");
        assert_eq!(has_index, 1, "idx_transactions_detail_product should exist");

        drop(db);
        let _ = std::fs::remove_file(&test_db_path);
    }
}
