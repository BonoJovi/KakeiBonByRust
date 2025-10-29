# Category Management API Documentation

## Overview

This document describes the backend API for category management in KakeiBon.
API exposure to the frontend is subject to the description of each item below.

---

## API List

### Category Tree Retrieval

#### `get_category_tree`
Get complete category tree for a user.

**Parameters:**
- `user_id` (i64): User ID

**Returns:**
- `Vec<CategoryTree>`: Array of category trees

**Example:**
```javascript
const tree = await invoke('get_category_tree', { user_id: 1 });
```

---

#### `get_category_tree_with_lang`
Get complete category tree with language-specific names.

**Parameters:**
- `user_id` (i64): User ID
- `lang_code` (Option<String>): Language code (e.g., "ja", "en")

**Returns:**
- `Vec<CategoryTree>`: Array of category trees with i18n names

**Response Structure:**
```javascript
[
  {
    category1: {
      user_id: 1,
      category1_code: "EXPENSE",
      category1_name: "支出",
      category1_name_i18n: "Expense",  // from I18N table
      display_order: 1,
      is_disabled: false,
      entry_dt: "2025-10-28...",
      update_dt: null
    },
    children: [
      {
        category2: {
          user_id: 1,
          category1_code: "EXPENSE",
          category2_code: "C2_E_1",
          category2_name: "食費",
          category2_name_i18n: "Food",
          display_order: 1,
          is_disabled: false,
          entry_dt: "2025-10-28...",
          update_dt: null
        },
        children: [
          {
            user_id: 1,
            category1_code: "EXPENSE",
            category2_code: "C2_E_1",
            category3_code: "C3_1",
            category3_name: "食料品",
            category3_name_i18n: "Groceries",
            display_order: 1,
            is_disabled: false,
            entry_dt: "2025-10-28...",
            update_dt: null
          }
        ]
      }
    ]
  }
]
```

**Example:**
```javascript
const tree = await invoke('get_category_tree_with_lang', { 
  user_id: 1, 
  lang_code: "en" 
});
```

---

## Category1 APIs (Major Categories)

### `add_category1`
Add a new major category.

**Parameters:**
- `user_id` (i64): User ID
- `code` (String): Category code (e.g., "EXPENSE")
- `name` (String): Category name (Japanese)

**Returns:**
- `Result<(), String>`: Success or error message

**Notes:**
- Display order is automatically set to max + 1
- In current design, major categories are fixed and this API is not used from UI

**Example:**
```javascript
await invoke('add_category1', { 
  user_id: 1, 
  code: "CUSTOM", 
  name: "カスタム" 
});
```

---

### `update_category1`
Update a major category name.

**Parameters:**
- `user_id` (i64): User ID
- `code` (String): Category code
- `name` (String): New category name

**Returns:**
- `Result<(), String>`: Success or error message

**Example:**
```javascript
await invoke('update_category1', { 
  user_id: 1, 
  code: "EXPENSE", 
  name: "支出（更新）" 
});
```

---

### `move_category1_order`
Move major category up or down.

**Parameters:**
- `user_id` (i64): User ID
- `code` (String): Category code
- `direction` (i32): -1 for up, 1 for down

**Returns:**
- `Result<(), String>`: Success or error message

**Logic:**
- Swaps display_order with adjacent category
- If already at top/bottom, no change occurs

**Example:**
```javascript
// Move up
await invoke('move_category1_order', { 
  user_id: 1, 
  code: "EXPENSE", 
  direction: -1 
});
```

---

### `delete_category1`
Delete a major category and all its children (CASCADE).

**Parameters:**
- `user_id` (i64): User ID
- `code` (String): Category code

**Returns:**
- `Result<(), String>`: Success or error message

**Notes:**
- This is an internal API, not exposed in UI
- Used only during user account deletion
- Foreign key constraints ensure CASCADE delete

**Example:**
```javascript
await invoke('delete_category1', { 
  user_id: 1, 
  code: "EXPENSE" 
});
```

---

## Category2 APIs (Medium Categories)

### `add_category2`
Add a new medium category.

**Parameters:**
- `user_id` (i64): User ID
- `category1_code` (String): Parent category code
- `name_ja` (String): Category name (Japanese)
- `name_en` (String): Category name (English)

**Returns:**
- `Result<String, String>`: New category code on success, error message on failure

**Notes:**
- Display order is automatically set to max + 1 within parent (appended to the end)
- Category code is auto-generated (e.g., "C2_E_1")
- **Duplicate check**: The following name duplicates are not allowed:
  - Japanese name matches existing Japanese name
  - English name matches existing English name
  - Japanese name matches existing English name
  - English name matches existing Japanese name
- Returns `CategoryError::DuplicateName` error if duplicate is detected

**Example:**
```javascript
try {
  const newCode = await invoke('add_category2', { 
    userId: 1, 
    category1Code: "EXPENSE",
    nameJa: "娯楽費",
    nameEn: "Entertainment"
  });
  console.log('Created category:', newCode);
} catch (error) {
  // Error message is translated according to current language
  alert(i18n.t('error.category_duplicate_name').replace('{0}', 'Entertainment'));
}
```

---

### `update_category2`
Update a medium category name.

**Parameters:**
- `user_id` (i64): User ID
- `category1_code` (String): Parent category code
- `category2_code` (String): Category code
- `name` (String): New category name

**Returns:**
- `Result<(), String>`: Success or error message

**Example:**
```javascript
await invoke('update_category2', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  name: "食費（更新）" 
});
```

---

### `move_category2_order`
Move medium category up or down within its parent.

**Parameters:**
- `user_id` (i64): User ID
- `category1_code` (String): Parent category code
- `category2_code` (String): Category code
- `direction` (i32): -1 for up, 1 for down

**Returns:**
- `Result<(), String>`: Success or error message

**Example:**
```javascript
await invoke('move_category2_order', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  direction: -1 
});
```

---

### `delete_category2`
Delete a medium category and all its children (CASCADE).

**Parameters:**
- `user_id` (i64): User ID
- `category1_code` (String): Parent category code
- `category2_code` (String): Category code

**Returns:**
- `Result<(), String>`: Success or error message

**Notes:**
- Internal API, not exposed in UI
- Used only during user account deletion

**Example:**
```javascript
await invoke('delete_category2', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1"
});
```

---

## Category3 APIs (Minor Categories)

### `add_category3`
Add a new minor category.

**Parameters:**
- `user_id` (i64): User ID
- `category1_code` (String): Major category code
- `category2_code` (String): Parent category code
- `name_ja` (String): Category name (Japanese)
- `name_en` (String): Category name (English)

**Returns:**
- `Result<String, String>`: New category code on success, error message on failure

**Notes:**
- Display order is automatically set to max + 1 within parent (appended to the end)
- Category code is auto-generated (e.g., "C3_E_1_1")
- **Duplicate check**: Same as `add_category2`, all language name combinations are checked for duplicates
- Returns `CategoryError::DuplicateName` error if duplicate is detected

**Example:**
```javascript
try {
  const newCode = await invoke('add_category3', { 
    userId: 1, 
    category1Code: "EXPENSE",
    category2Code: "C2_E_8",
    nameJa: "映画",
    nameEn: "Movie"
  });
  console.log('Created category:', newCode);
} catch (error) {
  alert(i18n.t('error.category_duplicate_name').replace('{0}', 'Movie'));
}
```

---

### `update_category3`
Update a minor category name.

**Parameters:**
- `user_id` (i64): User ID
- `category1_code` (String): Major category code
- `category2_code` (String): Parent category code
- `category3_code` (String): Category code
- `name` (String): New category name

**Returns:**
- `Result<(), String>`: Success or error message

**Example:**
```javascript
await invoke('update_category3', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  category3_code: "C3_1",
  name: "食料品（更新）" 
});
```

---

### `move_category3_order`
Move minor category up or down within its parent.

**Parameters:**
- `user_id` (i64): User ID
- `category1_code` (String): Major category code
- `category2_code` (String): Parent category code
- `category3_code` (String): Category code
- `direction` (i32): -1 for up, 1 for down

**Returns:**
- `Result<(), String>`: Success or error message

**Example:**
```javascript
await invoke('move_category3_order', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  category3_code: "C3_1",
  direction: 1 
});
```

---

### `delete_category3`
Delete a minor category.

**Parameters:**
- `user_id` (i64): User ID
- `category1_code` (String): Major category code
- `category2_code` (String): Parent category code
- `category3_code` (String): Category code

**Returns:**
- `Result<(), String>`: Success or error message

**Notes:**
- Internal API, not exposed in UI
- Used only during user account deletion

**Example:**
```javascript
await invoke('delete_category3', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  category3_code: "C3_1"
});
```

---

## Utility APIs

### `initialize_categories_for_new_user`
Initialize categories for a new user by copying from template user (USER_ID=1).

**Parameters:**
- `user_id` (i64): New user ID

**Returns:**
- `Result<(), String>`: Success or error message

**Notes:**
- Copies all Category1/2/3 from template user
- Copies all I18N records
- Executed automatically when a new user is created

**Example:**
```javascript
await invoke('initialize_categories_for_new_user', { 
  user_id: 2 
});
```

---

## Error Handling

All APIs return `Result<T, String>`:
- **Success**: `Ok(value)` - Operation completed successfully
- **Error**: `Err(message)` - Error message describing what went wrong

**Common Error Cases:**
1. **Database connection failure**
   - Message: "Failed to open database"
   
2. **Not found**
   - Message: "Category not found"
   
3. **Foreign key violation**
   - Message: "Parent category does not exist"
   
4. **SQL execution error**
   - Message: Specific SQLite error message

**Frontend Error Handling Example:**
```javascript
try {
  await invoke('add_category2', params);
  alert('Category added successfully');
} catch (error) {
  console.error('Failed to add category:', error);
  alert('Failed to add category: ' + error);
}
```

---

## Database Schema

### CATEGORY1 Table
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
```

### CATEGORY2 Table
```sql
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
    FOREIGN KEY(USER_ID, CATEGORY1_CODE) 
        REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE) 
        ON DELETE CASCADE
);
```

### CATEGORY3 Table
```sql
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
        ON DELETE CASCADE
);
```

### I18N Tables
Similar structure with `_I18N` suffix, storing language-specific names.

---

## Code Generation

**Frontend Code Generation (Recommended):**
```javascript
function generateCategoryCode(level) {
  const prefix = level === 2 ? 'C2_' : 'C3_';
  const timestamp = Date.now();
  return `${prefix}${timestamp}`;
}
```

**For Category2 with parent-specific prefix:**
```javascript
function generateCategory2Code(category1_code) {
  let prefix = 'C2_';
  if (category1_code === 'EXPENSE') prefix = 'C2_E_';
  else if (category1_code === 'INCOME') prefix = 'C2_I_';
  else if (category1_code === 'TRANSFER') prefix = 'C2_T_';
  
  return prefix + Date.now();
}
```

---

## Design Notes

### Major Categories (Category1)
- **Fixed set**: EXPENSE, INCOME, TRANSFER
- **No user modification**: Users cannot add/edit/delete
- **Display only**: Used as parent for medium categories

### Medium/Minor Categories (Category2/3)
- **User-managed**: Users can freely add/edit
- **No delete from UI**: Delete only during user account deletion
- **Automatic ordering**: New categories added to the end
- **Inline editing**: Direct editing without modal dialogs

### Delete Operations
- Delete APIs are implemented but not exposed in UI
- Used only during user account deletion
- CASCADE delete ensures referential integrity

---

## Version History

- **v1.0** (2025-10-28): Initial API documentation
  - Complete CRUD operations for Category1/2/3
  - Tree retrieval with i18n support
  - Reordering functionality

---

## See Also

- [Frontend Design (Phase 4)](./FRONTEND_DESIGN_PHASE4.md)
- [Testing Strategy](./TESTING.md)
- [TODO.md](../TODO.md)
