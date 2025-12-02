# Aggregation API Reference

**Last Updated**: 2025-11-21 05:10 JST

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Data Structures](#data-structures)
4. [Tauri Commands](#tauri-commands)
5. [Rust Backend Functions](#rust-backend-functions)
6. [Usage Examples](#usage-examples)
7. [Error Handling](#error-handling)

---

## Overview

The Aggregation API provides functionality to aggregate and analyze transaction data with various conditions.

### Key Features

- **Time-based aggregation**: Monthly, Daily, Weekly, Yearly, Custom Period
- **Grouping**: By Category, Account, Shop
- **Dynamic SQL generation**: Optimized queries based on conditions
- **Type-safe design**: Enums prevent invalid condition specifications

### Architecture Highlights

- **3-layer design**: Core → Wrapper → Tauri Command
- **Separation of concerns**: Core handles SQL, Wrapper handles validation
- **Extensibility**: Easy to add new aggregation types

---

## Architecture

### 3-Layer Architecture

```
┌─────────────────────────────────────┐
│  Frontend (JavaScript)              │
│  - UI operations                    │
│  - Parameter collection             │
│  - Result display                   │
└────────────┬────────────────────────┘
             │ Tauri invoke()
             ↓
┌─────────────────────────────────────┐
│  Tauri Command Layer                │
│  - Accept frontend calls            │
│  - Parse parameters                 │
│  - Call wrapper functions           │
└────────────┬────────────────────────┘
             │
             ↓
┌─────────────────────────────────────┐
│  Wrapper Function Layer             │
│  - Period validation                │
│  - Business logic                   │
│  - Call core functions              │
└────────────┬────────────────────────┘
             │
             ↓
┌─────────────────────────────────────┐
│  Core Function Layer (SQL Gen)      │
│  - Dynamic SQL generation           │
│  - Query execution                  │
│  - Result formatting                │
└─────────────────────────────────────┘
```

---

## Data Structures

### Enums

#### DateFilter

Represents date filter conditions.

```rust
pub enum DateFilter {
    Exact(NaiveDate),               // Specific date
    Between(NaiveDate, NaiveDate),  // Date range
}
```

#### GroupBy

Represents grouping axis.

```rust
pub enum GroupBy {
    Category1,  // Level 1 category
    Category2,  // Level 2 category
    Category3,  // Level 3 category
    Account,    // Account
    Shop,       // Shop
}
```

#### OrderField & SortOrder

```rust
pub enum OrderField {
    Amount,  // Amount
}

pub enum SortOrder {
    Asc,   // Ascending
    Desc,  // Descending
}
```

### Structs

#### AggregationFilter

```rust
pub struct AggregationFilter {
    date: DateFilter,                    // Date filter (required)
    amount: Option<AmountFilter>,        // Amount filter (future)
    category: Option<CategoryFilter>,    // Category filter (future)
    shop_id: Option<i64>,                // Shop ID (future)
}
```

#### AggregationRequest

```rust
pub struct AggregationRequest {
    user_id: i64,                  // User ID
    filter: AggregationFilter,     // Filter conditions
    group_by: GroupBy,             // Grouping axis
    order_by: OrderField,          // Sort field
    sort_order: SortOrder,         // Sort order
}
```

#### AggregationResult

```rust
#[derive(Debug, Serialize)]
pub struct AggregationResult {
    pub group_key: String,      // Group key (code)
    pub group_name: String,     // Display name
    pub total_amount: i64,      // Total amount (net change)
    pub count: i64,             // Transaction count
    pub avg_amount: i64,        // Average amount
}
```

---

## Tauri Commands

### get_monthly_aggregation

Execute monthly aggregation.

```rust
#[tauri::command]
async fn get_monthly_aggregation(
    user_id: i64,
    year: i32,
    month: u32,
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

**Frontend call**:
```javascript
const results = await invoke('get_monthly_aggregation', {
    userId: 1,
    year: 2025,
    month: 11,
    groupBy: 'category1'
});
```

---

### get_daily_aggregation

Execute daily aggregation.

```rust
#[tauri::command]
async fn get_daily_aggregation(
    user_id: i64,
    date: String,           // Format: "YYYY-MM-DD"
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

**Frontend call**:
```javascript
const results = await invoke('get_daily_aggregation', {
    userId: 1,
    date: '2025-11-20',
    groupBy: 'category2'
});
```

---

### get_weekly_aggregation_by_date

Execute weekly aggregation (reference date based).

```rust
#[tauri::command]
async fn get_weekly_aggregation_by_date(
    user_id: i64,
    reference_date: String,  // Format: "YYYY-MM-DD"
    week_start: String,      // "sunday" or "monday"
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

**Frontend call**:
```javascript
const results = await invoke('get_weekly_aggregation_by_date', {
    userId: 1,
    referenceDate: '2025-11-20',
    weekStart: 'monday',
    groupBy: 'account'
});
```

**Week calculation logic**:
```rust
// Find week start from reference date
let days_from_week_start = reference_date.weekday().num_days_from(week_start);
let week_start_date = reference_date - Duration::days(days_from_week_start);
let week_end_date = week_start_date + Duration::days(6);
```

---

### get_yearly_aggregation

Execute yearly aggregation.

```rust
#[tauri::command]
async fn get_yearly_aggregation(
    user_id: i64,
    year: i32,
    year_start_month: u32,  // 1 for January, 4 for April
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

**Frontend call**:
```javascript
// Calendar year (Jan start)
const results1 = await invoke('get_yearly_aggregation', {
    userId: 1,
    year: 2025,
    yearStartMonth: 1,
    groupBy: 'shop'
});

// Fiscal year (Apr start)
const results2 = await invoke('get_yearly_aggregation', {
    userId: 1,
    year: 2025,
    yearStartMonth: 4,
    groupBy: 'shop'
});
```

---

### get_period_aggregation

Execute custom period aggregation.

```rust
#[tauri::command]
async fn get_period_aggregation(
    user_id: i64,
    start_date: String,  // Format: "YYYY-MM-DD"
    end_date: String,    // Format: "YYYY-MM-DD"
    group_by: String,
    state: tauri::State<'_, AppState>
) -> Result<Vec<AggregationResult>, String>
```

**Frontend call**:
```javascript
const results = await invoke('get_period_aggregation', {
    userId: 1,
    startDate: '2025-10-01',
    endDate: '2025-11-20',
    groupBy: 'category3'
});
```

---

## Rust Backend Functions

### Core Functions

Core functions generate `AggregationRequest` objects.

#### monthly_aggregation

```rust
pub fn monthly_aggregation(
    user_id: i64,
    year: i32,
    month: u32,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**Features**:
- Calculate start/end dates from year/month
- Create `DateFilter::Between`
- Validate future dates

---

#### daily_aggregation

```rust
pub fn daily_aggregation(
    user_id: i64,
    date: NaiveDate,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**Features**:
- Create `DateFilter::Exact`
- Validate future dates

---

#### weekly_aggregation_by_date

```rust
pub fn weekly_aggregation_by_date(
    user_id: i64,
    reference_date: NaiveDate,
    week_start: Weekday,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**Features**:
- Calculate week range from reference date and week start
- O(1) calculation (fast)
- No week number needed (more intuitive)

---

#### yearly_aggregation

```rust
pub fn yearly_aggregation(
    user_id: i64,
    year: i32,
    year_start_month: u32,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**Features**:
- Calculate fiscal year range
- Support calendar year (Jan start) and fiscal year (Apr start)

---

#### period_aggregation

```rust
pub fn period_aggregation(
    user_id: i64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError>
```

**Features**:
- Validate date range
- Check `start_date <= end_date`
- Check future dates

---

### Wrapper Functions

Wrapper functions call core functions and execute queries.

#### execute_monthly_aggregation

```rust
pub async fn execute_monthly_aggregation(
    pool: &SqlitePool,
    user_id: i64,
    year: i32,
    month: u32,
    group_by: GroupBy,
    lang: &str,
) -> Result<Vec<AggregationResult>, String>
```

**Functionality**:
- Call core function to generate request
- Call `execute_aggregation` to execute SQL
- Convert errors to strings

Similar functions exist for daily, weekly, yearly, and period aggregations.

---

### Common Execution Function

#### execute_aggregation

Main function that executes all aggregation requests.

```rust
pub async fn execute_aggregation(
    pool: &SqlitePool,
    request: &AggregationRequest,
    lang: &str,
) -> Result<Vec<AggregationResult>, String>
```

**Functionality**:
- Generate dynamic SQL from request
- Execute SQL
- Format and return results

**Dynamic SQL example** (Category 1 grouping):
```sql
SELECT 
    h.CATEGORY1_CODE as group_key,
    COALESCE(i.CATEGORY1_NAME_I18N, c.CATEGORY1_NAME) as group_name,
    SUM(CASE 
        WHEN h.CATEGORY1_CODE = 'INCOME' THEN h.AMOUNT
        WHEN h.CATEGORY1_CODE = 'EXPENSE' THEN -h.AMOUNT
        ELSE 0
    END) as total_amount,
    COUNT(*) as count
FROM TRANSACTION_HEADERS h
LEFT JOIN CATEGORY1 c ON h.CATEGORY1_CODE = c.CATEGORY1_CODE
LEFT JOIN CATEGORY1_I18N i ON c.CATEGORY1_CODE = i.CATEGORY1_CODE AND i.LANG_CODE = ?
WHERE h.USER_ID = ? AND h.TRANSACTION_DATE >= ? AND h.TRANSACTION_DATE <= ?
GROUP BY h.CATEGORY1_CODE
ORDER BY total_amount DESC
```

---

## Usage Examples

### Example 1: Monthly Aggregation (Category 1)

```javascript
const results = await invoke('get_monthly_aggregation', {
    userId: 1,
    year: 2025,
    month: 11,
    groupBy: 'category1'
});

// Result
[
    {
        group_key: "EXPENSE",
        group_name: "Expense",
        total_amount: -150000,
        count: 45,
        avg_amount: -3333
    },
    {
        group_key: "INCOME",
        group_name: "Income",
        total_amount: 300000,
        count: 1,
        avg_amount: 300000
    }
]
```

### Example 2: Weekly Aggregation (Shop)

```javascript
const results = await invoke('get_weekly_aggregation_by_date', {
    userId: 1,
    referenceDate: '2025-11-20',
    weekStart: 'monday',
    groupBy: 'shop'
});

// Result
[
    {
        group_key: "1",
        group_name: "Convenience Store A",
        total_amount: -15000,
        count: 10,
        avg_amount: -1500
    },
    {
        group_key: "2",
        group_name: "Supermarket B",
        total_amount: -30000,
        count: 3,
        avg_amount: -10000
    }
]
```

---

## Error Handling

### Error Types

#### AggregationError

```rust
pub enum AggregationError {
    InvalidMonth { year: i32, month: u32 },
    InvalidDateRange { start: NaiveDate, end: NaiveDate },
    FutureDate { year: i32, month: u32 },
    DatabaseError(String),
}
```

### Error Example

```javascript
try {
    const results = await invoke('get_monthly_aggregation', {
        userId: 1,
        year: 2025,
        month: 13,  // Invalid month
        groupBy: 'category1'
    });
} catch (error) {
    console.error('Aggregation error:', error);
    // "Invalid month: 2025-13"
}
```

### Validation

Each function performs the following validations:

1. **Month validation** (1-12)
2. **Date range validation** (start_date <= end_date)
3. **Future date check** (date <= today)
4. **Week start validation** ("sunday" or "monday")

---

## Performance Optimization

### Index Usage

Aggregation queries use these indexes:

```sql
CREATE INDEX idx_transaction_headers_user_date 
ON TRANSACTION_HEADERS(USER_ID, TRANSACTION_DATE);

CREATE INDEX idx_transaction_headers_category 
ON TRANSACTION_HEADERS(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE);
```

### Query Optimization

- **LEFT JOIN**: Fallback when i18n name doesn't exist
- **COALESCE**: Safe NULL handling
- **GROUP BY**: Dynamic grouping based on axis
- **ORDER BY**: Default is amount descending (most expenses first)

---

## Future Enhancements

- [ ] `AmountFilter` implementation (amount range filter)
- [ ] `CategoryFilter` implementation (category filter)
- [ ] Product/Manufacturer aggregation
- [ ] Pagination support
- [ ] CSV export

---

## Related Documentation

- [Aggregation User Guide](AGGREGATION_USER_GUIDE.md) - User guide
- [Transaction API](API_TRANSACTION.md) - Transaction data API
- [Category API](API_CATEGORY.md) - Category management API

---

**Last Updated**: 2025-11-21 05:10 JST
