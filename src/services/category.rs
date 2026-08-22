use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use crate::api_error::ApiError;
use crate::{sql_queries, consts};

const ENTITY_LABEL: &str = "Category";

#[derive(Debug)]
pub enum CategoryError {
    DatabaseError(sqlx::Error),
    DuplicateName(String),
    Validation(String),
    NotFound,
}

impl std::fmt::Display for CategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CategoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            CategoryError::DuplicateName(name) => write!(f, "Category name '{}' already exists", name),
            CategoryError::Validation(msg) => write!(f, "{}", msg),
            CategoryError::NotFound => write!(f, "Category not found"),
        }
    }
}

impl std::error::Error for CategoryError {}

impl From<sqlx::Error> for CategoryError {
    fn from(err: sqlx::Error) -> Self {
        CategoryError::DatabaseError(err)
    }
}

/// Map the domain-specific `CategoryError` onto the wire-level `ApiError`
/// so tauri command wrappers can `?`-propagate it into a structured
/// `{ code, message, entity? }` payload for the frontend classifier
/// (`res/js/master-crud.js::mapMasterErrorCode`). Kept as `From` (rather
/// than a bespoke `.map_err`) so wrapper bodies stay one-line — the
/// mapping happens implicitly at the `?` boundary. Matches the
/// `From<UserManagementError>` shape (PR #100).
///
/// Codes:
///   - `NotFound`            → `not_found` (entity="category")
///   - `DuplicateName(_)`    → `duplicate_name` (entity="category")
///   - `Validation(msg)`     → `validation` (message preserved for logs)
///   - `DatabaseError(e)`    → `database`
impl From<CategoryError> for ApiError {
    fn from(err: CategoryError) -> Self {
        match err {
            CategoryError::NotFound => ApiError::not_found(ENTITY_LABEL),
            CategoryError::DuplicateName(_) => ApiError::duplicate_name(ENTITY_LABEL),
            CategoryError::Validation(msg) => ApiError::validation(msg),
            CategoryError::DatabaseError(e) => ApiError::database(e.to_string()),
        }
    }
}

/// Issue #37 Phase 2-3 — bounded-field length guard for category i18n
/// name columns (`CATEGORY*_I18N.*_NAME_I18N`). Counts characters, not
/// bytes, so Japanese input is not implicitly clipped to ~85 chars.
fn validate_i18n_name_length(name: &str, label: &str) -> Result<(), CategoryError> {
    crate::validation::validate_max_chars(label, name, consts::MAX_I18N_NAME_LEN)
        .map_err(CategoryError::Validation)
}

pub struct CategoryService {
    pool: SqlitePool,
}

impl CategoryService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // -----------------------------------------------------------------
    // PR9 (Fable-5 #28): shared bilingual-duplicate checkers
    //
    // Both add and update paths for CATEGORY2 / CATEGORY3 need the same
    // 4-way check against CATEGORY{2,3}_I18N: the incoming JA / EN
    // names must not already exist in either lang column, in either
    // direction (`ja in ja`, `en in en`, `ja in en`, `en in ja`). The
    // original code duplicated that block 16 times (4 checks × 4
    // callers) with subtly different bind orders between the ADD and
    // EXCLUDING SQL constants — a bug waiting to happen the next time
    // one of the four SQL clauses was extended.
    //
    // The helpers below take `exclude_code: Option<&str>` and dispatch
    // to the matching SQL: `None` → ADD variant (add flow), `Some(code)`
    // → EXCLUDING variant (update flow, exclude the row being edited).
    // Bind order per SQL is encoded once inside the helper so the
    // callers can never get it wrong.

    /// Return `Err(DuplicateName(name))` if either of the two incoming
    /// CATEGORY2 names collides with any existing CATEGORY2_I18N row
    /// in the (user, cat1) scope, across either lang. Pass
    /// `exclude_code = Some(cat2)` from the update path.
    async fn check_category2_bilingual_duplicate(
        &self,
        user_id: i64,
        category1_code: &str,
        exclude_code: Option<&str>,
        name_ja: &str,
        name_en: &str,
    ) -> Result<(), CategoryError> {
        for (name, lang) in [
            (name_ja, "ja"),
            (name_en, "en"),
            (name_ja, "en"),
            (name_en, "ja"),
        ] {
            let count: i64 = match exclude_code {
                None => sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME)
                    .bind(user_id)
                    .bind(category1_code)
                    .bind(name)
                    .bind(lang)
                    .fetch_one(&self.pool)
                    .await?,
                Some(code) => {
                    // CATEGORY2_CHECK_DUPLICATE_NAME_EXCLUDING binds
                    // `(user_id, cat1, exclude_code, lang, name)` — the
                    // LANG / NAME columns are ordered differently to
                    // the ADD constant, so keep the bind order fenced
                    // in here rather than at each call site.
                    sqlx::query_scalar(sql_queries::CATEGORY2_CHECK_DUPLICATE_NAME_EXCLUDING)
                        .bind(user_id)
                        .bind(category1_code)
                        .bind(code)
                        .bind(lang)
                        .bind(name)
                        .fetch_one(&self.pool)
                        .await?
                }
            };
            if count > 0 {
                return Err(CategoryError::DuplicateName(name.to_string()));
            }
        }
        Ok(())
    }

    /// Same shape as [`check_category2_bilingual_duplicate`] for the
    /// CATEGORY3 level. The extra `category2_code` bind lives in both
    /// SQL constants so it's threaded through explicitly.
    async fn check_category3_bilingual_duplicate(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        exclude_code: Option<&str>,
        name_ja: &str,
        name_en: &str,
    ) -> Result<(), CategoryError> {
        for (name, lang) in [
            (name_ja, "ja"),
            (name_en, "en"),
            (name_ja, "en"),
            (name_en, "ja"),
        ] {
            let count: i64 = match exclude_code {
                None => sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME)
                    .bind(user_id)
                    .bind(category1_code)
                    .bind(category2_code)
                    .bind(name)
                    .bind(lang)
                    .fetch_one(&self.pool)
                    .await?,
                Some(code) => sqlx::query_scalar(sql_queries::CATEGORY3_CHECK_DUPLICATE_NAME_EXCLUDING)
                    .bind(user_id)
                    .bind(category1_code)
                    .bind(category2_code)
                    .bind(code)
                    .bind(lang)
                    .bind(name)
                    .fetch_one(&self.pool)
                    .await?,
            };
            if count > 0 {
                return Err(CategoryError::DuplicateName(name.to_string()));
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------
    // PR9 (Fable-5 #27): shared display-order swap helpers
    //
    // The 4 `move_category{2,3}_{up,down}` publics were structural
    // clones: fetch current order, add ±1, look up the sibling at
    // that target, and if one exists, run two UPDATEs inside a tx.
    // The `<= 1` guard on the `_up` variants was redundant — when
    // current_order is 1 the target is 0 and the sibling lookup
    // returns None, which the `if let Some(...)` arm already
    // treats as a no-op. Two internal helpers below (one per level)
    // consume `delta = ±1`, and the 4 publics each collapse to a
    // one-line wrapper.

    async fn swap_category2_with_sibling(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        delta: i64,
    ) -> Result<(), CategoryError> {
        let current_order: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_GET_ORDER)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .fetch_one(&self.pool)
            .await?;

        let target_order = current_order + delta;

        // A missing sibling means there is nothing to swap with
        // (no-op). Any other database error must reach the caller
        // instead of being reported as success.
        let sibling_code: Option<String> =
            sqlx::query_scalar(sql_queries::CATEGORY2_GET_SIBLING_BY_ORDER)
                .bind(user_id)
                .bind(category1_code)
                .bind(target_order)
                .fetch_optional(&self.pool)
                .await?;

        if let Some(sibling_code) = sibling_code {
            let mut tx = self.pool.begin().await?;
            // Move current to target.
            sqlx::query(sql_queries::CATEGORY2_UPDATE_ORDER)
                .bind(target_order)
                .bind(user_id)
                .bind(category1_code)
                .bind(category2_code)
                .execute(&mut *tx)
                .await?;
            // Move sibling to the old current position.
            sqlx::query(sql_queries::CATEGORY2_UPDATE_ORDER)
                .bind(current_order)
                .bind(user_id)
                .bind(category1_code)
                .bind(&sibling_code)
                .execute(&mut *tx)
                .await?;
            tx.commit().await?;
        }

        Ok(())
    }

    async fn swap_category3_with_sibling(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        category3_code: &str,
        delta: i64,
    ) -> Result<(), CategoryError> {
        let current_order: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_GET_ORDER)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .fetch_one(&self.pool)
            .await?;

        let target_order = current_order + delta;

        let sibling_code: Option<String> =
            sqlx::query_scalar(sql_queries::CATEGORY3_GET_SIBLING_BY_ORDER)
                .bind(user_id)
                .bind(category1_code)
                .bind(category2_code)
                .bind(target_order)
                .fetch_optional(&self.pool)
                .await?;

        if let Some(sibling_code) = sibling_code {
            let mut tx = self.pool.begin().await?;
            sqlx::query(sql_queries::CATEGORY3_UPDATE_ORDER)
                .bind(target_order)
                .bind(user_id)
                .bind(category1_code)
                .bind(category2_code)
                .bind(category3_code)
                .execute(&mut *tx)
                .await?;
            sqlx::query(sql_queries::CATEGORY3_UPDATE_ORDER)
                .bind(current_order)
                .bind(user_id)
                .bind(category1_code)
                .bind(category2_code)
                .bind(&sibling_code)
                .execute(&mut *tx)
                .await?;
            tx.commit().await?;
        }

        Ok(())
    }
    
    /// Populate default categories for a new user
    /// This will be called when a general user is registered
    pub async fn populate_default_categories(&self, user_id: i64) -> Result<(), CategoryError> {
        // Check if categories already exist for this user (check CATEGORY2, not CATEGORY1)
        let count: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_COUNT_BY_USER)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;
        
        if count > 0 {
            // Categories already populated
            return Ok(());
        }
        
        // Seed SQL is embedded at compile time. Reading from a CWD-relative
        // path silently works under `cargo tauri dev` (CWD = project root) but
        // crashes installed .msi/.exe builds because the install directory is
        // the CWD and `res/sql/default_categories_seed.sql` isn't shipped there.
        const DEFAULT_CATEGORIES_SEED: &str = include_str!("../../res/sql/default_categories_seed.sql");

        // Replace :pUserID placeholder with actual user_id
        let sql_content = DEFAULT_CATEGORIES_SEED.replace(":pUserID", &user_id.to_string());
        
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
    
    /// Get category tree with internationalization.
    ///
    /// PR11 (Fable-5 #31): rewritten from a 1 + N + N×M nested-query
    /// loop into 3 flat queries followed by Rust-side HashMap
    /// grouping. A new user with the default 3 CATEGORY1 × ~10
    /// CATEGORY2 × ~5 CATEGORY3 layout would previously issue
    /// 1 + 3 + 30 = 34 queries every time the transaction form loaded
    /// its category dropdowns; now it issues 3.
    pub async fn get_category_tree(&self, user_id: i64, lang_code: &str) -> Result<serde_json::Value, CategoryError> {
        use serde_json::json;
        use std::collections::HashMap;

        // 3 flat queries. Both cat2 and cat3 rows are pre-ordered by
        // (parent code, DISPLAY_ORDER) so downstream grouping preserves
        // the display order per parent without re-sorting.
        let cat1_rows = sqlx::query(sql_queries::CATEGORY1_TREE)
            .bind(lang_code)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        let cat2_rows = sqlx::query(sql_queries::CATEGORY2_TREE)
            .bind(lang_code)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        let cat3_rows = sqlx::query(sql_queries::CATEGORY3_TREE)
            .bind(lang_code)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        // Bucket cat3 by (cat1_code, cat2_code) — this is the join key
        // the outer loop consumes. We build the JSON leaves eagerly so
        // the inner loop below is a plain HashMap lookup.
        let mut cat3_by_parent: HashMap<(String, String), Vec<serde_json::Value>> = HashMap::new();
        for row in &cat3_rows {
            let cat1_code: String = row.get("CATEGORY1_CODE");
            let cat2_code: String = row.get("CATEGORY2_CODE");
            let leaf = json!({
                "category3_code": row.get::<String, _>("CATEGORY3_CODE"),
                "category3_name_i18n": row.get::<String, _>("name"),
                "display_order": row.get::<i64, _>("DISPLAY_ORDER")
            });
            cat3_by_parent.entry((cat1_code, cat2_code)).or_default().push(leaf);
        }

        // Bucket cat2 by cat1_code, emitting the fully assembled
        // {category2, children} nodes.
        let mut cat2_by_parent: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
        for row in &cat2_rows {
            let cat1_code: String = row.get("CATEGORY1_CODE");
            let cat2_code: String = row.get("CATEGORY2_CODE");
            let cat2_name: String = row.get("name");
            let cat2_order: i64 = row.get("DISPLAY_ORDER");
            let children = cat3_by_parent
                .remove(&(cat1_code.clone(), cat2_code.clone()))
                .unwrap_or_default();
            cat2_by_parent.entry(cat1_code).or_default().push(json!({
                "category2": {
                    "category2_code": cat2_code,
                    "category2_name_i18n": cat2_name,
                    "display_order": cat2_order
                },
                "children": children
            }));
        }

        // Walk cat1 in its display order (already sorted by the SQL) and
        // attach the matching cat2 bucket.
        let categories: Vec<_> = cat1_rows
            .iter()
            .map(|row| {
                let cat1_code: String = row.get("CATEGORY1_CODE");
                let cat1_name: String = row.get("name");
                let cat1_order: i64 = row.get("DISPLAY_ORDER");
                let children = cat2_by_parent.remove(&cat1_code).unwrap_or_default();
                json!({
                    "category1": {
                        "category1_code": cat1_code,
                        "category1_name_i18n": cat1_name,
                        "display_order": cat1_order
                    },
                    "children": children
                })
            })
            .collect();

        Ok(json!(categories))
    }

    /// Get category tree including disabled items (for management screen).
    /// Same 3-flat-queries shape as [`get_category_tree`] plus the
    /// `is_disabled` fields the management UI needs.
    pub async fn get_category_tree_all(&self, user_id: i64, lang_code: &str) -> Result<serde_json::Value, CategoryError> {
        use serde_json::json;
        use std::collections::HashMap;

        let cat1_rows = sqlx::query(sql_queries::CATEGORY1_TREE)
            .bind(lang_code)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        let cat2_rows = sqlx::query(sql_queries::CATEGORY2_TREE_ALL)
            .bind(lang_code)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;
        let cat3_rows = sqlx::query(sql_queries::CATEGORY3_TREE_ALL)
            .bind(lang_code)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let mut cat3_by_parent: HashMap<(String, String), Vec<serde_json::Value>> = HashMap::new();
        for row in &cat3_rows {
            let cat1_code: String = row.get("CATEGORY1_CODE");
            let cat2_code: String = row.get("CATEGORY2_CODE");
            let leaf = json!({
                "category3_code": row.get::<String, _>("CATEGORY3_CODE"),
                "category3_name_i18n": row.get::<String, _>("name"),
                "display_order": row.get::<i64, _>("DISPLAY_ORDER"),
                "is_disabled": row.get::<i64, _>("IS_DISABLED")
            });
            cat3_by_parent.entry((cat1_code, cat2_code)).or_default().push(leaf);
        }

        let mut cat2_by_parent: HashMap<String, Vec<serde_json::Value>> = HashMap::new();
        for row in &cat2_rows {
            let cat1_code: String = row.get("CATEGORY1_CODE");
            let cat2_code: String = row.get("CATEGORY2_CODE");
            let cat2_name: String = row.get("name");
            let cat2_order: i64 = row.get("DISPLAY_ORDER");
            let cat2_disabled: i64 = row.get("IS_DISABLED");
            let children = cat3_by_parent
                .remove(&(cat1_code.clone(), cat2_code.clone()))
                .unwrap_or_default();
            cat2_by_parent.entry(cat1_code).or_default().push(json!({
                "category2": {
                    "category2_code": cat2_code,
                    "category2_name_i18n": cat2_name,
                    "display_order": cat2_order,
                    "is_disabled": cat2_disabled
                },
                "children": children
            }));
        }

        let categories: Vec<_> = cat1_rows
            .iter()
            .map(|row| {
                let cat1_code: String = row.get("CATEGORY1_CODE");
                let cat1_name: String = row.get("name");
                let cat1_order: i64 = row.get("DISPLAY_ORDER");
                let children = cat2_by_parent.remove(&cat1_code).unwrap_or_default();
                json!({
                    "category1": {
                        "category1_code": cat1_code,
                        "category1_name_i18n": cat1_name,
                        "display_order": cat1_order
                    },
                    "children": children
                })
            })
            .collect();

        Ok(json!(categories))
    }

    /// Re-enable a disabled CATEGORY2 and its child CATEGORY3 entries
    pub async fn enable_category2(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
    ) -> Result<(), CategoryError> {
        sqlx::query(sql_queries::CATEGORY2_ENABLE)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Re-enable a disabled CATEGORY3
    pub async fn enable_category3(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        category3_code: &str,
    ) -> Result<(), CategoryError> {
        sqlx::query(sql_queries::CATEGORY3_ENABLE)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Add a new category2 (middle category)
    pub async fn add_category2(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_name_ja: &str,
        category2_name_en: &str,
    ) -> Result<String, CategoryError> {
        validate_i18n_name_length(category2_name_ja, "Japanese name")?;
        validate_i18n_name_length(category2_name_en, "English name")?;

        // Bilingual dedup (PR9, Fable-5 #28) — 16 blocks collapsed to 1 call.
        self.check_category2_bilingual_duplicate(
            user_id,
            category1_code,
            None,
            category2_name_ja,
            category2_name_en,
        )
        .await?;

        // Generate new category2_code
        let count: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_COUNT_BY_USER_AND_CATEGORY1)
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
        validate_i18n_name_length(category3_name_ja, "Japanese name")?;
        validate_i18n_name_length(category3_name_en, "English name")?;

        // Bilingual dedup (PR9, Fable-5 #28) — see the CATEGORY2 add
        // caller above; identical 4-check contract via the shared helper.
        self.check_category3_bilingual_duplicate(
            user_id,
            category1_code,
            category2_code,
            None,
            category3_name_ja,
            category3_name_en,
        )
        .await?;

        // Generate new category3_code
        let count: i64 = sqlx::query_scalar(sql_queries::CATEGORY3_COUNT_BY_USER_AND_CATEGORY2)
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
    ///
    /// Uses `fetch_optional` so a concurrently disabled/deleted row surfaces
    /// as `CategoryError::NotFound` (mapped by the frontend to a dedicated
    /// `not_found` toast) instead of leaking a raw sqlx `RowNotFound` string
    /// through the generic save-error path — matching the Shop/Product/
    /// Manufacturer master-audit pattern (PR #75/#76/#77).
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
            .fetch_optional(&self.pool)
            .await?
            .ok_or(CategoryError::NotFound)?;

        Ok(CategoryForEdit {
            code: row.get("CATEGORY2_CODE"),
            name_ja: row.get("name_ja"),
            name_en: row.get("name_en"),
        })
    }

    /// Get category3 data for editing (see `get_category2_for_edit` for the
    /// not-found handling rationale).
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
            .fetch_optional(&self.pool)
            .await?
            .ok_or(CategoryError::NotFound)?;

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
        validate_i18n_name_length(name_ja, "Japanese name")?;
        validate_i18n_name_length(name_en, "English name")?;

        // Bilingual dedup (PR9, Fable-5 #28) — 16 blocks collapsed
        // to 1 call. `Some(category2_code)` excludes the row being
        // edited.
        self.check_category2_bilingual_duplicate(
            user_id,
            category1_code,
            Some(category2_code),
            name_ja,
            name_en,
        )
        .await?;

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
        validate_i18n_name_length(name_ja, "Japanese name")?;
        validate_i18n_name_length(name_en, "English name")?;

        // Bilingual dedup (PR9, Fable-5 #28) — see update_category2_i18n
        // above; identical contract.
        self.check_category3_bilingual_duplicate(
            user_id,
            category1_code,
            category2_code,
            Some(category3_code),
            name_ja,
            name_en,
        )
        .await?;

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
    
    /// Move a CATEGORY2 up in the display order. PR9 (Fable-5 #27):
    /// one-line wrapper over the shared swap helper. The old
    /// `current_order <= 1` early-return was redundant because the
    /// sibling lookup at `target_order = 0` returns None and the
    /// helper's `if let Some(...)` arm already treats that as a no-op.
    pub async fn move_category2_up(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
    ) -> Result<(), CategoryError> {
        self.swap_category2_with_sibling(user_id, category1_code, category2_code, -1)
            .await
    }

    /// Move a CATEGORY2 down in the display order.
    pub async fn move_category2_down(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
    ) -> Result<(), CategoryError> {
        self.swap_category2_with_sibling(user_id, category1_code, category2_code, 1)
            .await
    }

    /// Move a CATEGORY3 up in the display order.
    pub async fn move_category3_up(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        category3_code: &str,
    ) -> Result<(), CategoryError> {
        self.swap_category3_with_sibling(
            user_id,
            category1_code,
            category2_code,
            category3_code,
            -1,
        )
        .await
    }

    /// Move a CATEGORY3 down in the display order.
    pub async fn move_category3_down(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        category3_code: &str,
    ) -> Result<(), CategoryError> {
        self.swap_category3_with_sibling(
            user_id,
            category1_code,
            category2_code,
            category3_code,
            1,
        )
        .await
    }

    /// Disable (hide) a CATEGORY2 and its child CATEGORY3 entries
    ///
    /// The CATEGORY3 disable can legitimately touch zero rows (a leaf
    /// category), so its `rows_affected` is not checked. The CATEGORY2
    /// disable itself must hit exactly one row — zero means the target was
    /// removed by another window and we return `NotFound` (matches
    /// Shop/Product/Manufacturer master-audit contract, PR #75/#76/#77).
    pub async fn disable_category2(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
    ) -> Result<(), CategoryError> {
        let mut tx = self.pool.begin().await?;

        // Disable all child CATEGORY3 entries (may be zero — that is fine)
        sqlx::query(sql_queries::CATEGORY3_DISABLE_BY_CATEGORY2)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .execute(&mut *tx)
            .await?;

        // Disable the CATEGORY2 itself — must hit exactly one row
        let result = sqlx::query(sql_queries::CATEGORY2_DELETE_LOGICAL)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .execute(&mut *tx)
            .await?;

        if result.rows_affected() == 0 {
            return Err(CategoryError::NotFound);
        }

        tx.commit().await?;
        Ok(())
    }

    /// Disable (hide) a CATEGORY3
    ///
    /// Returns `NotFound` when the target row is already gone (concurrent
    /// removal from another window), so the frontend can show the dedicated
    /// `not_found` toast and reload the tree.
    pub async fn disable_category3(
        &self,
        user_id: i64,
        category1_code: &str,
        category2_code: &str,
        category3_code: &str,
    ) -> Result<(), CategoryError> {
        let result = sqlx::query(sql_queries::CATEGORY3_DELETE_LOGICAL)
            .bind(user_id)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(CategoryError::NotFound);
        }

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
    use crate::test_helpers::database::{init_db, TEST_DB_URL};

    async fn setup_test_db() -> SqlitePool {
        let pool = init_db(TEST_DB_URL).await.unwrap();

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
        let cat2_count: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_COUNT)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(cat2_count > 0, "No CATEGORY2 records created");
        assert_eq!(cat2_count, 20, "Expected 20 CATEGORY2 records");
        
        // Verify CATEGORY3 records were created
        let cat3_count: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY3_COUNT)
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(cat3_count > 0, "No CATEGORY3 records created");
        assert_eq!(cat3_count, 126, "Expected 126 CATEGORY3 records");
        
        // Verify I18N records were created (both Japanese and English)
        let cat2_i18n_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CATEGORY2_I18N WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(cat2_i18n_count > 0, "No CATEGORY2_I18N records created");
        assert_eq!(cat2_i18n_count, 40, "Expected 40 CATEGORY2_I18N records (20 Japanese + 20 English)");
        
        let cat3_i18n_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM CATEGORY3_I18N WHERE USER_ID = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(cat3_i18n_count > 0, "No CATEGORY3_I18N records created");
        assert_eq!(cat3_i18n_count, 252, "Expected 252 CATEGORY3_I18N records (126 Japanese + 126 English)");
        
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
        
        let cat2_count_after: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_COUNT)
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
    
    #[tokio::test]
    async fn test_move_category2_order() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        
        // Setup CATEGORY1
        setup_category1(&pool, user_id).await;
        
        // Add three CATEGORY2 entries
        let cat2_code1 = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        let cat2_code2 = service.add_category2(user_id, "EXPENSE", "交通費", "Transportation").await.unwrap();
        let cat2_code3 = service.add_category2(user_id, "EXPENSE", "娯楽費", "Entertainment").await.unwrap();
        
        // Get initial orders
        let order1: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code1)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let order2: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code2)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let order3: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code3)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        // Verify initial order: cat1 < cat2 < cat3
        assert!(order1 < order2);
        assert!(order2 < order3);
        
        // Move cat2 up (swap with cat1)
        service.move_category2_up(user_id, "EXPENSE", &cat2_code2).await.unwrap();
        
        // Check new orders
        let new_order1: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code1)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let new_order2: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code2)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        // cat2 should now be before cat1
        assert_eq!(new_order2, order1);
        assert_eq!(new_order1, order2);
        
        // Move cat2 down (swap back with cat1)
        service.move_category2_down(user_id, "EXPENSE", &cat2_code2).await.unwrap();
        
        // Check orders are back to original
        let final_order1: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code1)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let final_order2: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code2)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(final_order1, order1);
        assert_eq!(final_order2, order2);
    }
    
    #[tokio::test]
    async fn test_move_category3_order() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        
        // Setup CATEGORY1 and CATEGORY2
        setup_category1(&pool, user_id).await;
        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        
        // Add three CATEGORY3 entries
        let cat3_code1 = service.add_category3(user_id, "EXPENSE", &cat2_code, "米", "Rice").await.unwrap();
        let cat3_code2 = service.add_category3(user_id, "EXPENSE", &cat2_code, "野菜", "Vegetables").await.unwrap();
        let _cat3_code3 = service.add_category3(user_id, "EXPENSE", &cat2_code, "肉", "Meat").await.unwrap();
        
        // Get initial orders
        let order1: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY3_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat3_code1)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let order2: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY3_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat3_code2)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        // Verify initial order
        assert!(order1 < order2);
        
        // Move cat3_code2 up
        service.move_category3_up(user_id, "EXPENSE", &cat2_code, &cat3_code2).await.unwrap();
        
        // Check new orders
        let new_order1: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY3_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat3_code1)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let new_order2: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY3_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat3_code2)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        // cat3_code2 should now be before cat3_code1
        assert_eq!(new_order2, order1);
        assert_eq!(new_order1, order2);
        
        // Move cat3_code2 down
        service.move_category3_down(user_id, "EXPENSE", &cat2_code, &cat3_code2).await.unwrap();
        
        // Check orders are back
        let final_order1: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY3_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat3_code1)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        let final_order2: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY3_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat3_code2)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        assert_eq!(final_order1, order1);
        assert_eq!(final_order2, order2);
    }
    
    #[tokio::test]
    async fn test_update_category2() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        
        // Setup
        setup_category1(&pool, user_id).await;
        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        
        // Update both names
        service.update_category2_i18n(user_id, "EXPENSE", &cat2_code, "食費更新", "Food Updated")
            .await
            .unwrap();
        
        // Verify update
        let updated = service.get_category2_for_edit(user_id, "EXPENSE", &cat2_code)
            .await
            .unwrap();
        
        assert_eq!(updated.name_ja, "食費更新");
        assert_eq!(updated.name_en, "Food Updated");
    }
    
    #[tokio::test]
    async fn test_update_category3() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        
        // Setup
        setup_category1(&pool, user_id).await;
        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        let cat3_code = service.add_category3(user_id, "EXPENSE", &cat2_code, "米", "Rice").await.unwrap();
        
        // Update both names
        service.update_category3_i18n(user_id, "EXPENSE", &cat2_code, &cat3_code, "米更新", "Rice Updated")
            .await
            .unwrap();
        
        // Verify update
        let updated = service.get_category3_for_edit(user_id, "EXPENSE", &cat2_code, &cat3_code)
            .await
            .unwrap();
        
        assert_eq!(updated.name_ja, "米更新");
        assert_eq!(updated.name_en, "Rice Updated");
    }
    
    #[tokio::test]
    async fn test_update_category2_duplicate_name() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        
        // Setup
        setup_category1(&pool, user_id).await;
        let _cat2_code1 = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        let cat2_code2 = service.add_category2(user_id, "EXPENSE", "交通費", "Transportation").await.unwrap();
        
        // Try to update cat2_code2 with cat2_code1's name (should fail)
        let result = service.update_category2_i18n(user_id, "EXPENSE", &cat2_code2, "食費", "Food Updated").await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        println!("Error message: {}", err_msg);
        assert!(err_msg.contains("Duplicate") || err_msg.contains("duplicate") || err_msg.contains("already exists"));
    }
    
    #[tokio::test]
    async fn test_move_category2_boundary() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        
        // Setup
        setup_category1(&pool, user_id).await;
        let cat2_code1 = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        let cat2_code2 = service.add_category2(user_id, "EXPENSE", "交通費", "Transportation").await.unwrap();
        
        let order1_before: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code1)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        // Try to move first item up (should be no-op)
        service.move_category2_up(user_id, "EXPENSE", &cat2_code1).await.unwrap();
        
        let order1_after: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code1)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        // Order should remain unchanged
        assert_eq!(order1_before, order1_after);
        
        let order2_before: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code2)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        // Try to move last item down (should be no-op)
        service.move_category2_down(user_id, "EXPENSE", &cat2_code2).await.unwrap();
        
        let order2_after: i64 = sqlx::query_scalar(sql_queries::TEST_CATEGORY2_GET_DISPLAY_ORDER)
            .bind(user_id)
            .bind(&cat2_code2)
            .fetch_one(&pool)
            .await
            .unwrap();
        
        // Order should remain unchanged
        assert_eq!(order2_before, order2_after);
    }
    
    #[tokio::test]
    async fn test_get_category_for_edit() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        
        // Setup
        setup_category1(&pool, user_id).await;
        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
        let cat3_code = service.add_category3(user_id, "EXPENSE", &cat2_code, "米", "Rice").await.unwrap();
        
        // Get category2 for edit
        let cat2_edit = service.get_category2_for_edit(user_id, "EXPENSE", &cat2_code)
            .await
            .unwrap();
        
        assert_eq!(cat2_edit.code, cat2_code);
        assert_eq!(cat2_edit.name_ja, "食費");
        assert_eq!(cat2_edit.name_en, "Food");
        
        // Get category3 for edit
        let cat3_edit = service.get_category3_for_edit(user_id, "EXPENSE", &cat2_code, &cat3_code)
            .await
            .unwrap();
        
        assert_eq!(cat3_edit.code, cat3_code);
        assert_eq!(cat3_edit.name_ja, "米");
        assert_eq!(cat3_edit.name_en, "Rice");
    }

    // Fable-5 review #6 — a stale edit target (row removed from another
    // window between list load and edit-modal open) must surface as
    // `CategoryError::NotFound`, not as a raw sqlx `RowNotFound` string
    // being displayed on screen through the generic error path.
    #[tokio::test]
    async fn test_get_category2_for_edit_returns_not_found_for_missing() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        setup_category1(&pool, user_id).await;

        let result = service.get_category2_for_edit(user_id, "EXPENSE", "NONEXISTENT").await;
        assert!(matches!(result.unwrap_err(), CategoryError::NotFound));
    }

    #[tokio::test]
    async fn test_get_category3_for_edit_returns_not_found_for_missing() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        setup_category1(&pool, user_id).await;
        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();

        let result = service.get_category3_for_edit(user_id, "EXPENSE", &cat2_code, "NONEXISTENT").await;
        assert!(matches!(result.unwrap_err(), CategoryError::NotFound));
    }

    // Fable-5 review #7 — logical delete of a category that is already gone
    // (concurrent removal from another window) must return NotFound so the
    // frontend shows the dedicated not_found toast, not a silent success.
    #[tokio::test]
    async fn test_disable_category2_returns_not_found_for_missing() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        setup_category1(&pool, user_id).await;

        let result = service.disable_category2(user_id, "EXPENSE", "NONEXISTENT").await;
        assert!(matches!(result.unwrap_err(), CategoryError::NotFound));
    }

    #[tokio::test]
    async fn test_disable_category3_returns_not_found_for_missing() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        setup_category1(&pool, user_id).await;
        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();

        let result = service.disable_category3(user_id, "EXPENSE", &cat2_code, "NONEXISTENT").await;
        assert!(matches!(result.unwrap_err(), CategoryError::NotFound));
    }

    // Fable-5 review #7 — the CATEGORY3 disable inside `disable_category2`
    // sweeps children and may legitimately hit zero rows (leaf CATEGORY2);
    // that must not be treated as NotFound as long as the CATEGORY2 itself
    // is disabled successfully.
    #[tokio::test]
    async fn test_disable_category2_succeeds_with_no_children() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id: i64 = 1;
        setup_category1(&pool, user_id).await;
        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();

        let result = service.disable_category2(user_id, "EXPENSE", &cat2_code).await;
        assert!(result.is_ok(), "leaf CATEGORY2 disable should succeed: {:?}", result.err());
    }

    // Issue #37 Phase 2-3 — bounded-field length checks must count
    // characters (not bytes). CATEGORY*_I18N.*_NAME_I18N is VARCHAR(256).

    #[tokio::test]
    async fn test_add_category2_accepts_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 2;
        setup_category1(&pool, user_id).await;

        let result = service.add_category2(
            user_id,
            "EXPENSE",
            &"あ".repeat(consts::MAX_I18N_NAME_LEN),
            &"a".repeat(consts::MAX_I18N_NAME_LEN),
        ).await;
        assert!(result.is_ok(), "expected MAX_I18N_NAME_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_add_category2_rejects_over_max_chars_of_multibyte_ja_name() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 2;
        setup_category1(&pool, user_id).await;

        let result = service.add_category2(
            user_id,
            "EXPENSE",
            &"あ".repeat(consts::MAX_I18N_NAME_LEN + 1),
            "Food",
        ).await;
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_I18N_NAME_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }

    #[tokio::test]
    async fn test_add_category2_rejects_over_max_chars_of_en_name() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 2;
        setup_category1(&pool, user_id).await;

        let result = service.add_category2(
            user_id,
            "EXPENSE",
            "食費",
            &"a".repeat(consts::MAX_I18N_NAME_LEN + 1),
        ).await;
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_I18N_NAME_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }

    #[tokio::test]
    async fn test_add_category3_rejects_over_max_chars_of_multibyte_ja_name() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 2;
        setup_category1(&pool, user_id).await;

        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food")
            .await.unwrap();

        let result = service.add_category3(
            user_id,
            "EXPENSE",
            &cat2_code,
            &"あ".repeat(consts::MAX_I18N_NAME_LEN + 1),
            "Rice",
        ).await;
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_I18N_NAME_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }

    #[tokio::test]
    async fn test_update_category2_i18n_rejects_over_max_chars() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 2;
        setup_category1(&pool, user_id).await;

        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food")
            .await.unwrap();

        let result = service.update_category2_i18n(
            user_id,
            "EXPENSE",
            &cat2_code,
            &"あ".repeat(consts::MAX_I18N_NAME_LEN + 1),
            "Food",
        ).await;
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_I18N_NAME_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }

    #[tokio::test]
    async fn test_update_category3_i18n_rejects_over_max_chars() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 2;
        setup_category1(&pool, user_id).await;

        let cat2_code = service.add_category2(user_id, "EXPENSE", "食費", "Food")
            .await.unwrap();
        let cat3_code = service.add_category3(user_id, "EXPENSE", &cat2_code, "米", "Rice")
            .await.unwrap();

        let result = service.update_category3_i18n(
            user_id,
            "EXPENSE",
            &cat2_code,
            &cat3_code,
            "米",
            &"a".repeat(consts::MAX_I18N_NAME_LEN + 1),
        ).await;
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_I18N_NAME_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }

    // ---- From<CategoryError> for ApiError -------------------------------
    // These tests pin the wire codes that the frontend classifier
    // (`res/js/master-crud.js::mapMasterErrorCode`) matches on. If a
    // variant is renamed here or in api_error.rs, the JS side stops
    // classifying its errors — hence the assertions on the stable
    // `ApiError::CODE_*` constants.

    #[test]
    fn not_found_maps_to_not_found_code_with_category_entity() {
        let err: ApiError = CategoryError::NotFound.into();
        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
        assert_eq!(err.entity.as_deref(), Some("category"));
    }

    #[test]
    fn duplicate_name_maps_to_duplicate_name_code_with_category_entity() {
        let err: ApiError = CategoryError::DuplicateName("食費".to_string()).into();
        assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME);
        assert_eq!(err.entity.as_deref(), Some("category"));
    }

    #[test]
    fn validation_preserves_message_and_omits_entity() {
        let err: ApiError = CategoryError::Validation(
            "Japanese name must be 128 characters or less".to_string()
        ).into();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains("128 characters"));
        assert!(err.entity.is_none());
    }

    #[test]
    fn database_error_maps_to_database_code() {
        let err: ApiError = CategoryError::DatabaseError(sqlx::Error::RowNotFound).into();
        assert_eq!(err.code, ApiError::CODE_DATABASE);
        assert!(err.entity.is_none());
    }

    // PR11 (Fable-5 #31) — regression pins for the get_category_tree
    // and get_category_tree_all refactor from 1+N+N×M queries to a
    // 3-flat-queries + Rust HashMap grouping. Assert the JSON shape
    // preserves the (cat1 → cat2 → cat3) parent/child pairing and
    // per-parent display order.

    #[tokio::test]
    async fn test_get_category_tree_groups_children_under_parent() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 1;
        setup_category1(&pool, user_id).await;

        // Two CATEGORY2 rows under EXPENSE, each with its own CATEGORY3
        // child. If the HashMap grouping ever confused parent scope, a
        // child would land on the wrong cat2.
        let food = service
            .add_category2(user_id, "EXPENSE", "食費", "Food")
            .await
            .unwrap();
        let transport = service
            .add_category2(user_id, "EXPENSE", "交通費", "Transport")
            .await
            .unwrap();
        let rice = service
            .add_category3(user_id, "EXPENSE", &food, "米", "Rice")
            .await
            .unwrap();
        let train = service
            .add_category3(user_id, "EXPENSE", &transport, "電車", "Train")
            .await
            .unwrap();

        let tree = service.get_category_tree(user_id, "ja").await.unwrap();
        let root = tree.as_array().expect("tree root is a JSON array");
        // `setup_category1` seeds EXPENSE + INCOME + TRANSFER; we only
        // populated EXPENSE, so the other two exist with empty children.
        let expense = root
            .iter()
            .find(|n| n["category1"]["category1_code"] == "EXPENSE")
            .expect("EXPENSE cat1 node missing");
        let cat2 = expense["children"].as_array().expect("cat2 children array");
        assert_eq!(cat2.len(), 2, "two cat2 rows expected: {:?}", cat2);

        let food_node = cat2
            .iter()
            .find(|n| n["category2"]["category2_code"] == food.as_str())
            .expect("food cat2 node missing");
        let food_children = food_node["children"].as_array().unwrap();
        assert_eq!(food_children.len(), 1, "food should have 1 cat3");
        assert_eq!(food_children[0]["category3_code"], rice);

        let transport_node = cat2
            .iter()
            .find(|n| n["category2"]["category2_code"] == transport.as_str())
            .expect("transport cat2 node missing");
        let transport_children = transport_node["children"].as_array().unwrap();
        assert_eq!(transport_children.len(), 1, "transport should have 1 cat3");
        assert_eq!(transport_children[0]["category3_code"], train);
    }

    #[tokio::test]
    async fn test_get_category_tree_preserves_display_order() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 1;
        setup_category1(&pool, user_id).await;

        // Add three CATEGORY2 rows; they get DISPLAY_ORDER 1, 2, 3.
        let a = service.add_category2(user_id, "EXPENSE", "A_ja", "A").await.unwrap();
        let b = service.add_category2(user_id, "EXPENSE", "B_ja", "B").await.unwrap();
        let c = service.add_category2(user_id, "EXPENSE", "C_ja", "C").await.unwrap();

        // Move B up so the display order becomes: B, A, C (1, 2, 3).
        service.move_category2_up(user_id, "EXPENSE", &b).await.unwrap();

        let tree = service.get_category_tree(user_id, "ja").await.unwrap();
        let expense = tree.as_array().unwrap()
            .iter()
            .find(|n| n["category1"]["category1_code"] == "EXPENSE")
            .unwrap();
        let cat2 = expense["children"].as_array().unwrap();
        let order: Vec<&str> = cat2
            .iter()
            .map(|n| n["category2"]["category2_code"].as_str().unwrap())
            .collect();
        assert_eq!(order, vec![b.as_str(), a.as_str(), c.as_str()],
                   "display order must survive the flat-query grouping");
    }

    #[tokio::test]
    async fn test_get_category_tree_all_includes_disabled_flags() {
        let pool = setup_test_db().await;
        let service = CategoryService::new(pool.clone());
        let user_id = 1;
        setup_category1(&pool, user_id).await;

        let food = service
            .add_category2(user_id, "EXPENSE", "食費", "Food")
            .await
            .unwrap();
        let rice = service
            .add_category3(user_id, "EXPENSE", &food, "米", "Rice")
            .await
            .unwrap();

        // Disable both to confirm `_all` shows them.
        service.disable_category3(user_id, "EXPENSE", &food, &rice).await.unwrap();
        service.disable_category2(user_id, "EXPENSE", &food).await.unwrap();

        let tree_all = service.get_category_tree_all(user_id, "ja").await.unwrap();
        let expense_all = tree_all.as_array().unwrap()
            .iter()
            .find(|n| n["category1"]["category1_code"] == "EXPENSE")
            .unwrap();
        let cat2 = expense_all["children"].as_array().unwrap();
        let food_node = cat2
            .iter()
            .find(|n| n["category2"]["category2_code"] == food.as_str())
            .expect("disabled cat2 must still appear in _all");
        assert_eq!(
            food_node["category2"]["is_disabled"].as_i64(),
            Some(1),
            "disabled cat2 must carry is_disabled=1 in _all"
        );
        let cat3_children = food_node["children"].as_array().unwrap();
        assert_eq!(cat3_children.len(), 1);
        assert_eq!(
            cat3_children[0]["is_disabled"].as_i64(),
            Some(1),
            "disabled cat3 must carry is_disabled=1 in _all"
        );

        // Sanity: the visible-only tree filters them out.
        let visible = service.get_category_tree(user_id, "ja").await.unwrap();
        let visible_expense = visible.as_array().unwrap()
            .iter()
            .find(|n| n["category1"]["category1_code"] == "EXPENSE")
            .unwrap();
        let visible_cat2 = visible_expense["children"].as_array().unwrap();
        assert!(
            !visible_cat2.iter().any(|n| n["category2"]["category2_code"] == food.as_str()),
            "disabled cat2 must NOT appear in get_category_tree"
        );
    }
}
