use sqlx::sqlite::SqlitePool;
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME, SQL_INIT_FILE_PATH};
use crate::sql_queries;

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
        let pool = SqlitePool::connect(&db_url).await?;
        
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
}
