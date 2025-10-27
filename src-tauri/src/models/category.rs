use serde::{Deserialize, Serialize};

/// Category1 (Major category / 大分類)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category1 {
    pub user_id: i64,
    pub category1_code: String,
    pub display_order: i32,
    pub category1_name: String,
    pub category1_name_i18n: Option<String>,  // Multilingual name
    pub is_disabled: bool,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

/// Category2 (Middle category / 中分類)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category2 {
    pub user_id: i64,
    pub category1_code: String,
    pub category2_code: String,
    pub display_order: i32,
    pub category2_name: String,
    pub category2_name_i18n: Option<String>,  // Multilingual name
    pub is_disabled: bool,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

/// Category3 (Minor category / 小分類)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category3 {
    pub user_id: i64,
    pub category1_code: String,
    pub category2_code: String,
    pub category3_code: String,
    pub display_order: i32,
    pub category3_name: String,
    pub category3_name_i18n: Option<String>,  // Multilingual name
    pub is_disabled: bool,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

/// Category tree structure for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryTree {
    pub category1: Category1,
    pub children: Vec<Category2WithChildren>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category2WithChildren {
    pub category2: Category2,
    pub children: Vec<Category3>,
}
