use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use crate::security::{hash_password, verify_password, SecurityError};
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
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        sqlx::query(sql_queries::AUTH_INSERT_USER)
            .bind(1)  // USER_ID = 1 for admin
            .bind(username)
            .bind(password_hash)
            .bind(ROLE_ADMIN)
            .bind(now)
            .execute(&mut *tx)
            .await?;
        
        // Commit user creation first
        tx.commit().await?;
        
        // Populate default categories for admin user as template
        let category_service = category::CategoryService::new(self.pool.clone());
        category_service.populate_default_categories(1).await
            .map_err(|e| AuthError::DatabaseError(sqlx::Error::Configuration(
                format!("Failed to populate default categories for admin: {}", e).into()
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
        
        // Get next user ID
        let result = sqlx::query(sql_queries::AUTH_GET_NEXT_USER_ID)
            .fetch_one(&self.pool)
            .await?;
        
        let next_id: i64 = result.get(0);
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        sqlx::query(sql_queries::AUTH_INSERT_USER)
            .bind(next_id)
            .bind(username)
            .bind(password_hash)
            .bind(ROLE_USER)
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
