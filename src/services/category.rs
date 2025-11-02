use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use crate::sql_queries;

#[derive(Debug)]
pub enum CategoryError {
    DatabaseError(sqlx::Error),
    DuplicateName(String),
}

impl std::fmt::Display for CategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            CategoryError::DuplicateName(name) => write!(f, "Category name '{}' already exists", name),
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
    
    /// Populate default categories for a new user
    /// This will be called when a general user is registered
    pub async fn populate_default_categories(&self, user_id: i64) -> Result<(), CategoryError> {
        // Check if categories already exist for this user (check CATEGORY2, not CATEGORY1)
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CATEGORY2 WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        
        if count > 0 {
            // Categories already populated
            return Ok(());
        }
        
        // Read SQL file BEFORE starting transaction to avoid long-running transaction
        let sql_content = std::fs::read_to_string("res/sql/default_categories_seed.sql")
            .map_err(|e| CategoryError::DatabaseError(sqlx::Error::Io(e)))?;
        
        // Replace :pUserID placeholder with actual user_id
        let sql_content = sql_content.replace(":pUserID", &user_id.to_string());
        
        // Start transaction
        let mut tx = self.pool.begin().await?;
        
        // First, create CATEGORY1 (fixed categories)
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        // Insert CATEGORY1 records
        let category1_data = [
            ("EXPENSE", 1, "支出"),
            ("INCOME", 2, "収入"),
            ("TRANSFER", 3, "振替"),
        ];
        
        for (code, order, name) in category1_data.iter() {
            sqlx::query(sql_queries::CATEGORY_INSERT_CATEGORY1)
                .bind(user_id)
                .bind(code)
                .bind(order)
                .bind(name)
                .bind(&now)
                .execute(&mut *tx)
                .await?;
        }
        
        // Insert CATEGORY1_I18N records
        let cat1_i18n = [
            ("EXPENSE", "en", "Expense"),
            ("EXPENSE", "ja", "支出"),
            ("INCOME", "en", "Income"),
            ("INCOME", "ja", "収入"),
            ("TRANSFER", "en", "Transfer"),
            ("TRANSFER", "ja", "振替"),
        ];
        
        for (code, lang, name) in cat1_i18n.iter() {
            sqlx::query(sql_queries::CATEGORY_INSERT_CATEGORY1_I18N)
                .bind(user_id)
                .bind(code)
                .bind(lang)
                .bind(name)
                .bind(&now)
                .execute(&mut *tx)
                .await?;
        }
        
        // Execute SQL statements
        // Split by semicolon and filter out comments and empty lines
        for statement in sql_content.split(';') {
            let stmt = statement.trim();
            
            // Skip empty statements
            if stmt.is_empty() {
                continue;
            }
            
            // Skip comment-only statements
            let lines: Vec<&str> = stmt.lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty() && !l.starts_with("--"))
                .collect();
            
            if lines.is_empty() {
                continue;
            }
            
            // Reconstruct statement without comment-only lines
            let clean_stmt = lines.join(" ");
            
            sqlx::query(&clean_stmt)
                .execute(&mut *tx)
                .await?;
        }
        
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
    
    /// Add a new category2 (middle category)
    pub async fn add_category2(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_name_ja: &str,
        category2_name_en: &str,
    ) -> Result<String, CategoryError> {
        // Check for duplicate Japanese name
        let count_ja: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_name_ja)
            .bind("ja")
            .fetch_one(&self.pool)
            .await?;
        
        if count_ja > 0 {
            return Err(CategoryError::DuplicateName(category2_name_ja.to_string()));
        }
        
        // Check for duplicate English name
        let count_en: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_name_en)
            .bind("en")
            .fetch_one(&self.pool)
            .await?;
        
        if count_en > 0 {
            return Err(CategoryError::DuplicateName(category2_name_en.to_string()));
        }
        
        // Also check if Japanese name exists in English names
        let count_ja_in_en: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_name_ja)
            .bind("en")
            .fetch_one(&self.pool)
            .await?;
        
        if count_ja_in_en > 0 {
            return Err(CategoryError::DuplicateName(category2_name_ja.to_string()));
        }
        
        // Also check if English name exists in Japanese names
        let count_en_in_ja: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_name_en)
            .bind("ja")
            .fetch_one(&self.pool)
            .await?;
        
        if count_en_in_ja > 0 {
            return Err(CategoryError::DuplicateName(category2_name_en.to_string()));
        }
        
        // Generate new category2_code
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM CATEGORY2 WHERE USER_ID = ? AND CATEGORY1_CODE = ?"
        )
        .bind(user_id)
        .bind(category1_code)
        .fetch_one(&self.pool)
        .await?;
        
        let category2_code = format!("C2_{}_{}", 
            &category1_code[0..1], 
            count + 1
        );
        
        // Get max display_order
        let max_order_row = sqlx::query(sql_queries::CATEGORY2_GET_MAX_ORDER)
            .bind(user_id)
            .bind(category1_code)
            .fetch_one(&self.pool)
            .await?;
        let max_order: i64 = max_order_row.get("max_order");
        let new_order = max_order + 1;
        
        // Insert category2
        sqlx::query(sql_queries::CATEGORY2_INSERT)
            .bind(user_id)
            .bind(category1_code)
            .bind(&category2_code)
            .bind(new_order)
            .bind(category2_name_en) // Default name (English)
            .execute(&self.pool)
            .await?;
        
        // Insert i18n records
        sqlx::query(sql_queries::CATEGORY2_I18N_INSERT)
            .bind(user_id)
            .bind(category1_code)
            .bind(&category2_code)
            .bind("ja")
            .bind(category2_name_ja)
            .execute(&self.pool)
            .await?;
        
        sqlx::query(sql_queries::CATEGORY2_I18N_INSERT)
            .bind(user_id)
            .bind(category1_code)
            .bind(&category2_code)
            .bind("en")
            .bind(category2_name_en)
            .execute(&self.pool)
            .await?;
        
        Ok(category2_code)
    }
    
    /// Add a new category3 (minor category)
    pub async fn add_category3(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        category3_name_ja: &str,
        category3_name_en: &str,
    ) -> Result<String, CategoryError> {
        // Check for duplicate Japanese name
        let count_ja: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_name_ja)
            .bind("ja")
            .fetch_one(&self.pool)
            .await?;
        
        if count_ja > 0 {
            return Err(CategoryError::DuplicateName(category3_name_ja.to_string()));
        }
        
        // Check for duplicate English name
        let count_en: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_name_en)
            .bind("en")
            .fetch_one(&self.pool)
            .await?;
        
        if count_en > 0 {
            return Err(CategoryError::DuplicateName(category3_name_en.to_string()));
        }
        
        // Also check if Japanese name exists in English names
        let count_ja_in_en: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_name_ja)
            .bind("en")
            .fetch_one(&self.pool)
            .await?;
        
        if count_ja_in_en > 0 {
            return Err(CategoryError::DuplicateName(category3_name_ja.to_string()));
        }
        
        // Also check if English name exists in Japanese names
        let count_en_in_ja: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_name_en)
            .bind("ja")
            .fetch_one(&self.pool)
            .await?;
        
        if count_en_in_ja > 0 {
            return Err(CategoryError::DuplicateName(category3_name_en.to_string()));
        }
        
        // Generate new category3_code
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM CATEGORY3 WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ?"
        )
        .bind(user_id)
        .bind(category1_code)
        .bind(category2_code)
        .fetch_one(&self.pool)
        .await?;
        
        let category3_code = format!("C3_{}_{}_{}", 
            &category1_code[0..1],
            &category2_code.chars().rev().take(1).collect::<String>(),
            count + 1
        );
        
        // Get max display_order
        let max_order_row = sqlx::query(sql_queries::CATEGORY3_GET_MAX_ORDER)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .fetch_one(&self.pool)
            .await?;
        let max_order: i64 = max_order_row.get("max_order");
        let new_order = max_order + 1;
        
        // Insert category3
        sqlx::query(sql_queries::CATEGORY3_INSERT)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(&category3_code)
            .bind(new_order)
            .bind(category3_name_en) // Default name (English)
            .execute(&self.pool)
            .await?;
        
        // Insert i18n records
        sqlx::query(sql_queries::CATEGORY3_I18N_INSERT)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(&category3_code)
            .bind("ja")
            .bind(category3_name_ja)
            .execute(&self.pool)
            .await?;
        
        sqlx::query(sql_queries::CATEGORY3_I18N_INSERT)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(&category3_code)
            .bind("en")
            .bind(category3_name_en)
            .execute(&self.pool)
            .await?;
        
        Ok(category3_code)
    }
    
    /// Get category2 data for editing
    pub async fn get_category2_for_edit(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
    ) -> Result<CategoryForEdit, CategoryError> {
        let row = sqlx::query(sql_queries::CATEGORY2_GET_FOR_EDIT)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(CategoryForEdit {
            code: row.get("CATEGORY2_CODE"),
            name_ja: row.get("name_ja"),
            name_en: row.get("name_en"),
        })
    }
    
    /// Get category3 data for editing
    pub async fn get_category3_for_edit(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        category3_code: &str,
    ) -> Result<CategoryForEdit, CategoryError> {
        let row = sqlx::query(sql_queries::CATEGORY3_GET_FOR_EDIT)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .fetch_one(&self.pool)
            .await?;
        
        Ok(CategoryForEdit {
            code: row.get("CATEGORY3_CODE"),
            name_ja: row.get("name_ja"),
            name_en: row.get("name_en"),
        })
    }
    
    /// Update category2 i18n names
    pub async fn update_category2_i18n(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        name_ja: &str,
        name_en: &str,
    ) -> Result<(), CategoryError> {
        // Check for duplicate Japanese name (excluding current category)
        let count_ja: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME_EXCLUDING)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind("ja")
            .bind(name_ja)
            .fetch_one(&self.pool)
            .await?;
        
        if count_ja > 0 {
            return Err(CategoryError::DuplicateName(name_ja.to_string()));
        }
        
        // Check for duplicate English name (excluding current category)
        let count_en: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME_EXCLUDING)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind("en")
            .bind(name_en)
            .fetch_one(&self.pool)
            .await?;
        
        if count_en > 0 {
            return Err(CategoryError::DuplicateName(name_en.to_string()));
        }
        
        // Also check if Japanese name exists in English names
        let count_ja_in_en: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME_EXCLUDING)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind("en")
            .bind(name_ja)
            .fetch_one(&self.pool)
            .await?;
        
        if count_ja_in_en > 0 {
            return Err(CategoryError::DuplicateName(name_ja.to_string()));
        }
        
        // Also check if English name exists in Japanese names
        let count_en_in_ja: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME_EXCLUDING)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind("ja")
            .bind(name_en)
            .fetch_one(&self.pool)
            .await?;
        
        if count_en_in_ja > 0 {
            return Err(CategoryError::DuplicateName(name_en.to_string()));
        }
        
        // Update Japanese name
        sqlx::query(sql_queries::CATEGORY2_I18N_UPDATE)
            .bind(name_ja)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind("ja")
            .execute(&self.pool)
            .await?;
        
        // Update English name
        sqlx::query(sql_queries::CATEGORY2_I18N_UPDATE)
            .bind(name_en)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind("en")
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// Update category3 i18n names
    pub async fn update_category3_i18n(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        category3_code: &str,
        name_ja: &str,
        name_en: &str,
    ) -> Result<(), CategoryError> {
        // Check for duplicate Japanese name (excluding current category)
        let count_ja: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME_EXCLUDING)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .bind("ja")
            .bind(name_ja)
            .fetch_one(&self.pool)
            .await?;
        
        if count_ja > 0 {
            return Err(CategoryError::DuplicateName(name_ja.to_string()));
        }
        
        // Check for duplicate English name (excluding current category)
        let count_en: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME_EXCLUDING)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .bind("en")
            .bind(name_en)
            .fetch_one(&self.pool)
            .await?;
        
        if count_en > 0 {
            return Err(CategoryError::DuplicateName(name_en.to_string()));
        }
        
        // Also check if Japanese name exists in English names
        let count_ja_in_en: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME_EXCLUDING)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .bind("en")
            .bind(name_ja)
            .fetch_one(&self.pool)
            .await?;
        
        if count_ja_in_en > 0 {
            return Err(CategoryError::DuplicateName(name_ja.to_string()));
        }
        
        // Also check if English name exists in Japanese names
        let count_en_in_ja: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME_EXCLUDING)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .bind("ja")
            .bind(name_en)
            .fetch_one(&self.pool)
            .await?;
        
        if count_en_in_ja > 0 {
            return Err(CategoryError::DuplicateName(name_en.to_string()));
        }
        
        // Update Japanese name
        sqlx::query(sql_queries::CATEGORY3_I18N_UPDATE)
            .bind(name_ja)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .bind("ja")
            .execute(&self.pool)
            .await?;
        
        // Update English name
        sqlx::query(sql_queries::CATEGORY3_I18N_UPDATE)
            .bind(name_en)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .bind("en")
            .execute(&self.pool)
            .await?;
        
        Ok(())
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

#[derive(Debug, serde::Serialize)]
pub struct CategoryForEdit {
    pub code: String,
    pub name_ja: String,
    pub name_en: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
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

    async fn setup_category1(pool: &SqlitePool, user_id: i64) {
        // Insert CATEGORY1 - EXPENSE
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1)
            .bind(user_id)
            .bind("EXPENSE")
            .bind(1)
            .bind("Expense")
            .bind(0)
            .execute(pool)
            .await
            .unwrap();
        
        // Insert Japanese i18n for EXPENSE
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1_I18N)
            .bind(user_id)
            .bind("EXPENSE")
            .bind("ja")
            .bind("支出")
            .execute(pool)
            .await
            .unwrap();
        
        // Insert English i18n for EXPENSE
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1_I18N)
            .bind(user_id)
            .bind("EXPENSE")
            .bind("en")
            .bind("Expense")
            .execute(pool)
            .await
            .unwrap();
        
        // Insert CATEGORY1 - INCOME
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1)
            .bind(user_id)
            .bind("INCOME")
            .bind(2)
            .bind("Income")
            .bind(0)
            .execute(pool)
            .await
            .unwrap();
        
        // Insert Japanese i18n for INCOME
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1_I18N)
            .bind(user_id)
            .bind("INCOME")
            .bind("ja")
            .bind("収入")
            .execute(pool)
            .await
            .unwrap();
        
        // Insert English i18n for INCOME
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1_I18N)
            .bind(user_id)
            .bind("INCOME")
            .bind("en")
            .bind("Income")
            .execute(pool)
            .await
            .unwrap();
        
        // Insert CATEGORY1 - TRANSFER
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1)
            .bind(user_id)
            .bind("TRANSFER")
            .bind(3)
            .bind("Transfer")
            .bind(0)
            .execute(pool)
            .await
            .unwrap();
        
        // Insert Japanese i18n for TRANSFER
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1_I18N)
            .bind(user_id)
            .bind("TRANSFER")
            .bind("ja")
            .bind("振替")
            .execute(pool)
            .await
            .unwrap();
        
        // Insert English i18n for TRANSFER
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1_I18N)
            .bind(user_id)
            .bind("TRANSFER")
            .bind("en")
            .bind("Transfer")
            .execute(pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_populate_default_categories() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 2;  // Use a different user_id
        
        // populate_default_categories creates CATEGORY1 automatically
        // No need to call setup_category1
        
        // Populate default categories
        let result = service.populate_default_categories(user_id).await;
        assert!(result.is_ok(), "Failed to populate categories: {:?}", result.err());
        
        // Verify CATEGORY2 records were created
        let cat2_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CATEGORY2 WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(cat2_count > 0, "No CATEGORY2 records created");
        assert_eq!(cat2_count, 20, "Expected 20 CATEGORY2 records");
        
        // Verify CATEGORY3 records were created
        let cat3_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CATEGORY3 WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(cat3_count > 0, "No CATEGORY3 records created");
        assert_eq!(cat3_count, 126, "Expected 126 CATEGORY3 records");
        
        // Verify I18N records were created (English only, Japanese is in main table)
        let cat2_i18n_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CATEGORY2_I18N WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(cat2_i18n_count > 0, "No CATEGORY2_I18N records created");
        assert_eq!(cat2_i18n_count, 20, "Expected 20 CATEGORY2_I18N records (English only)");
        
        let cat3_i18n_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CATEGORY3_I18N WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(cat3_i18n_count > 0, "No CATEGORY3_I18N records created");
        assert_eq!(cat3_i18n_count, 126, "Expected 126 CATEGORY3_I18N records (English only)");
        
        // Verify all English I18N records are present
        let cat3_i18n_en: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CATEGORY3_I18N WHERE USER_ID = ? AND LANG_CODE = 'en'")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(cat3_i18n_en, 126, "Expected 126 English CATEGORY3_I18N records");
        
        // Verify it doesn't re-populate if called again
        let result2 = service.populate_default_categories(user_id).await;
        assert!(result2.is_ok());
        
        let cat2_count_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CATEGORY2 WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(cat2_count, cat2_count_after, "Categories should not be re-initialized");
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

    #[tokio::test]
    async fn test_add_category2() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 1;
        
        // Setup: Create CATEGORY1
        setup_category1(&pool, user_id).await;
        
        // Add category2
        let result = service.add_category2(user_id, "EXPENSE", "食費", "Food").await;
        assert!(result.is_ok());
        let category2_code = result.unwrap();
        
        // Verify category2 was created
        let row = sqlx::query(sql_queries::TEST_CATEGORY_GET_CATEGORY2_NAME)
            .bind(user_id)
            .bind(&category2_code)
            .fetch_one(&pool)
            .await
            .unwrap();
        let name: String = row.get(0);
        assert_eq!(name, "Food");
        
        // Verify i18n records were created
        let ja_row = sqlx::query(sql_queries::TEST_CATEGORY_GET_CATEGORY2_I18N_NAME)
            .bind(user_id)
            .bind(&category2_code)
            .bind("ja")
            .fetch_one(&pool)
            .await
            .unwrap();
        let ja_name: String = ja_row.get(0);
        assert_eq!(ja_name, "食費");
    }

    #[tokio::test]
    async fn test_add_category2_duplicate_name() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 1;
        
        // Setup
        setup_category1(&pool, user_id).await;
        
        // Add first category2
        service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        
        // Test 1: Try to add duplicate (same Japanese name)
        let result = service.add_category2(user_id, "EXPENSE", "食費", "Food2").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CategoryError::DuplicateName(_)));
        
        // Test 2: Try to add duplicate (same English name)
        let result2 = service.add_category2(user_id, "EXPENSE", "食費2", "Food").await;
        assert!(result2.is_err());
        assert!(matches!(result2.unwrap_err(), CategoryError::DuplicateName(_)));
        
        // Test 3: Try to add with Japanese name matching existing English name
        let result3 = service.add_category2(user_id, "EXPENSE", "Food", "Other").await;
        assert!(result3.is_err());
        assert!(matches!(result3.unwrap_err(), CategoryError::DuplicateName(_)));
        
        // Test 4: Try to add with English name matching existing Japanese name
        let result4 = service.add_category2(user_id, "EXPENSE", "Other", "食費").await;
        assert!(result4.is_err());
        assert!(matches!(result4.unwrap_err(), CategoryError::DuplicateName(_)));
    }

    #[tokio::test]
    async fn test_add_category3() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 1;
        
        // Setup
        setup_category1(&pool, user_id).await;
        service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        
        // Get the category2_code
        let cat2_row = sqlx::query(sql_queries::TEST_CATEGORY_GET_FIRST_CATEGORY2_CODE)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        let category2_code: String = cat2_row.get(0);
        
        // Add category3
        let result = service.add_category3(user_id, "EXPENSE", &category2_code, "食料品", "Groceries").await;
        assert!(result.is_ok());
        let category3_code = result.unwrap();
        
        // Verify category3 was created
        let row = sqlx::query(sql_queries::TEST_CATEGORY_GET_CATEGORY3_NAME)
            .bind(user_id)
            .bind(&category3_code)
            .fetch_one(&pool)
            .await
            .unwrap();
        let name: String = row.get(0);
        assert_eq!(name, "Groceries");
    }

    #[tokio::test]
    async fn test_add_category3_duplicate_name() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 1;
        
        // Setup
        setup_category1(&pool, user_id).await;
        service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        
        let cat2_row = sqlx::query(sql_queries::TEST_CATEGORY_GET_FIRST_CATEGORY2_CODE)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        let category2_code: String = cat2_row.get(0);
        
        // Add first category3
        service.add_category3(user_id, "EXPENSE", &category2_code, "食料品", "Groceries").await.unwrap();
        
        // Test 1: Try to add duplicate (same Japanese name)
        let result = service.add_category3(user_id, "EXPENSE", &category2_code, "食料品", "Groceries2").await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CategoryError::DuplicateName(_)));
        
        // Test 2: Try to add duplicate (same English name)
        let result2 = service.add_category3(user_id, "EXPENSE", &category2_code, "食料品2", "Groceries").await;
        assert!(result2.is_err());
        assert!(matches!(result2.unwrap_err(), CategoryError::DuplicateName(_)));
        
        // Test 3: Try to add with Japanese name matching existing English name
        let result3 = service.add_category3(user_id, "EXPENSE", &category2_code, "Groceries", "Other").await;
        assert!(result3.is_err());
        assert!(matches!(result3.unwrap_err(), CategoryError::DuplicateName(_)));
        
        // Test 4: Try to add with English name matching existing Japanese name
        let result4 = service.add_category3(user_id, "EXPENSE", &category2_code, "Other", "食料品").await;
        assert!(result4.is_err());
        assert!(matches!(result4.unwrap_err(), CategoryError::DuplicateName(_)));
    }
}
