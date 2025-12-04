# Category Management API Reference

**Last Updated**: 2025-12-05 02:25 JST

## Overview

This document defines APIs used in the category management screen (category-management.html). Manages a 3-level category structure (Category1, Category2, Category3).

---

## Table of Contents

1. [Category Tree Retrieval API](#category-tree-retrieval-api)
2. [Category2 Management API](#category2-management-api)
3. [Category3 Management API](#category3-management-api)
4. [Data Structures](#data-structures)

---

## Category Tree Retrieval API

### get_category_tree_with_lang

Retrieves the complete category tree (3 levels) with multilingual names.

**Parameters:**
- `user_id` (i64): User ID
- `lang_code` (Option<String>): Language code ("ja", "en", etc.)

**Return Value:**
- `Vec<CategoryTree>`: Array of category trees with multilingual names

**Response Structure:**
```javascript
[
  {
    category1: {
      user_id: 1,
      category1_code: "EXPENSE",
      category1_name: "支出",
      category1_name_i18n: "Expense",  // From I18N table
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

**Usage Example:**
```javascript
const tree = await invoke('get_category_tree_with_lang', { 
    userId: 1, 
    langCode: 'ja' 
});
```

**Note:**
- Category1 (top level) is fixed at 3: EXPENSE, INCOME, TRANSFER
- Automatically populated when user is created
- Category1 cannot be added, edited, or deleted

---

## Category2 Management API

### add_category2

Adds a new Category2 (middle level).

**Parameters:**
- `category1_code` (String): Parent Category1 code ("EXPENSE", "INCOME", "TRANSFER")
- `name_ja` (String): Japanese name
- `name_en` (String): English name

**Return Value:**
- `String`: Generated category code (e.g., "C2_E_21")

**Usage Example:**
```javascript
try {
    const code = await invoke('add_category2', {
        category1Code: 'EXPENSE',
        nameJa: '日用品',
        nameEn: 'Daily Necessities'
    });
    console.log(`Category2 added: ${code}`);
} catch (error) {
    alert(`Addition failed: ${error}`);
}
```

**Automatic Processing:**
1. Category code auto-generation (C2_E_1, C2_E_2...)
2. Display order auto-assignment (max + 1)
3. I18N table registration (Japanese & English)
4. is_disabled = 0 (enabled)

**Validation:**
- Duplicate name check within same parent

**Error:**
- `"Category name '...' already exists"`: Duplicate name

---

### get_category2_for_edit

Retrieves Category2 details for editing.

**Parameters:**
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code

**Return Value:**
- `CategoryForEdit`: Category information for editing

**CategoryForEdit Structure:**
```javascript
{
    code: string,
    name_ja: string,
    name_en: string,
    display_order: number
}
```

**Usage Example:**
```javascript
const category = await invoke('get_category2_for_edit', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1'
});

// Populate form
document.getElementById('name-ja').value = category.name_ja;
document.getElementById('name-en').value = category.name_en;
```

**Note:**
- Session user ID automatically retrieved
- Used when displaying edit modal

---

### update_category2

Updates a Category2.

**Parameters:**
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code
- `name_ja` (String): New Japanese name
- `name_en` (String): New English name

**Return Value:** None

**Usage Example:**
```javascript
await invoke('update_category2', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    nameJa: '食費（更新）',
    nameEn: 'Food (Updated)'
});
```

**Behavior:**
- Updates name in CATEGORY table
- Updates both languages in I18N table

**Validation:**
- Duplicate name check excluding self

---

### move_category2_up

Moves a Category2 up one position.

**Parameters:**
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code

**Return Value:** None

**Usage Example:**
```javascript
await invoke('move_category2_up', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_2'
});
```

**Behavior:**
- Swaps display_order with sibling above
- Does nothing if already at top

---

### move_category2_down

Moves a Category2 down one position.

**Parameters:**
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code

**Return Value:** None

**Usage Example:**
```javascript
await invoke('move_category2_down', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1'
});
```

**Behavior:**
- Swaps display_order with sibling below
- Does nothing if already at bottom

---

## Category3 Management API

### add_category3

Adds a new Category3 (lowest level).

**Parameters:**
- `category1_code` (String): Category1 code
- `category2_code` (String): Parent Category2 code
- `name_ja` (String): Japanese name
- `name_en` (String): English name

**Return Value:**
- `String`: Generated category code (e.g., "C3_127")

**Usage Example:**
```javascript
const code = await invoke('add_category3', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    nameJa: '外食',
    nameEn: 'Dining Out'
});
```

**Automatic Processing:**
- Same as add_category2

---

### get_category3_for_edit

Retrieves Category3 details for editing.

**Parameters:**
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code
- `category3_code` (String): Category3 code

**Return Value:**
- `CategoryForEdit`: Category information for editing

**Usage Example:**
```javascript
const category = await invoke('get_category3_for_edit', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1'
});
```

---

### update_category3

Updates a Category3.

**Parameters:**
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code
- `category3_code` (String): Category3 code
- `name_ja` (String): New Japanese name
- `name_en` (String): New English name

**Return Value:** None

**Usage Example:**
```javascript
await invoke('update_category3', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    nameJa: '食料品（更新）',
    nameEn: 'Groceries (Updated)'
});
```

---

### move_category3_up

Moves a Category3 up one position.

**Parameters:**
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code
- `category3_code` (String): Category3 code

**Return Value:** None

**Usage Example:**
```javascript
await invoke('move_category3_up', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_2'
});
```

---

### move_category3_down

Moves a Category3 down one position.

**Parameters:**
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code
- `category3_code` (String): Category3 code

**Return Value:** None

**Usage Example:**
```javascript
await invoke('move_category3_down', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1'
});
```

---

## Data Structures

### CategoryTree

```rust
// Top level structure (Category1)
pub struct CategoryTree {
    pub category1: Category1,
    pub children: Vec<Category2Tree>,  // Array of Category2
}

// Category2 level
pub struct Category2Tree {
    pub category2: Category2,
    pub children: Vec<Category3>,  // Array of Category3
}
```

### Category1 (Top Level)

```rust
pub struct Category1 {
    pub user_id: i64,
    pub category1_code: String,     // "EXPENSE", "INCOME", "TRANSFER"
    pub category1_name: String,     // "支出", "収入", "振替"
    pub category1_name_i18n: String, // Translated name
    pub display_order: i64,
    pub is_disabled: bool,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

**Fixed Values:**
- EXPENSE (Expense)
- INCOME (Income)
- TRANSFER (Transfer)

---

### Category2 (Middle Level)

```rust
pub struct Category2 {
    pub user_id: i64,
    pub category1_code: String,
    pub category2_code: String,     // "C2_E_1", "C2_E_2"...
    pub category2_name: String,
    pub category2_name_i18n: String,
    pub display_order: i64,
    pub is_disabled: bool,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

---

### Category3 (Lowest Level)

```rust
pub struct Category3 {
    pub user_id: i64,
    pub category1_code: String,
    pub category2_code: String,
    pub category3_code: String,     // "C3_1", "C3_2"...
    pub category3_name: String,
    pub category3_name_i18n: String,
    pub display_order: i64,
    pub is_disabled: bool,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

---

### CategoryForEdit (For Editing)

```rust
pub struct CategoryForEdit {
    pub code: String,
    pub name_ja: String,
    pub name_en: String,
    pub display_order: i64,
}
```

**Purpose:**
- Display data in edit modal
- Retrieve both Japanese and English names

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"User not authenticated"` | Session not authenticated | Login required |
| `"Category name '...' already exists"` | Duplicate name | Use different name |
| `"Failed to add category2: ..."` | Addition error | Check database |
| `"Failed to update category2: ..."` | Update error | Check database |
| `"Failed to move category2 up: ..."` | Move error | Check database |

### Frontend Error Handling Example

```javascript
// Add Category2
async function addCategory2(category1Code, nameJa, nameEn) {
    try {
        const code = await invoke('add_category2', {
            category1Code,
            nameJa,
            nameEn
        });
        
        alert(`Category2 added: ${code}`);
        await reloadCategoryTree();
        return code;
    } catch (error) {
        if (error.includes('already exists')) {
            alert('This name is already in use');
        } else {
            alert(`Error: ${error}`);
        }
        return null;
    }
}

// Move category
async function moveUp(category1Code, category2Code) {
    try {
        await invoke('move_category2_up', {
            category1Code,
            category2Code
        });
        
        // Optimistic UI update
        await reloadCategoryTree();
    } catch (error) {
        console.error('Move error:', error);
        alert(`Move failed: ${error}`);
    }
}
```

---

## Category Structure Design

### Hierarchical Structure

```
Category1 (Top) ← Fixed (3 categories)
  ├─ Category2 (Middle) ← User-addable
  │   ├─ Category3 (Lowest) ← User-addable
  │   └─ Category3
  └─ Category2
      └─ Category3
```

### Category Code System

**Category1:**
- EXPENSE (Expense)
- INCOME (Income)
- TRANSFER (Transfer)

**Category2:**
- C2_E_1, C2_E_2... (Expense subcategories)
- C2_I_1, C2_I_2... (Income subcategories)
- C2_T_1, C2_T_2... (Transfer subcategories)

**Category3:**
- C3_1, C3_2... (Sequential numbers only)

### Default Categories

Auto-populated when user is created:

- **Category2**: 20 categories
- **Category3**: 126 categories
- **I18N**: Japanese and English translations

**Population Process:**
- Automatically called when `create_general_user` executes
- `populate_default_categories` function
- Loaded from `res/sql/default_categories_seed.sql`

---

## Security Considerations

### User Isolation

1. **Session User ID**: Auto-retrieved in each API (`get_session_user_id`)
2. **Data Isolation**: Each user can only access their own categories
3. **Fixed Category1**: Prevents malicious operations

### Name Uniqueness

1. **No duplicates within same level and parent**
2. **Exclude self when checking during edit**
3. **I18N table also managed simultaneously**

### Cascade Deletion

1. **Category2 deletion**: Child Category3 also deleted (not implemented)
2. **User deletion**: All categories deleted
3. **Foreign key constraints**: Ensure data integrity

---

## Usage Example: Category Management Screen Implementation

### Category Tree Display

```javascript
async function loadCategoryTree() {
    try {
        const user = await invoke('get_current_session_user');
        const lang = localStorage.getItem('language') || 'ja';
        
        const tree = await invoke('get_category_tree_with_lang', {
            userId: user.user_id,
            langCode: lang
        });
        
        renderCategoryTree(tree);
    } catch (error) {
        console.error('Tree loading error:', error);
    }
}

function renderCategoryTree(tree) {
    const container = document.getElementById('category-tree');
    container.innerHTML = '';
    
    tree.forEach(cat1 => {
        const cat1Div = createCategory1Element(cat1);
        container.appendChild(cat1Div);
    });
}
```

### Add Category2 Modal

```javascript
async function handleAddCategory2(event) {
    event.preventDefault();
    
    const category1Code = document.getElementById('category1-code').value;
    const nameJa = document.getElementById('name-ja').value;
    const nameEn = document.getElementById('name-en').value;
    
    try {
        const code = await invoke('add_category2', {
            category1Code,
            nameJa,
            nameEn
        });
        
        alert(`Category2 added: ${code}`);
        closeModal();
        await loadCategoryTree();
    } catch (error) {
        alert(`Error: ${error}`);
    }
}
```

---

## Test Coverage

**CategoryService:**
- ✅ Category tree retrieval test
- ✅ Category2 addition test
- ✅ Category3 addition test
- ✅ Category update test (Japanese/English)
- ✅ Category move test (up/down)
- ✅ Duplicate name check
- ✅ Default category population test (20 Category2, 126 Category3)

---

## Related Documents

### Implementation Files

- Category Service: `src/services/category.rs`
- I18N Service: `src/services/i18n.rs`
- SQL Definitions: `src/sql_queries.rs`
- Default Data: `res/sql/default_categories_seed.sql`
- Tauri Commands: `src/lib.rs`

### Other API References

- [Common API](./API_COMMON.md) - Session management, I18n
- [Transaction Management API](./API_TRANSACTION.md) - Category usage

---

**Change History:**
- 2025-10-28: Initial version
- 2025-12-05: Complete revision based on implementation code
  - Removed unimplemented Category1 APIs
  - Added get_category2_for_edit, get_category3_for_edit
  - Unified with new template
  - Fixed parameter names to camelCase
