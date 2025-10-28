use sqlx::sqlite::SqlitePool;

#[derive(Debug)]
pub enum CategoryError {
    DatabaseError(sqlx::Error),
}

impl std::fmt::Display for CategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for CategoryError {}

impl From<sqlx::Error> for CategoryError {
    fn from(err: sqlx::Error) -> Self {
        CategoryError::DatabaseError(err)
    }
}

pub struct CategoryService {
    pool: SqlitePool,
}

impl CategoryService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    /// Initialize default categories for a new user
    /// This will be called when a general user is registered
    pub async fn initialize_user_categories(&self, user_id: i64) -> Result<(), CategoryError> {
        // Check if categories already exist for this user
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM CATEGORY1 WHERE USER_ID = ?"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        
        if count > 0 {
            // Categories already initialized
            return Ok(());
        }
        
        // Start transaction
        let tx = self.pool.begin().await?;
        
        // NOTE: Default category data will be inserted here
        // This will be populated from existing SQL data later
        // For now, just create the structure
        
        // Example structure (to be replaced with actual data):
        // 1. Insert CATEGORY1 records
        // 2. Insert CATEGORY1_I18N records (en, ja)
        // 3. Insert CATEGORY2 records
        // 4. Insert CATEGORY2_I18N records
        // 5. Insert CATEGORY3 records
        // 6. Insert CATEGORY3_I18N records
        
        tx.commit().await?;
        
        Ok(())
    }
    
    /// Get all category1 for a user
    pub async fn get_category1_list(&self, user_id: i64, lang_code: &str) -> Result<Vec<Category1>, CategoryError> {
        let rows = sqlx::query_as::<_, Category1>(
            r#"
            SELECT 
                c.USER_ID,
                c.CATEGORY1_CODE,
                c.DISPLAY_ORDER,
                COALESCE(i18n.CATEGORY1_NAME_I18N, c.CATEGORY1_NAME) as CATEGORY1_NAME,
                c.IS_DISABLED
            FROM CATEGORY1 c
            LEFT JOIN CATEGORY1_I18N i18n 
                ON c.USER_ID = i18n.USER_ID 
                AND c.CATEGORY1_CODE = i18n.CATEGORY1_CODE 
                AND i18n.LANG_CODE = ?
            WHERE c.USER_ID = ? AND c.IS_DISABLED = 0
            ORDER BY c.DISPLAY_ORDER
            "#
        )
        .bind(lang_code)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Category1 {
    #[sqlx(rename = "USER_ID")]
    pub user_id: i64,
    #[sqlx(rename = "CATEGORY1_CODE")]
    pub category1_code: String,
    #[sqlx(rename = "DISPLAY_ORDER")]
    pub display_order: i64,
    #[sqlx(rename = "CATEGORY1_NAME")]
    pub category1_name: String,
    #[sqlx(rename = "IS_DISABLED")]
    pub is_disabled: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        // Create tables
        sqlx::query(
            r#"
            CREATE TABLE CATEGORY1 (
                USER_ID INTEGER NOT NULL,
                CATEGORY1_CODE VARCHAR(64) NOT NULL,
                DISPLAY_ORDER INTEGER NOT NULL,
                CATEGORY1_NAME VARCHAR(128) NOT NULL,
                IS_DISABLED INTEGER NOT NULL DEFAULT 0,
                ENTRY_DT DATETIME NOT NULL,
                UPDATE_DT DATETIME,
                PRIMARY KEY(USER_ID, CATEGORY1_CODE)
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();
        
        sqlx::query(
            r#"
            CREATE TABLE CATEGORY1_I18N (
                USER_ID INTEGER NOT NULL,
                CATEGORY1_CODE VARCHAR(64) NOT NULL,
                LANG_CODE VARCHAR(10) NOT NULL,
                CATEGORY1_NAME_I18N VARCHAR(256) NOT NULL,
                ENTRY_DT DATETIME NOT NULL,
                UPDATE_DT DATETIME,
                PRIMARY KEY(USER_ID, CATEGORY1_CODE, LANG_CODE)
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();
        
        pool
    }
    
    #[tokio::test]
    async fn test_initialize_user_categories() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        
        let user_id = 1;
        service.initialize_user_categories(user_id).await.unwrap();
        
        // Check that calling twice doesn't cause errors
        service.initialize_user_categories(user_id).await.unwrap();
    }
    
    #[tokio::test]
    async fn test_get_category1_list() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        
        let user_id = 1;
        
        // Insert test data
        sqlx::query(
            "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, DISPLAY_ORDER, CATEGORY1_NAME, IS_DISABLED, ENTRY_DT) VALUES (?, ?, ?, ?, ?, datetime('now'))"
        )
        .bind(user_id)
        .bind("FOOD")
        .bind(1)
        .bind("Food")
        .bind(0)
        .execute(&pool)
        .await
        .unwrap();
        
        // Insert Japanese translation
        sqlx::query(
            "INSERT INTO CATEGORY1_I18N (USER_ID, CATEGORY1_CODE, LANG_CODE, CATEGORY1_NAME_I18N, ENTRY_DT) VALUES (?, ?, ?, ?, datetime('now'))"
        )
        .bind(user_id)
        .bind("FOOD")
        .bind("ja")
        .bind("食費")
        .execute(&pool)
        .await
        .unwrap();
        
        // Get with Japanese
        let categories_ja = service.get_category1_list(user_id, "ja").await.unwrap();
        assert_eq!(categories_ja.len(), 1);
        assert_eq!(categories_ja[0].category1_name, "食費");
        
        // Get with English (should fallback to default)
        let categories_en = service.get_category1_list(user_id, "en").await.unwrap();
        assert_eq!(categories_en.len(), 1);
        assert_eq!(categories_en[0].category1_name, "Food");
    }
}
