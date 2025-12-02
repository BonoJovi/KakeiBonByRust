# Manufacturer Management API Specification

**Last Updated**: 2025-11-12 01:31 JST

## Overview

This document defines the Tauri Command API specifications for Manufacturer Management functionality.

---

## API List

| API Name | Description |
|----------|-------------|
| `get_manufacturers` | Get manufacturer list |
| `add_manufacturer` | Add manufacturer |
| `update_manufacturer` | Update manufacturer |
| `delete_manufacturer` | Delete manufacturer (logical deletion) |

---

## get_manufacturers

### Overview

Retrieves the list of manufacturers for the specified user.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | `i64` | ✅ | User ID |
| `include_disabled` | `bool` | ✅ | Include disabled items (true: include, false: exclude) |

### Return Value

**On Success**: `Result<Vec<Manufacturer>, String>`

```rust
pub struct Manufacturer {
    pub manufacturer_id: i64,
    pub user_id: i64,
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,  // 0: active, 1: disabled
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

**On Error**: `String` (error message)

### Usage Example (JavaScript)

```javascript
// Get active manufacturers only
const manufacturers = await invoke('get_manufacturers', {
    userId: 1,
    includeDisabled: false
});

// Get including disabled items
const allManufacturers = await invoke('get_manufacturers', {
    userId: 1,
    includeDisabled: true
});
```

### SQL

**When include_disabled = false:**

```sql
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
WHERE USER_ID = ? AND IS_DISABLED = 0
ORDER BY DISPLAY_ORDER
```

**When include_disabled = true:**

```sql
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
```

---

## add_manufacturer

### Overview

Adds a new manufacturer.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | `i64` | ✅ | User ID |
| `manufacturer_name` | `String` | ✅ | Manufacturer name |
| `memo` | `Option<String>` | ❌ | Memo |
| `is_disabled` | `Option<i64>` | ❌ | Disabled flag (0: active, 1: disabled, default: 0) |

### Return Value

**On Success**: `Result<String, String>` - `"Manufacturer added successfully"`

**On Error**: `String` (error message)

### Validation

| Item | Rule | Error Message |
|------|------|--------------|
| `manufacturer_name` | Required, non-empty | `"Manufacturer name cannot be empty"` |
| `manufacturer_name` | Unique | `"Manufacturer name already exists"` |

### Usage Example (JavaScript)

```javascript
// Basic addition
await invoke('add_manufacturer', {
    userId: 1,
    manufacturerName: 'Nissui',
    memo: 'Nippon Suisan Kaisha, Ltd.',
    isDisabled: null  // Default: 0 (active)
});

// Add as disabled
await invoke('add_manufacturer', {
    userId: 1,
    manufacturerName: 'Test Manufacturer',
    memo: null,
    isDisabled: 1  // Disabled
});
```

### SQL

```sql
INSERT INTO MANUFACTURERS (
    USER_ID,
    MANUFACTURER_NAME,
    MEMO,
    DISPLAY_ORDER,
    IS_DISABLED,
    ENTRY_DT
) VALUES (?, ?, ?, ?, ?, datetime('now', 'localtime'))
```

### Notes

- `DISPLAY_ORDER` is automatically set (max value + 1)
- If `is_disabled` is `null`, default value `0` (active) is set
- Duplicate name check includes disabled items

---

## update_manufacturer

### Overview

Updates existing manufacturer information.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | `i64` | ✅ | User ID |
| `manufacturer_id` | `i64` | ✅ | Manufacturer ID |
| `manufacturer_name` | `String` | ✅ | Manufacturer name |
| `memo` | `Option<String>` | ❌ | Memo |
| `display_order` | `i64` | ✅ | Display order |
| `is_disabled` | `i64` | ✅ | Disabled flag (0: active, 1: disabled) |

### Return Value

**On Success**: `Result<String, String>` - `"Manufacturer updated successfully"`

**On Error**: `String` (error message)

### Validation

| Item | Rule | Error Message |
|------|------|--------------|
| `manufacturer_id` | Must exist | `"Manufacturer not found"` |
| `manufacturer_name` | Required, non-empty | `"Manufacturer name cannot be empty"` |
| `manufacturer_name` | Unique (excluding self) | `"Manufacturer name already exists"` |

### Usage Example (JavaScript)

```javascript
const manufacturer = manufacturers[0];

await invoke('update_manufacturer', {
    userId: 1,
    manufacturerId: manufacturer.manufacturer_id,
    manufacturerName: 'Nippon Suisan',
    memo: 'Updated memo',
    displayOrder: manufacturer.display_order,
    isDisabled: 0  // Active
});
```

### SQL

```sql
UPDATE MANUFACTURERS SET
    MANUFACTURER_NAME = ?,
    MEMO = ?,
    DISPLAY_ORDER = ?,
    IS_DISABLED = ?,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND MANUFACTURER_ID = ?
```

### Notes

- Updating with the same manufacturer name is allowed (when not changing own name)
- Duplicate check excludes self

---

## delete_manufacturer

### Overview

Logically deletes a manufacturer (sets IS_DISABLED to 1).

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | `i64` | ✅ | User ID |
| `manufacturer_id` | `i64` | ✅ | Manufacturer ID |

### Return Value

**On Success**: `Result<String, String>` - `"Manufacturer deleted successfully"`

**On Error**: `String` (error message)

### Validation

| Item | Rule | Error Message |
|------|------|--------------|
| `manufacturer_id` | Must exist | `"Manufacturer not found"` |

### Usage Example (JavaScript)

```javascript
await invoke('delete_manufacturer', {
    userId: 1,
    manufacturerId: 123
});
```

### SQL

```sql
UPDATE MANUFACTURERS SET
    IS_DISABLED = 1,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND MANUFACTURER_ID = ?
```

### Notes

- Performs logical deletion (IS_DISABLED = 1), not physical deletion
- Deleted manufacturers are not retrieved by `get_manufacturers` when `include_disabled=false`
- Products associated with deleted manufacturers are not deleted (product data is retained)

---

## Error Handling

### Common Errors

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"Manufacturer name cannot be empty"` | Manufacturer name is empty | Enter valid manufacturer name |
| `"Manufacturer name already exists"` | Duplicate manufacturer name | Enter different manufacturer name |
| `"Manufacturer not found"` | Manufacturer does not exist | Specify valid manufacturer ID |
| `"Failed to get manufacturers: ..."` | Database error | Check database connection |
| `"Failed to add manufacturer: ..."` | Database error | Check database connection |
| `"Failed to update manufacturer: ..."` | Database error | Check database connection |
| `"Failed to delete manufacturer: ..."` | Database error | Check database connection |

---

## Data Model

### MANUFACTURERS Table

| Column Name | Type | NULL | Description |
|------------|------|------|-------------|
| `MANUFACTURER_ID` | INTEGER | ❌ | Primary key (auto-increment) |
| `USER_ID` | INTEGER | ❌ | User ID |
| `MANUFACTURER_NAME` | TEXT | ❌ | Manufacturer name |
| `MEMO` | TEXT | ✅ | Memo |
| `DISPLAY_ORDER` | INTEGER | ❌ | Display order |
| `IS_DISABLED` | INTEGER | ❌ | Disabled flag (0: active, 1: disabled) Default: 0 |
| `ENTRY_DT` | TEXT | ❌ | Entry date/time |
| `UPDATE_DT` | TEXT | ✅ | Update date/time |

### Indexes

```sql
CREATE UNIQUE INDEX IDX_MANUFACTURERS_USER_NAME 
ON MANUFACTURERS(USER_ID, MANUFACTURER_NAME);

CREATE INDEX IDX_MANUFACTURERS_USER_ORDER 
ON MANUFACTURERS(USER_ID, DISPLAY_ORDER);
```

---

## Related Documentation

- [Manufacturer & Product Management - User Guide](./MANUFACTURER_PRODUCT_MANAGEMENT.md)
- [IS_DISABLED Implementation Guide (For Developers)](./IS_DISABLED_IMPLEMENTATION_GUIDE.md)
- [Product Management API Specification](./API_PRODUCT.md)

---

**Change History**
- 2025-11-12: Initial version (includes IS_DISABLED feature)
