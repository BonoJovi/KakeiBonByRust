# Transaction Management API Reference

**Last Updated**: 2025-12-05 02:30 JST

## Overview

This document defines APIs used in the transaction management screens (transaction-management.html, transaction-detail-management.html).

---

## Table of Contents

1. [Basic Transaction Operations API](#basic-transaction-operations-api)
2. [Transaction Header Management API](#transaction-header-management-api)
3. [Transaction Detail Management API](#transaction-detail-management-api)
4. [Data Structures](#data-structures)

---

## Basic Transaction Operations API

### add_transaction

Adds a simple transaction (simplified version).

**Parameters:**
- `transaction_date` (String): Transaction date and time ("YYYY-MM-DD HH:MM:SS")
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code
- `category3_code` (String): Category3 code
- `amount` (i64): Amount
- `description` (Option<String>): Description
- `memo` (Option<String>): Memo

**Return Value:**
- `i64`: Created transaction_id

**Usage Example:**
```javascript
const transactionId = await invoke('add_transaction', {
    transactionDate: '2025-12-05 10:30:00',
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    amount: 1500,
    description: 'Groceries',
    memo: 'Purchased at supermarket'
});
```

**Note:**
- Session user ID automatically retrieved
- For simple input (without details)

---

### get_transaction

Retrieves transaction information.

**Parameters:**
- `transaction_id` (i64): Transaction ID

**Return Value:**
- `Transaction`: Transaction information

**Usage Example:**
```javascript
const transaction = await invoke('get_transaction', {
    transactionId: 123
});
```

---

### get_transactions

Retrieves transaction list with filters (pagination supported).

**Parameters:**
- `start_date` (Option<String>): Start date ("YYYY-MM-DD")
- `end_date` (Option<String>): End date ("YYYY-MM-DD")
- `category1_code` (Option<String>): Category1 filter
- `category2_code` (Option<String>): Category2 filter
- `category3_code` (Option<String>): Category3 filter
- `min_amount` (Option<i64>): Minimum amount
- `max_amount` (Option<i64>): Maximum amount
- `keyword` (Option<String>): Keyword search
- `page` (i64): Page number (starts from 1)
- `per_page` (i64): Items per page

**Return Value:**
- `Vec<Transaction>`: Array of transactions

**Usage Example:**
```javascript
const transactions = await invoke('get_transactions', {
    startDate: '2025-12-01',
    endDate: '2025-12-31',
    category1Code: 'EXPENSE',
    category2Code: null,
    category3Code: null,
    minAmount: null,
    maxAmount: null,
    keyword: null,
    page: 1,
    perPage: 50
});
```

**Pagination:**
- `page`: Starts from 1
- `per_page`: Recommended range 1-100

---

### update_transaction

Updates a transaction.

**Parameters:**
- `transaction_id` (i64): Transaction ID
- `transaction_date` (String): Transaction date and time
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code
- `category3_code` (String): Category3 code
- `amount` (i64): Amount
- `description` (Option<String>): Description
- `memo` (Option<String>): Memo

**Return Value:** None

**Usage Example:**
```javascript
await invoke('update_transaction', {
    transactionId: 123,
    transactionDate: '2025-12-05 11:00:00',
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    amount: 2000,
    description: 'Groceries (Updated)',
    memo: null
});
```

---

### delete_transaction

Deletes a transaction.

**Parameters:**
- `transaction_id` (i64): Transaction ID

**Return Value:** None

**Usage Example:**
```javascript
if (confirm('Are you sure you want to delete?')) {
    await invoke('delete_transaction', { transactionId: 123 });
}
```

**Note:**
- Logical deletion (is_disabled = 1)
- Related details are also deleted

---

## Transaction Header Management API

### save_transaction_header

Saves a new transaction header (for detail management).

**Parameters:**
- `shop_id` (Option<i64>): Shop ID
- `category1_code` (String): Category1 code
- `from_account_code` (String): Source account code
- `to_account_code` (String): Destination account code
- `transaction_date` (String): Transaction date and time
- `total_amount` (i64): Total amount
- `tax_rounding_type` (i64): Tax rounding type (0=floor, 1=ceiling, 2=round)
- `tax_included_type` (i64): Tax inclusion type (0=exclusive, 1=inclusive)
- `memo` (Option<String>): Memo

**Return Value:**
- `i64`: Created transaction_id

**Usage Example:**
```javascript
const transactionId = await invoke('save_transaction_header', {
    shopId: 1,
    category1Code: 'EXPENSE',
    fromAccountCode: 'CASH',
    toAccountCode: 'NONE',
    transactionDate: '2025-12-05 10:00:00',
    totalAmount: 5000,
    taxRoundingType: 0,
    taxIncludedType: 1,
    memo: 'Shopping at supermarket'
});
```

**Note:**
- Details added separately via `add_transaction_detail`
- Memo is encrypted

---

### get_transaction_header

Retrieves transaction header and memo.

**Parameters:**
- `transaction_id` (i64): Transaction ID

**Return Value:**
- `JSON`: Header information and memo

**Response Structure:**
```javascript
{
    transaction_id: 123,
    user_id: 2,
    shop_id: 1,
    transaction_date: "2025-12-05 10:00:00",
    category1_code: "EXPENSE",
    from_account_code: "CASH",
    to_account_code: "NONE",
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo_id: 5,
    is_disabled: 0,
    entry_dt: "2025-12-05 10:00:00",
    update_dt: null,
    memo: "Shopping at supermarket"
}
```

**Usage Example:**
```javascript
const header = await invoke('get_transaction_header', {
    transactionId: 123
});
console.log(header.memo);
```

---

### get_transaction_header_with_info

Retrieves transaction header with related information (shop name, etc.).

**Parameters:**
- `transaction_id` (i64): Transaction ID

**Return Value:**
- `TransactionHeaderWithInfo`: Extended header information

**TransactionHeaderWithInfo Structure:**
```javascript
{
    transaction_id: 123,
    user_id: 2,
    shop_id: 1,
    shop_name: "AEON Shinjuku",  // Additional info
    transaction_date: "2025-12-05 10:00:00",
    category1_code: "EXPENSE",
    from_account_code: "CASH",
    from_account_name: "Cash",  // Additional info
    to_account_code: "NONE",
    to_account_name: "-",  // Additional info
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: "Shopping at supermarket",
    is_disabled: 0
}
```

**Usage Example:**
```javascript
const header = await invoke('get_transaction_header_with_info', {
    transactionId: 123
});
console.log(`Shop: ${header.shop_name}`);
```

**Purpose:**
- For screen display (includes shop name, account names)

---

### select_transaction_headers

Retrieves multiple transaction headers in bulk.

**Parameters:**
- `transaction_ids` (Vec<i64>): Array of transaction IDs

**Return Value:**
- `Vec<TransactionHeader>`: Array of headers

**Usage Example:**
```javascript
const headers = await invoke('select_transaction_headers', {
    transactionIds: [1, 2, 3, 5]
});
```

**Note:**
- Non-existent IDs are skipped (no error)
- For future bulk operations

---

### update_transaction_header

Updates a transaction header.

**Parameters:**
- `transaction_id` (i64): Transaction ID
- Other parameters same as `save_transaction_header`

**Return Value:** None

**Usage Example:**
```javascript
await invoke('update_transaction_header', {
    transactionId: 123,
    shopId: 2,
    category1Code: 'EXPENSE',
    fromAccountCode: 'CASH',
    toAccountCode: 'NONE',
    transactionDate: '2025-12-05 11:00:00',
    totalAmount: 6000,
    taxRoundingType: 0,
    taxIncludedType: 1,
    memo: 'Updated memo'
});
```

---

## Transaction Detail Management API

### get_transaction_details

Retrieves transaction detail list.

**Parameters:**
- `transaction_id` (i64): Transaction ID

**Return Value:**
- `Vec<TransactionDetailWithInfo>`: Array of details (with category names, etc.)

**TransactionDetailWithInfo Structure:**
```javascript
{
    detail_id: 1,
    transaction_id: 123,
    line_number: 1,
    category1_code: "EXPENSE",
    category1_name: "Expense",
    category2_code: "C2_E_1",
    category2_name: "Food",
    category3_code: "C3_1",
    category3_name: "Groceries",
    item_name: "Vegetables",
    quantity: 1,
    amount: 500,
    tax_rate: 10,
    tax_amount: 50,
    amount_including_tax: 550,
    product_id: null,
    manufacturer_id: null,
    memo: null
}
```

**Usage Example:**
```javascript
const details = await invoke('get_transaction_details', {
    transactionId: 123
});
details.forEach(detail => {
    console.log(`${detail.item_name}: ¥${detail.amount_including_tax}`);
});
```

---

### add_transaction_detail

Adds a transaction detail.

**Parameters:**
- `transaction_id` (i64): Transaction ID
- `category1_code` (String): Category1 code
- `category2_code` (String): Category2 code
- `category3_code` (String): Category3 code
- `item_name` (String): Item name
- `amount` (i64): Amount (tax-exclusive)
- `tax_rate` (i32): Tax rate (%)
- `tax_amount` (i64): Tax amount
- `amount_including_tax` (Option<i64>): Amount including tax
- `memo` (Option<String>): Memo
- `quantity` (Option<i64>): Quantity
- `product_id` (Option<i64>): Product ID
- `manufacturer_id` (Option<i64>): Manufacturer ID

**Return Value:**
- `i64`: Created detail_id

**Usage Example:**
```javascript
const detailId = await invoke('add_transaction_detail', {
    transactionId: 123,
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    itemName: 'Vegetables',
    amount: 500,
    taxRate: 10,
    taxAmount: 50,
    amountIncludingTax: 550,
    memo: null,
    quantity: 1,
    productId: null,
    manufacturerId: null
});
```

**Automatic Processing:**
- Auto-numbering of `line_number`

---

### update_transaction_detail

Updates a transaction detail.

**Parameters:**
- `detail_id` (i64): Detail ID
- Other parameters same as `add_transaction_detail` (except transaction_id)

**Return Value:** None

**Usage Example:**
```javascript
await invoke('update_transaction_detail', {
    detailId: 1,
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    itemName: 'Vegetables (Updated)',
    amount: 600,
    taxRate: 10,
    taxAmount: 60,
    amountIncludingTax: 660,
    memo: null,
    quantity: 1,
    productId: null,
    manufacturerId: null
});
```

---

### delete_transaction_detail

Deletes a transaction detail.

**Parameters:**
- `detail_id` (i64): Detail ID

**Return Value:** None

**Usage Example:**
```javascript
await invoke('delete_transaction_detail', { detailId: 1 });
```

---

## Data Structures

### Transaction (Simple Version)

```rust
pub struct Transaction {
    pub transaction_id: i64,
    pub user_id: i64,
    pub transaction_date: String,
    pub category1_code: String,
    pub category2_code: String,
    pub category3_code: String,
    pub amount: i64,
    pub description: Option<String>,
    pub memo: Option<String>,
}
```

---

### TransactionHeader

```rust
pub struct TransactionHeader {
    pub transaction_id: i64,
    pub user_id: i64,
    pub shop_id: Option<i64>,
    pub transaction_date: String,
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub total_amount: i64,
    pub tax_rounding_type: i64,  // 0=floor, 1=ceiling, 2=round
    pub tax_included_type: i64,  // 0=exclusive, 1=inclusive
    pub memo_id: Option<i64>,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

---

### TransactionDetail

```rust
pub struct TransactionDetail {
    pub detail_id: i64,
    pub transaction_id: i64,
    pub line_number: i64,
    pub category1_code: String,
    pub category2_code: String,
    pub category3_code: String,
    pub item_name: String,
    pub quantity: i64,
    pub amount: i64,           // Tax-exclusive
    pub tax_rate: i32,         // %
    pub tax_amount: i64,
    pub amount_including_tax: Option<i64>,  // Tax-inclusive
    pub product_id: Option<i64>,
    pub manufacturer_id: Option<i64>,
    pub memo: Option<String>,
}
```

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"User not authenticated"` | Session not authenticated | Login required |
| `"Failed to get transaction: ..."` | Retrieval error | Check database |
| `"Failed to delete transaction: ..."` | Deletion error | Check database |

### Frontend Error Handling Example

```javascript
// Add transaction
async function addTransaction(data) {
    try {
        const transactionId = await invoke('add_transaction', data);
        alert(`Transaction added (ID: ${transactionId})`);
        return transactionId;
    } catch (error) {
        alert(`Error: ${error}`);
        return null;
    }
}

// Save transaction with details
async function saveTransactionWithDetails(header, details) {
    try {
        // Save header
        const transactionId = await invoke('save_transaction_header', header);
        
        // Add details sequentially
        for (const detail of details) {
            await invoke('add_transaction_detail', {
                transactionId,
                ...detail
            });
        }
        
        alert('Saved');
        return transactionId;
    } catch (error) {
        alert(`Save error: ${error}`);
        return null;
    }
}
```

---

## Usage Scenarios

### Scenario 1: Simple Input (Without Details)

```javascript
// Record food expense of ¥1,500
await invoke('add_transaction', {
    transactionDate: '2025-12-05 10:30:00',
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',  // Food
    category3Code: 'C3_1',    // Groceries
    amount: 1500,
    description: 'Lunch',
    memo: null
});
```

### Scenario 2: Input with Details

```javascript
// 1. Create header
const transactionId = await invoke('save_transaction_header', {
    shopId: 1,
    category1Code: 'EXPENSE',
    fromAccountCode: 'CASH',
    toAccountCode: 'NONE',
    transactionDate: '2025-12-05 14:00:00',
    totalAmount: 3250,
    taxRoundingType: 0,
    taxIncludedType: 1,
    memo: 'Shopping at supermarket'
});

// 2. Add detail (vegetables)
await invoke('add_transaction_detail', {
    transactionId,
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    itemName: 'Vegetables',
    amount: 1500,
    taxRate: 8,
    taxAmount: 120,
    amountIncludingTax: 1620,
    memo: null,
    quantity: 1,
    productId: null,
    manufacturerId: null
});

// 3. Add detail (meat)
await invoke('add_transaction_detail', {
    transactionId,
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    itemName: 'Meat',
    amount: 1500,
    taxRate: 8,
    taxAmount: 120,
    amountIncludingTax: 1620,
    memo: null,
    quantity: 1,
    productId: null,
    manufacturerId: null
});
```

---

## Test Coverage

**TransactionService:**
- ✅ Transaction addition test
- ✅ Transaction retrieval test
- ✅ Transaction update test
- ✅ Transaction deletion test
- ✅ Header and detail integration test
- ✅ Memo encryption/decryption test
- ✅ Pagination test

---

## Related Documents

### Implementation Files

- Transaction Service: `src/services/transaction.rs`
- SQL Definitions: `src/sql_queries.rs`
- Tauri Commands: `src/lib.rs`

### Other API References

- [Common API](./API_COMMON.md) - Session management
- [Category Management API](./API_CATEGORY.md) - Category information
- [Account Management API](./API_ACCOUNT.md) - Account information
- [Master Data API](./API_MASTER_DATA.md) - Shop information

---

**Change History:**
- 2024-11-10: Initial version
- 2025-12-05: Complete revision based on implementation code
  - Added get_transaction_header_with_info
  - Unified parameter names to camelCase
  - Unified with new template
  - Added usage scenarios
