# Aggregation User Guide

**Last Updated**: 2025-11-21 05:05 JST

## Table of Contents

1. [Overview](#overview)
2. [5 Aggregation Methods](#5-aggregation-methods)
3. [Common Features](#common-features)
4. [How to Use Each Screen](#how-to-use-each-screen)
5. [Understanding Results](#understanding-results)
6. [Tips & Troubleshooting](#tips--troubleshooting)

---

## Overview

KakeiBon's aggregation feature allows you to analyze transaction data from various perspectives.

### What You Can Do

- **Period Analysis**: Daily, weekly, monthly, yearly, or custom period aggregation
- **Category Analysis**: Aggregate by category (level 1/2/3)
- **Account Analysis**: Track income/expenses per account
- **Shop Analysis**: Identify frequently used shops and manage expenses

### Information Displayed in Results

Each aggregation result shows:

- **Group Name**: Category name, account name, shop name, etc.
- **Total Amount (Net Change)**: Income added, expenses subtracted
- **Count**: Number of transactions
- **Average Amount**: Total amount divided by count

---

## 5 Aggregation Methods

KakeiBon provides 5 aggregation methods for different use cases.

| Method | Use Case | Input |
|--------|----------|-------|
| **Monthly** | Check monthly income/expenses | Year & month |
| **Daily** | Check daily income/expenses | Date |
| **Weekly** | Check weekly income/expenses | Reference date & week start day |
| **Yearly** | Check annual income/expenses | Year & fiscal year start month |
| **Period** | Check custom period income/expenses | Start date & end date |

### Which Method Should I Use?

- **Monthly income/expenses** → **Monthly Aggregation**
- **Today's transactions** → **Daily Aggregation**
- **This week's transactions** → **Weekly Aggregation**
- **Annual summary** → **Yearly Aggregation** (Calendar: Jan start, Fiscal: Apr start)
- **Specific period** → **Period Aggregation** (e.g., Oct 1 - Nov 20)

---

## Common Features

Features available on all aggregation screens.

### Grouping Axis Selection

Choose from 5 grouping axes:

1. **Category 1 (Level 1)** - Expense, Income, Transfer
2. **Category 2 (Level 2)** - Food, Transportation, etc.
3. **Category 3 (Level 3)** - Dining out, Groceries, etc.
4. **Account** - Cash, Bank accounts, etc.
5. **Shop** - Convenience stores, Supermarkets, etc.

### Filter Panel Toggle

Each screen has a collapsible filter panel.

- **Open/Close**: Click "Filter" heading or ▲/▼ button
- Execute button remains visible when collapsed

### Execution

1. Specify period
2. Select grouping axis (Category/Account/Shop)
3. Click "Execute" button

※ You can also press Enter in date input fields

---

## How to Use Each Screen

### 1. Monthly Aggregation

**Menu**: Management → Monthly Aggregation

#### Steps

1. **Select Year**
   - Use ↑↓ buttons next to year field
   - Or type year directly

2. **Select Month**
   - Choose 1-12 from month dropdown

3. **Select Grouping Axis**
   - Choose from "Group By" dropdown

4. **Execute**
   - Click "Execute" button

#### Default Settings

- Year: Current year
- Month: Current month
- Group By: Category 1

---

### 2. Daily Aggregation

**Menu**: Management → Daily Aggregation

#### Steps

1. **Select Date**
   - Click calendar icon to pick date
   - Or type date directly (YYYY-MM-DD format)

2. **Select Grouping Axis**
   - Choose from "Group By" dropdown

3. **Execute**
   - Click "Execute" button or press Enter

#### Default Settings

- Date: Today
- Group By: Category 1

---

### 3. Weekly Aggregation

**Menu**: Management → Weekly Aggregation

#### Steps

1. **Select Reference Date**
   - Pick any date from calendar
   - Any day within target week is OK (doesn't have to be Monday)

2. **Select Week Start Day**
   - Choose "Sunday" or "Monday"
   - Monday is commonly used in Japan

3. **Select Grouping Axis**
   - Choose from "Group By" dropdown

4. **Execute**
   - Click "Execute" button

#### Week Range Calculation

- **Monday Start**: Monday to Sunday of the week containing reference date
- **Sunday Start**: Sunday to Saturday of the week containing reference date

Example: Reference date = Nov 20, 2025 (Thursday)
- Monday Start: Nov 17 (Mon) - Nov 23 (Sun), 2025
- Sunday Start: Nov 16 (Sun) - Nov 22 (Sat), 2025

#### Default Settings

- Reference Date: Today
- Week Start: Monday
- Group By: Category 1

---

### 4. Yearly Aggregation

**Menu**: Management → Yearly Aggregation

#### Steps

1. **Select Year**
   - Use ↑↓ buttons next to year field
   - Or type year directly

2. **Select Year Start Month**
   - "January (Calendar)": Jan 1 - Dec 31
   - "April (Fiscal)": Apr 1 - Mar 31 next year

3. **Select Grouping Axis**
   - Choose from "Group By" dropdown

4. **Execute**
   - Click "Execute" button

#### Fiscal Year Concept

- **Calendar Year**: Standard Jan-Dec year
- **Fiscal Year**: Japanese fiscal year (Apr-Mar)

Example: Year=2025, Start Month=April
- Period: Apr 1, 2025 - Mar 31, 2026 (FY2025)

#### Default Settings

- Year: Current year
- Year Start: January (Calendar)
- Group By: Category 1

---

### 5. Period Aggregation

**Menu**: Management → Period Aggregation

#### Steps

1. **Select Start Date**
   - Pick start date from calendar

2. **Select End Date**
   - Pick end date from calendar

3. **Select Grouping Axis**
   - Choose from "Group By" dropdown

4. **Execute**
   - Click "Execute" button

#### Validation

- Start date must be before end date
- Future dates are not allowed

#### Default Settings

- Start Date: First day of current month
- End Date: Today
- Group By: Category 1

#### Use Cases

- **Last 7 days**: 7 days ago - today
- **Quarterly**: Oct 1 - Dec 31
- **Travel period**: Nov 1 - Nov 5

---

## Understanding Results

### Result Table Structure

Results are displayed in table format:

| Group Name | Total Amount (Net) | Count | Average |
|-----------|-------------------|-------|---------|
| Food | -50,000 | 30 | -1,667 |
| Salary | +300,000 | 1 | +300,000 |

### Amount Display Rules

- **Plus (+)**: Income (asset increase)
- **Minus (-)**: Expense (asset decrease)
- **Net Change**: Income and expenses combined

### Transfer Handling

#### General Aggregation (Category/Shop)

Transfer transactions (account-to-account moves) are **always 0** and excluded from results.

Reason: Transfers don't change total assets, so they're omitted.

#### Account Aggregation

Account aggregation includes transfer transactions:

- **FROM Account (source)**: Negative amount
- **TO Account (destination)**: Positive amount

**Note**: "※ Account aggregation includes transfers" is displayed at bottom.

---

## Tips & Troubleshooting

### FAQ

#### Q1. No Data Displayed

**Causes and Solutions**:
1. No data exists in selected period → Try different period
2. Filter too strict → Change grouping axis
3. No transaction data in database → Register data first

#### Q2. All Amounts are Negative

When expenses exceed income, most items show negative amounts.

- **Budget Tip**: Review items with largest negative amounts first

#### Q3. Account Aggregation Shows Different Amounts

Account aggregation includes transfers, so results differ from other axes.

- Category/Shop: Transfers excluded (pure income/expenses)
- Account: Transfers included (shows fund movements)

#### Q4. Weekly Range is Unclear

Week range is auto-calculated based on reference date.

**Verification**: After execution, check console log for actual period (Developer Tools → Console)

---

### Performance Tips

#### Large Dataset

- **Shorter periods**: Daily faster than monthly, monthly faster than yearly
- **Simpler grouping**: Category 1 faster than Category 3

#### Slow Display

- Clear browser cache
- Database optimization (planned feature)

---

### Keyboard Shortcuts

- **Enter**: Execute after date input
- **Tab**: Navigate between fields
- **Esc**: Close modal (if applicable)

---

## Planned Features

Currently in development or planned:

- [ ] Graph display (bar/pie/line charts)
- [ ] CSV export
- [ ] Advanced filters (amount range, keyword search)
- [ ] Product/Manufacturer aggregation
- [ ] Budget vs Actual comparison

---

## Related Documentation

- [Aggregation API Reference](API_AGGREGATION.md) - Developer API specs
- [Transaction Management Guide](TRANSACTION_MANAGEMENT_UI_V2.md) - Data entry
- [Category Management Guide](CATEGORY_MANAGEMENT_UI.md) - Category setup

---

**Last Updated**: 2025-11-21 05:05 JST
