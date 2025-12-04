# API Reference

**Last Updated**: 2025-12-05 00:56 JST

## Overview

This document is the consolidated API reference for all KakeiBon backend APIs. Code examples are kept minimal, focusing on API structure and parameters.

---

## Table of Contents

1. [Common Specifications](#common-specifications)
2. [Category Management API](#category-management-api)
3. [Transaction Management API](#transaction-management-api)
4. [Aggregation API](#aggregation-api)
5. [Shop Management API](#shop-management-api)
6. [Manufacturer Management API](#manufacturer-management-api)
7. [Product Management API](#product-management-api)
8. [Error Handling](#error-handling)

---

## Common Specifications

### Invocation Method

All APIs are implemented as Tauri Commands and invoked from the frontend using `invoke`.

```javascript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('command_name', { param1: value1, param2: value2 });
```

### Common Parameters

- `user_id` (i64): Required for all APIs. Data is isolated per user.

### Common Return Values

- Success: The `T` part of `Result<T, String>`
- Error: Error message as `String`

### Logical Deletion

Many entities use logical deletion:

- `is_disabled = 0`: Active
- `is_disabled = 1`: Hidden (logically deleted)

---

## Category Management API

### get_category_tree

Retrieves the 3-level category tree.

**Parameters:**
- `user_id` (i64): User ID

**Returns:**
- `Vec<CategoryTree>`: Array of category trees

---

### get_category_tree_with_lang

Retrieves category tree with multilingual names.

**Parameters:**
- `user_id` (i64): User ID
- `lang_code` (Option\<String\>): Language code ("ja", "en", etc.)

**Returns:**
- `Vec<CategoryTree>`: Category tree with i18n names

**Response Structure:**
- `category1_name_i18n`: Translated name from I18N table
- `category2_name_i18n`: Same
- `category3_name_i18n`: Same

---

### add_category1 / add_category2 / add_category3

Adds a category at each level.

**Common Parameters:**
- `user_id` (i64)
- `category_name` (String): Category name
- `display_order` (i64): Display order
- Parent category code (category2/3 only)

**Common Returns:**
- `String`: "Category added successfully"

**Validation:**
- Category name is required
- No duplicates within same level and parent

---

### update_category1 / update_category2 / update_category3

Updates a category at each level.

**Common Parameters:**
- `user_id` (i64)
- Category code
- `category_name` (String)
- `display_order` (i64)
- `is_disabled` (i64)

**Common Returns:**
- `String`: "Category updated successfully"

---

### delete_category1 / delete_category2 / delete_category3

Logically deletes a category.

**Parameters:**
- `user_id` (i64)
- Category code

**Returns:**
- `String`: "Category deleted successfully"

**Note:**
- Cannot delete if child categories exist

---

## Transaction Management API

### get_transaction_header

Retrieves a single transaction header.

**Parameters:**
- `transaction_id` (i64): Transaction ID

**Returns:**
- `serde_json::Value`: Transaction header with memo

**Response Fields:**
- `transaction_id`, `user_id`, `transaction_date`
- `category1_code`, `from_account_code`, `to_account_code`
- `total_amount`, `tax_rounding_type`
- `memo_id`, `shop_id`, `is_disabled`
- `entry_dt`, `update_dt`
- `memo`: Memo text

---

### get_transaction_headers_by_date_range

Retrieves headers within a date range.

**Parameters:**
- `user_id` (i64)
- `start_date` (String): "YYYY-MM-DD"
- `end_date` (String): "YYYY-MM-DD"

**Returns:**
- `Vec<serde_json::Value>`: Array of headers

---

### get_transaction_details

Retrieves transaction details.

**Parameters:**
- `transaction_id` (i64)

**Returns:**
- `Vec<TransactionDetail>`: Array of details

**Detail Fields:**
- `detail_id`, `transaction_id`, `line_number`
- `category2_code`, `category3_code`
- `item_name`, `quantity`, `unit_price`
- `subtotal`, `tax_rate`, `tax_amount`
- `product_id`, `manufacturer_id`

---

### save_transaction

Creates a new transaction.

**Parameters:**
- `user_id` (i64)
- `header` (TransactionHeader): Header data
- `details` (Vec\<TransactionDetail\>): Detail array
- `memo` (Option\<String\>): Memo text

**Returns:**
- `i64`: Created transaction_id

**Transaction:**
- Saves header, details, and memo within a transaction

---

### update_transaction

Updates an existing transaction.

**Parameters:**
- `user_id` (i64)
- `transaction_id` (i64)
- `header` (TransactionHeader)
- `details` (Vec\<TransactionDetail\>)
- `memo` (Option\<String\>)

**Returns:**
- `String`: "Transaction updated successfully"

**Behavior:**
- Deletes existing details and inserts new ones
- Updates memo

---

### delete_transaction

Logically deletes a transaction.

**Parameters:**
- `user_id` (i64)
- `transaction_id` (i64)

**Returns:**
- `String`: "Transaction deleted successfully"

**Note:**
- Only header is logically deleted (details are not physically deleted)

---

## Aggregation API

### aggregate_by_monthly

Performs monthly aggregation.

**Parameters:**
- `user_id` (i64)
- `start_date` (String): "YYYY-MM-DD"
- `end_date` (String): "YYYY-MM-DD"
- `group_by` (GroupBy): Grouping condition
  - `Category1`, `Category2`, `Category3`
  - `FromAccount`, `ToAccount`, `Shop`
- `category1_code` (Option\<String\>): Filter

**Returns:**
- `Vec<AggregationResult>`: Aggregation results

**Result Fields:**
- `period`: "YYYY-MM"
- `group_value`: Grouping key
- `total_amount`: Total amount
- `transaction_count`: Transaction count

---

### aggregate_by_daily

Performs daily aggregation.

**Parameters:**
- Same as monthly aggregation

**Returns:**
- `Vec<AggregationResult>` (with `period` in "YYYY-MM-DD" format)

---

### aggregate_by_period

Performs period aggregation (entire period as one unit).

**Parameters:**
- Same as monthly aggregation

**Returns:**
- `Vec<AggregationResult>` (with `period` in "YYYY-MM-DD to YYYY-MM-DD" format)

---

### aggregate_by_weekly

Performs weekly aggregation.

**Parameters:**
- Same as monthly aggregation

**Returns:**
- `Vec<AggregationResult>` (with `period` as week start date "YYYY-MM-DD")

---

### aggregate_by_yearly

Performs yearly aggregation.

**Parameters:**
- Same as monthly aggregation

**Returns:**
- `Vec<AggregationResult>` (with `period` in "YYYY" format)

---

### Architecture

The Aggregation API uses a 3-layer architecture:

1. **Core Function**: Dynamic SQL generation and query execution
2. **Wrapper Function**: Validation and database connection management
3. **Tauri Command**: Parameter conversion and error handling

**GroupBy Enum:**
```rust
pub enum GroupBy {
    Category1,
    Category2,
    Category3,
    FromAccount,
    ToAccount,
    Shop,
}
```

---

## Shop Management API

### get_shops

Retrieves shop list.

**Parameters:**
- `user_id` (i64)

**Returns:**
- `Vec<Shop>`: Non-deleted shops only

**Shop Fields:**
- `shop_id`, `user_id`, `shop_name`
- `memo`, `display_order`, `is_disabled`
- `entry_dt`, `update_dt`

---

### add_shop

Adds a shop.

**Parameters:**
- `user_id` (i64)
- `shop_name` (String): Required
- `memo` (Option\<String\>)

**Returns:**
- `String`: "Shop added successfully"

**Validation:**
- Shop name is required
- No duplicates within same user

**Auto-set:**
- `display_order`: max + 1
- `is_disabled`: 0

---

### update_shop

Updates a shop.

**Parameters:**
- `user_id` (i64)
- `shop_id` (i64)
- `shop_name` (String)
- `memo` (Option\<String\>)
- `display_order` (i64)

**Returns:**
- `String`: "Shop updated successfully"

---

### delete_shop

Logically deletes a shop.

**Parameters:**
- `user_id` (i64)
- `shop_id` (i64)

**Returns:**
- `String`: "Shop deleted successfully"

---

## Manufacturer Management API

### get_manufacturers

Retrieves manufacturer list.

**Parameters:**
- `user_id` (i64)
- `include_disabled` (bool): Include hidden items

**Returns:**
- `Vec<Manufacturer>`

**Manufacturer Fields:**
- `manufacturer_id`, `user_id`, `manufacturer_name`
- `memo`, `display_order`, `is_disabled`
- `entry_dt`, `update_dt`

---

### add_manufacturer

Adds a manufacturer.

**Parameters:**
- `user_id` (i64)
- `manufacturer_name` (String)
- `memo` (Option\<String\>)
- `is_disabled` (Option\<i64\>): Default 0

**Returns:**
- `String`: "Manufacturer added successfully"

---

### update_manufacturer

Updates a manufacturer.

**Parameters:**
- `user_id` (i64)
- `manufacturer_id` (i64)
- `manufacturer_name` (String)
- `memo` (Option\<String\>)
- `display_order` (i64)
- `is_disabled` (i64)

**Returns:**
- `String`: "Manufacturer updated successfully"

---

### delete_manufacturer

Logically deletes a manufacturer.

**Parameters:**
- `user_id` (i64)
- `manufacturer_id` (i64)

**Returns:**
- `String`: "Manufacturer deleted successfully"

**Note:**
- Related products are not deleted
- Product's `manufacturer_id` is retained, but `manufacturer_name` becomes `null` on retrieval

---

## Product Management API

### get_products

Retrieves product list (with manufacturer names).

**Parameters:**
- `user_id` (i64)
- `include_disabled` (bool)

**Returns:**
- `Vec<Product>`

**Product Fields:**
- `product_id`, `user_id`, `product_name`
- `manufacturer_id`, `manufacturer_name` (from LEFT JOIN)
- `memo`, `display_order`, `is_disabled`
- `entry_dt`, `update_dt`

**Note:**
- If manufacturer is hidden, `manufacturer_name` is `null`

---

### add_product

Adds a product.

**Parameters:**
- `user_id` (i64)
- `product_name` (String)
- `manufacturer_id` (Option\<i64\>)
- `memo` (Option\<String\>)
- `is_disabled` (Option\<i64\>): Default 0

**Returns:**
- `String`: "Product added successfully"

---

### update_product

Updates a product.

**Parameters:**
- `user_id` (i64)
- `product_id` (i64)
- `product_name` (String)
- `manufacturer_id` (Option\<i64\>)
- `memo` (Option\<String\>)
- `display_order` (i64)
- `is_disabled` (i64)

**Returns:**
- `String`: "Product updated successfully"

---

### delete_product

Logically deletes a product.

**Parameters:**
- `user_id` (i64)
- `product_id` (i64)

**Returns:**
- `String`: "Product deleted successfully"

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"... cannot be empty"` | Required field is empty | Enter valid value |
| `"... already exists"` | Duplicate data | Use different value |
| `"... not found"` | Data does not exist | Specify correct ID |
| `"Failed to ...: ..."` | Database error | Check connection/permissions |

### Frontend Error Handling

```javascript
try {
  const result = await invoke('command_name', params);
  // Success handling
} catch (error) {
  console.error('Error:', error);
  alert(`Error: ${error}`);
}
```

---

## Database Tables

| Table Name | Description | Primary Key |
|-----------|-------------|-------------|
| TRANSACTION_HEADERS | Transaction headers | TRANSACTION_ID |
| TRANSACTION_DETAILS | Transaction details | DETAIL_ID |
| TRANSACTION_MEMOS | Transaction memos | MEMO_ID |
| CATEGORIES1 | Category level 1 | CATEGORY1_CODE |
| CATEGORIES2 | Category level 2 | CATEGORY2_CODE |
| CATEGORIES3 | Category level 3 | CATEGORY3_CODE |
| SHOPS | Shops | SHOP_ID |
| MANUFACTURERS | Manufacturers | MANUFACTURER_ID |
| PRODUCTS | Products | PRODUCT_ID |
| ACCOUNTS | Accounts | ACCOUNT_CODE |

---

## Test Coverage

**Backend (Rust):**
- 201+ tests implemented
- Covers all major APIs

**Frontend (JavaScript):**
- Manual testing completed
- Automated tests to be implemented

---

## Related Documents

### Detailed Specifications (Legacy Docs)

The following individual API documents contain more detailed code examples and SQL statements:

- [Category Management API Details](./API_CATEGORY.md)
- [Transaction Management API Details](./API_TRANSACTION.md)
- [Aggregation API Details](./API_AGGREGATION.md)
- [Shop Management API Details](./API_SHOP.md)
- [Manufacturer Management API Details](./API_MANUFACTURER.md)
- [Product Management API Details](./API_PRODUCT.md)

### Implementation Guides

- [IS_DISABLED Implementation Guide](../guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md)
- [Database Configuration Guide](../guides/DATABASE_CONFIGURATION.md)

---

**Change History:**
- 2025-12-05: Initial version (consolidated 6 API documents)
