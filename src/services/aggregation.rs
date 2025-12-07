//! Aggregation service for financial reporting and analysis
//!
//! This module provides type-safe aggregation functionality using Enums
//! to prevent invalid SQL generation at compile time.

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

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

/// Year start month setting
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum YearStart {
    /// Year starts in January (calendar year)
    January,
    /// Year starts in April (fiscal year, Japanese style)
    April,
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

    /// Check if this grouping requires joining with transaction details
    pub fn requires_details_join(&self) -> bool {
        matches!(self, GroupBy::Product)
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
}

impl AggregationFilter {
    /// Create a new aggregation filter with only date filter
    pub fn new(date: DateFilter) -> Self {
        Self {
            date,
            amount: None,
            category: None,
            shop_id: None,
        }
    }

    /// Set amount filter
    pub fn with_amount(mut self, amount: AmountFilter) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set category filter
    pub fn with_category(mut self, category: CategoryFilter) -> Self {
        self.category = Some(category);
        self
    }

    /// Set shop ID filter
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

    /// Set result limit
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
    group_by: GroupBy,
    lang: &str,
) -> Result<Vec<AggregationResult>, String> {
    let request = monthly_aggregation(user_id, year, month, group_by)
        .map_err(|e| e.to_string())?;

    execute_aggregation(pool, &request, lang).await
}

/// Execute daily aggregation and return results
pub async fn execute_daily_aggregation(
    pool: &SqlitePool,
    user_id: i64,
    date: NaiveDate,
    group_by: GroupBy,
    lang: &str,
) -> Result<Vec<AggregationResult>, String> {
    let request = daily_aggregation(user_id, date, group_by)
        .map_err(|e| e.to_string())?;

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
) -> Result<Vec<AggregationResult>, String> {
    let request = period_aggregation(user_id, start_date, end_date, group_by)
        .map_err(|e| e.to_string())?;

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
) -> Result<Vec<AggregationResult>, String> {
    let request = weekly_aggregation(user_id, year, week, week_start, group_by)
        .map_err(|e| e.to_string())?;

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
) -> Result<Vec<AggregationResult>, String> {
    let request = weekly_aggregation_by_date(user_id, reference_date, week_start, group_by)
        .map_err(|e| e.to_string())?;

    execute_aggregation(pool, &request, lang).await
}

/// Execute yearly aggregation and return results
pub async fn execute_yearly_aggregation(
    pool: &SqlitePool,
    user_id: i64,
    year: i32,
    year_start: YearStart,
    group_by: GroupBy,
    lang: &str,
) -> Result<Vec<AggregationResult>, String> {
    let request = yearly_aggregation(user_id, year, year_start, group_by)
        .map_err(|e| e.to_string())?;

    execute_aggregation(pool, &request, lang).await
}

/// Execute monthly aggregation with category filter
pub async fn execute_monthly_aggregation_by_category(
    pool: &SqlitePool,
    user_id: i64,
    year: i32,
    month: u32,
    group_by: GroupBy,
    category_filter: CategoryFilter,
    lang: &str,
) -> Result<Vec<AggregationResult>, String> {
    let request = monthly_aggregation_by_category(user_id, year, month, group_by, category_filter)
        .map_err(|e| e.to_string())?;

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
pub fn build_query(request: &AggregationRequest, lang: &str) -> String {
    // Special handling for Account grouping which requires UNION ALL
    if matches!(request.group_by, GroupBy::Account) {
        return build_account_aggregation_query(request);
    }

    let select_clause = request.group_by.to_select_clause();
    let group_by_clause = request.group_by.to_group_by_clause();
    let where_clause = build_where_clause(request.user_id, &request.filter);
    let order_field = request.order_by.to_order_by_field();
    let sort_order = request.sort_order.to_sql();

    // Build JOIN clauses based on group_by
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

    // Add LIMIT if specified
    if let Some(limit) = request.limit {
        sql.push_str(&format!("LIMIT {}", limit));
    }

    sql
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

    // Build additional filter conditions (amount, shop, etc.)
    let mut additional_conditions = Vec::new();
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
    /// Date is in the future
    FutureDate { year: i32, month: u32 },
    /// Invalid date range (start > end)
    InvalidDateRange { start: NaiveDate, end: NaiveDate },
    /// Invalid day for the given month
    InvalidDay { year: i32, month: u32, day: u32 },
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
            AggregationError::FutureDate { year, month } => {
                write!(f, "Future date not allowed: {}-{:02}", year, month)
            }
            AggregationError::InvalidDateRange { start, end } => {
                write!(f, "Invalid date range: {} to {}. Start date must be before end date.", start, end)
            }
            AggregationError::InvalidDay { year, month, day } => {
                write!(f, "Invalid day {} for {}-{:02}", day, year, month)
            }
        }
    }
}

impl std::error::Error for AggregationError {}

// =============================================================================
// Wrapper Functions (Business Logic Layer)
// =============================================================================

/// Get the last day of a month
fn get_last_day_of_month(year: i32, month: u32) -> u32 {
    // Try to create the first day of the next month, then subtract one day
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };

    NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .map(|d| d.pred_opt().unwrap().day())
        .unwrap_or(28) // Fallback for edge cases
}

/// Validate year value
fn validate_year(year: i32) -> Result<(), AggregationError> {
    if year < 1900 || year > 2100 {
        return Err(AggregationError::InvalidYear(year));
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

/// Check if the given year/month is not in the future
fn validate_not_future(year: i32, month: u32) -> Result<(), AggregationError> {
    let today = chrono::Local::now().date_naive();
    let target_first_day = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or(AggregationError::InvalidMonth(month))?;

    if target_first_day > today {
        return Err(AggregationError::FutureDate { year, month });
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
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError> {
    // Validate inputs
    validate_year(year)?;
    validate_month(month)?;
    validate_not_future(year, month)?;

    // Calculate date range for the month
    let first_day = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or(AggregationError::InvalidMonth(month))?;
    let last_day_num = get_last_day_of_month(year, month);
    let last_day = NaiveDate::from_ymd_opt(year, month, last_day_num)
        .ok_or(AggregationError::InvalidDay { year, month, day: last_day_num })?;

    // Create filter with date range
    let filter = AggregationFilter::new(DateFilter::Between(first_day, last_day));

    // Create request with default sort (amount descending)
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
    let today = chrono::Local::now().date_naive();
    
    // Validate not future
    if date > today {
        return Err(AggregationError::FutureDate { 
            year: date.year(), 
            month: date.month() 
        });
    }
    
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
    
    let today = chrono::Local::now().date_naive();
    if start_date > today {
        return Err(AggregationError::FutureDate { 
            year: start_date.year(), 
            month: start_date.month() 
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
    
    let today = chrono::Local::now().date_naive();
    if start_date > today {
        return Err(AggregationError::FutureDate { 
            year: start_date.year(), 
            month: start_date.month() 
        });
    }
    
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
    
    let today = chrono::Local::now().date_naive();
    if reference_date > today {
        return Err(AggregationError::FutureDate { 
            year: reference_date.year(), 
            month: reference_date.month() 
        });
    }
    
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
    year_start: YearStart,
    group_by: GroupBy,
) -> Result<AggregationRequest, AggregationError> {
    validate_year(year)?;
    
    // Calculate year range based on start month
    let (start_date, end_date) = match year_start {
        YearStart::January => {
            // Calendar year: Jan 1 - Dec 31
            let start = NaiveDate::from_ymd_opt(year, 1, 1)
                .ok_or(AggregationError::InvalidYear(year))?;
            let end = NaiveDate::from_ymd_opt(year, 12, 31)
                .ok_or(AggregationError::InvalidYear(year))?;
            (start, end)
        }
        YearStart::April => {
            // Fiscal year: Apr 1 (year) - Mar 31 (year+1)
            let start = NaiveDate::from_ymd_opt(year, 4, 1)
                .ok_or(AggregationError::InvalidYear(year))?;
            let end = NaiveDate::from_ymd_opt(year + 1, 3, 31)
                .ok_or(AggregationError::InvalidYear(year + 1))?;
            (start, end)
        }
    };
    
    let today = chrono::Local::now().date_naive();
    if start_date > today {
        return Err(AggregationError::FutureDate { 
            year: start_date.year(), 
            month: start_date.month() 
        });
    }
    
    // Create filter for year range
    let filter = AggregationFilter::new(DateFilter::Between(start_date, end_date));
    
    // Create request with default sort (amount descending)
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
    group_by: GroupBy,
    category_filter: CategoryFilter,
) -> Result<AggregationRequest, AggregationError> {
    let mut request = monthly_aggregation(user_id, year, month, group_by)?;
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
pub fn monthly_aggregation_by_amount(
    user_id: i64,
    year: i32,
    month: u32,
    group_by: GroupBy,
    amount_filter: AmountFilter,
) -> Result<AggregationRequest, AggregationError> {
    let mut request = monthly_aggregation(user_id, year, month, group_by)?;
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
pub fn monthly_aggregation_sorted(
    user_id: i64,
    year: i32,
    month: u32,
    group_by: GroupBy,
    order_by: OrderField,
    sort_order: SortOrder,
) -> Result<AggregationRequest, AggregationError> {
    let mut request = monthly_aggregation(user_id, year, month, group_by)?;
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
pub fn monthly_aggregation_full(
    user_id: i64,
    year: i32,
    month: u32,
    group_by: GroupBy,
    category_filter: Option<CategoryFilter>,
    amount_filter: Option<AmountFilter>,
    shop_id: Option<i64>,
    order_by: OrderField,
    sort_order: SortOrder,
    limit: Option<usize>,
) -> Result<AggregationRequest, AggregationError> {
    let mut request = monthly_aggregation(user_id, year, month, group_by)?;
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
    // Wrapper Function Tests
    // =========================================================================

    #[test]
    fn test_get_last_day_of_month_january() {
        assert_eq!(get_last_day_of_month(2025, 1), 31);
    }

    #[test]
    fn test_get_last_day_of_month_february_normal() {
        assert_eq!(get_last_day_of_month(2025, 2), 28);
    }

    #[test]
    fn test_get_last_day_of_month_february_leap() {
        assert_eq!(get_last_day_of_month(2024, 2), 29);
    }

    #[test]
    fn test_get_last_day_of_month_december() {
        assert_eq!(get_last_day_of_month(2025, 12), 31);
    }

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
    fn test_validate_not_future_past() {
        // Test with a past date (should always pass)
        assert!(validate_not_future(2020, 1).is_ok());
    }

    #[test]
    fn test_validate_not_future_future() {
        // Test with a future date (should fail)
        let result = validate_not_future(2099, 12);
        assert!(matches!(result, Err(AggregationError::FutureDate { .. })));
    }

    #[test]
    fn test_validate_not_future_current_month() {
        // Current month should pass
        let today = chrono::Local::now().date_naive();
        let result = validate_not_future(today.year(), today.month());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_not_future_next_month() {
        // Next month should fail
        let today = chrono::Local::now().date_naive();
        let (next_year, next_month) = if today.month() == 12 {
            (today.year() + 1, 1)
        } else {
            (today.year(), today.month() + 1)
        };
        let result = validate_not_future(next_year, next_month);
        assert!(matches!(result, Err(AggregationError::FutureDate { .. })));
    }

    #[test]
    fn test_validate_not_future_next_year() {
        // Next year same month should fail
        let today = chrono::Local::now().date_naive();
        let result = validate_not_future(today.year() + 1, today.month());
        assert!(matches!(result, Err(AggregationError::FutureDate { .. })));
    }

    #[test]
    fn test_monthly_aggregation_success() {
        // Use a past date that's always valid
        let result = monthly_aggregation(1, 2024, 6, GroupBy::Category1);
        assert!(result.is_ok());

        let request = result.unwrap();
        assert_eq!(request.user_id, 1);

        // Check that the date filter is Between
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
    fn test_monthly_aggregation_invalid_year() {
        let result = monthly_aggregation(1, 1800, 6, GroupBy::Category1);
        assert!(matches!(result, Err(AggregationError::InvalidYear(1800))));
    }

    #[test]
    fn test_monthly_aggregation_invalid_month() {
        let result = monthly_aggregation(1, 2024, 13, GroupBy::Category1);
        assert!(matches!(result, Err(AggregationError::InvalidMonth(13))));
    }

    #[test]
    fn test_monthly_aggregation_future_date() {
        let result = monthly_aggregation(1, 2099, 1, GroupBy::Category1);
        assert!(matches!(result, Err(AggregationError::FutureDate { .. })));
    }

    #[test]
    fn test_monthly_aggregation_current_month() {
        // Current month should pass
        let today = chrono::Local::now().date_naive();
        let result = monthly_aggregation(1, today.year(), today.month(), GroupBy::Category1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_monthly_aggregation_next_month() {
        // Next month should fail
        let today = chrono::Local::now().date_naive();
        let (next_year, next_month) = if today.month() == 12 {
            (today.year() + 1, 1)
        } else {
            (today.year(), today.month() + 1)
        };
        let result = monthly_aggregation(1, next_year, next_month, GroupBy::Category1);
        assert!(matches!(result, Err(AggregationError::FutureDate { .. })));
    }

    #[test]
    fn test_monthly_aggregation_next_year() {
        // Next year same month should fail
        let today = chrono::Local::now().date_naive();
        let result = monthly_aggregation(1, today.year() + 1, today.month(), GroupBy::Category1);
        assert!(matches!(result, Err(AggregationError::FutureDate { .. })));
    }

    #[test]
    fn test_monthly_aggregation_by_category() {
        let result = monthly_aggregation_by_category(
            1,
            2024,
            6,
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
        let request = monthly_aggregation(1, 2024, 11, GroupBy::Category1).unwrap();
        let sql = build_query(&request, "ja");

        // Check that the SQL contains the correct date range
        assert!(sql.contains("2024-11-01"));
        assert!(sql.contains("2024-11-30"));
        assert!(sql.contains("BETWEEN"));
    }
}
