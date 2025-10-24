use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use crate::security::{hash_password, verify_password, SecurityError};
use crate::consts::{ROLE_ADMIN, ROLE_USER};
use super::encryption::EncryptionService;

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
        }
    }
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
        let rows = sqlx::query(
            r#"
            SELECT USER_ID, NAME, ROLE, ENTRY_DT, UPDATE_DT
            FROM USERS
            ORDER BY USER_ID
            "#
        )
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
        let row = sqlx::query(
            r#"
            SELECT USER_ID, NAME, ROLE, ENTRY_DT, UPDATE_DT
            FROM USERS
            WHERE USER_ID = ?
            "#
        )
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
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let exists = sqlx::query("SELECT COUNT(*) as count FROM USERS WHERE NAME = ?")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        let count: i64 = exists.get(0);
        if count > 0 {
            return Err(UserManagementError::DuplicateUsername);
        }

        let password_hash = hash_password(password)?;
        
        let result = sqlx::query("SELECT COALESCE(MAX(USER_ID), 0) + 1 as next_id FROM USERS")
            .fetch_one(&self.pool)
            .await?;
        let next_id: i64 = result.get(0);
        
        sqlx::query(
            r#"
            INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(next_id)
        .bind(username)
        .bind(password_hash)
        .bind(ROLE_USER)
        .bind(now)
        .execute(&self.pool)
        .await?;
        
        Ok(next_id)
    }

    /// Update user (username and/or password)
    /// When updating password, old_password must be provided for re-encryption
    async fn update_user_internal(
        &self,
        user_id: i64,
        new_username: Option<&str>,
        new_password: Option<&str>,
        old_password: Option<&str>,
    ) -> Result<(), UserManagementError> {
        let _user = self.get_user(user_id).await?;
        
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        if let Some(password) = new_password {
            // Re-encrypt encrypted fields if old password is provided
            if let Some(old_pwd) = old_password {
                self.re_encrypt_user_data(user_id, old_pwd, password).await?;
            }
            
            let password_hash = hash_password(password)?;
            
            sqlx::query(
                r#"
                UPDATE USERS
                SET PAW = ?, UPDATE_DT = ?
                WHERE USER_ID = ?
                "#
            )
            .bind(password_hash)
            .bind(&now)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        }
        
        if let Some(username) = new_username {
            let exists = sqlx::query(
                "SELECT COUNT(*) as count FROM USERS WHERE NAME = ? AND USER_ID != ?"
            )
            .bind(username)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
            let count: i64 = exists.get(0);
            if count > 0 {
                return Err(UserManagementError::DuplicateUsername);
            }
            
            sqlx::query(
                r#"
                UPDATE USERS
                SET NAME = ?, UPDATE_DT = ?
                WHERE USER_ID = ?
                "#
            )
            .bind(username)
            .bind(&now)
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }

    /// Update user (username and/or password) - simplified interface without re-encryption
    pub async fn update_user(
        &self,
        user_id: i64,
        new_username: Option<&str>,
        new_password: Option<&str>,
    ) -> Result<(), UserManagementError> {
        self.update_user_internal(user_id, new_username, new_password, None).await
    }

    /// Update general user (only for ROLE_USER)
    pub async fn update_general_user(
        &self,
        user_id: i64,
        new_username: Option<&str>,
        new_password: Option<&str>,
    ) -> Result<(), UserManagementError> {
        let user = self.get_user(user_id).await?;
        if user.role != ROLE_USER {
            return Err(UserManagementError::InvalidRole);
        }
        
        self.update_user(user_id, new_username, new_password).await
    }

    /// Update general user with old password verification (for re-encryption)
    pub async fn update_general_user_with_password(
        &self,
        user_id: i64,
        old_password: &str,
        new_username: Option<&str>,
        new_password: Option<&str>,
    ) -> Result<(), UserManagementError> {
        // Get user and verify role
        let user = self.get_user(user_id).await?;
        if user.role != ROLE_USER {
            return Err(UserManagementError::InvalidRole);
        }

        // Verify old password
        let row = sqlx::query("SELECT PAW FROM USERS WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        let current_hash: String = row.get(0);
        
        if !verify_password(old_password, &current_hash)? {
            return Err(UserManagementError::SecurityError(
                SecurityError::InvalidPassword("Old password is incorrect".to_string())
            ));
        }
        
        self.update_user_internal(user_id, new_username, new_password, Some(old_password)).await
    }

    /// Update admin user (only for ROLE_ADMIN)
    pub async fn update_admin_user(
        &self,
        user_id: i64,
        new_username: Option<&str>,
        new_password: Option<&str>,
    ) -> Result<(), UserManagementError> {
        let user = self.get_user(user_id).await?;
        if user.role != ROLE_ADMIN {
            return Err(UserManagementError::InvalidRole);
        }
        
        self.update_user(user_id, new_username, new_password).await
    }

    /// Update admin user with old password verification (for re-encryption)
    pub async fn update_admin_user_with_password(
        &self,
        user_id: i64,
        old_password: &str,
        new_username: Option<&str>,
        new_password: Option<&str>,
    ) -> Result<(), UserManagementError> {
        // Get user and verify role
        let user = self.get_user(user_id).await?;
        if user.role != ROLE_ADMIN {
            return Err(UserManagementError::InvalidRole);
        }

        // Verify old password
        let row = sqlx::query("SELECT PAW FROM USERS WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        let current_hash: String = row.get(0);
        
        if !verify_password(old_password, &current_hash)? {
            return Err(UserManagementError::SecurityError(
                SecurityError::InvalidPassword("Old password is incorrect".to_string())
            ));
        }
        
        self.update_user_internal(user_id, new_username, new_password, Some(old_password)).await
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
        
        sqlx::query("DELETE FROM USERS WHERE USER_ID = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    /// Re-encrypt all encrypted fields for a user with new password
    async fn re_encrypt_user_data(
        &self,
        user_id: i64,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), UserManagementError> {
        self.encryption_service
            .re_encrypt_user_data(user_id, old_password, new_password)
            .await
            .map_err(|e| UserManagementError::SecurityError(
                SecurityError::InvalidPassword(format!("Re-encryption failed: {}", e))
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS USERS (
                USER_ID INTEGER NOT NULL,
                NAME VARCHAR(128) NOT NULL UNIQUE,
                PAW VARCHAR(128) NOT NULL,
                ROLE INTEGER NOT NULL,
                ENTRY_DT DATETIME NOT NULL,
                UPDATE_DT DATETIME,
                PRIMARY KEY(USER_ID)
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();
        
        pool
    }

    async fn create_admin_user(pool: &SqlitePool) {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let password_hash = hash_password("admin123").unwrap();
        
        sqlx::query(
            "INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(1)
        .bind("admin")
        .bind(password_hash)
        .bind(ROLE_ADMIN)
        .bind(now)
        .execute(pool)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_register_general_user() {
        let pool = setup_test_db().await;
        create_admin_user(&pool).await;
        
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
        create_admin_user(&pool).await;
        
        let service = UserManagementService::new(pool.clone());
        let user_id = service.register_general_user("testuser", "password123")
            .await
            .unwrap();
        
        service.update_general_user(user_id, Some("newname"), None)
            .await
            .unwrap();
        
        let user = service.get_user(user_id).await.unwrap();
        assert_eq!(user.name, "newname");
        
        service.update_general_user(user_id, None, Some("newpassword"))
            .await
            .unwrap();
        
        let user = service.get_user(user_id).await.unwrap();
        assert!(user.update_dt.is_some());
    }

    #[tokio::test]
    async fn test_update_admin_user() {
        let pool = setup_test_db().await;
        create_admin_user(&pool).await;
        
        let service = UserManagementService::new(pool.clone());
        
        service.update_admin_user(1, Some("superadmin"), None)
            .await
            .unwrap();
        
        let user = service.get_user(1).await.unwrap();
        assert_eq!(user.name, "superadmin");
        assert_eq!(user.role, ROLE_ADMIN);
    }

    #[tokio::test]
    async fn test_delete_general_user() {
        let pool = setup_test_db().await;
        create_admin_user(&pool).await;
        
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
        create_admin_user(&pool).await;
        
        let service = UserManagementService::new(pool.clone());
        
        let result = service.delete_general_user(1).await;
        assert!(matches!(result, Err(UserManagementError::AdminUserCannotBeDeleted)));
    }

    #[tokio::test]
    async fn test_duplicate_username() {
        let pool = setup_test_db().await;
        create_admin_user(&pool).await;
        
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
        create_admin_user(&pool).await;
        
        let service = UserManagementService::new(pool.clone());
        service.register_general_user("user1", "password1").await.unwrap();
        service.register_general_user("user2", "password2").await.unwrap();
        
        let users = service.list_users().await.unwrap();
        assert_eq!(users.len(), 3);
    }
}
