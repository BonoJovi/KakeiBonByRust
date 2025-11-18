mod db;
mod validation;
mod consts;
mod security;
mod crypto;
mod settings;
mod sql_queries;
mod services {
    pub mod auth;
    pub mod user_management;
    pub mod encryption;
    pub mod category;
    pub mod i18n;
    pub mod transaction;
    pub mod account;
    pub mod shop;
    pub mod manufacturer;
    pub mod product;
    pub mod session;
}

#[cfg(test)]
mod test_helpers;

#[cfg(test)]
mod validation_tests;

#[cfg(test)]
mod font_size_tests;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Manager;
use db::Database;
use services::auth::AuthService;
use services::user_management::UserManagementService;
use services::encryption::EncryptionService;
use services::i18n::I18nService;
use services::category::CategoryService;
use services::transaction::TransactionService;
use services::session::SessionState;
use settings::SettingsManager;
use validation::{validate_password, validate_password_confirmation};
use crate::consts::{LANG_ENGLISH, LANG_JAPANESE, LANG_DEFAULT, FONT_SIZE_SMALL, FONT_SIZE_MEDIUM, FONT_SIZE_LARGE, FONT_SIZE_DEFAULT};

pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub auth: Arc<Mutex<AuthService>>,
    pub user_mgmt: Arc<Mutex<UserManagementService>>,
    pub encryption: Arc<Mutex<EncryptionService>>,
    pub settings: Arc<Mutex<SettingsManager>>,
    pub i18n: Arc<Mutex<I18nService>>,
    pub category: Arc<Mutex<CategoryService>>,
    pub transaction: Arc<Mutex<TransactionService>>,
    pub session: Arc<SessionState>,
}

#[tauri::command]
async fn login_user(
    username: String,
    password: String,
    state: tauri::State<'_, AppState>
) -> Result<services::session::User, String> {
    let auth = state.auth.lock().await;
    
    match auth.authenticate_user(&username, &password).await {
        Ok(Some(user)) => {
            // Create session user
            let session_user = services::session::User {
                user_id: user.user_id,
                name: user.name.clone(),
                role: user.role,
            };
            
            // Save to session state
            state.session.set_user(session_user.clone());
            
            Ok(session_user)
        }
        Ok(None) => {
            Err("Invalid username or password".to_string())
        }
        Err(e) => {
            Err(format!("Authentication error: {}", e))
        }
    }
}

#[tauri::command]
async fn register_admin(
    username: String,
    password: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    // Validate password
    validate_password(&password)?;
    
    let auth = state.auth.lock().await;
    
    match auth.register_admin_user(&username, &password).await {
        Ok(_) => Ok("Admin user registered successfully".to_string()),
        Err(e) => Err(format!("Registration failed: {}", e)),
    }
}

#[tauri::command]
async fn register_user(
    username: String,
    password: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    // Validate password
    validate_password(&password)?;
    
    let auth = state.auth.lock().await;
    
    match auth.register_user(&username, &password).await {
        Ok(_) => Ok("User registered successfully".to_string()),
        Err(e) => Err(format!("Registration failed: {}", e)),
    }
}

#[tauri::command]
async fn check_needs_setup(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let auth = state.auth.lock().await;
    
    match auth.has_users().await {
        Ok(has_users) => Ok(!has_users),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[tauri::command]
async fn check_needs_user_setup(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let auth = state.auth.lock().await;
    
    match auth.has_general_users().await {
        Ok(has_general_users) => Ok(!has_general_users),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[tauri::command]
fn handle_quit<R: tauri::Runtime>(handle: tauri::AppHandle<R>) {
    handle.cleanup_before_exit();
    handle.exit(0);
}

// ============================================================================
// Session Management Commands
// ============================================================================

#[tauri::command]
fn get_current_session_user(state: tauri::State<'_, AppState>) -> Result<Option<services::session::User>, String> {
    Ok(state.session.get_user())
}

#[tauri::command]
fn is_session_authenticated(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    Ok(state.session.is_authenticated())
}

#[tauri::command]
fn set_session_source_screen(
    source_screen: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    state.session.set_source_screen(source_screen);
    Ok(())
}

#[tauri::command]
fn get_session_source_screen(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    Ok(state.session.get_source_screen())
}

#[tauri::command]
fn clear_session_source_screen(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.session.clear_source_screen();
    Ok(())
}

#[tauri::command]
fn set_session_category1_code(
    category1_code: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    state.session.set_category1_code(category1_code);
    Ok(())
}

#[tauri::command]
fn get_session_category1_code(state: tauri::State<'_, AppState>) -> Result<Option<String>, String> {
    Ok(state.session.get_category1_code())
}

#[tauri::command]
fn clear_session_category1_code(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.session.clear_category1_code();
    Ok(())
}

#[tauri::command]
fn clear_session(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.session.clear_all();
    Ok(())
}

#[tauri::command]
async fn test_db_connection(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().await;
    match sqlx::query(sql_queries::DB_TEST_CONNECTION)
        .fetch_one(db.pool())
        .await
    {
        Ok(_) => Ok("Database connection successful!".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[tauri::command]
fn validate_password_frontend(password: String) -> Result<(), String> {
    validate_password(&password)
}

#[tauri::command]
fn validate_passwords_frontend(password: String, password_confirm: String) -> Result<(), String> {
    validate_password(&password)?;
    validate_password_confirmation(&password, &password_confirm)?;
    Ok(())
}

#[tauri::command]
async fn list_users(state: tauri::State<'_, AppState>) -> Result<Vec<serde_json::Value>, String> {
    let user_mgmt = state.user_mgmt.lock().await;
    
    match user_mgmt.list_users().await {
        Ok(users) => {
            let json_users: Vec<serde_json::Value> = users.into_iter().map(|u| {
                serde_json::json!({
                    "user_id": u.user_id,
                    "name": u.name,
                    "role": u.role,
                    "entry_dt": u.entry_dt,
                    "update_dt": u.update_dt,
                })
            }).collect();
            Ok(json_users)
        }
        Err(e) => Err(format!("Failed to list users: {}", e)),
    }
}

#[tauri::command]
async fn get_user(
    user_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<serde_json::Value, String> {
    let user_mgmt = state.user_mgmt.lock().await;
    
    match user_mgmt.get_user(user_id).await {
        Ok(user) => Ok(serde_json::json!({
            "user_id": user.user_id,
            "name": user.name,
            "role": user.role,
            "entry_dt": user.entry_dt,
            "update_dt": user.update_dt,
        })),
        Err(e) => Err(format!("Failed to get user: {}", e)),
    }
}

#[tauri::command]
async fn create_general_user(
    username: String,
    password: String,
    state: tauri::State<'_, AppState>
) -> Result<i64, String> {
    validate_password(&password)?;
    
    let user_mgmt = state.user_mgmt.lock().await;
    let category = state.category.lock().await;
    
    match user_mgmt.register_general_user(&username, &password).await {
        Ok(user_id) => {
            // Populate default categories for the new user
            if let Err(e) = category.populate_default_categories(user_id).await {
                eprintln!("Warning: Failed to populate default categories for user {}: {}", user_id, e);
                // Continue even if category population fails
            }
            Ok(user_id)
        },
        Err(e) => Err(format!("Failed to create user: {}", e)),
    }
}

#[tauri::command]
async fn update_general_user_info(
    user_id: i64,
    username: Option<String>,
    password: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    if let Some(ref pwd) = password {
        validate_password(pwd)?;
    }
    
    let user_mgmt = state.user_mgmt.lock().await;
    
    match user_mgmt.update_general_user(
        user_id,
        username.as_deref(),
        password.as_deref()
    ).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to update user: {}", e)),
    }
}

#[tauri::command]
async fn update_general_user_with_reencryption(
    user_id: i64,
    old_password: String,
    username: Option<String>,
    new_password: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    if let Some(ref pwd) = new_password {
        validate_password(pwd)?;
    }
    
    let user_mgmt = state.user_mgmt.lock().await;
    
    match user_mgmt.update_general_user_with_password(
        user_id,
        &old_password,
        username.as_deref(),
        new_password.as_deref()
    ).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to update user: {}", e)),
    }
}

#[tauri::command]
async fn update_admin_user_info(
    user_id: i64,
    username: Option<String>,
    password: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    if let Some(ref pwd) = password {
        validate_password(pwd)?;
    }
    
    let user_mgmt = state.user_mgmt.lock().await;
    
    match user_mgmt.update_admin_user(
        user_id,
        username.as_deref(),
        password.as_deref()
    ).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to update admin user: {}", e)),
    }
}

#[tauri::command]
async fn update_admin_user_with_reencryption(
    user_id: i64,
    old_password: String,
    username: Option<String>,
    new_password: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    if let Some(ref pwd) = new_password {
        validate_password(pwd)?;
    }
    
    let user_mgmt = state.user_mgmt.lock().await;
    
    match user_mgmt.update_admin_user_with_password(
        user_id,
        &old_password,
        username.as_deref(),
        new_password.as_deref()
    ).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to update admin user: {}", e)),
    }
}

#[tauri::command]
async fn delete_general_user_info(
    user_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let user_mgmt = state.user_mgmt.lock().await;
    
    match user_mgmt.delete_general_user(user_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to delete user: {}", e)),
    }
}

#[tauri::command]
async fn list_encrypted_fields(
    state: tauri::State<'_, AppState>
) -> Result<Vec<serde_json::Value>, String> {
    let encryption = state.encryption.lock().await;
    
    match encryption.get_encrypted_fields().await {
        Ok(fields) => {
            let json_fields: Vec<serde_json::Value> = fields.into_iter().map(|f| {
                serde_json::json!({
                    "field_id": f.field_id,
                    "table_name": f.table_name,
                    "column_name": f.column_name,
                    "description": f.description,
                    "is_active": f.is_active,
                })
            }).collect();
            Ok(json_fields)
        }
        Err(e) => Err(format!("Failed to list encrypted fields: {}", e)),
    }
}

#[tauri::command]
async fn register_encrypted_field(
    table_name: String,
    column_name: String,
    description: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<i64, String> {
    let encryption = state.encryption.lock().await;
    
    match encryption.register_encrypted_field(
        &table_name,
        &column_name,
        description.as_deref()
    ).await {
        Ok(field_id) => Ok(field_id),
        Err(e) => Err(format!("Failed to register encrypted field: {}", e)),
    }
}

#[tauri::command]
async fn get_setting(
    key: String,
    state: tauri::State<'_, AppState>
) -> Result<Option<serde_json::Value>, String> {
    let settings = state.settings.lock().await;
    Ok(settings.get(&key).cloned())
}

#[tauri::command]
async fn get_setting_string(
    key: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let settings = state.settings.lock().await;
    settings.get_string(&key)
        .map_err(|e| format!("Failed to get setting: {}", e))
}

#[tauri::command]
async fn get_setting_int(
    key: String,
    state: tauri::State<'_, AppState>
) -> Result<i64, String> {
    let settings = state.settings.lock().await;
    settings.get_int(&key)
        .map_err(|e| format!("Failed to get setting: {}", e))
}

#[tauri::command]
async fn get_setting_bool(
    key: String,
    state: tauri::State<'_, AppState>
) -> Result<bool, String> {
    let settings = state.settings.lock().await;
    settings.get_bool(&key)
        .map_err(|e| format!("Failed to get setting: {}", e))
}

#[tauri::command]
async fn set_setting(
    key: String,
    value: serde_json::Value,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let mut settings = state.settings.lock().await;
    settings.set(&key, value)
        .map_err(|e| format!("Failed to set setting: {}", e))?;
    settings.save()
        .map_err(|e| format!("Failed to save settings: {}", e))
}

#[tauri::command]
async fn remove_setting(
    key: String,
    state: tauri::State<'_, AppState>
) -> Result<bool, String> {
    let mut settings = state.settings.lock().await;
    let removed = settings.remove(&key).is_some();
    if removed {
        settings.save()
            .map_err(|e| format!("Failed to save settings: {}", e))?;
    }
    Ok(removed)
}

#[tauri::command]
async fn list_setting_keys(
    state: tauri::State<'_, AppState>
) -> Result<Vec<String>, String> {
    let settings = state.settings.lock().await;
    Ok(settings.keys())
}

#[tauri::command]
async fn reload_settings(
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let mut settings = state.settings.lock().await;
    settings.reload()
        .map_err(|e| format!("Failed to reload settings: {}", e))
}

#[tauri::command]
async fn set_language(
    language: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let mut settings = state.settings.lock().await;
    let i18n = state.i18n.lock().await;
    
    // Validate and normalize language code
    let lang_code = match language.as_str() {
        "English" | "en" => LANG_ENGLISH,
        "日本語" | "Japanese" | "ja" => LANG_JAPANESE,
        _ => return Err("Invalid language".to_string()),
    };
    
    // Save language setting
    settings.set("language", lang_code)
        .map_err(|e| format!("Failed to set language: {}", e))?;
    settings.save()
        .map_err(|e| format!("Failed to save settings: {}", e))?;
    
    // Get localized language name
    let lang_key = format!("lang.name.{}", lang_code);
    let lang_name = i18n.get(&lang_key, lang_code)
        .await
        .unwrap_or_else(|_| lang_code.to_string());
    
    // Get confirmation message with parameter substitution
    let message = i18n.get_with_params("msg.lang_changed", lang_code, &[&lang_name])
        .await
        .unwrap_or_else(|_| format!("Language changed to {}.", lang_name));
    
    Ok(message)
}

#[tauri::command]
async fn get_language(
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let settings = state.settings.lock().await;
    
    match settings.get_string("language") {
        Ok(lang) => Ok(lang),
        Err(_) => Ok(LANG_DEFAULT.to_string()),
    }
}

#[tauri::command]
async fn get_i18n_resource(
    key: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let settings = state.settings.lock().await;
    let i18n = state.i18n.lock().await;
    
    let lang_code = settings.get_string("language")
        .unwrap_or_else(|_| LANG_DEFAULT.to_string());
    
    i18n.get(&key, &lang_code)
        .await
        .map_err(|e| format!("Failed to get resource: {}", e))
}

#[tauri::command]
async fn get_i18n_resources_by_category(
    category: String,
    state: tauri::State<'_, AppState>
) -> Result<std::collections::HashMap<String, String>, String> {
    let settings = state.settings.lock().await;
    let i18n = state.i18n.lock().await;
    
    let lang_code = settings.get_string("language")
        .unwrap_or_else(|_| LANG_DEFAULT.to_string());
    
    i18n.get_by_category(&lang_code, &category)
        .await
        .map_err(|e| format!("Failed to get resources: {}", e))
}

#[tauri::command]
async fn get_translations(
    language: String,
    state: tauri::State<'_, AppState>
) -> Result<std::collections::HashMap<String, String>, String> {
    let i18n = state.i18n.lock().await;
    
    i18n.get_all(&language)
        .await
        .map_err(|e| format!("Failed to get translations: {}", e))
}

#[tauri::command]
async fn get_user_settings(
    state: tauri::State<'_, AppState>
) -> Result<serde_json::Value, String> {
    let settings = state.settings.lock().await;
    
    let mut result = serde_json::Map::new();
    let language = settings.get_string("language")
        .unwrap_or_else(|_| LANG_DEFAULT.to_string());
    result.insert("language".to_string(), serde_json::Value::String(language));
    
    let font_size = settings.get_string("font_size")
        .unwrap_or_else(|_| FONT_SIZE_DEFAULT.to_string());
    result.insert("font_size".to_string(), serde_json::Value::String(font_size));
    
    Ok(serde_json::Value::Object(result))
}

#[tauri::command]
async fn update_user_settings(
    settings: serde_json::Value,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let mut settings_mgr = state.settings.lock().await;
    
    if let Some(obj) = settings.as_object() {
        for (key, value) in obj {
            if let Some(s) = value.as_str() {
                settings_mgr.set(key, s)
                    .map_err(|e| format!("Failed to set {}: {}", key, e))?;
            }
        }
    }
    
    Ok(())
}

#[tauri::command]
async fn get_available_languages(
    state: tauri::State<'_, AppState>
) -> Result<Vec<String>, String> {
    let i18n = state.i18n.lock().await;
    
    i18n.get_available_languages()
        .await
        .map_err(|e| format!("Failed to get available languages: {}", e))
}

#[tauri::command]
async fn get_language_names(
    state: tauri::State<'_, AppState>
) -> Result<Vec<(String, String)>, String> {
    let i18n = state.i18n.lock().await;
    let settings = state.settings.lock().await;
    
    let current_lang = settings.get_string("language")
        .unwrap_or_else(|_| LANG_DEFAULT.to_string());
    
    let lang_codes = i18n.get_available_languages()
        .await
        .map_err(|e| format!("Failed to get available languages: {}", e))?;
    
    let mut language_names = Vec::new();
    for lang_code in lang_codes {
        let key = format!("lang.name.{}", lang_code);
        if let Ok(name) = i18n.get(&key, &current_lang).await {
            language_names.push((lang_code, name));
        }
    }
    
    // Sort by language code to maintain consistent order
    language_names.sort_by(|a, b| a.0.cmp(&b.0));
    
    Ok(language_names)
}

#[tauri::command]
async fn set_font_size(
    font_size: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let mut settings = state.settings.lock().await;
    let i18n = state.i18n.lock().await;
    
    // Validate font size
    let size = match font_size.as_str() {
        FONT_SIZE_SMALL => FONT_SIZE_SMALL.to_string(),
        FONT_SIZE_MEDIUM => FONT_SIZE_MEDIUM.to_string(),
        FONT_SIZE_LARGE => FONT_SIZE_LARGE.to_string(),
        _ => {
            // Try to parse as custom percentage (50-200)
            match font_size.parse::<u32>() {
                Ok(percent) if percent >= 50 && percent <= 200 => font_size.clone(),
                _ => return Err("Invalid font size: must be 'small', 'medium', 'large', or a percentage between 50 and 200".to_string()),
            }
        }
    };
    
    // Save font size setting
    settings.set("font_size", &size)
        .map_err(|e| format!("Failed to set font size: {}", e))?;
    settings.save()
        .map_err(|e| format!("Failed to save settings: {}", e))?;
    
    // Get current language for response message
    let lang_code = settings.get_string("language")
        .unwrap_or_else(|_| LANG_DEFAULT.to_string());
    
    // Get localized font size name
    let size_key = format!("font_size.{}", size);
    let size_name = i18n.get(&size_key, &lang_code)
        .await
        .unwrap_or_else(|_| size.to_string());
    
    // Get confirmation message with parameter substitution
    let message = i18n.get_with_params("msg.font_size_changed", &lang_code, &[&size_name])
        .await
        .unwrap_or_else(|_| format!("Font size changed to {}.", size_name));
    
    Ok(message)
}

#[tauri::command]
async fn get_font_size(
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let settings = state.settings.lock().await;
    
    match settings.get_string("font_size") {
        Ok(size) => Ok(size),
        Err(_) => Ok(FONT_SIZE_DEFAULT.to_string()),
    }
}

#[tauri::command]
async fn adjust_window_size(
    width: f64,
    height: f64,
    window: tauri::Window
) -> Result<(), String> {
    use tauri::LogicalSize;
    
    // Get current window size
    let current_size = window.inner_size()
        .map_err(|e| format!("Failed to get window size: {}", e))?;
    
    // Convert to logical size
    let logical_current = current_size.to_logical::<f64>(window.scale_factor()
        .map_err(|e| format!("Failed to get scale factor: {}", e))?);
    
    // Resize to match content size (both expand and shrink)
    if (width - logical_current.width).abs() > 1.0 || (height - logical_current.height).abs() > 1.0 {
        window.set_size(LogicalSize::new(width, height))
            .map_err(|e| format!("Failed to resize window: {}", e))?;
    }
    
    Ok(())
}

#[tauri::command]
async fn get_category_tree_with_lang(
    user_id: i64,
    lang_code: String,
    state: tauri::State<'_, AppState>
) -> Result<serde_json::Value, String> {
    let category = state.category.lock().await;
    
    category.get_category_tree(user_id, &lang_code)
        .await
        .map_err(|e| format!("Failed to get category tree: {}", e))
}

#[tauri::command]
async fn add_category2(
    user_id: i64,
    category1_code: String,
    name_ja: String,
    name_en: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let category = state.category.lock().await;
    
    category.add_category2(user_id, &category1_code, &name_ja, &name_en)
        .await
        .map_err(|e| format!("Failed to add category2: {}", e))
}

#[tauri::command]
async fn add_category3(
    user_id: i64,
    category1_code: String,
    category2_code: String,
    name_ja: String,
    name_en: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let category = state.category.lock().await;
    
    category.add_category3(user_id, &category1_code, &category2_code, &name_ja, &name_en)
        .await
        .map_err(|e| format!("Failed to add category3: {}", e))
}

#[tauri::command]
async fn get_category2_for_edit(
    user_id: i64,
    category1_code: String,
    category2_code: String,
    state: tauri::State<'_, AppState>
) -> Result<services::category::CategoryForEdit, String> {
    let category = state.category.lock().await;
    category.get_category2_for_edit(user_id, &category1_code, &category2_code)
        .await
        .map_err(|e| format!("Failed to get category2: {}", e))
}

#[tauri::command]
async fn get_category3_for_edit(
    user_id: i64,
    category1_code: String,
    category2_code: String,
    category3_code: String,
    state: tauri::State<'_, AppState>
) -> Result<services::category::CategoryForEdit, String> {
    let category = state.category.lock().await;
    category.get_category3_for_edit(user_id, &category1_code, &category2_code, &category3_code)
        .await
        .map_err(|e| format!("Failed to get category3: {}", e))
}

#[tauri::command]
async fn update_category2(
    user_id: i64,
    category1_code: String,
    category2_code: String,
    name_ja: String,
    name_en: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let category = state.category.lock().await;
    category.update_category2_i18n(user_id, &category1_code, &category2_code, &name_ja, &name_en)
        .await
        .map_err(|e| format!("Failed to update category2: {}", e))
}

#[tauri::command]
async fn update_category3(
    user_id: i64,
    category1_code: String,
    category2_code: String,
    category3_code: String,
    name_ja: String,
    name_en: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let category = state.category.lock().await;
    category.update_category3_i18n(user_id, &category1_code, &category2_code, &category3_code, &name_ja, &name_en)
        .await
        .map_err(|e| format!("Failed to update category3: {}", e))
}

#[tauri::command]
async fn move_category2_up(
    user_id: i64,
    category1_code: String,
    category2_code: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let category = state.category.lock().await;
    category.move_category2_up(user_id, &category1_code, &category2_code)
        .await
        .map_err(|e| format!("Failed to move category2 up: {}", e))
}

#[tauri::command]
async fn move_category2_down(
    user_id: i64,
    category1_code: String,
    category2_code: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let category = state.category.lock().await;
    category.move_category2_down(user_id, &category1_code, &category2_code)
        .await
        .map_err(|e| format!("Failed to move category2 down: {}", e))
}

#[tauri::command]
async fn move_category3_up(
    user_id: i64,
    category1_code: String,
    category2_code: String,
    category3_code: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let category = state.category.lock().await;
    category.move_category3_up(user_id, &category1_code, &category2_code, &category3_code)
        .await
        .map_err(|e| format!("Failed to move category3 up: {}", e))
}

#[tauri::command]
async fn move_category3_down(
    user_id: i64,
    category1_code: String,
    category2_code: String,
    category3_code: String,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let category = state.category.lock().await;
    category.move_category3_down(user_id, &category1_code, &category2_code, &category3_code)
        .await
        .map_err(|e| format!("Failed to move category3 down: {}", e))
}

// ============================================================================
// Transaction Management Commands
// ============================================================================

#[tauri::command]
async fn add_transaction(
    user_id: i64,
    transaction_date: String,
    category1_code: String,
    category2_code: String,
    category3_code: String,
    amount: i64,
    description: Option<String>,
    memo: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<i64, String> {
    let transaction = state.transaction.lock().await;
    transaction.add_transaction(
        user_id,
        &transaction_date,
        &category1_code,
        &category2_code,
        &category3_code,
        amount,
        description.as_deref(),
        memo.as_deref(),
    )
    .await
    .map_err(|e| format!("Failed to add transaction: {}", e))
}

#[tauri::command]
async fn get_transaction(
    user_id: i64,
    transaction_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<services::transaction::Transaction, String> {
    let transaction = state.transaction.lock().await;
    transaction.get_transaction(user_id, transaction_id)
        .await
        .map_err(|e| format!("Failed to get transaction: {}", e))
}

#[tauri::command]
async fn get_transactions(
    user_id: i64,
    start_date: Option<String>,
    end_date: Option<String>,
    category1_code: Option<String>,
    category2_code: Option<String>,
    category3_code: Option<String>,
    min_amount: Option<i64>,
    max_amount: Option<i64>,
    keyword: Option<String>,
    page: i64,
    per_page: i64,
    state: tauri::State<'_, AppState>
) -> Result<services::transaction::TransactionListResponse, String> {
    let transaction = state.transaction.lock().await;
    transaction.get_transactions(
        user_id,
        start_date.as_deref(),
        end_date.as_deref(),
        category1_code.as_deref(),
        category2_code.as_deref(),
        category3_code.as_deref(),
        min_amount,
        max_amount,
        keyword.as_deref(),
        page,
        per_page,
    )
    .await
    .map_err(|e| format!("Failed to get transactions: {}", e))
}

#[tauri::command]
async fn update_transaction(
    user_id: i64,
    transaction_id: i64,
    transaction_date: String,
    category1_code: String,
    category2_code: String,
    category3_code: String,
    amount: i64,
    description: Option<String>,
    memo: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let transaction = state.transaction.lock().await;
    transaction.update_transaction(
        user_id,
        transaction_id,
        &transaction_date,
        &category1_code,
        &category2_code,
        &category3_code,
        amount,
        description.as_deref(),
        memo.as_deref(),
    )
    .await
    .map_err(|e| format!("Failed to update transaction: {}", e))
}

#[tauri::command]
async fn delete_transaction(
    user_id: i64,
    transaction_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let transaction = state.transaction.lock().await;
    transaction.delete_transaction(user_id, transaction_id)
        .await
        .map_err(|e| format!("Failed to delete transaction: {}", e))
}

// ============================================================================
// Account Management Commands
// ============================================================================

#[tauri::command]
async fn get_account_templates(
    state: tauri::State<'_, AppState>
) -> Result<Vec<services::account::AccountTemplate>, String> {
    let db = state.db.lock().await;
    services::account::get_account_templates(db.pool()).await
}

#[tauri::command]
async fn get_accounts(
    user_id: i64,
    user_role: i64,
    state: tauri::State<'_, AppState>
) -> Result<Vec<services::account::Account>, String> {
    let db = state.db.lock().await;
    
    // Admin (role 0) can see all accounts, regular users see only their own
    if user_role == crate::consts::ROLE_ADMIN {
        services::account::get_all_accounts(db.pool()).await
    } else {
        services::account::get_accounts(db.pool(), user_id).await
    }
}

#[tauri::command]
async fn add_account(
    user_id: i64,
    account_code: String,
    account_name: String,
    template_code: String,
    initial_balance: i64,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;
    
    let request = services::account::AddAccountRequest {
        account_code,
        account_name,
        template_code,
        initial_balance,
    };
    
    services::account::add_account(db.pool(), user_id, request).await
}

#[tauri::command]
async fn update_account(
    user_id: i64,
    account_code: String,
    account_name: String,
    template_code: String,
    initial_balance: i64,
    display_order: i64,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;
    
    let request = services::account::UpdateAccountRequest {
        account_code,
        account_name,
        template_code,
        initial_balance,
        display_order,
    };
    
    services::account::update_account(db.pool(), user_id, request).await
}

#[tauri::command]
async fn delete_account(
    user_id: i64,
    account_code: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;
    
    services::account::delete_account(db.pool(), user_id, &account_code).await
}

// ============================================================================
// Shop Management Commands
// ============================================================================

#[tauri::command]
async fn get_shops(
    user_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<Vec<services::shop::Shop>, String> {
    let db = state.db.lock().await;
    services::shop::get_shops(db.pool(), user_id).await
}

#[tauri::command]
async fn add_shop(
    user_id: i64,
    shop_name: String,
    memo: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;

    let request = services::shop::AddShopRequest {
        shop_name,
        memo,
    };

    services::shop::add_shop(db.pool(), user_id, request).await
}

#[tauri::command]
async fn update_shop(
    user_id: i64,
    shop_id: i64,
    shop_name: String,
    memo: Option<String>,
    display_order: i64,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;

    let request = services::shop::UpdateShopRequest {
        shop_name,
        memo,
        display_order,
    };

    services::shop::update_shop(db.pool(), user_id, shop_id, request).await
}

#[tauri::command]
async fn delete_shop(
    user_id: i64,
    shop_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;
    services::shop::delete_shop(db.pool(), user_id, shop_id).await
}

// ============================================================================
// Manufacturer Management Commands
// ============================================================================

#[tauri::command]
async fn get_manufacturers(
    user_id: i64,
    include_disabled: bool,
    state: tauri::State<'_, AppState>
) -> Result<Vec<services::manufacturer::Manufacturer>, String> {
    let db = state.db.lock().await;
    services::manufacturer::get_manufacturers(db.pool(), user_id, include_disabled).await
}

#[tauri::command]
async fn add_manufacturer(
    user_id: i64,
    manufacturer_name: String,
    memo: Option<String>,
    is_disabled: Option<i64>,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;

    let request = services::manufacturer::AddManufacturerRequest {
        manufacturer_name,
        memo,
        is_disabled,
    };

    services::manufacturer::add_manufacturer(db.pool(), user_id, request).await
}

#[tauri::command]
async fn update_manufacturer(
    user_id: i64,
    manufacturer_id: i64,
    manufacturer_name: String,
    memo: Option<String>,
    display_order: i64,
    is_disabled: i64,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;

    let request = services::manufacturer::UpdateManufacturerRequest {
        manufacturer_name,
        memo,
        display_order,
        is_disabled,
    };

    services::manufacturer::update_manufacturer(db.pool(), user_id, manufacturer_id, request).await
}

#[tauri::command]
async fn delete_manufacturer(
    user_id: i64,
    manufacturer_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;
    services::manufacturer::delete_manufacturer(db.pool(), user_id, manufacturer_id).await
}

// ============================================================================
// Product Management Commands
// ============================================================================

#[tauri::command]
async fn get_products(
    user_id: i64,
    include_disabled: bool,
    state: tauri::State<'_, AppState>
) -> Result<Vec<services::product::Product>, String> {
    let db = state.db.lock().await;
    services::product::get_products(db.pool(), user_id, include_disabled).await
}

#[tauri::command]
async fn add_product(
    user_id: i64,
    product_name: String,
    manufacturer_id: Option<i64>,
    memo: Option<String>,
    is_disabled: Option<i64>,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;

    let request = services::product::AddProductRequest {
        product_name,
        manufacturer_id,
        memo,
        is_disabled,
    };

    services::product::add_product(db.pool(), user_id, request).await
}

#[tauri::command]
async fn update_product(
    user_id: i64,
    product_id: i64,
    product_name: String,
    manufacturer_id: Option<i64>,
    memo: Option<String>,
    display_order: i64,
    is_disabled: i64,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;

    let request = services::product::UpdateProductRequest {
        product_name,
        manufacturer_id,
        memo,
        display_order,
        is_disabled,
    };

    services::product::update_product(db.pool(), user_id, product_id, request).await
}

#[tauri::command]
async fn delete_product(
    user_id: i64,
    product_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let db = state.db.lock().await;
    services::product::delete_product(db.pool(), user_id, product_id).await
}

// ============================================================================
// Transaction Management Commands
// ============================================================================

#[tauri::command]
async fn save_transaction_header(
    user_id: i64,
    shop_id: Option<i64>,
    category1_code: String,
    from_account_code: String,
    to_account_code: String,
    transaction_date: String,
    total_amount: i64,
    tax_rounding_type: i64,
    tax_included_type: i64,
    memo: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<i64, String> {
    let transaction = state.transaction.lock().await;

    let request = services::transaction::SaveTransactionRequest {
        shop_id,
        category1_code,
        from_account_code,
        to_account_code,
        transaction_date,
        total_amount,
        tax_rounding_type,
        tax_included_type,
        memo,
    };

    transaction.save_transaction_header(user_id, request).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transaction_header(
    transaction_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<serde_json::Value, String> {
    let transaction = state.transaction.lock().await;
    // TODO: Get user_id from session/auth
    // For now, use user_id = 2 to match frontend currentUserId
    let user_id = 2;
    
    let (header, memo_text) = transaction.get_transaction_header_with_memo(user_id, transaction_id).await
        .map_err(|e| e.to_string())?;
    
    Ok(serde_json::json!({
        "transaction_id": header.transaction_id,
        "user_id": header.user_id,
        "shop_id": header.shop_id,
        "transaction_date": header.transaction_date,
        "category1_code": header.category1_code,
        "from_account_code": header.from_account_code,
        "to_account_code": header.to_account_code,
        "total_amount": header.total_amount,
        "tax_rounding_type": header.tax_rounding_type,
        "memo_id": header.memo_id,
        "is_disabled": header.is_disabled,
        "entry_dt": header.entry_dt,
        "update_dt": header.update_dt,
        "memo": memo_text
    }))
}

#[tauri::command]
async fn select_transaction_headers(
    transaction_ids: Vec<i64>,
    state: tauri::State<'_, AppState>
) -> Result<Vec<services::transaction::TransactionHeader>, String> {
    let transaction = state.transaction.lock().await;
    // TODO: Get user_id from session/auth
    // For now, use user_id = 2 to match frontend currentUserId
    let user_id = 2;
    
    let mut headers = Vec::new();
    for transaction_id in transaction_ids {
        match transaction.get_transaction_header(user_id, transaction_id).await {
            Ok(header) => headers.push(header),
            Err(_) => continue, // Skip not found transactions
        }
    }
    Ok(headers)
}

#[tauri::command]
async fn update_transaction_header(
    transaction_id: i64,
    shop_id: Option<i64>,
    category1_code: String,
    from_account_code: String,
    to_account_code: String,
    transaction_date: String,
    total_amount: i64,
    tax_rounding_type: i64,
    tax_included_type: i64,
    memo: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let transaction = state.transaction.lock().await;
    // TODO: Get user_id from session/auth
    // For now, use user_id = 2 to match frontend currentUserId
    let user_id = 2;

    let request = services::transaction::SaveTransactionRequest {
        shop_id,
        category1_code,
        from_account_code,
        to_account_code,
        transaction_date,
        total_amount,
        tax_rounding_type,
        tax_included_type,
        memo,
    };

    transaction.update_transaction_header(user_id, transaction_id, request).await
        .map_err(|e| e.to_string())
}

// ============================================================================
// Transaction Detail Management Commands
// ============================================================================

#[tauri::command]
async fn get_transaction_header_with_info(
    transaction_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<services::transaction::TransactionHeaderWithInfo, String> {
    let transaction = state.transaction.lock().await;
    // TODO: Get user_id from session/auth
    let user_id = 2;

    transaction.get_transaction_header_with_info(user_id, transaction_id).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_transaction_details(
    transaction_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<Vec<services::transaction::TransactionDetailWithInfo>, String> {
    let transaction = state.transaction.lock().await;
    // TODO: Get user_id from session/auth
    let user_id = 2;

    transaction.get_transaction_details(user_id, transaction_id).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_transaction_detail(
    transaction_id: i64,
    category1_code: String,
    category2_code: String,
    category3_code: String,
    item_name: String,
    amount: i64,
    tax_rate: i32,
    tax_amount: i64,
    amount_including_tax: Option<i64>,
    memo: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<i64, String> {
    let transaction = state.transaction.lock().await;
    // TODO: Get user_id from session/auth
    let user_id = 2;

    let request = services::transaction::SaveTransactionDetailRequest {
        detail_id: None,
        category1_code,
        category2_code,
        category3_code,
        item_name,
        amount,
        tax_rate,
        tax_amount,
        amount_including_tax,
        memo,
    };

    transaction.add_transaction_detail(user_id, transaction_id, request).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_transaction_detail(
    detail_id: i64,
    category1_code: String,
    category2_code: String,
    category3_code: String,
    item_name: String,
    amount: i64,
    tax_rate: i32,
    tax_amount: i64,
    amount_including_tax: Option<i64>,
    memo: Option<String>,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let transaction = state.transaction.lock().await;
    // TODO: Get user_id from session/auth
    let user_id = 2;

    let request = services::transaction::SaveTransactionDetailRequest {
        detail_id: Some(detail_id),
        category1_code,
        category2_code,
        category3_code,
        item_name,
        amount,
        tax_rate,
        tax_amount,
        amount_including_tax,
        memo,
    };

    transaction.update_transaction_detail(user_id, detail_id, request).await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_transaction_detail(
    detail_id: i64,
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let transaction = state.transaction.lock().await;
    // TODO: Get user_id from session/auth
    let user_id = 2;

    transaction.delete_transaction_detail(user_id, detail_id).await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login_user,
            register_admin,
            register_user,
            check_needs_setup,
            check_needs_user_setup,
            handle_quit,
            get_current_session_user,
            is_session_authenticated,
            set_session_source_screen,
            get_session_source_screen,
            clear_session_source_screen,
            set_session_category1_code,
            get_session_category1_code,
            clear_session_category1_code,
            clear_session,
            test_db_connection,
            validate_password_frontend,
            validate_passwords_frontend,
            list_users,
            get_user,
            create_general_user,
            update_general_user_info,
            update_general_user_with_reencryption,
            update_admin_user_info,
            update_admin_user_with_reencryption,
            delete_general_user_info,
            list_encrypted_fields,
            register_encrypted_field,
            get_setting,
            get_setting_string,
            get_setting_int,
            get_setting_bool,
            set_setting,
            remove_setting,
            list_setting_keys,
            reload_settings,
            set_language,
            get_language,
            get_i18n_resource,
            get_i18n_resources_by_category,
            get_translations,
            get_user_settings,
            update_user_settings,
            get_available_languages,
            get_language_names,
            set_font_size,
            get_font_size,
            adjust_window_size,
            get_category_tree_with_lang,
            add_category2,
            add_category3,
            get_category2_for_edit,
            get_category3_for_edit,
            update_category2,
            update_category3,
            move_category2_up,
            move_category2_down,
            move_category3_up,
            move_category3_down,
            add_transaction,
            get_transaction,
            get_transactions,
            update_transaction,
            delete_transaction,
            get_account_templates,
            get_accounts,
            add_account,
            update_account,
            delete_account,
            get_shops,
            add_shop,
            update_shop,
            delete_shop,
            get_manufacturers,
            add_manufacturer,
            update_manufacturer,
            delete_manufacturer,
            get_products,
            add_product,
            update_product,
            delete_product,
            save_transaction_header,
            get_transaction_header,
            select_transaction_headers,
            update_transaction_header,
            get_transaction_header_with_info,
            get_transaction_details,
            add_transaction_detail,
            update_transaction_detail,
            delete_transaction_detail
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize database
            let rt = tokio::runtime::Runtime::new().unwrap();
            let (db, auth, user_mgmt, encryption, settings, i18n, category, transaction) = rt.block_on(async {
                let database = Database::new().await
                    .expect("Failed to connect to database");
                database.initialize().await
                    .expect("Failed to initialize database");

                // Run transaction-related table migrations
                database.migrate_transactions().await
                    .expect("Failed to migrate transaction tables");

                let auth_service = AuthService::new(database.pool().clone());
                let user_mgmt_service = UserManagementService::new(database.pool().clone());
                let encryption_service = EncryptionService::new(database.pool().clone());
                let settings_manager = SettingsManager::new()
                    .expect("Failed to initialize settings");
                let i18n_service = I18nService::new(database.pool().clone());
                let category_service = CategoryService::new(database.pool().clone());
                let transaction_service = TransactionService::new(database.pool().clone());
                
                (database, auth_service, user_mgmt_service, encryption_service, settings_manager, i18n_service, category_service, transaction_service)
            });

            app.manage(AppState {
                db: Arc::new(Mutex::new(db)),
                auth: Arc::new(Mutex::new(auth)),
                user_mgmt: Arc::new(Mutex::new(user_mgmt)),
                encryption: Arc::new(Mutex::new(encryption)),
                settings: Arc::new(Mutex::new(settings)),
                i18n: Arc::new(Mutex::new(i18n)),
                category: Arc::new(Mutex::new(category)),
                transaction: Arc::new(Mutex::new(transaction)),
                session: Arc::new(SessionState::new()),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
