# IS_DISABLED Feature Implementation Guide (For Developers)

**Last Updated**: 2025-11-12 01:31 JST

## Overview

The IS_DISABLED feature enables logical deletion (hiding) of data without physical deletion. This document explains how to apply the functionality implemented in manufacturer and product management to other management screens.

---

## Implementation Pattern

IS_DISABLED feature implementation consists of 7 steps:

```
1. SQL: Add GET_ALL_INCLUDING_DISABLED query
2. SQL: Add IS_DISABLED column to INSERT/UPDATE
3. Backend: Add is_disabled field to Request struct
4. Backend: Add include_disabled parameter to get function
5. Tauri: Add is_disabled/include_disabled parameters to commands
6. HTML: Add toggle button + checkbox in modal
7. JavaScript: Add showDisabledItems state + styling + badge display
```

---

## Step 1: Add SQL Queries

### 1.1 GET_ALL_INCLUDING_DISABLED Query

Add a query to retrieve all items including disabled ones.

**Example (Manufacturer Management):**

```rust
// src/sql_queries.rs

pub const MANUFACTURER_GET_ALL_INCLUDING_DISABLED: &str = r#"
SELECT 
    MANUFACTURER_ID,
    USER_ID,
    MANUFACTURER_NAME,
    MEMO,
    DISPLAY_ORDER,
    IS_DISABLED,
    ENTRY_DT,
    UPDATE_DT
FROM MANUFACTURERS
WHERE USER_ID = ?
ORDER BY DISPLAY_ORDER
"#;
```

### 1.2 Update INSERT/UPDATE Queries

Update existing queries to include the IS_DISABLED column.

**Example (INSERT):**

```rust
pub const MANUFACTURER_INSERT: &str = r#"
INSERT INTO MANUFACTURERS (
    USER_ID,
    MANUFACTURER_NAME,
    MEMO,
    DISPLAY_ORDER,
    IS_DISABLED,
    ENTRY_DT
) VALUES (?, ?, ?, ?, ?, datetime('now', 'localtime'))
"#;
```

**Example (UPDATE):**

```rust
pub const MANUFACTURER_UPDATE: &str = r#"
UPDATE MANUFACTURERS SET
    MANUFACTURER_NAME = ?,
    MEMO = ?,
    DISPLAY_ORDER = ?,
    IS_DISABLED = ?,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND MANUFACTURER_ID = ?
"#;
```

---

## Step 2: Backend - Update Request Structs

### 2.1 AddRequest Struct

Add `is_disabled` field as `Option<i64>`.

```rust
// src/services/manufacturer.rs

#[derive(Debug, Deserialize, Clone)]
pub struct AddManufacturerRequest {
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub is_disabled: Option<i64>,  // Added
}
```

**Key Points:**
- Use `Option<i64>` (default value handled in function)
- 0 = active, 1 = disabled

### 2.2 UpdateRequest Struct

Add `is_disabled` field as `i64`.

```rust
#[derive(Debug, Deserialize)]
pub struct UpdateManufacturerRequest {
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,  // Added
}
```

**Key Points:**
- Required during update, so no `Option`

---

## Step 3: Backend - Update get Function

### 3.1 Add include_disabled Parameter

```rust
pub async fn get_manufacturers(
    pool: &SqlitePool, 
    user_id: i64, 
    include_disabled: bool  // Added
) -> Result<Vec<Manufacturer>, String> {
    let query = if include_disabled {
        sql_queries::MANUFACTURER_GET_ALL_INCLUDING_DISABLED
    } else {
        sql_queries::MANUFACTURER_GET_ALL
    };

    let manufacturers = sqlx::query_as::<_, Manufacturer>(query)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to get manufacturers: {}", e))?;

    Ok(manufacturers)
}
```

**Key Points:**
- Switch query based on `include_disabled` parameter
- Default excludes disabled items (false)

### 3.2 Update add/update Functions

**add Function:**

```rust
pub async fn add_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    request: AddManufacturerRequest,
) -> Result<String, String> {
    // Validation...

    let is_disabled = request.is_disabled.unwrap_or(0);  // Default 0

    sqlx::query(sql_queries::MANUFACTURER_INSERT)
        .bind(user_id)
        .bind(&request.manufacturer_name)
        .bind(&request.memo)
        .bind(display_order)
        .bind(is_disabled)  // Added
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to add manufacturer: {}", e))?;

    Ok("Manufacturer added successfully".to_string())
}
```

**update Function:**

```rust
pub async fn update_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: i64,
    request: UpdateManufacturerRequest,
) -> Result<String, String> {
    // Validation...

    sqlx::query(sql_queries::MANUFACTURER_UPDATE)
        .bind(&request.manufacturer_name)
        .bind(&request.memo)
        .bind(request.display_order)
        .bind(request.is_disabled)  // Added
        .bind(user_id)
        .bind(manufacturer_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update manufacturer: {}", e))?;

    Ok("Manufacturer updated successfully".to_string())
}
```

---

## Step 4: Tauri Commands

### 4.1 Update Command Functions

```rust
// src/lib.rs

#[tauri::command]
async fn get_manufacturers(
    state: tauri::State<'_, AppState>,
    user_id: i64,
    include_disabled: bool,  // Added
) -> Result<Vec<Manufacturer>, String> {
    let pool = &state.pool;
    manufacturer::get_manufacturers(pool, user_id, include_disabled).await
}

#[tauri::command]
async fn add_manufacturer(
    state: tauri::State<'_, AppState>,
    user_id: i64,
    manufacturer_name: String,
    memo: Option<String>,
    is_disabled: Option<i64>,  // Added
) -> Result<String, String> {
    let pool = &state.pool;
    let request = manufacturer::AddManufacturerRequest {
        manufacturer_name,
        memo,
        is_disabled,
    };
    manufacturer::add_manufacturer(pool, user_id, request).await
}

#[tauri::command]
async fn update_manufacturer(
    state: tauri::State<'_, AppState>,
    user_id: i64,
    manufacturer_id: i64,
    manufacturer_name: String,
    memo: Option<String>,
    display_order: i64,
    is_disabled: i64,  // Added
) -> Result<String, String> {
    let pool = &state.pool;
    let request = manufacturer::UpdateManufacturerRequest {
        manufacturer_name,
        memo,
        display_order,
        is_disabled,
    };
    manufacturer::update_manufacturer(pool, user_id, manufacturer_id, request).await
}
```

---

## Step 5: HTML - Add UI Components

### 5.1 Add Toggle Button

```html
<!-- res/manufacturer-management.html -->

<div class="toolbar">
    <button id="add-manufacturer-btn" class="btn btn-primary" data-i18n="manufacturer_mgmt.add"></button>
    <button id="toggle-disabled-btn" class="btn btn-secondary" data-i18n="common.show_disabled"></button>
</div>
```

### 5.2 Add Checkbox to Modal

```html
<div class="form-group">
    <label>
        <input type="checkbox" id="manufacturer-is-disabled">
        <span data-i18n="common.is_disabled"></span>
    </label>
</div>
```

---

## Step 6: JavaScript - Frontend Implementation

### 6.1 State Management

```javascript
// res/js/manufacturer-management.js

let showDisabledItems = false;
```

### 6.2 Event Listeners

```javascript
function setupEventListeners() {
    // Toggle button
    document.getElementById('toggle-disabled-btn').addEventListener('click', () => {
        showDisabledItems = !showDisabledItems;
        updateToggleButton();
        loadManufacturers();
    });
}

function updateToggleButton() {
    const btn = document.getElementById('toggle-disabled-btn');
    if (showDisabledItems) {
        btn.setAttribute('data-i18n', 'common.hide_disabled');
        btn.textContent = i18n.t('common.hide_disabled');
    } else {
        btn.setAttribute('data-i18n', 'common.show_disabled');
        btn.textContent = i18n.t('common.show_disabled');
    }
}
```

### 6.3 Data Loading

```javascript
async function loadManufacturers() {
    manufacturers = await invoke('get_manufacturers', {
        userId: currentUserId,
        includeDisabled: showDisabledItems  // Added
    });
    renderManufacturers();
}
```

### 6.4 Rendering - Styling

```javascript
function renderManufacturers() {
    manufacturers.forEach(manufacturer => {
        const row = tbody.insertRow();
        const isDisabled = manufacturer.is_disabled === 1;
        
        // Styling for disabled items
        if (isDisabled) {
            row.style.backgroundColor = '#6c757d';  // Dark gray
        }
        
        // Name cell + badge
        const nameCell = row.insertCell();
        if (isDisabled) {
            const badge = `<span style="color: #ffc107; font-weight: bold; margin-left: 8px;">[${i18n.t('common.disabled_label')}]</span>`;
            nameCell.innerHTML = `<span style="color: #ffffff;">${manufacturer.manufacturer_name}</span>${badge}`;
        } else {
            nameCell.textContent = manufacturer.manufacturer_name;
        }
        
        // Memo cell
        const memoCell = row.insertCell();
        if (isDisabled) {
            memoCell.style.color = '#ffffff';
        }
        // ...
    });
}
```

### 6.5 Modal - Data Read/Write

```javascript
function initManufacturerModal() {
    manufacturerModal = new Modal('manufacturer-modal', {
        onOpen: (mode, data) => {
            if (mode === 'add') {
                document.getElementById('manufacturer-is-disabled').checked = false;
            } else if (mode === 'edit') {
                document.getElementById('manufacturer-is-disabled').checked = data.is_disabled === 1;
            }
        },
        // ...
    });
}

async function saveManufacturer() {
    const isDisabled = document.getElementById('manufacturer-is-disabled').checked ? 1 : 0;
    
    if (editingManufacturerId) {
        await invoke('update_manufacturer', {
            // ...
            isDisabled: isDisabled
        });
    } else {
        await invoke('add_manufacturer', {
            // ...
            isDisabled: isDisabled === 1 ? isDisabled : null
        });
    }
}
```

---

## Step 7: Internationalization

### 7.1 Add Translation Resources

```javascript
// res/locales/ja.json
{
  "common": {
    "is_disabled": "非表示",
    "show_disabled": "非表示項目を表示",
    "hide_disabled": "非表示項目を隠す",
    "disabled_label": "非表示"
  }
}

// res/locales/en.json
{
  "common": {
    "is_disabled": "Disabled",
    "show_disabled": "Show Disabled Items",
    "hide_disabled": "Hide Disabled Items",
    "disabled_label": "Disabled"
  }
}
```

---

## Update Tests

### Backend Tests

```rust
#[tokio::test]
async fn test_add_manufacturer_with_is_disabled() {
    let pool = setup_test_db().await;

    let request = AddManufacturerRequest {
        manufacturer_name: "Test Manufacturer".to_string(),
        memo: None,
        is_disabled: Some(1),  // Add as disabled
    };

    add_manufacturer(&pool, 2, request).await.unwrap();

    // Not shown in normal retrieval
    let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
    assert_eq!(manufacturers.len(), 0);

    // Can retrieve with include_disabled=true
    let manufacturers_all = get_manufacturers(&pool, 2, true).await.unwrap();
    assert_eq!(manufacturers_all.len(), 1);
    assert_eq!(manufacturers_all[0].is_disabled, 1);
}
```

---

## Design Guidelines

### Color Palette

| Element | Color Code | Usage |
|---------|-----------|-------|
| Background (disabled row) | `#6c757d` | Dark Gray (Medium Gray) |
| Text (disabled row) | `#ffffff` | White (high contrast) |
| Badge color | `#ffc107` | Yellow (warning color) |

### Accessibility

- **Contrast Ratio**: Background `#6c757d` + Text `#ffffff` = High contrast
- **Buttons**: Action buttons remain normally visible even on disabled rows (no opacity)
- **Visual Feedback**: Badge clearly indicates disabled status

---

## Checklist

Items to verify during implementation:

### Backend
- [ ] Add `GET_ALL_INCLUDING_DISABLED` query
- [ ] Add `IS_DISABLED` to `INSERT` query
- [ ] Add `IS_DISABLED` to `UPDATE` query
- [ ] Add `is_disabled: Option<i64>` to `AddRequest`
- [ ] Add `is_disabled: i64` to `UpdateRequest`
- [ ] Add `include_disabled` parameter to `get` function
- [ ] Handle `is_disabled` default value in `add` function
- [ ] Add parameters to Tauri commands

### Frontend
- [ ] Add toggle button (HTML)
- [ ] Add checkbox to modal (HTML)
- [ ] Add `showDisabledItems` state (JS)
- [ ] Add toggle button event listener (JS)
- [ ] Add `includeDisabled` parameter to `loadXxx` function (JS)
- [ ] Add styling during rendering (JS)
- [ ] Add badge display (JS)
- [ ] Add read/write handling in modal (JS)

### Internationalization
- [ ] Add `common.is_disabled`
- [ ] Add `common.show_disabled`
- [ ] Add `common.hide_disabled`
- [ ] Add `common.disabled_label`

### Tests
- [ ] Add test for adding disabled item
- [ ] Add test for updating disabled item
- [ ] Add test for `include_disabled=false`
- [ ] Add test for `include_disabled=true`

---

## Reference Implementation

For complete implementation examples, refer to:

- **Backend**: `src/services/manufacturer.rs`
- **Backend**: `src/services/product.rs`
- **Frontend**: `res/js/manufacturer-management.js`
- **Frontend**: `res/js/product-management.js`
- **HTML**: `res/manufacturer-management.html`
- **HTML**: `res/product-management.html`
- **Commit**: `11269cb` - All changes

---

## Troubleshooting

### Issue: Disabled items are included in query

**Cause**: `include_disabled` parameter not passed correctly

**Solution**:
1. Check Tauri command parameter name
2. Verify `includeDisabled` in JavaScript `invoke` call
3. Check query branching in Backend get function

### Issue: Checkbox state not saved

**Cause**: `is_disabled` not passed in modal save processing

**Solution**:
1. Check `is_disabled` retrieval in `saveXxx` function
2. Verify `isDisabled` is included in `invoke` arguments
3. Check if Backend Request struct has `is_disabled` field

### Issue: Styling not applied

**Cause**: `is_disabled` value check incorrect

**Solution**:
1. Use strict comparison `manufacturer.is_disabled === 1`
2. Verify value returned from Backend is `i64` (number)

---

## Best Practices

1. **Default Value**: Default for `is_disabled` is 0 (active)
2. **Type Consistency**: Backend uses `i64`, Frontend uses JavaScript Number
3. **UI/UX**: Visually distinguish disabled items clearly
4. **Accessibility**: Maintain high contrast ratio
5. **Testing**: Test both disabled and active cases

---

**Change History**
- 2025-11-12: Initial version
