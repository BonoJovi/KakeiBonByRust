# User Settings Management System

## Overview
A system to save and manage user customization settings in the `$HOME/.kakeibon/KakeiBon.json` file.

## Settings File

### File Path
```
$HOME/.kakeibon/KakeiBon.json
```
- On Linux: `~/.kakeibon/KakeiBon.json`
- On Windows: `%USERPROFILE%\.kakeibon\KakeiBon.json`

### File Format
Stores data in JSON format as key-value pairs.

```json
{
  "language": "ja"
}
```

**Note**: Currently only the `language` setting is implemented. Other settings like theme and font size will be implemented in the future.

## Implementation Files

### `src/settings.rs`
Implementation of the settings management service

#### Main Structures
- `UserSettings`: Holds settings data
- `SettingsError`: Error type
- `SettingsManager`: Settings management service

#### Main Methods

##### `new() -> Result<Self, SettingsError>`
- Creates a new SettingsManager instance
- Creates an empty file if settings file doesn't exist
- Automatically loads the settings file

##### `get(key: &str) -> Option<&serde_json::Value>`
- Retrieves the setting value for a specified key (generic type)

##### `get_string(key: &str) -> Result<String, SettingsError>`
- Retrieves a string type setting value

##### `get_int(key: &str) -> Result<i64, SettingsError>`
- Retrieves an integer type setting value

##### `get_bool(key: &str) -> Result<bool, SettingsError>`
- Retrieves a boolean type setting value

##### `set<T>(key: &str, value: T) -> Result<(), SettingsError>`
- Sets a setting value

##### `remove(key: &str) -> Option<serde_json::Value>`
- Removes a setting

##### `save() -> Result<(), SettingsError>`
- Saves in-memory settings to file

##### `reload() -> Result<(), SettingsError>`
- Reloads settings from file

## Tauri Commands

### `get_setting_string(key: String) -> Result<String, String>`
- String type setting value retrieval
- Frontend call: `invoke('get_setting_string', { key: 'language' })`

### `set_setting(key: String, value: serde_json::Value) -> Result<(), String>`
- Sets and saves a setting value
- Frontend call: `invoke('set_setting', { key: 'language', value: 'en' })`

### `list_setting_keys() -> Result<Vec<String>, String>`
- Retrieves a list of all keys

### `reload_settings() -> Result<(), String>`
- Reloads settings from file

## Usage Examples

### Frontend (JavaScript)
```javascript
// Get current language
const lang = await invoke('get_setting_string', { key: 'language' });

// Change language
await invoke('set_setting', { key: 'language', value: 'en' });

// Get all keys
const keys = await invoke('list_setting_keys');

// Reload settings
await invoke('reload_settings');
```

## Testing

### Implemented Tests (9 tests, all passing)
1. SettingsManager creation
2. String retrieval and setting
3. Integer retrieval and setting
4. Boolean retrieval and setting
5. Save and reload
6. Entry removal
7. Error for non-existent key
8. Complex type support
9. Key list retrieval

### Running Tests
```bash
cargo test settings::tests --lib
```

## Auto-save Feature

The `set_setting` Tauri command automatically calls `save()` after changing settings.

## Summary

- ✅ Settings saved in JSON format
- ✅ Type-safe setting retrieval
- ✅ Auto-save feature
- ✅ Automatic file creation
- ✅ All 9 tests passing

**Current Implementation Status**:
- ✅ Language setting
- ⏳ Theme settings (planned)
- ⏳ Font size settings (planned)
- ⏳ Auto-save interval settings (planned)
