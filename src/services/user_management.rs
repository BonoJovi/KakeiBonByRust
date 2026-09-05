use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use crate::api_error::ApiError;
use crate::security::{generate_encryption_salt, hash_password, verify_password, SecurityError};
use crate::consts::{self, ROLE_ADMIN, ROLE_USER};
use crate::sql_queries;
use super::encryption::EncryptionService;

const ENTITY_LABEL: &str = "User";

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub user_id: i64,
    pub name: String,
    pub role: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

#[derive(Debug)]
pub enum UserManagementError {
    DatabaseError(sqlx::Error),
    SecurityError(SecurityError),
    UserNotFound,
    AdminUserCannotBeDeleted,
    InvalidRole,
    DuplicateUsername,
    /// Fable-5 #1/#5 — the caller-supplied current password did not
    /// verify. Kept as its own variant (rather than a
    /// `SecurityError::InvalidPassword` inside a generic
    /// `SecurityError`) so the `From<UserManagementError> for ApiError`
    /// bridge can map it to a dedicated `old_password_incorrect` code
    /// instead of the generic `validation` bucket.
    OldPasswordIncorrect,
    Validation(String),
}

impl std::fmt::Display for UserManagementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserManagementError::DatabaseError(e) => write!(f, "Database error: {}", e),
            UserManagementError::SecurityError(e) => write!(f, "Security error: {}", e),
            UserManagementError::UserNotFound => write!(f, "User not found"),
            UserManagementError::AdminUserCannotBeDeleted => write!(f, "Admin user cannot be deleted"),
            UserManagementError::InvalidRole => write!(f, "Invalid role"),
            UserManagementError::DuplicateUsername => write!(f, "Username already exists"),
            UserManagementError::OldPasswordIncorrect => write!(f, "Current password is incorrect"),
            UserManagementError::Validation(msg) => write!(f, "{}", msg),
        }
    }
}

/// Issue #37 Phase 2-3 — USERS.NAME length guard. Counts characters, not
/// bytes, mirroring the frontend `maxlength` and char counter.
fn validate_username_length(username: &str) -> Result<(), UserManagementError> {
    crate::validation::validate_max_chars("Username", username, consts::MAX_NAME_LEN)
        .map_err(UserManagementError::Validation)
}

impl std::error::Error for UserManagementError {}

impl From<sqlx::Error> for UserManagementError {
    fn from(err: sqlx::Error) -> Self {
        UserManagementError::DatabaseError(err)
    }
}

impl From<SecurityError> for UserManagementError {
    fn from(err: SecurityError) -> Self {
        UserManagementError::SecurityError(err)
    }
}

/// Map the domain-specific `UserManagementError` onto the wire-level
/// `ApiError` so tauri command wrappers can `?`-propagate it into a
/// structured `{ code, message, entity? }` payload for the frontend
/// classifier (`res/js/master-crud.js`). Kept as `From` (rather than
/// a bespoke `.map_err`) so wrapper bodies stay one-line — the
/// mapping happens implicitly at the `?` boundary.
///
/// Codes:
///   - `UserNotFound`               → `not_found` (entity="user")
///   - `AdminUserCannotBeDeleted`   → `admin_protected` (entity="user")
///   - `DuplicateUsername`          → `duplicate_name` (entity="user")
///   - `InvalidRole`, `SecurityError`, `Validation(...)` → `validation`
///     (with a message that keeps the original English text for logs)
///   - `DatabaseError`              → `database`
impl From<UserManagementError> for ApiError {
    fn from(err: UserManagementError) -> Self {
        match err {
            UserManagementError::UserNotFound => ApiError::not_found(ENTITY_LABEL),
            UserManagementError::AdminUserCannotBeDeleted => ApiError::admin_protected(ENTITY_LABEL),
            UserManagementError::DuplicateUsername => ApiError::duplicate_name(ENTITY_LABEL),
            UserManagementError::InvalidRole => ApiError::validation("Invalid role"),
            UserManagementError::OldPasswordIncorrect => ApiError::old_password_incorrect(),
            UserManagementError::Validation(msg) => ApiError::validation(msg),
            UserManagementError::SecurityError(e) => ApiError::validation(e.to_string()),
            UserManagementError::DatabaseError(e) => ApiError::database(e.to_string()),
        }
    }
}

pub struct UserManagementService {
    pool: SqlitePool,
    encryption_service: EncryptionService,
}

impl UserManagementService {
    pub fn new(pool: SqlitePool) -> Self {
        let encryption_service = EncryptionService::new(pool.clone());
        Self { pool, encryption_service }
    }

    /// Get all users
    pub async fn list_users(&self) -> Result<Vec<UserInfo>, UserManagementError> {
        let rows = sqlx::query(sql_queries::USER_LIST_USERS)
            .fetch_all(&self.pool)
            .await?;

        let users = rows.into_iter().map(|row| {
            UserInfo {
                user_id: row.get(0),
                name: row.get(1),
                role: row.get(2),
                entry_dt: row.get(3),
                update_dt: row.get(4),
            }
        }).collect();

        Ok(users)
    }

    /// Get user by ID
    pub async fn get_user(&self, user_id: i64) -> Result<UserInfo, UserManagementError> {
        let row = sqlx::query(sql_queries::USER_GET_BY_ID)
            .bind(user_id)
            .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(UserInfo {
                user_id: row.get(0),
                name: row.get(1),
                role: row.get(2),
                entry_dt: row.get(3),
                update_dt: row.get(4),
            }),
            None => Err(UserManagementError::UserNotFound),
        }
    }

    /// Register a new general user
    pub async fn register_general_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<i64, UserManagementError> {
        validate_username_length(username)?;

        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let exists = sqlx::query(sql_queries::USER_CHECK_NAME_EXISTS)
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        let count: i64 = exists.get(0);
        if count > 0 {
            return Err(UserManagementError::DuplicateUsername);
        }

        let password_hash = hash_password(password)?;

        // Per-user Argon2 encryption salt (Fable-5 review #15).
        let encryption_salt = generate_encryption_salt();

        let result = sqlx::query(sql_queries::USER_GET_NEXT_ID)
            .fetch_one(&self.pool)
            .await?;
        let next_id: i64 = result.get(0);

        sqlx::query(sql_queries::USER_INSERT)
            .bind(next_id)
            .bind(username)
            .bind(password_hash)
            .bind(ROLE_USER)
            .bind(encryption_salt.as_slice())
            .bind(now)
            .execute(&self.pool)
            .await?;

        // Insert "Unspecified" master data for the new user
        self.insert_unspecified_data(next_id).await?;

        Ok(next_id)
    }

    /// Update the username only. The password path is deliberately not
    /// reachable from here (Fable-5 review #1) — every password change
    /// must go through [`change_password_in_tx`] so it is bundled with
    /// re-encryption in a single transaction and never persists a
    /// "new hash + old-key ciphertext" split state (Fable-5 review #5).
    async fn update_username(
        &self,
        user_id: i64,
        new_username: &str,
    ) -> Result<(), UserManagementError> {
        validate_username_length(new_username)?;
        let _user = self.get_user(user_id).await?;

        let exists = sqlx::query(sql_queries::USER_CHECK_NAME_EXISTS_EXCLUDING_ID)
            .bind(new_username)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        let count: i64 = exists.get(0);
        if count > 0 {
            return Err(UserManagementError::DuplicateUsername);
        }

        sqlx::query(sql_queries::USER_UPDATE_NAME)
            .bind(new_username)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Update the username inside an existing transaction. Used by the
    /// combined password-change path so the name UPDATE commits with
    /// the re-encryption and the `USERS.PAW` update as one atomic step.
    async fn update_username_in_tx<'c>(
        &self,
        tx: &mut sqlx::Transaction<'c, sqlx::Sqlite>,
        user_id: i64,
        new_username: &str,
    ) -> Result<(), UserManagementError> {
        validate_username_length(new_username)?;

        let row = sqlx::query(sql_queries::USER_CHECK_NAME_EXISTS_EXCLUDING_ID)
            .bind(new_username)
            .bind(user_id)
            .fetch_one(&mut **tx)
            .await?;
        let count: i64 = row.get(0);
        if count > 0 {
            return Err(UserManagementError::DuplicateUsername);
        }

        sqlx::query(sql_queries::USER_UPDATE_NAME)
            .bind(new_username)
            .bind(user_id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    /// Bundled password change: verify the caller's old password, then
    /// atomically re-encrypt every `ENCRYPTED_FIELDS` row for the user
    /// with the new key, write the new password hash, and — if the
    /// caller also asked to rename the account — update the username,
    /// all inside a single `BEGIN`/`COMMIT`. Any failure (I/O, decode,
    /// FK, process kill) rolls the transaction back so the DB is left
    /// with either (old key, old hash) or (new key, new hash), never
    /// the (new key, old hash) split that would strand every encrypted
    /// value behind a password the user could still log in with
    /// (Fable-5 review #5). The optional rename is included because
    /// splitting it into a second transaction would re-introduce a
    /// narrower version of the same window and force the frontend to
    /// issue two invokes for one save.
    async fn change_password_in_tx(
        &self,
        user_id: i64,
        old_password: &str,
        new_password: &str,
        new_username: Option<&str>,
    ) -> Result<(), UserManagementError> {
        // Validate the new username up front so we fail before touching
        // any encrypted rows on validation errors.
        if let Some(name) = new_username {
            validate_username_length(name)?;
        }

        // Verify old password against the current hash.
        let row = sqlx::query(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        let current_hash: String = row.get(0);
        if !verify_password(old_password, &current_hash)? {
            return Err(UserManagementError::OldPasswordIncorrect);
        }

        // Hash the new password outside the tx (Argon2 is CPU-heavy;
        // holding the tx open while it runs would extend lock windows
        // for no correctness benefit).
        let password_hash = hash_password(new_password)?;

        let mut tx = self.pool.begin().await?;

        // Re-encrypt every row with the new key inside this tx.
        self.encryption_service
            .re_encrypt_user_data_in_tx(&mut tx, user_id, old_password, new_password)
            .await
            .map_err(|e| UserManagementError::SecurityError(
                SecurityError::InvalidPassword(format!("Re-encryption failed: {}", e))
            ))?;

        // Update the password hash in the same tx.
        sqlx::query(sql_queries::USER_UPDATE_PASSWORD)
            .bind(password_hash)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        // Optionally update the username in the same tx.
        if let Some(name) = new_username {
            self.update_username_in_tx(&mut tx, user_id, name).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    /// Update general user (only for ROLE_USER). Rename only — password
    /// changes must go through
    /// [`update_general_user_with_password`].
    pub async fn update_general_user(
        &self,
        user_id: i64,
        new_username: Option<&str>,
    ) -> Result<(), UserManagementError> {
        let user = self.get_user(user_id).await?;
        if user.role != ROLE_USER {
            return Err(UserManagementError::InvalidRole);
        }

        if let Some(name) = new_username {
            self.update_username(user_id, name).await?;
        }
        Ok(())
    }

    /// Update general user with password change (verifies the old
    /// password and re-encrypts every encrypted field atomically).
    /// `new_password` is optional so callers can rename the account
    /// while still authenticating with the old password.
    pub async fn update_general_user_with_password(
        &self,
        user_id: i64,
        old_password: &str,
        new_username: Option<&str>,
        new_password: Option<&str>,
    ) -> Result<(), UserManagementError> {
        let user = self.get_user(user_id).await?;
        if user.role != ROLE_USER {
            return Err(UserManagementError::InvalidRole);
        }

        match new_password {
            Some(new_pwd) => {
                self.change_password_in_tx(user_id, old_password, new_pwd, new_username).await
            }
            None => {
                // No password change — still verify the old password so
                // this entry point cannot be used to rename an account
                // without proving knowledge of the current password.
                let row = sqlx::query(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
                    .bind(user_id)
                    .fetch_one(&self.pool)
                    .await?;
                let current_hash: String = row.get(0);
                if !verify_password(old_password, &current_hash)? {
                    return Err(UserManagementError::SecurityError(
                        SecurityError::InvalidPassword("Old password is incorrect".to_string())
                    ));
                }
                if let Some(name) = new_username {
                    self.update_username(user_id, name).await?;
                }
                Ok(())
            }
        }
    }

    /// Update admin user (only for ROLE_ADMIN). Rename only — password
    /// changes must go through
    /// [`update_admin_user_with_password`].
    pub async fn update_admin_user(
        &self,
        user_id: i64,
        new_username: Option<&str>,
    ) -> Result<(), UserManagementError> {
        let user = self.get_user(user_id).await?;
        if user.role != ROLE_ADMIN {
            return Err(UserManagementError::InvalidRole);
        }

        if let Some(name) = new_username {
            self.update_username(user_id, name).await?;
        }
        Ok(())
    }

    /// Update admin user with password change (see
    /// [`update_general_user_with_password`] for the atomicity
    /// contract).
    pub async fn update_admin_user_with_password(
        &self,
        user_id: i64,
        old_password: &str,
        new_username: Option<&str>,
        new_password: Option<&str>,
    ) -> Result<(), UserManagementError> {
        let user = self.get_user(user_id).await?;
        if user.role != ROLE_ADMIN {
            return Err(UserManagementError::InvalidRole);
        }

        match new_password {
            Some(new_pwd) => {
                self.change_password_in_tx(user_id, old_password, new_pwd, new_username).await
            }
            None => {
                let row = sqlx::query(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
                    .bind(user_id)
                    .fetch_one(&self.pool)
                    .await?;
                let current_hash: String = row.get(0);
                if !verify_password(old_password, &current_hash)? {
                    return Err(UserManagementError::SecurityError(
                        SecurityError::InvalidPassword("Old password is incorrect".to_string())
                    ));
                }
                if let Some(name) = new_username {
                    self.update_username(user_id, name).await?;
                }
                Ok(())
            }
        }
    }

    /// Delete a general user
    pub async fn delete_general_user(&self, user_id: i64) -> Result<(), UserManagementError> {
        let user = self.get_user(user_id).await?;
        
        if user.role == ROLE_ADMIN {
            return Err(UserManagementError::AdminUserCannotBeDeleted);
        }
        
        if user.role != ROLE_USER {
            return Err(UserManagementError::InvalidRole);
        }
        
        sqlx::query(sql_queries::USER_DELETE)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    /// Insert "Unspecified" master data for a new user
    async fn insert_unspecified_data(&self, user_id: i64) -> Result<(), UserManagementError> {
        // Insert "Unspecified" account
        sqlx::query(sql_queries::INSERT_UNSPECIFIED_ACCOUNT)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        // Insert "Unspecified" CATEGORY2 for each CATEGORY1
        sqlx::query(sql_queries::INSERT_UNSPECIFIED_CATEGORY2)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        // Insert "Unspecified" CATEGORY3 for each CATEGORY2
        sqlx::query(sql_queries::INSERT_UNSPECIFIED_CATEGORY3)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::database::{setup_test_db, create_test_admin};

    #[tokio::test]
    async fn test_register_general_user() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;
        
        let service = UserManagementService::new(pool.clone());
        let user_id = service.register_general_user("testuser", "password123")
            .await
            .unwrap();
        
        assert!(user_id > 1);
        
        let user = service.get_user(user_id).await.unwrap();
        assert_eq!(user.name, "testuser");
        assert_eq!(user.role, ROLE_USER);
    }

    #[tokio::test]
    async fn test_update_general_user() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());
        let user_id = service.register_general_user("testuser", "password_123456789")
            .await
            .unwrap();

        service.update_general_user(user_id, Some("newname"))
            .await
            .unwrap();

        let user = service.get_user(user_id).await.unwrap();
        assert_eq!(user.name, "newname");

        service.update_general_user_with_password(
            user_id,
            "password_123456789",
            None,
            Some("new_password_123456"),
        ).await.unwrap();

        let user = service.get_user(user_id).await.unwrap();
        assert!(user.update_dt.is_some());
    }

    #[tokio::test]
    async fn test_update_general_user_username_only() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());
        let user_id = service.register_general_user("testuser", "password_123456789")
            .await
            .unwrap();

        // Update username only, password remains unchanged
        service.update_general_user(user_id, Some("updateduser"))
            .await
            .unwrap();

        let user = service.get_user(user_id).await.unwrap();
        assert_eq!(user.name, "updateduser");
        assert_eq!(user.role, ROLE_USER);
        assert!(user.update_dt.is_some());
    }

    #[tokio::test]
    async fn test_update_general_user_password_only() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());
        let user_id = service.register_general_user("testuser", "password_123456789")
            .await
            .unwrap();

        // Update password only, username remains unchanged
        service.update_general_user_with_password(
            user_id,
            "password_123456789",
            None,
            Some("new_password_123456"),
        ).await.unwrap();

        let user = service.get_user(user_id).await.unwrap();
        assert_eq!(user.name, "testuser");
        assert_eq!(user.role, ROLE_USER);
        assert!(user.update_dt.is_some());

        // Verify new password works
        let row = sqlx::query(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        let password_hash: String = row.get(0);
        assert!(verify_password("new_password_123456", &password_hash).unwrap());
    }

    #[tokio::test]
    async fn test_update_general_user_username_and_password() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());
        let user_id = service.register_general_user("testuser", "password_123456789")
            .await
            .unwrap();

        // Update both username and password in one atomic call.
        service.update_general_user_with_password(
            user_id,
            "password_123456789",
            Some("superuser"),
            Some("super_password_123456"),
        ).await.unwrap();

        let user = service.get_user(user_id).await.unwrap();
        assert_eq!(user.name, "superuser");
        assert_eq!(user.role, ROLE_USER);
        assert!(user.update_dt.is_some());

        // Verify new password works
        let row = sqlx::query(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        let password_hash: String = row.get(0);
        assert!(verify_password("super_password_123456", &password_hash).unwrap());
    }

    #[tokio::test]
    async fn test_update_admin_user() {
        let pool = setup_test_db().await;
        let admin_id = create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());

        service.update_admin_user(admin_id, Some("superadmin"))
            .await
            .unwrap();

        let user = service.get_user(admin_id).await.unwrap();
        assert_eq!(user.name, "superadmin");
        assert_eq!(user.role, ROLE_ADMIN);
    }

    #[tokio::test]
    async fn test_update_admin_user_username_only() {
        let pool = setup_test_db().await;
        let admin_id = create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());

        // Update username only, password remains unchanged
        service.update_admin_user(admin_id, Some("newadmin"))
            .await
            .unwrap();

        let user = service.get_user(admin_id).await.unwrap();
        assert_eq!(user.name, "newadmin");
        assert_eq!(user.role, ROLE_ADMIN);
        assert!(user.update_dt.is_some());
    }

    #[tokio::test]
    async fn test_update_admin_user_password_only() {
        let pool = setup_test_db().await;
        let admin_id = create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());

        // Update password only, username remains unchanged
        service.update_admin_user_with_password(
            admin_id,
            "admin_password123456",
            None,
            Some("new_password_123456"),
        ).await.unwrap();

        let user = service.get_user(admin_id).await.unwrap();
        assert_eq!(user.name, "admin");
        assert_eq!(user.role, ROLE_ADMIN);
        assert!(user.update_dt.is_some());

        // Verify new password works
        let row = sqlx::query(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
            .bind(admin_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        let password_hash: String = row.get(0);
        assert!(verify_password("new_password_123456", &password_hash).unwrap());
    }

    #[tokio::test]
    async fn test_update_admin_user_username_and_password() {
        let pool = setup_test_db().await;
        let admin_id = create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());

        // Update both username and password
        service.update_admin_user_with_password(
            admin_id,
            "admin_password123456",
            Some("superadmin"),
            Some("super_password_123456"),
        ).await.unwrap();

        let user = service.get_user(admin_id).await.unwrap();
        assert_eq!(user.name, "superadmin");
        assert_eq!(user.role, ROLE_ADMIN);
        assert!(user.update_dt.is_some());

        // Verify new password works
        let row = sqlx::query(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
            .bind(admin_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        let password_hash: String = row.get(0);
        assert!(verify_password("super_password_123456", &password_hash).unwrap());
    }

    #[tokio::test]
    async fn test_delete_general_user() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;
        
        let service = UserManagementService::new(pool.clone());
        let user_id = service.register_general_user("testuser", "password123")
            .await
            .unwrap();
        
        service.delete_general_user(user_id).await.unwrap();
        
        let result = service.get_user(user_id).await;
        assert!(matches!(result, Err(UserManagementError::UserNotFound)));
    }

    #[tokio::test]
    async fn test_cannot_delete_admin_user() {
        let pool = setup_test_db().await;
        let admin_id = create_test_admin(&pool, "admin", "admin_password123456").await;
        
        let service = UserManagementService::new(pool.clone());
        
        let result = service.delete_general_user(admin_id).await;
        assert!(matches!(result, Err(UserManagementError::AdminUserCannotBeDeleted)));
    }

    #[tokio::test]
    async fn test_duplicate_username() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;
        
        let service = UserManagementService::new(pool.clone());
        service.register_general_user("testuser", "password123")
            .await
            .unwrap();
        
        let result = service.register_general_user("testuser", "password456").await;
        assert!(matches!(result, Err(UserManagementError::DuplicateUsername)));
    }

    #[tokio::test]
    async fn test_list_users() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());
        service.register_general_user("user1", "password1").await.unwrap();
        service.register_general_user("user2", "password2").await.unwrap();

        let users = service.list_users().await.unwrap();
        assert_eq!(users.len(), 3);
    }

    // Issue #37 Phase 2-3 — bounded-field length checks must count
    // characters (not bytes). Japanese is 3 bytes per char in UTF-8.

    #[tokio::test]
    async fn test_register_general_user_accepts_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());
        let name = "あ".repeat(consts::MAX_NAME_LEN);
        let result = service.register_general_user(&name, "password123456").await;
        assert!(result.is_ok(), "expected MAX_NAME_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_register_general_user_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());
        let name = "あ".repeat(consts::MAX_NAME_LEN + 1);
        let err = service.register_general_user(&name, "password123456").await.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }

    /// Fable-5 review #1/#5 — a password change with the wrong "current
    /// password" must be rejected with `OldPasswordIncorrect` before any
    /// state changes. The password hash and update timestamp must remain
    /// untouched so the caller can retry, and so a mistyped current
    /// password can never advance the rest of the flow (which would
    /// leave `re_encrypt_user_data_in_tx` computing a new key from
    /// garbage input, then stamping the correct new hash next to
    /// stranded ciphertext).
    #[tokio::test]
    async fn test_update_general_user_with_password_rejects_wrong_old_password() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());
        let user_id = service.register_general_user("testuser", "password_123456789")
            .await
            .unwrap();

        // Snapshot the pre-change hash so we can assert it did not move.
        let before: String = sqlx::query_scalar(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        let err = service.update_general_user_with_password(
            user_id,
            "wrong_old_password!!",
            Some("would-not-take"),
            Some("would_not_take_1234"),
        ).await.unwrap_err();
        assert!(matches!(err, UserManagementError::OldPasswordIncorrect));

        // Hash unchanged — the rejected attempt did not partially commit.
        let after: String = sqlx::query_scalar(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(before, after);

        // Username unchanged — the rename bundled with the password
        // change did not sneak through either.
        let user = service.get_user(user_id).await.unwrap();
        assert_eq!(user.name, "testuser");
    }

    /// Admin-side counterpart to
    /// [`test_update_general_user_with_password_rejects_wrong_old_password`].
    #[tokio::test]
    async fn test_update_admin_user_with_password_rejects_wrong_old_password() {
        let pool = setup_test_db().await;
        let admin_id = create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());

        let before: String = sqlx::query_scalar(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
            .bind(admin_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        let err = service.update_admin_user_with_password(
            admin_id,
            "wrong_admin_password!",
            None,
            Some("would_not_take_1234"),
        ).await.unwrap_err();
        assert!(matches!(err, UserManagementError::OldPasswordIncorrect));

        let after: String = sqlx::query_scalar(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
            .bind(admin_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(before, after);
    }

    #[tokio::test]
    async fn test_update_general_user_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;
        create_test_admin(&pool, "admin", "admin_password123456").await;

        let service = UserManagementService::new(pool.clone());
        let user_id = service.register_general_user("testuser", "password123456")
            .await.unwrap();

        let name = "あ".repeat(consts::MAX_NAME_LEN + 1);
        let err = service.update_general_user(user_id, Some(&name)).await.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }
}
