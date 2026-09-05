use sqlx::sqlite::SqlitePool;
use sqlx::Acquire; // for `PoolConnection::begin`
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
    
    /// Run every statement in `res/sql/dbaccess.sql` in a single
    /// transaction. Fable-5 review #30: the previous shape autocommitted
    /// each of the ~500 CREATE / INSERT / CREATE INDEX statements
    /// separately, which meant one fsync per statement on cold startup.
    /// Batching into one transaction turns ~500 fsyncs into 1 — a
    /// noticeable startup improvement on the desktop app, most visible
    /// on the first launch after a version upgrade when the whole i18n
    /// resource pack is being seeded. Every statement in dbaccess.sql
    /// is transaction-safe (CREATE TABLE, CREATE INDEX, and INSERT are
    /// all fine inside a tx — no PRAGMA / VACUUM / ATTACH lives there),
    /// and `INSERT OR IGNORE` keeps re-runs idempotent so a mid-tx
    /// failure on a subsequent boot still recovers cleanly.
    pub async fn initialize(&self) -> Result<(), sqlx::Error> {
        // Remove comment lines first
        let cleaned_sql: Vec<&str> = INIT_SQL
            .lines()
            .filter(|line| !line.trim().starts_with("--") && !line.trim().is_empty())
            .collect();
        let sql_without_comments = cleaned_sql.join("\n");

        let mut tx = self.pool.begin().await?;
        for statement in sql_without_comments.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed).execute(&mut *tx).await?;
            }
        }
        tx.commit().await?;

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

    /// Migrate TRANSACTIONS_DETAIL table from old schema to new schema.
    ///
    /// `PRAGMA foreign_keys` is a connection-local setting in SQLite, and
    /// its value cannot be changed inside a transaction — the engine
    /// silently ignores the write mid-transaction. The previous shape
    /// (begin → `PRAGMA foreign_keys = OFF` on the tx → COPY → commit)
    /// therefore ran COPY_DATA with FK still ON, so a DB carrying any
    /// orphaned rows from earlier builds would fail with
    /// `FOREIGN KEY constraint failed` and the app would refuse to
    /// start. Additionally, running the final `PRAGMA ... = ON` against
    /// `self.pool` grabbed *some* pooled connection — not necessarily
    /// the one that had FK turned off — so the state fix was also
    /// unreliable.
    ///
    /// Fix (Fable-5 review #11): pin one connection with `pool.acquire`,
    /// flip the PRAGMA on that connection *outside* any transaction,
    /// run the migration in a transaction on the same connection,
    /// commit, then restore the PRAGMA on the same connection before it
    /// returns to the pool. On error, best-effort restore FK to avoid
    /// leaking a FK-off connection back into circulation.
    async fn migrate_transactions_detail_table(&self) -> Result<(), sqlx::Error> {
        let mut conn = self.pool.acquire().await?;

        // PRAGMA outside any transaction, on the same connection that
        // will run the migration. Any transaction bounded by BEGIN/COMMIT
        // opened after this point inherits FK = OFF.
        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&mut *conn)
            .await?;

        let migration = async {
            let mut tx = conn.begin().await?;
            sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_CREATE_NEW)
                .execute(&mut *tx)
                .await?;
            sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_COPY_DATA)
                .execute(&mut *tx)
                .await?;
            sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_DROP_OLD)
                .execute(&mut *tx)
                .await?;
            sqlx::query(sql_queries::MIGRATE_TRANSACTIONS_DETAIL_RENAME_NEW)
                .execute(&mut *tx)
                .await?;
            tx.commit().await
        }
        .await;

        // Restore FK on the SAME connection, regardless of migration
        // outcome — otherwise a mid-migration failure would return a
        // FK-off connection to the pool and every later borrower would
        // silently skip cascade deletes.
        let restore = sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&mut *conn)
            .await;

        migration?;
        restore?;
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

        // Recreate table with nullable CATEGORY2_CODE and CATEGORY3_CODE.
        // Same PRAGMA-outside-tx / same-connection dance as
        // `migrate_transactions_detail_table` above — see that function
        // for the full rationale.
        let mut conn = self.pool.acquire().await?;

        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&mut *conn)
            .await?;

        let migration = async {
            let mut tx = conn.begin().await?;
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
            tx.commit().await
        }
        .await;

        let restore = sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&mut *conn)
            .await;

        migration?;
        restore?;
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

    /// Fable-5 review #15 — per-user random salt for Argon2 key derivation.
    ///
    /// Adds `ENCRYPTION_SALT BLOB` to USERS if the column is absent, then
    /// backfills every legacy row that has `ENCRYPTION_SALT IS NULL` with
    /// a fresh cryptographically-random 16-byte salt. Safe on live DBs
    /// because `ENCRYPTED_FIELDS` is unseeded in production and no
    /// frontend path invokes `encrypt_field` / `decrypt_field` /
    /// `re_encrypt_user_data`, so there is no existing ciphertext whose
    /// old (predictable) salt we would have to preserve.
    pub async fn migrate_encryption_salt(&self) -> Result<(), sqlx::Error> {
        // Add the column (idempotent).
        let has_column: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_table_info('USERS') WHERE name = 'ENCRYPTION_SALT'"
        )
        .fetch_one(&self.pool)
        .await?;

        if has_column == 0 {
            sqlx::query("ALTER TABLE USERS ADD COLUMN ENCRYPTION_SALT BLOB")
                .execute(&self.pool)
                .await?;
        }

        // Backfill any row whose salt is still NULL (new column, or a row
        // that skipped the register path somehow). One UPDATE per row so
        // every user ends up with a *different* salt — the whole point of
        // the fix is that attackers can't share a rainbow table across
        // users.
        let user_ids: Vec<i64> = sqlx::query_scalar(
            "SELECT USER_ID FROM USERS WHERE ENCRYPTION_SALT IS NULL"
        )
        .fetch_all(&self.pool)
        .await?;

        for user_id in user_ids {
            let salt = crate::security::generate_encryption_salt();
            sqlx::query("UPDATE USERS SET ENCRYPTION_SALT = ? WHERE USER_ID = ?")
                .bind(salt.as_slice())
                .bind(user_id)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    /// PR15 (Fable-5 #20): make SHOPS.SHOP_NAME unique per user.
    ///
    /// Fresh DBs pick up `UNIQUE(USER_ID, SHOP_NAME)` from the CREATE
    /// TABLE clause in dbaccess.sql, which auto-creates the underlying
    /// index. Existing DBs may already carry duplicate rows because the
    /// old code path only ran a race-vulnerable SELECT-then-INSERT
    /// dedup check — this migration walks each duplicate group, moves
    /// live references (TRANSACTIONS_HEADER.SHOP_ID,
    /// RECURRING_RULES.SHOP_ID) onto the surviving smallest SHOP_ID,
    /// deletes the losers, then creates a manually-named UNIQUE index
    /// so the constraint is enforced going forward.
    ///
    /// Everything after the initial no-op probe runs inside a single
    /// transaction so a mid-migration failure rolls back. The migration
    /// is idempotent: re-running after success is a no-op because the
    /// probe finds the index and returns early.
    pub async fn migrate_shops_unique(&self) -> Result<(), sqlx::Error> {
        let has_unique: i64 = sqlx::query_scalar(sql_queries::SHOPS_HAS_UNIQUE_USER_NAME_INDEX)
            .fetch_one(&self.pool)
            .await?;
        if has_unique > 0 {
            // Fresh installs (auto-index from the inline UNIQUE) and DBs
            // that already went through this migration both take this
            // branch.
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        // 1. Move any transaction / recurring-rule reference off the
        //    doomed duplicate rows and onto the surviving smallest id.
        sqlx::query(sql_queries::MIGRATE_SHOPS_UNIQUE_REPOINT_HEADER)
            .execute(&mut *tx)
            .await?;
        sqlx::query(sql_queries::MIGRATE_SHOPS_UNIQUE_REPOINT_RECURRING)
            .execute(&mut *tx)
            .await?;

        // 2. Drop the duplicate SHOPS rows.
        sqlx::query(sql_queries::MIGRATE_SHOPS_UNIQUE_DELETE_DUPLICATES)
            .execute(&mut *tx)
            .await?;

        // 3. Create the unique index that will refuse future duplicates.
        sqlx::query(sql_queries::MIGRATE_SHOPS_UNIQUE_CREATE_INDEX)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    /// Fable-5 review #11 — SHOPS was missing `ON DELETE CASCADE` on
    /// its `USER_ID` FK, so deleting a user with SHOPS rows failed
    /// with `FOREIGN KEY constraint failed` and rolled the whole
    /// DELETE back. SQLite has no `ALTER TABLE ADD FOREIGN KEY`, so
    /// we recreate the table with the CASCADE clause and copy every
    /// row across.
    ///
    /// Idempotent — a probe against `pragma_foreign_key_list('SHOPS')`
    /// returns early when the CASCADE FK is already present (fresh
    /// installs and DBs that already ran this migration).
    ///
    /// FK enforcement is disabled for the duration of the recreate
    /// because DROP TABLE on a table that other tables reference
    /// (`TRANSACTIONS_HEADER.SHOP_ID`, `RECURRING_RULES.SHOP_ID`)
    /// would otherwise trigger the parent-check. We flip it back on
    /// and run `PRAGMA foreign_key_check` before returning so a
    /// dangling reference from a botched copy is caught here rather
    /// than at the next query. The connection is acquired once and
    /// used for every step so the `PRAGMA foreign_keys` toggle stays
    /// within scope (it is per-connection).
    pub async fn migrate_shops_user_id_cascade(&self) -> Result<(), sqlx::Error> {
        use sqlx::{Acquire, Executor, Row};

        let has_cascade: i64 = sqlx::query_scalar(sql_queries::SHOPS_HAS_USER_CASCADE_FK)
            .fetch_one(&self.pool)
            .await?;
        if has_cascade > 0 {
            return Ok(());
        }

        let mut conn = self.pool.acquire().await?;

        // Turn FKs off on THIS connection so the recreate doesn't trip
        // the parent-check when we DROP the old SHOPS table.
        conn.execute(sql_queries::PRAGMA_FOREIGN_KEYS_OFF).await?;

        let mut tx = conn.begin().await?;
        tx.execute(sql_queries::MIGRATE_SHOPS_CASCADE_CREATE_NEW_TABLE).await?;
        tx.execute(sql_queries::MIGRATE_SHOPS_CASCADE_COPY_ROWS).await?;
        tx.execute(sql_queries::MIGRATE_SHOPS_CASCADE_DROP_OLD_TABLE).await?;
        tx.execute(sql_queries::MIGRATE_SHOPS_CASCADE_RENAME_TABLE).await?;
        tx.execute(sql_queries::MIGRATE_SHOPS_CASCADE_CREATE_USER_ORDER_INDEX).await?;
        // Re-create the manually-named unique index too, in case any
        // caller looked it up by name. `IF NOT EXISTS` is safe because
        // the inline `UNIQUE` on the new table already gave us the
        // constraint via an auto-index; this is a friendlier alias.
        tx.execute(sql_queries::MIGRATE_SHOPS_UNIQUE_CREATE_INDEX).await?;
        tx.commit().await?;

        conn.execute(sql_queries::PRAGMA_FOREIGN_KEYS_ON).await?;

        // Belt and braces — if the copy dropped a reference on the
        // floor, catch it here (returned as one row per violation).
        let violations = sqlx::query(sql_queries::PRAGMA_FOREIGN_KEY_CHECK)
            .fetch_all(&mut *conn)
            .await?;
        if !violations.is_empty() {
            let sample: String = violations
                .first()
                .and_then(|row| row.try_get::<String, _>(0).ok())
                .unwrap_or_else(|| "unknown table".to_string());
            return Err(sqlx::Error::Protocol(format!(
                "SHOPS CASCADE migration left {} foreign-key violations (first: {})",
                violations.len(),
                sample,
            )));
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

    /// In-memory database, no schema. Migrations only touch `self.pool`, so
    /// `sqlite::memory:` is enough and keeps the tests file-system free.
    async fn memory_db() -> Database {
        let pool = connect_db(crate::test_helpers::database::TEST_DB_URL)
            .await
            .expect("Failed to connect to in-memory database");
        Database { pool }
    }

    async fn column_count(db: &Database, table: &str, column: &str) -> i64 {
        sqlx::query_scalar(sql_queries::TEST_DB_COUNT_TABLE_COLUMN)
            .bind(table)
            .bind(column)
            .fetch_one(db.pool())
            .await
            .expect("Failed to query pragma_table_info")
    }

    async fn table_count(db: &Database, table: &str) -> i64 {
        sqlx::query_scalar(sql_queries::TEST_DB_COUNT_TABLE)
            .bind(table)
            .fetch_one(db.pool())
            .await
            .expect("Failed to query sqlite_master")
    }

    #[tokio::test]
    async fn test_initialize_creates_core_schema() {
        let db = memory_db().await;

        db.initialize().await.expect("initialize should succeed");

        for table in [
            "USERS",
            "CATEGORY1",
            "CATEGORY2",
            "CATEGORY3",
            "ACCOUNTS",
            "ACCOUNT_TEMPLATES",
            "TRANSACTIONS_HEADER",
            "TRANSACTIONS_DETAIL",
            "RECURRING_RULES",
            "HOLIDAYS_STANDARD",
        ] {
            assert_eq!(table_count(&db, table).await, 1, "{} should be created", table);
        }

        // Re-running on an existing database must stay a no-op (every statement
        // in dbaccess.sql is IF NOT EXISTS / INSERT OR IGNORE).
        db.initialize().await.expect("initialize should be idempotent");
    }

    #[tokio::test]
    async fn test_migrate_transactions_creates_current_schema() {
        let db = memory_db().await;
        db.initialize().await.expect("initialize");

        db.migrate_transactions().await.expect("first run");

        assert_eq!(table_count(&db, "MEMOS").await, 1);
        for column in ["AMOUNT_INCLUDING_TAX", "PRODUCT_ID"] {
            assert_eq!(
                column_count(&db, "TRANSACTIONS_DETAIL", column).await,
                1,
                "TRANSACTIONS_DETAIL.{} should exist",
                column
            );
        }
        assert_eq!(
            column_count(&db, "TRANSACTIONS_HEADER", "IS_SCHEDULED").await,
            1
        );

        db.migrate_transactions().await.expect("second run");
    }

    #[tokio::test]
    async fn test_check_transactions_detail_needs_migration_transitions() {
        // No TRANSACTIONS_DETAIL table at all
        let db = memory_db().await;
        assert!(!db
            .check_transactions_detail_needs_migration()
            .await
            .expect("missing table"));

        // dbaccess.sql still ships the pre-USER_ID detail schema, so a freshly
        // initialized database needs the migration...
        db.initialize().await.expect("initialize");
        assert!(db
            .check_transactions_detail_needs_migration()
            .await
            .expect("fresh schema"));

        // ...and stops needing it once migrate_transactions has run.
        db.migrate_transactions().await.expect("migrate");
        assert!(!db
            .check_transactions_detail_needs_migration()
            .await
            .expect("migrated schema"));
    }

    #[tokio::test]
    async fn test_migrate_recurring_upgrades_legacy_schema() {
        let db = memory_db().await;
        sqlx::query(sql_queries::TEST_DB_CREATE_LEGACY_USERS_TABLE)
            .execute(db.pool())
            .await
            .expect("legacy USERS");
        sqlx::query(sql_queries::TEST_DB_CREATE_LEGACY_HEADER_TABLE)
            .execute(db.pool())
            .await
            .expect("legacy TRANSACTIONS_HEADER");

        db.migrate_recurring().await.expect("first run");

        assert_eq!(column_count(&db, "TRANSACTIONS_HEADER", "RULE_ID").await, 1);
        for column in ["HOLIDAY_LOCALE", "WEEK_START_DAY"] {
            assert_eq!(column_count(&db, "USERS", column).await, 1, "USERS.{}", column);
        }
        for table in [
            "RECURRING_RULES",
            "RECURRING_RULE_DETAILS",
            "HOLIDAYS_STANDARD",
            "HOLIDAYS_USER_CUSTOM",
        ] {
            assert_eq!(table_count(&db, table).await, 1, "{} should be created", table);
        }

        // The obsolete linked-list columns from the unreleased dev build are gone
        assert_eq!(column_count(&db, "TRANSACTIONS_HEADER", "GROUP_HEAD").await, 0);
        assert_eq!(
            column_count(&db, "TRANSACTIONS_HEADER", "NEXT_TRANSACTION_ID").await,
            0
        );

        let seeded: i64 = sqlx::query_scalar(sql_queries::TEST_DB_COUNT_STANDARD_HOLIDAYS)
            .fetch_one(db.pool())
            .await
            .expect("count seeded holidays");
        assert!(seeded > 0, "Japanese holidays should be seeded");

        db.migrate_recurring().await.expect("second run");

        let seeded_after: i64 = sqlx::query_scalar(sql_queries::TEST_DB_COUNT_STANDARD_HOLIDAYS)
            .fetch_one(db.pool())
            .await
            .expect("count seeded holidays after rerun");
        assert_eq!(
            seeded, seeded_after,
            "re-seeding must not duplicate holidays"
        );
    }

    #[tokio::test]
    async fn test_migrate_period_customization_adds_columns_idempotently() {
        let db = memory_db().await;
        sqlx::query(sql_queries::TEST_DB_CREATE_LEGACY_USERS_TABLE)
            .execute(db.pool())
            .await
            .expect("legacy USERS");

        db.migrate_period_customization().await.expect("first run");
        db.migrate_period_customization().await.expect("second run");

        for column in [
            "MONTH_PERIOD_START_DAY",
            "YEAR_PERIOD_START_MONTH",
            "YEAR_PERIOD_START_DAY",
        ] {
            assert_eq!(column_count(&db, "USERS", column).await, 1, "USERS.{}", column);
        }
    }

    #[tokio::test]
    async fn test_migrate_period_holiday_shift_adds_column_idempotently() {
        let db = memory_db().await;
        sqlx::query(sql_queries::TEST_DB_CREATE_LEGACY_USERS_TABLE)
            .execute(db.pool())
            .await
            .expect("legacy USERS");

        db.migrate_period_holiday_shift().await.expect("first run");
        db.migrate_period_holiday_shift().await.expect("second run");

        assert_eq!(
            column_count(&db, "USERS", "MONTH_PERIOD_HOLIDAY_SHIFT").await,
            1
        );
    }

    #[test]
    fn test_get_db_path_points_at_app_directory() {
        let path = get_db_path();

        assert_eq!(
            path.file_name().and_then(|n| n.to_str()),
            Some(DB_FILE_NAME)
        );
        assert_eq!(
            path.parent().and_then(|p| p.file_name()).and_then(|n| n.to_str()),
            Some(DB_DIR_NAME)
        );
    }

    /// Fable-5 review #11 — reproduce the "orphaned MEMO_ID crashes the
    /// upgrade" case Fable described. Legacy DBs from earlier builds
    /// occasionally carry TRANSACTIONS_DETAIL rows whose MEMO_ID points
    /// at a memo row that no longer exists. The new schema declares
    /// `FOREIGN KEY (MEMO_ID) REFERENCES MEMOS(MEMO_ID)`, so with FK
    /// enforcement ON the COPY_DATA step would fail with
    /// `FOREIGN KEY constraint failed` and the app would refuse to
    /// start.
    ///
    /// The pre-fix `migrate_transactions_detail_table` tried to disable
    /// FK enforcement with `PRAGMA foreign_keys = OFF` executed *inside*
    /// the transaction — SQLite silently ignores mid-tx PRAGMA writes,
    /// so FK stayed ON and the migration blew up. The fix pins one
    /// connection with `pool.acquire`, flips PRAGMA outside the tx on
    /// that same connection, and only then begins the tx.
    #[tokio::test]
    async fn test_migrate_survives_orphaned_memo_reference() {
        let temp_dir = std::env::temp_dir();
        let test_db_name = format!("test_migrate_orphan_memo_{}.db", std::process::id());
        let test_db_path = temp_dir.join(&test_db_name);
        let _ = std::fs::remove_file(&test_db_path);

        let db_url = format!("sqlite://{}?mode=rwc", test_db_path.display());
        let pool = connect_db(&db_url).await.expect("connect");

        // FK ON — this is the production default, and also the state the
        // migration path must cope with (pre-fix code assumed it could
        // flip it off mid-tx, which is a no-op).
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("enable fk");

        // Seed the minimum schema the migration touches: users,
        // categories, memos, accounts, transactions_header, plus the
        // MANUFACTURERS/PRODUCTS tables referenced by the new schema's
        // FOREIGN KEY clauses.
        sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_INSERT_USER_ADMIN).execute(&pool).await.unwrap();

        sqlx::query(sql_queries::TEST_CREATE_CATEGORY1_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_INSERT_CATEGORY1).execute(&pool).await.unwrap();

        sqlx::query(sql_queries::CREATE_MEMOS_TABLE).execute(&pool).await.unwrap();

        sqlx::query(sql_queries::TEST_ACCOUNT_CREATE_TEMPLATES_TABLE).execute(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO ACCOUNT_TEMPLATES (TEMPLATE_CODE, TEMPLATE_NAME_JA, TEMPLATE_NAME_EN, DISPLAY_ORDER) \
             VALUES ('CASH', '現金', 'Cash', 1)"
        ).execute(&pool).await.unwrap();

        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_ACCOUNTS_TABLE).execute(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO ACCOUNTS (USER_ID, ACCOUNT_CODE, ACCOUNT_NAME, TEMPLATE_CODE) \
             VALUES (1, 'NONE', 'None', 'CASH')"
        ).execute(&pool).await.unwrap();

        sqlx::query(sql_queries::CREATE_TRANSACTIONS_HEADER_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTION_HEADER).execute(&pool).await.unwrap();

        sqlx::query(sql_queries::TEST_MANUFACTURER_CREATE_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_PRODUCT_CREATE_TABLE).execute(&pool).await.unwrap();

        sqlx::query(sql_queries::TEST_CREATE_OLD_TRANSACTIONS_DETAIL_TABLE)
            .execute(&pool).await.unwrap();

        // The core provocation: a detail row whose MEMO_ID does NOT
        // resolve to any memo. Old schema declares the FK but with
        // PRAGMA foreign_keys = OFF the row is insertable; production
        // DBs from earlier builds can contain rows like this.
        //
        // Pin one connection for the PRAGMA-off / INSERT / PRAGMA-on
        // sequence so all three land on the same connection. Running
        // them via the pool would distribute across connections and
        // the INSERT would arrive on a fresh (FK-on-by-default) one.
        {
            let mut seed_conn = pool.acquire().await.unwrap();
            sqlx::query("PRAGMA foreign_keys = OFF")
                .execute(&mut *seed_conn).await.unwrap();
            sqlx::query(
                "INSERT INTO TRANSACTIONS_DETAIL \
                 (DETAIL_ID, TRANSACTION_ID, CATEGORY2_CODE, CATEGORY3_CODE, ITEM_NAME, AMOUNT, MEMO_ID) \
                 VALUES (1, 1, 'SALARY', 'MONTHLY', 'Test Item', 1000, 9999)"
            ).execute(&mut *seed_conn).await.unwrap();
            sqlx::query("PRAGMA foreign_keys = ON")
                .execute(&mut *seed_conn).await.unwrap();
        }

        let db = Database { pool };

        // The migration must succeed even though the seeded detail's
        // MEMO_ID references a nonexistent memo.
        db.migrate_transactions_detail_table()
            .await
            .expect("migration must survive orphaned MEMO_ID references");

        // Post-check: the row landed in the new table with its dangling
        // MEMO_ID intact (the app is expected to null it out at read
        // time or ignore missing memos — the migration itself is not
        // in the data-cleanup business).
        let memo_id: Option<i64> = sqlx::query_scalar(
            "SELECT MEMO_ID FROM TRANSACTIONS_DETAIL WHERE DETAIL_ID = 1",
        )
        .fetch_one(db.pool())
        .await
        .expect("select migrated row");
        assert_eq!(memo_id, Some(9999));

        drop(db);
        let _ = std::fs::remove_file(&test_db_path);
    }

    /// Fable-5 review #11 — the pre-fix code called `PRAGMA
    /// foreign_keys = ON` on `self.pool` after commit, which grabbed
    /// whatever connection the pool handed out (not necessarily the one
    /// that ran the PRAGMA OFF). With the fix, the OFF/ON pair runs on
    /// the same acquired connection, and by the time the connection
    /// returns to the pool FK enforcement is ON again. Test that any
    /// pool connection returns `1` for `PRAGMA foreign_keys` after
    /// `migrate_transactions_detail_table` finishes.
    #[tokio::test]
    async fn test_migrate_leaves_foreign_keys_on() {
        let temp_dir = std::env::temp_dir();
        let test_db_name = format!("test_migrate_fk_on_{}.db", std::process::id());
        let test_db_path = temp_dir.join(&test_db_name);
        let _ = std::fs::remove_file(&test_db_path);

        let db_url = format!("sqlite://{}?mode=rwc", test_db_path.display());
        let pool = connect_db(&db_url).await.expect("connect");

        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("enable fk");

        sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_INSERT_USER_ADMIN).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_CREATE_CATEGORY1_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_INSERT_CATEGORY1).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::CREATE_MEMOS_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_ACCOUNT_CREATE_TEMPLATES_TABLE).execute(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO ACCOUNT_TEMPLATES (TEMPLATE_CODE, TEMPLATE_NAME_JA, TEMPLATE_NAME_EN, DISPLAY_ORDER) \
             VALUES ('CASH', '現金', 'Cash', 1)"
        ).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_ACCOUNTS_TABLE).execute(&pool).await.unwrap();
        sqlx::query(
            "INSERT INTO ACCOUNTS (USER_ID, ACCOUNT_CODE, ACCOUNT_NAME, TEMPLATE_CODE) \
             VALUES (1, 'NONE', 'None', 'CASH')"
        ).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::CREATE_TRANSACTIONS_HEADER_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTION_HEADER).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_MANUFACTURER_CREATE_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_PRODUCT_CREATE_TABLE).execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_CREATE_OLD_TRANSACTIONS_DETAIL_TABLE)
            .execute(&pool).await.unwrap();
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTION_DETAIL)
            .execute(&pool).await.unwrap();

        let db = Database { pool };

        db.migrate_transactions_detail_table()
            .await
            .expect("migration");

        // Sample many pool connections to catch a "some connection has
        // FK OFF" regression. The pool default is 10 connections in
        // sqlx-sqlite; sampling 20 exercises each at least once.
        for i in 0..20 {
            let fk: i64 = sqlx::query_scalar("PRAGMA foreign_keys")
                .fetch_one(db.pool())
                .await
                .expect("read pragma");
            assert_eq!(
                fk, 1,
                "iteration {}: connection returned to pool must have FK enforcement ON",
                i,
            );
        }

        drop(db);
        let _ = std::fs::remove_file(&test_db_path);
    }

    // ---- PR15 / Fable-5 #20 — migrate_shops_unique tests ----
    //
    // Fresh installs pick up the inline `UNIQUE(USER_ID, SHOP_NAME)`
    // constraint from dbaccess.sql and its auto-index; existing DBs
    // reach here without either. These tests spin up the pre-#20
    // schema (SHOPS with no UNIQUE + the two tables that carry live
    // SHOP_ID references), insert duplicate + referenced rows, and
    // confirm the migration dedupes, repoints, and pins the constraint.

    /// SHOPS DDL as it shipped BEFORE the PR15 inline UNIQUE — no
    /// constraint, so the migration is the one that adds it.
    const LEGACY_SHOPS_DDL: &str = r#"
        CREATE TABLE SHOPS (
            SHOP_ID INTEGER PRIMARY KEY AUTOINCREMENT,
            USER_ID INTEGER NOT NULL,
            SHOP_NAME TEXT NOT NULL,
            MEMO TEXT,
            DISPLAY_ORDER INTEGER NOT NULL DEFAULT 0,
            IS_DISABLED INTEGER DEFAULT 0,
            ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
            UPDATE_DT DATETIME
        )
    "#;

    async fn setup_legacy_shops_db() -> Database {
        let db = memory_db().await;
        sqlx::query(LEGACY_SHOPS_DDL)
            .execute(db.pool())
            .await
            .expect("legacy SHOPS");
        // Minimal TRANSACTIONS_HEADER + RECURRING_RULES with just the
        // columns the migration touches — the full schemas are noisier
        // than the migration cares about.
        sqlx::query(
            "CREATE TABLE TRANSACTIONS_HEADER (TRANSACTION_ID INTEGER PRIMARY KEY, SHOP_ID INTEGER)",
        )
        .execute(db.pool())
        .await
        .expect("legacy TRANSACTIONS_HEADER");
        sqlx::query(
            "CREATE TABLE RECURRING_RULES (RULE_ID INTEGER PRIMARY KEY, SHOP_ID INTEGER)",
        )
        .execute(db.pool())
        .await
        .expect("legacy RECURRING_RULES");
        db
    }

    async fn shop_ids(db: &Database) -> Vec<i64> {
        sqlx::query_scalar::<_, i64>("SELECT SHOP_ID FROM SHOPS ORDER BY SHOP_ID")
            .fetch_all(db.pool())
            .await
            .expect("SHOPS scan")
    }

    async fn unique_index_present(db: &Database) -> bool {
        let count: i64 = sqlx::query_scalar(sql_queries::SHOPS_HAS_UNIQUE_USER_NAME_INDEX)
            .fetch_one(db.pool())
            .await
            .expect("index probe");
        count > 0
    }

    #[tokio::test]
    async fn migrate_shops_unique_dedupes_and_repoints_references() {
        let db = setup_legacy_shops_db().await;

        // 3 SHOPS: (1, "AEON") and (1, "AEON") — duplicate pair; plus (1, "LAWSON").
        // SHOP_IDs 1, 2, 3 respectively.
        sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (1, 'AEON')")
            .execute(db.pool()).await.unwrap();
        sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (1, 'AEON')")
            .execute(db.pool()).await.unwrap();
        sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (1, 'LAWSON')")
            .execute(db.pool()).await.unwrap();

        // Two TRANSACTIONS_HEADER rows: one points at the duplicate id (2),
        // one at the surviving id (1). After migration both must point at 1.
        sqlx::query("INSERT INTO TRANSACTIONS_HEADER (TRANSACTION_ID, SHOP_ID) VALUES (100, 1)")
            .execute(db.pool()).await.unwrap();
        sqlx::query("INSERT INTO TRANSACTIONS_HEADER (TRANSACTION_ID, SHOP_ID) VALUES (101, 2)")
            .execute(db.pool()).await.unwrap();
        // A RECURRING_RULES row also pointing at the doomed 2.
        sqlx::query("INSERT INTO RECURRING_RULES (RULE_ID, SHOP_ID) VALUES (200, 2)")
            .execute(db.pool()).await.unwrap();

        db.migrate_shops_unique().await.expect("migration");

        // Duplicate SHOP_ID=2 is gone; 1 (AEON) and 3 (LAWSON) survive.
        assert_eq!(shop_ids(&db).await, vec![1, 3]);

        // References previously on 2 now sit on the surviving 1.
        let hdr_shop: Vec<i64> = sqlx::query_scalar(
            "SELECT SHOP_ID FROM TRANSACTIONS_HEADER ORDER BY TRANSACTION_ID",
        )
        .fetch_all(db.pool())
        .await
        .unwrap();
        assert_eq!(hdr_shop, vec![1, 1], "duplicate ref must be repointed: {:?}", hdr_shop);

        let rule_shop: Vec<i64> = sqlx::query_scalar("SELECT SHOP_ID FROM RECURRING_RULES")
            .fetch_all(db.pool()).await.unwrap();
        assert_eq!(rule_shop, vec![1]);

        // The unique index is in place.
        assert!(unique_index_present(&db).await, "unique index must be created");

        // Inserting a new duplicate is rejected.
        let dup = sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (1, 'AEON')")
            .execute(db.pool())
            .await;
        assert!(dup.is_err(), "post-migration duplicate insert must be rejected");
    }

    #[tokio::test]
    async fn migrate_shops_unique_is_idempotent() {
        let db = setup_legacy_shops_db().await;
        sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (1, 'AEON')")
            .execute(db.pool()).await.unwrap();

        db.migrate_shops_unique().await.expect("first run");
        assert!(unique_index_present(&db).await);
        // Second run finds the index already present and returns early.
        db.migrate_shops_unique().await.expect("second run must be a no-op");
        assert_eq!(shop_ids(&db).await, vec![1], "no data should be touched");
    }

    #[tokio::test]
    async fn migrate_shops_unique_scopes_per_user() {
        let db = setup_legacy_shops_db().await;
        // Two users each have a shop named "AEON" — that's NOT a duplicate.
        sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (1, 'AEON')")
            .execute(db.pool()).await.unwrap();
        sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (2, 'AEON')")
            .execute(db.pool()).await.unwrap();

        db.migrate_shops_unique().await.expect("migration");
        assert_eq!(shop_ids(&db).await, vec![1, 2], "cross-user names must not be treated as duplicates");
    }

    /// Regression pin for the Devin #118 review. A user can legitimately
    /// end up with a soft-deleted old shop (small SHOP_ID, IS_DISABLED=1)
    /// alongside a re-created active shop with the same name (larger
    /// SHOP_ID, IS_DISABLED=0) because `SHOP_CHECK_DUPLICATE_FOR_ADD`
    /// only counts IS_DISABLED=0 rows. Before this fix the migration
    /// picked the smallest SHOP_ID unconditionally — repointing
    /// transactions onto the disabled row and deleting the active one,
    /// so the shop vanished from the (IS_DISABLED=0 filtered)
    /// `get_shops` listing. The survivor rule now prefers active rows.
    #[tokio::test]
    async fn migrate_shops_unique_keeps_active_row_over_soft_deleted_older_id() {
        let db = setup_legacy_shops_db().await;

        // SHOP_ID=1: old "AEON", soft-deleted after use.
        sqlx::query("INSERT INTO SHOPS (SHOP_ID, USER_ID, SHOP_NAME, IS_DISABLED) VALUES (1, 1, 'AEON', 1)")
            .execute(db.pool()).await.unwrap();
        // SHOP_ID=42: user re-added "AEON" and is using it now.
        sqlx::query("INSERT INTO SHOPS (SHOP_ID, USER_ID, SHOP_NAME, IS_DISABLED) VALUES (42, 1, 'AEON', 0)")
            .execute(db.pool()).await.unwrap();

        // Old transaction pointed at the now-disabled row; a new
        // transaction was recorded against the current active row.
        sqlx::query("INSERT INTO TRANSACTIONS_HEADER (TRANSACTION_ID, SHOP_ID) VALUES (500, 1)")
            .execute(db.pool()).await.unwrap();
        sqlx::query("INSERT INTO TRANSACTIONS_HEADER (TRANSACTION_ID, SHOP_ID) VALUES (501, 42)")
            .execute(db.pool()).await.unwrap();

        db.migrate_shops_unique().await.expect("migration");

        // The active SHOP_ID=42 must survive; the disabled 1 gets removed.
        assert_eq!(shop_ids(&db).await, vec![42], "active row (larger id) must be the survivor when the smaller id is disabled");

        // Every reference now points at the active survivor — nothing
        // is left pointing at the deleted (disabled) 1.
        let hdr_shop: Vec<i64> = sqlx::query_scalar(
            "SELECT SHOP_ID FROM TRANSACTIONS_HEADER ORDER BY TRANSACTION_ID",
        )
        .fetch_all(db.pool())
        .await
        .unwrap();
        assert_eq!(hdr_shop, vec![42, 42], "old txn must be repointed to the active row: {:?}", hdr_shop);

        // And the surviving row is active — future `get_shops` (which
        // filters IS_DISABLED=0) will still show it.
        let is_disabled: i64 = sqlx::query_scalar("SELECT IS_DISABLED FROM SHOPS WHERE SHOP_ID = 42")
            .fetch_one(db.pool())
            .await
            .unwrap();
        assert_eq!(is_disabled, 0, "the surviving shop must still be active");
    }

    // ---- Fable-5 #11 — migrate_shops_user_id_cascade tests ----
    //
    // Pre-fix, SHOPS had `FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID)`
    // with no `ON DELETE CASCADE`, so deleting a user with SHOPS rows
    // failed the FK check and aborted the whole DELETE. These tests
    // spin up the pre-fix shape (SHOPS with a non-cascading FK to a
    // real USERS table), run the migration, and assert both the
    // schema change and the resulting delete behaviour.

    /// SHOPS as it shipped before #11 — real FK to USERS but no
    /// `ON DELETE CASCADE` clause. Kept separate from
    /// `LEGACY_SHOPS_DDL` (which has no FK at all, since the
    /// migrate_shops_unique tests don't need one).
    const PRE_CASCADE_SHOPS_DDL: &str = r#"
        CREATE TABLE SHOPS (
            SHOP_ID INTEGER PRIMARY KEY AUTOINCREMENT,
            USER_ID INTEGER NOT NULL,
            SHOP_NAME TEXT NOT NULL,
            MEMO TEXT,
            DISPLAY_ORDER INTEGER NOT NULL DEFAULT 0,
            IS_DISABLED INTEGER DEFAULT 0,
            ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
            UPDATE_DT DATETIME,
            FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID)
        )
    "#;

    async fn setup_pre_cascade_shops_db() -> Database {
        let db = memory_db().await;
        sqlx::query("CREATE TABLE USERS (USER_ID INTEGER PRIMARY KEY, NAME TEXT NOT NULL)")
            .execute(db.pool())
            .await
            .expect("USERS");
        sqlx::query(PRE_CASCADE_SHOPS_DDL)
            .execute(db.pool())
            .await
            .expect("pre-#11 SHOPS");
        db
    }

    async fn shops_user_fk_cascade_count(db: &Database) -> i64 {
        sqlx::query_scalar(sql_queries::SHOPS_HAS_USER_CASCADE_FK)
            .fetch_one(db.pool())
            .await
            .expect("cascade probe")
    }

    #[tokio::test]
    async fn migrate_shops_user_id_cascade_adds_cascade_fk_and_preserves_rows() {
        let db = setup_pre_cascade_shops_db().await;
        // Seed a user + a couple of shops with non-default values for
        // every column the copy touches, so a regression that drops a
        // column from `MIGRATE_SHOPS_CASCADE_COPY_ROWS` fails this
        // test (CodeRabbit on #128).
        sqlx::query("INSERT INTO USERS (USER_ID, NAME) VALUES (1, 'alice')")
            .execute(db.pool()).await.unwrap();
        sqlx::query(
            "INSERT INTO SHOPS \
             (SHOP_ID, USER_ID, SHOP_NAME, MEMO, DISPLAY_ORDER, IS_DISABLED, ENTRY_DT, UPDATE_DT) \
             VALUES (1, 1, 'AEON', 'nearby', 10, 0, '2024-01-15 10:00:00', '2024-06-20 12:34:56')",
        )
        .execute(db.pool()).await.unwrap();
        sqlx::query(
            "INSERT INTO SHOPS \
             (SHOP_ID, USER_ID, SHOP_NAME, MEMO, DISPLAY_ORDER, IS_DISABLED, ENTRY_DT, UPDATE_DT) \
             VALUES (2, 1, 'LAWSON', NULL, 20, 1, '2024-02-01 09:15:00', NULL)",
        )
        .execute(db.pool()).await.unwrap();

        // Baseline: no cascade FK yet.
        assert_eq!(shops_user_fk_cascade_count(&db).await, 0);

        db.migrate_shops_user_id_cascade().await.expect("migration");

        // Post-migration: cascade FK is now present exactly once.
        assert_eq!(shops_user_fk_cascade_count(&db).await, 1);

        // Every one of the eight copied columns survives verbatim.
        let rows: Vec<(i64, i64, String, Option<String>, i64, i64, String, Option<String>)> =
            sqlx::query_as(
                "SELECT SHOP_ID, USER_ID, SHOP_NAME, MEMO, DISPLAY_ORDER, IS_DISABLED, ENTRY_DT, UPDATE_DT \
                 FROM SHOPS ORDER BY SHOP_ID",
            )
            .fetch_all(db.pool())
            .await
            .unwrap();
        assert_eq!(
            rows,
            vec![
                (
                    1,
                    1,
                    "AEON".to_string(),
                    Some("nearby".to_string()),
                    10,
                    0,
                    "2024-01-15 10:00:00".to_string(),
                    Some("2024-06-20 12:34:56".to_string()),
                ),
                (
                    2,
                    1,
                    "LAWSON".to_string(),
                    None,
                    20,
                    1,
                    "2024-02-01 09:15:00".to_string(),
                    None,
                ),
            ]
        );

        // UNIQUE(USER_ID, SHOP_NAME) constraint carried over — a new
        // duplicate INSERT for the same user must be rejected.
        let dup = sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (1, 'AEON')")
            .execute(db.pool())
            .await;
        assert!(
            dup.is_err(),
            "post-migration duplicate (USER_ID, SHOP_NAME) INSERT must be rejected"
        );

        // The non-unique lookup index recreated during the migration
        // must exist — otherwise the DISPLAY_ORDER-scoped scans in
        // shop.rs quietly degrade to full-table scans.
        let idx_user_present: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master \
             WHERE type = 'index' AND tbl_name = 'SHOPS' AND name = 'idx_shops_user'",
        )
        .fetch_one(db.pool())
        .await
        .unwrap();
        assert_eq!(idx_user_present, 1, "idx_shops_user must be recreated after the migration");
    }

    #[tokio::test]
    async fn migrate_shops_user_id_cascade_is_idempotent() {
        let db = setup_pre_cascade_shops_db().await;
        sqlx::query("INSERT INTO USERS (USER_ID, NAME) VALUES (1, 'alice')")
            .execute(db.pool()).await.unwrap();
        sqlx::query("INSERT INTO SHOPS (SHOP_ID, USER_ID, SHOP_NAME) VALUES (1, 1, 'AEON')")
            .execute(db.pool()).await.unwrap();

        db.migrate_shops_user_id_cascade().await.expect("first run");
        assert_eq!(shops_user_fk_cascade_count(&db).await, 1);

        // Second run must find the cascade FK already present and
        // return early — no DROP TABLE, no data touched.
        db.migrate_shops_user_id_cascade().await.expect("second run must be a no-op");
        let ids: Vec<i64> = sqlx::query_scalar("SELECT SHOP_ID FROM SHOPS")
            .fetch_all(db.pool())
            .await
            .unwrap();
        assert_eq!(ids, vec![1], "no data should be touched by the idempotent run");
    }

    /// The end-to-end guarantee that motivates the fix: after the
    /// migration, deleting a user cascades to their SHOPS rows
    /// instead of aborting with `FOREIGN KEY constraint failed`.
    #[tokio::test]
    async fn user_delete_cascades_to_shops_after_migration() {
        let db = setup_pre_cascade_shops_db().await;
        sqlx::query("INSERT INTO USERS (USER_ID, NAME) VALUES (1, 'alice')")
            .execute(db.pool()).await.unwrap();
        sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (1, 'AEON')")
            .execute(db.pool()).await.unwrap();
        sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (1, 'LAWSON')")
            .execute(db.pool()).await.unwrap();

        db.migrate_shops_user_id_cascade().await.expect("migration");

        // FK enforcement is per-connection in SQLite; the pool's
        // `after_connect` in `connect_db` sets it, but be explicit here
        // in case the connection returned to us was reused from an
        // earlier `PRAGMA foreign_keys = OFF` window inside the
        // migration.
        sqlx::query(sql_queries::PRAGMA_FOREIGN_KEYS_ON)
            .execute(db.pool())
            .await
            .unwrap();

        // Deleting the user must succeed and take the SHOPS rows with
        // it. Before the migration the same DELETE aborted with
        // `FOREIGN KEY constraint failed`.
        sqlx::query("DELETE FROM USERS WHERE USER_ID = 1")
            .execute(db.pool())
            .await
            .expect("USER delete must succeed with CASCADE FK");

        let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM SHOPS")
            .fetch_one(db.pool())
            .await
            .unwrap();
        assert_eq!(remaining, 0, "SHOPS rows must cascade with their owner");
    }
}
