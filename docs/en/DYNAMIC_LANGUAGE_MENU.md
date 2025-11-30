# Dynamic Language Menu Implementation

## Overview

Changed the language selection menu from hardcoded options to dynamically loading languages from the database.
Languages are sorted by language code to maintain consistent order when switching languages.

## Changes Made

### 1. Backend Changes (Rust)

#### src/lib.rs

Added two new Tauri commands:

1. **`get_available_languages`** - Gets list of available language codes from database
   
2. **`get_language_names`** - Gets array of language codes to their localized names in the current language (sorted by language code)

These commands leverage the existing `I18nService::get_available_languages()` method which queries:

```sql
SELECT DISTINCT LANG_CODE FROM I18N_RESOURCES ORDER BY LANG_CODE
```

### 2. Frontend Changes (HTML/JavaScript)

#### res/index.html

Added language selection dropdown to menu bar:

```html
<div id="language-menu" class="menu-item">
    <span data-i18n="menu.language">Language</span>
    <div id="language-dropdown" class="dropdown">
        <!-- Language options will be populated dynamically -->
    </div>
</div>
```

#### res/js/menu.js

Added new functions:

1. **`setupLanguageMenu()`** - Fetches language list from database and dynamically generates menu items
   - Marks current language (with filled green circle)
   - Sets up event listeners for language switching

2. **`handleLanguageChange(langCode)`** - Handles language change when user selects a language
   - Updates UI with new translations
   - Refreshes language menu to show updated names

Modified initialization to call `setupLanguageMenu()` during startup

## Benefits

1. **Better maintainability** - Just add data to database to support new languages

2. **Fewer bugs** - No need to maintain hardcoded language lists in multiple places

3. **Consistency** - All language data managed in one place (database)

4. **Extensibility** - No code changes needed to add new languages

5. **Consistent ordering** - Languages sorted by code, order doesn't change when switching languages

## Database Structure

Language data is stored in the `I18N_RESOURCES` table:

```sql
CREATE TABLE IF NOT EXISTS I18N_RESOURCES (
    RESOURCE_ID INTEGER NOT NULL,
    RESOURCE_KEY VARCHAR(256) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    RESOURCE_VALUE TEXT NOT NULL,
    CATEGORY VARCHAR(64),
    DESCRIPTION VARCHAR(512),
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(RESOURCE_ID),
    UNIQUE(RESOURCE_KEY, LANG_CODE)
);
```

### Resource Keys for Language Names

- `lang.name.en` - English language name in current language
- `lang.name.ja` - Japanese language name in current language

Examples:
- When displaying in English: `lang.name.ja` → "Japanese"
- When displaying in Japanese: `lang.name.ja` → "日本語"

## How to Add New Languages

1. Add new language resources to `I18N_RESOURCES` table

2. Add translations for all resource keys for the new language (menu.*, error.*, etc.)

3. Add `lang.name.{lang_code}` resources in each language

4. Restart application and new language will automatically appear in menu

### Example: Adding Chinese

```sql
-- Language name resources
INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT)
VALUES 
(1001, 'lang.name.zh', 'en', 'Chinese', 'language', datetime('now')),
(1002, 'lang.name.zh', 'ja', '中国語', 'language', datetime('now')),
(1003, 'lang.name.zh', 'zh', '中文', 'language', datetime('now'));

-- Then add all other translations for zh language code
-- menu.file, menu.login, error messages, etc.
```

## Testing

All existing unit tests pass:

```bash
cargo test --lib
# Result: ok. 90 passed; 0 failed; 0 ignored
```

## Compatibility

Maintains backward compatibility with existing functionality. Currently supported languages:

- English (en)
- Japanese (ja)

## Implementation Files

### Modified Files

1. **src/lib.rs**
   - Added new Tauri commands
   - `get_available_languages()` - List of available languages
   - `get_language_names()` - Localized language names

2. **res/index.html**
   - Added language dropdown menu

3. **res/js/menu.js**
   - Implemented dynamic language menu
   - `setupLanguageMenu()` - Generate menu items
   - `setupLanguageMenuHandlers()` - Setup event handlers
   - `handleLanguageChange()` - Handle language change

4. **res/css/menu.css**
   - Language menu styles

## Technical Details

### Order Guarantee

Return as `Vec<(String, String)>` from backend to guarantee order:

```rust
let mut language_names = Vec::new();
for lang_code in lang_codes {
    let key = format!("lang.name.{}", lang_code);
    if let Ok(name) = i18n.get(&key, &current_lang).await {
        language_names.push((lang_code, name));
    }
}

// Sort by language code
language_names.sort_by(|a, b| a.0.cmp(&b.0));
```

Frontend receives as array and maintains order:

```javascript
for (const [langCode, langName] of languageNames) {
    // Array order is preserved
}
```

## Future Enhancements

This implementation enables:

- Support new languages by simply adding data to database
- Add/remove languages without code changes
- Centralized translation management in database
- Easy integration with translation management tools

## Change Log

### 2024-10-24
- Initial version
- Implemented dynamic language menu
- Added language sorting feature
