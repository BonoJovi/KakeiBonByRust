# Internationalization (I18N) System - Implementation Documentation

## Overview
A database-driven multilingual support system has been implemented. It supports internationalization for system resources (menus, messages, labels) and expense categories.

## Implementation Date
2024-10-24

## Database Tables

### 1. I18N_RESOURCES (System Resources)
Table for managing multilingual resources across the entire system.

```sql
CREATE TABLE I18N_RESOURCES (
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

**Purpose**: UI elements such as menus, messages, and labels
**Examples**: 
- `menu.file` → "File" (en), "ファイル" (ja)
- `msg.lang_changed` → "Language changed to {0}." (en), "言語を{0}に変更しました。" (ja)

### 2. CATEGORY1/2/3 (Category Master)
Tables for managing hierarchical expense categories.

```sql
CREATE TABLE CATEGORY1 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY1_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE)
);

CREATE TABLE CATEGORY2 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY2_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE) REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE)
);

CREATE TABLE CATEGORY3 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY3_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) 
        REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE)
);
```

**Design Features**:
- **CODE Identification**: Uses readable `CATEGORY_CODE` (e.g., "FOOD", "TRANSPORT")
- **User-specific Management**: Each user can have their own categories
- **Hierarchical Structure**: CATEGORY1 (major) → CATEGORY2 (middle) → CATEGORY3 (minor)
- **Display Order**: Controlled by `DISPLAY_ORDER`
- **Soft Delete**: Logical deletion using `IS_DISABLED` flag

### 3. CATEGORY1/2/3_I18N (Category Multilingual)
Tables for managing multilingual names of categories.

```sql
CREATE TABLE CATEGORY1_I18N (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    CATEGORY1_NAME_I18N VARCHAR(256) NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, LANG_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE) REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE)
);
```

**Features**:
- Independent I18N table for each category level
- Database size optimization through normalization
- Improved maintainability

## Service Layer

### I18nService (`src/services/i18n.rs`)

Service for managing multilingual resources.

#### Main Methods

##### `get(key: &str, lang_code: &str) -> Result<String, I18nError>`
Retrieves resource value by key and language code.
- Falls back to default language (ja) if requested language is not found

##### `get_with_params(key: &str, lang_code: &str, params: &[&str]) -> Result<String, I18nError>`
Retrieves resource with parameter substitution.
```rust
// "言語を{0}に変更しました。" → "言語を日本語に変更しました。"
i18n.get_with_params("msg.lang_changed", "ja", &["日本語"]).await
```

##### `get_all(lang_code: &str) -> Result<HashMap<String, String>, I18nError>`
Retrieves all resources for a specific language.

##### `get_by_category(lang_code: &str, category: &str) -> Result<HashMap<String, String>, I18nError>`
Retrieves resources by category (e.g., "menu", "message").

### CategoryService (`src/services/category.rs`)

Service for managing expense categories.

#### Main Methods

##### `initialize_user_categories(user_id: i64) -> Result<(), CategoryError>`
Initializes default categories when registering a new user.
- Skips if already initialized
- Ensures data consistency with transaction processing

##### `get_category1_list(user_id: i64, lang_code: &str) -> Result<Vec<Category1>, CategoryError>`
Retrieves list of major categories (CATEGORY1) in multiple languages.
- Joins with I18N table to get translated names
- Falls back to default name if translation is not available

## Tauri Commands

### Language Settings

#### `set_language(language: String) -> Result<String, String>`
Sets the language and returns a confirmation message.

**Arguments**:
- `language`: "English", "en", "日本語", "Japanese", "ja"

**Return Value**: Confirmation message in the selected language
- English: "Language changed to English."
- Japanese: "言語を日本語に変更しました。"

**Processing Flow**:
1. Normalize language code (en/ja)
2. Save to KakeiBon.json
3. Retrieve language name from database
4. Return confirmation message with parameter substitution

#### `get_language() -> Result<String, String>`
Retrieves current language setting.

**Default**: "ja" (Japanese)

### Resource Retrieval

#### `get_i18n_resource(key: String) -> Result<String, String>`
Retrieves resource value by resource key.
- Automatically uses current language setting

**Example**:
```javascript
const fileMenu = await invoke('get_i18n_resource', { key: 'menu.file' });
// With Japanese setting: "ファイル"
// With English setting: "File"
```

#### `get_i18n_resources_by_category(category: String) -> Result<HashMap<String, String>, String>`
Retrieves multiple resources at once by category.

**Example**:
```javascript
const menuResources = await invoke('get_i18n_resources_by_category', { category: 'menu' });
// { "menu.file": "ファイル", "menu.settings": "設定", ... }
```

## Default Resources

### Menus
| Resource Key | English | Japanese |
|-------------|---------|----------|
| menu.file | File | ファイル |
| menu.settings | Settings | 設定 |
| menu.language | Language | 言語 |
| menu.quit | Quit | 終了 |

### Language Options
| Resource Key | English | Japanese |
|-------------|---------|----------|
| lang.english | English | English |
| lang.japanese | 日本語 (Japanese) | 日本語 |
| lang.name.en | English | 英語 |
| lang.name.ja | Japanese | 日本語 |

### Messages
| Resource Key | English | Japanese |
|-------------|---------|----------|
| msg.lang_changed | Language changed to {0}. | 言語を{0}に変更しました。 |
| msg.error | Error | エラー |
| msg.success | Success | 成功 |
| msg.info | Information | 情報 |

## Testing

### I18nService Tests
- ✅ `test_get_resource`: Resource retrieval
- ✅ `test_get_with_params`: Parameter substitution
- ✅ `test_fallback_to_default`: Fallback to default language
- ✅ `test_get_by_category`: Category-based retrieval

### CategoryService Tests
- ✅ `test_initialize_user_categories`: User category initialization
- ✅ `test_get_category1_list`: Multilingual category list retrieval

### Overall Test Results
```
Total Tests: 90
Passed: 90
Failed: 0
Success Rate: 100%
```

## Usage Examples

### Frontend (JavaScript)

#### Language Switching
```javascript
// Change language to Japanese
const message = await invoke('set_language', { language: '日本語' });
alert(message); // "言語を日本語に変更しました。"

// Change language to English
const message = await invoke('set_language', { language: 'English' });
alert(message); // "Language changed to English."
```

#### Resource Retrieval
```javascript
// Single resource retrieval
const fileMenuLabel = await invoke('get_i18n_resource', { key: 'menu.file' });

// Bulk retrieval by category
const menuResources = await invoke('get_i18n_resources_by_category', { category: 'menu' });
console.log(menuResources['menu.file']);
console.log(menuResources['menu.settings']);
```

### Backend (Rust)

```rust
use crate::services::i18n::I18nService;

let i18n = I18nService::new(pool);

// Resource retrieval
let menu_file = i18n.get("menu.file", "ja").await?;

// Parameter substitution
let message = i18n.get_with_params(
    "msg.lang_changed", 
    "ja", 
    &["日本語"]
).await?;
// "言語を日本語に変更しました。"
```

## Next Steps (Not Yet Implemented)

### 1. Menu Implementation
- Add File > Settings > Language menu
- Language selection submenu (English / 日本語)

### 2. Language Change Dialog
- Implement language change confirmation dialog
- Display message

### 3. Category Data Migration
- Migrate category data from existing SQL
- Populate CATEGORY1/2/3 tables
- Populate CATEGORY1/2/3_I18N tables with translation data

### 4. Dynamic Menu Updates
- Dynamically update menus when language changes
- Redraw entire application

## File Structure

```
src/
  ├── services/
  │   ├── i18n.rs          # Multilingual resource management
  │   └── category.rs      # Category management
  ├── lib.rs               # Tauri command definitions
  └── ...

res/
  └── sql/
      └── dbaccess.sql     # Database schema + initial data

$HOME/.kakeibon/
  ├── KakeiBonDB.sqlite3   # Database
  └── KakeiBon.json        # User settings (including language)
```

## Summary

Backend implementation of the multilingual support system is complete:

✅ Database table design completed
✅ Service layer implementation completed
✅ Tauri command implementation completed
✅ Test implementation completed (90/90 passed)
✅ Documentation completed

Frontend implementation (menus and dialogs) is planned for the next phase.
