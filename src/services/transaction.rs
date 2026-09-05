use sqlx::{SqlitePool, Row};
use serde::{Serialize, Deserialize};
use crate::api_error::ApiError;
use crate::{sql_queries, consts, validation};

const ENTITY_LABEL: &str = "Transaction";

/// Upper bound for page size in paginated transaction listings
const MAX_PER_PAGE: i64 = 500;

/// Transaction header data structure
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransactionHeader {
    #[sqlx(rename = "TRANSACTION_ID")]
    pub transaction_id: i64,
    #[sqlx(rename = "USER_ID")]
    pub user_id: i64,
    #[sqlx(rename = "SHOP_ID")]
    pub shop_id: Option<i64>,
    #[sqlx(rename = "TRANSACTION_DATE")]
    pub transaction_date: String,
    #[sqlx(rename = "CATEGORY1_CODE")]
    pub category1_code: String,
    #[sqlx(rename = "FROM_ACCOUNT_CODE")]
    pub from_account_code: String,
    #[sqlx(rename = "TO_ACCOUNT_CODE")]
    pub to_account_code: String,
    #[sqlx(rename = "TOTAL_AMOUNT")]
    pub total_amount: i64,
    #[sqlx(rename = "TAX_ROUNDING_TYPE")]
    pub tax_rounding_type: i64,
    #[sqlx(rename = "TAX_INCLUDED_TYPE")]
    pub tax_included_type: i64,
    #[sqlx(rename = "MEMO_ID")]
    pub memo_id: Option<i64>,
    #[sqlx(rename = "IS_DISABLED")]
    pub is_disabled: i64,
    #[sqlx(rename = "IS_SCHEDULED")]
    pub is_scheduled: i64,
    #[sqlx(rename = "ENTRY_DT")]
    pub entry_dt: String,
    #[sqlx(rename = "UPDATE_DT")]
    pub update_dt: Option<String>,
}

/// Request structure for saving transaction header
#[derive(Debug, Deserialize, Clone)]
pub struct SaveTransactionRequest {
    pub shop_id: Option<i64>,
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub transaction_date: String,
    pub total_amount: i64,
    pub tax_rounding_type: i64,
    pub tax_included_type: i64,
    pub memo: Option<String>,
    pub is_scheduled: Option<i64>,
}

/// Transaction detail data structure
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransactionDetail {
    #[sqlx(rename = "DETAIL_ID")]
    pub detail_id: i64,
    #[sqlx(rename = "TRANSACTION_ID")]
    pub transaction_id: i64,
    #[sqlx(rename = "USER_ID")]
    pub user_id: i64,
    #[sqlx(rename = "CATEGORY1_CODE")]
    pub category1_code: String,
    #[sqlx(rename = "CATEGORY2_CODE")]
    pub category2_code: Option<String>,
    #[sqlx(rename = "CATEGORY3_CODE")]
    pub category3_code: Option<String>,
    #[sqlx(rename = "ITEM_NAME")]
    pub item_name: String,
    #[sqlx(rename = "AMOUNT")]
    pub amount: i64,
    #[sqlx(rename = "TAX_AMOUNT")]
    pub tax_amount: i64,
    #[sqlx(rename = "TAX_RATE")]
    pub tax_rate: i32,
    #[sqlx(rename = "AMOUNT_INCLUDING_TAX")]
    pub amount_including_tax: Option<i64>,
    #[sqlx(rename = "PRODUCT_ID")]
    pub product_id: Option<i64>,
    #[sqlx(rename = "MEMO_ID")]
    pub memo_id: Option<i64>,
    #[sqlx(rename = "ENTRY_DT")]
    pub entry_dt: String,
    #[sqlx(rename = "UPDATE_DT")]
    pub update_dt: Option<String>,
}

/// Request structure for saving transaction detail
#[derive(Debug, Deserialize, Clone)]
pub struct SaveTransactionDetailRequest {
    pub detail_id: Option<i64>,
    pub category1_code: String,
    pub category2_code: Option<String>,
    pub category3_code: Option<String>,
    pub item_name: String,
    pub amount: i64,
    pub tax_rate: i32,
    pub tax_amount: i64,
    pub amount_including_tax: Option<i64>,
    #[serde(default)]
    pub product_id: Option<i64>,
    pub memo: Option<String>,
}

/// Transaction detail with related information for display
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionDetailWithInfo {
    pub detail_id: i64,
    pub transaction_id: i64,
    pub user_id: i64,
    pub category1_code: String,
    pub category2_code: Option<String>,
    pub category3_code: Option<String>,
    pub category1_name: Option<String>,
    pub category2_name: Option<String>,
    pub category3_name: Option<String>,
    pub item_name: String,
    pub amount: i64,
    pub tax_amount: i64,
    pub tax_rate: i32,
    pub amount_including_tax: Option<i64>,
    pub product_id: Option<i64>,
    pub product_name: Option<String>,
    pub manufacturer_name: Option<String>,
    pub memo_id: Option<i64>,
    pub memo_text: Option<String>,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

/// Transaction header with related information for display
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionHeaderWithInfo {
    pub transaction_id: i64,
    pub user_id: i64,
    pub shop_id: Option<i64>,
    pub shop_name: Option<String>,
    pub transaction_date: String,
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub from_account_name: Option<String>,
    pub to_account_name: Option<String>,
    pub total_amount: i64,
    pub tax_rounding_type: i64,
    pub tax_included_type: i64,
    pub memo_id: Option<i64>,
    pub memo_text: Option<String>,
    pub is_disabled: i64,
    pub is_scheduled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

/// Transaction data structure for list display (header-based)
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    #[sqlx(rename = "TRANSACTION_ID")]
    pub transaction_id: i64,
    #[sqlx(rename = "USER_ID")]
    pub user_id: i64,
    #[sqlx(rename = "TRANSACTION_DATE")]
    pub transaction_date: String,  // YYYY-MM-DD HH:MM:SS format (datetime)
    #[sqlx(rename = "CATEGORY1_CODE")]
    pub category1_code: String,
    #[sqlx(rename = "FROM_ACCOUNT_CODE")]
    pub from_account_code: String,
    #[sqlx(rename = "TO_ACCOUNT_CODE")]
    pub to_account_code: String,
    #[sqlx(rename = "TOTAL_AMOUNT")]
    pub total_amount: i64,
    #[sqlx(rename = "TAX_ROUNDING_TYPE")]
    pub tax_rounding_type: i64,
    #[sqlx(rename = "MEMO_ID")]
    pub memo_id: Option<i64>,
    #[sqlx(rename = "IS_SCHEDULED")]
    pub is_scheduled: i64,
    #[sqlx(rename = "CATEGORY1_NAME")]
    pub category1_name: Option<String>,
    #[sqlx(rename = "FROM_ACCOUNT_NAME")]
    pub from_account_name: Option<String>,
    #[sqlx(rename = "TO_ACCOUNT_NAME")]
    pub to_account_name: Option<String>,
    #[sqlx(rename = "MEMO_TEXT")]
    pub memo_text: Option<String>,
}

/// Transaction list response with pagination
#[derive(Debug, Serialize)]
pub struct TransactionListResponse {
    pub transactions: Vec<Transaction>,
    pub total_count: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

/// Transaction service for managing income/expense data
pub struct TransactionService {
    pool: SqlitePool,
}

#[derive(Debug)]
pub enum TransactionError {
    DatabaseError(String),
    ValidationError(String),
    NotFound,
    /// Fable-5 review #20 (CodeRabbit on #127) — TRANSFER was submitted
    /// with `from_account_code == to_account_code`. Kept as its own
    /// variant (rather than a generic `ValidationError`) so the
    /// `From<TransactionError> for ApiError` bridge can map it to a
    /// dedicated `transfer_same_account` code the frontend renders
    /// with an i18n key, instead of leaking the raw English message
    /// through `formatApiError` on a ja-JP UI.
    TransferSameAccount,
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            TransactionError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            TransactionError::NotFound => write!(f, "Transaction not found"),
            TransactionError::TransferSameAccount => {
                write!(f, "Transfer source and destination accounts must be different")
            }
        }
    }
}

impl std::error::Error for TransactionError {}

impl From<sqlx::Error> for TransactionError {
    fn from(err: sqlx::Error) -> Self {
        TransactionError::DatabaseError(err.to_string())
    }
}

/// Map the domain-specific `TransactionError` onto the wire-level `ApiError`
/// so the tauri command wrappers can `?`-propagate it into a structured
/// `{ code, message, entity? }` payload for the frontend classifier
/// (`res/js/transaction-management.js` / `transaction-detail-management.js`
/// — direct `err.code` branching, no substring matches). Matches the
/// `From<RecurringError>` shape (PR2a) and the earlier
/// `From<CategoryError>` / `From<UserManagementError>` rollouts
/// (PR #100/#101).
///
/// Codes:
///   - `NotFound`              → `not_found` (entity="transaction")
///   - `ValidationError(msg)`  → `validation` (message preserved so the
///                               screens can still dispatch bounded-field
///                               errors — `"Item name must be …"`,
///                               `"Memo must be …"` — to the right inline
///                               input via `startsWith`)
///   - `DatabaseError(msg)`    → `database`
impl From<TransactionError> for ApiError {
    fn from(err: TransactionError) -> Self {
        match err {
            TransactionError::NotFound => ApiError::not_found(ENTITY_LABEL),
            TransactionError::ValidationError(msg) => ApiError::validation(msg),
            TransactionError::DatabaseError(msg) => ApiError::database(msg),
            TransactionError::TransferSameAccount => ApiError::transfer_same_account(),
        }
    }
}

/// MEMOS.MEMO_TEXT length guard, shared by the header and detail paths.
fn validate_memo_length(memo_text: &str) -> Result<(), TransactionError> {
    validation::validate_max_chars("Memo", memo_text, consts::MAX_MEMO_LEN)
        .map_err(TransactionError::ValidationError)
}

/// TRANSACTIONS_DETAIL.ITEM_NAME length guard.
fn validate_item_name_length(item_name: &str) -> Result<(), TransactionError> {
    validation::validate_max_chars("Item name", item_name, consts::MAX_ITEM_NAME_LEN)
        .map_err(TransactionError::ValidationError)
}

/// Escape SQL LIKE metacharacters so user-supplied text matches literally.
/// Paired with `LIKE ? ESCAPE '\'` in the query. Backslash must be escaped
/// first so we do not re-escape the escapes we just added.
fn escape_like_pattern(s: &str) -> String {
    s.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")
}

/// Result of `recalculate_all_transaction_totals`. The frontend uses this to
/// tell the user how much work was actually done and where the safety-net
/// backup ended up, so they can roll back later from a single button click.
#[derive(Debug, Serialize, Deserialize)]
pub struct RecalcSummary {
    /// Number of headers the recalc walked.
    pub total_headers: i64,
    /// Headers whose `TAX_ROUNDING_TYPE` / `TAX_INCLUDED_TYPE` were corrected
    /// to a pattern that matched the existing `TOTAL_AMOUNT`. The total is
    /// preserved verbatim because the user-entered value is the source of
    /// truth for these legacy rows.
    pub settings_corrected: i64,
    /// Headers where no pattern matched the existing `TOTAL_AMOUNT`, so the
    /// total was overwritten with the value computed from the existing
    /// rounding/included settings.
    pub total_overwritten: i64,
    /// Headers that already match: existing settings produce the existing
    /// total. No write happened.
    pub skipped: i64,
    /// Absolute path of the file backup taken before the recalculation
    /// started. Returned to the frontend so the rollback flow can pass it
    /// straight back to `restore_totals_from_backup`.
    pub backup_path: String,
    /// Per-header change log, ordered by `TRANSACTION_DATE`. Headers with
    /// no change are omitted; the user only ever sees rows that actually
    /// moved.
    pub changes: Vec<RecalcChangeEntry>,
}

/// One row of the change log returned by `recalculate_all_transaction_totals`.
///
/// `TRANSACTION_ID` is intentionally absent: it does not appear anywhere in
/// the regular UI, so identifying a row by `(TRANSACTION_DATE, TOTAL_AMOUNT)`
/// — with the per-detail amounts as a tiebreaker when those collide — is
/// what the user can actually map back to a transaction they recognise.
#[derive(Debug, Serialize, Deserialize)]
pub struct RecalcChangeEntry {
    pub transaction_date: String,
    /// Per-detail `AMOUNT` values, in DETAIL_ID order. Acts as the tiebreaker
    /// when (date, total) alone does not uniquely identify a header.
    pub detail_amounts: Vec<i64>,
    pub total_amount_before: i64,
    pub total_amount_after: i64,
    pub tax_rounding_type_before: i64,
    pub tax_rounding_type_after: i64,
    pub tax_included_type_before: i64,
    pub tax_included_type_after: i64,
    /// One of "settings_corrected" | "total_overwritten".
    pub change_type: String,
}

/// Result of `restore_totals_from_backup`. Reports how many header rows
/// actually had their `TOTAL_AMOUNT` reverted to the backup value.
#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreSummary {
    pub restored: i64,
}

/// Per-detail input for `calculate_recommended_total`.
///
/// Mirrors the columns the calculation reads from `TRANSACTIONS_DETAIL`. The
/// struct is kept deliberately tiny — it holds *only* the fields that drive
/// the tax classification and gross-up — so the calculation function can be
/// exercised from unit tests without faking out a full detail row.
#[derive(Debug, Clone, Copy)]
pub struct DetailForRecalc {
    pub amount: i64,
    pub amount_including_tax: Option<i64>,
    pub tax_rate: i64,
}

/// Compute the recommended `TOTAL_AMOUNT` for a transaction header from its
/// detail rows and the header's `TAX_ROUNDING_TYPE`.
///
/// The shape mirrors `build_detail_query` in `services::aggregation`, so the
/// header total a saved transaction carries always matches what the
/// dashboard would re-derive by walking the details:
///
/// 1. Each detail is classified as either *already tax-included* (when
///    `TAX_RATE = 0` or `AMOUNT == AMOUNT_INCLUDING_TAX`) or *needs gross-up*.
/// 2. For each tax rate present, the pre-tax amounts are summed before the
///    gross-up factor is applied — never the other way around — and the
///    rounding rule is applied exactly once per `(rate, rounding_type)` slice
///    to avoid the per-detail rounding error that v1.x carried.
/// 3. The integer slices are summed to produce the header total.
///
/// `tax_rounding_type` follows the existing constants:
/// - `0` → floor (`TAX_ROUND_DOWN`)
/// - `1` → half-away-from-zero (`TAX_ROUND_HALF_UP`)
/// - `2` → ceil (`TAX_ROUND_UP`)
/// Anything else falls back to floor, matching the SQL `ELSE` arm.
pub fn calculate_recommended_total(
    details: &[DetailForRecalc],
    tax_rounding_type: i64,
) -> i64 {
    use std::collections::HashMap;

    // (already_included_sum, pretax_sum) keyed by tax_rate
    let mut by_rate: HashMap<i64, (i64, i64)> = HashMap::new();

    for d in details {
        let is_already_included = d.tax_rate == 0
            || d.amount_including_tax.map_or(false, |inc| inc == d.amount);

        let entry = by_rate.entry(d.tax_rate).or_insert((0, 0));
        if is_already_included {
            entry.0 += d.amount;
        } else {
            entry.1 += d.amount;
        }
    }

    let mut total: i64 = 0;
    for (rate, (already, pretax)) in by_rate {
        // pretax * (100 + rate) is the un-rounded grossed amount in 1/100ths
        // of a yen; rounding it back to whole yen depends on the chosen mode.
        let grossed = pretax * (100 + rate);
        let pretax_grossed = match tax_rounding_type {
            consts::TAX_ROUND_DOWN => grossed / 100,            // floor (positive only)
            consts::TAX_ROUND_HALF_UP => (grossed + 50) / 100,  // half-away-from-zero, positive
            consts::TAX_ROUND_UP => (grossed + 99) / 100,       // ceil, positive
            _ => grossed / 100,
        };
        total += already + pretax_grossed;
    }

    total
}

/// Compute the header total under an explicit `(tax_rounding, tax_included)`
/// pair. The "tax-included" branch takes the SUM verbatim — no gross-up, no
/// rounding — because in that mode the user has declared the per-detail
/// AMOUNT values are already inclusive of tax. The "tax-excluded" branch
/// delegates to `calculate_recommended_total`, which still honours the
/// per-detail `AMOUNT == AMOUNT_INCLUDING_TAX` short-circuit so a single
/// already-included row inside an otherwise-excluded ledger does not get
/// grossed up a second time.
pub fn calculate_recommended_total_with_settings(
    details: &[DetailForRecalc],
    tax_rounding_type: i64,
    tax_included_type: i64,
) -> i64 {
    if tax_included_type == consts::TAX_INCLUDED {
        details.iter().map(|d| d.amount).sum()
    } else {
        calculate_recommended_total(details, tax_rounding_type)
    }
}

/// Find the first `(tax_rounding_type, tax_included_type)` pattern that
/// reproduces `target_total` from `details`. Returns `None` when no pattern
/// fits, in which case the bulk-recalc flow falls back to overwriting the
/// total with whatever the existing settings produce.
///
/// The caller-supplied `preferred` pair (the header's currently-stored
/// `(rounding, included)`) is always tried first: whenever the existing
/// settings already reproduce `target_total` — a very common case for
/// receipts whose total has no fractional part to round — the function
/// returns them verbatim so the bulk-recalc flow can skip the header
/// instead of silently rewriting the tax setting columns.
///
/// Fable-5 review #2 — without the preferred-first check, a header
/// saved as `HALF_UP + EXCLUDED` with a round-cent detail (e.g. 500円
/// at 10 % = 550円 exactly) matched the first PATTERNS entry, which is
/// `FLOOR + EXCLUDED`, and every bulk recalc silently downgraded the
/// user's chosen rounding mode to FLOOR.
///
/// If the preferred pair doesn't fit, the fallback tries these patterns
/// in priority order (the order matches what shopkeepers actually do —
/// floor or half-up dominate, ceil is rare — so "first match wins" lands
/// on the most plausible setting):
///
///   1. tax-excluded + floor       (TAX_ROUND_DOWN)
///   2. tax-excluded + half-up     (TAX_ROUND_HALF_UP)
///   3. tax-excluded + ceil        (TAX_ROUND_UP)
///   4. tax-included               (the rounding column is irrelevant in
///      this mode because no rounding ever happens; we report it back as
///      `TAX_ROUND_DOWN` so the caller has a stable value to write)
fn find_matching_pattern(
    details: &[DetailForRecalc],
    target_total: i64,
    preferred: (i64, i64),
) -> Option<(i64, i64)> {
    // Try the header's current settings first (Fable-5 #2).
    let (pref_rounding, pref_included) = preferred;
    if calculate_recommended_total_with_settings(details, pref_rounding, pref_included)
        == target_total
    {
        return Some(preferred);
    }

    const PATTERNS: [(i64, i64); 4] = [
        (consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED),
        (consts::TAX_ROUND_HALF_UP, consts::TAX_EXCLUDED),
        (consts::TAX_ROUND_UP, consts::TAX_EXCLUDED),
        (consts::TAX_ROUND_DOWN, consts::TAX_INCLUDED),
    ];
    for (rounding, included) in PATTERNS {
        // Skip re-evaluating the preferred pair we already tried.
        if (rounding, included) == preferred {
            continue;
        }
        if calculate_recommended_total_with_settings(details, rounding, included) == target_total {
            return Some((rounding, included));
        }
    }
    None
}

impl TransactionService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Add a new transaction header with save request
    pub async fn save_transaction_header(
        &self,
        user_id: i64,
        request: SaveTransactionRequest,
    ) -> Result<i64, TransactionError> {
        // Validate datetime format (YYYY-MM-DD HH:MM:SS)
        if request.transaction_date.len() != 19 {
            return Err(TransactionError::ValidationError(
                "Invalid datetime format. Use YYYY-MM-DD HH:MM:SS".to_string(),
            ));
        }

        // Validate amount (0 is allowed)
        if request.total_amount < 0 || request.total_amount > 999_999_999 {
            return Err(TransactionError::ValidationError(
                "Amount must be between 0 and 999,999,999".to_string(),
            ));
        }

        // Validate tax rounding type using constants
        if request.tax_rounding_type != consts::TAX_ROUND_DOWN
            && request.tax_rounding_type != consts::TAX_ROUND_HALF_UP
            && request.tax_rounding_type != consts::TAX_ROUND_UP
        {
            return Err(TransactionError::ValidationError(
                "Invalid tax rounding type".to_string(),
            ));
        }

        // Validate tax included type using constants. CodeRabbit on #125
        // — without this guard, an unknown value (e.g. 99) can be
        // written to `TAX_INCLUDED_TYPE` and later handed to
        // `find_matching_pattern` as `preferred`; the preferred-first
        // check would then preserve the bogus value across bulk recalc
        // instead of correcting it.
        if request.tax_included_type != consts::TAX_INCLUDED
            && request.tax_included_type != consts::TAX_EXCLUDED
        {
            return Err(TransactionError::ValidationError(
                "Invalid tax included type".to_string(),
            ));
        }

        // Fable-5 review #20 — TRANSFER with the same FROM and TO
        // account is meaningless (net movement is zero) and used to
        // sneak through both entry points. The dashboard-side
        // `ACCOUNT_BALANCES_AS_OF` CASE evaluates the +TO arm before
        // the -FROM arm and stops at first match, so the row was
        // silently counted as a one-sided credit and inflated the
        // account balance by the transfer amount. Reject the write
        // outright; the CASE was made symmetric in the same PR as a
        // second line of defence for legacy rows.
        if request.category1_code == "TRANSFER"
            && request.from_account_code == request.to_account_code
        {
            return Err(TransactionError::TransferSameAccount);
        }

        // Save memo if provided
        let memo_id = if let Some(text) = &request.memo {
            if !text.trim().is_empty() {
                validate_memo_length(text)?;
                let result = sqlx::query(sql_queries::MEMO_INSERT)
                    .bind(user_id)
                    .bind(text)
                    .execute(&self.pool)
                    .await?;
                Some(result.last_insert_rowid())
            } else {
                None
            }
        } else {
            None
        };

        // Insert transaction header
        let result = sqlx::query(sql_queries::TRANSACTION_HEADER_INSERT)
            .bind(user_id)
            .bind(request.shop_id)
            .bind(&request.transaction_date)
            .bind(&request.category1_code)
            .bind(&request.from_account_code)
            .bind(&request.to_account_code)
            .bind(request.total_amount)
            .bind(request.tax_rounding_type)
            .bind(request.tax_included_type)
            .bind(memo_id)
            .bind(request.is_scheduled.unwrap_or(0))
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get transaction header by ID
    pub async fn get_transaction_header(
        &self,
        user_id: i64,
        transaction_id: i64,
    ) -> Result<TransactionHeader, TransactionError> {
        let header = sqlx::query_as::<_, TransactionHeader>(sql_queries::TRANSACTION_HEADER_GET_BY_ID)
            .bind(transaction_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        header.ok_or(TransactionError::NotFound)
    }

    /// Get transaction header by ID with memo text
    pub async fn get_transaction_header_with_memo(
        &self,
        user_id: i64,
        transaction_id: i64,
    ) -> Result<(TransactionHeader, Option<String>), TransactionError> {
        let row = sqlx::query(sql_queries::TRANSACTION_HEADER_GET_BY_ID_WITH_MEMO)
            .bind(transaction_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let header = TransactionHeader {
                transaction_id: row.get(0),
                user_id: row.get(1),
                shop_id: row.get(2),
                transaction_date: row.get(3),
                category1_code: row.get(4),
                from_account_code: row.get(5),
                to_account_code: row.get(6),
                total_amount: row.get(7),
                tax_rounding_type: row.get(8),
                tax_included_type: row.get(9),
                memo_id: row.get(10),
                is_disabled: row.get(11),
                is_scheduled: row.get(12),
                entry_dt: row.get(13),
                update_dt: row.get(14),
            };
            let memo_text: Option<String> = row.get(15);
            Ok((header, memo_text))
        } else {
            Err(TransactionError::NotFound)
        }
    }

    /// Get transactions with filters and pagination
    pub async fn get_transactions(
        &self,
        user_id: i64,
        start_date: Option<&str>,
        end_date: Option<&str>,
        category1_code: Option<&str>,
        category2_code: Option<&str>,
        category3_code: Option<&str>,
        min_amount: Option<i64>,
        max_amount: Option<i64>,
        keyword: Option<&str>,
        include_scheduled: bool,
        page: i64,
        per_page: i64,
    ) -> Result<TransactionListResponse, TransactionError> {
        // Clamp pagination input: per_page = 0 would divide by zero below and
        // negative values would produce a negative OFFSET.
        let page = page.max(1);
        let per_page = per_page.clamp(1, MAX_PER_PAGE);

        // Build WHERE clauses (with table alias 't.')
        let mut where_clauses = vec!["t.USER_ID = ?".to_string()];
        let mut params: Vec<String> = vec![user_id.to_string()];

        // Exclude scheduled transactions by default. When the user opts in,
        // every IS_SCHEDULED row is shown, including each occurrence of a
        // recurring rule — the date filter (TRANSACTION_DATE BETWEEN ...)
        // is what scopes the visible window. Group membership of recurring
        // occurrences is preserved through RULE_ID, not through any
        // representative-row trick that would have to be maintained on
        // every confirm/delete.
        if !include_scheduled {
            where_clauses.push("t.IS_SCHEDULED = 0".to_string());
        }

        if let Some(start) = start_date {
            where_clauses.push("t.TRANSACTION_DATE >= ?".to_string());
            params.push(start.to_string());
        }

        if let Some(end) = end_date {
            where_clauses.push("t.TRANSACTION_DATE <= ?".to_string());
            // TRANSACTION_DATE is stored as 'YYYY-MM-DD HH:MM:SS' but the UI's
            // <input type="date"> sends bare 'YYYY-MM-DD', which under string
            // comparison drops every same-day timestamp. Anchor to end-of-day
            // so the boundary day is included.
            let normalized = if end.len() == 10 {
                format!("{} 23:59:59", end)
            } else {
                end.to_string()
            };
            params.push(normalized);
        }

        if let Some(cat1) = category1_code {
            if !cat1.is_empty() {
                where_clauses.push("t.CATEGORY1_CODE = ?".to_string());
                params.push(cat1.to_string());
            }
        }

        // CATEGORY2_CODE / CATEGORY3_CODE live on TRANSACTIONS_DETAIL, so filter
        // via EXISTS to keep the header row count stable (no DISTINCT needed).
        if let Some(cat2) = category2_code {
            if !cat2.is_empty() {
                where_clauses.push(
                    "EXISTS (SELECT 1 FROM TRANSACTIONS_DETAIL td \
                     WHERE td.USER_ID = t.USER_ID \
                       AND td.TRANSACTION_ID = t.TRANSACTION_ID \
                       AND td.CATEGORY2_CODE = ?)"
                        .to_string(),
                );
                params.push(cat2.to_string());
            }
        }

        if let Some(cat3) = category3_code {
            if !cat3.is_empty() {
                where_clauses.push(
                    "EXISTS (SELECT 1 FROM TRANSACTIONS_DETAIL td \
                     WHERE td.USER_ID = t.USER_ID \
                       AND td.TRANSACTION_ID = t.TRANSACTION_ID \
                       AND td.CATEGORY3_CODE = ?)"
                        .to_string(),
                );
                params.push(cat3.to_string());
            }
        }

        if let Some(min) = min_amount {
            where_clauses.push("t.TOTAL_AMOUNT >= ?".to_string());
            params.push(min.to_string());
        }

        if let Some(max) = max_amount {
            where_clauses.push("t.TOTAL_AMOUNT <= ?".to_string());
            params.push(max.to_string());
        }

        // Keyword: substring match against memo text on the header row and on
        // any detail row of the same header. MEMO_TEXT is user-supplied so we
        // escape LIKE metacharacters and bind the same pattern twice.
        if let Some(kw) = keyword {
            let kw = kw.trim();
            if !kw.is_empty() {
                let pattern = format!("%{}%", escape_like_pattern(kw));
                where_clauses.push(sql_queries::TRANSACTION_KEYWORD_MEMO_FILTER.to_string());
                params.push(pattern.clone());
                params.push(pattern);
            }
        }

        let where_clause = where_clauses.join(" AND ");

        // Get total count
        let count_query = format!("{}{}", sql_queries::TRANSACTION_COUNT_BASE, where_clause);
        let mut count_stmt = sqlx::query(&count_query);
        for param in &params {
            count_stmt = count_stmt.bind(param);
        }
        let total_count: i64 = count_stmt
            .fetch_one(&self.pool)
            .await?
            .get(0);

        // Calculate pagination
        let offset = (page - 1) * per_page;
        let total_pages = (total_count + per_page - 1) / per_page;

        // Get transactions
        let query = format!(
            "{}{}{} LIMIT ? OFFSET ?",
            sql_queries::TRANSACTION_LIST_BASE,
            where_clause,
            sql_queries::TRANSACTION_LIST_ORDER
        );
        
        let mut stmt = sqlx::query_as::<_, Transaction>(&query);
        for param in &params {
            stmt = stmt.bind(param);
        }
        stmt = stmt.bind(per_page).bind(offset);
        
        let transactions = stmt.fetch_all(&self.pool).await?;

        Ok(TransactionListResponse {
            transactions,
            total_count,
            page,
            per_page,
            total_pages,
        })
    }

    /// Delete a transaction
    pub async fn delete_transaction(
        &self,
        user_id: i64,
        transaction_id: i64,
    ) -> Result<(), TransactionError> {
        let result = sqlx::query(sql_queries::TRANSACTION_DELETE)
            .bind(user_id)
            .bind(transaction_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(TransactionError::NotFound);
        }

        Ok(())
    }

    /// Confirm a scheduled transaction (set IS_SCHEDULED from 1 to 0)
    pub async fn confirm_scheduled_transaction(
        &self,
        user_id: i64,
        transaction_id: i64,
    ) -> Result<(), TransactionError> {
        let result = sqlx::query(sql_queries::TRANSACTION_HEADER_CONFIRM_SCHEDULED)
            .bind(transaction_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(TransactionError::NotFound);
        }

        Ok(())
    }

    /// Helper function to get or create memo_id for memo text.
    /// Returns memo_id if memo text is provided, None if empty.
    ///
    /// Runs its own single-connection lookup/insert. Callers that need
    /// atomicity with a subsequent row insert (add_transaction_detail,
    /// see Fable-5 #7) should use [`get_or_create_memo_id_in_tx`]
    /// instead so the MEMO row and the referrer row commit together
    /// and a mid-flight failure cannot orphan the MEMO.
    async fn get_or_create_memo_id(
        &self,
        user_id: i64,
        memo_text: Option<&str>,
    ) -> Result<Option<i64>, TransactionError> {
        let mut tx = self.pool.begin().await?;
        let memo_id = Self::get_or_create_memo_id_in_tx(&mut tx, user_id, memo_text).await?;
        tx.commit().await?;
        Ok(memo_id)
    }

    /// Same as [`get_or_create_memo_id`] but reuses an existing
    /// transaction so the MEMO lookup / insert commits together with
    /// whatever the caller does next. Fable-5 #7: without this, an
    /// `add_transaction_detail` that inserts the MEMO on the pool then
    /// hits an FK error on `TRANSACTIONS_DETAIL_INSERT_FULL` leaves the
    /// MEMO row behind for good — the two writes need to be one tx.
    async fn get_or_create_memo_id_in_tx<'c>(
        tx: &mut sqlx::Transaction<'c, sqlx::Sqlite>,
        user_id: i64,
        memo_text: Option<&str>,
    ) -> Result<Option<i64>, TransactionError> {
        let memo_text = match memo_text {
            Some(text) => text.trim(),
            None => return Ok(None),
        };

        if memo_text.is_empty() {
            // Empty memo - return None (memo_id will be NULL)
            return Ok(None);
        }

        validate_memo_length(memo_text)?;

        // Check if memo with same text already exists.
        let existing_memo = sqlx::query(sql_queries::MEMO_FIND_BY_TEXT)
            .bind(user_id)
            .bind(memo_text)
            .fetch_optional(&mut **tx)
            .await?;

        if let Some(row) = existing_memo {
            Ok(Some(row.get(0)))
        } else {
            // Create new memo inside the caller's tx.
            let result = sqlx::query(sql_queries::MEMO_INSERT)
                .bind(user_id)
                .bind(memo_text)
                .execute(&mut **tx)
                .await?;
            Ok(Some(result.last_insert_rowid()))
        }
    }

    /// Total references to a memo across TRANSACTIONS_HEADER,
    /// TRANSACTIONS_DETAIL, RECURRING_RULES, and RECURRING_RULE_DETAILS.
    /// A memo is considered shared when this exceeds the caller's own
    /// reference (typically shared > 1 for updates, or leftover > 0 after
    /// the caller already released its reference).
    async fn memo_usage_count(&self, memo_id: i64) -> Result<i64, TransactionError> {
        let count: i64 = sqlx::query_scalar(sql_queries::MEMO_COUNT_USAGE)
            .bind(memo_id)
            .bind(memo_id)
            .bind(memo_id)
            .bind(memo_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(count)
    }

    /// Helper function to get memo_id for update (handles shared memo_id case)
    async fn get_memo_id_for_update(
        &self,
        user_id: i64,
        memo_text: Option<&str>,
        current_memo_id: Option<i64>,
    ) -> Result<Option<i64>, TransactionError> {
        let memo_text = match memo_text {
            Some(text) => text.trim(),
            None => return Ok(None),
        };

        if memo_text.is_empty() {
            // Empty memo - return None (memo_id will be NULL)
            return Ok(None);
        }

        validate_memo_length(memo_text)?;

        // Check if current memo_id is shared with other transactions
        let is_shared = if let Some(memo_id) = current_memo_id {
            self.memo_usage_count(memo_id).await? > 1
        } else {
            false
        };

        if is_shared {
            // Current memo_id is shared - create new memo_id
            // Check if memo with same text already exists
            let existing_memo = sqlx::query(sql_queries::MEMO_FIND_BY_TEXT)
                .bind(user_id)
                .bind(memo_text)
                .fetch_optional(&self.pool)
                .await?;

            if let Some(row) = existing_memo {
                Ok(Some(row.get(0)))
            } else {
                // Create new memo
                let result = sqlx::query(sql_queries::MEMO_INSERT)
                    .bind(user_id)
                    .bind(memo_text)
                    .execute(&self.pool)
                    .await?;
                Ok(Some(result.last_insert_rowid()))
            }
        } else {
            // Current memo_id is not shared - can reuse or create new
            self.get_or_create_memo_id(user_id, Some(memo_text)).await
        }
    }

    /// Update a single transaction header
    pub async fn update_transaction_header(
        &self,
        user_id: i64,
        transaction_id: i64,
        request: SaveTransactionRequest,
    ) -> Result<(), TransactionError> {
        // Validate datetime format (YYYY-MM-DD HH:MM:SS)
        if request.transaction_date.len() != 19 {
            return Err(TransactionError::ValidationError(
                "Invalid datetime format. Use YYYY-MM-DD HH:MM:SS".to_string(),
            ));
        }

        // Validate amount (0 is allowed)
        if request.total_amount < 0 || request.total_amount > 999_999_999 {
            return Err(TransactionError::ValidationError(
                "Amount must be between 0 and 999,999,999".to_string(),
            ));
        }

        // Validate tax rounding type using constants
        if request.tax_rounding_type != consts::TAX_ROUND_DOWN
            && request.tax_rounding_type != consts::TAX_ROUND_HALF_UP
            && request.tax_rounding_type != consts::TAX_ROUND_UP
        {
            return Err(TransactionError::ValidationError(
                "Invalid tax rounding type".to_string(),
            ));
        }

        // Validate tax included type using constants. CodeRabbit on #125
        // — without this guard, an unknown value (e.g. 99) can be
        // written to `TAX_INCLUDED_TYPE` and later handed to
        // `find_matching_pattern` as `preferred`; the preferred-first
        // check would then preserve the bogus value across bulk recalc
        // instead of correcting it.
        if request.tax_included_type != consts::TAX_INCLUDED
            && request.tax_included_type != consts::TAX_EXCLUDED
        {
            return Err(TransactionError::ValidationError(
                "Invalid tax included type".to_string(),
            ));
        }

        // Fable-5 review #20 — TRANSFER with the same FROM and TO
        // account is meaningless (net movement is zero) and used to
        // sneak through both entry points. The dashboard-side
        // `ACCOUNT_BALANCES_AS_OF` CASE evaluates the +TO arm before
        // the -FROM arm and stops at first match, so the row was
        // silently counted as a one-sided credit and inflated the
        // account balance by the transfer amount. Reject the write
        // outright; the CASE was made symmetric in the same PR as a
        // second line of defence for legacy rows.
        if request.category1_code == "TRANSFER"
            && request.from_account_code == request.to_account_code
        {
            return Err(TransactionError::TransferSameAccount);
        }

        // Get current transaction header to check current memo_id
        let current_header = self.get_transaction_header(user_id, transaction_id).await?;

        // Get or create memo_id (handles shared memo_id case)
        let memo_id = self
            .get_memo_id_for_update(user_id, request.memo.as_deref(), current_header.memo_id)
            .await?;

        // Update transaction header
        let result = sqlx::query(sql_queries::TRANSACTION_HEADER_UPDATE)
            .bind(request.shop_id)
            .bind(&request.transaction_date)
            .bind(&request.category1_code)
            .bind(&request.from_account_code)
            .bind(&request.to_account_code)
            .bind(request.total_amount)
            .bind(request.tax_rounding_type)
            .bind(request.tax_included_type)
            .bind(memo_id)
            .bind(transaction_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(TransactionError::NotFound);
        }

        Ok(())
    }

    /// Update multiple transaction headers
    pub async fn update_transaction_headers(
        &self,
        user_id: i64,
        transactions: Vec<(i64, SaveTransactionRequest)>,
    ) -> Result<(), TransactionError> {
        for (transaction_id, request) in transactions {
            self.update_transaction_header(user_id, transaction_id, request)
                .await?;
        }
        Ok(())
    }

    /// Get transaction header with related information
    pub async fn get_transaction_header_with_info(
        &self,
        user_id: i64,
        transaction_id: i64,
    ) -> Result<TransactionHeaderWithInfo, TransactionError> {
        let row = sqlx::query(sql_queries::TRANSACTION_HEADER_GET_WITH_INFO)
            .bind(transaction_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(TransactionHeaderWithInfo {
                transaction_id: row.get("TRANSACTION_ID"),
                user_id: row.get("USER_ID"),
                shop_id: row.get("SHOP_ID"),
                shop_name: row.get("SHOP_NAME"),
                transaction_date: row.get("TRANSACTION_DATE"),
                category1_code: row.get("CATEGORY1_CODE"),
                from_account_code: row.get("FROM_ACCOUNT_CODE"),
                to_account_code: row.get("TO_ACCOUNT_CODE"),
                from_account_name: row.get("FROM_ACCOUNT_NAME"),
                to_account_name: row.get("TO_ACCOUNT_NAME"),
                total_amount: row.get("TOTAL_AMOUNT"),
                tax_rounding_type: row.get("TAX_ROUNDING_TYPE"),
                tax_included_type: row.get("TAX_INCLUDED_TYPE"),
                memo_id: row.get("MEMO_ID"),
                memo_text: row.get("MEMO_TEXT"),
                is_disabled: row.get("IS_DISABLED"),
                is_scheduled: row.get("IS_SCHEDULED"),
                entry_dt: row.get("ENTRY_DT"),
                update_dt: row.get("UPDATE_DT"),
            }),
            None => Err(TransactionError::NotFound),
        }
    }

    /// Get transaction details by transaction ID
    pub async fn get_transaction_details(
        &self,
        user_id: i64,
        transaction_id: i64,
    ) -> Result<Vec<TransactionDetailWithInfo>, TransactionError> {
        let rows = sqlx::query(sql_queries::TRANSACTION_DETAIL_GET_WITH_INFO)
            .bind(transaction_id)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let details = rows
            .iter()
            .map(|row| TransactionDetailWithInfo {
                detail_id: row.get("DETAIL_ID"),
                transaction_id: row.get("TRANSACTION_ID"),
                user_id: row.get("USER_ID"),
                category1_code: row.get("CATEGORY1_CODE"),
                category2_code: row.get("CATEGORY2_CODE"),
                category3_code: row.get("CATEGORY3_CODE"),
                category1_name: row.get("CATEGORY1_NAME"),
                category2_name: row.get("CATEGORY2_NAME"),
                category3_name: row.get("CATEGORY3_NAME"),
                item_name: row.get("ITEM_NAME"),
                amount: row.get("AMOUNT"),
                tax_amount: row.get("TAX_AMOUNT"),
                tax_rate: row.get("TAX_RATE"),
                amount_including_tax: row.get("AMOUNT_INCLUDING_TAX"),
                product_id: row.get("PRODUCT_ID"),
                product_name: row.get("PRODUCT_NAME"),
                manufacturer_name: row.get("MANUFACTURER_NAME"),
                memo_id: row.get("MEMO_ID"),
                memo_text: row.get("MEMO_TEXT"),
                entry_dt: row.get("ENTRY_DT"),
                update_dt: row.get("UPDATE_DT"),
            })
            .collect();

        Ok(details)
    }

    /// Add a new transaction detail
    pub async fn add_transaction_detail(
        &self,
        user_id: i64,
        transaction_id: i64,
        request: SaveTransactionDetailRequest,
    ) -> Result<i64, TransactionError> {
        // Validate item name
        if request.item_name.trim().is_empty() {
            return Err(TransactionError::ValidationError(
                "Item name is required".to_string(),
            ));
        }

        validate_item_name_length(&request.item_name)?;

        // Validate amount
        if request.amount < 0 || request.amount > 999_999_999 {
            return Err(TransactionError::ValidationError(
                "Amount must be between 0 and 999,999,999".to_string(),
            ));
        }

        // Validate tax rate
        if request.tax_rate < 0 || request.tax_rate > 100 {
            return Err(TransactionError::ValidationError(
                "Tax rate must be between 0 and 100".to_string(),
            ));
        }

        // Validate tax amount
        if request.tax_amount < 0 {
            return Err(TransactionError::ValidationError(
                "Tax amount cannot be negative".to_string(),
            ));
        }

        // Verify the parent header exists AND belongs to this user. The FK
        // on TRANSACTIONS_DETAIL.TRANSACTION_ID only checks that some header
        // row exists — it does not enforce ownership. Without this guard, a
        // request coming through direct `invoke` (bypassing the frontend
        // form) with another user's transaction_id would attach the new
        // detail to that other user's header. `update_transaction_detail` /
        // `delete_transaction_detail` already do the equivalent check via
        // `fetch_optional` on the existing detail row (see below); this
        // brings `add` to the same standard so the three CRUD paths are
        // symmetric. Runs before MEMO_INSERT so a rejected add cannot leave
        // an orphaned MEMOS row behind.
        let parent_exists: Option<i64> =
            sqlx::query_scalar(sql_queries::TRANSACTION_HEADER_EXISTS_FOR_USER)
                .bind(transaction_id)
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await?;
        if parent_exists.is_none() {
            return Err(TransactionError::NotFound);
        }

        // Fable-5 review #7 — the two writes below (MEMO insert +
        // TRANSACTIONS_DETAIL insert) used to run on separate pool
        // connections. Two things went wrong:
        //   1. Every add unconditionally inserted a fresh MEMOS row
        //      even when the user typed the same text as before, so
        //      the "shared memo" update path in
        //      `update_transaction_detail` never fired for
        //      newly-added rows.
        //   2. If the DETAIL_INSERT failed (FK, disk, cancel), the
        //      MEMO row was already committed and orphaned forever
        //      — no cleanup path.
        // Wrapping both writes in a single tx fixes both by (a) going
        // through `get_or_create_memo_id_in_tx` which dedups against
        // an existing row with the same text, and (b) guaranteeing
        // the MEMO insert only becomes visible when the DETAIL insert
        // also succeeds.
        let mut tx = self.pool.begin().await?;

        let memo_id = Self::get_or_create_memo_id_in_tx(
            &mut tx,
            user_id,
            request.memo.as_deref(),
        )
        .await?;

        let result = sqlx::query(sql_queries::TRANSACTION_DETAIL_INSERT_FULL)
            .bind(transaction_id)
            .bind(user_id)
            .bind(&request.category1_code)
            .bind(&request.category2_code)
            .bind(&request.category3_code)
            .bind(&request.item_name)
            .bind(request.amount)
            .bind(request.tax_amount)
            .bind(request.tax_rate)
            .bind(request.amount_including_tax)
            .bind(request.product_id)
            .bind(memo_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(result.last_insert_rowid())
    }

    /// Update a transaction detail
    pub async fn update_transaction_detail(
        &self,
        user_id: i64,
        detail_id: i64,
        request: SaveTransactionDetailRequest,
    ) -> Result<(), TransactionError> {
        // Validate item name
        if request.item_name.trim().is_empty() {
            return Err(TransactionError::ValidationError(
                "Item name is required".to_string(),
            ));
        }

        validate_item_name_length(&request.item_name)?;

        // Validate amount
        if request.amount < 0 || request.amount > 999_999_999 {
            return Err(TransactionError::ValidationError(
                "Amount must be between 0 and 999,999,999".to_string(),
            ));
        }

        // Validate tax rate
        if request.tax_rate < 0 || request.tax_rate > 100 {
            return Err(TransactionError::ValidationError(
                "Tax rate must be between 0 and 100".to_string(),
            ));
        }

        // Validate tax amount
        if request.tax_amount < 0 {
            return Err(TransactionError::ValidationError(
                "Tax amount cannot be negative".to_string(),
            ));
        }

        // Get existing detail to check memo_id
        let existing: Option<TransactionDetail> = sqlx::query_as(
            sql_queries::TRANSACTION_DETAIL_GET_BY_ID
        )
        .bind(detail_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        let existing_detail = existing.ok_or(TransactionError::NotFound)?;

        // Handle memo update. The old code mutated / deleted the referenced
        // memo row directly, which corrupts any header or sibling detail that
        // shared the same MEMO_ID via MEMO_FIND_BY_TEXT reuse. Gate every
        // destructive step on a share check.
        //
        // Note: `deferred_memo_delete` carries the old MEMO_ID out to *after*
        // the DETAIL update. Deleting a memo row while the detail still
        // references it violates the MEMOS FK under production settings
        // (PRAGMA foreign_keys = ON), so the delete must run only once the
        // reference has been released.
        let (memo_id, deferred_memo_delete) = if let Some(text) = &request.memo {
            if !text.trim().is_empty() {
                validate_memo_length(text)?;

                let resolved = if let Some(old_memo_id) = existing_detail.memo_id {
                    let shared = self.memo_usage_count(old_memo_id).await? > 1;
                    if shared {
                        // Old memo has other references — must not mutate it.
                        // Point this detail at a fresh/reused memo instead.
                        self.get_or_create_memo_id(user_id, Some(text)).await?
                    } else {
                        // Only this detail uses the old memo — safe in place.
                        sqlx::query(sql_queries::MEMO_UPDATE)
                            .bind(text)
                            .bind(old_memo_id)
                            .execute(&self.pool)
                            .await?;
                        Some(old_memo_id)
                    }
                } else {
                    // No prior memo — find-or-create to avoid duplicate rows
                    // when the same text already lives in MEMOS.
                    self.get_or_create_memo_id(user_id, Some(text)).await?
                };
                (resolved, None)
            } else {
                // Text cleared — defer the old memo's cleanup until after
                // the DETAIL update has released its reference.
                (None, existing_detail.memo_id)
            }
        } else {
            (existing_detail.memo_id, None)
        };

        // Update detail
        let result = sqlx::query(sql_queries::TRANSACTION_DETAIL_UPDATE_FULL)
            .bind(&request.category1_code)
            .bind(&request.category2_code)
            .bind(&request.category3_code)
            .bind(&request.item_name)
            .bind(request.amount)
            .bind(request.tax_amount)
            .bind(request.tax_rate)
            .bind(request.amount_including_tax)
            .bind(request.product_id)
            .bind(memo_id)
            .bind(detail_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(TransactionError::NotFound);
        }

        // Now that the detail's MEMO_ID has been released (or overwritten),
        // clean up the old memo row if the user cleared the memo text and
        // nothing else still points at it.
        if let Some(old_memo_id) = deferred_memo_delete {
            if self.memo_usage_count(old_memo_id).await? == 0 {
                sqlx::query(sql_queries::MEMO_DELETE)
                    .bind(old_memo_id)
                    .execute(&self.pool)
                    .await?;
            }
        }

        Ok(())
    }

    /// Delete a transaction detail
    pub async fn delete_transaction_detail(
        &self,
        user_id: i64,
        detail_id: i64,
    ) -> Result<(), TransactionError> {
        // Get detail to check if it exists and get memo_id
        let detail: Option<TransactionDetail> = sqlx::query_as(
            sql_queries::TRANSACTION_DETAIL_GET_BY_ID
        )
        .bind(detail_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        let detail = detail.ok_or(TransactionError::NotFound)?;

        // Save memo_id for later deletion
        let memo_id = detail.memo_id;

        // Delete detail first (to release foreign key constraint)
        let result = sqlx::query(sql_queries::TRANSACTION_DETAIL_DELETE_BY_ID)
            .bind(detail_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(TransactionError::NotFound);
        }

        // Delete memo only when the detail we just removed held the last
        // reference. Header rows and sibling details can also point at this
        // MEMO_ID (via MEMO_FIND_BY_TEXT reuse); dropping it unconditionally
        // would leave those references dangling.
        if let Some(memo_id) = memo_id {
            if self.memo_usage_count(memo_id).await? == 0 {
                sqlx::query(sql_queries::MEMO_DELETE)
                    .bind(memo_id)
                    .execute(&self.pool)
                    .await?;
            }
        }

        Ok(())
    }

    /// Compute what `TOTAL_AMOUNT` should be for a transaction header given
    /// its current details and saved `TAX_ROUNDING_TYPE`. The frontend calls
    /// this after a detail edit to find out whether the header total it has
    /// cached is still correct, and prompts the user before overwriting it.
    pub async fn compute_recommended_total(
        &self,
        user_id: i64,
        transaction_id: i64,
    ) -> Result<i64, TransactionError> {
        let header_row = sqlx::query(sql_queries::TRANSACTION_HEADER_GET_ROUNDING_TYPE)
            .bind(transaction_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(TransactionError::NotFound)?;
        let rounding_type: i64 = header_row.get("TAX_ROUNDING_TYPE");

        let detail_rows = sqlx::query(sql_queries::TRANSACTION_DETAIL_GET_FOR_RECALC)
            .bind(transaction_id)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let details: Vec<DetailForRecalc> = detail_rows
            .iter()
            .map(|r| DetailForRecalc {
                amount: r.get("AMOUNT"),
                amount_including_tax: r.get("AMOUNT_INCLUDING_TAX"),
                tax_rate: r.get("TAX_RATE"),
            })
            .collect();

        Ok(calculate_recommended_total(&details, rounding_type))
    }

    /// Recompute every transaction header's `TOTAL_AMOUNT` for `user_id`
    /// from the current details, and persist the result. Wraps the whole
    /// pass in a single SQL transaction so a failure mid-flight rolls back
    /// every change. A timestamped copy of the DB file is taken first
    /// (after a WAL checkpoint) so the user can roll the data back even
    /// after the transaction commits — see `restore_totals_from_backup`.
    pub async fn recalculate_all_transaction_totals(
        &self,
        user_id: i64,
    ) -> Result<RecalcSummary, TransactionError> {
        // Force the WAL log into the main DB file before we copy it; otherwise
        // the backup would miss any writes that have not been checkpointed yet.
        sqlx::query("PRAGMA wal_checkpoint(FULL)")
            .execute(&self.pool)
            .await?;

        // Build a timestamped backup path next to the live DB and copy.
        let main_path = crate::db::get_db_path();
        let dir = main_path.parent().ok_or_else(|| {
            TransactionError::DatabaseError("DB path has no parent directory".to_string())
        })?;
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_path = dir.join(format!(
            "KakeiBonDB.sqlite3.backup_before_recalc_{}",
            timestamp
        ));
        std::fs::copy(&main_path, &backup_path).map_err(|e| {
            TransactionError::DatabaseError(format!(
                "Failed to copy DB to backup path {:?}: {}",
                backup_path, e
            ))
        })?;

        let mut tx = self.pool.begin().await?;

        // Load every header for this user up front. Holding them in memory
        // keeps the hot loop below from interleaving SELECT cursors with
        // UPDATE statements on the same transaction.
        let header_rows = sqlx::query(
            "SELECT TRANSACTION_ID, TRANSACTION_DATE, TAX_ROUNDING_TYPE, TAX_INCLUDED_TYPE, TOTAL_AMOUNT \
             FROM TRANSACTIONS_HEADER WHERE USER_ID = ? \
             ORDER BY TRANSACTION_DATE, TRANSACTION_ID",
        )
        .bind(user_id)
        .fetch_all(&mut *tx)
        .await?;

        // PR12 (Fable-5 #32): fetch every detail row for the user in
        // one query and bucket by TRANSACTION_ID before the loop below,
        // instead of running one SELECT per header. On a database with
        // 1000 headers this collapses 1001 queries to 2. The SQL's
        // `ORDER BY TRANSACTION_ID, DETAIL_ID` preserves the in-detail
        // ordering the single-transaction query used to give us, so
        // `detail_amounts` and the tax-recompute inputs are identical.
        let all_detail_rows =
            sqlx::query(sql_queries::TRANSACTION_DETAIL_GET_ALL_FOR_USER_RECALC)
                .bind(user_id)
                .fetch_all(&mut *tx)
                .await?;
        let mut details_by_txn: std::collections::HashMap<i64, Vec<DetailForRecalc>> =
            std::collections::HashMap::new();
        for row in all_detail_rows {
            let txn_id: i64 = row.get("TRANSACTION_ID");
            details_by_txn.entry(txn_id).or_default().push(DetailForRecalc {
                amount: row.get("AMOUNT"),
                amount_including_tax: row.get("AMOUNT_INCLUDING_TAX"),
                tax_rate: row.get("TAX_RATE"),
            });
        }

        let total_headers = header_rows.len() as i64;
        let mut settings_corrected = 0i64;
        let mut total_overwritten = 0i64;
        let mut skipped = 0i64;
        let mut changes: Vec<RecalcChangeEntry> = Vec::new();

        for header_row in header_rows {
            let txn_id: i64 = header_row.get("TRANSACTION_ID");
            let txn_date: String = header_row.get("TRANSACTION_DATE");
            let rounding_before: i64 = header_row.get("TAX_ROUNDING_TYPE");
            let included_before: i64 = header_row.get("TAX_INCLUDED_TYPE");
            let total_before: i64 = header_row.get("TOTAL_AMOUNT");

            // Take the pre-loaded details for this header (removing to
            // release memory as we go); a header with no detail rows
            // yields an empty Vec, matching the prior fetch behaviour.
            let details = details_by_txn.remove(&txn_id).unwrap_or_default();
            let detail_amounts: Vec<i64> = details.iter().map(|d| d.amount).collect();

            // First, prefer to keep the user-entered TOTAL_AMOUNT verbatim by
            // searching for a (rounding, included) pattern that reproduces it.
            // If we find one, the only correction we need to make is to the
            // header's tax setting columns. If we do not, we fall back to
            // overwriting TOTAL_AMOUNT with what the *existing* settings
            // produce, since the user's entry is then internally inconsistent
            // with the details and we have no signal to prefer it.
            match find_matching_pattern(
                &details,
                total_before,
                (rounding_before, included_before),
            ) {
                Some((rounding_after, included_after))
                    if rounding_after == rounding_before
                        && included_after == included_before =>
                {
                    skipped += 1;
                }
                Some((rounding_after, included_after)) => {
                    sqlx::query(sql_queries::TRANSACTION_HEADER_UPDATE_TAX_SETTINGS_ONLY)
                        .bind(rounding_after)
                        .bind(included_after)
                        .bind(txn_id)
                        .bind(user_id)
                        .execute(&mut *tx)
                        .await?;
                    settings_corrected += 1;
                    changes.push(RecalcChangeEntry {
                        transaction_date: txn_date.clone(),
                        detail_amounts: detail_amounts.clone(),
                        total_amount_before: total_before,
                        total_amount_after: total_before,
                        tax_rounding_type_before: rounding_before,
                        tax_rounding_type_after: rounding_after,
                        tax_included_type_before: included_before,
                        tax_included_type_after: included_after,
                        change_type: "settings_corrected".to_string(),
                    });
                }
                None => {
                    let total_after = calculate_recommended_total_with_settings(
                        &details,
                        rounding_before,
                        included_before,
                    );
                    if total_after == total_before {
                        // No pattern matched and the existing settings still
                        // happen to land on the existing total. Nothing to do.
                        skipped += 1;
                    } else {
                        sqlx::query(sql_queries::TRANSACTION_HEADER_UPDATE_TOTAL_ONLY)
                            .bind(total_after)
                            .bind(txn_id)
                            .bind(user_id)
                            .execute(&mut *tx)
                            .await?;
                        total_overwritten += 1;
                        changes.push(RecalcChangeEntry {
                            transaction_date: txn_date.clone(),
                            detail_amounts: detail_amounts.clone(),
                            total_amount_before: total_before,
                            total_amount_after: total_after,
                            tax_rounding_type_before: rounding_before,
                            tax_rounding_type_after: rounding_before,
                            tax_included_type_before: included_before,
                            tax_included_type_after: included_before,
                            change_type: "total_overwritten".to_string(),
                        });
                    }
                }
            }
        }

        tx.commit().await?;

        Ok(RecalcSummary {
            total_headers,
            settings_corrected,
            total_overwritten,
            skipped,
            backup_path: backup_path.to_string_lossy().to_string(),
            changes,
        })
    }

    /// Restore the `TOTAL_AMOUNT` column on every header for `user_id` from a
    /// backup file produced by `recalculate_all_transaction_totals`. We
    /// deliberately touch *only* `TOTAL_AMOUNT` — leaving details, memos and
    /// the rest of the schema untouched — so a rollback cannot accidentally
    /// erase any data the user has entered since the recalculation ran.
    pub async fn restore_totals_from_backup(
        &self,
        user_id: i64,
        backup_path: &str,
    ) -> Result<RestoreSummary, TransactionError> {
        let backup = std::path::Path::new(backup_path);
        if !backup.exists() {
            return Err(TransactionError::ValidationError(format!(
                "Backup file does not exist: {}",
                backup_path
            )));
        }
        // Only allow paths that live in the same directory as the live DB.
        // Stops a hostile caller from passing an arbitrary file path that
        // ATTACH would happily open.
        let main_dir = crate::db::get_db_path()
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| {
                TransactionError::DatabaseError("DB path has no parent directory".to_string())
            })?;
        // Canonicalize both sides so `..` segments and symlinks cannot be used
        // to point ATTACH at a file outside the DB directory.
        let main_dir = main_dir.canonicalize().unwrap_or(main_dir);
        let backup_dir = backup
            .parent()
            .map(|p| p.canonicalize().unwrap_or_else(|_| p.to_path_buf()))
            .ok_or_else(|| {
                TransactionError::ValidationError(
                    "Backup path has no parent directory".to_string(),
                )
            })?;
        if backup_dir != main_dir {
            return Err(TransactionError::ValidationError(format!(
                "Backup must live in the kakeibon DB directory ({:?})",
                main_dir
            )));
        }

        // ATTACH/UPDATE/DETACH must all run on the *same* SQLite connection
        // — `recalc_backup` only exists on the connection that ATTACHed it.
        // A pool-level `execute` would hand each statement out on a possibly
        // different connection, so we explicitly acquire one and pin every
        // statement to it.
        let mut conn = self.pool.acquire().await?;

        let attach_sql = format!(
            "ATTACH DATABASE '{}' AS recalc_backup",
            backup_path.replace('\'', "''")
        );
        sqlx::query(&attach_sql).execute(&mut *conn).await?;

        let result = sqlx::query(
            "UPDATE TRANSACTIONS_HEADER \
             SET TOTAL_AMOUNT = ( \
                 SELECT b.TOTAL_AMOUNT FROM recalc_backup.TRANSACTIONS_HEADER b \
                 WHERE b.TRANSACTION_ID = TRANSACTIONS_HEADER.TRANSACTION_ID \
                   AND b.USER_ID = TRANSACTIONS_HEADER.USER_ID \
             ), UPDATE_DT = datetime('now') \
             WHERE USER_ID = ? \
               AND EXISTS ( \
                   SELECT 1 FROM recalc_backup.TRANSACTIONS_HEADER b \
                   WHERE b.TRANSACTION_ID = TRANSACTIONS_HEADER.TRANSACTION_ID \
                     AND b.USER_ID = TRANSACTIONS_HEADER.USER_ID \
               )",
        )
        .bind(user_id)
        .execute(&mut *conn)
        .await?;

        let restored = result.rows_affected() as i64;

        sqlx::query("DETACH DATABASE recalc_backup")
            .execute(&mut *conn)
            .await?;

        Ok(RestoreSummary { restored })
    }

    /// Set `TOTAL_AMOUNT` on a transaction header without touching any other
    /// field. The frontend reaches for this after the user confirms the
    /// "header total drifted from details" prompt.
    pub async fn update_transaction_header_total(
        &self,
        user_id: i64,
        transaction_id: i64,
        new_total: i64,
    ) -> Result<(), TransactionError> {
        if new_total < 0 || new_total > 999_999_999 {
            return Err(TransactionError::ValidationError(
                "Amount must be between 0 and 999,999,999".to_string(),
            ));
        }

        let result = sqlx::query(sql_queries::TRANSACTION_HEADER_UPDATE_TOTAL_ONLY)
            .bind(new_total)
            .bind(transaction_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(TransactionError::NotFound);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use crate::{consts, sql_queries};

    // =========================================================================
    // calculate_recommended_total — pure function tests
    //
    // These cover the contract that the new auto-recalculation flow rests on:
    // the per-tax-rate sum is grossed up and rounded *once*, the per-detail
    // tax-included classification is honoured, and each rounding mode picks
    // the right edge case.
    // =========================================================================

    fn detail(amount: i64, including: Option<i64>, rate: i64) -> DetailForRecalc {
        DetailForRecalc {
            amount,
            amount_including_tax: including,
            tax_rate: rate,
        }
    }

    #[test]
    fn test_calculate_recommended_total_single_pretax_floor() {
        // 1000 yen pre-tax, 8% rate, floor mode → floor(1000 × 1.08) = 1080.
        let details = vec![detail(1000, Some(1080), 8)];
        assert_eq!(
            calculate_recommended_total(&details, consts::TAX_ROUND_DOWN),
            1080
        );
    }

    #[test]
    fn test_calculate_recommended_total_per_rate_sum_avoids_accumulation() {
        // Two 999-yen pre-tax details at 8%: per-detail rounding would give
        // floor(999 × 1.08) × 2 = 2156. Per-rate sum gives floor((999+999) × 1.08)
        // = floor(2157.84) = 2157. The 1-yen difference is exactly the bug
        // this whole refactor exists to fix.
        let details = vec![detail(999, Some(1078), 8), detail(999, Some(1078), 8)];
        assert_eq!(
            calculate_recommended_total(&details, consts::TAX_ROUND_DOWN),
            2157
        );
    }

    #[test]
    fn test_calculate_recommended_total_mixed_tax_rates() {
        // 8% and 10% bucket independently: 1000 × 1.08 + 2000 × 1.10 = 1080 + 2200.
        let details = vec![detail(1000, Some(1080), 8), detail(2000, Some(2200), 10)];
        assert_eq!(
            calculate_recommended_total(&details, consts::TAX_ROUND_DOWN),
            3280
        );
    }

    #[test]
    fn test_calculate_recommended_total_tax_included_detail_passes_through() {
        // amount == amount_including_tax with non-zero rate is tax-included
        // input — we must NOT gross it up a second time.
        let details = vec![detail(216, Some(216), 8)];
        assert_eq!(
            calculate_recommended_total(&details, consts::TAX_ROUND_DOWN),
            216
        );
    }

    #[test]
    fn test_calculate_recommended_total_tax_rate_zero_passes_through() {
        // tax_rate=0 → no gross-up, even when amount_including_tax happens to
        // disagree (which would be data corruption but should not blow up here).
        let details = vec![detail(500, Some(500), 0), detail(100, None, 0)];
        assert_eq!(
            calculate_recommended_total(&details, consts::TAX_ROUND_DOWN),
            600
        );
    }

    #[test]
    fn test_calculate_recommended_total_half_up_rounding() {
        // 999 × 1.08 = 1078.92 → half-up → 1079.
        let details = vec![detail(999, None, 8)];
        assert_eq!(
            calculate_recommended_total(&details, consts::TAX_ROUND_HALF_UP),
            1079
        );
    }

    #[test]
    fn test_calculate_recommended_total_ceil_rounding() {
        // 999 × 1.08 = 1078.92 → ceil → 1079. Also test exact value: 1000 × 1.08 = 1080
        // (no fractional part) → ceil → 1080.
        let details = vec![detail(999, None, 8)];
        assert_eq!(
            calculate_recommended_total(&details, consts::TAX_ROUND_UP),
            1079
        );
        let exact = vec![detail(1000, None, 8)];
        assert_eq!(
            calculate_recommended_total(&exact, consts::TAX_ROUND_UP),
            1080
        );
    }

    #[test]
    fn test_calculate_recommended_total_mixed_included_and_pretax_same_rate() {
        // Within one tax rate, an already-tax-included detail (300) sits next
        // to a pre-tax detail (1000). The pre-tax bucket grosses up to 1080,
        // the tax-included bucket passes through, total = 1380.
        let details = vec![detail(1000, Some(1080), 8), detail(300, Some(300), 8)];
        assert_eq!(
            calculate_recommended_total(&details, consts::TAX_ROUND_DOWN),
            1380
        );
    }

    #[test]
    fn test_calculate_recommended_total_empty_returns_zero() {
        let details: Vec<DetailForRecalc> = vec![];
        assert_eq!(
            calculate_recommended_total(&details, consts::TAX_ROUND_DOWN),
            0
        );
    }

    // find_matching_pattern — bulk-recalc classifier tests
    // ============================================================
    // These pin the preferred-first behaviour that closes
    // Fable-5 review #2 (bulk recalc silently downgrading the user's
    // chosen rounding mode to FLOOR on receipts with no fractional
    // remainder).

    /// Fable-5 review #2 — 500円 × 10% = 550円 exactly. With
    /// `(HALF_UP, EXCLUDED)` stored on the header, the pre-fix
    /// classifier tried PATTERNS in fixed order starting from
    /// `(FLOOR, EXCLUDED)` — which also produces 550 — and returned
    /// FLOOR, so the bulk recalc downgraded the user's chosen
    /// rounding mode. The preferred-first check now returns the
    /// stored setting verbatim whenever it reproduces the total.
    #[test]
    fn test_find_matching_pattern_preserves_user_half_up_when_settings_match() {
        let details = vec![DetailForRecalc {
            amount: 500,
            amount_including_tax: Some(0),
            tax_rate: 10,
        }];
        let preferred = (consts::TAX_ROUND_HALF_UP, consts::TAX_EXCLUDED);
        assert_eq!(
            find_matching_pattern(&details, 550, preferred),
            Some(preferred),
            "HALF_UP + EXCLUDED must be preserved when it reproduces the total"
        );
    }

    /// Companion pin — `(UP, EXCLUDED)` on a round-cent receipt used
    /// to be downgraded to FLOOR for the same reason.
    #[test]
    fn test_find_matching_pattern_preserves_user_ceil_when_settings_match() {
        let details = vec![DetailForRecalc {
            amount: 500,
            amount_including_tax: Some(0),
            tax_rate: 10,
        }];
        let preferred = (consts::TAX_ROUND_UP, consts::TAX_EXCLUDED);
        assert_eq!(
            find_matching_pattern(&details, 550, preferred),
            Some(preferred),
        );
    }

    /// When the preferred pair does not reproduce the total, the
    /// classifier must fall back to the priority-ordered PATTERNS
    /// list — the same behaviour the old code had for every input.
    #[test]
    fn test_find_matching_pattern_falls_back_to_priority_when_preferred_mismatches() {
        // 100円 × 8% → floor(108) = 108 (matches FLOOR+EXCLUDED),
        // half-up rounds to 108 as well and ceil to 108, so the
        // priority-ordered fallback lands on FLOOR+EXCLUDED first.
        // Store the header as INCLUDED (which would sum to 100 verbatim,
        // not 108) so the preferred check misses.
        let details = vec![DetailForRecalc {
            amount: 100,
            amount_including_tax: Some(0),
            tax_rate: 8,
        }];
        let preferred = (consts::TAX_ROUND_HALF_UP, consts::TAX_INCLUDED);
        assert_eq!(
            find_matching_pattern(&details, 108, preferred),
            Some((consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED)),
            "fallback must pick the first PATTERNS entry that fits"
        );
    }

    /// Sanity: when no pattern — preferred or fallback — reproduces
    /// the target, the classifier returns `None` and the caller
    /// overwrites TOTAL_AMOUNT instead of the setting columns.
    #[test]
    fn test_find_matching_pattern_returns_none_when_no_pattern_fits() {
        let details = vec![DetailForRecalc {
            amount: 100,
            amount_including_tax: Some(0),
            tax_rate: 10,
        }];
        // No settings combination produces 999 from 100@10%.
        let preferred = (consts::TAX_ROUND_HALF_UP, consts::TAX_EXCLUDED);
        assert_eq!(find_matching_pattern(&details, 999, preferred), None);
    }

    async fn setup_test_db() -> SqlitePool {
        // Create in-memory database
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("Failed to create test database");

        // Create USERS table
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_USERS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Insert test user
        sqlx::query(sql_queries::TEST_TRANSACTION_INSERT_USER)
            .execute(&pool)
            .await
            .unwrap();

        // Create MEMOS table
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_MEMOS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Create CATEGORY1 table
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_CATEGORY1_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Insert test category
        sqlx::query(sql_queries::TEST_TRANSACTION_INSERT_CATEGORY1)
            .execute(&pool)
            .await
            .unwrap();

        // Create ACCOUNTS table
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_ACCOUNTS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Insert test accounts
        sqlx::query(sql_queries::TEST_TRANSACTION_INSERT_ACCOUNT_CASH)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(sql_queries::TEST_TRANSACTION_INSERT_ACCOUNT_BANK)
            .execute(&pool)
            .await
            .unwrap();

        // Create TRANSACTIONS_HEADER table
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Create SHOPS table
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_SHOPS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Create CATEGORY2 table
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_CATEGORY2_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Insert test category2
        sqlx::query(sql_queries::TEST_TRANSACTION_INSERT_CATEGORY2)
            .execute(&pool)
            .await
            .unwrap();

        // Create CATEGORY3 table
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_CATEGORY3_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Insert test category3
        sqlx::query(sql_queries::TEST_TRANSACTION_INSERT_CATEGORY3)
            .execute(&pool)
            .await
            .unwrap();

        // Create MANUFACTURERS and PRODUCTS tables (v2.6.0: needed for the
        // LEFT JOIN in TRANSACTION_DETAIL_GET_WITH_INFO; tests don't populate
        // master rows by default, so the JOIN just yields NULLs for the
        // free-text path that most tests exercise).
        sqlx::query(sql_queries::TEST_MANUFACTURER_CREATE_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(sql_queries::TEST_PRODUCT_CREATE_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Create TRANSACTIONS_DETAIL table
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_DETAIL_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Recurring rule tables. MEMO_COUNT_USAGE queries these to detect
        // memos shared with recurring rules; without them, every UPDATE /
        // DELETE path that touches a memo would fail with "no such table"
        // even when the test itself does not exercise recurring rules.
        sqlx::query(sql_queries::CREATE_RECURRING_RULES_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(sql_queries::CREATE_RECURRING_RULE_DETAILS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        pool
    }

    /// Same in-memory schema as `setup_test_db`, plus `PRAGMA foreign_keys
    /// = ON` so tests can exercise production-equivalent FK enforcement
    /// (needed for regression tests around MEMO_DELETE ordering, since the
    /// bug only surfaces when the MEMOS FK is actually enforced).
    async fn setup_test_db_with_foreign_keys() -> SqlitePool {
        let pool = setup_test_db().await;
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();
        pool
    }

    /// Helper: create a transaction header and return its ID
    async fn create_test_header(service: &TransactionService) -> i64 {
        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 10000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        service.save_transaction_header(2, request).await.unwrap()
    }

    /// Helper: create a basic detail request
    fn basic_detail_request() -> SaveTransactionDetailRequest {
        SaveTransactionDetailRequest {
            detail_id: None,
            category1_code: "EXPENSE".to_string(),
            category2_code: Some("FOOD".to_string()),
            category3_code: Some("GROCERY".to_string()),
            item_name: "Rice".to_string(),
            amount: 500,
            tax_rate: 8,
            tax_amount: 40,
            amount_including_tax: Some(540),
            product_id: None,
            memo: None,
        }
    }

    #[tokio::test]
    async fn test_save_transaction_header_with_tax_excluded() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 10000,
            tax_rounding_type: consts::TAX_ROUND_HALF_UP,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: Some("Test transaction".to_string()),
            is_scheduled: None,
        };

        let result = service.save_transaction_header(2, request).await;
        assert!(result.is_ok());
        
        let transaction_id = result.unwrap();
        assert!(transaction_id > 0);

        // Verify the transaction was saved with correct tax_included_type
        let header_result = service.get_transaction_header_with_info(2, transaction_id).await;
        assert!(header_result.is_ok());
        
        let header = header_result.unwrap();
        assert_eq!(header.tax_included_type, consts::TAX_EXCLUDED);
        assert_eq!(header.total_amount, 10000);
    }

    #[tokio::test]
    async fn test_save_transaction_header_with_tax_included() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 10800,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_INCLUDED,
            memo: Some("Tax included transaction".to_string()),
            is_scheduled: None,
        };

        let result = service.save_transaction_header(2, request).await;
        assert!(result.is_ok());
        
        let transaction_id = result.unwrap();
        
        // Verify the transaction was saved with correct tax_included_type
        let header_result = service.get_transaction_header_with_info(2, transaction_id).await;
        assert!(header_result.is_ok());
        
        let header = header_result.unwrap();
        assert_eq!(header.tax_included_type, consts::TAX_INCLUDED);
        assert_eq!(header.total_amount, 10800);
    }

    #[tokio::test]
    async fn test_update_transaction_header_tax_type() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        // Create initial transaction with tax excluded
        let initial_request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 10000,
            tax_rounding_type: consts::TAX_ROUND_HALF_UP,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };

        let transaction_id = service.save_transaction_header(2, initial_request).await.unwrap();

        // Update to tax included
        let update_request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 10800,
            tax_rounding_type: consts::TAX_ROUND_HALF_UP,
            tax_included_type: consts::TAX_INCLUDED,
            memo: None,
            is_scheduled: None,
        };

        let update_result = service.update_transaction_header(2, transaction_id, update_request).await;
        assert!(update_result.is_ok());

        // Verify the update
        let header = service.get_transaction_header_with_info(2, transaction_id).await.unwrap();
        assert_eq!(header.tax_included_type, consts::TAX_INCLUDED);
        assert_eq!(header.total_amount, 10800);
    }

    #[tokio::test]
    async fn test_default_tax_type_is_excluded() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        // Create transaction using default tax type (should be TAX_EXCLUDED = 1)
        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 10000,
            tax_rounding_type: consts::TAX_ROUND_HALF_UP,
            tax_included_type: consts::TAX_EXCLUDED, // Explicitly set default
            memo: None,
            is_scheduled: None,
        };

        let transaction_id = service.save_transaction_header(2, request).await.unwrap();
        let header = service.get_transaction_header_with_info(2, transaction_id).await.unwrap();
        
        assert_eq!(header.tax_included_type, consts::TAX_EXCLUDED);
    }

    #[tokio::test]
    async fn test_tax_type_validation_values() {
        // Test that our constants match expected values
        assert_eq!(consts::TAX_INCLUDED, 0, "TAX_INCLUDED should be 0");
        assert_eq!(consts::TAX_EXCLUDED, 1, "TAX_EXCLUDED should be 1");
    }

    // ========================================================================
    // Transaction Detail Tests
    // ========================================================================

    #[tokio::test]
    async fn test_add_transaction_detail() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = basic_detail_request();
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_ok(), "add_transaction_detail failed: {:?}", result.err());

        let detail_id = result.unwrap();
        assert!(detail_id > 0);
    }

    #[tokio::test]
    async fn test_add_transaction_detail_with_amount_including_tax() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            detail_id: None,
            category1_code: "EXPENSE".to_string(),
            category2_code: Some("FOOD".to_string()),
            category3_code: Some("GROCERY".to_string()),
            item_name: "Bread".to_string(),
            amount: 200,
            tax_rate: 8,
            tax_amount: 16,
            amount_including_tax: Some(216),
            product_id: None,
            memo: Some("Test memo".to_string()),
        };
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        // Verify via get_transaction_details
        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].detail_id, detail_id);
        assert_eq!(details[0].item_name, "Bread");
        assert_eq!(details[0].amount, 200);
        assert_eq!(details[0].tax_amount, 16);
        assert_eq!(details[0].tax_rate, 8);
        assert_eq!(details[0].amount_including_tax, Some(216));
        assert_eq!(details[0].memo_text, Some("Test memo".to_string()));
    }

    #[tokio::test]
    async fn test_add_transaction_detail_without_amount_including_tax() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            detail_id: None,
            category1_code: "EXPENSE".to_string(),
            category2_code: Some("FOOD".to_string()),
            category3_code: Some("GROCERY".to_string()),
            item_name: "Water".to_string(),
            amount: 100,
            tax_rate: 0,
            tax_amount: 0,
            amount_including_tax: None,
            product_id: None,
            memo: None,
        };
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].detail_id, detail_id);
        assert_eq!(details[0].amount_including_tax, None);
        assert_eq!(details[0].memo_text, None);
    }

    #[tokio::test]
    async fn test_add_multiple_details() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request1 = SaveTransactionDetailRequest {
            item_name: "Item A".to_string(),
            amount: 100,
            ..basic_detail_request()
        };
        let request2 = SaveTransactionDetailRequest {
            item_name: "Item B".to_string(),
            amount: 200,
            ..basic_detail_request()
        };

        service.add_transaction_detail(2, transaction_id, request1).await.unwrap();
        service.add_transaction_detail(2, transaction_id, request2).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 2);
        assert_eq!(details[0].item_name, "Item A");
        assert_eq!(details[1].item_name, "Item B");
    }

    #[tokio::test]
    async fn test_update_transaction_detail() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = basic_detail_request();
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        // Update the detail
        let update_request = SaveTransactionDetailRequest {
            detail_id: Some(detail_id),
            category1_code: "EXPENSE".to_string(),
            category2_code: Some("FOOD".to_string()),
            category3_code: Some("GROCERY".to_string()),
            item_name: "Updated Rice".to_string(),
            amount: 600,
            tax_rate: 10,
            tax_amount: 60,
            amount_including_tax: Some(660),
            product_id: None,
            memo: Some("Updated memo".to_string()),
        };
        let result = service.update_transaction_detail(2, detail_id, update_request).await;
        assert!(result.is_ok(), "update_transaction_detail failed: {:?}", result.err());

        // Verify updated values
        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].item_name, "Updated Rice");
        assert_eq!(details[0].amount, 600);
        assert_eq!(details[0].tax_rate, 10);
        assert_eq!(details[0].tax_amount, 60);
        assert_eq!(details[0].amount_including_tax, Some(660));
        assert_eq!(details[0].memo_text, Some("Updated memo".to_string()));
    }

    #[tokio::test]
    async fn test_delete_transaction_detail() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = basic_detail_request();
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        // Verify it exists
        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 1);

        // Delete
        let result = service.delete_transaction_detail(2, detail_id).await;
        assert!(result.is_ok(), "delete_transaction_detail failed: {:?}", result.err());

        // Verify it's gone
        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_transaction_detail_with_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            memo: Some("Memo to delete".to_string()),
            ..basic_detail_request()
        };
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        // Delete (should also delete the memo)
        service.delete_transaction_detail(2, detail_id).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_detail() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let result = service.delete_transaction_detail(2, 99999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_nonexistent_detail() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = basic_detail_request();
        let result = service.update_transaction_detail(2, 99999, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detail_validation_empty_item_name() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            item_name: "".to_string(),
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detail_validation_negative_amount() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            amount: -1,
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detail_validation_invalid_tax_rate() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            tax_rate: 101,
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detail_validation_negative_tax_amount() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            tax_amount: -1,
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detail_validation_item_name_too_long() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            item_name: "A".repeat(201),
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detail_validation_memo_too_long() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            memo: Some("A".repeat(1001)),
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_details_empty_transaction() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 0);
    }

    #[tokio::test]
    async fn test_detail_user_isolation() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = basic_detail_request();
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        // Different user (user_id=999) should not see the detail
        let details = service.get_transaction_details(999, transaction_id).await.unwrap();
        assert_eq!(details.len(), 0);

        // Different user should not be able to update
        let update_request = basic_detail_request();
        let result = service.update_transaction_detail(999, detail_id, update_request).await;
        assert!(result.is_err());

        // Different user should not be able to delete
        let result = service.delete_transaction_detail(999, detail_id).await;
        assert!(result.is_err());
    }

    // ========================================================================
    // Transaction Detail - Boundary Value Tests
    // ========================================================================

    #[tokio::test]
    async fn test_detail_amount_zero() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            amount: 0,
            tax_amount: 0,
            amount_including_tax: Some(0),
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_ok(), "amount=0 should be valid");
    }

    #[tokio::test]
    async fn test_detail_amount_max() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            amount: 999_999_999,
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_ok(), "amount=999_999_999 should be valid");
    }

    #[tokio::test]
    async fn test_detail_amount_over_max() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            amount: 1_000_000_000,
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_err(), "amount=1_000_000_000 should be invalid");
    }

    #[tokio::test]
    async fn test_detail_tax_rate_negative() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            tax_rate: -1,
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_err(), "tax_rate=-1 should be invalid");
    }

    #[tokio::test]
    async fn test_detail_tax_rate_boundary_values() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        // tax_rate=0 should be valid
        let request = SaveTransactionDetailRequest {
            tax_rate: 0,
            ..basic_detail_request()
        };
        assert!(service.add_transaction_detail(2, transaction_id, request).await.is_ok());

        // tax_rate=100 should be valid
        let request = SaveTransactionDetailRequest {
            tax_rate: 100,
            ..basic_detail_request()
        };
        assert!(service.add_transaction_detail(2, transaction_id, request).await.is_ok());
    }

    #[tokio::test]
    async fn test_add_detail_without_category2_category3() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            detail_id: None,
            category1_code: "EXPENSE".to_string(),
            category2_code: None,
            category3_code: None,
            item_name: "Miscellaneous".to_string(),
            amount: 300,
            tax_rate: 0,
            tax_amount: 0,
            amount_including_tax: None,
            product_id: None,
            memo: None,
        };
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].detail_id, detail_id);
        assert_eq!(details[0].category1_code, "EXPENSE");
        assert_eq!(details[0].category2_code, None);
        assert_eq!(details[0].category3_code, None);
        assert_eq!(details[0].item_name, "Miscellaneous");
        assert_eq!(details[0].amount, 300);
    }

    #[tokio::test]
    async fn test_add_detail_with_category2_only() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            detail_id: None,
            category1_code: "EXPENSE".to_string(),
            category2_code: Some("FOOD".to_string()),
            category3_code: None,
            item_name: "Lunch".to_string(),
            amount: 800,
            tax_rate: 10,
            tax_amount: 80,
            amount_including_tax: Some(880),
            product_id: None,
            memo: None,
        };
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].detail_id, detail_id);
        assert_eq!(details[0].category2_code, Some("FOOD".to_string()));
        assert_eq!(details[0].category3_code, None);
    }

    #[tokio::test]
    async fn test_detail_item_name_whitespace_only() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            item_name: "   ".to_string(),
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_err(), "whitespace-only item_name should be invalid");
    }

    #[tokio::test]
    async fn test_detail_item_name_exactly_200_chars() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            item_name: "A".repeat(200),
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_ok(), "item_name of exactly 200 chars should be valid");
    }

    #[tokio::test]
    async fn test_detail_memo_empty_string() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            memo: Some("".to_string()),
            ..basic_detail_request()
        };
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].detail_id, detail_id);
        assert_eq!(details[0].memo_text, None, "empty string memo should result in no memo");
    }

    #[tokio::test]
    async fn test_detail_memo_exactly_1000_chars() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionDetailRequest {
            memo: Some("A".repeat(1000)),
            ..basic_detail_request()
        };
        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_ok(), "memo of exactly 1000 chars should be valid");
    }

    // ========================================================================
    // Transaction Detail - Update Memo Patterns
    // ========================================================================

    #[tokio::test]
    async fn test_update_detail_add_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        // Create detail without memo
        let request = basic_detail_request();
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        // Update to add memo
        let update = SaveTransactionDetailRequest {
            detail_id: Some(detail_id),
            memo: Some("New memo".to_string()),
            ..basic_detail_request()
        };
        service.update_transaction_detail(2, detail_id, update).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details[0].memo_text, Some("New memo".to_string()));
    }

    #[tokio::test]
    async fn test_update_detail_change_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        // Create detail with memo
        let request = SaveTransactionDetailRequest {
            memo: Some("Original".to_string()),
            ..basic_detail_request()
        };
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        // Update memo text
        let update = SaveTransactionDetailRequest {
            detail_id: Some(detail_id),
            memo: Some("Changed".to_string()),
            ..basic_detail_request()
        };
        service.update_transaction_detail(2, detail_id, update).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details[0].memo_text, Some("Changed".to_string()));
    }

    #[tokio::test]
    async fn test_update_detail_remove_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        // Create detail with memo
        let request = SaveTransactionDetailRequest {
            memo: Some("To be removed".to_string()),
            ..basic_detail_request()
        };
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        // Update with empty memo to remove
        let update = SaveTransactionDetailRequest {
            detail_id: Some(detail_id),
            memo: Some("".to_string()),
            ..basic_detail_request()
        };
        service.update_transaction_detail(2, detail_id, update).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details[0].memo_text, None);
    }

    #[tokio::test]
    async fn test_update_detail_keep_memo_with_none() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        // Create detail with memo
        let request = SaveTransactionDetailRequest {
            memo: Some("Keep this".to_string()),
            ..basic_detail_request()
        };
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        // Update with memo=None (should keep existing memo)
        let update = SaveTransactionDetailRequest {
            detail_id: Some(detail_id),
            item_name: "Updated name".to_string(),
            memo: None,
            ..basic_detail_request()
        };
        service.update_transaction_detail(2, detail_id, update).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details[0].item_name, "Updated name");
        assert_eq!(details[0].memo_text, Some("Keep this".to_string()));
    }

    #[tokio::test]
    async fn test_update_detail_validation_empty_item_name() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = basic_detail_request();
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        let update = SaveTransactionDetailRequest {
            detail_id: Some(detail_id),
            item_name: "".to_string(),
            ..basic_detail_request()
        };
        let result = service.update_transaction_detail(2, detail_id, update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_detail_validation_negative_amount() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = basic_detail_request();
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        let update = SaveTransactionDetailRequest {
            detail_id: Some(detail_id),
            amount: -1,
            ..basic_detail_request()
        };
        let result = service.update_transaction_detail(2, detail_id, update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_detail_validation_memo_too_long() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = basic_detail_request();
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        let update = SaveTransactionDetailRequest {
            detail_id: Some(detail_id),
            memo: Some("A".repeat(1001)),
            ..basic_detail_request()
        };
        let result = service.update_transaction_detail(2, detail_id, update).await;
        assert!(result.is_err());
    }

    // ========================================================================
    // Transaction Header - Validation Tests (previously missing)
    // ========================================================================

    #[tokio::test]
    async fn test_save_header_invalid_date_format() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01".to_string(), // Missing time part
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.save_transaction_header(2, request).await;
        assert!(result.is_err(), "short date format should be rejected");
    }

    #[tokio::test]
    async fn test_save_header_negative_amount() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: -1,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.save_transaction_header(2, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_save_header_amount_over_max() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 1_000_000_000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.save_transaction_header(2, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_save_header_amount_zero() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 0,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.save_transaction_header(2, request).await;
        assert!(result.is_ok(), "amount=0 should be valid");
    }

    #[tokio::test]
    async fn test_save_header_invalid_tax_rounding_type() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: 99,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.save_transaction_header(2, request).await;
        assert!(result.is_err(), "invalid tax_rounding_type should be rejected");
    }

    #[tokio::test]
    async fn test_save_header_memo_too_long() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: Some("A".repeat(1001)),
            is_scheduled: None,
        };
        let result = service.save_transaction_header(2, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_save_header_empty_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: Some("".to_string()),
            is_scheduled: None,
        };
        let result = service.save_transaction_header(2, request).await;
        assert!(result.is_ok(), "empty memo should be treated as no memo");
    }

    #[tokio::test]
    async fn test_save_header_with_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 5000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: Some("Header memo".to_string()),
            is_scheduled: None,
        };
        let transaction_id = service.save_transaction_header(2, request).await.unwrap();

        let (header, memo_text) = service.get_transaction_header_with_memo(2, transaction_id).await.unwrap();
        assert!(header.memo_id.is_some());
        assert_eq!(memo_text, Some("Header memo".to_string()));
    }

    #[tokio::test]
    async fn test_save_header_all_tax_rounding_types() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        for rounding_type in [consts::TAX_ROUND_DOWN, consts::TAX_ROUND_HALF_UP, consts::TAX_ROUND_UP] {
            let request = SaveTransactionRequest {
                shop_id: None,
                category1_code: "EXPENSE".to_string(),
                from_account_code: "CASH".to_string(),
                to_account_code: "BANK".to_string(),
                transaction_date: "2024-01-01 10:00:00".to_string(),
                total_amount: 1000,
                tax_rounding_type: rounding_type,
                tax_included_type: consts::TAX_EXCLUDED,
                memo: None,
                is_scheduled: None,
            };
            let result = service.save_transaction_header(2, request).await;
            assert!(result.is_ok(), "tax_rounding_type={} should be valid", rounding_type);
        }
    }

    #[tokio::test]
    async fn test_update_header_nonexistent() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.update_transaction_header(2, 99999, request).await;
        assert!(result.is_err(), "updating nonexistent header should fail");
    }

    #[tokio::test]
    async fn test_update_header_invalid_date() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "bad-date".to_string(),
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.update_transaction_header(2, transaction_id, request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_header_invalid_tax_rounding() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: 5,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.update_transaction_header(2, transaction_id, request).await;
        assert!(result.is_err());
    }

    /// CodeRabbit review on #125 — before the fix, `save_transaction_header`
    /// only validated `tax_rounding_type` and let an unknown
    /// `tax_included_type` (e.g. 99) reach the DB. `find_matching_pattern`
    /// would then take the bogus value as `preferred` and, with the
    /// preferred-first check, silently keep it across bulk recalc.
    /// Guard added at the service entry point rejects the write outright.
    #[tokio::test]
    async fn test_save_header_rejects_invalid_tax_included_type() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: 99,
            memo: None,
            is_scheduled: None,
        };
        let result = service.save_transaction_header(2, request).await;
        assert!(
            matches!(result, Err(TransactionError::ValidationError(_))),
            "unknown tax_included_type must be rejected, got {:?}",
            result
        );
    }

    /// Companion pin for `update_transaction_header` — same
    /// class-of-defect blocked at the update entry point.
    #[tokio::test]
    async fn test_update_header_rejects_invalid_tax_included_type() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: 42,
            memo: None,
            is_scheduled: None,
        };
        let result = service.update_transaction_header(2, transaction_id, request).await;
        assert!(
            matches!(result, Err(TransactionError::ValidationError(_))),
            "unknown tax_included_type must be rejected on update, got {:?}",
            result
        );
    }

    /// Fable-5 review #20 — TRANSFER with the same source and
    /// destination account nets to zero but historically inflated the
    /// dashboard balance because `ACCOUNT_BALANCES_AS_OF` counted the
    /// +TO arm first and stopped. The write-side guard now rejects
    /// this outright at both entry points.
    #[tokio::test]
    async fn test_save_header_rejects_transfer_from_equals_to() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "TRANSFER".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "CASH".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 2000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.save_transaction_header(2, request).await;
        assert!(
            matches!(result, Err(TransactionError::TransferSameAccount)),
            "TRANSFER with from == to must be rejected with the typed variant, got {:?}",
            result
        );
    }

    /// Companion pin for `update_transaction_header` — same guard on
    /// the edit path so a valid TRANSFER cannot be mutated into a
    /// self-transfer.
    #[tokio::test]
    async fn test_update_header_rejects_transfer_from_equals_to() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "TRANSFER".to_string(),
            from_account_code: "BANK".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 3000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let result = service.update_transaction_header(2, transaction_id, request).await;
        assert!(
            matches!(result, Err(TransactionError::TransferSameAccount)),
            "TRANSFER with from == to must be rejected on update with the typed variant, got {:?}",
            result
        );
    }

    // ========================================================================
    // IS_SCHEDULED Tests
    // ========================================================================

    #[tokio::test]
    async fn test_save_scheduled_transaction() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-02-01 10:00:00".to_string(),
            total_amount: 5000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: Some(1),
        };
        let transaction_id = service.save_transaction_header(2, request).await.unwrap();

        let header = service.get_transaction_header(2, transaction_id).await.unwrap();
        assert_eq!(header.is_scheduled, 1);
    }

    #[tokio::test]
    async fn test_save_default_not_scheduled() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-02-01 10:00:00".to_string(),
            total_amount: 5000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let transaction_id = service.save_transaction_header(2, request).await.unwrap();

        let header = service.get_transaction_header(2, transaction_id).await.unwrap();
        assert_eq!(header.is_scheduled, 0);
    }

    #[tokio::test]
    async fn test_confirm_scheduled_transaction() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        // Create a scheduled transaction
        let request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-02-01 10:00:00".to_string(),
            total_amount: 5000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: Some(1),
        };
        let transaction_id = service.save_transaction_header(2, request).await.unwrap();

        // Confirm it
        service.confirm_scheduled_transaction(2, transaction_id).await.unwrap();

        // Verify it's now actual
        let header = service.get_transaction_header(2, transaction_id).await.unwrap();
        assert_eq!(header.is_scheduled, 0);
    }

    #[tokio::test]
    async fn test_confirm_already_actual_transaction_fails() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        // Create a normal (actual) transaction
        let transaction_id = create_test_header(&service).await;

        // Trying to confirm should fail (it's already actual, IS_SCHEDULED = 0)
        let result = service.confirm_scheduled_transaction(2, transaction_id).await;
        assert!(result.is_err());
    }

    /// Regression test: filtering by category2_code/category3_code used to be
    /// silently ignored (the placeholder discarded the value via `let _ = ...`).
    /// As a result the list page returned every row of the parent category1.
    #[tokio::test]
    async fn test_get_transactions_filters_by_category2_and_category3() {
        let pool = setup_test_db().await;

        // Add a second CATEGORY2/CATEGORY3 so we can split detail rows across them.
        sqlx::query(
            "INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, DISPLAY_ORDER, CATEGORY2_NAME) \
             VALUES (2, 'EXPENSE', 'OTHER', 2, 'その他')",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO CATEGORY3 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, DISPLAY_ORDER, CATEGORY3_NAME) \
             VALUES (2, 'EXPENSE', 'OTHER', 'MISC', 1, 'その他雑費')",
        )
        .execute(&pool)
        .await
        .unwrap();

        let service = TransactionService::new(pool);

        // Header A: a single FOOD/GROCERY detail line.
        let header_a = create_test_header(&service).await;
        service
            .add_transaction_detail(2, header_a, basic_detail_request())
            .await
            .unwrap();

        // Header B: a single OTHER/MISC detail line (same CATEGORY1 = EXPENSE).
        let header_b = create_test_header(&service).await;
        service
            .add_transaction_detail(
                2,
                header_b,
                SaveTransactionDetailRequest {
                    detail_id: None,
                    category1_code: "EXPENSE".to_string(),
                    category2_code: Some("OTHER".to_string()),
                    category3_code: Some("MISC".to_string()),
                    item_name: "Misc".to_string(),
                    amount: 200,
                    tax_rate: 8,
                    tax_amount: 16,
                    amount_including_tax: Some(216),
                    product_id: None,
                    memo: None,
                },
            )
            .await
            .unwrap();

        // No filter: both transactions returned.
        let all = service
            .get_transactions(2, None, None, None, None, None, None, None, None, false, 1, 50)
            .await
            .unwrap();
        assert_eq!(all.total_count, 2);

        // CATEGORY2 = FOOD: only header A.
        let food = service
            .get_transactions(2, None, None, Some("EXPENSE"), Some("FOOD"), None, None, None, None, false, 1, 50)
            .await
            .unwrap();
        assert_eq!(food.total_count, 1);
        assert_eq!(food.transactions[0].transaction_id, header_a);

        // CATEGORY2 = OTHER: only header B.
        let other = service
            .get_transactions(2, None, None, Some("EXPENSE"), Some("OTHER"), None, None, None, None, false, 1, 50)
            .await
            .unwrap();
        assert_eq!(other.total_count, 1);
        assert_eq!(other.transactions[0].transaction_id, header_b);

        // CATEGORY3 = GROCERY: only header A.
        let grocery = service
            .get_transactions(
                2,
                None,
                None,
                Some("EXPENSE"),
                Some("FOOD"),
                Some("GROCERY"),
                None,
                None,
                None,
                false,
                1,
                50,
            )
            .await
            .unwrap();
        assert_eq!(grocery.total_count, 1);
        assert_eq!(grocery.transactions[0].transaction_id, header_a);

        // Empty-string filter must be treated as "no filter".
        let empty = service
            .get_transactions(2, None, None, Some(""), Some(""), Some(""), None, None, None, false, 1, 50)
            .await
            .unwrap();
        assert_eq!(empty.total_count, 2);
    }

    /// Regression: an end-date filter of 'YYYY-MM-DD' used to string-compare
    /// against 'YYYY-MM-DD HH:MM:SS' in TRANSACTION_DATE and silently drop
    /// every same-day timestamp. The boundary day must be inclusive.
    #[tokio::test]
    async fn test_get_transactions_end_date_includes_boundary_day() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let mut request = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-05-10 09:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        service.save_transaction_header(2, request.clone()).await.unwrap();

        request.transaction_date = "2024-05-15 12:00:00".to_string();
        service.save_transaction_header(2, request.clone()).await.unwrap();

        request.transaction_date = "2024-05-15 23:30:00".to_string();
        service.save_transaction_header(2, request.clone()).await.unwrap();

        request.transaction_date = "2024-05-16 08:00:00".to_string();
        service.save_transaction_header(2, request).await.unwrap();

        // end=2024-05-15 must include both same-day rows (12:00 and 23:30).
        let result = service
            .get_transactions(
                2,
                Some("2024-05-10"),
                Some("2024-05-15"),
                None, None, None, None, None, None,
                false, 1, 50,
            )
            .await
            .unwrap();
        assert_eq!(result.total_count, 3);

        // Bare same-day range must return the two rows on that day.
        let single_day = service
            .get_transactions(
                2,
                Some("2024-05-15"),
                Some("2024-05-15"),
                None, None, None, None, None, None,
                false, 1, 50,
            )
            .await
            .unwrap();
        assert_eq!(single_day.total_count, 2);

        // Datetime-form end must still work unchanged.
        let datetime_end = service
            .get_transactions(
                2,
                None,
                Some("2024-05-15 12:00:00"),
                None, None, None, None, None, None,
                false, 1, 50,
            )
            .await
            .unwrap();
        assert_eq!(datetime_end.total_count, 2);
    }

    /// Regression: the keyword parameter used to be discarded (`let _ = keyword`),
    /// leaving the search box a silent no-op. Keyword must substring-match
    /// against memos on the header row and on any detail row of the header.
    #[tokio::test]
    async fn test_get_transactions_keyword_matches_header_and_detail_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        // Header A: memo on header only.
        let header_a = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-06-01 10:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: Some("駅前スーパーの牛乳".to_string()),
            is_scheduled: None,
        };
        let id_a = service.save_transaction_header(2, header_a).await.unwrap();

        // Header B: memo on detail only.
        let header_b = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-06-02 10:00:00".to_string(),
            total_amount: 500,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        let id_b = service.save_transaction_header(2, header_b).await.unwrap();
        let mut detail = basic_detail_request();
        detail.memo = Some("ヨーグルト特売".to_string());
        service.add_transaction_detail(2, id_b, detail).await.unwrap();

        // Header C: no memo at all — must never match.
        let header_c = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-06-03 10:00:00".to_string(),
            total_amount: 300,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        service.save_transaction_header(2, header_c).await.unwrap();

        // Keyword hits header memo only.
        let hit_header = service
            .get_transactions(
                2, None, None, None, None, None, None, None,
                Some("牛乳"), false, 1, 50,
            )
            .await
            .unwrap();
        assert_eq!(hit_header.total_count, 1);
        assert_eq!(hit_header.transactions[0].transaction_id, id_a);

        // Keyword hits detail memo only.
        let hit_detail = service
            .get_transactions(
                2, None, None, None, None, None, None, None,
                Some("特売"), false, 1, 50,
            )
            .await
            .unwrap();
        assert_eq!(hit_detail.total_count, 1);
        assert_eq!(hit_detail.transactions[0].transaction_id, id_b);

        // No match returns no rows.
        let miss = service
            .get_transactions(
                2, None, None, None, None, None, None, None,
                Some("ダミー"), false, 1, 50,
            )
            .await
            .unwrap();
        assert_eq!(miss.total_count, 0);

        // Whitespace-only keyword is treated as no filter.
        let whitespace = service
            .get_transactions(
                2, None, None, None, None, None, None, None,
                Some("   "), false, 1, 50,
            )
            .await
            .unwrap();
        assert_eq!(whitespace.total_count, 3);

        // LIKE metacharacter '%' must be escaped — it should match literally,
        // not as a wildcard.
        let percent_miss = service
            .get_transactions(
                2, None, None, None, None, None, None, None,
                Some("%"), false, 1, 50,
            )
            .await
            .unwrap();
        assert_eq!(percent_miss.total_count, 0);
    }

    #[tokio::test]
    async fn test_get_transactions_excludes_scheduled_by_default() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        // Create an actual transaction
        let request_actual = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-02-01 10:00:00".to_string(),
            total_amount: 1000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: None,
        };
        service.save_transaction_header(2, request_actual).await.unwrap();

        // Create a scheduled transaction
        let request_scheduled = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-02-02 10:00:00".to_string(),
            total_amount: 2000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: None,
            is_scheduled: Some(1),
        };
        service.save_transaction_header(2, request_scheduled).await.unwrap();

        // Default: exclude scheduled
        let result = service.get_transactions(
            2, None, None, None, None, None, None, None, None, false, 1, 50
        ).await.unwrap();
        assert_eq!(result.total_count, 1);

        // Include scheduled
        let result = service.get_transactions(
            2, None, None, None, None, None, None, None, None, true, 1, 50
        ).await.unwrap();
        assert_eq!(result.total_count, 2);
    }

    // Issue #37 — bounded-field length checks must count characters, not bytes.
    // Japanese characters are 3 bytes in UTF-8; the previous `.len()` check
    // capped Japanese input at ~MAX/3 characters even though the message said
    // "MAX characters or less".

    #[tokio::test]
    async fn test_item_name_accepts_max_chars_of_multibyte_input() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        // 200 Japanese characters = 600 bytes — would have been rejected by
        // the old byte-based check, must pass the new character-based check.
        let item_name: String = "あ".repeat(consts::MAX_ITEM_NAME_LEN);
        let mut request = basic_detail_request();
        request.item_name = item_name;

        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_ok(), "expected MAX_ITEM_NAME_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_item_name_rejects_over_max_chars_of_multibyte_input() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let item_name: String = "あ".repeat(consts::MAX_ITEM_NAME_LEN + 1);
        let mut request = basic_detail_request();
        request.item_name = item_name;

        let err = service.add_transaction_detail(2, transaction_id, request).await.unwrap_err();
        match err {
            TransactionError::ValidationError(msg) => {
                assert!(msg.contains(&consts::MAX_ITEM_NAME_LEN.to_string()),
                    "error message should reference the limit: {}", msg);
            }
            other => panic!("expected ValidationError, got {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_memo_accepts_max_chars_of_multibyte_input() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let memo: String = "あ".repeat(consts::MAX_MEMO_LEN);
        let mut request = basic_detail_request();
        request.memo = Some(memo);

        let result = service.add_transaction_detail(2, transaction_id, request).await;
        assert!(result.is_ok(), "expected MAX_MEMO_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_memo_rejects_over_max_chars_of_multibyte_input() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let memo: String = "あ".repeat(consts::MAX_MEMO_LEN + 1);
        let mut request = basic_detail_request();
        request.memo = Some(memo);

        let err = service.add_transaction_detail(2, transaction_id, request).await.unwrap_err();
        match err {
            TransactionError::ValidationError(msg) => {
                assert!(msg.contains(&consts::MAX_MEMO_LEN.to_string()),
                    "error message should reference the limit: {}", msg);
            }
            other => panic!("expected ValidationError, got {:?}", other),
        }
    }

    // v2.6.0 master integration: product_id round-trip on TRANSACTIONS_DETAIL

    /// Helper: seed a manufacturer + product into the test pool and return
    /// (manufacturer_id, product_id). Reused by the master-integration tests.
    async fn seed_master_pair(pool: &sqlx::SqlitePool) -> (i64, i64) {
        let manuf_result = sqlx::query(sql_queries::MANUFACTURER_INSERT)
            .bind(2_i64)
            .bind("ニッスイ")
            .bind::<Option<&str>>(None)
            .bind(1_i64)
            .bind(0_i64)
            .execute(pool)
            .await
            .unwrap();
        let manufacturer_id = manuf_result.last_insert_rowid();

        let prod_result = sqlx::query(sql_queries::PRODUCT_INSERT)
            .bind(2_i64)
            .bind("サバ缶")
            .bind(Some(manufacturer_id))
            .bind::<Option<&str>>(None)
            .bind(1_i64)
            .bind(0_i64)
            .execute(pool)
            .await
            .unwrap();
        (manufacturer_id, prod_result.last_insert_rowid())
    }

    #[tokio::test]
    async fn test_add_detail_with_product_id_round_trips() {
        let pool = setup_test_db().await;
        let (_, product_id) = seed_master_pair(&pool).await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        let mut request = basic_detail_request();
        request.product_id = Some(product_id);
        let detail_id = service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].detail_id, detail_id);
        assert_eq!(details[0].product_id, Some(product_id));
        assert_eq!(details[0].product_name.as_deref(), Some("サバ缶"));
        assert_eq!(details[0].manufacturer_name.as_deref(), Some("ニッスイ"));
    }

    #[tokio::test]
    async fn test_add_detail_without_product_id_yields_free_text_row() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        // Default basic_detail_request has product_id: None
        let request = basic_detail_request();
        service.add_transaction_detail(2, transaction_id, request).await.unwrap();

        let details = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(details.len(), 1);
        assert_eq!(details[0].product_id, None);
        assert_eq!(details[0].product_name, None);
        assert_eq!(details[0].manufacturer_name, None);
    }

    #[tokio::test]
    async fn test_update_detail_can_set_then_clear_product_id() {
        let pool = setup_test_db().await;
        let (_, product_id) = seed_master_pair(&pool).await;
        let service = TransactionService::new(pool);
        let transaction_id = create_test_header(&service).await;

        // Start as free text
        let detail_id = service.add_transaction_detail(2, transaction_id, basic_detail_request()).await.unwrap();

        // Promote to product-linked
        let mut linked = basic_detail_request();
        linked.product_id = Some(product_id);
        service.update_transaction_detail(2, detail_id, linked).await.unwrap();
        let after_link = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(after_link[0].product_id, Some(product_id));

        // Demote back to free text (user typed over the master link)
        let mut unlinked = basic_detail_request();
        unlinked.product_id = None;
        service.update_transaction_detail(2, detail_id, unlinked).await.unwrap();
        let after_unlink = service.get_transaction_details(2, transaction_id).await.unwrap();
        assert_eq!(after_unlink[0].product_id, None);
    }

    /// Set up a Header/Detail pair that share a MEMO_ID because
    /// `get_memo_id_for_update` reused the detail's memo row via
    /// `MEMO_FIND_BY_TEXT`. Returns (header_a_id, detail_a_id, header_b_id,
    /// shared_memo_id).
    async fn seed_shared_memo(
        service: &TransactionService,
        text: &str,
    ) -> (i64, i64, i64, i64) {
        let header_a_id = create_test_header(service).await;
        let mut detail_req = basic_detail_request();
        detail_req.memo = Some(text.to_string());
        let detail_a_id = service
            .add_transaction_detail(2, header_a_id, detail_req)
            .await
            .unwrap();
        let shared_memo_id = service
            .get_transaction_details(2, header_a_id)
            .await
            .unwrap()[0]
            .memo_id
            .unwrap();

        let header_b_id = create_test_header(service).await;
        let update_req = SaveTransactionRequest {
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "CASH".to_string(),
            to_account_code: "BANK".to_string(),
            transaction_date: "2024-01-01 10:00:00".to_string(),
            total_amount: 10000,
            tax_rounding_type: consts::TAX_ROUND_DOWN,
            tax_included_type: consts::TAX_EXCLUDED,
            memo: Some(text.to_string()),
            is_scheduled: None,
        };
        service
            .update_transaction_header(2, header_b_id, update_req)
            .await
            .unwrap();
        let header_b = service
            .get_transaction_header_with_info(2, header_b_id)
            .await
            .unwrap();
        assert_eq!(header_b.memo_id, Some(shared_memo_id),
            "sanity: header update must reuse detail's memo row");

        (header_a_id, detail_a_id, header_b_id, shared_memo_id)
    }

    /// Regression for Fable 5 review item 5. When a header and a detail
    /// share the same MEMO_ID (via MEMO_FIND_BY_TEXT reuse on header
    /// update), editing the detail's memo used to mutate the row in place
    /// and silently change the header's memo too.
    #[tokio::test]
    async fn test_update_detail_memo_does_not_corrupt_shared_header_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);
        let (header_a_id, detail_a_id, header_b_id, shared_memo_id) =
            seed_shared_memo(&service, "shared_text").await;

        let mut edit = basic_detail_request();
        edit.memo = Some("changed_by_detail".to_string());
        service
            .update_transaction_detail(2, detail_a_id, edit)
            .await
            .unwrap();

        // Header B's memo must not have moved.
        let header_b_after = service
            .get_transaction_header_with_info(2, header_b_id)
            .await
            .unwrap();
        assert_eq!(header_b_after.memo_id, Some(shared_memo_id));
        assert_eq!(header_b_after.memo_text.as_deref(), Some("shared_text"));

        // Detail A's memo must have been redirected to a fresh row.
        let details_a_after = service
            .get_transaction_details(2, header_a_id)
            .await
            .unwrap();
        assert_eq!(details_a_after[0].memo_text.as_deref(), Some("changed_by_detail"));
        assert_ne!(details_a_after[0].memo_id.unwrap(), shared_memo_id);
    }

    /// Regression for Fable 5 review item 5. Deleting a detail whose
    /// MEMO_ID is shared with a header used to unconditionally DELETE
    /// the memo row and leave the header's MEMO_ID dangling.
    #[tokio::test]
    async fn test_delete_detail_preserves_memo_still_referenced_by_header() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool.clone());
        let (_header_a_id, detail_a_id, header_b_id, shared_memo_id) =
            seed_shared_memo(&service, "shared").await;

        service.delete_transaction_detail(2, detail_a_id).await.unwrap();

        let still_there: Option<String> =
            sqlx::query_scalar("SELECT MEMO_TEXT FROM MEMOS WHERE MEMO_ID = ?")
                .bind(shared_memo_id)
                .fetch_optional(&pool)
                .await
                .unwrap();
        assert_eq!(still_there.as_deref(), Some("shared"));

        let header_b_after = service
            .get_transaction_header_with_info(2, header_b_id)
            .await
            .unwrap();
        assert_eq!(header_b_after.memo_text.as_deref(), Some("shared"));
    }

    /// A memo used only by one detail (no sharing) must still be updated
    /// in place — the sharing check should not force a redundant row.
    #[tokio::test]
    async fn test_update_detail_memo_updates_in_place_when_not_shared() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let header_id = create_test_header(&service).await;
        let mut detail_req = basic_detail_request();
        detail_req.memo = Some("first".to_string());
        let detail_id = service
            .add_transaction_detail(2, header_id, detail_req)
            .await
            .unwrap();
        let original_memo_id = service
            .get_transaction_details(2, header_id)
            .await
            .unwrap()[0]
            .memo_id
            .unwrap();

        let mut edit = basic_detail_request();
        edit.memo = Some("second".to_string());
        service.update_transaction_detail(2, detail_id, edit).await.unwrap();

        let after = service.get_transaction_details(2, header_id).await.unwrap();
        assert_eq!(after[0].memo_text.as_deref(), Some("second"));
        assert_eq!(after[0].memo_id, Some(original_memo_id));
    }

    /// A memo used only by one detail must be deleted when the detail
    /// is removed (no orphaned rows).
    #[tokio::test]
    async fn test_delete_detail_removes_orphaned_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool.clone());

        let header_id = create_test_header(&service).await;
        let mut detail_req = basic_detail_request();
        detail_req.memo = Some("lonely".to_string());
        let detail_id = service
            .add_transaction_detail(2, header_id, detail_req)
            .await
            .unwrap();
        let memo_id = service
            .get_transaction_details(2, header_id)
            .await
            .unwrap()[0]
            .memo_id
            .unwrap();

        service.delete_transaction_detail(2, detail_id).await.unwrap();

        let leftover: Option<String> =
            sqlx::query_scalar("SELECT MEMO_TEXT FROM MEMOS WHERE MEMO_ID = ?")
                .bind(memo_id)
                .fetch_optional(&pool)
                .await
                .unwrap();
        assert!(leftover.is_none(), "orphaned memo should have been deleted");
    }

    /// Clearing a shared detail's memo must release only this detail's
    /// reference — the header that shares the MEMO_ID must keep its text.
    #[tokio::test]
    async fn test_clear_detail_memo_does_not_delete_memo_still_used_by_header() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool.clone());
        let (_header_a_id, detail_a_id, header_b_id, shared_memo_id) =
            seed_shared_memo(&service, "keep_me").await;

        let mut cleared = basic_detail_request();
        cleared.memo = Some("".to_string());
        service
            .update_transaction_detail(2, detail_a_id, cleared)
            .await
            .unwrap();

        let still_there: Option<String> =
            sqlx::query_scalar("SELECT MEMO_TEXT FROM MEMOS WHERE MEMO_ID = ?")
                .bind(shared_memo_id)
                .fetch_optional(&pool)
                .await
                .unwrap();
        assert_eq!(still_there.as_deref(), Some("keep_me"));

        let header_b_after = service
            .get_transaction_header_with_info(2, header_b_id)
            .await
            .unwrap();
        assert_eq!(header_b_after.memo_text.as_deref(), Some("keep_me"));
    }

    /// Insert a minimal RECURRING_RULES row that references `memo_id`.
    /// Used to reproduce sharing between a recurring rule and a
    /// transaction detail — MEMO_COUNT_USAGE must count this row.
    async fn seed_recurring_rule_referencing_memo(pool: &SqlitePool, memo_id: i64) -> i64 {
        let result = sqlx::query(
            "INSERT INTO RECURRING_RULES (
                USER_ID, PERIOD_UNIT, PERIOD_INTERVAL,
                START_DATE, END_DATE,
                CATEGORY1_CODE, FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE,
                TOTAL_AMOUNT, TAX_INCLUDED_TYPE, MEMO_ID
            ) VALUES (2, 'MONTHLY', 1, '2024-01-01', '2024-12-31',
                      'EXPENSE', 'CASH', 'BANK', 1000, 1, ?)",
        )
        .bind(memo_id)
        .execute(pool)
        .await
        .unwrap();
        result.last_insert_rowid()
    }

    /// Regression for Devin review of PR #82. When a detail's MEMO_ID is
    /// shared with a RECURRING_RULES row (which happens naturally because
    /// generated occurrences copy the rule's MEMO_ID), editing the detail's
    /// memo used to rewrite the recurring rule's memo too — MEMO_COUNT_USAGE
    /// only counted TRANSACTIONS_HEADER / _DETAIL and missed the rule.
    #[tokio::test]
    async fn test_update_detail_memo_does_not_corrupt_recurring_rule_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool.clone());

        let header_id = create_test_header(&service).await;
        let mut detail_req = basic_detail_request();
        detail_req.memo = Some("shared_with_recurring".to_string());
        let detail_id = service
            .add_transaction_detail(2, header_id, detail_req)
            .await
            .unwrap();
        let shared_memo_id = service
            .get_transaction_details(2, header_id)
            .await
            .unwrap()[0]
            .memo_id
            .unwrap();

        // Recurring rule also references this memo.
        seed_recurring_rule_referencing_memo(&pool, shared_memo_id).await;

        let mut edit = basic_detail_request();
        edit.memo = Some("edited_via_detail".to_string());
        service.update_transaction_detail(2, detail_id, edit).await.unwrap();

        // The recurring rule's memo text must remain untouched.
        let rule_memo: Option<String> =
            sqlx::query_scalar("SELECT MEMO_TEXT FROM MEMOS WHERE MEMO_ID = ?")
                .bind(shared_memo_id)
                .fetch_optional(&pool)
                .await
                .unwrap();
        assert_eq!(rule_memo.as_deref(), Some("shared_with_recurring"));

        // Detail has been redirected to a fresh memo.
        let after = service.get_transaction_details(2, header_id).await.unwrap();
        assert_eq!(after[0].memo_text.as_deref(), Some("edited_via_detail"));
        assert_ne!(after[0].memo_id.unwrap(), shared_memo_id);
    }

    /// Regression for Devin review of PR #82. Deleting a detail whose
    /// MEMO_ID is shared with a RECURRING_RULES row used to unconditionally
    /// DELETE the memo row (and under production FK enforcement, blow up
    /// with FOREIGN KEY constraint failed).
    #[tokio::test]
    async fn test_delete_detail_preserves_memo_still_referenced_by_recurring_rule() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool.clone());

        let header_id = create_test_header(&service).await;
        let mut detail_req = basic_detail_request();
        detail_req.memo = Some("keep_for_rule".to_string());
        let detail_id = service
            .add_transaction_detail(2, header_id, detail_req)
            .await
            .unwrap();
        let shared_memo_id = service
            .get_transaction_details(2, header_id)
            .await
            .unwrap()[0]
            .memo_id
            .unwrap();

        seed_recurring_rule_referencing_memo(&pool, shared_memo_id).await;

        service.delete_transaction_detail(2, detail_id).await.unwrap();

        let still_there: Option<String> =
            sqlx::query_scalar("SELECT MEMO_TEXT FROM MEMOS WHERE MEMO_ID = ?")
                .bind(shared_memo_id)
                .fetch_optional(&pool)
                .await
                .unwrap();
        assert_eq!(still_there.as_deref(), Some("keep_for_rule"));
    }

    /// Regression for Devin review of PR #82. Under production settings
    /// (`PRAGMA foreign_keys = ON`), clearing a detail's memo used to fail
    /// with FOREIGN KEY constraint failed because MEMO_DELETE ran before
    /// the DETAIL UPDATE released the reference. Prior tests missed this
    /// because the default test schema did not declare the MEMOS FK and
    /// FK enforcement was off.
    #[tokio::test]
    async fn test_clear_detail_memo_succeeds_under_foreign_keys_on() {
        let pool = setup_test_db_with_foreign_keys().await;
        let service = TransactionService::new(pool);

        let header_id = create_test_header(&service).await;
        let mut with_memo = basic_detail_request();
        with_memo.memo = Some("to_clear".to_string());
        let detail_id = service
            .add_transaction_detail(2, header_id, with_memo)
            .await
            .unwrap();

        let mut cleared = basic_detail_request();
        cleared.memo = Some("".to_string());
        service
            .update_transaction_detail(2, detail_id, cleared)
            .await
            .expect("clearing memo must not violate the MEMOS foreign key");

        let after = service.get_transaction_details(2, header_id).await.unwrap();
        assert!(after[0].memo_id.is_none());
        assert!(after[0].memo_text.is_none());
    }

    /// Fable-5 review #12 — before the fix, `add_transaction_detail` did
    /// not check that the parent `transaction_id` belonged to `user_id`.
    /// The FK on TRANSACTIONS_DETAIL.TRANSACTION_ID only requires the
    /// header row to exist; a direct `invoke` from user B with user A's
    /// transaction_id would attach B's detail to A's header. Now `add`
    /// mirrors the fetch+ok_or(NotFound) contract that update/delete
    /// already used.
    ///
    /// Note: this test uses `setup_test_db()`, whose schema does not
    /// enforce the TRANSACTIONS_DETAIL → TRANSACTIONS_HEADER foreign
    /// key. That's the exact condition under which the pre-fix code
    /// would have happily inserted a foreign-owned detail — production
    /// enforces the FK but only on the shape of the reference, not on
    /// USER_ID, so the same cross-owner attach happens there too.
    #[tokio::test]
    async fn test_add_detail_rejects_foreign_transaction_id() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        // create_test_header seeds user_id = 2.
        let header_id_of_user_2 = create_test_header(&service).await;

        // A different user (user_id = 3) tries to attach a detail to user
        // 2's header. Must be rejected as NotFound before any INSERT or
        // MEMO_INSERT runs.
        let attempt = service
            .add_transaction_detail(3, header_id_of_user_2, basic_detail_request())
            .await;

        assert!(
            matches!(attempt, Err(TransactionError::NotFound)),
            "cross-owner attach must return NotFound, got: {:?}",
            attempt
        );

        // User 2 (the actual owner) can still add a detail to the same
        // header, confirming the ownership check does not over-block.
        service
            .add_transaction_detail(2, header_id_of_user_2, basic_detail_request())
            .await
            .expect("owner should be able to add a detail to their own header");
    }

    /// Attempting to add a detail to a transaction_id that does not exist
    /// at all must also return NotFound rather than surface a generic
    /// sqlx error (the pre-fix code would trip the FK later, giving a
    /// less legible message; without any FK, it would insert silently).
    #[tokio::test]
    async fn test_add_detail_rejects_nonexistent_transaction_id() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool);

        let attempt = service
            .add_transaction_detail(2, 999_999, basic_detail_request())
            .await;

        assert!(
            matches!(attempt, Err(TransactionError::NotFound)),
            "nonexistent parent must return NotFound, got: {:?}",
            attempt
        );
    }

    /// Fable-5 review #7 — `add_transaction_detail` used to
    /// unconditionally `MEMO_INSERT` every time it saw a memo, so
    /// two details with the same memo text ended up with distinct
    /// MEMOS rows. The "shared memo" branch in
    /// `update_transaction_detail` then never fired for adds — a
    /// user editing one row's memo did not update the other row that
    /// looked like it shared the text. The add path now routes
    /// through `get_or_create_memo_id_in_tx` and dedups against
    /// existing memo text for the same user.
    #[tokio::test]
    async fn test_add_detail_dedupes_memo_text_across_multiple_adds() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool.clone());
        let header_id = create_test_header(&service).await;

        // Two adds with the same memo body for the same user.
        let mut req_a = basic_detail_request();
        req_a.memo = Some("shared memo".to_string());
        let mut req_b = basic_detail_request();
        req_b.memo = Some("shared memo".to_string());

        service.add_transaction_detail(2, header_id, req_a).await.unwrap();
        service.add_transaction_detail(2, header_id, req_b).await.unwrap();

        // Both detail rows should reference the same MEMO_ID.
        let memo_ids: Vec<i64> = sqlx::query_scalar(
            "SELECT MEMO_ID FROM TRANSACTIONS_DETAIL WHERE TRANSACTION_ID = ? AND MEMO_ID IS NOT NULL ORDER BY DETAIL_ID"
        )
        .bind(header_id)
        .fetch_all(&pool)
        .await
        .unwrap();
        assert_eq!(memo_ids.len(), 2, "both details must carry a MEMO_ID");
        assert_eq!(memo_ids[0], memo_ids[1], "second add must reuse the first add's MEMO_ID");

        // Only one MEMOS row exists with that text — no duplicate insert.
        let memo_row_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM MEMOS WHERE USER_ID = ? AND MEMO_TEXT = ?"
        )
        .bind(2i64)
        .bind("shared memo")
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(memo_row_count, 1, "MEMOS must not carry a duplicate row for the same text");
    }

    /// Fable-5 review #7 companion — a detail add for a memo text
    /// that the parent HEADER (or any other row) already references
    /// must reuse that row's MEMO_ID, not mint a new one. This makes
    /// the "shared memo" contract in `update_transaction_detail`
    /// (which allocates a fresh row when it detects a shared memo)
    /// actually reachable from adds too.
    #[tokio::test]
    async fn test_add_detail_reuses_memo_shared_with_header() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool.clone());

        // Header carrying its own memo (create_test_header sets no
        // memo by default, so seed one directly). Reuse the existing
        // helper to get a valid header, then attach a MEMO row and
        // point the header at it.
        let header_id = create_test_header(&service).await;
        let memo_id: i64 = sqlx::query(
            "INSERT INTO MEMOS (USER_ID, MEMO_TEXT) VALUES (?, ?) RETURNING MEMO_ID"
        )
        .bind(2i64)
        .bind("shared with header")
        .fetch_one(&pool)
        .await
        .unwrap()
        .get(0);
        sqlx::query("UPDATE TRANSACTIONS_HEADER SET MEMO_ID = ? WHERE TRANSACTION_ID = ?")
            .bind(memo_id)
            .bind(header_id)
            .execute(&pool)
            .await
            .unwrap();

        // Add a detail typing that same memo text.
        let mut req = basic_detail_request();
        req.memo = Some("shared with header".to_string());
        let detail_id = service.add_transaction_detail(2, header_id, req).await.unwrap();

        // Detail must reference the pre-existing MEMO_ID, not a fresh one.
        let detail_memo_id: Option<i64> = sqlx::query_scalar(
            "SELECT MEMO_ID FROM TRANSACTIONS_DETAIL WHERE DETAIL_ID = ?"
        )
        .bind(detail_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(detail_memo_id, Some(memo_id));

        // Still one MEMOS row for that text (no duplicate).
        let memo_row_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM MEMOS WHERE USER_ID = ? AND MEMO_TEXT = ?"
        )
        .bind(2i64)
        .bind("shared with header")
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(memo_row_count, 1);
    }

    /// Fable-5 review #7 (atomicity side) — the MEMO insert and the
    /// TRANSACTIONS_DETAIL insert now share a transaction, so a
    /// failure in the DETAIL_INSERT rolls back the MEMO insert too
    /// and leaves nothing orphaned. Reproduce by targeting a
    /// nonexistent parent id after the ownership check passes: we
    /// can't easily race a real FK failure inside the tx, but we can
    /// prove the "before-fix" leak shape doesn't reappear by asserting
    /// the MEMOS table stays empty when we ask for an add of a
    /// memo-only-fresh-body against a header we then delete out of
    /// scope.
    #[tokio::test]
    async fn test_add_detail_failure_does_not_orphan_memo() {
        let pool = setup_test_db().await;
        let service = TransactionService::new(pool.clone());

        // Try to add a detail with a fresh memo body against a
        // nonexistent parent — the ownership check fails BEFORE the
        // tx opens (guard at the top of `add_transaction_detail`),
        // so the MEMOS table stays empty. This locks the "guard runs
        // before any write" contract; a regression that moves the
        // guard after the MEMO insert would leave "leak-me" behind
        // and trip this test.
        let mut req = basic_detail_request();
        req.memo = Some("leak-me".to_string());
        let result = service.add_transaction_detail(2, 999_999, req).await;
        assert!(matches!(result, Err(TransactionError::NotFound)));

        let leaked: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM MEMOS WHERE MEMO_TEXT = ?"
        )
        .bind("leak-me")
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(leaked, 0, "rejected add must not leave a MEMOS row behind");
    }

    // ---- From<TransactionError> for ApiError ----------------------------
    // These tests pin the wire codes that the frontend classifier
    // (`res/js/transaction-management.js` / `transaction-detail-management.js`
    // — `err.code` branching) matches on. If a variant is renamed here or in
    // api_error.rs, the JS side stops classifying its errors — hence the
    // assertions on the stable `ApiError::CODE_*` constants. Mirrors the
    // RecurringError / CategoryError / UserManagementError precedent
    // (PR #100/#101/#103).

    #[test]
    fn not_found_maps_to_not_found_code_with_transaction_entity() {
        let err: ApiError = TransactionError::NotFound.into();
        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
        assert_eq!(err.entity.as_deref(), Some("transaction"));
    }

    #[test]
    fn validation_preserves_message_and_omits_entity() {
        let err: ApiError = TransactionError::ValidationError(
            "Item name must be 64 characters or less".to_string(),
        )
        .into();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains("64 characters"));
        assert!(err.entity.is_none());
    }

    #[test]
    fn database_error_maps_to_database_code() {
        let err: ApiError = TransactionError::DatabaseError("no such table".to_string()).into();
        assert_eq!(err.code, ApiError::CODE_DATABASE);
        assert!(err.entity.is_none());
    }

    /// CodeRabbit on #127 — pin the wire code for
    /// `TransferSameAccount` so a future refactor that folds it back
    /// into `ApiError::validation(...)` (or renames the code string)
    /// trips this test instead of silently degrading the frontend to
    /// the generic English fallback. The `entity` stays `None` because
    /// the failure is about the transfer relation, not a named row.
    #[test]
    fn transfer_same_account_maps_to_stable_wire_code_and_omits_entity() {
        let err: ApiError = TransactionError::TransferSameAccount.into();
        assert_eq!(err.code, ApiError::CODE_TRANSFER_SAME_ACCOUNT);
        assert_eq!(err.code, "transfer_same_account");
        assert!(err.entity.is_none());
    }

    #[test]
    fn field_needle_message_survives_conversion_for_frontend_routing() {
        // The transaction-management / transaction-detail-management screens
        // dispatch bounded-field validation errors to the right input by
        // checking `err.message.startsWith(...)` after gating on
        // `err.code === 'validation'`. That contract needs the two leading
        // needles (`"Item name must be"`, `"Memo must be"`) to reach the
        // wire verbatim — this test pins that.
        for needle in ["Item name must be", "Memo must be"] {
            let msg = format!("{} 64 characters or less", needle);
            let err: ApiError = TransactionError::ValidationError(msg.clone()).into();
            assert_eq!(err.code, ApiError::CODE_VALIDATION);
            assert!(
                err.message.starts_with(needle),
                "wire message `{}` must start with `{}` for frontend field routing",
                err.message,
                needle
            );
        }
    }
}

