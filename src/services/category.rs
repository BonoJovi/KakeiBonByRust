use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use crate::sql_queries;

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
        let count: i64 = sqlx::query_scalar(sql_queries::CATEGORY_COUNT_BY_USER)
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
        let rows = sqlx::query_as::<_, Category1>(sql_queries::CATEGORY1_LIST)
            .bind(lang_code)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        
        Ok(rows)
    }
    
    /// Get category tree with internationalization
    pub async fn get_category_tree(&self, user_id: i64, lang_code: &str) -> Result<serde_json::Value, CategoryError> {
        use serde_json::json;
        
        // Get all category1
        let cat1_rows = sqlx::query(sql_queries::CATEGORY1_TREE)
            .bind(lang_code)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        
        let mut categories = Vec::new();
        
        for row in cat1_rows {
            let cat1_code: String = row.get("CATEGORY1_CODE");
            let cat1_name: String = row.get("name");
            let cat1_order: i64 = row.get("DISPLAY_ORDER");
            
            // Get category2 for this category1
            let cat2_rows = sqlx::query(sql_queries::CATEGORY2_TREE)
                .bind(lang_code)
                .bind(user_id)
                .bind(&cat1_code)
                .fetch_all(&self.pool)
                .await?;
            
            let mut cat2_list = Vec::new();
            
            for cat2_row in cat2_rows {
                let cat2_code: String = cat2_row.get("CATEGORY2_CODE");
                let cat2_name: String = cat2_row.get("name");
                let cat2_order: i64 = cat2_row.get("DISPLAY_ORDER");
                
                // Get category3 for this category2
                let cat3_rows = sqlx::query(sql_queries::CATEGORY3_TREE)
                    .bind(lang_code)
                    .bind(user_id)
                    .bind(&cat1_code)
                    .bind(&cat2_code)
                    .fetch_all(&self.pool)
                    .await?;
                
                let cat3_list: Vec<_> = cat3_rows.iter().map(|row| {
                    json!({
                        "category3_code": row.get::<String, _>("CATEGORY3_CODE"),
                        "category3_name_i18n": row.get::<String, _>("name"),
                        "display_order": row.get::<i64, _>("DISPLAY_ORDER")
                    })
                }).collect();
                
                cat2_list.push(json!({
                    "category2": {
                        "category2_code": cat2_code,
                        "category2_name_i18n": cat2_name,
                        "display_order": cat2_order
                    },
                    "children": cat3_list
                }));
            }
            
            categories.push(json!({
                "category1": {
                    "category1_code": cat1_code,
                    "category1_name_i18n": cat1_name,
                    "display_order": cat1_order
                },
                "children": cat2_list
            }));
        }
        
        Ok(json!(categories))
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
