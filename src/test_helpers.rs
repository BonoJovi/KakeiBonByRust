/// Common test helper functions
/// 
/// This module provides shared test utilities used across different test modules,
/// similar to the validation-helpers.js pattern in the JavaScript tests.

#[cfg(test)]
pub mod validation {
    /// Test result structure for validation tests
    #[derive(Debug, PartialEq)]
    pub struct ValidationTestResult {
        pub passed: bool,
        pub message: String,
    }

    /// Helper function to check if a password validation result matches expected outcome
    pub fn check_password_validation(
        password: &str,
        expected_valid: bool,
        expected_message: Option<&str>,
    ) -> ValidationTestResult {
        use crate::validation::validate_password;
        
        let result = validate_password(password);
        
        match (result, expected_valid) {
            (Ok(_), true) => ValidationTestResult {
                passed: true,
                message: format!("Password '{}' correctly validated", password),
            },
            (Err(err), false) => {
                if let Some(expected_msg) = expected_message {
                    if err == expected_msg {
                        ValidationTestResult {
                            passed: true,
                            message: format!("Password '{}' correctly rejected with expected message", password),
                        }
                    } else {
                        ValidationTestResult {
                            passed: false,
                            message: format!(
                                "Expected message '{}', got '{}'",
                                expected_msg, err
                            ),
                        }
                    }
                } else {
                    ValidationTestResult {
                        passed: true,
                        message: format!("Password '{}' correctly rejected", password),
                    }
                }
            }
            (Ok(_), false) => ValidationTestResult {
                passed: false,
                message: format!("Password '{}' should have been rejected", password),
            },
            (Err(err), true) => ValidationTestResult {
                passed: false,
                message: format!("Password '{}' should have been accepted, but got error: {}", password, err),
            },
        }
    }

    /// Helper function to check password confirmation validation
    pub fn check_password_confirmation(
        password: &str,
        confirmation: &str,
        expected_valid: bool,
    ) -> ValidationTestResult {
        use crate::validation::validate_password_confirmation;
        
        let result = validate_password_confirmation(password, confirmation);
        
        match (result, expected_valid) {
            (Ok(_), true) => ValidationTestResult {
                passed: true,
                message: "Passwords correctly matched".to_string(),
            },
            (Err(_), false) => ValidationTestResult {
                passed: true,
                message: "Passwords correctly identified as not matching".to_string(),
            },
            (Ok(_), false) => ValidationTestResult {
                passed: false,
                message: "Passwords should not have matched".to_string(),
            },
            (Err(err), true) => ValidationTestResult {
                passed: false,
                message: format!("Passwords should have matched, but got error: {}", err),
            },
        }
    }
}

#[cfg(test)]
pub mod database {
    use sqlx::sqlite::SqlitePool;
    use crate::security::hash_password;
    use crate::consts::{ROLE_ADMIN, ROLE_USER};

    /// Setup an in-memory test database with USERS table
    pub async fn setup_test_db() -> SqlitePool {
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

    /// Create a test admin user in the database
    pub async fn create_test_admin(pool: &SqlitePool, username: &str, password: &str) -> i64 {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let password_hash = hash_password(password).unwrap();
        
        let result = sqlx::query(
            "INSERT INTO USERS (NAME, PAW, ROLE, ENTRY_DT) VALUES (?, ?, ?, ?)"
        )
        .bind(username)
        .bind(password_hash)
        .bind(ROLE_ADMIN)
        .bind(now)
        .execute(pool)
        .await
        .unwrap();
        
        result.last_insert_rowid()
    }

    /// Create a test general user in the database
    pub async fn create_test_user(pool: &SqlitePool, username: &str, password: &str) -> i64 {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let password_hash = hash_password(password).unwrap();
        
        let result = sqlx::query(
            "INSERT INTO USERS (NAME, PAW, ROLE, ENTRY_DT) VALUES (?, ?, ?, ?)"
        )
        .bind(username)
        .bind(password_hash)
        .bind(ROLE_USER)
        .bind(now)
        .execute(pool)
        .await
        .unwrap();
        
        result.last_insert_rowid()
    }
}
