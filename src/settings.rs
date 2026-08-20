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
    /// Create a new SettingsManager instance using the default settings path
    /// derived from the user's home directory.
    pub fn new() -> Result<Self, SettingsError> {
        Self::with_path(Self::get_settings_path())
    }

    /// Create a new SettingsManager bound to a specific settings file path.
    /// Used by `new()` and by tests that need an isolated settings file.
    pub(crate) fn with_path(settings_path: PathBuf) -> Result<Self, SettingsError> {
        if let Some(parent) = settings_path.parent() {
            fs::create_dir_all(parent)?;
        }

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

    /// Get the default settings file path
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
    
    /// Save settings to file.
    ///
    /// Uses the classic write-tmp-then-rename pattern so a crash (or a
    /// kill, or a disk-full) mid-write cannot leave a half-written JSON
    /// file at `path`. Before this change, `fs::write` truncated the
    /// target before streaming the new bytes; if the process died between
    /// the truncate and the final flush, subsequent starts would hit a
    /// `JsonError` in `load_from_file` and `SettingsManager::new()` would
    /// error, which `lib.rs` `setup` propagates via `?` — the app then
    /// fails to launch until the user deletes the file by hand.
    ///
    /// The tmp file is created in the *same directory* as `path` because
    /// `fs::rename` is only guaranteed to be atomic when source and
    /// destination share a filesystem (a rename across mounts fails with
    /// `EXDEV` on Linux). On any error the tmp file is best-effort cleaned
    /// up so we don't accumulate `.tmp` cruft next to the real settings.
    fn save_to_file(path: &PathBuf, settings: &UserSettings) -> Result<(), SettingsError> {
        // Ensure directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(settings)?;

        // Build the sibling tmp path: same directory, filename + ".tmp".
        // `file_name()` returns None only for paths that end in `..`,
        // which settings paths never do, so an empty fallback is fine —
        // the ensuing rename would then fail loudly rather than silently
        // corrupting anything.
        let mut tmp_path = path.clone();
        let mut tmp_name = path
            .file_name()
            .unwrap_or_default()
            .to_os_string();
        tmp_name.push(".tmp");
        tmp_path.set_file_name(tmp_name);

        if let Err(e) = fs::write(&tmp_path, &content) {
            let _ = fs::remove_file(&tmp_path);
            return Err(SettingsError::IoError(e));
        }

        if let Err(e) = fs::rename(&tmp_path, path) {
            let _ = fs::remove_file(&tmp_path);
            return Err(SettingsError::IoError(e));
        }

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
    use tempfile::TempDir;

    fn make_test_path() -> (PathBuf, TempDir) {
        let temp_dir = TempDir::new().expect("create temp_dir");
        let path = temp_dir.path().join(".kakeibon").join("KakeiBon.json");
        (path, temp_dir)
    }

    #[test]
    fn test_settings_manager_creation() {
        let (path, _temp) = make_test_path();
        let manager = SettingsManager::with_path(path).unwrap();
        assert!(manager.settings_path.exists());
    }

    #[test]
    fn test_get_and_set_string() {
        let (path, _temp) = make_test_path();
        let mut manager = SettingsManager::with_path(path).unwrap();

        manager.set("username", "test_user").unwrap();
        let username = manager.get_string("username").unwrap();
        assert_eq!(username, "test_user");
    }

    #[test]
    fn test_get_and_set_int() {
        let (path, _temp) = make_test_path();
        let mut manager = SettingsManager::with_path(path).unwrap();

        manager.set("age", 25).unwrap();
        let age = manager.get_int("age").unwrap();
        assert_eq!(age, 25);
    }

    #[test]
    fn test_get_and_set_bool() {
        let (path, _temp) = make_test_path();
        let mut manager = SettingsManager::with_path(path).unwrap();

        manager.set("enabled", true).unwrap();
        let enabled = manager.get_bool("enabled").unwrap();
        assert!(enabled);
    }

    #[test]
    fn test_save_and_reload() {
        let (path, _temp) = make_test_path();

        {
            let mut manager = SettingsManager::with_path(path.clone()).unwrap();
            manager.set("font_size", "medium").unwrap();
            manager.save().unwrap();
        } // manager is dropped here

        // Re-open the same settings file (simulates app restart)
        let manager2 = SettingsManager::with_path(path).unwrap();
        let font_size = manager2.get_string("font_size").unwrap();
        assert_eq!(font_size, "medium");
    }

    #[test]
    fn test_remove_entry() {
        let (path, _temp) = make_test_path();
        let mut manager = SettingsManager::with_path(path).unwrap();
        manager.set("temp_key", "temp_value").unwrap();

        assert!(manager.contains_key("temp_key"));

        manager.remove("temp_key");
        assert!(!manager.contains_key("temp_key"));
    }

    #[test]
    fn test_entry_not_found() {
        let (path, _temp) = make_test_path();
        let manager = SettingsManager::with_path(path).unwrap();
        let result = manager.get_string("nonexistent");

        assert!(result.is_err());
        match result {
            Err(SettingsError::EntryNotFound(key)) => assert_eq!(key, "nonexistent"),
            _ => panic!("Expected EntryNotFound error"),
        }
    }

    #[test]
    fn test_complex_type() {
        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct WindowSettings {
            width: i32,
            height: i32,
            maximized: bool,
        }

        let (path, _temp) = make_test_path();
        let mut manager = SettingsManager::with_path(path).unwrap();
        let window_settings = WindowSettings {
            width: 1920,
            height: 1080,
            maximized: false,
        };

        manager.set("window", &window_settings).unwrap();
        let loaded: WindowSettings = manager.get_as("window").unwrap();

        assert_eq!(loaded, window_settings);
    }

    #[test]
    fn test_keys_list() {
        let (path, _temp) = make_test_path();
        let mut manager = SettingsManager::with_path(path).unwrap();
        manager.set("key1", "value1").unwrap();
        manager.set("key2", "value2").unwrap();
        manager.set("key3", "value3").unwrap();

        let keys = manager.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
        assert!(keys.contains(&"key3".to_string()));
    }

    /// Fable-5 review #10 — `save_to_file` now uses the write-tmp-then-
    /// rename pattern. Verify that a successful save leaves neither the
    /// sibling `.tmp` file behind (would accumulate clutter on every save)
    /// nor an incomplete target (would make the next start fail to parse).
    #[test]
    fn test_save_leaves_no_tmp_sibling_and_target_is_parseable() {
        let (path, _temp) = make_test_path();

        let mut manager = SettingsManager::with_path(path.clone()).unwrap();
        manager.set("language", "ja").unwrap();
        manager.set("font_size", "medium").unwrap();
        manager.save().unwrap();

        // Target exists and is valid JSON round-trippable through the manager.
        assert!(path.exists(), "target settings file should exist after save");
        let reloaded = SettingsManager::with_path(path.clone()).unwrap();
        assert_eq!(reloaded.get_string("language").unwrap(), "ja");
        assert_eq!(reloaded.get_string("font_size").unwrap(), "medium");

        // Sibling `<file>.tmp` must have been renamed away — its presence
        // here would mean either the rename failed silently or the fix
        // regressed to a plain `fs::write`.
        let mut tmp_path = path.clone();
        let mut tmp_name = path.file_name().unwrap().to_os_string();
        tmp_name.push(".tmp");
        tmp_path.set_file_name(tmp_name);
        assert!(
            !tmp_path.exists(),
            "sibling tmp file should not remain after save: {:?}",
            tmp_path,
        );
    }

    /// Repeated saves must be idempotent w.r.t. filesystem entries — no
    /// stray `.tmp` file left after any of them, target parseable every
    /// time. Guards against a partial fix that only handled the first
    /// save (e.g. tmp cleanup only in an error branch).
    #[test]
    fn test_repeated_saves_do_not_accumulate_tmp_files() {
        let (path, _temp) = make_test_path();
        let mut manager = SettingsManager::with_path(path.clone()).unwrap();

        for i in 0..5 {
            manager.set("counter", i).unwrap();
            manager.save().unwrap();
        }

        let mut tmp_path = path.clone();
        let mut tmp_name = path.file_name().unwrap().to_os_string();
        tmp_name.push(".tmp");
        tmp_path.set_file_name(tmp_name);
        assert!(!tmp_path.exists(), "no tmp file should be left after 5 saves");

        let reloaded = SettingsManager::with_path(path).unwrap();
        assert_eq!(reloaded.get_int("counter").unwrap(), 4);
    }

    /// Regression guard: a `.tmp` file left behind from a previous crashed
    /// save must not be treated as the real settings by `with_path` / the
    /// load path. `with_path` looks at `path`, not `path.tmp`, so the
    /// leftover is inert until the next save happens to rename over it —
    /// which is fine as long as the previous target (or an empty state)
    /// is still valid to load from.
    #[test]
    fn test_stale_tmp_file_is_not_loaded() {
        let (path, _temp) = make_test_path();

        // Seed a normal settings file with the manager first.
        {
            let mut manager = SettingsManager::with_path(path.clone()).unwrap();
            manager.set("font_size", "small").unwrap();
            manager.save().unwrap();
        }

        // Now plant a corrupted `.tmp` next to it (as if a previous save
        // had crashed after write but before rename).
        let mut tmp_path = path.clone();
        let mut tmp_name = path.file_name().unwrap().to_os_string();
        tmp_name.push(".tmp");
        tmp_path.set_file_name(tmp_name);
        fs::write(&tmp_path, "{ this is not valid json").unwrap();

        // Re-opening the manager must still succeed and read the real file,
        // not the corrupt tmp.
        let manager = SettingsManager::with_path(path.clone()).unwrap();
        assert_eq!(manager.get_string("font_size").unwrap(), "small");

        // The next save should overwrite the tmp cleanly.
        let mut manager = manager;
        manager.set("font_size", "large").unwrap();
        manager.save().unwrap();
        assert!(!tmp_path.exists(), "next save must clean the tmp path via rename");
    }
}
