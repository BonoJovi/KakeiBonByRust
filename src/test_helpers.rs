/// Common test helper functions
///
/// This module provides shared test utilities used across different test modules,
/// similar to the validation-helpers.js pattern in the JavaScript tests.

#[cfg(test)]
#[allow(dead_code)]
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
#[allow(dead_code)]
pub mod database {
    use sqlx::sqlite::SqlitePool;
    use crate::security::hash_password;
    use crate::consts::{ROLE_ADMIN, ROLE_USER};
    use crate::db::connect_db;

    /// Test database URL for in-memory SQLite
    pub const TEST_DB_URL: &str = "sqlite::memory:";

    /// Initialize an in-memory SQLite database for testing
    pub async fn init_db(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
        connect_db(db_url).await
    }

    /// Setup an in-memory test database with all tables from dbaccess.sql
    pub async fn setup_test_db() -> SqlitePool {
        let pool = connect_db(TEST_DB_URL).await.unwrap();
        
        // Read and execute DDL from dbaccess.sql
        let sql_content = std::fs::read_to_string("res/sql/dbaccess.sql")
            .expect("Failed to read dbaccess.sql");
        
        // Remove comments and split by semicolon
        let mut current_statement = String::new();
        for line in sql_content.lines() {
            let trimmed = line.trim();
            // Skip comment-only lines
            if trimmed.starts_with("--") || trimmed.is_empty() {
                continue;
            }
            // Remove inline comments
            let line_without_comment = if let Some(pos) = line.find("--") {
                &line[..pos]
            } else {
                line
            };
            current_statement.push_str(line_without_comment);
            current_statement.push(' ');
            
            // If line ends with semicolon, execute the statement
            if line_without_comment.trim().ends_with(';') {
                let stmt = current_statement.trim().trim_end_matches(';').trim();
                if !stmt.is_empty() {
                    sqlx::query(stmt)
                        .execute(&pool)
                        .await
                        .unwrap_or_else(|e| panic!("Failed to execute SQL: {}\nError: {}", stmt, e));
                }
                current_statement.clear();
            }
        }
        
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
