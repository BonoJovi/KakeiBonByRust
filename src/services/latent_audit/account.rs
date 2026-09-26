//! Latent-audit 2026-09 regression tests for the Account master (TDD red phase).

use super::*;
use crate::test_helpers::database::{init_db, TEST_DB_URL};

/// Copy of the module-test fixture. USER_ID 1 = admin, 2 = general user.
async fn setup_test_db() -> SqlitePool {
    let pool = init_db(TEST_DB_URL).await.unwrap();
    sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_ACCOUNT_CREATE_TEMPLATES_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_ACCOUNT_CREATE_ACCOUNTS_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_CREATE_RECURRING_RULES_MINIMAL).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_INSERT_USER_ADMIN).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_INSERT_USER_GENERAL).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_ACCOUNT_INSERT_TEMPLATES).execute(&pool).await.unwrap();
    pool
}

fn req(code: &str, name: &str) -> AddAccountRequest {
    AddAccountRequest {
        account_code: code.to_string(),
        account_name: name.to_string(),
        template_code: "BANK".to_string(),
        initial_balance: 0,
    }
}

/// M4 (仕様確認待ち): `lib.rs::get_accounts` routes admins (ROLE_ADMIN) to
/// `get_all_accounts`, whose SQL (`ACCOUNT_LIST_ALL`) has no USER_ID and no
/// IS_DISABLED filter — the admin's list therefore contains other users'
/// accounts and logically deleted accounts. Editing one by code then
/// rewrites the admin's own same-code account, and the transaction /
/// recurring dropdowns get polluted.
///
/// Expected: the account list served to the admin contains only the
/// admin's own, active accounts.
#[tokio::test]
#[ignore = "latent-audit M4 (仕様確認待ち)"]
async fn latent_m4_admin_account_list_excludes_other_users_and_deleted() {
    let pool = setup_test_db().await;
    let admin_id = 1;
    let other_id = 2;

    add_account(&pool, admin_id, req("ADMINBANK", "Admin bank")).await.expect("admin add");
    add_account(&pool, admin_id, req("OLDCARD", "Deleted card")).await.expect("admin add 2");
    delete_account(&pool, admin_id, "OLDCARD").await.expect("admin logical delete");
    add_account(&pool, other_id, req("USERBANK", "Other user's bank")).await.expect("user add");

    // The service `lib.rs::get_accounts` uses for admins today.
    let accounts = get_all_accounts(&pool).await.expect("list");

    let foreign: Vec<_> = accounts.iter().filter(|a| a.user_id != admin_id).collect();
    assert!(
        foreign.is_empty(),
        "admin account list must not include other users' accounts: {:?}",
        foreign
    );
    let disabled: Vec<_> = accounts.iter().filter(|a| a.is_disabled != 0).collect();
    assert!(
        disabled.is_empty(),
        "admin account list must not include logically deleted accounts: {:?}",
        disabled
    );
    assert!(
        accounts.iter().any(|a| a.account_code == "ADMINBANK"),
        "admin's own active account must still be listed"
    );
}
