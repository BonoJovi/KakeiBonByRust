use std::collections::HashMap;
use crate::db::i18n::get_all_translations;

#[tauri::command]
pub fn get_translations(language: String) -> Result<HashMap<String, String>, String> {
    get_all_translations(&language)
        .map_err(|e| format!("Failed to get translations: {}", e))
}

