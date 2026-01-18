# Transaction Management Screen Implementation Document (V2)

## Overview

This document records the latest implementation of KakeiBon's transaction management functionality.

**Implementation Period**: 2025-11-05 ~ 2025-11-08
**Last Updated**: 2025-11-12 22:30 JST
**Implementation Status**: Phase 1-3 Complete (New schema support, list display, registration, i18n), Detail table normalization complete

---

## Architecture Changes

### Schema Changes (V2 Design)

Transitioned from single-table structure to header-detail separation structure.

#### TRANSACTIONS_HEADER (Transaction Header)
```sql
CREATE TABLE TRANSACTIONS_HEADER (
    TRANSACTION_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    TRANSACTION_DATE DATETIME NOT NULL,  -- DATE → DATETIME (supports hour:minute)
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    FROM_ACCOUNT_CODE VARCHAR(50) NOT NULL,
    TO_ACCOUNT_CODE VARCHAR(50) NOT NULL,
    TOTAL_AMOUNT INTEGER NOT NULL,
    TAX_ROUNDING_TYPE INTEGER NOT NULL,  -- 0:Round down, 1:Round half, 2:Round up
    MEMO_ID INTEGER,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY(USER_ID) REFERENCES USERS(USER_ID),
    FOREIGN KEY(USER_ID, FROM_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY(USER_ID, TO_ACCOUNT_CODE) REFERENCES ACCOUNTS(USER_ID, ACCOUNT_CODE),
    FOREIGN KEY(MEMO_ID) REFERENCES MEMOS(MEMO_ID)
);
```

#### TRANSACTIONS_DETAIL (Transaction Detail)
```sql
CREATE TABLE TRANSACTIONS_DETAIL (
    DETAIL_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    TRANSACTION_ID INTEGER NOT NULL,
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(50) NOT NULL,
    CATEGORY2_CODE VARCHAR(50) NOT NULL,
    CATEGORY3_CODE VARCHAR(50) NOT NULL,
    ITEM_NAME TEXT NOT NULL,
    AMOUNT INTEGER NOT NULL,
    TAX_AMOUNT INTEGER DEFAULT 0,
    TAX_RATE INTEGER DEFAULT 8,
    MEMO_ID INTEGER,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT DATETIME,
    FOREIGN KEY (TRANSACTION_ID) REFERENCES TRANSACTIONS_HEADER(TRANSACTION_ID) ON DELETE CASCADE,
    FOREIGN KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE),
    FOREIGN KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE) REFERENCES CATEGORY3(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE),
    FOREIGN KEY (MEMO_ID) REFERENCES MEMOS(MEMO_ID),
    CHECK (ITEM_NAME != '')
);
```

### Key Changes

1. **Date Type Change**: DATE → DATETIME (supports hour:minute input)
2. **Tax Rate Migration**: HEADER → DETAIL (supports mixed tax rates)
3. **Account Information**: Added FROM_ACCOUNT_CODE / TO_ACCOUNT_CODE
4. **Memo Separation**: MEMO_ID reference to external MEMOS table
5. **Detail Table Normalization** (Added 2025-11-12):
   - Added USER_ID and CATEGORY1_CODE fields
   - Set composite foreign key constraints to CATEGORY2/CATEGORY3
   - Improved data integrity and user-specific category management

---

## Implemented Features

### Phase 1: Database & Backend API ✅

#### Backend API (Rust)
Implementation Location: `src/services/transaction.rs`

| API | Description | Status |
|-----|-------------|--------|
| `save_transaction_header` | Save header | ✅ Implemented |
| `get_transactions` | List retrieval | ✅ Implemented |
| `delete_transaction` | Delete | ✅ Implemented |

**Note**: Detail (DETAIL) management API not implemented

#### Data Validation
- DateTime format: `YYYY-MM-DD HH:MM:SS` (19 characters)
- Amount: Required, integer value
- Tax rounding: Range 0-2

---

### Phase 2: Transaction Registration ✅

#### Registration Modal

**Input Fields**:
- Transaction date/time (datetime-local input)
- Category (Major) - SELECT box
- From account - SELECT box
- To account - SELECT box
- Total amount
- Tax rounding (Round down/Round half/Round up)
- Memo (textarea)
- Manage details button (not implemented)

**Operations**:
1. Category selection: Dynamically loaded from CATEGORY1 table
2. Account selection: Shows only user's accounts (filtered by currentUserId)
3. DateTime default: OS local time, time set to 00:00
4. Validation: Required field checks
5. Save: Calls `save_transaction_header` API

**Limitations**:
- Detail functionality not implemented (header only)
- Edit functionality not implemented

---

### Phase 3: List Display ✅

#### Display Items
- Transaction date/time (YYYY-MM-DD HH:MM format)
- Category name (CATEGORY1_NAME)
- Account info (FROM_ACCOUNT_NAME → TO_ACCOUNT_NAME)
- Total amount (color-coded by category)
  - Expense: Red
  - Income: Green
  - Transfer: Blue
- Memo (max 20 chars, "..." for overflow, full text on hover)
- Action buttons (Edit✏️, Delete[Trash]️)

#### Data Retrieval
Multiple table JOINs:
```sql
SELECT 
    t.*, 
    c1.CATEGORY1_NAME,
    a1.ACCOUNT_NAME as FROM_ACCOUNT_NAME,
    a2.ACCOUNT_NAME as TO_ACCOUNT_NAME,
    m.MEMO_TEXT
FROM TRANSACTIONS_HEADER t
LEFT JOIN CATEGORY1 c1 ON ...
LEFT JOIN ACCOUNTS a1 ON ...
LEFT JOIN ACCOUNTS a2 ON ...
LEFT JOIN MEMOS m ON ...
```

#### Pagination
- 50 items per page
- Total count display
- Previous/Next page buttons

---

### Phase 4: Filter Functionality ✅

#### Filter Options
- Date range: Start date ~ End date
- Category: Category1/2/3 (cascading SELECT boxes)
- Amount range: Min ~ Max
- Keyword: Memo search

**Note**: 
- Category2/3, keyword search currently disabled (detail table not supported)
- Implemented filters: Date range, Category1, Amount range

---

### Phase 5: Internationalization ✅

#### Translation Resources (34 entries)

**Buttons & Labels**:
- `transaction_mgmt.add_new`: Add New Transaction / 新規入出金追加
- `transaction_mgmt.filter`: Filter / フィルタ
- `transaction_mgmt.total`: Total / 合計
- `transaction_mgmt.items`: items / 件
- `transaction_mgmt.page`: Page / ページ

**Modal**:
- `transaction_mgmt.select_category`: - Select category - / - 費目を選択 -
- `transaction_mgmt.manage_details`: Manage Details / 明細管理
- `transaction_mgmt.delete_confirm`: Delete confirmation message

**Filter**:
- `transaction_mgmt.filter_options`: Filter Options / フィルタオプション
- `transaction_mgmt.clear_filter`: Clear / クリア
- `transaction_mgmt.apply_filter`: Apply / 適用
- `transaction_mgmt.min_placeholder`: Min / 最小
- `transaction_mgmt.max_placeholder`: Max / 最大
- `transaction_mgmt.search_placeholder`: Search in memo / メモを検索

**Common**:
- `common.all`: All / すべて
- `common.unspecified`: Unspecified / 指定なし

#### Bug Fix
Fixed issue where dynamically updating SELECT boxes with `innerHTML` was overwriting 
default options with `data-i18n` attributes. Changed to preserve default options.

---

## File Structure

### Frontend
- `res/transaction-management.html`: Screen HTML
- `res/js/transaction-management.js`: Business logic
- `res/css/transaction-management.css`: Style definitions

### Backend
- `src/services/transaction.rs`: Transaction logic
- `src/sql_queries.rs`: SQL query definitions
- `src/lib.rs`: Tauri command registration

### Database
- `sql/create_transactions_header_table.sql`: Header table creation
- `sql/create_transactions_detail_table.sql`: Detail table creation
- `sql/migrate_transaction_date_to_datetime.sql`: Date type migration

---

## Test Status

### Verified ✅
1. ✅ Transaction registration (header only)
2. ✅ List display (3 records registered)
3. ✅ Account name & category name display
4. ✅ Memo display (20 char limit, tooltip)
5. ✅ Delete functionality
6. ✅ Language switching (Japanese/English)
7. ✅ User filtering (verified with USER_ID=2)

### Untested Items
- Pagination (data exceeding 50 items)
- Filter functionality (date range, category1, amount range)
- Responsive design (narrow screens)

---

## Known Issues & Limitations

### Temporary Implementations
1. **Session Management Not Implemented**: 
   - `currentUserId`, `currentUserRole` managed as constants
   - Manual changes required for testing

2. **Detail Functionality Not Implemented**:
   - TRANSACTIONS_DETAIL table CRUD not implemented
   - "Manage Details" button display only

3. **Edit Functionality Not Implemented**:
   - Edit button displayed but non-functional

4. **Category2/3 Filter Disabled**:
   - JOIN with detail table not implemented

5. **Keyword Search Disabled**:
   - Memo search implementation incomplete

---

## Future Implementation Plans

### Priority: High
1. **Detail Management**:
   - Implement TRANSACTIONS_DETAIL CRUD
   - Create detail input modal
   - Manage tax rates and amounts per detail line

2. **Edit Functionality**:
   - Edit existing transactions
   - Update header and details simultaneously

### Priority: Medium
3. **Complete Filter Implementation**:
   - Category2/3 filter (JOIN with details)
   - Keyword search (full-text memo search)

4. **Session Management**:
   - Persist logged-in user information
   - Dynamic retrieval of currentUserId/currentUserRole

### Priority: Low
5. **UI/UX Improvements**:
   - Loading indicators
   - Detailed error messages
   - Enhanced operation feedback

---

## Change History

### 2025-11-12
- TRANSACTIONS_DETAIL table normalization complete
- Added USER_ID and CATEGORY1_CODE fields
- Set composite foreign key constraints to CATEGORY2/CATEGORY3
- Implemented automatic migration (preserving existing data)
- Added test cases (all 151 tests passed)

### 2025-11-08
- Internationalization complete (34 resources added)
- Fixed category selection bug (innerHTML issue)
- Implemented account name & memo display
- Document update

### 2025-11-07
- Account management screen completed
- NONE account auto-generation
- User filtering implementation

### 2025-11-06
- Transaction registration functionality
- DateTime input support (datetime-local)
- Tax rate design change (header→detail)

### 2025-11-05
- New schema design (V2)
- Database table creation
- Basic list display implementation

---

## References

- [TRANSACTION_DESIGN_V2.md](./TRANSACTION_DESIGN_V2.md) - V2 Design Details
- [ACCOUNT_MANAGEMENT_UI.md](./ACCOUNT_MANAGEMENT_UI.md) - Account Management Screen
- [I18N_IMPLEMENTATION.md](./I18N_IMPLEMENTATION.md) - Internationalization Implementation

---

**Author**: AI Assistant  
**Supervisor**: Yoshihiro NAKAHARA (bonojovi@zundou.org)
