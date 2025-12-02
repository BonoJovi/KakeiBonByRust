# Product Management API Specification

**Last Updated**: 2025-11-12 01:31 JST

## Overview

This document defines the Tauri Command API specifications for Product Management functionality.

---

## API List

| API Name | Description |
|----------|-------------|
| `get_products` | Get product list |
| `add_product` | Add product |
| `update_product` | Update product |
| `delete_product` | Delete product (logical deletion) |

---

## get_products

### Overview

Retrieves the list of products for the specified user. Returns with manufacturer names joined.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | `i64` | ✅ | User ID |
| `include_disabled` | `bool` | ✅ | Include disabled items (true: include, false: exclude) |

### Return Value

**On Success**: `Result<Vec<Product>, String>`

```rust
pub struct Product {
    pub product_id: i64,
    pub user_id: i64,
    pub product_name: String,
    pub manufacturer_id: Option<i64>,
    pub manufacturer_name: Option<String>,  // Retrieved via LEFT JOIN
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
// Get active products only
const products = await invoke('get_products', {
    userId: 1,
    includeDisabled: false
});

// Get including disabled items
const allProducts = await invoke('get_products', {
    userId: 1,
    includeDisabled: true
});
```

### SQL

**When include_disabled = false:**

```sql
SELECT 
    P.PRODUCT_ID,
    P.USER_ID,
    P.PRODUCT_NAME,
    P.MANUFACTURER_ID,
    M.MANUFACTURER_NAME,
    P.MEMO,
    P.DISPLAY_ORDER,
    P.IS_DISABLED,
    P.ENTRY_DT,
    P.UPDATE_DT
FROM PRODUCTS P
LEFT JOIN MANUFACTURERS M 
    ON P.MANUFACTURER_ID = M.MANUFACTURER_ID 
    AND M.IS_DISABLED = 0
WHERE P.USER_ID = ? AND P.IS_DISABLED = 0
ORDER BY P.DISPLAY_ORDER
```

**When include_disabled = true:**

```sql
SELECT 
    P.PRODUCT_ID,
    P.USER_ID,
    P.PRODUCT_NAME,
    P.MANUFACTURER_ID,
    M.MANUFACTURER_NAME,
    P.MEMO,
    P.DISPLAY_ORDER,
    P.IS_DISABLED,
    P.ENTRY_DT,
    P.UPDATE_DT
FROM PRODUCTS P
LEFT JOIN MANUFACTURERS M 
    ON P.MANUFACTURER_ID = M.MANUFACTURER_ID
WHERE P.USER_ID = ?
ORDER BY P.DISPLAY_ORDER
```

### Notes

- When manufacturer is disabled, `manufacturer_name` becomes `None` (due to LEFT JOIN condition)
- For products without manufacturer, both `manufacturer_id` and `manufacturer_name` are `None`

---

## add_product

### Overview

Adds a new product.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | `i64` | ✅ | User ID |
| `product_name` | `String` | ✅ | Product name |
| `manufacturer_id` | `Option<i64>` | ❌ | Manufacturer ID |
| `memo` | `Option<String>` | ❌ | Memo |
| `is_disabled` | `Option<i64>` | ❌ | Disabled flag (0: active, 1: disabled, default: 0) |

### Return Value

**On Success**: `Result<String, String>` - `"Product added successfully"`

**On Error**: `String` (error message)

### Validation

| Item | Rule | Error Message |
|------|------|--------------|
| `product_name` | Required, non-empty | `"Product name cannot be empty"` |
| `product_name` | Unique | `"Product name already exists"` |

### Usage Example (JavaScript)

```javascript
// Without manufacturer
await invoke('add_product', {
    userId: 1,
    productName: 'Canned Mackerel',
    manufacturerId: null,
    memo: 'In Water',
    isDisabled: null  // Default: 0 (active)
});

// With manufacturer
await invoke('add_product', {
    userId: 1,
    productName: 'Canned Mackerel',
    manufacturerId: 5,
    memo: 'In Water',
    isDisabled: null
});

// Add as disabled
await invoke('add_product', {
    userId: 1,
    productName: 'Test Product',
    manufacturerId: null,
    memo: null,
    isDisabled: 1  // Disabled
});
```

### SQL

```sql
INSERT INTO PRODUCTS (
    USER_ID,
    PRODUCT_NAME,
    MANUFACTURER_ID,
    MEMO,
    DISPLAY_ORDER,
    IS_DISABLED,
    ENTRY_DT
) VALUES (?, ?, ?, ?, ?, ?, datetime('now', 'localtime'))
```

### Notes

- `DISPLAY_ORDER` is automatically set (max value + 1)
- If `is_disabled` is `null`, default value `0` (active) is set
- Duplicate name check includes disabled items
- `manufacturer_id` is not validated for existence (NULL allowed)

---

## update_product

### Overview

Updates existing product information.

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | `i64` | ✅ | User ID |
| `product_id` | `i64` | ✅ | Product ID |
| `product_name` | `String` | ✅ | Product name |
| `manufacturer_id` | `Option<i64>` | ❌ | Manufacturer ID |
| `memo` | `Option<String>` | ❌ | Memo |
| `display_order` | `i64` | ✅ | Display order |
| `is_disabled` | `i64` | ✅ | Disabled flag (0: active, 1: disabled) |

### Return Value

**On Success**: `Result<String, String>` - `"Product updated successfully"`

**On Error**: `String` (error message)

### Validation

| Item | Rule | Error Message |
|------|------|--------------|
| `product_id` | Must exist | `"Product not found"` |
| `product_name` | Required, non-empty | `"Product name cannot be empty"` |
| `product_name` | Unique (excluding self) | `"Product name already exists"` |

### Usage Example (JavaScript)

```javascript
const product = products[0];

await invoke('update_product', {
    userId: 1,
    productId: product.product_id,
    productName: 'Mackerel in Water',
    manufacturerId: 5,
    memo: 'Updated memo',
    displayOrder: product.display_order,
    isDisabled: 0  // Active
});
```

### SQL

```sql
UPDATE PRODUCTS SET
    PRODUCT_NAME = ?,
    MANUFACTURER_ID = ?,
    MEMO = ?,
    DISPLAY_ORDER = ?,
    IS_DISABLED = ?,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND PRODUCT_ID = ?
```

### Notes

- Updating with the same product name is allowed (when not changing own name)
- Duplicate check excludes self
- Setting `manufacturer_id` to `null` removes manufacturer association

---

## delete_product

### Overview

Logically deletes a product (sets IS_DISABLED to 1).

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `user_id` | `i64` | ✅ | User ID |
| `product_id` | `i64` | ✅ | Product ID |

### Return Value

**On Success**: `Result<String, String>` - `"Product deleted successfully"`

**On Error**: `String` (error message)

### Validation

| Item | Rule | Error Message |
|------|------|--------------|
| `product_id` | Must exist | `"Product not found"` |

### Usage Example (JavaScript)

```javascript
await invoke('delete_product', {
    userId: 1,
    productId: 123
});
```

### SQL

```sql
UPDATE PRODUCTS SET
    IS_DISABLED = 1,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND PRODUCT_ID = ?
```

### Notes

- Performs logical deletion (IS_DISABLED = 1), not physical deletion
- Deleted products are not retrieved by `get_products` when `include_disabled=false`

---

## Error Handling

### Common Errors

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"Product name cannot be empty"` | Product name is empty | Enter valid product name |
| `"Product name already exists"` | Duplicate product name | Enter different product name |
| `"Product not found"` | Product does not exist | Specify valid product ID |
| `"Failed to get products: ..."` | Database error | Check database connection |
| `"Failed to add product: ..."` | Database error | Check database connection |
| `"Failed to update product: ..."` | Database error | Check database connection |
| `"Failed to delete product: ..."` | Database error | Check database connection |

---

## Data Model

### PRODUCTS Table

| Column Name | Type | NULL | Description |
|------------|------|------|-------------|
| `PRODUCT_ID` | INTEGER | ❌ | Primary key (auto-increment) |
| `USER_ID` | INTEGER | ❌ | User ID |
| `PRODUCT_NAME` | TEXT | ❌ | Product name |
| `MANUFACTURER_ID` | INTEGER | ✅ | Manufacturer ID (foreign key) |
| `MEMO` | TEXT | ✅ | Memo |
| `DISPLAY_ORDER` | INTEGER | ❌ | Display order |
| `IS_DISABLED` | INTEGER | ❌ | Disabled flag (0: active, 1: disabled) Default: 0 |
| `ENTRY_DT` | TEXT | ❌ | Entry date/time |
| `UPDATE_DT` | TEXT | ✅ | Update date/time |

### Indexes

```sql
CREATE UNIQUE INDEX IDX_PRODUCTS_USER_NAME 
ON PRODUCTS(USER_ID, PRODUCT_NAME);

CREATE INDEX IDX_PRODUCTS_USER_ORDER 
ON PRODUCTS(USER_ID, DISPLAY_ORDER);

CREATE INDEX IDX_PRODUCTS_MANUFACTURER 
ON PRODUCTS(MANUFACTURER_ID);
```

### Foreign Key Constraint

```sql
FOREIGN KEY (MANUFACTURER_ID) 
REFERENCES MANUFACTURERS(MANUFACTURER_ID) 
ON DELETE SET NULL
```

**Behavior:**
- Products are not deleted when manufacturer is deleted
- `MANUFACTURER_ID` is set to `NULL` (ON DELETE SET NULL)

---

## Manufacturer Relationship

### Behavior When Manufacturer is Deleted

When a manufacturer is logically deleted (IS_DISABLED = 1):

1. **Product Data**: Not deleted (`PRODUCT.MANUFACTURER_ID` is retained)
2. **List Display**: `get_products` returns `manufacturer_name` as `None`
3. **Reason**: LEFT JOIN condition `M.IS_DISABLED = 0` prevents joining disabled manufacturers

### Usage Example

```javascript
// Logically delete manufacturer
await invoke('delete_manufacturer', { userId: 1, manufacturerId: 5 });

// Get products (manufacturer_id remains but name is None)
const products = await invoke('get_products', { userId: 1, includeDisabled: false });
// products[0].manufacturer_id = 5
// products[0].manufacturer_name = null  ← Because manufacturer is disabled
```

---

## Related Documentation

- [Manufacturer & Product Management - User Guide](./MANUFACTURER_PRODUCT_MANAGEMENT.md)
- [IS_DISABLED Implementation Guide (For Developers)](./IS_DISABLED_IMPLEMENTATION_GUIDE.md)
- [Manufacturer Management API Specification](./API_MANUFACTURER.md)

---

**Change History**
- 2025-11-12: Initial version (includes IS_DISABLED feature)
