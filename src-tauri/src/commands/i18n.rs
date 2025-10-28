use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::db::i18n::get_all_translations;

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationsResponse {
    pub translations: HashMap<String, String>,
    pub debug_info: TranslationsDebugInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationsDebugInfo {
    pub total_count: usize,
    pub has_menu_admin: bool,
    pub has_menu_font_size: bool,
    pub menu_admin_value: Option<String>,
    pub sample_keys: Vec<String>,
}

fn log_to_file(message: &str) {
    let log_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("work")
        .join("i18n_debug.log");
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let _ = writeln!(file, "[{}] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), message);
    } else {
        eprintln!("Failed to open log file: {:?}", log_path);
    }
}

#[tauri::command]
pub fn get_translations(language: String) -> Result<HashMap<String, String>, String> {
    let translations = get_all_translations(&language)
        .map_err(|e| {
            format!("Failed to get translations: {}", e)
        })?;
    
    // Create a new HashMap with test key FIRST
    let mut result = HashMap::new();
    result.insert("TEST_KEY_123".to_string(), "TEST_VALUE_WORKS".to_string());
    
    // Check counts BEFORE processing
    let original_count = translations.len();
    let has_menu_admin_before = translations.contains_key("menu.admin");
    let menu_admin_value = translations.get("menu.admin").cloned().unwrap_or_else(|| "NOT_FOUND".to_string());
    
    let menu_keys: Vec<String> = translations.keys()
        .filter(|k| k.starts_with("menu."))
        .cloned()
        .collect();
    
    // Add debug info
    result.insert("DEBUG_ORIGINAL_COUNT".to_string(), original_count.to_string());
    result.insert("DEBUG_HAS_MENU_ADMIN_BEFORE".to_string(), has_menu_admin_before.to_string());
    result.insert("DEBUG_MENU_ADMIN_VALUE".to_string(), menu_admin_value);
    result.insert("DEBUG_MENU_KEYS_COUNT".to_string(), menu_keys.len().to_string());
    result.insert("DEBUG_MENU_KEYS".to_string(), menu_keys.join(", "));
    
    // Add all original translations
    for (key, value) in translations {
        result.insert(key, value);
    }
    
    // Check count AFTER adding debug keys
    let final_count = result.len();
    let has_menu_admin_after = result.contains_key("menu.admin");
    result.insert("DEBUG_FINAL_COUNT".to_string(), final_count.to_string());
    result.insert("DEBUG_HAS_MENU_ADMIN_AFTER".to_string(), has_menu_admin_after.to_string());
    
    Ok(result)
}

#[tauri::command]
pub fn get_translations_debug(language: String) -> Result<TranslationsResponse, String> {
    log_to_file(&format!("get_translations_debug called with language: {}", language));
    
    let translations = get_all_translations(&language)
        .map_err(|e| format!("Failed to get translations: {}", e))?;
    
    let menu_admin_value = translations.get("menu.admin").cloned();
    let sample_keys: Vec<String> = translations.keys()
        .filter(|k| k.starts_with("menu."))
        .take(10)
        .cloned()
        .collect();
    
    let debug_info = TranslationsDebugInfo {
        total_count: translations.len(),
        has_menu_admin: translations.contains_key("menu.admin"),
        has_menu_font_size: translations.contains_key("menu.font_size"),
        menu_admin_value: menu_admin_value.clone(),
        sample_keys: sample_keys.clone(),
    };
    
    log_to_file(&format!("Debug info: total={}, has_menu_admin={}, menu_admin_value={:?}", 
        debug_info.total_count, debug_info.has_menu_admin, menu_admin_value));
    log_to_file(&format!("Sample keys: {:?}", sample_keys));
    
    Ok(TranslationsResponse {
        translations,
        debug_info,
    })
}
