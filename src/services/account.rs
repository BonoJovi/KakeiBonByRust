use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use crate::sql_queries;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct AccountTemplate {
    pub template_id: i64,
    pub template_code: String,
    pub template_name_ja: String,
    pub template_name_en: String,
    pub display_order: i64,
    pub entry_dt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
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
pub async fn get_account_templates(pool: &SqlitePool) -> Result<Vec<AccountTemplate>, String> {
    let templates = sqlx::query_as::<_, AccountTemplate>(
        r#"
        SELECT TEMPLATE_ID as template_id, TEMPLATE_CODE as template_code, 
               TEMPLATE_NAME_JA as template_name_ja, TEMPLATE_NAME_EN as template_name_en, 
               DISPLAY_ORDER as display_order, ENTRY_DT as entry_dt
        FROM ACCOUNT_TEMPLATES
        ORDER BY DISPLAY_ORDER
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to get account templates: {}", e))?;

    Ok(templates)
}

/// Get all accounts for a user
pub async fn get_accounts(pool: &SqlitePool, user_id: i64) -> Result<Vec<Account>, String> {
    let accounts = sqlx::query_as::<_, Account>(
        r#"
        SELECT ACCOUNT_ID as account_id, USER_ID as user_id, ACCOUNT_CODE as account_code, 
               ACCOUNT_NAME as account_name, TEMPLATE_CODE as template_code, 
               INITIAL_BALANCE as initial_balance, DISPLAY_ORDER as display_order, 
               IS_DISABLED as is_disabled, ENTRY_DT as entry_dt, UPDATE_DT as update_dt
        FROM ACCOUNTS
        WHERE USER_ID = ? AND IS_DISABLED = 0
        ORDER BY DISPLAY_ORDER, ACCOUNT_CODE
        "#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to get accounts: {}", e))?;

    Ok(accounts)
}

/// Get a single account by code
pub async fn get_account_by_code(
    pool: &SqlitePool,
    user_id: i64,
    account_code: &str,
) -> Result<Option<Account>, String> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        SELECT ACCOUNT_ID as account_id, USER_ID as user_id, ACCOUNT_CODE as account_code, 
               ACCOUNT_NAME as account_name, TEMPLATE_CODE as template_code, 
               INITIAL_BALANCE as initial_balance, DISPLAY_ORDER as display_order, 
               IS_DISABLED as is_disabled, ENTRY_DT as entry_dt, UPDATE_DT as update_dt
        FROM ACCOUNTS
        WHERE USER_ID = ? AND ACCOUNT_CODE = ?
        "#
    )
    .bind(user_id)
    .bind(account_code)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to get account: {}", e))?;

    Ok(account)
}

/// Check if account code already exists
async fn check_duplicate_code(
    pool: &SqlitePool,
    user_id: i64,
    account_code: &str,
) -> Result<bool, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::ACCOUNT_CHECK_DUPLICATE_CODE)
        .bind(user_id)
        .bind(account_code)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check duplicate code: {}", e))?;

    Ok(result.0 > 0)
}

/// Get next display order
async fn get_next_display_order(pool: &SqlitePool, user_id: i64) -> Result<i64, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::ACCOUNT_GET_NEXT_DISPLAY_ORDER)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to get next display order: {}", e))?;

    Ok(result.0)
}

/// Add a new account (or reactivate if deleted)
pub async fn add_account(
    pool: &SqlitePool,
    user_id: i64,
    request: AddAccountRequest,
) -> Result<String, String> {
    // Validate account code
    if request.account_code.trim().is_empty() {
        return Err("Account code cannot be empty".to_string());
    }

    // Check for duplicate code (only active accounts)
    if check_duplicate_code(pool, user_id, &request.account_code).await? {
        return Err("Account code already exists".to_string());
    }

    // Get next display order
    let display_order = get_next_display_order(pool, user_id).await?;

    // Upsert account (insert or reactivate if deleted)
    sqlx::query(sql_queries::ACCOUNT_UPSERT)
        .bind(user_id)
        .bind(&request.account_code)
        .bind(&request.account_name)
        .bind(&request.template_code)
        .bind(request.initial_balance)
        .bind(display_order)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to add account: {}", e))?;

    Ok("Account added successfully".to_string())
}

/// Update an account
pub async fn update_account(
    pool: &SqlitePool,
    user_id: i64,
    request: UpdateAccountRequest,
) -> Result<String, String> {
    // Check if account exists
    get_account_by_code(pool, user_id, &request.account_code)
        .await?
        .ok_or("Account not found")?;

    // Update account
    sqlx::query(sql_queries::ACCOUNT_UPDATE)
        .bind(&request.account_name)
        .bind(&request.template_code)
        .bind(request.initial_balance)
        .bind(request.display_order)
        .bind(user_id)
        .bind(&request.account_code)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update account: {}", e))?;

    Ok("Account updated successfully".to_string())
}

/// Delete an account (logical deletion)
pub async fn delete_account(
    pool: &SqlitePool,
    user_id: i64,
    account_code: &str,
) -> Result<String, String> {
    // Check if account exists
    get_account_by_code(pool, user_id, account_code)
        .await?
        .ok_or("Account not found")?;

    // Logical delete
    sqlx::query(sql_queries::ACCOUNT_DELETE_LOGICAL)
        .bind(user_id)
        .bind(account_code)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete account: {}", e))?;

    Ok("Account deleted successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::database::init_db;

    async fn setup_test_db() -> SqlitePool {
        let pool = init_db(":memory:").await.unwrap();
        
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

        // Add second time - should fail
        let result = add_account(&pool, 2, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }
}
