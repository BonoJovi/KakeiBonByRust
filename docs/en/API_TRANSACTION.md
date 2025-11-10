# Transaction Management API Documentation

## Overview

This document describes the backend API for transaction management in KakeiBon.
API exposure to the frontend follows the descriptions in each section below.

---

## API List

### Get Transaction Header

#### `get_transaction_header`
Retrieves a single transaction header including memo text.

**Parameters:**
- `transaction_id` (i64): Transaction ID

**Return Value:**
- `serde_json::Value`: Transaction header information (JSON)

**Response Structure:**
```javascript
{
  transaction_id: 123,
  user_id: 2,
  transaction_date: "2024-01-15 10:00:00",
  category1_code: "EXPENSE",
  from_account_code: "CASH",
  to_account_code: "NONE",
  total_amount: 1000,
  tax_rounding_type: 0,
  memo_id: 5,
  shop_id: 1,
  is_disabled: 0,
  entry_dt: "2024-01-15 10:00:00",
  update_dt: null,
  memo: "Groceries at supermarket"
}
```

**Usage Example:**
```javascript
const transaction = await invoke('get_transaction_header', {
  transactionId: 123
});
```

**Notes:**
- user_id is currently fixed at 2 (session management not yet implemented)
- When memo_id = NULL, no corresponding memo exists in the MEMOS table
- In the frontend transaction list, '-' (hyphen) is displayed for no memo
- transaction_date uses SQLite DATETIME format (YYYY-MM-DD HH:MM:SS)

---

#### `select_transaction_headers`
Retrieves multiple transaction headers (for future batch operations).

**Parameters:**
- `transaction_ids` (Vec<i64>): Array of transaction IDs

**Return Value:**
- `Vec<TransactionHeader>`: Array of transaction headers

**Usage Example:**
```javascript
const transactions = await invoke('select_transaction_headers', {
  transactionIds: [1, 2, 3]
});
```

**Notes:**
- Non-existent IDs are ignored (no error)
- Memo text is not included (memo_id only)
- For future batch edit feature implementation

---

### Save Transaction Header

#### `save_transaction_header`
Registers a new transaction header.

**Parameters:**
- `user_id` (i64): User ID
- `category1_code` (String): Category 1 code (e.g., "EXPENSE", "INCOME", "TRANSFER")
- `from_account_code` (String): Withdrawal account code (use "NONE" if not applicable)
- `to_account_code` (String): Deposit account code (use "NONE" if not applicable)
- `transaction_date` (String): Transaction date/time (YYYY-MM-DD HH:MM:SS format)
- `total_amount` (i64): Total amount
- `tax_rounding_type` (i64): Tax rounding type (0=round down, 1=round up, 2=round half up)
- `memo` (Option<String>): Memo (memo_id becomes NULL if null or empty string)
- `shop_id` (Option<i64>): Shop ID (optional)

**Return Value:**
- `Result<i64, String>`: New transaction_id on success, error message on failure

**Usage Example:**
```javascript
const transactionId = await invoke('save_transaction_header', {
  userId: 2,
  category1Code: 'EXPENSE',
  fromAccountCode: 'CASH',
  toAccountCode: 'NONE',
  transactionDate: '2024-01-15 10:00:00',
  totalAmount: 1000,
  taxRoundingType: 0,
  memo: 'Groceries at supermarket',
  shopId: 1
});
console.log('Created transaction ID:', transactionId);
```

**Validation:**
- Date format: YYYY-MM-DD HH:MM:SS (strict check)
- Amount range: 0 ≤ total_amount ≤ 999,999,999
- Tax rounding type: 0, 1, or 2
- account_code: "NONE" is a special account code meaning "not specified" (stored as string)

**Notes:**
- Reuses existing memo if found (same memo_id)
- Automatically adds to MEMOS table if new memo
- transaction_id is auto-incremented (AUTOINCREMENT)

---

### Update Transaction Header

#### `update_transaction_header`
Updates an existing transaction header.

**Parameters:**
- `transaction_id` (i64): Transaction ID to update
- `category1_code` (String): Category 1 code
- `from_account_code` (String): Withdrawal account code
- `to_account_code` (String): Deposit account code
- `transaction_date` (String): Transaction date/time (YYYY-MM-DD HH:MM:SS format)
- `total_amount` (i64): Total amount
- `tax_rounding_type` (i64): Tax rounding type
- `memo` (Option<String>): Memo
- `shop_id` (Option<i64>): Shop ID

**Return Value:**
- `Result<(), String>`: Empty on success, error message on failure

**Usage Example:**
```javascript
await invoke('update_transaction_header', {
  transactionId: 123,
  category1Code: 'EXPENSE',
  fromAccountCode: 'BANK',
  toAccountCode: 'NONE',
  transactionDate: '2024-01-15 11:00:00',
  totalAmount: 2000,
  taxRoundingType: 0,
  memo: 'Groceries at supermarket (amount corrected)',
  shopId: 2
});
```

**Validation:**
- Same validation rules as `save_transaction_header`
- Error if transaction_id does not exist

**Notes:**
- user_id is currently fixed at 2 (session management not yet implemented)
- UPDATE_DT field is automatically updated

---

## Memo Management Logic

### Overview
Transaction memos are designed to be shareable; identical memo content is reused.

### Memo States

#### 1. Empty Memo (No Memo)
```javascript
memo: null  // or empty string ""
```
- **Behavior**: memo_id = NULL
- **MEMOS Table**: No record created
- **Frontend Display**: '-' (hyphen) is displayed (meaning "no memo")
- **Use Case**: Transactions without memo

#### 2. Reusing Existing Memo
```javascript
memo: "Shopping"  // Already exists in MEMOS table
```
- **Behavior**: Searches and references existing memo_id
- **MEMOS Table**: No change
- **Frontend Display**: Memo text is displayed
- **Use Case**: Sharing same memo across multiple transactions

#### 3. New Memo
```javascript
memo: "New memo content"  // Does not exist in MEMOS table
```
- **Behavior**: Auto-increments new memo_id
- **MEMOS Table**: New record created
- **Frontend Display**: New memo text is displayed
- **Use Case**: First-time memo content

### Frontend Display

**Memo Display in Transaction List:**
```javascript
// res/js/transaction-management.js:323
if (transaction.memo) {
    memoDiv.textContent = transaction.memo;
} else {
    memoDiv.textContent = '-';  // Hyphen for no memo
}
```

### Memo Update Behavior

#### Case 1: Changing Shared Memo
```
Original memo: "Shopping" (memo_id=10)
In use: Transaction A, Transaction B → memo_id=10

Edit Transaction A, change memo to: "Groceries"
Result:
  - New memo_id=20 is created
  - Transaction A → memo_id=20
  - Transaction B → memo_id=10 (unchanged)
```
**Reason**: To avoid affecting other transactions

#### Case 2: Deleting Memo
```
Original memo: "Shopping" (memo_id=10)

Change memo to empty: memo = null
Result:
  - memo_id=NULL
  - MEMOS table: memo_id=10 remains (orphaned)
  - Frontend display: '-' (hyphen)
```
**Note**: Orphaned memo cleanup is under consideration for future feature

#### Case 3: Changing to Same Memo Content
```
Original memo: "Shopping" (memo_id=10)
Changed memo: "Groceries" (memo_id=15 already exists)

Result:
  - Does not create new, uses existing memo_id=15
```

### Database Structure

**MEMOS Table:**
```sql
CREATE TABLE MEMOS (
    MEMO_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    MEMO_TEXT TEXT NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME
);
```

**TRANSACTION_HEADERS Table (memo part):**
```sql
CREATE TABLE TRANSACTION_HEADERS (
    ...
    MEMO_ID INTEGER,  -- Reference to MEMOS table (nullable)
    ...
    FOREIGN KEY(MEMO_ID) REFERENCES MEMOS(MEMO_ID)
);
```

---

## Validation Details

### Date/Time Format
**Format**: `YYYY-MM-DD HH:MM:SS`

**Valid Examples:**
```
2024-01-15 10:00:00  ✅
2024-12-31 23:59:59  ✅
```

**Invalid Examples:**
```
2024-1-15 10:00:00   ❌ (single-digit month)
2024-01-15 10:00     ❌ (missing seconds)
2024/01/15 10:00:00  ❌ (wrong delimiter)
```

**Implementation:**
```rust
// Regex check (src/services/transaction.rs:713-717)
if !RE_DATETIME.is_match(&request.transaction_date) {
    return Err(TransactionError::ValidationError(
        "Invalid date format. Use YYYY-MM-DD HH:MM:SS".to_string(),
    ));
}
```

### Amount Range
**Range**: 0 ≤ total_amount ≤ 999,999,999

**Valid Examples:**
```
0           ✅ (zero allowed)
1           ✅
1000000     ✅
999999999   ✅ (maximum)
```

**Invalid Examples:**
```
-1          ❌ (negative not allowed)
1000000000  ❌ (1 billion or more not allowed)
```

**Implementation:**
```rust
// Amount check (src/services/transaction.rs:719-723)
if request.total_amount < 0 || request.total_amount > 999_999_999 {
    return Err(TransactionError::ValidationError(
        "Amount must be between 0 and 999,999,999".to_string(),
    ));
}
```

### Tax Rounding Type
**Valid Values**: 0, 1, 2

| Value | Description | Constant Name |
|-------|-------------|---------------|
| 0     | Round down  | TAX_ROUND_DOWN |
| 1     | Round up    | TAX_ROUND_UP |
| 2     | Round half up | TAX_ROUND_HALF_UP |

**Implementation:**
```rust
// Tax rounding type check (src/services/transaction.rs:725-731)
if request.tax_rounding_type != consts::TAX_ROUND_DOWN
    && request.tax_rounding_type != consts::TAX_ROUND_UP
    && request.tax_rounding_type != consts::TAX_ROUND_HALF_UP
{
    return Err(TransactionError::ValidationError(
        "Invalid tax rounding type".to_string(),
    ));
}
```

---

## Error Handling

### Error Types

```rust
pub enum TransactionError {
    DatabaseError(String),     // Database error
    ValidationError(String),   // Validation error
    NotFound,                  // Transaction not found
}
```

### Common Error Cases

#### 1. Validation Error
**Cause:**
- Invalid date format
- Amount out of range
- Invalid tax rounding type

**Example Messages:**
```
"Validation error: Invalid date format. Use YYYY-MM-DD HH:MM:SS"
"Validation error: Amount must be between 0 and 999,999,999"
"Validation error: Invalid tax rounding type"
```

#### 2. Transaction Not Found
**Cause:**
- Specified non-existent transaction_id

**Message:**
```
"Transaction not found"
```

#### 3. Database Error
**Cause:**
- Foreign key constraint violation
- SQL execution error
- Connection error

**Example Messages:**
```
"Database error: FOREIGN KEY constraint failed"
"Database error: no such table: TRANSACTION_HEADERS"
```

### Frontend Error Handling Example

```javascript
try {
  await invoke('update_transaction_header', {
    transactionId: 123,
    // ... parameters
  });
  alert('Transaction updated');
  await loadTransactions(); // Reload list
} catch (error) {
  console.error('Update failed:', error);

  if (error.includes('not found')) {
    alert('Transaction not found. It may have been deleted.');
  } else if (error.includes('Validation error')) {
    alert('Invalid input: ' + error);
  } else {
    alert('Failed to update transaction: ' + error);
  }
}
```

---

## Database Schema

### TRANSACTION_HEADERS Table
```sql
CREATE TABLE TRANSACTION_HEADERS (
    TRANSACTION_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    TRANSACTION_DATE DATETIME NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    FROM_ACCOUNT_CODE VARCHAR(64),
    TO_ACCOUNT_CODE VARCHAR(64),
    TOTAL_AMOUNT INTEGER NOT NULL,
    TAX_ROUNDING_TYPE INTEGER NOT NULL,
    MEMO_ID INTEGER,
    SHOP_ID INTEGER,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY(USER_ID) REFERENCES USERS(USER_ID),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE)
        REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE),
    FOREIGN KEY(USER_ID, FROM_ACCOUNT_CODE)
        REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY(USER_ID, TO_ACCOUNT_CODE)
        REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY(MEMO_ID) REFERENCES MEMOS(MEMO_ID),
    FOREIGN KEY(SHOP_ID) REFERENCES SHOPS(SHOP_ID)
);
```

### MEMOS Table
```sql
CREATE TABLE MEMOS (
    MEMO_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    MEMO_TEXT TEXT NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME
);
```

### TRANSACTION_DETAILS Table
```sql
CREATE TABLE TRANSACTION_DETAILS (
    DETAIL_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    TRANSACTION_ID INTEGER NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    ITEM_NAME VARCHAR(256) NOT NULL,
    AMOUNT INTEGER NOT NULL,
    TAX_AMOUNT INTEGER NOT NULL,
    TAX_RATE INTEGER NOT NULL,
    MEMO_ID INTEGER,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY(TRANSACTION_ID)
        REFERENCES TRANSACTION_HEADERS(TRANSACTION_ID)
        ON DELETE CASCADE,
    FOREIGN KEY(MEMO_ID) REFERENCES MEMOS(MEMO_ID)
);
```

**Notes:**
- Foreign key constraints to CATEGORY2/3 are currently not set
- Reason: CATEGORY2/3 primary keys are composite keys `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE)` and `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE)`, but TRANSACTION_DETAILS table does not have USER_ID and CATEGORY1_CODE columns
- USER_ID and CATEGORY1_CODE are designed to be obtained from TRANSACTION_HEADERS
- Referential integrity is currently enforced at the application level (planned to be implemented as database constraints in the future)

**Note:**
- TRANSACTION_DETAILS planned for future implementation (detail feature)
- Currently header-only transaction management

---

## Data Structures

### TransactionHeader
```rust
pub struct TransactionHeader {
    pub transaction_id: i64,
    pub user_id: i64,
    pub transaction_date: String,      // YYYY-MM-DD HH:MM:SS
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub total_amount: i64,
    pub tax_rounding_type: i64,
    pub memo_id: Option<i64>,
    pub shop_id: Option<i64>,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

### SaveTransactionRequest
```rust
pub struct SaveTransactionRequest {
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub transaction_date: String,
    pub total_amount: i64,
    pub tax_rounding_type: i64,
    pub memo: Option<String>,
    pub shop_id: Option<i64>,
}
```

---

## Design Notes

### Account Codes
- **NONE**: Special account code specified when account is not used
- **Database Storage**: "NONE" is stored as-is as a string (not converted to NULL)
- **ACCOUNTS Table**: Record with ACCOUNT_CODE='NONE', ACCOUNT_NAME='Not specified' exists
- **Data Type**: from_account_code, to_account_code are String type
- **Category-Specific Usage**:
  - EXPENSE: FROM_ACCOUNT_CODE only
  - INCOME: TO_ACCOUNT_CODE only
  - TRANSFER: Both used

### Tax Rounding Type
- **Current**: Maintained at header level
- **Future**: For tax calculation at detail level
- **Default Value**: 0 (round down)

### Session Management
- **Current**: user_id is fixed (2)
- **Future**: Dynamic retrieval after session/auth implementation
- **TODO**: Comments in code `// TODO: Get user_id from session/auth`

### Orphaned Memo Records
- **Current**: Deletion feature not implemented
- **Future**: Periodic cleanup process under consideration
- **Reason**: Careful deletion needed to maintain memo sharing feature

---

## Implemented Features

- ✅ Create new transaction header
- ✅ Get transaction header (single)
- ✅ Get transaction headers (multiple)
- ✅ Update transaction header
- ✅ Delete transaction header
- ✅ Automatic memo management (new/reuse/NULL)
- ✅ Validation (date/amount/tax rounding type)
- ✅ Filtering feature (list screen)
- ✅ Paging feature (list screen)

---

## Planned Features

- [ ] Transaction detail (TRANSACTION_DETAILS) management
- [ ] Dynamic user_id retrieval via session management
- [ ] Orphaned memo record cleanup
- [ ] Batch edit feature
- [ ] Soft delete (using IS_DISABLED)
- [ ] Automatic tax calculation (per detail)
- [ ] Integration with account balance management

---

## Testing

### Backend Tests (Rust)
```bash
# Transaction service tests
cargo test services::transaction::tests --lib
```

### Frontend Tests (JavaScript)
```bash
cd res/tests

# Transaction edit feature tests
npm test -- transaction-edit.test.js
```

**Test Results:**
- Backend: 121 tests passing
- Frontend: 404 tests passing (transaction edit: 98 tests)
- Total Tests: 525 tests
- Success Rate: 100%

---

## Version History

- **v0.2** (2025-11-09): Edit feature added
  - update_transaction_header API implementation
  - Memo management logic during edit
  - Test cases added (98 tests)

- **v0.1** (2025-11-08): Initial API documentation
  - save_transaction_header API
  - get_transaction_header API
  - select_transaction_headers API
  - Automatic memo management feature

---

## Related Documentation

- [Test Summary](../ja/TEST_SUMMARY.md)
- [Category Management API](API_CATEGORY.md)
- [Shop Management API](API_SHOP.md)
- [Japanese Version](../ja/API_TRANSACTION.md)

---

**Last Updated**: 2025-11-10 JST
**Version**: v0.3 (Shop ID added)
