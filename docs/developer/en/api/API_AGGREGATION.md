# Aggregation API Reference

**Last Updated**: 2025-12-05 02:36 JST

## Overview

This document defines APIs used in the aggregation screen. Aggregates transaction data by various conditions and returns analysis results.

---

## Table of Contents

1. [Aggregation API List](#aggregation-api-list)
2. [Data Structures](#data-structures)
3. [Error Handling](#error-handling)
4. [Usage Examples](#usage-examples)

---

## Aggregation API List

### get_monthly_aggregation

Executes monthly aggregation.

**Parameters:**
- `year` (i32): Year (e.g., 2025)
- `month` (u32): Month (1-12)
- `group_by` (String): Aggregation axis ("category1", "category2", "category3", "account", "shop")

**Return Value:**
- `Vec<AggregationResult>`: Array of aggregation results

**Usage Example:**
```javascript
const results = await invoke('get_monthly_aggregation', {
    year: 2025,
    month: 12,
    groupBy: 'category1'
});

results.forEach(r => {
    console.log(`${r.name}: ¥${r.total_amount}`);
});
```

**Note:**
- Session user ID automatically retrieved
- Names retrieved according to language setting (I18N table)

---

### get_daily_aggregation

Executes daily aggregation.

**Parameters:**
- `date` (String): Date ("YYYY-MM-DD" format)
- `group_by` (String): Aggregation axis

**Return Value:**
- `Vec<AggregationResult>`: Array of aggregation results

**Usage Example:**
```javascript
const results = await invoke('get_daily_aggregation', {
    date: '2025-12-05',
    groupBy: 'category2'
});
```

---

### get_period_aggregation

Executes period aggregation.

**Parameters:**
- `start_date` (String): Start date ("YYYY-MM-DD" format)
- `end_date` (String): End date ("YYYY-MM-DD" format)
- `group_by` (String): Aggregation axis

**Return Value:**
- `Vec<AggregationResult>`: Array of aggregation results

**Usage Example:**
```javascript
const results = await invoke('get_period_aggregation', {
    startDate: '2025-12-01',
    endDate: '2025-12-31',
    groupBy: 'category3'
});
```

**Validation:**
- `start_date <= end_date`
- Future dates not allowed

---

### get_weekly_aggregation

Executes weekly aggregation (by year and week number).

**Parameters:**
- `year` (i32): Year
- `week` (u32): Week number (1-53)
- `week_start` (String): Week start day ("sunday" or "monday")
- `group_by` (String): Aggregation axis

**Return Value:**
- `Vec<AggregationResult>`: Array of aggregation results

**Usage Example:**
```javascript
const results = await invoke('get_weekly_aggregation', {
    year: 2025,
    week: 49,
    weekStart: 'monday',
    groupBy: 'account'
});
```

---

### get_weekly_aggregation_by_date

Executes weekly aggregation (by reference date).

**Parameters:**
- `reference_date` (String): Reference date ("YYYY-MM-DD" format)
- `week_start` (String): Week start day ("sunday" or "monday")
- `group_by` (String): Aggregation axis

**Return Value:**
- `Vec<AggregationResult>`: Array of aggregation results

**Usage Example:**
```javascript
const results = await invoke('get_weekly_aggregation_by_date', {
    referenceDate: '2025-12-05',
    weekStart: 'sunday',
    groupBy: 'shop'
});
```

**Behavior:**
- Automatically calculates week containing specified date
- Determines period based on week start day

---

### get_yearly_aggregation

Executes yearly aggregation.

**Parameters:**
- `year` (i32): Year
- `year_start` (String): Fiscal year start month ("january" or "april")
- `group_by` (String): Aggregation axis

**Return Value:**
- `Vec<AggregationResult>`: Array of aggregation results

**Usage Example:**
```javascript
// January to December (calendar year)
const results = await invoke('get_yearly_aggregation', {
    year: 2025,
    yearStart: 'january',
    groupBy: 'category1'
});

// April to March (fiscal year)
const resultsApr = await invoke('get_yearly_aggregation', {
    year: 2025,
    yearStart: 'april',
    groupBy: 'category1'
});
```

---

### get_monthly_aggregation_by_category

Executes monthly aggregation with category filter.

**Parameters:**
- `year` (i32): Year
- `month` (u32): Month
- `group_by` (String): Aggregation axis
- `category1_code` (String): Category1 code
- `category2_code` (Option<String>): Category2 code (filter)
- `category3_code` (Option<String>): Category3 code (filter)

**Return Value:**
- `Vec<AggregationResult>`: Array of aggregation results

**Usage Example:**
```javascript
// Aggregate all expenses by category2
const results = await invoke('get_monthly_aggregation_by_category', {
    year: 2025,
    month: 12,
    groupBy: 'category2',
    category1Code: 'EXPENSE',
    category2Code: null,
    category3Code: null
});

// Aggregate food expenses by category3
const foodResults = await invoke('get_monthly_aggregation_by_category', {
    year: 2025,
    month: 12,
    groupBy: 'category3',
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',  // Food
    category3Code: null
});
```

---

## Data Structures

### AggregationResult

```rust
pub struct AggregationResult {
    pub code: Option<String>,       // Category code, account code, etc.
    pub name: String,                // Name (multilingual)
    pub total_amount: i64,           // Total amount
    pub transaction_count: i64,      // Transaction count
}
```

**Frontend reception:**
```javascript
{
    code: "EXPENSE",
    name: "Expense",
    total_amount: 150000,
    transaction_count: 25
}
```

---

## Aggregation Axes (group_by)

| Value | Description | Example |
|-------|-------------|---------|
| `"category1"` | By Category1 | Expense, Income, Transfer |
| `"category2"` | By Category2 | Food, Transportation, Entertainment |
| `"category3"` | By Category3 | Groceries, Dining Out, Beverages |
| `"account"` | By Account | Cash, Bank Account, Credit Card |
| `"shop"` | By Shop | AEON, 7-Eleven |

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"User not authenticated"` | Session not authenticated | Login required |
| `"Invalid month: ..."` | Month out of 1-12 range | Specify valid month |
| `"Invalid date format: ..."` | Invalid date format | Fix to "YYYY-MM-DD" format |
| `"start_date must be <= end_date"` | Period reversed | Fix start_date ≤ end_date |
| `"Invalid week_start: ..."` | Invalid week start day | Specify "sunday" or "monday" |
| `"Invalid year_start: ..."` | Invalid fiscal year start | Specify "january" or "april" |

### Frontend Error Handling Example

```javascript
async function loadMonthlyAggregation(year, month, groupBy) {
    try {
        const results = await invoke('get_monthly_aggregation', {
            year,
            month,
            groupBy
        });
        
        return results;
    } catch (error) {
        if (error.includes('Invalid month')) {
            alert('Invalid month specified (1-12)');
        } else if (error.includes('not authenticated')) {
            window.location.href = 'index.html';
        } else {
            alert(`Aggregation error: ${error}`);
        }
        return [];
    }
}
```

---

## Usage Examples: Aggregation Screen Implementation

### Monthly Aggregation Display

```javascript
async function displayMonthlyAggregation() {
    const year = parseInt(document.getElementById('year-select').value);
    const month = parseInt(document.getElementById('month-select').value);
    const groupBy = document.getElementById('groupby-select').value;
    
    try {
        const results = await invoke('get_monthly_aggregation', {
            year,
            month,
            groupBy
        });
        
        const tbody = document.getElementById('aggregation-table-body');
        tbody.innerHTML = '';
        
        // Calculate total amount
        const totalAmount = results.reduce((sum, r) => sum + r.total_amount, 0);
        
        results.forEach(result => {
            const row = document.createElement('tr');
            const percentage = ((result.total_amount / totalAmount) * 100).toFixed(1);
            
            row.innerHTML = `
                <td>${result.name}</td>
                <td>¥${result.total_amount.toLocaleString()}</td>
                <td>${result.transaction_count} items</td>
                <td>${percentage}%</td>
            `;
            tbody.appendChild(row);
        });
        
        // Add total row
        const totalRow = document.createElement('tr');
        totalRow.innerHTML = `
            <td><strong>Total</strong></td>
            <td><strong>¥${totalAmount.toLocaleString()}</strong></td>
            <td><strong>${results.reduce((sum, r) => sum + r.transaction_count, 0)} items</strong></td>
            <td><strong>100%</strong></td>
        `;
        tbody.appendChild(totalRow);
        
    } catch (error) {
        alert(`Aggregation error: ${error}`);
    }
}
```

### Period Aggregation Implementation

```javascript
async function displayPeriodAggregation() {
    const startDate = document.getElementById('start-date').value;
    const endDate = document.getElementById('end-date').value;
    const groupBy = document.getElementById('groupby-select').value;
    
    // Date validation
    if (new Date(startDate) > new Date(endDate)) {
        alert('Start date must be before end date');
        return;
    }
    
    try {
        const results = await invoke('get_period_aggregation', {
            startDate,
            endDate,
            groupBy
        });
        
        renderAggregationResults(results);
    } catch (error) {
        alert(`Aggregation error: ${error}`);
    }
}
```

### Category-based Aggregation Implementation

```javascript
async function displayCategoryAggregation() {
    const year = parseInt(document.getElementById('year-select').value);
    const month = parseInt(document.getElementById('month-select').value);
    const category1Code = document.getElementById('category1-select').value;
    const category2Code = document.getElementById('category2-select').value || null;
    
    try {
        const results = await invoke('get_monthly_aggregation_by_category', {
            year,
            month,
            groupBy: 'category3',
            category1Code,
            category2Code,
            category3Code: null
        });
        
        renderAggregationResults(results);
    } catch (error) {
        alert(`Aggregation error: ${error}`);
    }
}
```

---

## Multilingual Support

Aggregation APIs return names according to configured language.

**Mechanism:**
1. Get language from `settings.get_string("language")`
2. Retrieve name in that language from I18N table
3. Fallback to default name if not found via LEFT JOIN

**Example:**
```javascript
// When Japanese is configured
{
    code: "EXPENSE",
    name: "支出",  // Retrieved from I18N table
    total_amount: 150000,
    transaction_count: 25
}

// When English is configured
{
    code: "EXPENSE",
    name: "Expense",  // Retrieved from I18N table
    total_amount: 150000,
    transaction_count: 25
}
```

---

## Performance Optimization

### Index Utilization

Aggregation queries use the following indexes:

```sql
CREATE INDEX idx_transaction_headers_user_date 
ON TRANSACTION_HEADERS(USER_ID, TRANSACTION_DATE);

CREATE INDEX idx_transaction_headers_category 
ON TRANSACTION_HEADERS(USER_ID, CATEGORY1_CODE);
```

### Processing Large Data

- **GROUP BY**: Aggregation on database side
- **ORDER BY**: Descending by amount (highest expenses first)
- **LEFT JOIN**: Efficient retrieval of multilingual names

---

## Future Extensions

- [ ] Amount range filter
- [ ] Product/manufacturer aggregation
- [ ] Pagination support
- [ ] CSV export
- [ ] Data formatting for graphs

---

## Test Coverage

**AggregationService:**
- ✅ Monthly aggregation test
- ✅ Daily aggregation test
- ✅ Period aggregation test
- ✅ Weekly aggregation test
- ✅ Yearly aggregation test
- ✅ Category-filtered aggregation test
- ✅ Date validation test

---

## Related Documents

### Implementation Files

- Aggregation Service: `src/services/aggregation.rs`
- SQL Definitions: `src/sql_queries.rs`
- Tauri Commands: `src/lib.rs`

### Other API References

- [Common API](./API_COMMON.md) - Session management
- [Transaction Management API](./API_TRANSACTION.md) - Transaction data
- [Category Management API](./API_CATEGORY.md) - Category information

---

**Change History:**
- 2025-11-21: Initial version
- 2025-12-05: Revised based on implementation code
  - Removed user_id parameter (auto-retrieved from session)
  - Fixed usage examples to match implementation
  - Unified with new template
