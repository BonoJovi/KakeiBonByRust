# Translation Resource Statistics

## Overview

This document provides statistical information about the internationalization (i18n) translation resources used in the KakeiBon application.

## Total Translation Resources

**Total: 298 resources**

- Unique keys: approximately 149 keys
- Supported languages: 2 languages (Japanese, English)

## Breakdown by Category

| Category | Resource Count | SQL File | Description |
|----------|---------------|----------|-------------|
| Base Resources | 126 | `insert_translation_resources.sql` | User management, settings, category management, etc. |
| Account & Transaction | 76 | `insert_account_transaction_i18n.sql` | Account management and transaction related |
| Transaction Management | 30 | `add_transaction_mgmt_i18n.sql` | Transaction management screen |
| Shop Management | 30 | `insert_shop_i18n.sql` | Shop management screen |
| Transaction Modal | 36 | `insert_transaction_modal_i18n.sql` | Transaction edit modal |

## Main Categories

Translation resources are classified into the following categories:

### 1. User Management (user_mgmt)
- Add, edit, delete users
- Password change
- Role management

### 2. Category Management (category_mgmt)
- Major and minor category management
- Add, edit, delete categories

### 3. Account Management (account_mgmt)
- Add, edit, delete accounts
- Balance management
- Account templates

### 4. Transaction Management (transaction_mgmt)
- Transaction list
- Add, edit, delete transactions
- Filter functionality

### 5. Shop Management (shop_mgmt)
- Add, edit, delete shops
- Shop list
- Display order management

### 6. Settings (settings)
- Language settings
- Font size settings
- Other settings

### 7. Common (common)
- Button labels (Save, Cancel, Delete, etc.)
- Error messages
- Confirmation dialogs

## Database Structure

### I18N_RESOURCES Table

```sql
CREATE TABLE I18N_RESOURCES (
    RESOURCE_ID INTEGER PRIMARY KEY,
    RESOURCE_KEY TEXT NOT NULL,
    LANG_CODE TEXT NOT NULL,
    RESOURCE_VALUE TEXT NOT NULL,
    CATEGORY TEXT,
    DESCRIPTION TEXT,
    ENTRY_DT TEXT NOT NULL,
    UPDATE_DT TEXT,
    UNIQUE(RESOURCE_KEY, LANG_CODE)
);
```

### Resource Key Naming Convention

Resource keys follow this format:

```
{category}.{subcategory}.{element}
```

**Examples:**
- `user_mgmt.title` - User management screen title
- `transaction_mgmt.add_new` - Add transaction button
- `common.btn.save` - Save button

## Supported Languages

### Currently Supported
- **Japanese** (`ja`) - Primary language
- **English** (`en`) - Secondary language

### Language Codes
- `LANG_DEFAULT = "ja"` (Default language)

## Using Translation Resources

### Frontend (JavaScript)

```javascript
// Initialize i18n instance
await i18n.init();

// Get translation
const title = i18n.t('user_mgmt.title');

// Translation with parameters
const message = i18n.t('msg.user_added', { username: 'John' });

// Auto-apply to HTML elements
<button data-i18n="common.btn.save">Save</button>
```

### Backend (Rust)

```rust
// Use i18n service
let i18n = I18nService::new(pool);
let value = i18n.get("user_mgmt.title", "ja").await?;

// With parameters
let message = i18n.get_with_params(
    "msg.user_added",
    "ja",
    &["John"]
).await?;

// Get all resources
let translations = i18n.get_all("ja").await?;
```

## Adding New Resources

Steps to add translation resources when adding new screens or features:

### 1. Create SQL File

```sql
-- insert_{feature}_i18n.sql
INSERT INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT)
VALUES 
('feature.title', 'ja', '機能タイトル', 'feature', datetime('now')),
('feature.title', 'en', 'Feature Title', 'feature', datetime('now'));
```

### 2. Apply to Database

```bash
sqlite3 ~/.local/share/kakeibo/kakeibo.db < sql/insert_{feature}_i18n.sql
```

### 3. Use in Frontend

```html
<h1 data-i18n="feature.title">Feature Title</h1>
```

## Test Coverage

Translation system tests include:

- Resource key existence verification
- Fallback behavior (switching to default language)
- Parameter substitution
- Language switching functionality

## Related Documentation

- [I18N Implementation Details](./I18N_IMPLEMENTATION.md)
- [Dynamic Language Menu](./DYNAMIC_LANGUAGE_MENU.md)
- [User Management](./USER_MANAGEMENT.md)

## Updating Statistics

Statistics in this document can be updated with the following command:

```bash
# Check resource count in each SQL file
for file in sql/insert_*.sql sql/add_*_i18n.sql; do
  echo "$file: $(grep -c 'RESOURCE_KEY\|VALUES' $file)"
done
```

---

**Last Updated**: 2025-11-10 JST  
**Total Resources**: 298  
**Supported Languages**: Japanese, English
