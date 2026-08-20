use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use crate::crypto::Crypto;
use crate::security::{derive_encryption_key, SecurityError};
use crate::sql_queries;

#[derive(Debug, Clone)]
pub struct EncryptedField {
    pub field_id: i64,
    pub table_name: String,
    pub column_name: String,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug)]
pub enum EncryptionError {
    DatabaseError(sqlx::Error),
    SecurityError(SecurityError),
    DecryptionFailed(String),
    EncryptionFailed(String),
    NoEncryptedFields,
    InvalidIdentifier(String),
}

impl std::fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionError::DatabaseError(e) => write!(f, "Database error: {}", e),
            EncryptionError::SecurityError(e) => write!(f, "Security error: {}", e),
            EncryptionError::DecryptionFailed(e) => write!(f, "Decryption failed: {}", e),
            EncryptionError::EncryptionFailed(e) => write!(f, "Encryption failed: {}", e),
            EncryptionError::NoEncryptedFields => write!(f, "No encrypted fields defined"),
            EncryptionError::InvalidIdentifier(name) => {
                write!(f, "Invalid SQL identifier: {}", name)
            }
        }
    }
}

impl std::error::Error for EncryptionError {}

/// Validate a table or column name before it is interpolated into SQL.
///
/// Identifiers can never be bound as parameters, so the only safe option is to
/// restrict them to a conservative character set and length.
fn validate_sql_identifier(name: &str) -> Result<(), EncryptionError> {
    let valid = !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !name.starts_with(|c: char| c.is_ascii_digit());

    if valid {
        Ok(())
    } else {
        Err(EncryptionError::InvalidIdentifier(name.to_string()))
    }
}

impl From<sqlx::Error> for EncryptionError {
    fn from(err: sqlx::Error) -> Self {
        EncryptionError::DatabaseError(err)
    }
}

impl From<SecurityError> for EncryptionError {
    fn from(err: SecurityError) -> Self {
        EncryptionError::SecurityError(err)
    }
}

pub struct EncryptionService {
    pool: SqlitePool,
}

impl EncryptionService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get all active encrypted fields
    pub async fn get_encrypted_fields(&self) -> Result<Vec<EncryptedField>, EncryptionError> {
        let rows = sqlx::query(
            r#"
            SELECT FIELD_ID, TABLE_NAME, COLUMN_NAME, DESCRIPTION, IS_ACTIVE
            FROM ENCRYPTED_FIELDS
            WHERE IS_ACTIVE = 1
            ORDER BY TABLE_NAME, COLUMN_NAME
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let fields = rows.into_iter().map(|row| {
            EncryptedField {
                field_id: row.get(0),
                table_name: row.get(1),
                column_name: row.get(2),
                description: row.get(3),
                is_active: row.get::<i64, _>(4) == 1,
            }
        }).collect();

        Ok(fields)
    }

    /// Register a new encrypted field
    pub async fn register_encrypted_field(
        &self,
        table_name: &str,
        column_name: &str,
        description: Option<&str>,
    ) -> Result<i64, EncryptionError> {
        validate_sql_identifier(table_name)?;
        validate_sql_identifier(column_name)?;

        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = sqlx::query(sql_queries::ENCRYPTION_GET_NEXT_FIELD_ID)
            .fetch_one(&self.pool)
            .await?;
        let next_id: i64 = result.get(0);

        sqlx::query(sql_queries::ENCRYPTION_INSERT_FIELD)
            .bind(next_id)
            .bind(table_name)
            .bind(column_name)
            .bind(description)
            .bind(now)
            .execute(&self.pool)
            .await?;

        Ok(next_id)
    }

    /// Re-encrypt all encrypted fields for a user
    pub async fn re_encrypt_user_data(
        &self,
        user_id: i64,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), EncryptionError> {
        // Get all encrypted fields
        let encrypted_fields = self.get_encrypted_fields().await?;
        
        if encrypted_fields.is_empty() {
            return Ok(());
        }

        // Derive encryption keys from passwords
        let salt = user_id.to_le_bytes();
        let old_key = derive_encryption_key(old_password, &salt)?;
        let new_key = derive_encryption_key(new_password, &salt)?;

        let old_crypto = Crypto::new(old_key);
        let new_crypto = Crypto::new(new_key);

        // Start transaction
        let mut tx = self.pool.begin().await?;

        // Group fields by table
        let mut tables: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for field in &encrypted_fields {
            tables.entry(field.table_name.clone())
                .or_insert_with(Vec::new)
                .push(field.column_name.clone());
        }

        // Re-encrypt data for each table
        for (table_name, columns) in tables {
            // Identifiers cannot be bound, so validate them before interpolating
            validate_sql_identifier(&table_name)?;
            for column in &columns {
                validate_sql_identifier(column)?;
            }

            // Build SELECT query.
            //
            // Fable-5 review #14 — the pre-fix shape used
            // `fetch_optional` (reads one row) and later
            // `UPDATE ... WHERE USER_ID = ?` (writes every row for the
            // user). If a user had multiple rows in the same table
            // (e.g., multiple TRANSACTIONS_HEADER with encrypted memos),
            // only the first row's decrypted plaintext was re-encrypted
            // and then **stamped over every other row**, permanently
            // destroying the other rows' data.
            //
            // Fix: read every row with its `ROWID` and re-encrypt each
            // one independently, then UPDATE that specific row by
            // `ROWID`. `ROWID` is guaranteed for every non-WITHOUT-ROWID
            // table in SQLite and is stable within a transaction, so
            // it's the right cheap PK for this loop even when the table
            // uses a composite or non-integer primary key.
            let column_list = columns.join(", ");
            let select_query = format!(
                "SELECT ROWID, {} FROM {} WHERE USER_ID = ?",
                column_list, table_name
            );

            // Fetch every encrypted row for this user (not just the first).
            let rows = sqlx::query(&select_query)
                .bind(user_id)
                .fetch_all(&mut *tx)
                .await?;

            for row in rows {
                let rowid: i64 = row
                    .try_get(0)
                    .map_err(EncryptionError::DatabaseError)?;

                // Decrypt and re-encrypt each field for this specific row.
                let mut updates = Vec::new();
                for (idx, column) in columns.iter().enumerate() {
                    // A decode failure here would otherwise skip the column and
                    // leave it encrypted with the old key, making it permanently
                    // unreadable after the password change.
                    let encrypted_value: Option<String> = row
                        .try_get(idx + 1)
                        .map_err(EncryptionError::DatabaseError)?;

                    if let Some(enc_val) = encrypted_value {
                        // Decrypt with old key
                        let decrypted = old_crypto.decrypt(&enc_val)
                            .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

                        // Re-encrypt with new key
                        let re_encrypted = new_crypto.encrypt(&decrypted)
                            .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

                        updates.push((column.clone(), re_encrypted));
                    }
                }

                // Build and execute UPDATE query for THIS row only.
                if !updates.is_empty() {
                    let set_clause = updates.iter()
                        .map(|(col, _)| format!("{} = ?", col))
                        .collect::<Vec<_>>()
                        .join(", ");

                    let update_query = format!(
                        "UPDATE {} SET {} WHERE ROWID = ?",
                        table_name, set_clause
                    );

                    let mut query = sqlx::query(&update_query);
                    for (_, value) in &updates {
                        query = query.bind(value);
                    }
                    query = query.bind(rowid);

                    query.execute(&mut *tx).await?;
                }
            }
        }

        // Commit transaction
        tx.commit().await?;

        Ok(())
    }

    /// Encrypt data for a new user
    pub async fn encrypt_field(
        &self,
        user_id: i64,
        password: &str,
        plaintext: &str,
    ) -> Result<String, EncryptionError> {
        let salt = user_id.to_le_bytes();
        let key = derive_encryption_key(password, &salt)?;
        let crypto = Crypto::new(key);
        
        crypto.encrypt(plaintext)
            .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))
    }

    /// Decrypt data for a user
    pub async fn decrypt_field(
        &self,
        user_id: i64,
        password: &str,
        ciphertext: &str,
    ) -> Result<String, EncryptionError> {
        let salt = user_id.to_le_bytes();
        let key = derive_encryption_key(password, &salt)?;
        let crypto = Crypto::new(key);
        
        crypto.decrypt(ciphertext)
            .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::database::{init_db, TEST_DB_URL};

    async fn setup_test_db() -> SqlitePool {
        let pool = init_db(TEST_DB_URL).await.unwrap();

        // Create ENCRYPTED_FIELDS table
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS ENCRYPTED_FIELDS (
                FIELD_ID INTEGER NOT NULL,
                TABLE_NAME VARCHAR(128) NOT NULL,
                COLUMN_NAME VARCHAR(128) NOT NULL,
                DESCRIPTION VARCHAR(256),
                IS_ACTIVE INTEGER NOT NULL DEFAULT 1,
                ENTRY_DT DATETIME NOT NULL,
                PRIMARY KEY(FIELD_ID),
                UNIQUE(TABLE_NAME, COLUMN_NAME)
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        // Create a test table with encrypted fields
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS TEST_DATA (
                USER_ID INTEGER NOT NULL,
                SECRET_NOTE TEXT,
                SECRET_MEMO TEXT,
                PRIMARY KEY(USER_ID)
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_register_encrypted_field() {
        let pool = setup_test_db().await;
        let service = EncryptionService::new(pool.clone());

        let field_id = service.register_encrypted_field(
            "TEST_DATA",
            "SECRET_NOTE",
            Some("Test encrypted note field")
        ).await.unwrap();

        assert!(field_id > 0);

        let fields = service.get_encrypted_fields().await.unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].table_name, "TEST_DATA");
        assert_eq!(fields[0].column_name, "SECRET_NOTE");
    }

    #[tokio::test]
    async fn test_register_encrypted_field_rejects_invalid_identifiers() {
        let pool = setup_test_db().await;
        let service = EncryptionService::new(pool.clone());

        for (table, column) in [
            ("TEST_DATA; DROP TABLE USERS--", "SECRET_NOTE"),
            ("TEST_DATA", "SECRET_NOTE, PAW"),
            ("", "SECRET_NOTE"),
            ("1TEST", "SECRET_NOTE"),
        ] {
            let result = service
                .register_encrypted_field(table, column, None)
                .await;
            assert!(
                matches!(result, Err(EncryptionError::InvalidIdentifier(_))),
                "expected rejection for {}.{}",
                table,
                column
            );
        }

        assert!(service.get_encrypted_fields().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_encrypt_decrypt_field() {
        let pool = setup_test_db().await;
        let service = EncryptionService::new(pool.clone());

        let user_id = 1;
        let password = "test_password_123";
        let plaintext = "This is a secret message";

        // Encrypt
        let ciphertext = service.encrypt_field(user_id, password, plaintext)
            .await
            .unwrap();

        // Decrypt
        let decrypted = service.decrypt_field(user_id, password, &ciphertext)
            .await
            .unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[tokio::test]
    async fn test_re_encrypt_user_data() {
        let pool = setup_test_db().await;
        let service = EncryptionService::new(pool.clone());

        // Register encrypted fields
        service.register_encrypted_field("TEST_DATA", "SECRET_NOTE", None).await.unwrap();
        service.register_encrypted_field("TEST_DATA", "SECRET_MEMO", None).await.unwrap();

        let user_id = 1;
        let old_password = "old_password_123";
        let new_password = "new_password_456";

        // Encrypt data with old password
        let note_encrypted = service.encrypt_field(user_id, old_password, "Secret note").await.unwrap();
        let memo_encrypted = service.encrypt_field(user_id, old_password, "Secret memo").await.unwrap();

        // Insert test data
        sqlx::query(
            "INSERT INTO TEST_DATA (USER_ID, SECRET_NOTE, SECRET_MEMO) VALUES (?, ?, ?)"
        )
        .bind(user_id)
        .bind(&note_encrypted)
        .bind(&memo_encrypted)
        .execute(&pool)
        .await
        .unwrap();

        // Re-encrypt with new password
        service.re_encrypt_user_data(user_id, old_password, new_password)
            .await
            .unwrap();

        // Verify data can be decrypted with new password
        let row = sqlx::query("SELECT SECRET_NOTE, SECRET_MEMO FROM TEST_DATA WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        let note_cipher: String = row.get(0);
        let memo_cipher: String = row.get(1);

        let note_decrypted = service.decrypt_field(user_id, new_password, &note_cipher)
            .await
            .unwrap();
        let memo_decrypted = service.decrypt_field(user_id, new_password, &memo_cipher)
            .await
            .unwrap();

        assert_eq!(note_decrypted, "Secret note");
        assert_eq!(memo_decrypted, "Secret memo");
    }

    /// Fable-5 review #14 — before the fix, `re_encrypt_user_data`
    /// read one row with `fetch_optional`, decrypted its columns, then
    /// wrote the re-encrypted plaintext back with
    /// `UPDATE ... WHERE USER_ID = ?` — which stamped that single
    /// value onto every row for the user. If a user had multiple rows
    /// in the same encrypted table (perfectly possible for a table
    /// like `TRANSACTIONS_HEADER` whose encrypted memo would be
    /// registered in `ENCRYPTED_FIELDS`), rows 2..N had their
    /// plaintext permanently replaced with row 1's plaintext on every
    /// password change. This test reproduces the multi-row shape and
    /// asserts each row keeps its own plaintext after re-encryption.
    #[tokio::test]
    async fn test_re_encrypt_user_data_preserves_per_row_plaintext() {
        let pool = init_db(TEST_DB_URL).await.unwrap();

        // Same ENCRYPTED_FIELDS table as `setup_test_db`, but the data
        // table drops the USER_ID primary key so we can seed multiple
        // rows for the same user — the exact shape that would exist for
        // (e.g.) transaction memos, where one user has many rows.
        sqlx::query(
            r#"
            CREATE TABLE ENCRYPTED_FIELDS (
                FIELD_ID INTEGER NOT NULL,
                TABLE_NAME VARCHAR(128) NOT NULL,
                COLUMN_NAME VARCHAR(128) NOT NULL,
                DESCRIPTION VARCHAR(256),
                IS_ACTIVE INTEGER NOT NULL DEFAULT 1,
                ENTRY_DT DATETIME NOT NULL,
                PRIMARY KEY(FIELD_ID),
                UNIQUE(TABLE_NAME, COLUMN_NAME)
            )
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            r#"
            CREATE TABLE MULTI_ROW (
                ROW_ID INTEGER PRIMARY KEY AUTOINCREMENT,
                USER_ID INTEGER NOT NULL,
                SECRET_NOTE TEXT
            )
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        let service = EncryptionService::new(pool.clone());
        service
            .register_encrypted_field("MULTI_ROW", "SECRET_NOTE", None)
            .await
            .unwrap();

        let user_id = 1;
        let old_password = "old_password_123";
        let new_password = "new_password_456";

        // Three rows for the same user, each with distinct plaintext.
        // The pre-fix shape would end up with all three rows storing
        // whichever plaintext happened to be read first.
        let plaintexts = ["first note", "second note", "third note"];
        for pt in &plaintexts {
            let cipher = service.encrypt_field(user_id, old_password, pt).await.unwrap();
            sqlx::query("INSERT INTO MULTI_ROW (USER_ID, SECRET_NOTE) VALUES (?, ?)")
                .bind(user_id)
                .bind(&cipher)
                .execute(&pool)
                .await
                .unwrap();
        }

        service
            .re_encrypt_user_data(user_id, old_password, new_password)
            .await
            .unwrap();

        // Every row must still decrypt to *its own* plaintext.
        let rows: Vec<(i64, String)> = sqlx::query_as(
            "SELECT ROW_ID, SECRET_NOTE FROM MULTI_ROW WHERE USER_ID = ? ORDER BY ROW_ID",
        )
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .unwrap();

        assert_eq!(rows.len(), 3);
        for ((_, cipher), expected_plaintext) in rows.iter().zip(plaintexts.iter()) {
            let decrypted = service
                .decrypt_field(user_id, new_password, cipher)
                .await
                .unwrap();
            assert_eq!(
                &decrypted, expected_plaintext,
                "row must keep its own plaintext across re-encryption"
            );
        }
    }

    #[tokio::test]
    async fn test_decrypt_with_wrong_password_fails() {
        let pool = setup_test_db().await;
        let service = EncryptionService::new(pool.clone());

        let user_id = 1;
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let plaintext = "Secret data";

        let ciphertext = service.encrypt_field(user_id, password, plaintext)
            .await
            .unwrap();

        let result = service.decrypt_field(user_id, wrong_password, &ciphertext).await;
        assert!(result.is_err());
    }
}
