# Shop Management API Documentation

## Overview

This document describes the backend API for shop management in KakeiBon. It provides functionality for retrieving, adding, updating, and deleting shop information.

---

## Data Structures

### Shop

```rust
pub struct Shop {
    pub shop_id: i64,
    pub user_id: i64,
    pub shop_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

### AddShopRequest

```rust
pub struct AddShopRequest {
    pub shop_name: String,
    pub memo: Option<String>,
}
```

### UpdateShopRequest

```rust
pub struct UpdateShopRequest {
    pub shop_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
}
```

---

## API Reference

### 1. Get Shop List

#### `get_shops`

Retrieves the list of shops for a specified user. Only non-deleted shops are returned.

**Parameters:**
- `user_id` (i64): User ID

**Return Value:**
- `Vec<Shop>`: Array of shop information

**Response Example:**
```javascript
[
  {
    shop_id: 1,
    user_id: 2,
    shop_name: "AEON Shinjuku Store",
    memo: "Frequently used supermarket",
    display_order: 1,
    is_disabled: 0,
    entry_dt: "2024-11-10 12:00:00",
    update_dt: null
  },
  {
    shop_id: 2,
    user_id: 2,
    shop_name: "7-Eleven Station Front",
    memo: null,
    display_order: 2,
    is_disabled: 0,
    entry_dt: "2024-11-10 13:00:00",
    update_dt: null
  }
]
```

**Usage Example:**
```javascript
const shops = await invoke('get_shops', { userId: 2 });
console.log('Number of shops:', shops.length);
```

**Errors:**
- Returns error message if database error occurs

---

### 2. Add Shop

#### `add_shop`

Adds a new shop.

**Parameters:**
- `user_id` (i64): User ID
- `shop_name` (String): Shop name (required)
- `memo` (Option<String>): Memo (optional)

**Return Value:**
- `String`: Success message "Shop added successfully"

**Usage Example:**
```javascript
try {
  const result = await invoke('add_shop', {
    userId: 2,
    shopName: "AEON Shinjuku Store",
    memo: "Frequently used supermarket"
  });
  console.log(result); // "Shop added successfully"
} catch (error) {
  console.error('Add shop error:', error);
}
```

**Validation:**
- Returns error if shop name is empty
- Returns error if shop name already exists for the same user

**Error Messages:**
- `"Shop name cannot be empty"`: Shop name is empty
- `"Shop name already exists"`: Duplicate shop name
- `"Failed to add shop: {details}"`: Database error

**Notes:**
- `display_order` is automatically set (max existing value + 1)
- `is_disabled` is set to 0 (enabled) on creation

---

### 3. Update Shop

#### `update_shop`

Updates existing shop information.

**Parameters:**
- `user_id` (i64): User ID
- `shop_id` (i64): Shop ID
- `shop_name` (String): Shop name (required)
- `memo` (Option<String>): Memo (optional)
- `display_order` (i64): Display order

**Return Value:**
- `String`: Success message "Shop updated successfully"

**Usage Example:**
```javascript
try {
  const result = await invoke('update_shop', {
    userId: 2,
    shopId: 1,
    shopName: "AEON Shibuya Store",
    memo: "Store after renewal",
    displayOrder: 1
  });
  console.log(result); // "Shop updated successfully"
} catch (error) {
  console.error('Update shop error:', error);
}
```

**Validation:**
- Returns error if shop name is empty
- Returns error if specified shop doesn't exist
- Returns error if shop name already exists for another shop of the same user

**Error Messages:**
- `"Shop name cannot be empty"`: Shop name is empty
- `"Shop not found"`: Shop not found
- `"Shop name already exists"`: Duplicate shop name
- `"Failed to update shop: {details}"`: Database error

**Notes:**
- `update_dt` is automatically updated

---

### 4. Delete Shop

#### `delete_shop`

Logically deletes a shop (not physical deletion).

**Parameters:**
- `user_id` (i64): User ID
- `shop_id` (i64): Shop ID

**Return Value:**
- `String`: Success message "Shop deleted successfully"

**Usage Example:**
```javascript
try {
  const result = await invoke('delete_shop', {
    userId: 2,
    shopId: 1
  });
  console.log(result); // "Shop deleted successfully"
} catch (error) {
  console.error('Delete shop error:', error);
}
```

**Validation:**
- Returns error if specified shop doesn't exist

**Error Messages:**
- `"Shop not found"`: Shop not found
- `"Failed to delete shop: {details}"`: Database error

**Notes:**
- Logical deletion sets `is_disabled` flag to 1
- Data is not physically removed from database
- Deleted shops are not retrieved by `get_shops`

---

## Database Table

### SHOPS Table

```sql
CREATE TABLE SHOPS (
    SHOP_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    SHOP_NAME TEXT NOT NULL,
    MEMO TEXT,
    DISPLAY_ORDER INTEGER NOT NULL DEFAULT 0,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT TEXT NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT TEXT,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID)
);
```

### Indexes

- Index on `USER_ID` (for fast retrieval)
- Unique constraint on combination of `SHOP_NAME` and `USER_ID` (when not logically deleted)

---

## Usage Example: Complete CRUD Operations

```javascript
import { invoke } from '@tauri-apps/api/core';

// 1. Get shop list
async function loadShops(userId) {
  try {
    const shops = await invoke('get_shops', { userId });
    return shops;
  } catch (error) {
    console.error('Load shops error:', error);
    return [];
  }
}

// 2. Add shop
async function addNewShop(userId, shopName, memo) {
  try {
    await invoke('add_shop', {
      userId,
      shopName,
      memo
    });
    alert('Shop added successfully');
    return true;
  } catch (error) {
    alert(`Error: ${error}`);
    return false;
  }
}

// 3. Update shop
async function updateExistingShop(userId, shopId, shopName, memo, displayOrder) {
  try {
    await invoke('update_shop', {
      userId,
      shopId,
      shopName,
      memo,
      displayOrder
    });
    alert('Shop updated successfully');
    return true;
  } catch (error) {
    alert(`Error: ${error}`);
    return false;
  }
}

// 4. Delete shop
async function deleteExistingShop(userId, shopId) {
  if (!confirm('Are you sure you want to delete this shop?')) {
    return false;
  }
  
  try {
    await invoke('delete_shop', { userId, shopId });
    alert('Shop deleted successfully');
    return true;
  } catch (error) {
    alert(`Error: ${error}`);
    return false;
  }
}
```

---

## Security Considerations

1. **User Isolation**: Each user can only access their own shops
2. **Duplicate Check**: Prevents duplicate shop names within the same user
3. **Logical Deletion**: Physical deletion is not performed to maintain data integrity
4. **Validation**: Pre-checks for empty strings and invalid data

---

## Related Documentation

- [Transaction Management API](./API_TRANSACTION.md)
- [Account Management API](./API_ACCOUNT.md) (To be created)
- [Shop Management UI](./SHOP_MANAGEMENT_UI.md) (To be created)

---

## Tests

The shop management feature includes the following tests:

- `test_add_shop`: Add shop functionality
- `test_update_shop`: Update shop functionality
- `test_delete_shop`: Delete shop functionality
- `test_get_shops`: Get shop list functionality
- `test_duplicate_shop_name`: Duplicate check functionality

Run tests with:
```bash
cargo test --lib services::shop::tests
```

---

**Last Updated**: 2025-11-10 JST  
**Version**: 1.0.0
