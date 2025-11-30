use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserSettings {
    #[serde(flatten)]
    entries: HashMap<String, serde_json::Value>,
}

#[derive(Debug)]
pub enum SettingsError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    EntryNotFound(String),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::IoError(e) => write!(f, "IO error: {}", e),
            SettingsError::JsonError(e) => write!(f, "JSON error: {}", e),
            SettingsError::EntryNotFound(key) => write!(f, "Entry not found: {}", key),
        }
    }
}

impl std::error::Error for SettingsError {}

impl From<std::io::Error> for SettingsError {
    fn from(err: std::io::Error) -> Self {
        SettingsError::IoError(err)
    }
}

impl From<serde_json::Error> for SettingsError {
    fn from(err: serde_json::Error) -> Self {
        SettingsError::JsonError(err)
    }
}

pub struct SettingsManager {
    settings_path: PathBuf,
    settings: UserSettings,
}

impl SettingsManager {
    /// Create a new SettingsManager instance
    pub fn new() -> Result<Self, SettingsError> {
        let settings_path = Self::get_settings_path();
        
        // Ensure directory exists
        if let Some(parent) = settings_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Load or create settings file
        let settings = if settings_path.exists() {
            Self::load_from_file(&settings_path)?
        } else {
            let default_settings = UserSettings::default();
            Self::save_to_file(&settings_path, &default_settings)?;
            default_settings
        };
        
        Ok(Self {
            settings_path,
            settings,
        })
    }
    
    /// Get the settings file path
    fn get_settings_path() -> PathBuf {
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        
        PathBuf::from(home)
            .join(".kakeibon")
            .join("KakeiBon.json")
    }
    
    /// Load settings from file
    fn load_from_file(path: &PathBuf) -> Result<UserSettings, SettingsError> {
        let content = fs::read_to_string(path)?;
        
        // Handle empty file
        if content.trim().is_empty() {
            return Ok(UserSettings::default());
        }
        
        let settings: UserSettings = serde_json::from_str(&content)?;
        Ok(settings)
    }
    
    /// Save settings to file
    fn save_to_file(path: &PathBuf, settings: &UserSettings) -> Result<(), SettingsError> {
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(settings)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    /// Reload settings from file
    pub fn reload(&mut self) -> Result<(), SettingsError> {
        self.settings = Self::load_from_file(&self.settings_path)?;
        Ok(())
    }
    
    /// Save current settings to file
    pub fn save(&self) -> Result<(), SettingsError> {
        Self::save_to_file(&self.settings_path, &self.settings)
    }
    
    /// Get a setting value by key
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.settings.entries.get(key)
    }
    
    /// Get a setting value as a specific type
    pub fn get_as<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T, SettingsError> {
        let value = self.settings.entries.get(key)
            .ok_or_else(|| SettingsError::EntryNotFound(key.to_string()))?;
        
        serde_json::from_value(value.clone())
            .map_err(|e| SettingsError::JsonError(e))
    }
    
    /// Get a string value
    pub fn get_string(&self, key: &str) -> Result<String, SettingsError> {
        self.get_as::<String>(key)
    }
    
    /// Get an integer value
    pub fn get_int(&self, key: &str) -> Result<i64, SettingsError> {
        self.get_as::<i64>(key)
    }
    
    /// Get a boolean value
    pub fn get_bool(&self, key: &str) -> Result<bool, SettingsError> {
        self.get_as::<bool>(key)
    }
    
    /// Set a setting value
    pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<(), SettingsError> {
        let json_value = serde_json::to_value(value)?;
        self.settings.entries.insert(key.to_string(), json_value);
        Ok(())
    }
    
    /// Remove a setting
    pub fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        self.settings.entries.remove(key)
    }
    
    /// Check if a key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.settings.entries.contains_key(key)
    }
    
    /// Get all keys
    pub fn keys(&self) -> Vec<String> {
        self.settings.entries.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    fn setup_test_env() -> PathBuf {
        let temp_dir = env::temp_dir().join(format!("kakeibon_test_{}", std::process::id()));
        env::set_var("HOME", temp_dir.to_str().unwrap());
        temp_dir
    }
    
    fn cleanup_test_env(temp_dir: &PathBuf) {
        let _ = fs::remove_dir_all(temp_dir);
    }
    
    #[test]
    fn test_settings_manager_creation() {
        let temp_dir = setup_test_env();
        
        let manager = SettingsManager::new().unwrap();
        assert!(manager.settings_path.exists());
        
        cleanup_test_env(&temp_dir);
    }
    
    #[test]
    fn test_get_and_set_string() {
        let temp_dir = setup_test_env();
        
        let mut manager = SettingsManager::new().unwrap();
        
        // Set a string value
        manager.set("username", "test_user").unwrap();
        
        // Get the value back
        let username = manager.get_string("username").unwrap();
        assert_eq!(username, "test_user");
        
        cleanup_test_env(&temp_dir);
    }
    
    #[test]
    fn test_get_and_set_int() {
        let temp_dir = setup_test_env();
        
        let mut manager = SettingsManager::new().unwrap();
        
        manager.set("age", 25).unwrap();
        let age = manager.get_int("age").unwrap();
        assert_eq!(age, 25);
        
        cleanup_test_env(&temp_dir);
    }
    
    #[test]
    fn test_get_and_set_bool() {
        let temp_dir = setup_test_env();
        
        let mut manager = SettingsManager::new().unwrap();
        
        manager.set("enabled", true).unwrap();
        let enabled = manager.get_bool("enabled").unwrap();
        assert!(enabled);
        
        cleanup_test_env(&temp_dir);
    }
    
    #[test]
    fn test_save_and_reload() {
        let temp_dir = setup_test_env();
        
        {
            let mut manager = SettingsManager::new().unwrap();
            manager.set("theme", "dark").unwrap();
            manager.save().unwrap();
        } // manager is dropped here
        
        // Create a new manager instance (simulates app restart)
        let manager2 = SettingsManager::new().unwrap();
        let theme = manager2.get_string("theme").unwrap();
        assert_eq!(theme, "dark");
        
        cleanup_test_env(&temp_dir);
    }
    
    #[test]
    fn test_remove_entry() {
        let temp_dir = setup_test_env();
        
        let mut manager = SettingsManager::new().unwrap();
        manager.set("temp_key", "temp_value").unwrap();
        
        assert!(manager.contains_key("temp_key"));
        
        manager.remove("temp_key");
        assert!(!manager.contains_key("temp_key"));
        
        cleanup_test_env(&temp_dir);
    }
    
    #[test]
    fn test_entry_not_found() {
        let temp_dir = setup_test_env();
        
        let manager = SettingsManager::new().unwrap();
        let result = manager.get_string("nonexistent");
        
        assert!(result.is_err());
        match result {
            Err(SettingsError::EntryNotFound(key)) => assert_eq!(key, "nonexistent"),
            _ => panic!("Expected EntryNotFound error"),
        }
        
        cleanup_test_env(&temp_dir);
    }
    
    #[test]
    fn test_complex_type() {
        let temp_dir = setup_test_env();
        
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct WindowSettings {
            width: i32,
            height: i32,
            maximized: bool,
        }
        
        let mut manager = SettingsManager::new().unwrap();
        let window_settings = WindowSettings {
            width: 1920,
            height: 1080,
            maximized: false,
        };
        
        manager.set("window", &window_settings).unwrap();
        let loaded: WindowSettings = manager.get_as("window").unwrap();
        
        assert_eq!(loaded, window_settings);
        
        cleanup_test_env(&temp_dir);
    }
    
    #[test]
    fn test_keys_list() {
        let temp_dir = setup_test_env();
        
        let mut manager = SettingsManager::new().unwrap();
        manager.set("key1", "value1").unwrap();
        manager.set("key2", "value2").unwrap();
        manager.set("key3", "value3").unwrap();
        
        let keys = manager.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
        assert!(keys.contains(&"key3".to_string()));
        
        cleanup_test_env(&temp_dir);
    }
}
