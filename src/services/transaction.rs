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
    #[sqlx(rename = "MEMO_ID")]
    pub memo_id: Option<i64>,
    #[sqlx(rename = "IS_DISABLED")]
    pub is_disabled: i64,
    #[sqlx(rename = "ENTRY_DT")]
    pub entry_dt: String,
    #[sqlx(rename = "UPDATE_DT")]
    pub update_dt: Option<String>,
}

/// Request structure for saving transaction header
#[derive(Debug, Deserialize, Clone)]
pub struct SaveTransactionRequest {
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub transaction_date: String,
    pub total_amount: i64,
    pub tax_rounding_type: i64,
    pub memo: Option<String>,
}

/// Transaction detail data structure
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct TransactionDetail {
    #[sqlx(rename = "DETAIL_ID")]
    pub detail_id: i64,
    #[sqlx(rename = "TRANSACTION_ID")]
    pub transaction_id: i64,
    #[sqlx(rename = "CATEGORY2_CODE")]
    pub category2_code: String,
    #[sqlx(rename = "CATEGORY3_CODE")]
    pub category3_code: String,
    #[sqlx(rename = "ITEM_NAME")]
    pub item_name: String,
    #[sqlx(rename = "AMOUNT")]
    pub amount: i64,
    #[sqlx(rename = "TAX_AMOUNT")]
    pub tax_amount: i64,
    #[sqlx(rename = "TAX_RATE")]
    pub tax_rate: i32,
    #[sqlx(rename = "MEMO_ID")]
    pub memo_id: Option<i64>,
    #[sqlx(rename = "ENTRY_DT")]
    pub entry_dt: String,
    #[sqlx(rename = "UPDATE_DT")]
    pub update_dt: Option<String>,
}

/// Memo data structure
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Memo {
    #[sqlx(rename = "MEMO_ID")]
    pub memo_id: i64,
    #[sqlx(rename = "MEMO_TEXT")]
    pub memo_text: String,
    #[sqlx(rename = "ENTRY_DT")]
    pub entry_dt: String,
    #[sqlx(rename = "UPDATE_DT")]
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
                if text.len() > 1000 {
                    return Err(TransactionError::ValidationError(
                        "Memo must be 1000 characters or less".to_string(),
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
            .bind(&request.transaction_date)
            .bind(&request.category1_code)
            .bind(&request.from_account_code)
            .bind(&request.to_account_code)
            .bind(request.total_amount)
            .bind(request.tax_rounding_type)
            .bind(memo_id)
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
                if text.len() > 1000 {
                    return Err(TransactionError::ValidationError(
                        "Memo must be 1000 characters or less".to_string(),
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
                transaction_date: row.get(2),
                category1_code: row.get(3),
                from_account_code: row.get(4),
                to_account_code: row.get(5),
                total_amount: row.get(6),
                tax_rounding_type: row.get(7),
                memo_id: row.get(8),
                is_disabled: row.get(9),
                entry_dt: row.get(10),
                update_dt: row.get(11),
            };
            let memo_text: Option<String> = row.get(12);
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
            if desc.len() > 500 {
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
            if m.len() > 1000 {
                return Err(TransactionError::ValidationError(
                    "Memo must be 1000 characters or less".to_string(),
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
        page: i64,
        per_page: i64,
    ) -> Result<TransactionListResponse, TransactionError> {
        // Build WHERE clauses (with table alias 't.')
        let mut where_clauses = vec!["t.USER_ID = ?".to_string()];
        let mut params: Vec<String> = vec![user_id.to_string()];

        if let Some(start) = start_date {
            where_clauses.push("t.TRANSACTION_DATE >= ?".to_string());
            params.push(start.to_string());
        }

        if let Some(end) = end_date {
            where_clauses.push("t.TRANSACTION_DATE <= ?".to_string());
            params.push(end.to_string());
        }

        if let Some(cat1) = category1_code {
            where_clauses.push("t.CATEGORY1_CODE = ?".to_string());
            params.push(cat1.to_string());
        }

        // Note: CATEGORY2_CODE and CATEGORY3_CODE are in TRANSACTIONS_DETAIL, not in HEADER
        // These filters are not implemented for header-only queries
        let _ = category2_code; // Suppress unused warning
        let _ = category3_code; // Suppress unused warning

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
        if memo_text.len() > 1000 {
            return Err(TransactionError::ValidationError(
                "Memo must be 1000 characters or less".to_string(),
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
        if memo_text.len() > 1000 {
            return Err(TransactionError::ValidationError(
                "Memo must be 1000 characters or less".to_string(),
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
            .bind(&request.transaction_date)
            .bind(&request.category1_code)
            .bind(&request.from_account_code)
            .bind(&request.to_account_code)
            .bind(request.total_amount)
            .bind(request.tax_rounding_type)
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
}
