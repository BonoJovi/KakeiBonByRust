//! Latent-audit 2026-09 regression tests (TDD red phase) for
//! `services::auth`. Every test is `#[ignore]`d and asserts the CORRECT
//! behaviour, so it fails on the current code and passes once the
//! corresponding bug is fixed.

use super::*;
use crate::api_error::ApiError;
use crate::test_helpers::database::setup_test_db;

/// Build a credential of at least MIN_PASSWORD_LENGTH characters at runtime
/// (same shape as `tests::test_credential`).
fn test_credential() -> String {
    let letters: String = ('a'..='p').collect();
    format!("{}{}", letters.to_uppercase(), letters)
}

async fn count_users(pool: &SqlitePool) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM USERS")
        .fetch_one(pool)
        .await
        .expect("count users")
}

async fn count_for_user(pool: &SqlitePool, table: &str, user_id: i64) -> i64 {
    let sql = format!("SELECT COUNT(*) FROM {} WHERE USER_ID = ?", table);
    sqlx::query_scalar(&sql)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .expect("count query")
}

// ---------------------------------------------------------------------------
// L25 — register_admin / register_user do not validate the username
// ---------------------------------------------------------------------------

/// L25 — register_admin_user / register_user accept an empty or
/// whitespace-only username (no validation at all on the setup path).
/// Expected: blank usernames are rejected and no USERS row is created.
#[tokio::test]
#[ignore = "latent-audit L25"]
async fn latent_l25_register_rejects_blank_username() {
    let pool = setup_test_db().await;
    let auth = AuthService::new(pool.clone());
    let credential = test_credential();

    for name in ["", "   "] {
        let r = auth.register_admin_user(name, &credential).await;
        assert!(r.is_err(), "register_admin_user({:?}) must be rejected", name);
    }
    assert_eq!(count_users(&pool).await, 0, "no admin row may be created for a blank name");

    auth.register_admin_user("admin", &credential).await.unwrap();
    let r = auth.register_user("   ", &credential).await;
    assert!(r.is_err(), "register_user(blank) must be rejected");
    assert_eq!(count_users(&pool).await, 1);
}

/// L25 — the setup path skips the MAX_NAME_LEN guard that
/// user_management applies, so an over-long name is stored.
/// Expected: a name longer than MAX_NAME_LEN chars is rejected.
#[tokio::test]
#[ignore = "latent-audit L25"]
async fn latent_l25_register_rejects_overlong_username() {
    let pool = setup_test_db().await;
    let auth = AuthService::new(pool.clone());
    let credential = test_credential();
    let too_long = "あ".repeat(crate::consts::MAX_NAME_LEN + 1);

    let r = auth.register_admin_user(&too_long, &credential).await;
    assert!(r.is_err(), "over-long admin name must be rejected");
    assert_eq!(count_users(&pool).await, 0);

    auth.register_admin_user("admin", &credential).await.unwrap();
    let r = auth.register_user(&too_long, &credential).await;
    assert!(r.is_err(), "over-long user name must be rejected");
    assert_eq!(count_users(&pool).await, 1);
}

/// L25 — a duplicate username on register_user surfaces the raw sqlx
/// "UNIQUE constraint failed" message under the generic `database` code.
/// Expected: it maps to the structured `duplicate_name` ApiError code.
#[tokio::test]
#[ignore = "latent-audit L25"]
async fn latent_l25_register_duplicate_name_maps_to_duplicate_code() {
    let pool = setup_test_db().await;
    let auth = AuthService::new(pool.clone());
    let credential = test_credential();

    auth.register_admin_user("admin", &credential).await.unwrap();
    let err = auth
        .register_user("admin", &credential)
        .await
        .expect_err("duplicate username must fail");
    let api: ApiError = err.into();

    assert_eq!(
        api.code,
        ApiError::CODE_DUPLICATE_NAME,
        "duplicate name must map to duplicate_name, got code={} message={}",
        api.code,
        api.message
    );
    assert!(
        !api.message.contains("UNIQUE constraint failed"),
        "raw sqlx message leaked: {}",
        api.message
    );
}

// ---------------------------------------------------------------------------
// L26 — post-registration seeding is outside the user-creation transaction
// ---------------------------------------------------------------------------

/// L26 — register_admin_user commits the USERS row before seeding default
/// categories; if seeding fails the error is returned but the user row
/// stays, so setup is "completed" (has_users()=true) with no categories and
/// cannot be retried.
/// Failure is injected deterministically by dropping CATEGORY3_I18N (a leaf
/// table the category seed writes to).
/// Expected: registration fails AND leaves no user row behind (atomic).
#[tokio::test]
#[ignore = "latent-audit L26"]
async fn latent_l26_admin_category_seed_failure_rolls_back_user() {
    let pool = setup_test_db().await;
    sqlx::query("DROP TABLE CATEGORY3_I18N")
        .execute(&pool)
        .await
        .expect("drop CATEGORY3_I18N");
    let auth = AuthService::new(pool.clone());

    let r = auth.register_admin_user("admin", &test_credential()).await;
    assert!(r.is_err(), "precondition: seeding failure must surface as Err");

    assert_eq!(
        count_users(&pool).await,
        0,
        "user row must not survive a failed seeding (registration must be atomic)"
    );
    assert!(
        !auth.has_users().await.unwrap(),
        "setup must remain retryable after a seeding failure"
    );
}

/// L26 — same as above for the NONE-account step: categories and the user
/// row are already committed when initialize_none_account fails.
/// Failure is injected by removing the NONE account template.
/// Expected: registration fails AND leaves neither the user row nor the
/// categories behind.
#[tokio::test]
#[ignore = "latent-audit L26"]
async fn latent_l26_admin_none_account_failure_rolls_back_user_and_categories() {
    let pool = setup_test_db().await;
    sqlx::query("DELETE FROM ACCOUNT_TEMPLATES WHERE TEMPLATE_CODE = 'NONE'")
        .execute(&pool)
        .await
        .expect("delete NONE template");
    let auth = AuthService::new(pool.clone());

    let r = auth.register_admin_user("admin", &test_credential()).await;
    assert!(r.is_err(), "precondition: NONE-account failure must surface as Err");

    assert_eq!(count_users(&pool).await, 0, "user row must be rolled back");
    assert_eq!(
        count_for_user(&pool, "CATEGORY2", 1).await,
        0,
        "seeded categories must be rolled back together with the user"
    );
}

/// L26 — register_user (general-user setup) has the same non-atomic shape.
/// Expected: a seeding failure leaves no general-user row behind, so
/// has_general_users() stays false and the setup step can be retried.
#[tokio::test]
#[ignore = "latent-audit L26"]
async fn latent_l26_register_user_seed_failure_rolls_back_user() {
    let pool = setup_test_db().await;
    let auth = AuthService::new(pool.clone());
    let credential = test_credential();
    auth.register_admin_user("admin", &credential).await.unwrap();

    sqlx::query("DROP TABLE CATEGORY3_I18N")
        .execute(&pool)
        .await
        .expect("drop CATEGORY3_I18N");

    let r = auth.register_user("member", &credential).await;
    assert!(r.is_err(), "precondition: seeding failure must surface as Err");

    assert!(
        !auth.has_general_users().await.unwrap(),
        "general-user row must not survive a failed seeding"
    );
    assert_eq!(count_users(&pool).await, 1);
}
