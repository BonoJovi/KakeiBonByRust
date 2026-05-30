use sqlx::{SqlitePool, Row};
use serde::{Serialize, Deserialize};
use crate::{sql_queries, consts};

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
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            TransactionError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            TransactionError::NotFound => write!(f, "Transaction not found"),
        }
    }
}

impl From<sqlx::Error> for TransactionError {
    fn from(err: sqlx::Error) -> Self {
        TransactionError::DatabaseError(err.to_string())
    }
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
/// Patterns are tried in priority order:
///
///   1. tax-excluded + floor       (TAX_ROUND_DOWN)
///   2. tax-excluded + half-up     (TAX_ROUND_HALF_UP)
///   3. tax-excluded + ceil        (TAX_ROUND_UP)
///   4. tax-included               (the rounding column is irrelevant in
///      this mode because no rounding ever happens; we report it back as
///      `TAX_ROUND_DOWN` so the caller has a stable value to write)
///
/// The order matches what shopkeepers actually do in the wild — most use
/// floor or half-up, ceil is rare — so the "first match wins" rule lands
/// on the most plausible setting when several match (which happens often
/// for receipts whose total has no fractional part to round).
fn find_matching_pattern(details: &[DetailForRecalc], target_total: i64) -> Option<(i64, i64)> {
    const PATTERNS: [(i64, i64); 4] = [
        (consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED),
        (consts::TAX_ROUND_HALF_UP, consts::TAX_EXCLUDED),
        (consts::TAX_ROUND_UP, consts::TAX_EXCLUDED),
        (consts::TAX_ROUND_DOWN, consts::TAX_INCLUDED),
    ];
    for (rounding, included) in PATTERNS {
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

        // Save memo if provided
        let memo_id = if let Some(text) = &request.memo {
            if !text.trim().is_empty() {
                if text.chars().count() > consts::MAX_MEMO_LEN {
                    return Err(TransactionError::ValidationError(
                        format!("Memo must be {} characters or less", consts::MAX_MEMO_LEN),
                    ));
                }
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

    /// Add a new transaction header
    pub async fn add_transaction_header(
        &self,
        user_id: i64,
        transaction_date: &str,
        category1_code: &str,
        from_account_code: &str,
        to_account_code: &str,
        total_amount: i64,
        tax_rate: i32,
        tax_rounding: &str,
        memo_text: Option<&str>,
    ) -> Result<i64, TransactionError> {
        // Validate datetime format (YYYY-MM-DD HH:MM:SS)
        if transaction_date.len() != 19 {
            return Err(TransactionError::ValidationError(
                "Invalid datetime format. Use YYYY-MM-DD HH:MM:SS".to_string(),
            ));
        }

        // Validate amount (0 is allowed)
        if total_amount < 0 || total_amount > 999_999_999 {
            return Err(TransactionError::ValidationError(
                "Amount must be between 0 and 999,999,999".to_string(),
            ));
        }

        // Validate tax rate (typically 8% or 10% in Japan)
        if tax_rate < 0 || tax_rate > 100 {
            return Err(TransactionError::ValidationError(
                "Tax rate must be between 0 and 100".to_string(),
            ));
        }

        // Validate tax rounding method
        if !["ROUND_UP", "ROUND_DOWN", "ROUND_HALF"].contains(&tax_rounding) {
            return Err(TransactionError::ValidationError(
                "Invalid tax rounding method".to_string(),
            ));
        }

        // Save memo if provided
        let memo_id = if let Some(text) = memo_text {
            if !text.trim().is_empty() {
                if text.chars().count() > consts::MAX_MEMO_LEN {
                    return Err(TransactionError::ValidationError(
                        format!("Memo must be {} characters or less", consts::MAX_MEMO_LEN),
                    ));
                }
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
            .bind(transaction_date)
            .bind(category1_code)
            .bind(from_account_code)
            .bind(to_account_code)
            .bind(total_amount)
            .bind(tax_rate)
            .bind(tax_rounding)
            .bind(memo_id)
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

    /// Add a new transaction (legacy method for backward compatibility)
    pub async fn add_transaction(
        &self,
        user_id: i64,
        transaction_date: &str,
        category1_code: &str,
        category2_code: &str,
        category3_code: &str,
        amount: i64,
        description: Option<&str>,
        memo: Option<&str>,
    ) -> Result<i64, TransactionError> {
        // Validate amount
        if amount < 1 || amount > 999_999_999 {
            return Err(TransactionError::ValidationError(
                "Amount must be between 1 and 999,999,999".to_string(),
            ));
        }

        // Validate datetime format (YYYY-MM-DD HH:MM:SS)
        if transaction_date.len() != 19 {
            return Err(TransactionError::ValidationError(
                "Invalid datetime format. Use YYYY-MM-DD HH:MM:SS".to_string(),
            ));
        }

        // Validate description length
        if let Some(desc) = description {
            if desc.trim().is_empty() {
                return Err(TransactionError::ValidationError(
                    "Description cannot be empty or whitespace only".to_string(),
                ));
            }
            if desc.chars().count() > 500 {
                return Err(TransactionError::ValidationError(
                    "Description must be 500 characters or less".to_string(),
                ));
            }
        }

        // Validate memo length
        if let Some(m) = memo {
            if m.trim().is_empty() {
                return Err(TransactionError::ValidationError(
                    "Memo cannot be empty or whitespace only".to_string(),
                ));
            }
            if m.chars().count() > consts::MAX_MEMO_LEN {
                return Err(TransactionError::ValidationError(
                    format!("Memo must be {} characters or less", consts::MAX_MEMO_LEN),
                ));
            }
        }

        let result = sqlx::query(sql_queries::TRANSACTION_INSERT)
            .bind(user_id)
            .bind(transaction_date)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .bind(amount)
            .bind(description)
            .bind(memo)
            .execute(&self.pool)
            .await?;

        Ok(result.last_insert_rowid())
    }

    /// Get a single transaction by ID
    pub async fn get_transaction(
        &self,
        user_id: i64,
        transaction_id: i64,
    ) -> Result<Transaction, TransactionError> {
        let transaction = sqlx::query_as::<_, Transaction>(sql_queries::TRANSACTION_SELECT_BY_ID)
            .bind(user_id)
            .bind(transaction_id)
            .fetch_optional(&self.pool)
            .await?;

        transaction.ok_or(TransactionError::NotFound)
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
            params.push(end.to_string());
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

        // Note: DESCRIPTION and MEMO are not in HEADER
        // Keyword search is not implemented for header-only queries
        let _ = keyword; // Suppress unused warning

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

    /// Update a transaction
    pub async fn update_transaction(
        &self,
        user_id: i64,
        transaction_id: i64,
        transaction_date: &str,
        category1_code: &str,
        category2_code: &str,
        category3_code: &str,
        amount: i64,
        description: Option<&str>,
        memo: Option<&str>,
    ) -> Result<(), TransactionError> {
        // Validate amount
        if amount < 1 || amount > 999_999_999 {
            return Err(TransactionError::ValidationError(
                "Amount must be between 1 and 999,999,999".to_string(),
            ));
        }

        // Validate datetime format (YYYY-MM-DD HH:MM:SS)
        if transaction_date.len() != 19 {
            return Err(TransactionError::ValidationError(
                "Invalid datetime format. Use YYYY-MM-DD HH:MM:SS".to_string(),
            ));
        }

        let result = sqlx::query(sql_queries::TRANSACTION_UPDATE)
            .bind(transaction_date)
            .bind(category1_code)
            .bind(category2_code)
            .bind(category3_code)
            .bind(amount)
            .bind(description)
            .bind(memo)
            .bind(user_id)
            .bind(transaction_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(TransactionError::NotFound);
        }

        Ok(())
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

    /// Helper function to get or create memo_id for memo text
    /// Returns memo_id if memo text is provided, None if empty
    async fn get_or_create_memo_id(
        &self,
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

        // Validate memo length
        if memo_text.chars().count() > consts::MAX_MEMO_LEN {
            return Err(TransactionError::ValidationError(
                format!("Memo must be {} characters or less", consts::MAX_MEMO_LEN),
            ));
        }

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

        // Validate memo length
        if memo_text.chars().count() > consts::MAX_MEMO_LEN {
            return Err(TransactionError::ValidationError(
                format!("Memo must be {} characters or less", consts::MAX_MEMO_LEN),
            ));
        }

        // Check if current memo_id is shared with other transactions
        let is_shared = if let Some(memo_id) = current_memo_id {
            let count: i64 = sqlx::query_scalar(sql_queries::MEMO_COUNT_USAGE)
                .bind(memo_id)
                .fetch_one(&self.pool)
                .await?;
            count > 1
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

        if request.item_name.chars().count() > consts::MAX_ITEM_NAME_LEN {
            return Err(TransactionError::ValidationError(
                format!("Item name must be {} characters or less", consts::MAX_ITEM_NAME_LEN),
            ));
        }

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

        // Save memo if provided
        let memo_id = if let Some(text) = &request.memo {
            if !text.trim().is_empty() {
                if text.chars().count() > consts::MAX_MEMO_LEN {
                    return Err(TransactionError::ValidationError(
                        format!("Memo must be {} characters or less", consts::MAX_MEMO_LEN),
                    ));
                }
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

        // Insert detail
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
            .execute(&self.pool)
            .await?;

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

        if request.item_name.chars().count() > consts::MAX_ITEM_NAME_LEN {
            return Err(TransactionError::ValidationError(
                format!("Item name must be {} characters or less", consts::MAX_ITEM_NAME_LEN),
            ));
        }

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

        // Handle memo update
        let memo_id = if let Some(text) = &request.memo {
            if !text.trim().is_empty() {
                if text.chars().count() > consts::MAX_MEMO_LEN {
                    return Err(TransactionError::ValidationError(
                        format!("Memo must be {} characters or less", consts::MAX_MEMO_LEN),
                    ));
                }
                
                if let Some(old_memo_id) = existing_detail.memo_id {
                    // Update existing memo
                    sqlx::query(sql_queries::MEMO_UPDATE)
                        .bind(text)
                        .bind(old_memo_id)
                        .execute(&self.pool)
                        .await?;
                    Some(old_memo_id)
                } else {
                    // Create new memo
                    let result = sqlx::query(sql_queries::MEMO_INSERT)
                        .bind(user_id)
                        .bind(text)
                        .execute(&self.pool)
                        .await?;
                    Some(result.last_insert_rowid())
                }
            } else {
                // Delete old memo if exists
                if let Some(old_memo_id) = existing_detail.memo_id {
                    sqlx::query(sql_queries::MEMO_DELETE)
                        .bind(old_memo_id)
                        .execute(&self.pool)
                        .await?;
                }
                None
            }
        } else {
            existing_detail.memo_id
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

        // Delete memo if exists (after detail is deleted)
        if let Some(memo_id) = memo_id {
            sqlx::query(sql_queries::MEMO_DELETE)
                .bind(memo_id)
                .execute(&self.pool)
                .await?;
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

            let detail_rows = sqlx::query(sql_queries::TRANSACTION_DETAIL_GET_FOR_RECALC)
                .bind(txn_id)
                .bind(user_id)
                .fetch_all(&mut *tx)
                .await?;

            let details: Vec<DetailForRecalc> = detail_rows
                .iter()
                .map(|r| DetailForRecalc {
                    amount: r.get("AMOUNT"),
                    amount_including_tax: r.get("AMOUNT_INCLUDING_TAX"),
                    tax_rate: r.get("TAX_RATE"),
                })
                .collect();
            let detail_amounts: Vec<i64> = details.iter().map(|d| d.amount).collect();

            // First, prefer to keep the user-entered TOTAL_AMOUNT verbatim by
            // searching for a (rounding, included) pattern that reproduces it.
            // If we find one, the only correction we need to make is to the
            // header's tax setting columns. If we do not, we fall back to
            // overwriting TOTAL_AMOUNT with what the *existing* settings
            // produce, since the user's entry is then internally inconsistent
            // with the details and we have no signal to prefer it.
            match find_matching_pattern(&details, total_before) {
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
        let backup_dir = backup.parent().map(|p| p.to_path_buf()).ok_or_else(|| {
            TransactionError::ValidationError("Backup path has no parent directory".to_string())
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
}

