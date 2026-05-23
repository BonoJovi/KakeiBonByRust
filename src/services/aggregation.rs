//! Aggregation service for financial reporting and analysis
//!
//! This module provides type-safe aggregation functionality using Enums
//! to prevent invalid SQL generation at compile time.

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

use crate::services::period::{monthly_period_bounds, yearly_period_bounds};

// =============================================================================
// Filter Enums
// =============================================================================

/// Date filter for aggregation queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateFilter {
    /// Filter transactions from a specific date onwards
    From(NaiveDate),
    /// Filter transactions up to a specific date
    To(NaiveDate),
    /// Filter transactions within a date range (inclusive)
    Between(NaiveDate, NaiveDate),
    /// Filter transactions on an exact date
    Exact(NaiveDate),
}

impl DateFilter {
    /// Generate SQL WHERE clause for date filtering
    pub fn to_sql(&self) -> String {
        match self {
            DateFilter::From(date) => {
                format!("DATE(th.TRANSACTION_DATE) >= '{}'", date.format("%Y-%m-%d"))
            }
            DateFilter::To(date) => {
                format!("DATE(th.TRANSACTION_DATE) <= '{}'", date.format("%Y-%m-%d"))
            }
            DateFilter::Between(from, to) => {
                format!(
                    "DATE(th.TRANSACTION_DATE) BETWEEN '{}' AND '{}'",
                    from.format("%Y-%m-%d"),
                    to.format("%Y-%m-%d")
                )
            }
            DateFilter::Exact(date) => {
                format!("DATE(th.TRANSACTION_DATE) = '{}'", date.format("%Y-%m-%d"))
            }
        }
    }
}

/// Amount filter for aggregation queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AmountFilter {
    /// Filter transactions with amount greater than or equal to value
    GreaterThan(i64),
    /// Filter transactions with amount less than or equal to value
    LessThan(i64),
    /// Filter transactions within an amount range (inclusive)
    Between(i64, i64),
    /// Filter transactions with exact amount
    Exact(i64),
    /// No amount filter
    None,
}

impl AmountFilter {
    /// Generate SQL WHERE clause for amount filtering
    pub fn to_sql(&self) -> String {
        match self {
            AmountFilter::GreaterThan(amount) => {
                format!("th.TOTAL_AMOUNT >= {}", amount)
            }
            AmountFilter::LessThan(amount) => {
                format!("th.TOTAL_AMOUNT <= {}", amount)
            }
            AmountFilter::Between(min, max) => {
                format!("th.TOTAL_AMOUNT BETWEEN {} AND {}", min, max)
            }
            AmountFilter::Exact(amount) => {
                format!("th.TOTAL_AMOUNT = {}", amount)
            }
            AmountFilter::None => String::new(),
        }
    }

    /// Check if the filter has any condition
    pub fn has_condition(&self) -> bool {
        !matches!(self, AmountFilter::None)
    }
}

/// Category filter for aggregation queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CategoryFilter {
    /// Filter by category1 code only
    Category1(String),
    /// Filter by category1 and category2 codes
    Category2(String, String),
    /// Filter by category1, category2, and category3 codes
    Category3(String, String, String),
    /// No category filter
    None,
}

impl CategoryFilter {
    /// Generate SQL WHERE clause for category filtering
    pub fn to_sql(&self) -> String {
        match self {
            CategoryFilter::Category1(cat1) => {
                format!("th.CATEGORY1_CODE = '{}'", cat1)
            }
            CategoryFilter::Category2(cat1, cat2) => {
                format!(
                    "th.CATEGORY1_CODE = '{}' AND th.CATEGORY2_CODE = '{}'",
                    cat1, cat2
                )
            }
            CategoryFilter::Category3(cat1, cat2, cat3) => {
                format!(
                    "th.CATEGORY1_CODE = '{}' AND th.CATEGORY2_CODE = '{}' AND th.CATEGORY3_CODE = '{}'",
                    cat1, cat2, cat3
                )
            }
            CategoryFilter::None => String::new(),
        }
    }

    /// Check if the filter has any condition
    pub fn has_condition(&self) -> bool {
        !matches!(self, CategoryFilter::None)
    }
}

// =============================================================================
// Period Settings Enums
// =============================================================================

/// Week start day setting
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeekStart {
    /// Week starts on Sunday (US style)
    Sunday,
    /// Week starts on Monday (ISO 8601, Europe/Japan style)
    Monday,
}

// =============================================================================
// Aggregation Axis Enums
// =============================================================================

/// Group by axis for aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupBy {
    /// Group by category1 (大分類)
    Category1,
    /// Group by category2 (中分類)
    Category2,
    /// Group by category3 (小分類)
    Category3,
    /// Group by account
    Account,
    /// Group by shop
    Shop,
    /// Group by product
    Product,
    /// Group by date
    Date,
}

impl GroupBy {
    /// Generate SELECT clause fields for grouping
    pub fn to_select_clause(&self) -> String {
        match self {
            GroupBy::Category1 => {
                "th.CATEGORY1_CODE as group_key, \
                 COALESCE(c1i.CATEGORY1_NAME_I18N, c1.CATEGORY1_NAME) as group_name"
                    .to_string()
            }
            GroupBy::Category2 => {
                "td.CATEGORY1_CODE || '/' || td.CATEGORY2_CODE as group_key, \
                 COALESCE(c2i.CATEGORY2_NAME_I18N, c2.CATEGORY2_NAME) as group_name"
                    .to_string()
            }
            GroupBy::Category3 => {
                "td.CATEGORY1_CODE || '/' || td.CATEGORY2_CODE || '/' || td.CATEGORY3_CODE as group_key, \
                 COALESCE(c3i.CATEGORY3_NAME_I18N, c3.CATEGORY3_NAME) as group_name"
                    .to_string()
            }
            GroupBy::Account => {
                "COALESCE(th.FROM_ACCOUNT_CODE, th.TO_ACCOUNT_CODE, 'NONE') as group_key, \
                 COALESCE(a.ACCOUNT_NAME, '指定なし') as group_name"
                    .to_string()
            }
            GroupBy::Shop => {
                "CAST(COALESCE(th.SHOP_ID, 0) AS TEXT) as group_key, \
                 COALESCE(s.SHOP_NAME, '指定なし') as group_name"
                    .to_string()
            }
            GroupBy::Product => {
                "CAST(COALESCE(td.PRODUCT_ID, 0) AS TEXT) as group_key, \
                 COALESCE(p.PRODUCT_NAME, '指定なし') as group_name"
                    .to_string()
            }
            GroupBy::Date => {
                "DATE(th.TRANSACTION_DATE) as group_key, \
                 DATE(th.TRANSACTION_DATE) as group_name"
                    .to_string()
            }
        }
    }

    /// Generate GROUP BY clause
    pub fn to_group_by_clause(&self) -> String {
        match self {
            GroupBy::Category1 => "th.CATEGORY1_CODE".to_string(),
            GroupBy::Category2 => "td.CATEGORY1_CODE, td.CATEGORY2_CODE".to_string(),
            GroupBy::Category3 => {
                "td.CATEGORY1_CODE, td.CATEGORY2_CODE, td.CATEGORY3_CODE".to_string()
            }
            GroupBy::Account => "COALESCE(th.FROM_ACCOUNT_CODE, th.TO_ACCOUNT_CODE, 'NONE')".to_string(),
            GroupBy::Shop => "COALESCE(th.SHOP_ID, 0)".to_string(),
            GroupBy::Product => "COALESCE(td.PRODUCT_ID, 0)".to_string(),
            GroupBy::Date => "DATE(th.TRANSACTION_DATE)".to_string(),
        }
    }

}

// =============================================================================
// Sort Enums
// =============================================================================

/// Sort field for aggregation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderField {
    /// Sort by transaction date
    TransactionDate,
    /// Sort by amount
    Amount,
    /// Sort by category name
    CategoryName,
    /// Sort by shop name
    ShopName,
    /// Sort by count
    Count,
    /// Sort by group key
    GroupKey,
}

impl OrderField {
    /// Generate ORDER BY field
    pub fn to_order_by_field(&self) -> String {
        match self {
            OrderField::TransactionDate => "group_key".to_string(),
            OrderField::Amount => "total_amount".to_string(),
            OrderField::CategoryName => "group_name".to_string(),
            OrderField::ShopName => "group_name".to_string(),
            OrderField::Count => "count".to_string(),
            OrderField::GroupKey => "group_key".to_string(),
        }
    }
}

/// Sort order for aggregation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

impl SortOrder {
    /// Generate SQL sort direction
    pub fn to_sql(&self) -> &'static str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
}

// =============================================================================
// Composite Structures
// =============================================================================

/// Combined filter for aggregation queries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationFilter {
    /// Date filter (required)
    pub date: DateFilter,
    /// Amount filter (optional)
    pub amount: Option<AmountFilter>,
    /// Category filter (optional)
    pub category: Option<CategoryFilter>,
    /// Shop ID filter (optional)
    pub shop_id: Option<i64>,
    /// Include scheduled transactions (default: false = exclude scheduled)
    pub include_scheduled: bool,
}

impl AggregationFilter {
    /// Create a new aggregation filter with only date filter
    pub fn new(date: DateFilter) -> Self {
        Self {
            date,
            amount: None,
            category: None,
            shop_id: None,
            include_scheduled: false,
        }
    }

    /// Set include_scheduled filter
    #[allow(dead_code)]
    pub fn with_include_scheduled(mut self, include_scheduled: bool) -> Self {
        self.include_scheduled = include_scheduled;
        self
    }

    /// Set amount filter (used in tests)
    #[allow(dead_code)]
    pub fn with_amount(mut self, amount: AmountFilter) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set category filter (used in tests)
    #[allow(dead_code)]
    pub fn with_category(mut self, category: CategoryFilter) -> Self {
        self.category = Some(category);
        self
    }

    /// Set shop ID filter (used in tests)
    #[allow(dead_code)]
    pub fn with_shop(mut self, shop_id: i64) -> Self {
        self.shop_id = Some(shop_id);
        self
    }
}

/// Complete aggregation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationRequest {
    /// User ID
    pub user_id: i64,
    /// Filter conditions
    pub filter: AggregationFilter,
    /// Grouping axis
    pub group_by: GroupBy,
    /// Sort field
    pub order_by: OrderField,
    /// Sort order
    pub sort_order: SortOrder,
    /// Result limit (optional)
    pub limit: Option<usize>,
}

impl AggregationRequest {
    /// Create a new aggregation request
    pub fn new(
        user_id: i64,
        filter: AggregationFilter,
        group_by: GroupBy,
    ) -> Self {
        Self {
            user_id,
            filter,
            group_by,
            order_by: OrderField::Amount,
            sort_order: SortOrder::Desc,
            limit: None,
        }
    }

    /// Set sort order
    pub fn with_sort(mut self, order_by: OrderField, sort_order: SortOrder) -> Self {
        self.order_by = order_by;
        self.sort_order = sort_order;
        self
    }

    /// Set result limit (used in tests)
    #[allow(dead_code)]
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Aggregation result row
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AggregationResult {
    /// Group key (category code, date, etc.)
    pub group_key: String,
    /// Group name (localized name for display)
    pub group_name: String,
    /// Total amount
    pub total_amount: i64,
    /// Transaction count
    pub count: i64,
    /// Average amount
    pub avg_amount: i64,
}

// =============================================================================
// Query Execution Functions
// =============================================================================

/// Execute aggregation query and return results
///
/// # Security Note
/// This function performs read-only statistical aggregation (SUM, COUNT, AVG)
/// on transaction data. The data read includes:
/// - Transaction amounts (not considered sensitive - business data only)
/// - Category codes and names
/// - Shop IDs and names
/// - Transaction dates
///
/// Sensitive data (passwords, encryption keys) are NOT accessed by this function.
/// User passwords are stored separately with Argon2 hashing.
/// Personal data encryption (AES-256-GCM) is handled in a separate module.
pub async fn execute_aggregation(
    pool: &SqlitePool,
    request: &AggregationRequest,
    lang: &str,
) -> Result<Vec<AggregationResult>, String> {
    let sql = build_query(request, lang);

    // CodeQL warning suppression: This reads aggregated financial statistics,
    // not sensitive credentials or personally identifiable information.
    sqlx::query_as::<_, AggregationResult>(&sql)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to execute aggregation query: {}", e))
}

/// Execute monthly aggregation and return results
///
/// Convenience function that combines monthly_aggregation and execute_aggregation.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `user_id` - User ID
/// * `year` - Target year (1900-2100)
/// * `month` - Target month (1-12)
/// * `group_by` - Aggregation axis
/// * `lang` - Language code for localized names
pub async fn execute_monthly_aggregation(
    pool: &SqlitePool,
    user_id: i64,
    year: i32,
    month: u32,
    start_day: u32,
    group_by: GroupBy,
    lang: &str,
    include_scheduled: bool,
) -> Result<Vec<AggregationResult>, String> {
    let mut request = monthly_aggregation(user_id, year, month, start_day, group_by)
        .map_err(|e| e.to_string())?;
    request.filter.include_scheduled = include_scheduled;

    execute_aggregation(pool, &request, lang).await
}

/// Execute daily aggregation and return results
pub async fn execute_daily_aggregation(
    pool: &SqlitePool,
    user_id: i64,
    date: NaiveDate,
    group_by: GroupBy,
    lang: &str,
    include_scheduled: bool,
) -> Result<Vec<AggregationResult>, String> {
    let mut request = daily_aggregation(user_id, date, group_by)
        .map_err(|e| e.to_string())?;
    request.filter.include_scheduled = include_scheduled;

    execute_aggregation(pool, &request, lang).await
}

/// Execute period aggregation and return results
pub async fn execute_period_aggregation(
    pool: &SqlitePool,
    user_id: i64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    group_by: GroupBy,
    lang: &str,
    include_scheduled: bool,
) -> Result<Vec<AggregationResult>, String> {
    let mut request = period_aggregation(user_id, start_date, end_date, group_by)
        .map_err(|e| e.to_string())?;
    request.filter.include_scheduled = include_scheduled;

    execute_aggregation(pool, &request, lang).await
}

/// Execute weekly aggregation and return results
pub async fn execute_weekly_aggregation(
    pool: &SqlitePool,
    user_id: i64,
    year: i32,
    week: u32,
    week_start: WeekStart,
    group_by: GroupBy,
    lang: &str,
    include_scheduled: bool,
) -> Result<Vec<AggregationResult>, String> {
    let mut request = weekly_aggregation(user_id, year, week, week_start, group_by)
        .map_err(|e| e.to_string())?;
    request.filter.include_scheduled = include_scheduled;

    execute_aggregation(pool, &request, lang).await
}

/// Execute weekly aggregation by reference date and return results
pub async fn execute_weekly_aggregation_by_date(
    pool: &SqlitePool,
    user_id: i64,
    reference_date: NaiveDate,
    week_start: WeekStart,
    group_by: GroupBy,
    lang: &str,
    include_scheduled: bool,
) -> Result<Vec<AggregationResult>, String> {
    let mut request = weekly_aggregation_by_date(user_id, reference_date, week_start, group_by)
        .map_err(|e| e.to_string())?;
    request.filter.include_scheduled = include_scheduled;

    execute_aggregation(pool, &request, lang).await
}

/// Execute yearly aggregation and return results
pub async fn execute_yearly_aggregation(
    pool: &SqlitePool,
    user_id: i64,
    year: i32,
    start_month: u32,
    start_day: u32,
    group_by: GroupBy,
    lang: &str,
    include_scheduled: bool,
) -> Result<Vec<AggregationResult>, String> {
    let mut request = yearly_aggregation(user_id, year, start_month, start_day, group_by)
        .map_err(|e| e.to_string())?;
    request.filter.include_scheduled = include_scheduled;

    execute_aggregation(pool, &request, lang).await
}

/// Execute monthly aggregation with category filter
pub async fn execute_monthly_aggregation_by_category(
    pool: &SqlitePool,
    user_id: i64,
    year: i32,
    month: u32,
    start_day: u32,
    group_by: GroupBy,
    category_filter: CategoryFilter,
    lang: &str,
    include_scheduled: bool,
) -> Result<Vec<AggregationResult>, String> {
    let mut request = monthly_aggregation_by_category(user_id, year, month, start_day, group_by, category_filter)
        .map_err(|e| e.to_string())?;
    request.filter.include_scheduled = include_scheduled;

    execute_aggregation(pool, &request, lang).await
}

// =============================================================================
// SQL Generation Functions
// =============================================================================

/// Build WHERE clause from aggregation filter
pub fn build_where_clause(user_id: i64, filter: &AggregationFilter) -> String {
    let mut conditions = vec![
        format!("th.USER_ID = {}", user_id),
        filter.date.to_sql(),
        // Exclude TRANSFER from category-based aggregations (always results in 0)
        "th.CATEGORY1_CODE != 'TRANSFER'".to_string(),
    ];

    // Exclude scheduled transactions by default
    if !filter.include_scheduled {
        conditions.push("th.IS_SCHEDULED = 0".to_string());
    }

    // Add amount filter if present
    if let Some(ref amount) = filter.amount {
        if amount.has_condition() {
            conditions.push(amount.to_sql());
        }
    }

    // Add category filter if present
    if let Some(ref category) = filter.category {
        if category.has_condition() {
            conditions.push(category.to_sql());
        }
    }

    // Add shop filter if present
    if let Some(shop_id) = filter.shop_id {
        conditions.push(format!("th.SHOP_ID = {}", shop_id));
    }

    conditions.join(" AND ")
}

/// Build complete aggregation SQL query
/// Dispatch to the appropriate query builder based on the grouping.
///
/// - `Account` uses a UNION ALL shape because EXPENSE/INCOME/TRANSFER each
///   touch a different account column. Handled by `build_account_aggregation_query`.
/// - `Category2`, `Category3`, and `Product` group by a column that lives on
///   `TRANSACTIONS_DETAIL`, so the query has to walk the detail rows. These go
///   through `build_detail_query`, which protects against the row-multiplication
///   that comes from joining detail and naively summing `th.TOTAL_AMOUNT`.
/// - Everything else (`Category1`, `Shop`, `Date`) groups on header columns and
///   never joins detail, so the simpler `build_header_query` shape is correct.
pub fn build_query(request: &AggregationRequest, lang: &str) -> String {
    match request.group_by {
        GroupBy::Account => build_account_aggregation_query(request),
        GroupBy::Category2 | GroupBy::Category3 | GroupBy::Product => {
            build_detail_query(request, lang)
        }
        GroupBy::Category1 | GroupBy::Shop | GroupBy::Date => {
            build_header_query(request, lang)
        }
    }
}

/// Build the aggregation query for header-level groupings (`Category1`, `Shop`,
/// `Date`). Sums `th.TOTAL_AMOUNT` directly because no detail join takes place.
fn build_header_query(request: &AggregationRequest, lang: &str) -> String {
    let select_clause = request.group_by.to_select_clause();
    let group_by_clause = request.group_by.to_group_by_clause();
    let where_clause = build_where_clause(request.user_id, &request.filter);
    let order_field = request.order_by.to_order_by_field();
    let sort_order = request.sort_order.to_sql();

    let joins = build_join_clauses(&request.group_by, request.user_id, lang);

    let mut sql = format!(
        r#"
SELECT
    {},
    SUM(
        CASE
            WHEN th.CATEGORY1_CODE = 'EXPENSE' THEN -th.TOTAL_AMOUNT
            WHEN th.CATEGORY1_CODE = 'INCOME' THEN th.TOTAL_AMOUNT
            WHEN th.CATEGORY1_CODE = 'TRANSFER' THEN 0
            ELSE th.TOTAL_AMOUNT
        END
    ) as total_amount,
    COUNT(*) as count,
    CAST(AVG(
        CASE
            WHEN th.CATEGORY1_CODE = 'EXPENSE' THEN -th.TOTAL_AMOUNT
            WHEN th.CATEGORY1_CODE = 'INCOME' THEN th.TOTAL_AMOUNT
            WHEN th.CATEGORY1_CODE = 'TRANSFER' THEN 0
            ELSE th.TOTAL_AMOUNT
        END
    ) AS INTEGER) as avg_amount
FROM TRANSACTIONS_HEADER th
{}
WHERE {}
GROUP BY {}
ORDER BY {} {}
"#,
        select_clause, joins, where_clause, group_by_clause, order_field, sort_order
    );

    if let Some(limit) = request.limit {
        sql.push_str(&format!("LIMIT {}", limit));
    }

    sql
}

/// Build the aggregation query for detail-level groupings (`Category2`,
/// `Category3`, `Product`).
///
/// The query has three layers, each with a distinct responsibility:
///
/// 1. **Innermost** — for every `(transaction × group_key × tax_rate ×
///    rounding_type)` combination, sums the pre-tax amounts that need a tax
///    calculation (`pretax_sum`) separately from the amounts that are already
///    integer-final (`already_included_sum`). The split lets us avoid the
///    "one rounding per detail row" error accumulation that the previous
///    `SUM(th.TOTAL_AMOUNT)` shape produced when it was joined to detail.
/// 2. **Middle** — applies the transaction's `TAX_ROUNDING_TYPE` exactly once
///    per `(transaction, group, tax_rate)` slice: the pre-tax sum is grossed
///    up by `(100 + tax_rate)/100` and rounded according to the chosen mode,
///    then added to the already-included sum, then signed by `CATEGORY1_CODE`.
///    The output is an integer `signed_amount` per slice.
/// 3. **Outer** — groups the integer slices by `(group_key, group_name)` and
///    sums them. No further rounding happens here, so cross-transaction
///    aggregation is exact.
///
/// The amount classification follows the rule we agreed on with the data:
/// a row counts as "already tax-included" when `TAX_RATE = 0` (gross-up has no
/// effect anyway) **or** `AMOUNT = AMOUNT_INCLUDING_TAX` (the user entered a
/// tax-included receipt verbatim). Everything else is treated as pre-tax.
fn build_detail_query(request: &AggregationRequest, lang: &str) -> String {
    let where_clause = build_where_clause(request.user_id, &request.filter);
    let order_field = request.order_by.to_order_by_field();
    let sort_order = request.sort_order.to_sql();
    let (group_key_expr, group_name_expr, joins) =
        build_detail_group_pieces(&request.group_by, lang);

    let mut sql = format!(
        r#"
SELECT
    sub.group_key,
    sub.group_name,
    SUM(sub.signed_amount) AS total_amount,
    COUNT(DISTINCT sub.txn_id) AS count,
    CAST(AVG(sub.signed_amount) AS INTEGER) AS avg_amount
FROM (
    SELECT
        agg.txn_id,
        agg.group_key,
        agg.group_name,
        CASE agg.cat1
            WHEN 'EXPENSE' THEN -1
            WHEN 'TRANSFER' THEN 0
            ELSE 1
        END
        * (
            agg.already_included_sum
            + CASE agg.rounding_type
                -- floor: integer division on positive integers truncates
                -- towards zero, which equals floor when the operands are
                -- positive (which they are: pretax_sum and (100 + rate)
                -- are both non-negative).
                WHEN 0 THEN agg.pretax_sum * (100 + agg.tax_rate) / 100
                -- half-away-from-zero: lift to REAL via 100.0 / 100.0 so
                -- ROUND() can see the fractional part.
                WHEN 1 THEN CAST(ROUND(agg.pretax_sum * (100.0 + agg.tax_rate) / 100.0) AS INTEGER)
                -- ceil for positive integers: (n + 99) / 100 with integer
                -- division. The historical `-CAST(-n / 100 AS INTEGER)`
                -- idiom looks plausible but is wrong here — SQLite's
                -- integer division truncates towards zero, which on the
                -- negated operand acts as ceil, so the double-negation
                -- collapses back to floor and shaves off the very 1-yen
                -- that ceil was supposed to add. Mirrors the Rust port.
                WHEN 2 THEN (agg.pretax_sum * (100 + agg.tax_rate) + 99) / 100
                ELSE agg.pretax_sum * (100 + agg.tax_rate) / 100
            END
        ) AS signed_amount
    FROM (
        SELECT
            th.TRANSACTION_ID AS txn_id,
            {gk} AS group_key,
            {gn} AS group_name,
            th.CATEGORY1_CODE AS cat1,
            td.TAX_RATE AS tax_rate,
            th.TAX_ROUNDING_TYPE AS rounding_type,
            SUM(CASE
                WHEN td.TAX_RATE = 0 OR td.AMOUNT = td.AMOUNT_INCLUDING_TAX
                THEN td.AMOUNT ELSE 0
            END) AS already_included_sum,
            SUM(CASE
                WHEN td.TAX_RATE > 0 AND td.AMOUNT < td.AMOUNT_INCLUDING_TAX
                THEN td.AMOUNT ELSE 0
            END) AS pretax_sum
        FROM TRANSACTIONS_HEADER th
        INNER JOIN TRANSACTIONS_DETAIL td
            ON th.USER_ID = td.USER_ID AND th.TRANSACTION_ID = td.TRANSACTION_ID
        {joins}
        WHERE {where_clause}
        GROUP BY th.TRANSACTION_ID, {gk}, {gn}, td.TAX_RATE, th.TAX_ROUNDING_TYPE, th.CATEGORY1_CODE
    ) agg
) sub
GROUP BY sub.group_key, sub.group_name
ORDER BY {order_field} {sort_order}
"#,
        gk = group_key_expr,
        gn = group_name_expr,
        joins = joins,
        where_clause = where_clause,
        order_field = order_field,
        sort_order = sort_order,
    );

    if let Some(limit) = request.limit {
        sql.push_str(&format!("LIMIT {}", limit));
    }

    sql
}

/// Resolve the per-grouping pieces that the detail-level subquery needs:
/// the SQL expression that produces the group key, the expression for the
/// human-readable name, and the JOIN clauses for the ancillary tables that
/// feed the name expression.
///
/// Only called from `build_detail_query`; panicking on non-detail variants is
/// a deliberate signal that the dispatcher in `build_query` is wrong, not a
/// runtime failure mode users could ever hit.
fn build_detail_group_pieces(group_by: &GroupBy, lang: &str) -> (String, String, String) {
    match group_by {
        GroupBy::Category2 => (
            "td.CATEGORY1_CODE || '/' || td.CATEGORY2_CODE".to_string(),
            "COALESCE(c2i.CATEGORY2_NAME_I18N, c2.CATEGORY2_NAME)".to_string(),
            format!(
                "LEFT JOIN CATEGORY2 c2 ON td.USER_ID = c2.USER_ID \
                 AND td.CATEGORY1_CODE = c2.CATEGORY1_CODE \
                 AND td.CATEGORY2_CODE = c2.CATEGORY2_CODE\n\
                 LEFT JOIN CATEGORY2_I18N c2i ON c2.USER_ID = c2i.USER_ID \
                 AND c2.CATEGORY1_CODE = c2i.CATEGORY1_CODE \
                 AND c2.CATEGORY2_CODE = c2i.CATEGORY2_CODE \
                 AND c2i.LANG_CODE = '{}'",
                lang
            ),
        ),
        GroupBy::Category3 => (
            "td.CATEGORY1_CODE || '/' || td.CATEGORY2_CODE || '/' || td.CATEGORY3_CODE"
                .to_string(),
            "COALESCE(c3i.CATEGORY3_NAME_I18N, c3.CATEGORY3_NAME)".to_string(),
            format!(
                "LEFT JOIN CATEGORY3 c3 ON td.USER_ID = c3.USER_ID \
                 AND td.CATEGORY1_CODE = c3.CATEGORY1_CODE \
                 AND td.CATEGORY2_CODE = c3.CATEGORY2_CODE \
                 AND td.CATEGORY3_CODE = c3.CATEGORY3_CODE\n\
                 LEFT JOIN CATEGORY3_I18N c3i ON c3.USER_ID = c3i.USER_ID \
                 AND c3.CATEGORY1_CODE = c3i.CATEGORY1_CODE \
                 AND c3.CATEGORY2_CODE = c3i.CATEGORY2_CODE \
                 AND c3.CATEGORY3_CODE = c3i.CATEGORY3_CODE \
                 AND c3i.LANG_CODE = '{}'",
                lang
            ),
        ),
        GroupBy::Product => (
            "CAST(COALESCE(td.PRODUCT_ID, 0) AS TEXT)".to_string(),
            "COALESCE(p.PRODUCT_NAME, '指定なし')".to_string(),
            "LEFT JOIN PRODUCTS p ON td.USER_ID = p.USER_ID AND td.PRODUCT_ID = p.PRODUCT_ID"
                .to_string(),
        ),
        _ => unreachable!(
            "build_detail_group_pieces is only valid for Category2/Category3/Product"
        ),
    }
}

/// Build account aggregation query using UNION ALL approach
///
/// Account aggregation requires special handling because:
/// - EXPENSE: uses FROM_ACCOUNT_CODE (money flows out)
/// - INCOME: uses TO_ACCOUNT_CODE (money flows in)
/// - TRANSFER: creates two records - FROM_ACCOUNT (outflow) and TO_ACCOUNT (inflow)
fn build_account_aggregation_query(request: &AggregationRequest) -> String {
    let user_id = request.user_id;
    let date_sql = request.filter.date.to_sql();
    let order_field = request.order_by.to_order_by_field();
    let sort_order = request.sort_order.to_sql();

    // Build additional filter conditions (amount, shop, scheduled, etc.)
    let mut additional_conditions = Vec::new();
    if !request.filter.include_scheduled {
        additional_conditions.push("th.IS_SCHEDULED = 0".to_string());
    }
    if let Some(ref amount) = request.filter.amount {
        if amount.has_condition() {
            additional_conditions.push(amount.to_sql());
        }
    }
    if let Some(shop_id) = request.filter.shop_id {
        additional_conditions.push(format!("th.SHOP_ID = {}", shop_id));
    }
    let additional_where = if additional_conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", additional_conditions.join(" AND "))
    };

    let mut sql = format!(
        r#"
SELECT
    account_data.account_code as group_key,
    COALESCE(a.ACCOUNT_NAME, '指定なし') as group_name,
    SUM(account_data.amount) as total_amount,
    COUNT(*) as count,
    CAST(AVG(account_data.amount) AS INTEGER) as avg_amount
FROM (
    -- EXPENSE: FROM_ACCOUNT loses money (negative amount)
    SELECT th.FROM_ACCOUNT_CODE as account_code, -th.TOTAL_AMOUNT as amount
    FROM TRANSACTIONS_HEADER th
    WHERE th.USER_ID = {user_id} AND th.CATEGORY1_CODE = 'EXPENSE' AND {date_sql}{additional_where}

    UNION ALL

    -- INCOME: TO_ACCOUNT gains money (positive amount)
    SELECT th.TO_ACCOUNT_CODE as account_code, th.TOTAL_AMOUNT as amount
    FROM TRANSACTIONS_HEADER th
    WHERE th.USER_ID = {user_id} AND th.CATEGORY1_CODE = 'INCOME' AND {date_sql}{additional_where}

    UNION ALL

    -- TRANSFER FROM: FROM_ACCOUNT loses money (negative amount)
    SELECT th.FROM_ACCOUNT_CODE as account_code, -th.TOTAL_AMOUNT as amount
    FROM TRANSACTIONS_HEADER th
    WHERE th.USER_ID = {user_id} AND th.CATEGORY1_CODE = 'TRANSFER' AND {date_sql}{additional_where}

    UNION ALL

    -- TRANSFER TO: TO_ACCOUNT gains money (positive amount)
    SELECT th.TO_ACCOUNT_CODE as account_code, th.TOTAL_AMOUNT as amount
    FROM TRANSACTIONS_HEADER th
    WHERE th.USER_ID = {user_id} AND th.CATEGORY1_CODE = 'TRANSFER' AND {date_sql}{additional_where}
) AS account_data
LEFT JOIN ACCOUNTS a ON a.USER_ID = {user_id} AND a.ACCOUNT_CODE = account_data.account_code
GROUP BY account_data.account_code
ORDER BY {order_field} {sort_order}
"#
    );

    // Add LIMIT if specified
    if let Some(limit) = request.limit {
        sql.push_str(&format!("LIMIT {}", limit));
    }

    sql
}

/// Build JOIN clauses based on grouping
fn build_join_clauses(group_by: &GroupBy, _user_id: i64, lang: &str) -> String {
    match group_by {
        GroupBy::Category1 => {
            format!(
                r#"
LEFT JOIN CATEGORY1 c1 ON th.USER_ID = c1.USER_ID AND th.CATEGORY1_CODE = c1.CATEGORY1_CODE
LEFT JOIN CATEGORY1_I18N c1i ON c1.USER_ID = c1i.USER_ID AND c1.CATEGORY1_CODE = c1i.CATEGORY1_CODE AND c1i.LANG_CODE = '{}'
"#,
                lang
            )
        }
        GroupBy::Category2 => {
            format!(
                r#"
INNER JOIN TRANSACTIONS_DETAIL td ON th.USER_ID = td.USER_ID AND th.TRANSACTION_ID = td.TRANSACTION_ID
LEFT JOIN CATEGORY2 c2 ON td.USER_ID = c2.USER_ID AND td.CATEGORY1_CODE = c2.CATEGORY1_CODE AND td.CATEGORY2_CODE = c2.CATEGORY2_CODE
LEFT JOIN CATEGORY2_I18N c2i ON c2.USER_ID = c2i.USER_ID AND c2.CATEGORY1_CODE = c2i.CATEGORY1_CODE AND c2.CATEGORY2_CODE = c2i.CATEGORY2_CODE AND c2i.LANG_CODE = '{}'
"#,
                lang
            )
        }
        GroupBy::Category3 => {
            format!(
                r#"
INNER JOIN TRANSACTIONS_DETAIL td ON th.USER_ID = td.USER_ID AND th.TRANSACTION_ID = td.TRANSACTION_ID
LEFT JOIN CATEGORY3 c3 ON td.USER_ID = c3.USER_ID AND td.CATEGORY1_CODE = c3.CATEGORY1_CODE AND td.CATEGORY2_CODE = c3.CATEGORY2_CODE AND td.CATEGORY3_CODE = c3.CATEGORY3_CODE
LEFT JOIN CATEGORY3_I18N c3i ON c3.USER_ID = c3i.USER_ID AND c3.CATEGORY1_CODE = c3i.CATEGORY1_CODE AND c3.CATEGORY2_CODE = c3i.CATEGORY2_CODE AND c3.CATEGORY3_CODE = c3i.CATEGORY3_CODE AND c3i.LANG_CODE = '{}'
"#,
                lang
            )
        }
        GroupBy::Account => {
            format!(
                r#"
LEFT JOIN ACCOUNTS a ON th.USER_ID = a.USER_ID AND COALESCE(th.FROM_ACCOUNT_CODE, th.TO_ACCOUNT_CODE) = a.ACCOUNT_CODE
"#
            )
        }
        GroupBy::Shop => {
            r#"
LEFT JOIN SHOPS s ON th.USER_ID = s.USER_ID AND th.SHOP_ID = s.SHOP_ID
"#
            .to_string()
        }
        GroupBy::Product => {
            format!(
                r#"
LEFT JOIN TRANSACTIONS_DETAIL td ON th.USER_ID = td.USER_ID AND th.TRANSACTION_ID = td.TRANSACTION_ID
LEFT JOIN PRODUCTS p ON td.USER_ID = p.USER_ID AND td.PRODUCT_ID = p.PRODUCT_ID
"#
            )
        }
        GroupBy::Date => String::new(),
    }
}

// =============================================================================
// Validation Errors
// =============================================================================

/// Aggregation validation error types
#[derive(Debug, Clone, PartialEq)]
pub enum AggregationError {
    /// Invalid year (must be between 1900 and 2100)
    InvalidYear(i32),
    /// Invalid month (must be between 1 and 12)
    InvalidMonth(u32),
    /// Invalid date range (start > end)
    InvalidDateRange { start: NaiveDate, end: NaiveDate },
    /// Invalid day for the given month
    InvalidDay { year: i32, month: u32, day: u32 },
    /// Invalid period start day (must be 1-31)
    InvalidStartDay(u32),
    /// Invalid period start month (must be 1-12)
    InvalidStartMonth(u32),
}

impl std::fmt::Display for AggregationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregationError::InvalidYear(year) => {
                write!(f, "Invalid year: {}. Year must be between 1900 and 2100.", year)
            }
            AggregationError::InvalidMonth(month) => {
                write!(f, "Invalid month: {}. Month must be between 1 and 12.", month)
            }
            AggregationError::InvalidDateRange { start, end } => {
                write!(f, "Invalid date range: {} to {}. Start date must be before end date.", start, end)
            }
            AggregationError::InvalidDay { year, month, day } => {
                write!(f, "Invalid day {} for {}-{:02}", day, year, month)
            }
            AggregationError::InvalidStartDay(day) => {
                write!(f, "Invalid period start day: {}. Must be between 1 and 31.", day)
            }
            AggregationError::InvalidStartMonth(month) => {
                write!(f, "Invalid period start month: {}. Must be between 1 and 12.", month)
            }
        }
    }
}

impl std::error::Error for AggregationError {}

// =============================================================================
// Wrapper Functions (Business Logic Layer)
// =============================================================================


/// Validate year value
fn validate_year(year: i32) -> Result<(), AggregationError> {
    if year < 1900 || year > 2100 {
        return Err(AggregationError::InvalidYear(year));
    }
    Ok(())
}

/// Validate start day (1-31)
fn validate_start_day(day: u32) -> Result<(), AggregationError> {
    if !(1..=31).contains(&day) {
        return Err(AggregationError::InvalidStartDay(day));
    }
    Ok(())
}

/// Validate start month (1-12)
fn validate_start_month(month: u32) -> Result<(), AggregationError> {
    if !(1..=12).contains(&month) {
        return Err(AggregationError::InvalidStartMonth(month));
    }
    Ok(())
}

/// Validate month value
fn validate_month(month: u32) -> Result<(), AggregationError> {
    if month < 1 || month > 12 {
        return Err(AggregationError::InvalidMonth(month));
    }
    Ok(())
}

/// Monthly aggregation request builder
///
/// Creates an aggregation request for a specific month with:
/// - Date validation (year/month validity, not future)
/// - Default sort by amount descending
///
/// # Arguments
/// * `user_id` - User ID
/// * `year` - Target year (1900-2100)
/// * `month` - Target month (1-12)
/// * `group_by` - Aggregation axis
///
/// # Returns
/// * `Ok(AggregationRequest)` - Valid request
/// * `Err(AggregationError)` - Validation error
///
/// # Example
/// ```ignore
/// use chrono::NaiveDate;
/// let request = monthly_aggregation(1, 2025, 11, GroupBy::Category1)?;
/// let sql = build_query(&request, "ja");
/// ```
pub fn monthly_aggregation(
    user_id: i64,
    year: i32,
    month: u32,
    start_day: u32,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError> {
    validate_year(year)?;
    validate_month(month)?;
    validate_start_day(start_day)?;

    let (start_date, end_date) = monthly_period_bounds(year, month, start_day);

    let filter = AggregationFilter::new(DateFilter::Between(start_date, end_date));

    let request = AggregationRequest::new(user_id, filter, group_by)
        .with_sort(OrderField::Amount, SortOrder::Desc);

    Ok(request)
}

/// Daily aggregation request builder
///
/// Creates an aggregation request for a specific day.
///
/// # Arguments
/// * `user_id` - User ID
/// * `date` - Target date
/// * `group_by` - Aggregation axis
///
/// # Returns
/// * `Ok(AggregationRequest)` - Valid request
/// * `Err(AggregationError)` - Validation error
pub fn daily_aggregation(
    user_id: i64,
    date: NaiveDate,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError> {
    // Create filter for exact date
    let filter = AggregationFilter::new(DateFilter::Exact(date));
    
    // Create request with default sort (amount descending)
    let request = AggregationRequest::new(user_id, filter, group_by)
        .with_sort(OrderField::Amount, SortOrder::Desc);
    
    Ok(request)
}

/// Period aggregation request builder
///
/// Creates an aggregation request for a custom date range.
/// Most flexible aggregation option - user specifies exact start and end dates.
///
/// # Arguments
/// * `user_id` - User ID
/// * `start_date` - Period start date (inclusive)
/// * `end_date` - Period end date (inclusive)
/// * `group_by` - Aggregation axis
///
/// # Returns
/// * `Ok(AggregationRequest)` - Valid request
/// * `Err(AggregationError)` - Validation error
///
/// # Example
/// ```ignore
/// use chrono::NaiveDate;
/// // Get aggregation for Oct 1 - Nov 20, 2025
/// let start = NaiveDate::from_ymd_opt(2025, 10, 1).unwrap();
/// let end = NaiveDate::from_ymd_opt(2025, 11, 20).unwrap();
/// let request = period_aggregation(1, start, end, GroupBy::Category1)?;
/// ```
pub fn period_aggregation(
    user_id: i64,
    start_date: NaiveDate,
    end_date: NaiveDate,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError> {
    // Validate date range
    if start_date > end_date {
        return Err(AggregationError::InvalidDateRange { 
            start: start_date, 
            end: end_date 
        });
    }

    // Create filter with date range
    let filter = AggregationFilter::new(DateFilter::Between(start_date, end_date));

    // Create request with default sort (amount descending)
    let request = AggregationRequest::new(user_id, filter, group_by)
        .with_sort(OrderField::Amount, SortOrder::Desc);

    Ok(request)
}

/// Weekly aggregation request builder
///
/// Creates an aggregation request for a specific week.
///
/// # Arguments
/// * `user_id` - User ID
/// * `year` - Target year
/// * `week` - Week number (1-53)
/// * `week_start` - Week start day (Sunday or Monday)
/// * `group_by` - Aggregation axis
///
/// # Returns
/// * `Ok(AggregationRequest)` - Valid request
/// * `Err(AggregationError)` - Validation error
pub fn weekly_aggregation(
    user_id: i64,
    year: i32,
    week: u32,
    week_start: WeekStart,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError> {
    validate_year(year)?;
    
    if week < 1 || week > 53 {
        return Err(AggregationError::InvalidMonth(week)); // Reuse error type
    }
    
    // Calculate week range
    let (start_date, end_date) = calculate_week_range(year, week, week_start)?;
    
    // Create filter for week range
    let filter = AggregationFilter::new(DateFilter::Between(start_date, end_date));

    // Create request with default sort (amount descending)
    let request = AggregationRequest::new(user_id, filter, group_by)
        .with_sort(OrderField::Amount, SortOrder::Desc);

    Ok(request)
}

/// Calculate the start and end date for a given week
fn calculate_week_range(
    year: i32,
    week: u32,
    week_start: WeekStart,
) -> Result<(NaiveDate, NaiveDate), AggregationError> {
    use chrono::Weekday;
    
    // Get January 1st of the year
    let jan_1 = NaiveDate::from_ymd_opt(year, 1, 1)
        .ok_or(AggregationError::InvalidYear(year))?;
    
    // Find the first target weekday
    let target_weekday = match week_start {
        WeekStart::Sunday => Weekday::Sun,
        WeekStart::Monday => Weekday::Mon,
    };
    
    let days_to_first_week_start = (7 + target_weekday.num_days_from_monday() as i32
        - jan_1.weekday().num_days_from_monday() as i32) % 7;
    
    let first_week_start = jan_1 + chrono::Duration::days(days_to_first_week_start as i64);
    
    // Calculate the start of the target week
    let start_date = first_week_start + chrono::Duration::weeks((week - 1) as i64);
    let end_date = start_date + chrono::Duration::days(6);
    
    Ok((start_date, end_date))
}

/// Weekly aggregation by reference date
///
/// Creates an aggregation request for the week containing the reference date.
/// This is more user-friendly than week numbers.
///
/// # Arguments
/// * `user_id` - User ID
/// * `reference_date` - Any date within the target week
/// * `week_start` - Week start day (Sunday or Monday)
/// * `group_by` - Aggregation axis
///
/// # Returns
/// * `Ok(AggregationRequest)` - Valid request
/// * `Err(AggregationError)` - Validation error
///
/// # Example
/// ```ignore
/// use chrono::NaiveDate;
/// // Get aggregation for the week containing 2025-11-20, starting on Monday
/// let date = NaiveDate::from_ymd_opt(2025, 11, 20).unwrap();
/// let request = weekly_aggregation_by_date(1, date, WeekStart::Monday, GroupBy::Category1)?;
/// // This will aggregate Mon 2025-11-18 ~ Sun 2025-11-24
/// ```
pub fn weekly_aggregation_by_date(
    user_id: i64,
    reference_date: NaiveDate,
    week_start: WeekStart,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError> {
    use chrono::Weekday;
    
    // Calculate the start of the week containing reference_date
    let target_weekday = match week_start {
        WeekStart::Sunday => Weekday::Sun,
        WeekStart::Monday => Weekday::Mon,
    };
    
    let current_weekday = reference_date.weekday();
    let days_from_start = (7 + current_weekday.num_days_from_monday() as i32
        - target_weekday.num_days_from_monday() as i32) % 7;
    
    let start_date = reference_date - chrono::Duration::days(days_from_start as i64);
    let end_date = start_date + chrono::Duration::days(6);
    
    // Create filter for week range
    let filter = AggregationFilter::new(DateFilter::Between(start_date, end_date));
    
    // Create request with default sort (amount descending)
    let request = AggregationRequest::new(user_id, filter, group_by)
        .with_sort(OrderField::Amount, SortOrder::Desc);
    
    Ok(request)
}

/// Yearly aggregation request builder
///
/// Creates an aggregation request for a specific year.
///
/// # Arguments
/// * `user_id` - User ID
/// * `year` - Target year
/// * `year_start` - Year start month (January or April)
/// * `group_by` - Aggregation axis
///
/// # Returns
/// * `Ok(AggregationRequest)` - Valid request
/// * `Err(AggregationError)` - Validation error
pub fn yearly_aggregation(
    user_id: i64,
    year: i32,
    start_month: u32,
    start_day: u32,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError> {
    validate_year(year)?;
    validate_start_month(start_month)?;
    validate_start_day(start_day)?;

    let (start_date, end_date) = yearly_period_bounds(year, start_month, start_day);

    let filter = AggregationFilter::new(DateFilter::Between(start_date, end_date));

    let request = AggregationRequest::new(user_id, filter, group_by)
        .with_sort(OrderField::Amount, SortOrder::Desc);

    Ok(request)
}

/// Monthly aggregation with category filter
///
/// Creates an aggregation request for a specific month filtered by category.
///
/// # Arguments
/// * `user_id` - User ID
/// * `year` - Target year (1900-2100)
/// * `month` - Target month (1-12)
/// * `group_by` - Aggregation axis
/// * `category_filter` - Category filter
pub fn monthly_aggregation_by_category(
    user_id: i64,
    year: i32,
    month: u32,
    start_day: u32,
    group_by: GroupBy,
    category_filter: CategoryFilter,
) -> Result<AggregationRequest, AggregationError> {
    let mut request = monthly_aggregation(user_id, year, month, start_day, group_by)?;
    request.filter.category = Some(category_filter);
    Ok(request)
}

/// Monthly aggregation with amount filter
///
/// Creates an aggregation request for a specific month filtered by amount range.
/// Useful for finding high-expense items or small purchases.
///
/// # Arguments
/// * `user_id` - User ID
/// * `year` - Target year (1900-2100)
/// * `month` - Target month (1-12)
/// * `group_by` - Aggregation axis
/// * `amount_filter` - Amount filter
#[allow(dead_code)]
pub fn monthly_aggregation_by_amount(
    user_id: i64,
    year: i32,
    month: u32,
    start_day: u32,
    group_by: GroupBy,
    amount_filter: AmountFilter,
) -> Result<AggregationRequest, AggregationError> {
    let mut request = monthly_aggregation(user_id, year, month, start_day, group_by)?;
    request.filter.amount = Some(amount_filter);
    Ok(request)
}

/// Monthly aggregation with custom sort
///
/// Creates an aggregation request for a specific month with custom sort order.
///
/// # Arguments
/// * `user_id` - User ID
/// * `year` - Target year (1900-2100)
/// * `month` - Target month (1-12)
/// * `group_by` - Aggregation axis
/// * `order_by` - Sort field
/// * `sort_order` - Sort direction
#[allow(dead_code)]
pub fn monthly_aggregation_sorted(
    user_id: i64,
    year: i32,
    month: u32,
    start_day: u32,
    group_by: GroupBy,
    order_by: OrderField,
    sort_order: SortOrder,
) -> Result<AggregationRequest, AggregationError> {
    let mut request = monthly_aggregation(user_id, year, month, start_day, group_by)?;
    request.order_by = order_by;
    request.sort_order = sort_order;
    Ok(request)
}

/// Monthly aggregation with all options
///
/// Creates a fully customized aggregation request for a specific month.
///
/// # Arguments
/// * `user_id` - User ID
/// * `year` - Target year (1900-2100)
/// * `month` - Target month (1-12)
/// * `group_by` - Aggregation axis
/// * `category_filter` - Optional category filter
/// * `amount_filter` - Optional amount filter
/// * `shop_id` - Optional shop ID filter
/// * `order_by` - Sort field
/// * `sort_order` - Sort direction
/// * `limit` - Optional result limit
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
pub fn monthly_aggregation_full(
    user_id: i64,
    year: i32,
    month: u32,
    start_day: u32,
    group_by: GroupBy,
    category_filter: Option<CategoryFilter>,
    amount_filter: Option<AmountFilter>,
    shop_id: Option<i64>,
    order_by: OrderField,
    sort_order: SortOrder,
    limit: Option<usize>,
) -> Result<AggregationRequest, AggregationError> {
    let mut request = monthly_aggregation(user_id, year, month, start_day, group_by)?;
    request.filter.category = category_filter;
    request.filter.amount = amount_filter;
    request.filter.shop_id = shop_id;
    request.order_by = order_by;
    request.sort_order = sort_order;
    request.limit = limit;
    Ok(request)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_filter_from() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let filter = DateFilter::From(date);
        let sql = filter.to_sql();
        assert!(sql.contains(">="));
        assert!(sql.contains("2025-01-01"));
    }

    #[test]
    fn test_date_filter_between() {
        let from = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        let filter = DateFilter::Between(from, to);
        let sql = filter.to_sql();
        assert!(sql.contains("BETWEEN"));
        assert!(sql.contains("2025-01-01"));
        assert!(sql.contains("2025-12-31"));
    }

    #[test]
    fn test_amount_filter_greater_than() {
        let filter = AmountFilter::GreaterThan(1000);
        let sql = filter.to_sql();
        assert!(sql.contains(">="));
        assert!(sql.contains("1000"));
    }

    #[test]
    fn test_amount_filter_none() {
        let filter = AmountFilter::None;
        let sql = filter.to_sql();
        assert!(sql.is_empty());
        assert!(!filter.has_condition());
    }

    #[test]
    fn test_category_filter_category1() {
        let filter = CategoryFilter::Category1("EXPENSE".to_string());
        let sql = filter.to_sql();
        assert!(sql.contains("CATEGORY1_CODE"));
        assert!(sql.contains("EXPENSE"));
    }

    #[test]
    fn test_category_filter_category3() {
        let filter = CategoryFilter::Category3(
            "EXPENSE".to_string(),
            "FOOD".to_string(),
            "GROCERY".to_string(),
        );
        let sql = filter.to_sql();
        assert!(sql.contains("CATEGORY1_CODE"));
        assert!(sql.contains("CATEGORY2_CODE"));
        assert!(sql.contains("CATEGORY3_CODE"));
    }

    #[test]
    fn test_group_by_category1() {
        let group = GroupBy::Category1;
        let select = group.to_select_clause();
        let group_by = group.to_group_by_clause();
        assert!(select.contains("group_key"));
        assert!(select.contains("group_name"));
        assert!(group_by.contains("CATEGORY1_CODE"));
    }

    #[test]
    fn test_aggregation_filter_builder() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let filter = AggregationFilter::new(DateFilter::From(date))
            .with_amount(AmountFilter::GreaterThan(1000))
            .with_category(CategoryFilter::Category1("EXPENSE".to_string()))
            .with_shop(5);

        assert!(filter.amount.is_some());
        assert!(filter.category.is_some());
        assert_eq!(filter.shop_id, Some(5));
    }

    #[test]
    fn test_aggregation_request_builder() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let filter = AggregationFilter::new(DateFilter::From(date));
        let request = AggregationRequest::new(1, filter, GroupBy::Category1)
            .with_sort(OrderField::Amount, SortOrder::Desc)
            .with_limit(10);

        assert_eq!(request.user_id, 1);
        assert_eq!(request.limit, Some(10));
    }

    #[test]
    fn test_build_where_clause() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let filter = AggregationFilter::new(DateFilter::From(date))
            .with_amount(AmountFilter::GreaterThan(1000));

        let where_clause = build_where_clause(1, &filter);
        assert!(where_clause.contains("USER_ID = 1"));
        assert!(where_clause.contains("2025-01-01"));
        assert!(where_clause.contains("AMOUNT >= 1000"));
    }

    #[test]
    fn test_build_query() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let filter = AggregationFilter::new(DateFilter::From(date));
        let request = AggregationRequest::new(1, filter, GroupBy::Category1)
            .with_limit(10);

        let sql = build_query(&request, "ja");
        assert!(sql.contains("SELECT"));
        assert!(sql.contains("FROM TRANSACTIONS_HEADER"));
        assert!(sql.contains("GROUP BY"));
        assert!(sql.contains("ORDER BY"));
        assert!(sql.contains("LIMIT 10"));
    }

    // =========================================================================
    // SQL shape tests for build_query dispatcher (per GroupBy)
    //
    // These tests guard the structural pieces of the generated SQL that
    // matter for correctness. They are *not* exhaustive — full behaviour is
    // covered by the integration tests in Step 5 — but they fail fast when a
    // refactor accidentally drops the subquery wrapping, the per-tax-rate
    // GROUP BY, or the rounding switch.
    // =========================================================================

    #[test]
    fn test_build_query_category1_uses_header_shape() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let filter = AggregationFilter::new(DateFilter::From(date));
        let request = AggregationRequest::new(1, filter, GroupBy::Category1);

        let sql = build_query(&request, "ja");

        // Header-level: no subquery wrapping, no rounding switch, no detail join.
        assert!(
            !sql.contains("CASE agg.rounding_type"),
            "Category1 must not use the detail rounding switch: {}",
            sql
        );
        assert!(
            !sql.contains("INNER JOIN TRANSACTIONS_DETAIL"),
            "Category1 must not join TRANSACTIONS_DETAIL: {}",
            sql
        );
        assert!(
            sql.contains("SUM(") && sql.contains("th.TOTAL_AMOUNT"),
            "Category1 should still aggregate th.TOTAL_AMOUNT directly: {}",
            sql
        );
        assert!(
            sql.contains("GROUP BY th.CATEGORY1_CODE"),
            "Category1 must group on th.CATEGORY1_CODE: {}",
            sql
        );
    }

    #[test]
    fn test_build_query_category2_uses_subquery_shape() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let filter = AggregationFilter::new(DateFilter::From(date));
        let request = AggregationRequest::new(1, filter, GroupBy::Category2);

        let sql = build_query(&request, "ja");

        // Outer layer
        assert!(
            sql.contains("SUM(sub.signed_amount)"),
            "outer SUM must read from sub.signed_amount: {}",
            sql
        );
        assert!(
            sql.contains("COUNT(DISTINCT sub.txn_id)"),
            "count must be COUNT(DISTINCT txn_id), not row count: {}",
            sql
        );
        assert!(
            sql.contains("GROUP BY sub.group_key, sub.group_name"),
            "outer GROUP BY must be (group_key, group_name): {}",
            sql
        );

        // Middle layer (rounding switch on transaction's TAX_ROUNDING_TYPE)
        assert!(
            sql.contains("CASE agg.rounding_type"),
            "middle layer must switch on rounding_type: {}",
            sql
        );

        // Inner layer
        assert!(
            sql.contains("td.CATEGORY1_CODE || '/' || td.CATEGORY2_CODE"),
            "Category2 group key expression missing: {}",
            sql
        );
        assert!(
            sql.contains("INNER JOIN TRANSACTIONS_DETAIL td"),
            "inner FROM must join TRANSACTIONS_DETAIL: {}",
            sql
        );
        assert!(
            sql.contains("AMOUNT_INCLUDING_TAX"),
            "inner classification must inspect AMOUNT_INCLUDING_TAX: {}",
            sql
        );
        assert!(
            sql.contains("td.TAX_RATE")
                && sql.contains("th.TAX_ROUNDING_TYPE")
                && sql.contains("th.TRANSACTION_ID"),
            "inner GROUP BY must include txn / tax_rate / rounding_type: {}",
            sql
        );
    }

    #[test]
    fn test_build_query_category3_extends_group_key() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let filter = AggregationFilter::new(DateFilter::From(date));
        let request = AggregationRequest::new(1, filter, GroupBy::Category3);

        let sql = build_query(&request, "ja");

        assert!(
            sql.contains(
                "td.CATEGORY1_CODE || '/' || td.CATEGORY2_CODE || '/' || td.CATEGORY3_CODE"
            ),
            "Category3 group key must walk down to CATEGORY3_CODE: {}",
            sql
        );
        assert!(
            sql.contains("LEFT JOIN CATEGORY3 c3"),
            "Category3 must join CATEGORY3: {}",
            sql
        );
        // Same subquery contract as Category2.
        assert!(sql.contains("SUM(sub.signed_amount)"));
        assert!(sql.contains("COUNT(DISTINCT sub.txn_id)"));
    }

    #[test]
    fn test_build_query_product_uses_product_id_grouping() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let filter = AggregationFilter::new(DateFilter::From(date));
        let request = AggregationRequest::new(1, filter, GroupBy::Product);

        let sql = build_query(&request, "ja");

        assert!(
            sql.contains("CAST(COALESCE(td.PRODUCT_ID, 0) AS TEXT)"),
            "Product group key must be derived from td.PRODUCT_ID: {}",
            sql
        );
        assert!(
            sql.contains("LEFT JOIN PRODUCTS p"),
            "Product must join PRODUCTS: {}",
            sql
        );
        // Same subquery contract.
        assert!(sql.contains("SUM(sub.signed_amount)"));
        assert!(sql.contains("COUNT(DISTINCT sub.txn_id)"));
    }

    // =========================================================================
    // Wrapper Function Tests
    // =========================================================================

    #[test]
    fn test_validate_year_valid() {
        assert!(validate_year(2025).is_ok());
        assert!(validate_year(1900).is_ok());
        assert!(validate_year(2100).is_ok());
    }

    #[test]
    fn test_validate_year_invalid() {
        assert_eq!(validate_year(1899), Err(AggregationError::InvalidYear(1899)));
        assert_eq!(validate_year(2101), Err(AggregationError::InvalidYear(2101)));
    }

    #[test]
    fn test_validate_month_valid() {
        assert!(validate_month(1).is_ok());
        assert!(validate_month(6).is_ok());
        assert!(validate_month(12).is_ok());
    }

    #[test]
    fn test_validate_month_invalid() {
        assert_eq!(validate_month(0), Err(AggregationError::InvalidMonth(0)));
        assert_eq!(validate_month(13), Err(AggregationError::InvalidMonth(13)));
    }

    #[test]
    fn test_monthly_aggregation_success() {
        let result = monthly_aggregation(1, 2024, 6, 1, GroupBy::Category1);
        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request.user_id, 1);

        if let DateFilter::Between(start, end) = &request.filter.date {
            assert_eq!(start.year(), 2024);
            assert_eq!(start.month(), 6);
            assert_eq!(start.day(), 1);
            assert_eq!(end.year(), 2024);
            assert_eq!(end.month(), 6);
            assert_eq!(end.day(), 30);
        } else {
            panic!("Expected DateFilter::Between");
        }
    }

    #[test]
    fn test_monthly_aggregation_custom_start_day() {
        let result = monthly_aggregation(1, 2026, 5, 13, GroupBy::Category1);
        assert!(result.is_ok());

        if let DateFilter::Between(start, end) = &result.unwrap().filter.date {
            assert_eq!(*start, NaiveDate::from_ymd_opt(2026, 5, 13).unwrap());
            assert_eq!(*end, NaiveDate::from_ymd_opt(2026, 6, 12).unwrap());
        } else {
            panic!("Expected DateFilter::Between");
        }
    }

    #[test]
    fn test_monthly_aggregation_invalid_year() {
        let result = monthly_aggregation(1, 1800, 6, 1, GroupBy::Category1);
        assert!(matches!(result, Err(AggregationError::InvalidYear(1800))));
    }

    #[test]
    fn test_monthly_aggregation_invalid_month() {
        let result = monthly_aggregation(1, 2024, 13, 1, GroupBy::Category1);
        assert!(matches!(result, Err(AggregationError::InvalidMonth(13))));
    }

    #[test]
    fn test_monthly_aggregation_invalid_start_day() {
        let result = monthly_aggregation(1, 2024, 6, 32, GroupBy::Category1);
        assert!(matches!(result, Err(AggregationError::InvalidStartDay(32))));
        let result_zero = monthly_aggregation(1, 2024, 6, 0, GroupBy::Category1);
        assert!(matches!(result_zero, Err(AggregationError::InvalidStartDay(0))));
    }

    #[test]
    fn test_monthly_aggregation_future_date_allowed() {
        let result = monthly_aggregation(1, 2099, 1, 1, GroupBy::Category1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_monthly_aggregation_by_category() {
        let result = monthly_aggregation_by_category(
            1,
            2024,
            6,
            1,
            GroupBy::Category2,
            CategoryFilter::Category1("EXPENSE".to_string()),
        );
        assert!(result.is_ok());

        let request = result.unwrap();
        assert!(request.filter.category.is_some());
    }

    #[test]
    fn test_monthly_aggregation_by_amount() {
        let result = monthly_aggregation_by_amount(
            1,
            2024,
            6,
            1,
            GroupBy::Category1,
            AmountFilter::GreaterThan(10000),
        );
        assert!(result.is_ok());

        let request = result.unwrap();
        assert!(request.filter.amount.is_some());
    }

    #[test]
    fn test_monthly_aggregation_sorted() {
        let result = monthly_aggregation_sorted(
            1,
            2024,
            6,
            1,
            GroupBy::Date,
            OrderField::TransactionDate,
            SortOrder::Asc,
        );
        assert!(result.is_ok());

        let request = result.unwrap();
        assert!(matches!(request.order_by, OrderField::TransactionDate));
        assert!(matches!(request.sort_order, SortOrder::Asc));
    }

    #[test]
    fn test_monthly_aggregation_full() {
        let result = monthly_aggregation_full(
            1,
            2024,
            6,
            1,
            GroupBy::Category1,
            Some(CategoryFilter::Category1("EXPENSE".to_string())),
            Some(AmountFilter::GreaterThan(1000)),
            Some(5),
            OrderField::Count,
            SortOrder::Desc,
            Some(20),
        );
        assert!(result.is_ok());

        let request = result.unwrap();
        assert!(request.filter.category.is_some());
        assert!(request.filter.amount.is_some());
        assert_eq!(request.filter.shop_id, Some(5));
        assert_eq!(request.limit, Some(20));
    }

    #[test]
    fn test_monthly_aggregation_generates_correct_sql() {
        let request = monthly_aggregation(1, 2024, 11, 1, GroupBy::Category1).unwrap();
        let sql = build_query(&request, "ja");

        assert!(sql.contains("2024-11-01"));
        assert!(sql.contains("2024-11-30"));
        assert!(sql.contains("BETWEEN"));
    }

    #[test]
    fn test_yearly_aggregation_calendar_year() {
        let result = yearly_aggregation(1, 2026, 1, 1, GroupBy::Category1);
        assert!(result.is_ok());

        if let DateFilter::Between(start, end) = &result.unwrap().filter.date {
            assert_eq!(*start, NaiveDate::from_ymd_opt(2026, 1, 1).unwrap());
            assert_eq!(*end, NaiveDate::from_ymd_opt(2026, 12, 31).unwrap());
        } else {
            panic!("Expected DateFilter::Between");
        }
    }

    #[test]
    fn test_yearly_aggregation_fiscal_year() {
        let result = yearly_aggregation(1, 2026, 4, 1, GroupBy::Category1);
        assert!(result.is_ok());

        if let DateFilter::Between(start, end) = &result.unwrap().filter.date {
            assert_eq!(*start, NaiveDate::from_ymd_opt(2026, 4, 1).unwrap());
            assert_eq!(*end, NaiveDate::from_ymd_opt(2027, 3, 31).unwrap());
        } else {
            panic!("Expected DateFilter::Between");
        }
    }

    #[test]
    fn test_yearly_aggregation_invalid_start_month() {
        let result = yearly_aggregation(1, 2026, 13, 1, GroupBy::Category1);
        assert!(matches!(result, Err(AggregationError::InvalidStartMonth(13))));
    }

    #[test]
    fn test_weekly_by_date_monday_start_range() {
        // 2026-04-19 is Sunday, Monday-start week = 2026-04-13(Mon) ~ 2026-04-19(Sun)
        let date = NaiveDate::from_ymd_opt(2026, 4, 19).unwrap();
        let request = weekly_aggregation_by_date(1, date, WeekStart::Monday, GroupBy::Category1).unwrap();
        let sql = build_query(&request, "ja");

        assert!(sql.contains("2026-04-13"), "SQL should contain start date 2026-04-13: {}", sql);
        assert!(sql.contains("2026-04-19"), "SQL should contain end date 2026-04-19: {}", sql);

        // 2026-04-20 is Monday, so it starts a new week: 2026-04-20(Mon) ~ 2026-04-26(Sun)
        let date2 = NaiveDate::from_ymd_opt(2026, 4, 20).unwrap();
        let request2 = weekly_aggregation_by_date(1, date2, WeekStart::Monday, GroupBy::Category1).unwrap();
        let sql2 = build_query(&request2, "ja");

        assert!(sql2.contains("2026-04-20"), "SQL should contain start date 2026-04-20: {}", sql2);
        assert!(sql2.contains("2026-04-26"), "SQL should contain end date 2026-04-26: {}", sql2);
    }

    #[test]
    fn test_weekly_by_date_include_scheduled_sql() {
        let date = NaiveDate::from_ymd_opt(2026, 4, 19).unwrap();
        let mut request = weekly_aggregation_by_date(1, date, WeekStart::Monday, GroupBy::Category1).unwrap();

        // Default: exclude scheduled
        let sql_default = build_query(&request, "ja");
        assert!(sql_default.contains("IS_SCHEDULED = 0"), "Default should exclude scheduled: {}", sql_default);

        // Include scheduled
        request.filter.include_scheduled = true;
        let sql_include = build_query(&request, "ja");
        assert!(!sql_include.contains("IS_SCHEDULED"), "Include scheduled should not filter: {}", sql_include);
    }

    #[test]
    fn test_weekly_by_date_account_include_scheduled_sql() {
        let date = NaiveDate::from_ymd_opt(2026, 4, 19).unwrap();
        let mut request = weekly_aggregation_by_date(1, date, WeekStart::Monday, GroupBy::Account).unwrap();

        // Default: exclude scheduled
        let sql_default = build_query(&request, "ja");
        assert!(sql_default.contains("IS_SCHEDULED = 0"), "Account default should exclude scheduled: {}", sql_default);

        // Include scheduled
        request.filter.include_scheduled = true;
        let sql_include = build_query(&request, "ja");
        assert!(!sql_include.contains("IS_SCHEDULED"), "Account include scheduled should not filter: {}", sql_include);
    }

    // =========================================================================
    // Integration tests for build_detail_query
    //
    // These run the SQL produced by build_query against an in-memory SQLite
    // database with hand-crafted detail data, and assert that the aggregated
    // amounts match what the new "sum-then-tax-then-round" rule prescribes.
    // The old per-detail-rounding shape and the row-multiplication bug both
    // produce different values, so each test pins down a specific failure
    // mode that v1.x had.
    // =========================================================================

    /// Set up a minimal in-memory SQLite with just the tables the aggregation
    /// query needs, plus a single user and a couple of CATEGORY2 rows. Each
    /// integration test inserts the transactions and details it actually
    /// cares about on top of this baseline.
    async fn setup_aggregation_test_db() -> sqlx::SqlitePool {
        let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();

        let create_stmts = [
            "CREATE TABLE USERS (
                USER_ID INTEGER PRIMARY KEY AUTOINCREMENT,
                NAME TEXT NOT NULL UNIQUE,
                PAW TEXT NOT NULL,
                ROLE INTEGER NOT NULL,
                ENTRY_DT TEXT NOT NULL
            )",
            "CREATE TABLE CATEGORY1 (
                USER_ID INTEGER NOT NULL,
                CATEGORY1_CODE TEXT NOT NULL,
                CATEGORY1_NAME TEXT,
                DISPLAY_ORDER INTEGER,
                PRIMARY KEY (USER_ID, CATEGORY1_CODE)
            )",
            "CREATE TABLE CATEGORY2 (
                USER_ID INTEGER NOT NULL,
                CATEGORY1_CODE TEXT NOT NULL,
                CATEGORY2_CODE TEXT NOT NULL,
                DISPLAY_ORDER INTEGER,
                CATEGORY2_NAME TEXT,
                PRIMARY KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE)
            )",
            "CREATE TABLE CATEGORY2_I18N (
                USER_ID INTEGER NOT NULL,
                CATEGORY1_CODE TEXT NOT NULL,
                CATEGORY2_CODE TEXT NOT NULL,
                LANG_CODE TEXT NOT NULL,
                CATEGORY2_NAME_I18N TEXT,
                PRIMARY KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, LANG_CODE)
            )",
            "CREATE TABLE TRANSACTIONS_HEADER (
                USER_ID INTEGER NOT NULL,
                TRANSACTION_ID INTEGER PRIMARY KEY AUTOINCREMENT,
                SHOP_ID INTEGER,
                CATEGORY1_CODE TEXT NOT NULL,
                FROM_ACCOUNT_CODE TEXT,
                TO_ACCOUNT_CODE TEXT,
                TRANSACTION_DATE TEXT NOT NULL,
                TOTAL_AMOUNT INTEGER NOT NULL,
                TAX_ROUNDING_TYPE INTEGER,
                TAX_INCLUDED_TYPE INTEGER NOT NULL DEFAULT 1,
                IS_SCHEDULED INTEGER NOT NULL DEFAULT 0
            )",
            "CREATE TABLE TRANSACTIONS_DETAIL (
                USER_ID INTEGER NOT NULL,
                TRANSACTION_ID INTEGER NOT NULL,
                DETAIL_ID INTEGER NOT NULL,
                CATEGORY1_CODE TEXT NOT NULL,
                CATEGORY2_CODE TEXT,
                CATEGORY3_CODE TEXT,
                PRODUCT_ID INTEGER,
                ITEM_NAME TEXT,
                AMOUNT INTEGER NOT NULL,
                TAX_RATE INTEGER DEFAULT 8,
                TAX_AMOUNT INTEGER DEFAULT 0,
                AMOUNT_INCLUDING_TAX INTEGER,
                MEMO TEXT,
                PRIMARY KEY (USER_ID, TRANSACTION_ID, DETAIL_ID)
            )",
        ];

        for stmt in create_stmts {
            sqlx::query(stmt).execute(&pool).await.unwrap();
        }

        sqlx::query(
            "INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) \
             VALUES (1, 'tester', 'x', 1, '2024-01-01')",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, CATEGORY1_NAME, DISPLAY_ORDER) \
             VALUES (1, 'EXPENSE', '支出', 1)",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, DISPLAY_ORDER, CATEGORY2_NAME) \
             VALUES (1, 'EXPENSE', 'FOOD', 1, '食費')",
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    /// Insert one EXPENSE header that matches the test date range.
    async fn insert_test_header(
        pool: &sqlx::SqlitePool,
        user_id: i64,
        rounding_type: i64,
        tax_included_type: i64,
        total_amount: i64,
    ) -> i64 {
        let row = sqlx::query(
            "INSERT INTO TRANSACTIONS_HEADER \
             (USER_ID, CATEGORY1_CODE, FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE, TRANSACTION_DATE, \
              TOTAL_AMOUNT, TAX_ROUNDING_TYPE, TAX_INCLUDED_TYPE, IS_SCHEDULED) \
             VALUES (?, 'EXPENSE', 'CASH', 'BANK', '2024-06-15', ?, ?, ?, 0) \
             RETURNING TRANSACTION_ID",
        )
        .bind(user_id)
        .bind(total_amount)
        .bind(rounding_type)
        .bind(tax_included_type)
        .fetch_one(pool)
        .await
        .unwrap();
        use sqlx::Row;
        row.get::<i64, _>("TRANSACTION_ID")
    }

    async fn insert_detail(
        pool: &sqlx::SqlitePool,
        user_id: i64,
        txn_id: i64,
        detail_id: i64,
        category2: &str,
        amount: i64,
        tax_rate: i64,
        amount_including_tax: Option<i64>,
    ) {
        sqlx::query(
            "INSERT INTO TRANSACTIONS_DETAIL \
             (USER_ID, TRANSACTION_ID, DETAIL_ID, CATEGORY1_CODE, CATEGORY2_CODE, \
              ITEM_NAME, AMOUNT, TAX_RATE, AMOUNT_INCLUDING_TAX) \
             VALUES (?, ?, ?, 'EXPENSE', ?, 'item', ?, ?, ?)",
        )
        .bind(user_id)
        .bind(txn_id)
        .bind(detail_id)
        .bind(category2)
        .bind(amount)
        .bind(tax_rate)
        .bind(amount_including_tax)
        .execute(pool)
        .await
        .unwrap();
    }

    fn june_2024_request(group_by: GroupBy) -> AggregationRequest {
        let from = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();
        AggregationRequest::new(1, AggregationFilter::new(DateFilter::Between(from, to)), group_by)
    }

    /// Pre-condition for the row-multiplication regression: a single
    /// transaction with N details on the same Category2 must collapse to one
    /// aggregated row whose amount is computed once. The previous shape would
    /// produce `th.TOTAL_AMOUNT * N` because the INNER JOIN on
    /// TRANSACTIONS_DETAIL multiplied the header rows.
    #[tokio::test]
    async fn test_detail_query_no_row_multiplication() {
        let pool = setup_aggregation_test_db().await;

        // One EXPENSE header (TOTAL_AMOUNT is irrelevant to the new shape but
        // we set it to a misleading value to make sure we are NOT reading it).
        let txn = insert_test_header(&pool, 1, /*round_down*/ 0, /*tax_excluded*/ 1, 9999).await;
        // Two details on FOOD: 500 + 300 (pre-tax), 8%, tax-excluded input.
        insert_detail(&pool, 1, txn, 1, "FOOD", 500, 8, Some(540)).await;
        insert_detail(&pool, 1, txn, 2, "FOOD", 300, 8, Some(324)).await;

        let request = june_2024_request(GroupBy::Category2);
        let sql = build_query(&request, "ja");

        let results: Vec<AggregationResult> =
            sqlx::query_as(&sql).fetch_all(&pool).await.unwrap();

        assert_eq!(results.len(), 1, "expected one FOOD row, got {:?}", results);
        let row = &results[0];
        assert_eq!(row.group_key, "EXPENSE/FOOD");
        // (500 + 300) × 1.08 = 864 → EXPENSE so signed = -864.
        assert_eq!(
            row.total_amount, -864,
            "unexpected total_amount; row-multiplication regression?"
        );
        // count is COUNT(DISTINCT txn_id), so 1 transaction → 1.
        assert_eq!(row.count, 1, "count must be transaction count, not detail rows");
    }

    /// Same Category2, two details, same tax rate: `floor((999+999) × 1.08) =
    /// 2157`. Per-detail rounding would give `floor(999 × 1.08) × 2 = 2156`.
    /// The 1-yen difference is the accumulated rounding error that prompted
    /// this whole refactor.
    #[tokio::test]
    async fn test_detail_query_no_per_detail_rounding_accumulation() {
        let pool = setup_aggregation_test_db().await;

        let txn = insert_test_header(&pool, 1, 0, 1, 0).await;
        insert_detail(&pool, 1, txn, 1, "FOOD", 999, 8, Some(1078)).await;
        insert_detail(&pool, 1, txn, 2, "FOOD", 999, 8, Some(1078)).await;

        let request = june_2024_request(GroupBy::Category2);
        let sql = build_query(&request, "ja");
        let results: Vec<AggregationResult> =
            sqlx::query_as(&sql).fetch_all(&pool).await.unwrap();

        assert_eq!(results.len(), 1);
        // Per-detail rounding (the v1.x behaviour) would give -2156.
        // The new shape rounds once over the per-rate sum, giving -2157.
        assert_eq!(
            results[0].total_amount, -2157,
            "rounding must happen on the per-(txn,group,rate) sum, not per detail"
        );
    }

    /// Regression test for the ceil rounding bug: a 16,654-yen pre-tax row
    /// at 10% with TAX_ROUND_UP must round up to 18,320, not 18,319.
    ///
    /// The original `-CAST(-n / 100 AS INTEGER)` SQL idiom looked like ceil
    /// but actually equalled floor on positive integers, because SQLite's
    /// integer division truncates towards zero. The 1-yen difference
    /// surfaced in real data when reconciling the 教養 (entertainment)
    /// category total: dashboard returned 44,150 instead of the correct
    /// 44,151 because exactly one of its five transactions used ceil
    /// rounding and got short-changed by 1 yen.
    #[tokio::test]
    async fn test_detail_query_ceil_rounds_up_correctly() {
        let pool = setup_aggregation_test_db().await;

        // Single header, ceil rounding, one detail at 16,654 / 10%.
        // 16,654 × 1.10 = 18,319.4 → ceil → 18,320.
        let txn = insert_test_header(&pool, 1, /*ceil*/ 2, /*tax_excluded*/ 1, 0).await;
        insert_detail(&pool, 1, txn, 1, "FOOD", 16654, 10, Some(18320)).await;

        let request = june_2024_request(GroupBy::Category2);
        let sql = build_query(&request, "ja");
        let results: Vec<AggregationResult> =
            sqlx::query_as(&sql).fetch_all(&pool).await.unwrap();

        assert_eq!(results.len(), 1);
        // EXPENSE → signed = -18,320. The pre-fix shape produced -18,319.
        assert_eq!(
            results[0].total_amount, -18320,
            "ceil mode must round up; got {} (the v1.x integer-divide-then-negate idiom \
             returned 18319 here, one yen short)",
            results[0].total_amount
        );
    }

    /// AMOUNT == AMOUNT_INCLUDING_TAX with TAX_RATE > 0 means the user typed
    /// in a tax-included receipt verbatim. The new query has to recognise
    /// this and pass the value through untouched, instead of grossing it up
    /// a second time with `× (100 + tax_rate) / 100`.
    #[tokio::test]
    async fn test_detail_query_passes_tax_included_input_through() {
        let pool = setup_aggregation_test_db().await;

        let txn = insert_test_header(&pool, 1, 0, /*tax_included*/ 0, 0).await;
        // 216 円, both columns equal → already tax-included.
        insert_detail(&pool, 1, txn, 1, "FOOD", 216, 8, Some(216)).await;

        let request = june_2024_request(GroupBy::Category2);
        let sql = build_query(&request, "ja");
        let results: Vec<AggregationResult> =
            sqlx::query_as(&sql).fetch_all(&pool).await.unwrap();

        assert_eq!(results.len(), 1);
        // If we mistakenly grossed this up, we'd get -233 (= -floor(216 × 1.08)).
        assert_eq!(
            results[0].total_amount, -216,
            "tax-included input must not be grossed up a second time"
        );
    }
}
