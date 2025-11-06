# Transaction Management Design Specification v2.0 (Header-Detail Structure)

## Overview

This document records the database design and UI/UX design for KakeiBot's transaction management feature.

**Created**: 2025-11-07 00:36 JST  
**Design Approach**: Header-Detail Structure (Normalized)  
**Status**: Design Complete, Pre-Implementation

---

## Design Philosophy

### Why Header-Detail Structure?

#### Background
- Shopping typically involves purchasing multiple items at once
- Users need to review "what was bought" later (especially homemakers)
- Track price history per product (to buy nutritious products cheaply)
- Receipt-based management is natural

#### Comparison with Flat Structure

| Aspect | Flat Structure | Header-Detail Structure |
|--------|---------------|------------------------|
| Simplicity | ⭕ Simple | ❌ Complex |
| Shopping Representation | ❌ Multiple records with same date/store | ⭕ 1 header, multiple details |
| Product History | ❌ Difficult | ⭕ Easy |
| Bulk Editing | ❌ Difficult | ⭕ Easy |
| Future Extensibility | ❌ Low | ⭕ High |

**Conclusion**: Prioritize usability and future potential by adopting header-detail structure

---

## Database Design

### 1. Account Master (ACCOUNTS)

```sql
CREATE TABLE ACCOUNTS (
    ACCOUNT_ID INTEGER PRIMARY KEY,
    USER_ID INTEGER NOT NULL,
    ACCOUNT_CODE VARCHAR(50) NOT NULL,      -- Unique within user
    ACCOUNT_NAME TEXT NOT NULL,             -- e.g., "Wallet", "Mitsubishi UFJ Bank"
    ACCOUNT_TYPE VARCHAR(20),               -- Cash/Bank/Credit etc. (optional)
    INITIAL_BALANCE INTEGER DEFAULT 0,      -- Balance at app start
    DISPLAY_ORDER INTEGER,
    IS_DISABLED INTEGER DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID),
    UNIQUE(USER_ID, ACCOUNT_CODE)
);

CREATE INDEX idx_accounts_user ON ACCOUNTS(USER_ID, ACCOUNT_CODE);
```

#### Design Points
- **Single Master Table**: FROM/TO accounts managed in one table, not separate
  - Less code
  - Higher versatility
  - Easier management
- **INITIAL_BALANCE**: Manually set by user (at account creation)
- **Logical Deletion**: IS_DISABLED=1 to disable (no physical deletion)

### 2. Header Table (TRANSACTION_HEADERS)

```sql
CREATE TABLE TRANSACTION_HEADERS (
    HEADER_ID INTEGER PRIMARY KEY,
    USER_ID INTEGER NOT NULL,
    TRANSACTION_DATE DATE NOT NULL,
    CATEGORY1_CODE VARCHAR(50) NOT NULL,    -- Major category (Expense/Income/Transfer)
    FROM_ACCOUNT_CODE VARCHAR(50),          -- Source account code
    TO_ACCOUNT_CODE VARCHAR(50),            -- Destination account code
    DESCRIPTION TEXT NOT NULL,              -- Store name or summary
    TOTAL_AMOUNT INTEGER NOT NULL,          -- Receipt total (user input)
    TAX_ROUNDING_TYPE VARCHAR(20),          -- Tax rounding method
    MEMO TEXT,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID),
    FOREIGN KEY (USER_ID, CATEGORY1_CODE) REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE),
    FOREIGN KEY (USER_ID, FROM_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY (USER_ID, TO_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE)
);

CREATE INDEX idx_headers_user_date ON TRANSACTION_HEADERS(USER_ID, TRANSACTION_DATE);
CREATE INDEX idx_headers_category1 ON TRANSACTION_HEADERS(USER_ID, CATEGORY1_CODE);
CREATE INDEX idx_headers_from_account ON TRANSACTION_HEADERS(USER_ID, FROM_ACCOUNT_CODE);
CREATE INDEX idx_headers_to_account ON TRANSACTION_HEADERS(USER_ID, TO_ACCOUNT_CODE);
```

#### Design Points

##### Major Category and Account Relationship

| Major Category | FROM_ACCOUNT | TO_ACCOUNT | Example |
|----------------|--------------|------------|---------|
| **Expense** | ✅ Required | ❌ NULL | Wallet → NULL (spent at store) |
| **Income** | ❌ NULL | ✅ Required | NULL → Bank account (salary) |
| **Transfer** | ✅ Required | ✅ Required | Bank A → Bank B (transfer) |

##### Why Major Category in Header?
1. **Consistency**: All details have same major category (data integrity)
2. **Simplification**: Detail category selection is only 2 levels (middle/minor)
3. **Account Control**: Display/requirement of account fields based on major category

##### TAX_ROUNDING_TYPE Values

| Value | Meaning |
|-------|---------|
| NULL | Detail total matches receipt total |
| 'ROUND_UP' | Matches with round-up pattern |
| 'ROUND_DOWN' | Matches with round-down pattern |
| 'ROUND_HALF' | Matches with half-round pattern |
| 'MANUAL' | Manual adjustment (saved despite mismatch) |

### 3. Detail Table (TRANSACTION_DETAILS)

```sql
CREATE TABLE TRANSACTION_DETAILS (
    DETAIL_ID INTEGER PRIMARY KEY,
    HEADER_ID INTEGER NOT NULL,
    CATEGORY2_CODE VARCHAR(50),             -- Middle category
    CATEGORY3_CODE VARCHAR(50),             -- Minor category
    AMOUNT INTEGER NOT NULL,                -- Base price (tax-excluded)
    TAX_RATE DECIMAL(5,2) DEFAULT 8.00,     -- Tax rate (%) default 8%
    TAX_AMOUNT INTEGER,                     -- Tax amount (auto-calculated, manually adjustable)
    ITEM_DESCRIPTION TEXT NOT NULL,         -- Product name
    ITEM_MEMO TEXT,
    DISPLAY_ORDER INTEGER,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY (HEADER_ID) REFERENCES TRANSACTION_HEADERS(HEADER_ID) ON DELETE CASCADE
);

CREATE INDEX idx_details_header ON TRANSACTION_DETAILS(HEADER_ID);
```

#### Design Points

##### Header-Detail Relationship
- **Required Constraint**: Header must have at least 1 detail
- **Cascade Delete**: Details auto-deleted when header is deleted
- **Category Hierarchy**: CATEGORY1 from header, CATEGORY2/3 from detail

##### Tax Handling

###### Background
In Japan, stores can freely choose tax rounding (round-up/down/half).
Therefore, receipt total may not match product base price × tax rate calculation.

###### Tax Calculation Flow

```
[At Detail Input]
1. User inputs base price (tax-excluded)
2. System auto-calculates tax: tax_amount = round(amount × tax_rate / 100)
3. User can manually adjust tax amount (for decimal precision issues)
4. Tax-included amount = amount + tax_amount (real-time display)

[At Header Save]
1. Sum all detail tax-included amounts: calculated_total = Σ(amount + tax_amount)
2. Compare with receipt total: calculated_total vs total_amount
3. Match determination:
   a) Match → OK (tax_rounding_type = NULL)
   b) Mismatch → Recalculate with 3 patterns
      - Round-up, round-down, half-round - any match?
      - Matched → Save to tax_rounding_type
      - No match → Show warning, allow user to save (tax_rounding_type = 'MANUAL')
```

###### Default Tax Rate
- **Default 8%**: Daily shopping centers on food items (reduced tax rate)
- **Adjustable**: Support for 10% items like daily necessities
- **Future Extension**: Consider adding tax rate field to category master in Phase 5

##### Amount Constraints
- **≥ 0 yen**: No negative, 0 yen OK
  - Reason: Allow unpredictable 0 yen data (e.g., transfer resulting in 0 balance)
  - Prevent operational deadlock

---

## UI/UX Design

### Screen Transition Flow

```
[Transaction List Screen]
  - Display header info only (1 row = 1 header)
  - Display: Date, Major Category, FROM/TO Account, Description, Total Amount
  
  ├─ [+ Add New] → ① Header Registration Modal
  └─ Header Row [Edit] or Double-click → ② Header Edit Modal

① [Header Registration Modal]
  - Date
  - Major Category (Expense/Income/Transfer) ← Dropdown changes account field display
  - Source Account (shown only for Expense or Transfer)
  - Destination Account (shown only for Income or Transfer)
  - Store/Description
  - Receipt Total Amount
  - Memo
  - [Next (Add Details)] → Save header, then go to ②
  - [Cancel]

② [Header Edit Modal]
  - Edit header info (same fields as above)
  - Detail list display (table format)
    | Middle | Minor | Product | Base | Tax Rate | Tax | Total | Actions |
  - Detail Total (tax-included): xxx yen (auto-calculated)
  - Receipt Total: xxx yen
  - Difference: ±xx yen (warning if difference exists)
  - [+ Add Detail] → ③ Detail Registration Modal
  - Detail Row [Edit] or Double-click → ③ Detail Edit Modal
  - Detail Row [Delete] → Confirm then delete (cannot delete last one)
  - [Save] / [Cancel]

③ [Detail Registration/Edit Modal] (Sub-modal)
  - Middle Category (dropdown) *Major category inherited from parent
  - Minor Category (dropdown) *Filtered after middle category selection
  - Product/Description
  - Base Price (tax-excluded) ← Main input
  - Tax Rate (%) Default 8.00%
  - Tax Amount Auto-calculated (gray display) ← Adjustable
  - Tax-included Amount (display only) ← Real-time calculation
  - Memo
  - [Save] → Return to ②
  - [Save and Add Another] → Save, clear form, continue input
  - [Cancel] → Return to ②
```

### List Screen Display Example

```
Transaction List
-------------------------------------------------------------
Date        | Category | Account      | Description        | Amount    | Actions
-------------------------------------------------------------
2025-11-06  | Expense  | Wallet →     | Shopping at Aeon   | 1,062 yen | [Edit][Delete]
2025-11-05  | Expense  | Wallet →     | Gas station        | 3,500 yen | [Edit][Delete]
2025-11-01  | Income   | → Bank       | November salary    | 250,000 yen| [Edit][Delete]
2025-10-30  | Transfer | Bank A→Bank B| Fund transfer      | 50,000 yen | [Edit][Delete]
```

### Header Edit Modal Display Example

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  Edit Transaction
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Date: [2025-11-06]
Category: [Expense ▼]
From: [Wallet ▼]
Store: [Shopping at Aeon_______]
Receipt Total: [1062] yen
Memo: [________________________]

Detail List
┌─────────────────────────────────────┐
│Middle │Minor  │Product  │Base│Rate│Tax│Total│Actions│
├─────────────────────────────────────┤
│Drinks │Dairy  │Milk 1L  │276│ 8% │ 22│ 298│[Ed][Del]│
│Staple │Bread  │Bread 6pc│119│ 8% │  9│ 128│[Ed][Del]│
│Food   │Eggs   │Eggs 10  │220│ 8% │ 17│ 237│[Ed][Del]│
│Daily  │Paper  │Tissue   │368│10% │ 36│ 404│[Ed][Del]│
└─────────────────────────────────────┘
                         Detail Total: 1,067 yen
                         Difference: +5 yen ⚠️

ℹ️ There is a difference between detail total and receipt total.
   You can adjust receipt total or save as is.

[+ Add Detail]            [Save] [Cancel]
```

---

## Validation Rules

| Item | Rule | Error Message |
|------|------|---------------|
| **Category & Account** | Expense→FROM required/TO=NULL | "For expenses, please select source account" |
| | Income→FROM=NULL/TO required | "For income, please select destination account" |
| | Transfer→Both required | "For transfers, please select both source and destination" |
| **Detail Count** | At least 1 required | "No details found. Please add at least one detail" |
| **Amount** | ≥0 integer | "Please enter an amount of 0 or greater" |
| **Tax Rate** | ≥0% | "Please enter a tax rate of 0 or greater" |
| **Total Match** | Receipt vs Detail Total | Warning (save allowed) |
| **Account Code** | Exists in ACCOUNTS | "Selected account not found" |
| **Last Detail Delete** | Cannot delete when only 1 detail | "Cannot delete the last detail" |

---

## Implementation Phases

### Phase 0: Account Master Management 🔴 **← Current**
- [ ] ACCOUNTS table creation SQL
- [ ] Account management backend API (Rust)
  - `add_account`
  - `get_accounts`
  - `update_account`
  - `delete_account` (logical deletion)
- [ ] Account management screen (HTML/JS/CSS)
- [ ] Auto-generation of default accounts
  - Auto-add "Cash", "Bank Account" etc. at user creation

### Phase 1: Table Creation and Migration
- [ ] TRANSACTION_HEADERS/DETAILS table creation SQL
- [ ] Migration script from existing TRANSACTIONS table
  - Migrate existing data as "header + 1 detail"

### Phase 2: Header Management
- [ ] Header registration modal (HTML/JS/CSS)
- [ ] Header edit modal (HTML/JS/CSS)
- [ ] Backend API (Rust)
  - `add_transaction_header`
  - `get_transaction_header`
  - `update_transaction_header`
  - `delete_transaction_header`

### Phase 3: Detail Management
- [ ] Detail registration/edit modal (sub-modal)
- [ ] Tax calculation logic (frontend + backend)
- [ ] Backend API (Rust)
  - `add_transaction_detail`
  - `get_transaction_details`
  - `update_transaction_detail`
  - `delete_transaction_detail`

### Phase 4: List Screen Update
- [ ] Support for header list display
- [ ] Filter feature update (add account filter)
- [ ] Aggregation features (by account, by category)

### Phase 5: Future Extensions (Later)
- [ ] Add tax rate field to category master
- [ ] Implement monthly balance table
- [ ] Auto-calculate and display account balances
- [ ] Graph display features

---

## Technical Considerations

### Transaction Control

Header and detail are inseparable, so always process within transaction:

```rust
// At header registration
let mut tx = pool.begin().await?;

// 1. Insert header
let header_id = insert_header(&mut tx, header_data).await?;

// 2. Insert details (at least 1 required)
for detail in details {
    insert_detail(&mut tx, header_id, detail).await?;
}

// 3. Commit
tx.commit().await?;
```

### Existing Data Migration

```sql
-- Migration example from existing TRANSACTIONS
INSERT INTO TRANSACTION_HEADERS (
    USER_ID, TRANSACTION_DATE, CATEGORY1_CODE, 
    FROM_ACCOUNT_CODE, DESCRIPTION, TOTAL_AMOUNT
)
SELECT 
    USER_ID, TRANSACTION_DATE, CATEGORY1_CODE,
    'DEFAULT_ACCOUNT', DESCRIPTION, AMOUNT
FROM TRANSACTIONS;

-- Create corresponding detail for each
INSERT INTO TRANSACTION_DETAILS (
    HEADER_ID, CATEGORY2_CODE, CATEGORY3_CODE,
    AMOUNT, TAX_AMOUNT, ITEM_DESCRIPTION
)
SELECT 
    h.HEADER_ID, t.CATEGORY2_CODE, t.CATEGORY3_CODE,
    t.AMOUNT, 0, t.DESCRIPTION
FROM TRANSACTIONS t
JOIN TRANSACTION_HEADERS h ON (appropriate JOIN condition);
```

---

## References

### Related Documents
- [Transaction Requirements](../TRANSACTION_REQUIREMENTS.md)
- [Transaction Management UI Implementation v1](./TRANSACTION_MANAGEMENT_UI.md)
- [Category Management API](./API_CATEGORY.md)

### Database Schema
- `res/sql/dbaccess.sql`

### Existing Implementation
- Flat structure implementation: `src/services/transaction.rs` (to be refactored in Phase 1)

---

**Created**: 2025-11-07 00:36 JST  
**Last Updated**: 2025-11-07 00:36 JST
