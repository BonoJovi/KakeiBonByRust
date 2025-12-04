# Settings API Reference

**Last Updated**: 2025-12-05 02:38 JST

## Overview

This document defines APIs used in the settings screen. Provides functionality to read and write various application settings (language, font size, theme, etc.).

---

## Table of Contents

1. [Basic Settings API](#basic-settings-api)
2. [Language Settings API](#language-settings-api)
3. [Font Size Settings API](#font-size-settings-api)
4. [Window Adjustment API](#window-adjustment-api)
5. [Data Structures](#data-structures)

---

## Basic Settings API

### get_setting

Retrieves a setting value as a string.

**Parameters:**
- `key` (String): Setting key

**Return Value:**
- `String`: Setting value

**Usage Example:**
```javascript
const language = await invoke('get_setting', { key: 'language' });
console.log(`Language: ${language}`);
```

**Error:**
- Key not found → Exception thrown
- For optional retrieval, use `get_setting_or_default`

---

### get_setting_or_default

Retrieves a setting value, or default if not found.

**Parameters:**
- `key` (String): Setting key
- `default_value` (String): Default value

**Return Value:**
- `String`: Setting value or default

**Usage Example:**
```javascript
const theme = await invoke('get_setting_or_default', {
    key: 'theme',
    defaultValue: 'light'
});
```

---

### get_setting_int

Retrieves a setting value as an integer.

**Parameters:**
- `key` (String): Setting key

**Return Value:**
- `i64`: Setting value

**Usage Example:**
```javascript
const maxItems = await invoke('get_setting_int', { key: 'max_items' });
```

**Error:**
- Type mismatch → Exception thrown

---

### get_setting_bool

Retrieves a setting value as a boolean.

**Parameters:**
- `key` (String): Setting key

**Return Value:**
- `bool`: Setting value

**Usage Example:**
```javascript
const autoSave = await invoke('get_setting_bool', { key: 'auto_save' });
```

---

### set_setting

Sets a setting value.

**Parameters:**
- `key` (String): Setting key
- `value` (varies): Setting value (String, i64, bool, f64)

**Return Value:** None

**Usage Example:**
```javascript
// String
await invoke('set_setting', {
    key: 'theme',
    value: 'dark'
});

// Integer
await invoke('set_setting', {
    key: 'max_items',
    value: 100
});

// Boolean
await invoke('set_setting', {
    key: 'auto_save',
    value: true
});
```

**Note:**
- Automatically saved to settings file

---

### remove_setting

Removes a setting.

**Parameters:**
- `key` (String): Setting key to remove

**Return Value:** None

**Usage Example:**
```javascript
await invoke('remove_setting', { key: 'theme' });
```

**Note:**
- Does not error if key doesn't exist

---

### list_setting_keys

Retrieves all setting keys.

**Parameters:** None

**Return Value:**
- `Vec<String>`: Array of setting keys

**Usage Example:**
```javascript
const keys = await invoke('list_setting_keys');
console.log('Setting keys:', keys);
```

---

### reload_settings

Reloads settings from file.

**Parameters:** None

**Return Value:** None

**Usage Example:**
```javascript
await invoke('reload_settings');
console.log('Settings reloaded');
```

**Purpose:**
- When settings file edited externally
- Settings initialization

---

## Language Settings API

### set_language

Sets the language.

**Parameters:**
- `language` (String): Language code or language name

**Return Value:**
- `String`: Success message (multilingual)

**Usage Example:**
```javascript
// Specify by language code
await invoke('set_language', { language: 'ja' });

// Specify by language name
await invoke('set_language', { language: '日本語' });
await invoke('set_language', { language: 'English' });
```

**Supported Languages:**
- `"ja"`, `"Japanese"`, `"日本語"` → Japanese
- `"en"`, `"English"` → English

**Automatic Processing:**
1. Language code normalization
2. Validation
3. Save to settings file

**Error:**
- `"Unsupported language: ..."`: Unsupported language

---

### get_language

Retrieves current language setting.

**Parameters:** None

**Return Value:**
- `String`: Language code ("ja" or "en")

**Usage Example:**
```javascript
const lang = await invoke('get_language');
console.log(`Current language: ${lang}`);
```

**Default:**
- `"ja"` (LANG_DEFAULT) if not configured

---

### get_available_languages

Retrieves list of available languages.

**Parameters:** None

**Return Value:**
- `Vec<String>`: Array of language codes

**Usage Example:**
```javascript
const languages = await invoke('get_available_languages');
console.log('Available languages:', languages);
// ["ja", "en"]
```

---

### get_language_names

Retrieves pairs of language codes and display names.

**Parameters:** None

**Return Value:**
- `Vec<(String, String)>`: Array of (language code, display name) tuples

**Usage Example:**
```javascript
const languageNames = await invoke('get_language_names');

// Create language selection dropdown
languageNames.forEach(([code, name]) => {
    const option = document.createElement('option');
    option.value = code;
    option.textContent = name;
    selectElement.appendChild(option);
});

// Example: [["ja", "日本語"], ["en", "English"]]
```

**Note:**
- Display names retrieved in current language setting
- Retrieved from I18N table with `lang.name.{code}` key

---

## Font Size Settings API

### set_font_size

Sets font size.

**Parameters:**
- `font_size` (String): Font size ("small", "medium", "large")

**Return Value:**
- `String`: Success message (multilingual)

**Usage Example:**
```javascript
await invoke('set_font_size', { fontSize: 'large' });
```

**Supported Sizes:**
- `"small"`: Small
- `"medium"`: Medium (default)
- `"large"`: Large

**Automatic Processing:**
1. Size validation
2. Save to settings file

**Error:**
- `"Unsupported font size: ..."`: Unsupported size

---

### get_font_size

Retrieves current font size setting.

**Parameters:** None

**Return Value:**
- `String`: Font size ("small", "medium", "large")

**Usage Example:**
```javascript
const fontSize = await invoke('get_font_size');
console.log(`Font size: ${fontSize}`);
```

**Default:**
- `"medium"` (FONT_SIZE_DEFAULT) if not configured

---

## Window Adjustment API

### adjust_window_size

Adjusts window size.

**Parameters:**
- `width` (f64): Width (pixels)
- `height` (f64): Height (pixels)

**Return Value:** None

**Usage Example:**
```javascript
// Resize window to 1280x720
await invoke('adjust_window_size', {
    width: 1280,
    height: 720
});
```

**Behavior:**
- Adjusts relative to current window size
- Min/max size constraints apply (OS-dependent)

**Errors:**
- `"Failed to get window size: ..."`: Window size retrieval error
- `"Failed to set window size: ..."`: Window size setting error

---

## Data Structures

### Settings File Format

Settings are saved in JSON format to a file.

**Default Storage Location:**
- Windows: `%APPDATA%\KakeiBon\settings.json`
- macOS: `~/Library/Application Support/KakeiBon/settings.json`
- Linux: `~/.config/KakeiBon/settings.json`

**Settings File Example:**
```json
{
    "language": "ja",
    "font_size": "medium",
    "theme": "light",
    "auto_save": true,
    "max_items": 100
}
```

---

### Standard Setting Keys

| Key | Type | Default Value | Description |
|-----|------|---------------|-------------|
| `language` | String | "ja" | Language setting |
| `font_size` | String | "medium" | Font size |
| `theme` | String | - | Theme (not implemented) |
| `auto_save` | bool | - | Auto save (not implemented) |

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"Failed to get setting: ..."` | Key doesn't exist or type mismatch | Check key or use different get function |
| `"Failed to set setting: ..."` | Setting error | Check setting value |
| `"Failed to save settings: ..."` | File save error | Check disk space/permissions |
| `"Failed to reload settings: ..."` | File read error | Check settings file format |
| `"Unsupported language: ..."` | Unsupported language | Specify "ja" or "en" |
| `"Unsupported font size: ..."` | Unsupported size | Specify "small", "medium", "large" |

### Frontend Error Handling Example

```javascript
// Language setting
async function changeLanguage(language) {
    try {
        const message = await invoke('set_language', { language });
        alert(message);
        
        // Reload page to apply new language
        location.reload();
    } catch (error) {
        if (error.includes('Unsupported language')) {
            alert('Unsupported language');
        } else {
            alert(`Error: ${error}`);
        }
    }
}

// Font size setting
async function changeFontSize(fontSize) {
    try {
        await invoke('set_font_size', { fontSize });
        
        // Update CSS
        document.documentElement.style.fontSize = 
            fontSize === 'small' ? '14px' : 
            fontSize === 'large' ? '18px' : '16px';
        
        alert('Font size changed');
    } catch (error) {
        alert(`Error: ${error}`);
    }
}
```

---

## Usage Example: Settings Screen Implementation

### Loading Settings

```javascript
async function loadSettings() {
    try {
        // Language setting
        const language = await invoke('get_language');
        document.getElementById('language-select').value = language;
        
        // Font size setting
        const fontSize = await invoke('get_font_size');
        document.getElementById('font-size-select').value = fontSize;
        
        // Display all setting keys (for debugging)
        const keys = await invoke('list_setting_keys');
        console.log('Setting keys:', keys);
    } catch (error) {
        console.error('Settings load error:', error);
    }
}

// Execute on page load
document.addEventListener('DOMContentLoaded', loadSettings);
```

### Creating Language Selection Dropdown

```javascript
async function initializeLanguageSelect() {
    try {
        const languageNames = await invoke('get_language_names');
        const currentLang = await invoke('get_language');
        
        const select = document.getElementById('language-select');
        select.innerHTML = '';
        
        languageNames.forEach(([code, name]) => {
            const option = document.createElement('option');
            option.value = code;
            option.textContent = name;
            option.selected = (code === currentLang);
            select.appendChild(option);
        });
        
        // Change event
        select.addEventListener('change', async (e) => {
            await changeLanguage(e.target.value);
        });
    } catch (error) {
        console.error('Language select initialization error:', error);
    }
}
```

### Font Size Change

```javascript
async function handleFontSizeChange(event) {
    const fontSize = event.target.value;
    
    try {
        const message = await invoke('set_font_size', { fontSize });
        
        // Update CSS class
        document.body.className = `font-${fontSize}`;
        
        alert(message);
    } catch (error) {
        alert(`Error: ${error}`);
    }
}
```

### Saving Custom Settings

```javascript
async function saveCustomSettings() {
    try {
        // Theme setting
        const theme = document.getElementById('theme-select').value;
        await invoke('set_setting', {
            key: 'theme',
            value: theme
        });
        
        // Auto save setting
        const autoSave = document.getElementById('auto-save-checkbox').checked;
        await invoke('set_setting', {
            key: 'auto_save',
            value: autoSave
        });
        
        alert('Settings saved');
    } catch (error) {
        alert(`Save error: ${error}`);
    }
}
```

---

## Language Switching Mechanism

### Flow

```
1. User selects language
   ↓
2. Call set_language()
   ↓
3. Normalize language code
   ↓
4. Validation
   ↓
5. Save to settings.json
   ↓
6. Reload page
   ↓
7. I18N service loads new language
```

### Implementation Example

```javascript
async function switchLanguage(newLang) {
    try {
        // 1. Set language
        await invoke('set_language', { language: newLang });
        
        // 2. Reload page (apply new language)
        location.reload();
    } catch (error) {
        alert(`Language switch error: ${error}`);
    }
}

// Or update dynamically without reload
async function switchLanguageDynamic(newLang) {
    try {
        // 1. Set language
        await invoke('set_language', { language: newLang });
        
        // 2. Re-fetch all I18N keys
        const translations = await invoke('get_translations', { language: newLang });
        
        // 3. Update UI texts
        updateUITexts(translations);
        
    } catch (error) {
        alert(`Language switch error: ${error}`);
    }
}
```

---

## Applying Font Size

### CSS Application Example

```css
/* Default (medium) */
body {
    font-size: 16px;
}

/* Small */
body.font-small {
    font-size: 14px;
}

/* Large */
body.font-large {
    font-size: 18px;
}
```

### JavaScript Application Example

```javascript
async function applyFontSize() {
    const fontSize = await invoke('get_font_size');
    
    // Apply CSS class
    document.body.className = `font-${fontSize}`;
    
    // Or apply style directly
    const sizes = {
        small: '14px',
        medium: '16px',
        large: '18px'
    };
    document.documentElement.style.fontSize = sizes[fontSize];
}

// Apply on page load
document.addEventListener('DOMContentLoaded', applyFontSize);
```

---

## Security Considerations

### Settings File Protection

1. **Access permissions**: Accessible only by user
2. **Validation**: Always validate setting values
3. **Default values**: Use defaults for invalid values

### Input Validation

```javascript
// Prevent invalid values
async function setSafeSetting(key, value) {
    // Validation
    if (typeof value === 'string' && value.length > 1000) {
        throw new Error('Setting value too long');
    }
    
    await invoke('set_setting', { key, value });
}
```

---

## Test Coverage

**SettingsService:**
- ✅ Setting retrieval test (String, Int, Bool)
- ✅ Setting save test
- ✅ Setting delete test
- ✅ Setting list retrieval test
- ✅ Settings reload test
- ✅ Language switch test
- ✅ Font size change test
- ✅ Default value fallback test

---

## Related Documents

### Implementation Files

- Settings Service: `src/services/settings.rs`
- I18N Service: `src/services/i18n.rs`
- Tauri Commands: `src/lib.rs`

### Other API References

- [Common API](./API_COMMON.md) - I18n related APIs
- [User Management API](./API_USER.md) - get_user_settings, update_user_settings

---

**Change History:**
- 2025-12-05: Created (based on implementation code)
