use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;
use crate::consts::LANG_DEFAULT;
use crate::sql_queries;

#[derive(Debug)]
pub enum I18nError {
    DatabaseError(sqlx::Error),
    ResourceNotFound(String),
}

impl std::fmt::Display for I18nError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            I18nError::DatabaseError(e) => write!(f, "Database error: {}", e),
            I18nError::ResourceNotFound(key) => write!(f, "Resource not found: {}", key),
        }
    }
}

impl std::error::Error for I18nError {}

impl From<sqlx::Error> for I18nError {
    fn from(err: sqlx::Error) -> Self {
        I18nError::DatabaseError(err)
    }
}

pub struct I18nService {
    pool: SqlitePool,
}

impl I18nService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    /// Get a resource value by key and language code
    pub async fn get(&self, key: &str, lang_code: &str) -> Result<String, I18nError> {
        let result = sqlx::query_scalar::<_, String>(
            "SELECT RESOURCE_VALUE FROM I18N_RESOURCES WHERE RESOURCE_KEY = ? AND LANG_CODE = ?"
        )
        .bind(key)
        .bind(lang_code)
        .fetch_optional(&self.pool)
        .await?;
        
        match result {
            Some(value) => Ok(value),
            None => {
                // Fallback to default language
                let fallback = sqlx::query_scalar::<_, String>(
                    "SELECT RESOURCE_VALUE FROM I18N_RESOURCES WHERE RESOURCE_KEY = ? AND LANG_CODE = ?"
                )
                .bind(key)
                .bind(LANG_DEFAULT)
                .fetch_optional(&self.pool)
                .await?;
                
                fallback.ok_or_else(|| I18nError::ResourceNotFound(key.to_string()))
            }
        }
    }
    
    /// Get a resource value with parameter substitution
    /// Example: get_with_params("msg.lang_changed", "ja", &["日本語"]) 
    /// -> "言語を日本語に変更しました。"
    pub async fn get_with_params(&self, key: &str, lang_code: &str, params: &[&str]) -> Result<String, I18nError> {
        let mut template = self.get(key, lang_code).await?;
        
        for (i, param) in params.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            template = template.replace(&placeholder, param);
        }
        
        Ok(template)
    }
    
    /// Get all resources for a specific language
    pub async fn get_all(&self, lang_code: &str) -> Result<HashMap<String, String>, I18nError> {
        let rows = sqlx::query_as::<_, (String, String)>(
            "SELECT RESOURCE_KEY, RESOURCE_VALUE FROM I18N_RESOURCES WHERE LANG_CODE = ?"
        )
        .bind(lang_code)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().collect())
    }
    
    /// Get resources by category
    pub async fn get_by_category(&self, lang_code: &str, category: &str) -> Result<HashMap<String, String>, I18nError> {
        let rows = sqlx::query_as::<_, (String, String)>(
            "SELECT RESOURCE_KEY, RESOURCE_VALUE FROM I18N_RESOURCES WHERE LANG_CODE = ? AND CATEGORY = ?"
        )
        .bind(lang_code)
        .bind(category)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().collect())
    }
    
    /// Get list of available languages
    pub async fn get_available_languages(&self) -> Result<Vec<String>, I18nError> {
        let rows = sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT LANG_CODE FROM I18N_RESOURCES ORDER BY LANG_CODE"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        // Create table
        sqlx::query(
            r#"
            CREATE TABLE I18N_RESOURCES (
                RESOURCE_ID INTEGER NOT NULL,
                RESOURCE_KEY VARCHAR(256) NOT NULL,
                LANG_CODE VARCHAR(10) NOT NULL,
                RESOURCE_VALUE TEXT NOT NULL,
                CATEGORY VARCHAR(64),
                DESCRIPTION VARCHAR(512),
                ENTRY_DT DATETIME NOT NULL,
                UPDATE_DT DATETIME,
                PRIMARY KEY(RESOURCE_ID),
                UNIQUE(RESOURCE_KEY, LANG_CODE)
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();
        
        // Insert test data
        sqlx::query(
            "INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (?, ?, ?, ?, ?, datetime('now'))"
        )
        .bind(1)
        .bind("menu.file")
        .bind("en")
        .bind("File")
        .bind("menu")
        .execute(&pool)
        .await
        .unwrap();
        
        sqlx::query(
            "INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (?, ?, ?, ?, ?, datetime('now'))"
        )
        .bind(2)
        .bind("menu.file")
        .bind("ja")
        .bind("ファイル")
        .bind("menu")
        .execute(&pool)
        .await
        .unwrap();
        
        sqlx::query(
            "INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (?, ?, ?, ?, ?, datetime('now'))"
        )
        .bind(3)
        .bind("msg.lang_changed")
        .bind("ja")
        .bind("言語を{0}に変更しました。")
        .bind("message")
        .execute(&pool)
        .await
        .unwrap();
        
        // Insert error message test data
        let error_messages = vec![
            (701, "error.category_name_required", "en", "Please enter at least one name (Japanese or English)", "error"),
            (702, "error.category_name_required", "ja", "名前を少なくとも1つ入力してください（日本語または英語）", "error"),
            (703, "error.category_save_failed", "en", "Failed to save", "error"),
            (704, "error.category_save_failed", "ja", "保存に失敗しました", "error"),
            (705, "error.category_move_failed", "en", "Failed to move category", "error"),
            (706, "error.category_move_failed", "ja", "費目の移動に失敗しました", "error"),
            (707, "error.category_load_failed", "en", "Failed to load categories", "error"),
            (708, "error.category_load_failed", "ja", "費目の読み込みに失敗しました", "error"),
            (709, "error.language_change_failed", "en", "Failed to change language", "error"),
            (710, "error.language_change_failed", "ja", "言語の変更に失敗しました", "error"),
            (711, "error.font_size_change_failed", "en", "Failed to change font size", "error"),
            (712, "error.font_size_change_failed", "ja", "フォントサイズの変更に失敗しました", "error"),
            (713, "error.font_size_apply_failed", "en", "Failed to apply font size", "error"),
            (714, "error.font_size_apply_failed", "ja", "フォントサイズの適用に失敗しました", "error"),
            (715, "validation.required", "en", "Please fill out this field", "validation"),
            (716, "validation.required", "ja", "このフィールドを入力してください", "validation"),
        ];
        
        for (id, key, lang, value, category) in error_messages {
            sqlx::query(
                "INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT) VALUES (?, ?, ?, ?, ?, datetime('now'))"
            )
            .bind(id)
            .bind(key)
            .bind(lang)
            .bind(value)
            .bind(category)
            .execute(&pool)
            .await
            .unwrap();
        }
        
        pool
    }
    
    #[tokio::test]
    async fn test_get_resource() {
        let pool = setup_test_db().await;
        let service = I18nService::new(pool);
        
        let value_ja = service.get("menu.file", "ja").await.unwrap();
        assert_eq!(value_ja, "ファイル");
        
        let value_en = service.get("menu.file", "en").await.unwrap();
        assert_eq!(value_en, "File");
    }
    
    #[tokio::test]
    async fn test_get_with_params() {
        let pool = setup_test_db().await;
        let service = I18nService::new(pool);
        
        let message = service.get_with_params("msg.lang_changed", "ja", &["日本語"]).await.unwrap();
        assert_eq!(message, "言語を日本語に変更しました。");
    }
    
    #[tokio::test]
    async fn test_fallback_to_default() {
        let pool = setup_test_db().await;
        let service = I18nService::new(pool);
        
        // Request non-existent language, should fallback to ja
        let value = service.get("menu.file", "fr").await.unwrap();
        assert_eq!(value, "ファイル");
    }
    
    #[tokio::test]
    async fn test_get_by_category() {
        let pool = setup_test_db().await;
        let service = I18nService::new(pool);
        
        let menu_resources = service.get_by_category("ja", "menu").await.unwrap();
        assert!(menu_resources.contains_key("menu.file"));
        assert_eq!(menu_resources.get("menu.file").unwrap(), "ファイル");
    }
    
    #[tokio::test]
    async fn test_error_messages_exist() {
        let pool = setup_test_db().await;
        let service = I18nService::new(pool);
        
        // Test category management error messages
        let error_keys = vec![
            "error.category_name_required",
            "error.category_save_failed",
            "error.category_move_failed",
            "error.category_load_failed",
        ];
        
        for key in error_keys {
            // Test English
            let en_result = service.get(key, "en").await;
            assert!(en_result.is_ok(), "Missing English error message: {}", key);
            
            // Test Japanese
            let ja_result = service.get(key, "ja").await;
            assert!(ja_result.is_ok(), "Missing Japanese error message: {}", key);
        }
    }
    
    #[tokio::test]
    async fn test_language_and_font_error_messages_exist() {
        let pool = setup_test_db().await;
        let service = I18nService::new(pool);
        
        let error_keys = vec![
            "error.language_change_failed",
            "error.font_size_change_failed",
            "error.font_size_apply_failed",
        ];
        
        for key in error_keys {
            // Test English
            let en_result = service.get(key, "en").await;
            assert!(en_result.is_ok(), "Missing English error message: {}", key);
            
            // Test Japanese
            let ja_result = service.get(key, "ja").await;
            assert!(ja_result.is_ok(), "Missing Japanese error message: {}", key);
        }
    }
    
    #[tokio::test]
    async fn test_validation_messages_exist() {
        let pool = setup_test_db().await;
        let service = I18nService::new(pool);
        
        // Test validation.required
        let en_result = service.get("validation.required", "en").await;
        assert!(en_result.is_ok(), "Missing English validation message: validation.required");
        assert_eq!(en_result.unwrap(), "Please fill out this field");
        
        let ja_result = service.get("validation.required", "ja").await;
        assert!(ja_result.is_ok(), "Missing Japanese validation message: validation.required");
        assert_eq!(ja_result.unwrap(), "このフィールドを入力してください");
    }
    
    #[tokio::test]
    async fn test_all_error_messages_have_both_languages() {
        let pool = setup_test_db().await;
        let service = I18nService::new(pool);
        
        // Get all error message keys
        let error_resources = service.get_by_category("en", "error").await.unwrap();
        
        for (key, _) in error_resources.iter() {
            // Check that Japanese translation exists
            let ja_result = service.get(key, "ja").await;
            assert!(
                ja_result.is_ok(),
                "Error message '{}' is missing Japanese translation",
                key
            );
        }
    }
}
