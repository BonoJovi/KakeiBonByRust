mod db;
mod validation;
mod consts;
mod security;
mod crypto;
mod settings;
mod services {
    pub mod auth;
    pub mod user_management;
    pub mod encryption;
    pub mod category;
    pub mod i18n;
}

#[cfg(test)]
mod test_helpers;

#[cfg(test)]
mod validation_tests;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Manager;
use db::Database;
use services::auth::AuthService;
use services::user_management::UserManagementService;
use services::encryption::EncryptionService;
use services::i18n::I18nService;
use settings::SettingsManager;
use validation::{validate_password, validate_password_confirmation};
use crate::consts::{LANG_ENGLISH, LANG_JAPANESE, LANG_DEFAULT};

pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub auth: Arc<Mutex<AuthService>>,
    pub user_mgmt: Arc<Mutex<UserManagementService>>,
    pub encryption: Arc<Mutex<EncryptionService>>,
    pub settings: Arc<Mutex<SettingsManager>>,
    pub i18n: Arc<Mutex<I18nService>>,
}

#[tauri::command]
async fn login_user(
    username: String,
    password: String,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    let auth = state.auth.lock().await;
    
    match auth.authenticate_user(&username, &password).await {
        Ok(Some(user)) => {
            Ok(format!("Welcome, {}!", user.name))
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

#[tauri::command]
async fn test_db_connection(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().await;
    match sqlx::query("SELECT 1 as test")
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
    
    match user_mgmt.register_general_user(&username, &password).await {
        Ok(user_id) => Ok(user_id),
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
            get_language_names
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
            let (db, auth, user_mgmt, encryption, settings, i18n) = rt.block_on(async {
                let database = Database::new().await
                    .expect("Failed to connect to database");
                database.initialize().await
                    .expect("Failed to initialize database");
                
                let auth_service = AuthService::new(database.pool().clone());
                let user_mgmt_service = UserManagementService::new(database.pool().clone());
                let encryption_service = EncryptionService::new(database.pool().clone());
                let settings_manager = SettingsManager::new()
                    .expect("Failed to initialize settings");
                let i18n_service = I18nService::new(database.pool().clone());
                
                (database, auth_service, user_mgmt_service, encryption_service, settings_manager, i18n_service)
            });

            app.manage(AppState {
                db: Arc::new(Mutex::new(db)),
                auth: Arc::new(Mutex::new(auth)),
                user_mgmt: Arc::new(Mutex::new(user_mgmt)),
                encryption: Arc::new(Mutex::new(encryption)),
                settings: Arc::new(Mutex::new(settings)),
                i18n: Arc::new(Mutex::new(i18n)),
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
