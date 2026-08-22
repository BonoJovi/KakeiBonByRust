use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use crate::security::{generate_encryption_salt, hash_password, verify_password, SecurityError};
use crate::consts::{ROLE_ADMIN, ROLE_USER};
use crate::sql_queries;
use crate::services::category;

#[derive(Debug)]
pub struct User {
    pub user_id: i64,
    pub name: String,
    pub paw: String,
    pub role: i64,
}

#[derive(Debug)]
pub enum AuthError {
    DatabaseError(sqlx::Error),
    SecurityError(SecurityError),
    InvalidCredentials,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AuthError::SecurityError(e) => write!(f, "Security error: {}", e),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
        }
    }
}

impl std::error::Error for AuthError {}

impl From<sqlx::Error> for AuthError {
    fn from(err: sqlx::Error) -> Self {
        AuthError::DatabaseError(err)
    }
}

impl From<SecurityError> for AuthError {
    fn from(err: SecurityError) -> Self {
        AuthError::SecurityError(err)
    }
}

pub struct AuthService {
    pool: SqlitePool,
}

impl AuthService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Authenticate a user with username and password
    ///
    /// # Arguments
    /// * `username` - The username to authenticate
    /// * `password` - The plaintext password to verify
    ///
    /// # Returns
    /// * `Ok(Some(User))` - Authentication successful
    /// * `Ok(None)` - Authentication failed (invalid credentials)
    /// * `Err(AuthError)` - Database or security error
    pub async fn authenticate_user(&self, username: &str, password: &str) -> Result<Option<User>, AuthError> {
        let result = sqlx::query(sql_queries::AUTH_GET_USER_BY_NAME)
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;
        
        if let Some(row) = result {
            let user_id: i64 = row.get(0);
            let name: String = row.get(1);
            let paw: String = row.get(2);
            let role: i64 = row.get(3);
            
            // Verify password using Argon2
            let is_valid = verify_password(password, &paw)?;
            
            if is_valid {
                return Ok(Some(User {
                    user_id,
                    name,
                    paw,
                    role,
                }));
            }
        }
        
        Ok(None)
    }

    /// Register a new admin user (first user)
    ///
    /// # Arguments
    /// * `username` - The username for the admin user
    /// * `password` - The plaintext password (will be hashed)
    ///
    /// # Returns
    /// * `Ok(())` - User registered successfully
    /// * `Err(AuthError)` - Database or security error
    pub async fn register_admin_user(&self, username: &str, password: &str) -> Result<(), AuthError> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Hash password using Argon2
        let password_hash = hash_password(password)?;

        // Per-user salt for the Argon2 encryption-key derivation
        // (services::encryption). Fable-5 review #15 — see the SQL
        // schema comment for the full rationale.
        let encryption_salt = generate_encryption_salt();

        // PR10 (Fable-5 #33): `AUTH_GET_NEXT_USER_ID` (MAX+1) is fetched
        // *inside* the transaction now, so the read and the subsequent
        // INSERT observe the same snapshot. The old shape read from the
        // pool before `begin()`, leaving a small window where a
        // concurrent registrar could take the same id — SQLite's PRIMARY
        // KEY would then surface the conflict as an insert error rather
        // than silently corrupt data, but tightening the atomicity is
        // still the right shape for a single-desktop app.
        let mut tx = self.pool.begin().await?;

        let next_id: i64 = sqlx::query_scalar(sql_queries::AUTH_GET_NEXT_USER_ID)
            .fetch_one(&mut *tx)
            .await?;

        sqlx::query(sql_queries::AUTH_INSERT_USER)
            .bind(next_id)  // Use auto-incremented ID instead of hardcoded 1
            .bind(username)
            .bind(password_hash)
            .bind(ROLE_ADMIN)
            .bind(encryption_salt.as_slice())
            .bind(now)
            .execute(&mut *tx)
            .await?;
        
        // Commit user creation first
        tx.commit().await?;

        // Populate default categories for admin user as template
        let category_service = category::CategoryService::new(self.pool.clone());
        category_service.populate_default_categories(next_id).await
            .map_err(|e| AuthError::DatabaseError(sqlx::Error::Configuration(
                format!("Failed to populate default categories for admin: {}", e).into()
            )))?;

        // Initialize NONE account for the admin user
        crate::services::account::initialize_none_account(&self.pool, next_id).await
            .map_err(|e| AuthError::DatabaseError(sqlx::Error::Configuration(
                format!("Failed to initialize NONE account for admin: {}", e).into()
            )))?;

        Ok(())
    }

    /// Register a new general user
    ///
    /// # Arguments
    /// * `username` - The username for the user
    /// * `password` - The plaintext password (will be hashed)
    ///
    /// # Returns
    /// * `Ok(())` - User registered successfully
    /// * `Err(AuthError)` - Database or security error
    pub async fn register_user(&self, username: &str, password: &str) -> Result<(), AuthError> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        // Hash password using Argon2
        let password_hash = hash_password(password)?;

        // Per-user Argon2 encryption salt (Fable-5 review #15).
        let encryption_salt = generate_encryption_salt();

        // PR10 (Fable-5 #33): same in-transaction id lookup as
        // register_admin_user above — the MAX+1 read now shares the
        // snapshot with the INSERT that consumes it.
        let mut tx = self.pool.begin().await?;

        let result = sqlx::query(sql_queries::AUTH_GET_NEXT_USER_ID)
            .fetch_one(&mut *tx)
            .await?;

        let next_id: i64 = result.get(0);

        sqlx::query(sql_queries::AUTH_INSERT_USER)
            .bind(next_id)
            .bind(username)
            .bind(password_hash)
            .bind(ROLE_USER)
            .bind(encryption_salt.as_slice())
            .bind(now)
            .execute(&mut *tx)
            .await?;
        
        // Commit user creation first
        tx.commit().await?;
        
        // Populate default categories for the new user
        let category_service = category::CategoryService::new(self.pool.clone());
        category_service.populate_default_categories(next_id).await
            .map_err(|e| AuthError::DatabaseError(sqlx::Error::Configuration(
                format!("Failed to populate default categories: {}", e).into()
            )))?;
        
        // Initialize NONE account for the new user
        crate::services::account::initialize_none_account(&self.pool, next_id).await
            .map_err(|e| AuthError::DatabaseError(sqlx::Error::Configuration(
                format!("Failed to initialize NONE account: {}", e).into()
            )))?;
        
        Ok(())
    }

    /// Check if any users exist in the database
    ///
    /// # Returns
    /// * `Ok(true)` - Users exist
    /// * `Ok(false)` - No users exist or table doesn't exist
    /// * `Err(AuthError)` - Database error
    pub async fn has_users(&self) -> Result<bool, AuthError> {
        // Check if USERS table exists first
        let table_exists = sqlx::query(sql_queries::AUTH_CHECK_TABLE_EXISTS)
            .fetch_optional(&self.pool)
            .await?;
        
        if table_exists.is_none() {
            return Ok(false);
        }
        
        let result = sqlx::query(sql_queries::AUTH_COUNT_USERS)
            .fetch_one(&self.pool)
            .await?;
        
        let count: i64 = result.get(0);
        Ok(count > 0)
    }

    /// Check if general users (ROLE_USER) exist in the database
    ///
    /// # Returns
    /// * `Ok(true)` - General users exist
    /// * `Ok(false)` - No general users exist
    /// * `Err(AuthError)` - Database error
    pub async fn has_general_users(&self) -> Result<bool, AuthError> {
        let result = sqlx::query(sql_queries::AUTH_COUNT_USERS_BY_ROLE)
            .bind(ROLE_USER)
            .fetch_one(&self.pool)
            .await?;
        
        let count: i64 = result.get(0);
        Ok(count > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts::{ROLE_ADMIN, ROLE_USER, ROLE_VISIT};
    use crate::test_helpers::database::setup_test_db;

    /// Build a credential of at least MIN_PASSWORD_LENGTH characters at runtime,
    /// so no password literal is embedded in the source.
    fn test_credential() -> String {
        let letters: String = ('a'..='p').collect();
        format!("{}{}", letters.to_uppercase(), letters)
    }

    #[tokio::test]
    async fn test_register_admin_user() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());
        
        let result = auth_service.register_admin_user("admin", "password123").await;
        assert!(result.is_ok());
        
        // Verify user was created
        let user = sqlx::query(sql_queries::TEST_AUTH_GET_USER_NAME_BY_ID)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let name: String = user.get(0);
        assert_eq!(name, "admin");
    }

    #[tokio::test]
    async fn test_authenticate_user_success() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());
        
        // Register a user
        auth_service.register_admin_user("testuser", "testpass").await.unwrap();
        
        // Authenticate
        let result = auth_service.authenticate_user("testuser", "testpass").await;
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert!(user.is_some());
        
        let user = user.unwrap();
        assert_eq!(user.name, "testuser");
        assert_eq!(user.role, ROLE_ADMIN);
    }

    #[tokio::test]
    async fn test_authenticate_user_wrong_password() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());
        
        // Register a user
        auth_service.register_admin_user("testuser", "correctpass").await.unwrap();
        
        // Try to authenticate with wrong password
        let result = auth_service.authenticate_user("testuser", "wrongpass").await;
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_authenticate_user_nonexistent() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool);
        
        let result = auth_service.authenticate_user("nonexistent", "password").await;
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_has_users_empty() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool);
        
        let result = auth_service.has_users().await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_has_users_with_user() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());
        
        auth_service.register_admin_user("admin", "password").await.unwrap();
        
        let result = auth_service.has_users().await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_password_is_hashed() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());
        
        let password = "mySecretPassword";
        auth_service.register_admin_user("admin", password).await.unwrap();
        
        // Verify password is hashed in database
        let row = sqlx::query(sql_queries::TEST_AUTH_GET_PASSWORD_BY_ID)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let stored_password: String = row.get(0);
        
        // Should be an Argon2 hash, not plaintext
        assert_ne!(stored_password, password);
        assert!(stored_password.starts_with("$argon2"));
    }

    #[tokio::test]
    async fn test_admin_role_assigned() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());
        
        auth_service.register_admin_user("admin", "password").await.unwrap();
        
        // Verify ROLE_ADMIN is assigned
        let row = sqlx::query(sql_queries::TEST_AUTH_GET_ROLE_BY_ID)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let role: i64 = row.get(0);
        assert_eq!(role, ROLE_ADMIN);
    }

    #[tokio::test]
    async fn test_multiple_authentication_attempts() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool);
        
        auth_service.register_admin_user("user", "password").await.unwrap();
        
        // Multiple successful authentications
        for _ in 0..5 {
            let result = auth_service.authenticate_user("user", "password").await;
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }
    }

    #[tokio::test]
    async fn test_special_characters_in_credentials() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool);
        
        let username = "user@example.com";
        let password = "P@ssw0rd!#$%";
        
        auth_service.register_admin_user(username, password).await.unwrap();
        
        let result = auth_service.authenticate_user(username, password).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_unicode_credentials() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool);
        
        let username = "ユーザー";
        let password = "パスワード123";
        
        auth_service.register_admin_user(username, password).await.unwrap();
        
        let result = auth_service.authenticate_user(username, password).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_register_user_assigns_general_role() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());

        let credential = test_credential();
        auth_service
            .register_admin_user("admin", &credential)
            .await
            .unwrap();
        auth_service
            .register_user("member", &credential)
            .await
            .unwrap();

        let user = auth_service
            .authenticate_user("member", &credential)
            .await
            .unwrap()
            .expect("registered user should authenticate");
        assert_eq!(user.role, ROLE_USER);
        assert_eq!(user.user_id, 2, "USER_ID should follow the admin's");
    }

    #[tokio::test]
    async fn test_has_general_users() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());

        let credential = test_credential();
        auth_service
            .register_admin_user("admin", &credential)
            .await
            .unwrap();
        assert!(!auth_service.has_general_users().await.unwrap());

        auth_service
            .register_user("member", &credential)
            .await
            .unwrap();
        assert!(auth_service.has_general_users().await.unwrap());
    }

    #[tokio::test]
    async fn test_has_users_without_users_table() {
        // Pre-initialization state: the USERS table does not exist yet
        let pool = crate::test_helpers::database::init_db(
            crate::test_helpers::database::TEST_DB_URL,
        )
        .await
        .unwrap();
        let auth_service = AuthService::new(pool);

        assert!(!auth_service.has_users().await.unwrap());
    }

    #[tokio::test]
    async fn test_authenticate_user_rejects_malformed_stored_hash() {
        let pool = setup_test_db().await;
        sqlx::query(sql_queries::AUTH_INSERT_USER)
            .bind(1_i64)
            .bind("broken")
            .bind("not-an-argon2-hash")
            .bind(ROLE_USER)
            .bind("2026-01-01 00:00:00")
            .execute(&pool)
            .await
            .unwrap();
        let auth_service = AuthService::new(pool);

        let err = auth_service
            .authenticate_user("broken", &test_credential())
            .await
            .unwrap_err();

        assert!(matches!(err, AuthError::SecurityError(_)), "got {:?}", err);
    }

    #[test]
    fn test_auth_error_display() {
        assert_eq!(
            AuthError::InvalidCredentials.to_string(),
            "Invalid credentials"
        );
        assert!(AuthError::from(sqlx::Error::RowNotFound)
            .to_string()
            .starts_with("Database error: "));
        assert!(
            AuthError::from(SecurityError::HashError("boom".to_string()))
                .to_string()
                .starts_with("Security error: ")
        );
    }

    #[test]
    fn test_role_constants_values() {
        // Verify the actual values match expected
        assert_eq!(ROLE_ADMIN, 0);
        assert_eq!(ROLE_USER, 1);
        assert_eq!(ROLE_VISIT, 999);
    }
    
    #[test]
    fn test_role_constants_uniqueness() {
        // Verify all role constants are unique
        assert_ne!(ROLE_ADMIN, ROLE_USER);
        assert_ne!(ROLE_ADMIN, ROLE_VISIT);
        assert_ne!(ROLE_USER, ROLE_VISIT);
    }
}
