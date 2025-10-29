use crate::db::category;
use crate::models::category::CategoryTree;

/// Get category tree for a user
#[tauri::command]
pub fn get_category_tree(user_id: i64) -> Result<Vec<CategoryTree>, String> {
    category::get_category_tree(user_id)
        .map_err(|e| format!("Failed to get category tree: {}", e))
}

/// Get category tree for a user with language support
#[tauri::command]
pub fn get_category_tree_with_lang(user_id: i64, lang_code: Option<String>) -> Result<Vec<CategoryTree>, String> {
    category::get_category_tree_with_lang(user_id, lang_code)
        .map_err(|e| format!("Failed to get category tree: {}", e))
}

/// Add new Category1
#[tauri::command]
pub fn add_category1(user_id: i64, code: String, name: String) -> Result<(), String> {
    category::add_category1(user_id, code, name)
        .map_err(|e| format!("Failed to add category1: {}", e))
}

/// Update Category1
#[tauri::command]
pub fn update_category1(user_id: i64, code: String, name: String) -> Result<(), String> {
    category::update_category1(user_id, code, name)
        .map_err(|e| format!("Failed to update category1: {}", e))
}

/// Move Category1 order
#[tauri::command]
pub fn move_category1_order(user_id: i64, code: String, direction: i32) -> Result<(), String> {
    category::move_category1_order(user_id, code, direction)
        .map_err(|e| format!("Failed to move category1 order: {}", e))
}

/// Delete Category1
#[tauri::command]
pub fn delete_category1(user_id: i64, code: String) -> Result<(), String> {
    category::delete_category1(user_id, code)
        .map_err(|e| format!("Failed to delete category1: {}", e))
}

/// Add new Category2
#[tauri::command]
pub fn add_category2(user_id: i64, category1_code: String, name_ja: String, name_en: String) -> Result<(), String> {
    category::add_category2_i18n(user_id, category1_code, name_ja, name_en)
        .map_err(|e| format!("Failed to add category2: {}", e))
}

/// Update Category2
#[tauri::command]
pub fn update_category2(user_id: i64, category1_code: String, category2_code: String, name_ja: String, name_en: String) -> Result<(), String> {
    category::update_category2_i18n(user_id, category1_code, category2_code, name_ja, name_en)
        .map_err(|e| format!("Failed to update category2: {}", e))
}

/// Move Category2 order
#[tauri::command]
pub fn move_category2_order(user_id: i64, category1_code: String, category2_code: String, direction: i32) -> Result<(), String> {
    category::move_category2_order(user_id, category1_code, category2_code, direction)
        .map_err(|e| format!("Failed to move category2 order: {}", e))
}

/// Delete Category2
#[tauri::command]
pub fn delete_category2(user_id: i64, category1_code: String, category2_code: String) -> Result<(), String> {
    category::delete_category2(user_id, category1_code, category2_code)
        .map_err(|e| format!("Failed to delete category2: {}", e))
}

/// Add new Category3
#[tauri::command]
pub fn add_category3(user_id: i64, category1_code: String, category2_code: String, name_ja: String, name_en: String) -> Result<(), String> {
    category::add_category3_i18n(user_id, category1_code, category2_code, name_ja, name_en)
        .map_err(|e| format!("Failed to add category3: {}", e))
}

/// Update Category3
#[tauri::command]
pub fn update_category3(user_id: i64, category1_code: String, category2_code: String, category3_code: String, name_ja: String, name_en: String) -> Result<(), String> {
    category::update_category3_i18n(user_id, category1_code, category2_code, category3_code, name_ja, name_en)
        .map_err(|e| format!("Failed to update category3: {}", e))
}

/// Move Category3 order
#[tauri::command]
pub fn move_category3_order(user_id: i64, category1_code: String, category2_code: String, category3_code: String, direction: i32) -> Result<(), String> {
    category::move_category3_order(user_id, category1_code, category2_code, category3_code, direction)
        .map_err(|e| format!("Failed to move category3 order: {}", e))
}

/// Delete Category3
#[tauri::command]
pub fn delete_category3(user_id: i64, category1_code: String, category2_code: String, category3_code: String) -> Result<(), String> {
    category::delete_category3(user_id, category1_code, category2_code, category3_code)
        .map_err(|e| format!("Failed to delete category3: {}", e))
}

/// Get Category2 data for editing
#[tauri::command]
pub fn get_category2_for_edit(user_id: i64, category1_code: String, category2_code: String) -> Result<serde_json::Value, String> {
    category::get_category2_for_edit(user_id, category1_code, category2_code)
        .map_err(|e| format!("Failed to get category2 for edit: {}", e))
}

/// Get Category3 data for editing
#[tauri::command]
pub fn get_category3_for_edit(user_id: i64, category1_code: String, category2_code: String, category3_code: String) -> Result<serde_json::Value, String> {
    category::get_category3_for_edit(user_id, category1_code, category2_code, category3_code)
        .map_err(|e| format!("Failed to get category3 for edit: {}", e))
}

/// Initialize categories for a new user
#[tauri::command]
pub fn initialize_categories_for_new_user(user_id: i64) -> Result<(), String> {
    category::initialize_categories_for_new_user(user_id)
        .map_err(|e| format!("Failed to initialize categories: {}", e))
}

/// Get category2 data for editing
#[tauri::command]
pub fn get_category2_for_edit(user_id: i64, category1_code: String, category2_code: String) -> Result<serde_json::Value, String> {
    category::get_category2_for_edit(user_id, category1_code, category2_code)
        .map_err(|e| format!("Failed to get category2: {}", e))
}

/// Get category3 data for editing
#[tauri::command]
pub fn get_category3_for_edit(user_id: i64, category1_code: String, category2_code: String, category3_code: String) -> Result<serde_json::Value, String> {
    category::get_category3_for_edit(user_id, category1_code, category2_code, category3_code)
        .map_err(|e| format!("Failed to get category3: {}", e))
}
