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

/// Initialize categories for a new user
#[tauri::command]
pub fn initialize_categories_for_new_user(user_id: i64) -> Result<(), String> {
    category::initialize_categories_for_new_user(user_id)
        .map_err(|e| format!("Failed to initialize categories: {}", e))
}
