use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub language: String,
    pub font_size: String,
}

#[tauri::command]
pub fn get_user_settings() -> Result<UserSettings, String> {
    // TODO: Get from database or config file
    // For now, return default settings
    Ok(UserSettings {
        language: "ja".to_string(),
        font_size: "medium".to_string(),
    })
}

#[tauri::command]
pub fn update_user_settings(settings: UserSettings) -> Result<(), String> {
    // TODO: Save to database or config file
    println!("Settings updated: {:?}", settings);
    Ok(())
}
