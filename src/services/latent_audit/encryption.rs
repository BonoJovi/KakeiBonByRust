//! Latent-audit 2026-09 regression tests (TDD red phase) for
//! `services::encryption`. Every test is `#[ignore]`d and asserts the
//! CORRECT behaviour, so it fails on the current code and passes once the
//! corresponding bug is fixed.

use super::*;
use crate::security::verify_password;
use crate::services::user_management::UserManagementService;
use crate::test_helpers::database::{create_test_admin, setup_test_db};

const ADMIN_CREDENTIAL: &str = "admin_password123456";
const OLD_CREDENTIAL: &str = "password_123456789";
const NEW_CREDENTIAL: &str = "new_password_123456";

/// Register `table.column` as an encrypted field, then change a general
/// user's password. Returns the password-change result.
async fn change_password_after_registering(
    table: &str,
    column: &str,
) -> (Result<(), String>, SqlitePool, i64) {
    let pool = setup_test_db().await;
    create_test_admin(&pool, "admin", ADMIN_CREDENTIAL).await;
    let users = UserManagementService::new(pool.clone());
    let user_id = users
        .register_general_user("alice", OLD_CREDENTIAL)
        .await
        .expect("register general user");

    // The admin-only command accepts this today (identifier syntax is the
    // only check). A fix may reject the registration instead — both are fine
    // as long as the password change below still works.
    let _ = EncryptionService::new(pool.clone())
        .register_encrypted_field(table, column, Some("latent-audit L27"))
        .await;

    let result = users
        .update_general_user_with_password(user_id, OLD_CREDENTIAL, None, Some(NEW_CREDENTIAL))
        .await
        .map_err(|e| e.to_string());
    (result, pool, user_id)
}

/// L27 — register_encrypted_field accepts any table/column that passes the
/// identifier-syntax check (e.g. USERS.NAME, a plaintext column). The next
/// password change then tries to decrypt plaintext, fails, and every user's
/// password change becomes impossible.
/// Expected: after such a registration attempt, a password change still
/// succeeds (either the registration is rejected or the column is ignored).
#[tokio::test]
#[ignore = "latent-audit L27"]
async fn latent_l27_password_change_survives_plaintext_column_registration() {
    let (result, pool, user_id) = change_password_after_registering("USERS", "NAME").await;
    assert!(
        result.is_ok(),
        "password change must still succeed after registering USERS.NAME: {:?}",
        result
    );

    let hash: String = sqlx::query_scalar(sql_queries::TEST_USER_GET_PASSWORD_BY_ID)
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(verify_password(NEW_CREDENTIAL, &hash).unwrap());
}

/// L27 — same failure mode for a non-text column: ACCOUNTS.INITIAL_BALANCE
/// (INTEGER) cannot be decoded as an encrypted TEXT value, so re-encryption
/// aborts and the password change is refused.
/// Expected: the password change still succeeds.
#[tokio::test]
#[ignore = "latent-audit L27"]
async fn latent_l27_password_change_survives_non_text_column_registration() {
    let (result, _pool, _user_id) =
        change_password_after_registering("ACCOUNTS", "INITIAL_BALANCE").await;
    assert!(
        result.is_ok(),
        "password change must still succeed after registering ACCOUNTS.INITIAL_BALANCE: {:?}",
        result
    );
}
