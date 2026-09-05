use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use crate::api_error::ApiError;
use crate::services::master_data;
use crate::sql_queries;
use crate::validation;

const NAME_LABEL: &str = "Account name";
const ENTITY_LABEL: &str = "Account";

/// Normalize account code to uppercase
fn normalize_account_code(code: &str) -> String {
    code.trim().to_uppercase()
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AccountTemplate {
    pub template_id: i64,
    pub template_code: String,
    pub template_name_ja: String,
    pub template_name_en: String,
    pub display_order: i64,
    pub entry_dt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Account {
    pub account_id: i64,
    pub user_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

/// Result row for `get_account_balances_as_of`. One row per active account,
/// representing the running balance after every actualised transaction up to
/// and including the given as-of date. Used by the dashboard so the user can
/// reconcile the chart totals against per-account ledgers.
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct AccountBalance {
    pub account_code: String,
    pub account_name: String,
    pub balance: i64,
    pub display_order: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddAccountRequest {
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAccountRequest {
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
    pub display_order: i64,
}

/// Get all account templates
pub async fn get_account_templates(pool: &SqlitePool) -> Result<Vec<AccountTemplate>, ApiError> {
    let templates = sqlx::query_as::<_, AccountTemplate>(sql_queries::ACCOUNT_TEMPLATE_LIST)
        .fetch_all(pool)
        .await?;

    Ok(templates)
}

/// Get all accounts for a user
pub async fn get_accounts(pool: &SqlitePool, user_id: i64) -> Result<Vec<Account>, ApiError> {
    let accounts = sqlx::query_as::<_, Account>(sql_queries::ACCOUNT_LIST_BY_USER)
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(accounts)
}

/// Compute the running balance of every active account for `user_id`,
/// counted up to and including `as_of_date` (`YYYY-MM-DD`). Each balance
/// starts from the account's `INITIAL_BALANCE` and applies actualised
/// transactions only (`IS_SCHEDULED = 0`):
///
///   + INCOME  with TO_ACCOUNT  matching the account
///   - EXPENSE with FROM_ACCOUNT matching the account
///   + TRANSFER with TO_ACCOUNT  matching the account
///   - TRANSFER with FROM_ACCOUNT matching the account
///
/// The single-pass CASE keeps each transaction visible to both the source
/// and destination accounts of a TRANSFER, with the sign flipped per side.
/// Disabled accounts are excluded.
pub async fn get_account_balances_as_of(
    pool: &SqlitePool,
    user_id: i64,
    as_of_date: &str,
) -> Result<Vec<AccountBalance>, ApiError> {
    let balances = sqlx::query_as::<_, AccountBalance>(sql_queries::ACCOUNT_BALANCES_AS_OF)
        .bind(as_of_date)
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(balances)
}

/// Get all accounts (for admin users)
pub async fn get_all_accounts(pool: &SqlitePool) -> Result<Vec<Account>, ApiError> {
    let accounts = sqlx::query_as::<_, Account>(sql_queries::ACCOUNT_LIST_ALL)
        .fetch_all(pool)
        .await?;

    Ok(accounts)
}

/// Get a single account by code. Kept for module tests only after the
/// PR3/PR4 Fable-5 #26 refactor removed the delete/update pre-checks;
/// still useful as a post-write "did the row actually land" probe.
#[allow(dead_code)]
pub async fn get_account_by_code(
    pool: &SqlitePool,
    user_id: i64,
    account_code: &str,
) -> Result<Option<Account>, ApiError> {
    let account = sqlx::query_as::<_, Account>(sql_queries::ACCOUNT_GET_BY_CODE)
        .bind(user_id)
        .bind(account_code)
        .fetch_optional(pool)
        .await?;

    Ok(account)
}

/// Check if account code already exists
async fn check_duplicate_code(
    pool: &SqlitePool,
    user_id: i64,
    account_code: &str,
) -> Result<bool, ApiError> {
    master_data::value_exists(
        pool,
        sql_queries::ACCOUNT_CHECK_DUPLICATE_CODE,
        user_id,
        account_code,
    )
    .await
}

/// Add a new account (or reactivate if deleted)
pub async fn add_account(
    pool: &SqlitePool,
    user_id: i64,
    mut request: AddAccountRequest,
) -> Result<String, ApiError> {
    // Normalize account code to uppercase
    request.account_code = normalize_account_code(&request.account_code);

    // Validate account code
    if request.account_code.is_empty() {
        return Err(ApiError::validation("Account code cannot be empty"));
    }

    // `validate_master_name` bundles the non-empty and max-length checks that
    // shop/manufacturer/product go through. Using the same helper (rather than
    // the bare `validate_max_chars`) closes a gap where a request coming in
    // through a direct `invoke` — bypassing the frontend guard at
    // account-management.js:286-288 — could persist an all-whitespace name.
    validation::validate_master_name(NAME_LABEL, &request.account_name)
        .map_err(ApiError::validation)?;

    // Check for duplicate code (only active accounts)
    if check_duplicate_code(pool, user_id, &request.account_code).await? {
        return Err(ApiError::duplicate_code(ENTITY_LABEL));
    }

    // Get next display order
    let display_order = master_data::fetch_next_display_order(
        pool,
        sql_queries::ACCOUNT_GET_NEXT_DISPLAY_ORDER,
        user_id,
    )
    .await?;

    // Upsert account (insert or reactivate if deleted)
    sqlx::query(sql_queries::ACCOUNT_UPSERT)
        .bind(user_id)
        .bind(&request.account_code)
        .bind(&request.account_name)
        .bind(&request.template_code)
        .bind(request.initial_balance)
        .bind(display_order)
        .execute(pool)
        .await?;

    Ok("Account added successfully".to_string())
}

/// Update an account
pub async fn update_account(
    pool: &SqlitePool,
    user_id: i64,
    mut request: UpdateAccountRequest,
) -> Result<String, ApiError> {
    // Normalize account code to uppercase
    request.account_code = normalize_account_code(&request.account_code);

    // `validate_master_name` bundles the non-empty and max-length checks that
    // shop/manufacturer/product go through. Using the same helper (rather than
    // the bare `validate_max_chars`) closes a gap where a request coming in
    // through a direct `invoke` — bypassing the frontend guard at
    // account-management.js:286-288 — could persist an all-whitespace name.
    validation::validate_master_name(NAME_LABEL, &request.account_name)
        .map_err(ApiError::validation)?;

    // Pre-check `get_account_by_code().ok_or(NotFound)?` was removed here
    // (Fable-5 review #26): rows_affected from the UPDATE tells us the
    // same thing in one round-trip instead of two, and it closes the
    // TOCTOU window where the row could vanish between the pre-check
    // and the update. shop/manufacturer/product got the same treatment
    // in PR3 via master_data::ensure_update_affected_one.
    let affected = sqlx::query(sql_queries::ACCOUNT_UPDATE)
        .bind(&request.account_name)
        .bind(&request.template_code)
        .bind(request.initial_balance)
        .bind(request.display_order)
        .bind(user_id)
        .bind(&request.account_code)
        .execute(pool)
        .await?
        .rows_affected();
    if affected == 0 {
        return Err(ApiError::not_found(ENTITY_LABEL));
    }

    Ok("Account updated successfully".to_string())
}

/// Delete an account (logical deletion). Rejected with
/// `ApiError::in_use("Account")` when any transaction or recurring rule
/// still names this account on either side (FROM/TO). See
/// `sql_queries::ACCOUNT_CHECK_IN_USE`. Kept off the shared
/// `MasterCrudSpec` path because the account handle is a String CODE,
/// not an i64 id (Fable-5 #26 note in `account.rs`).
pub async fn delete_account(
    pool: &SqlitePool,
    user_id: i64,
    account_code: &str,
) -> Result<String, ApiError> {
    // Normalize account code to uppercase
    let account_code = normalize_account_code(account_code);

    let (in_use,): (i64,) = sqlx::query_as(sql_queries::ACCOUNT_CHECK_IN_USE)
        .bind(user_id)
        .bind(&account_code)
        .bind(&account_code)
        .bind(user_id)
        .bind(&account_code)
        .bind(&account_code)
        .fetch_one(pool)
        .await?;
    master_data::reject_if_in_use(ENTITY_LABEL, in_use)?;

    // Same rows_affected treatment as delete_shop / delete_manufacturer /
    // delete_product (PR3, Fable-5 #26): a single logical-delete UPDATE
    // that maps 0-rows → NotFound eliminates the earlier pre-check +
    // execute pair and its TOCTOU window.
    let affected = sqlx::query(sql_queries::ACCOUNT_DELETE_LOGICAL)
        .bind(user_id)
        .bind(&account_code)
        .execute(pool)
        .await?
        .rows_affected();
    if affected == 0 {
        return Err(ApiError::not_found(ENTITY_LABEL));
    }

    Ok("Account deleted successfully".to_string())
}

/// Initialize NONE account for a new user
/// This is required for irregular transactions that don't specify an account.
/// Kept on the old `Result<_, String>` return type because this is called
/// from `lib.rs` setup (not a Tauri command) and its callers already handle
/// String errors — the ApiError from `add_account` is unwrapped to its
/// message here.
pub async fn initialize_none_account(pool: &SqlitePool, user_id: i64) -> Result<(), String> {
    // Get NONE template
    let none_template = sqlx::query_as::<_, AccountTemplate>(sql_queries::ACCOUNT_TEMPLATE_GET_NONE)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to get NONE template: {}", e))?;

    // Create NONE account
    let request = AddAccountRequest {
        account_code: "NONE".to_string(),
        account_name: none_template.template_name_ja.clone(),
        template_code: "NONE".to_string(),
        initial_balance: 0,
    };

    // The NONE account may already exist (re-initialization); anything else is
    // a real failure and must be propagated. Match on the ApiError code, not
    // the message text.
    match add_account(pool, user_id, request).await {
        Ok(_) => Ok(()),
        Err(e) if e.code == ApiError::CODE_DUPLICATE_CODE => Ok(()),
        Err(e) => Err(format!("Failed to initialize NONE account: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts;
    use crate::test_helpers::database::{init_db, TEST_DB_URL};

    async fn setup_test_db() -> SqlitePool {
        let pool = init_db(TEST_DB_URL).await.unwrap();

        // Create USERS table
        sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Create ACCOUNT_TEMPLATES table
        sqlx::query(sql_queries::TEST_ACCOUNT_CREATE_TEMPLATES_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Create ACCOUNTS table
        sqlx::query(sql_queries::TEST_ACCOUNT_CREATE_ACCOUNTS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // TRANSACTIONS_HEADER / RECURRING_RULES so ACCOUNT_CHECK_IN_USE
        // has tables to read against. `delete_account` runs the guard on
        // every call — including the "not in use" happy paths — so these
        // must exist in every account-service test.
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(sql_queries::TEST_CREATE_RECURRING_RULES_MINIMAL)
            .execute(&pool)
            .await
            .unwrap();

        // Insert test users
        sqlx::query(sql_queries::TEST_INSERT_USER_ADMIN)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query(sql_queries::TEST_INSERT_USER_GENERAL)
            .execute(&pool)
            .await
            .unwrap();

        // Insert account templates
        sqlx::query(sql_queries::TEST_ACCOUNT_INSERT_TEMPLATES)
            .execute(&pool)
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_add_account() {
        let pool = setup_test_db().await;

        let request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "Test Account".to_string(),
            template_code: "BANK".to_string(),
            initial_balance: 10000,
        };

        let result = add_account(&pool, 2, request).await;
        assert!(result.is_ok());

        let accounts = get_accounts(&pool, 2).await.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].account_code, "TEST");
    }

    #[tokio::test]
    async fn test_update_account() {
        let pool = setup_test_db().await;

        // Add account first
        let add_request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "Test Account".to_string(),
            template_code: "BANK".to_string(),
            initial_balance: 10000,
        };
        add_account(&pool, 2, add_request).await.unwrap();

        // Update account
        let update_request = UpdateAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "Updated Account".to_string(),
            template_code: "CASH".to_string(),
            initial_balance: 20000,
            display_order: 1,
        };

        let result = update_account(&pool, 2, update_request).await;
        assert!(result.is_ok());

        let account = get_account_by_code(&pool, 2, "TEST").await.unwrap().unwrap();
        assert_eq!(account.account_name, "Updated Account");
        assert_eq!(account.initial_balance, 20000);
    }

    #[tokio::test]
    async fn test_delete_account() {
        let pool = setup_test_db().await;

        // Add account first
        let request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "Test Account".to_string(),
            template_code: "CASH".to_string(),
            initial_balance: 0,
        };
        add_account(&pool, 2, request).await.unwrap();

        // Delete account
        let result = delete_account(&pool, 2, "TEST").await;
        assert!(result.is_ok());

        // Verify account is disabled
        let accounts = get_accounts(&pool, 2).await.unwrap();
        assert_eq!(accounts.len(), 0);
    }

    #[tokio::test]
    async fn test_duplicate_code() {
        let pool = setup_test_db().await;

        let request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "Test Account".to_string(),
            template_code: "CASH".to_string(),
            initial_balance: 0,
        };

        // Add first time - should succeed
        add_account(&pool, 2, request.clone()).await.unwrap();

        // Add second time - should fail with duplicate_code (not duplicate_name)
        let err = add_account(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_DUPLICATE_CODE);
        assert_eq!(err.entity.as_deref(), Some("account"));
    }

    // Issue #37 Phase 2-3 — bounded-field length checks must count
    // characters (not bytes). Japanese is 3 bytes per char in UTF-8.

    #[tokio::test]
    async fn test_add_account_accepts_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "あ".repeat(consts::MAX_NAME_LEN),
            template_code: "BANK".to_string(),
            initial_balance: 0,
        };
        let result = add_account(&pool, 2, request).await;
        assert!(result.is_ok(), "expected MAX_NAME_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_add_account_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "あ".repeat(consts::MAX_NAME_LEN + 1),
            template_code: "BANK".to_string(),
            initial_balance: 0,
        };
        let err = add_account(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_update_account_accepts_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let add_request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "Test".to_string(),
            template_code: "BANK".to_string(),
            initial_balance: 0,
        };
        add_account(&pool, 2, add_request).await.unwrap();

        let update_request = UpdateAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "あ".repeat(consts::MAX_NAME_LEN),
            template_code: "BANK".to_string(),
            initial_balance: 0,
            display_order: 1,
        };
        let result = update_account(&pool, 2, update_request).await;
        assert!(result.is_ok(), "expected MAX_NAME_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_update_account_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let add_request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "Test".to_string(),
            template_code: "BANK".to_string(),
            initial_balance: 0,
        };
        add_account(&pool, 2, add_request).await.unwrap();

        let update_request = UpdateAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "あ".repeat(consts::MAX_NAME_LEN + 1),
            template_code: "BANK".to_string(),
            initial_balance: 0,
            display_order: 1,
        };
        let err = update_account(&pool, 2, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    /// Fable-5 review #16 — `add_account` used to call
    /// `validate_max_chars` only, letting an empty (or all-whitespace)
    /// account name through when the request skipped the frontend guard
    /// at `account-management.js:286-288` (e.g. a direct `invoke` call).
    /// Shop / Manufacturer / Product already use `validate_master_name`,
    /// which bundles the non-empty check; this test locks the same
    /// behaviour in for accounts.
    #[tokio::test]
    async fn test_add_account_rejects_empty_name() {
        let pool = setup_test_db().await;

        let request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: String::new(),
            template_code: "BANK".to_string(),
            initial_balance: 0,
        };
        let err = add_account(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains("cannot be empty"), "unexpected error: {}", err.message);
    }

    #[tokio::test]
    async fn test_add_account_rejects_whitespace_only_name() {
        let pool = setup_test_db().await;

        let request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "   \t\n".to_string(),
            template_code: "BANK".to_string(),
            initial_balance: 0,
        };
        let err = add_account(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains("cannot be empty"), "unexpected error: {}", err.message);
    }

    #[tokio::test]
    async fn test_update_account_rejects_empty_name() {
        let pool = setup_test_db().await;

        let add_request = AddAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "Test".to_string(),
            template_code: "BANK".to_string(),
            initial_balance: 0,
        };
        add_account(&pool, 2, add_request).await.unwrap();

        let update_request = UpdateAccountRequest {
            account_code: "TEST".to_string(),
            account_name: "   ".to_string(),
            template_code: "BANK".to_string(),
            initial_balance: 0,
            display_order: 1,
        };
        let err = update_account(&pool, 2, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains("cannot be empty"), "unexpected error: {}", err.message);
    }

    async fn add_test_account(pool: &SqlitePool, code: &str, initial_balance: i64) {
        let request = AddAccountRequest {
            account_code: code.to_string(),
            account_name: format!("{} account", code),
            template_code: "CASH".to_string(),
            initial_balance,
        };
        add_account(pool, 2, request).await.unwrap();
    }

    #[allow(clippy::too_many_arguments)]
    async fn insert_header(
        pool: &SqlitePool,
        category1: &str,
        from_account: &str,
        to_account: &str,
        date: &str,
        amount: i64,
        is_scheduled: i64,
    ) {
        sqlx::query(sql_queries::TEST_ACCOUNT_INSERT_HEADER)
            .bind(2_i64)
            .bind(category1)
            .bind(from_account)
            .bind(to_account)
            .bind(date)
            .bind(amount)
            .bind(is_scheduled)
            .execute(pool)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_account_templates_ordered_by_display_order() {
        let pool = setup_test_db().await;

        let templates = get_account_templates(&pool).await.unwrap();

        let codes: Vec<&str> = templates.iter().map(|t| t.template_code.as_str()).collect();
        assert_eq!(codes, vec!["CASH", "BANK"]);
    }

    #[tokio::test]
    async fn test_get_account_by_code_returns_none_when_missing() {
        let pool = setup_test_db().await;

        let account = get_account_by_code(&pool, 2, "MISSING").await.unwrap();

        assert!(account.is_none());
    }

    #[tokio::test]
    async fn test_get_all_accounts_spans_users() {
        let pool = setup_test_db().await;
        add_test_account(&pool, "CASH", 0).await;
        add_account(
            &pool,
            1,
            AddAccountRequest {
                account_code: "ADMIN".to_string(),
                account_name: "Admin account".to_string(),
                template_code: "BANK".to_string(),
                initial_balance: 0,
            },
        )
        .await
        .unwrap();

        let accounts = get_all_accounts(&pool).await.unwrap();

        assert_eq!(accounts.len(), 2);
        assert_eq!(accounts[0].user_id, 1, "should be ordered by USER_ID");
        assert_eq!(accounts[1].user_id, 2);
    }

    #[tokio::test]
    async fn test_get_all_accounts_includes_disabled() {
        let pool = setup_test_db().await;
        add_test_account(&pool, "CASH", 0).await;
        delete_account(&pool, 2, "CASH").await.unwrap();

        assert!(get_accounts(&pool, 2).await.unwrap().is_empty());
        assert_eq!(get_all_accounts(&pool).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_add_account_normalizes_code_to_uppercase() {
        let pool = setup_test_db().await;

        add_account(
            &pool,
            2,
            AddAccountRequest {
                account_code: "  cash  ".to_string(),
                account_name: "Cash".to_string(),
                template_code: "CASH".to_string(),
                initial_balance: 0,
            },
        )
        .await
        .unwrap();

        assert!(get_account_by_code(&pool, 2, "CASH").await.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_add_account_rejects_blank_code() {
        let pool = setup_test_db().await;

        let err = add_account(
            &pool,
            2,
            AddAccountRequest {
                account_code: "   ".to_string(),
                account_name: "Cash".to_string(),
                template_code: "CASH".to_string(),
                initial_balance: 0,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains("empty"), "unexpected error: {}", err.message);
    }

    #[tokio::test]
    async fn test_add_account_reactivates_deleted_account() {
        let pool = setup_test_db().await;
        add_test_account(&pool, "CASH", 1000).await;
        delete_account(&pool, 2, "CASH").await.unwrap();

        add_test_account(&pool, "CASH", 2000).await;

        let is_disabled: i64 = sqlx::query_scalar(sql_queries::TEST_ACCOUNT_GET_IS_DISABLED)
            .bind(2_i64)
            .bind("CASH")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(is_disabled, 0, "re-adding should reactivate the account");
        assert_eq!(get_accounts(&pool, 2).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_update_account_rejects_unknown_code() {
        let pool = setup_test_db().await;

        let err = update_account(
            &pool,
            2,
            UpdateAccountRequest {
                account_code: "MISSING".to_string(),
                account_name: "Missing".to_string(),
                template_code: "CASH".to_string(),
                initial_balance: 0,
                display_order: 1,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
        assert_eq!(err.entity.as_deref(), Some("account"));
    }

    #[tokio::test]
    async fn test_delete_account_rejects_unknown_code() {
        let pool = setup_test_db().await;

        let err = delete_account(&pool, 2, "MISSING").await.unwrap_err();

        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
        assert_eq!(err.entity.as_deref(), Some("account"));
    }

    // Post-migration ApiError contract locks: the two "target vanished"
    // paths must return `code=not_found` with `entity="account"` so the
    // JS `mapMasterErrorCode` classifier routes them to
    // `account_mgmt.not_found` instead of the generic failure branch.
    #[tokio::test]
    async fn test_update_account_not_found_has_stable_code_and_entity() {
        let pool = setup_test_db().await;

        let err = update_account(
            &pool,
            2,
            UpdateAccountRequest {
                account_code: "MISSING".to_string(),
                account_name: "Missing".to_string(),
                template_code: "CASH".to_string(),
                initial_balance: 0,
                display_order: 1,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
        assert_eq!(err.entity.as_deref(), Some("account"));
    }

    #[tokio::test]
    async fn test_delete_account_not_found_has_stable_code_and_entity() {
        let pool = setup_test_db().await;
        let err = delete_account(&pool, 2, "MISSING").await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
        assert_eq!(err.entity.as_deref(), Some("account"));
    }

    #[tokio::test]
    async fn test_delete_account_rejected_when_referenced_as_from_account() {
        let pool = setup_test_db().await;
        add_test_account(&pool, "CASH", 0).await;

        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_ACCOUNT_REF)
            .bind(2_i64)
            .bind("CASH")
            .bind("BANK")
            .execute(&pool)
            .await
            .unwrap();

        let err = delete_account(&pool, 2, "CASH").await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
        assert_eq!(err.entity.as_deref(), Some("account"));

        assert_eq!(get_accounts(&pool, 2).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_delete_account_rejected_when_referenced_as_to_account() {
        let pool = setup_test_db().await;
        add_test_account(&pool, "CASH", 0).await;

        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_ACCOUNT_REF)
            .bind(2_i64)
            .bind("BANK")
            .bind("CASH")
            .execute(&pool)
            .await
            .unwrap();

        let err = delete_account(&pool, 2, "CASH").await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
    }

    #[tokio::test]
    async fn test_delete_account_rejected_when_referenced_by_recurring_rule() {
        let pool = setup_test_db().await;
        add_test_account(&pool, "CASH", 0).await;

        sqlx::query(sql_queries::TEST_INSERT_RECURRING_RULES_ACCOUNT_REF)
            .bind(2_i64)
            .bind("CASH")
            .bind("BANK")
            .execute(&pool)
            .await
            .unwrap();

        let err = delete_account(&pool, 2, "CASH").await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
    }

    #[tokio::test]
    async fn test_delete_account_ignores_other_users_references() {
        let pool = setup_test_db().await;
        add_test_account(&pool, "CASH", 0).await;

        // User 1's transaction references the same code, but user 2's
        // delete must succeed — codes are scoped by user.
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_ACCOUNT_REF)
            .bind(1_i64)
            .bind("CASH")
            .bind("BANK")
            .execute(&pool)
            .await
            .unwrap();

        assert!(delete_account(&pool, 2, "CASH").await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_account_normalizes_input_before_in_use_check() {
        let pool = setup_test_db().await;
        add_test_account(&pool, "CASH", 0).await;

        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_ACCOUNT_REF)
            .bind(2_i64)
            .bind("CASH")
            .bind("BANK")
            .execute(&pool)
            .await
            .unwrap();

        // Lower-case + spacing is normalised on the way in — the guard
        // still sees CASH in the DB and blocks the delete.
        let err = delete_account(&pool, 2, "  cash  ").await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
    }

    #[tokio::test]
    async fn test_initialize_none_account_is_idempotent() {
        let pool = setup_test_db().await;
        sqlx::query(sql_queries::TEST_ACCOUNT_INSERT_NONE_TEMPLATE)
            .execute(&pool)
            .await
            .unwrap();

        initialize_none_account(&pool, 2).await.unwrap();
        // Second call swallows the duplicate-code error and still succeeds
        initialize_none_account(&pool, 2).await.unwrap();

        let accounts = get_accounts(&pool, 2).await.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].account_code, "NONE");
        assert_eq!(accounts[0].template_code, "NONE");
    }

    #[tokio::test]
    async fn test_get_account_balances_as_of_applies_transaction_signs() {
        let pool = setup_test_db().await;
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        add_test_account(&pool, "CASH", 1000).await;
        add_test_account(&pool, "BANK", 5000).await;

        insert_header(&pool, "INCOME", "NONE", "CASH", "2026-01-05", 300, 0).await;
        insert_header(&pool, "EXPENSE", "CASH", "NONE", "2026-01-06", 100, 0).await;
        insert_header(&pool, "TRANSFER", "BANK", "CASH", "2026-01-07", 2000, 0).await;

        let balances = get_account_balances_as_of(&pool, 2, "2026-01-31")
            .await
            .unwrap();

        let cash = balances.iter().find(|b| b.account_code == "CASH").unwrap();
        let bank = balances.iter().find(|b| b.account_code == "BANK").unwrap();
        assert_eq!(cash.balance, 1000 + 300 - 100 + 2000);
        assert_eq!(bank.balance, 5000 - 2000);
    }

    #[tokio::test]
    async fn test_get_account_balances_as_of_ignores_future_and_scheduled() {
        let pool = setup_test_db().await;
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        add_test_account(&pool, "CASH", 1000).await;

        insert_header(&pool, "INCOME", "NONE", "CASH", "2026-01-05", 300, 0).await;
        // After the as-of date
        insert_header(&pool, "INCOME", "NONE", "CASH", "2026-02-01", 700, 0).await;
        // Not actualised yet
        insert_header(&pool, "INCOME", "NONE", "CASH", "2026-01-06", 900, 1).await;

        let balances = get_account_balances_as_of(&pool, 2, "2026-01-31")
            .await
            .unwrap();

        assert_eq!(balances.len(), 1);
        assert_eq!(balances[0].balance, 1300);
    }

    /// Fable-5 review #20 — the pre-fix `ACCOUNT_BALANCES_AS_OF` used a
    /// first-match CASE that put the TRANSFER-TO arm ahead of the
    /// TRANSFER-FROM arm. A self-transfer (from == to == this account)
    /// therefore matched only the +TO arm and inflated the account
    /// balance by the transfer amount. The write-side guard in
    /// `save_transaction_header` rejects new self-transfers, and the
    /// query was made symmetric (independent CASE per arm) so any
    /// stale row already in the DB nets to zero on the dashboard too.
    #[tokio::test]
    async fn test_get_account_balances_as_of_self_transfer_nets_to_zero() {
        let pool = setup_test_db().await;
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        add_test_account(&pool, "CASH", 1000).await;

        // Stale self-transfer written before the write-side guard existed.
        insert_header(&pool, "TRANSFER", "CASH", "CASH", "2026-01-05", 2000, 0).await;

        let balances = get_account_balances_as_of(&pool, 2, "2026-01-31")
            .await
            .unwrap();

        let cash = balances.iter().find(|b| b.account_code == "CASH").unwrap();
        assert_eq!(
            cash.balance, 1000,
            "self-transfer must net to zero (pre-fix returned 3000)"
        );
    }

    #[tokio::test]
    async fn test_get_account_balances_as_of_excludes_disabled_accounts() {
        let pool = setup_test_db().await;
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        add_test_account(&pool, "CASH", 1000).await;
        delete_account(&pool, 2, "CASH").await.unwrap();

        let balances = get_account_balances_as_of(&pool, 2, "2026-01-31")
            .await
            .unwrap();

        assert!(balances.is_empty());
    }

    #[tokio::test]
    async fn test_initialize_none_account_propagates_failure() {
        // No NONE template exists in the test fixtures, so initialization must
        // fail loudly instead of silently leaving the user without the account.
        let pool = setup_test_db().await;

        let err = initialize_none_account(&pool, 2).await.unwrap_err();
        assert!(err.contains("NONE template"), "unexpected error: {}", err);
        assert!(get_accounts(&pool, 2).await.unwrap().is_empty());
    }
}
