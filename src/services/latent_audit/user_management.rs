//! Latent-audit 2026-09 regression tests (TDD red phase) for
//! `services::user_management`. Every test is `#[ignore]`d and asserts the
//! CORRECT behaviour, so it fails on the current code and passes once the
//! corresponding bug is fixed.

use super::*;
use crate::services::category::CategoryService;
use crate::test_helpers::database::{create_test_admin, setup_test_db};
use sqlx::SqlitePool;

const ADMIN_CREDENTIAL: &str = "admin_password123456";
const USER_CREDENTIAL: &str = "password_123456789";

/// Tables holding per-user category data that must not outlive the user.
const CATEGORY_TABLES: [&str; 6] = [
    "CATEGORY1",
    "CATEGORY2",
    "CATEGORY3",
    "CATEGORY1_I18N",
    "CATEGORY2_I18N",
    "CATEGORY3_I18N",
];

async fn count_rows_for_user(pool: &SqlitePool, table: &str, user_id: i64) -> i64 {
    // Table name comes from the fixed CATEGORY_TABLES list above.
    let sql = format!("SELECT COUNT(*) FROM {} WHERE USER_ID = ?", table);
    sqlx::query_scalar(&sql)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .expect("count query")
}

/// Mirror of the `create_general_user` tauri command: register the user,
/// then seed the default categories.
async fn create_general_user_like_command(pool: &SqlitePool, name: &str) -> i64 {
    let service = UserManagementService::new(pool.clone());
    let user_id = service
        .register_general_user(name, USER_CREDENTIAL)
        .await
        .expect("register general user");
    CategoryService::new(pool.clone())
        .populate_default_categories(user_id)
        .await
        .expect("populate default categories");
    user_id
}

async fn count_category2_named(pool: &SqlitePool, user_id: i64, name: &str) -> i64 {
    sqlx::query_scalar(
        "SELECT COUNT(*) FROM CATEGORY2_I18N WHERE USER_ID = ? AND CATEGORY2_NAME_I18N = ?",
    )
    .bind(user_id)
    .bind(name)
    .fetch_one(pool)
    .await
    .expect("count custom category2")
}

/// M3 — deleting a general user leaves CATEGORY1/2/3 and *_I18N rows behind
/// (no FK/CASCADE to USERS, delete_general_user only deletes the USERS row).
/// Expected: after deletion no category row for that USER_ID remains.
#[tokio::test]
#[ignore = "latent-audit M3"]
async fn latent_m3_delete_user_removes_categories() {
    let pool = setup_test_db().await;
    create_test_admin(&pool, "admin", ADMIN_CREDENTIAL).await;
    let user_id = create_general_user_like_command(&pool, "alice").await;

    // Precondition: the user really had category rows.
    assert!(count_rows_for_user(&pool, "CATEGORY2", user_id).await > 0);

    UserManagementService::new(pool.clone())
        .delete_general_user(user_id)
        .await
        .expect("delete general user");

    for table in CATEGORY_TABLES {
        let remaining = count_rows_for_user(&pool, table, user_id).await;
        assert_eq!(
            remaining, 0,
            "{} still has {} row(s) for deleted USER_ID={}",
            table, remaining, user_id
        );
    }
}

/// M3 — USER_ID is MAX+1, so after deleting the newest user the next user
/// reuses the same USER_ID and inherits the deleted user's customised
/// categories (populate_default_categories skips because CATEGORY2 rows exist).
/// Expected: the new user gets exactly the default category set.
#[tokio::test]
#[ignore = "latent-audit M3"]
async fn latent_m3_reused_user_id_gets_default_categories() {
    let pool = setup_test_db().await;
    create_test_admin(&pool, "admin", ADMIN_CREDENTIAL).await;
    let category = CategoryService::new(pool.clone());

    // Old user customises their categories.
    let old_id = create_general_user_like_command(&pool, "alice").await;
    category
        .add_category2(old_id, "EXPENSE", "削除済ユーザー独自費目", "DeletedUserCustomCat")
        .await
        .expect("add custom category2");
    UserManagementService::new(pool.clone())
        .delete_general_user(old_id)
        .await
        .expect("delete general user");

    // New user created through the same path.
    let new_id = create_general_user_like_command(&pool, "bob").await;
    assert_eq!(new_id, old_id, "precondition: USER_ID is reused (MAX+1)");

    // Reference: a clean user with the default set.
    let reference_id = create_general_user_like_command(&pool, "carol").await;

    assert_eq!(
        count_category2_named(&pool, new_id, "DeletedUserCustomCat").await,
        0,
        "new user inherited the deleted user's custom category"
    );
    for table in CATEGORY_TABLES {
        assert_eq!(
            count_rows_for_user(&pool, table, new_id).await,
            count_rows_for_user(&pool, table, reference_id).await,
            "{}: new user (reused id) must have the same default set as a fresh user",
            table
        );
    }
}

/// M13 — register_general_user accepts an empty / whitespace-only name
/// (frontend trims to '' and the backend only checks max length), creating
/// an account nobody can log in to.
/// Expected: both are rejected and no USERS row is created.
#[tokio::test]
#[ignore = "latent-audit M13"]
async fn latent_m13_create_rejects_blank_username() {
    let pool = setup_test_db().await;
    create_test_admin(&pool, "admin", ADMIN_CREDENTIAL).await;
    let service = UserManagementService::new(pool.clone());

    for name in ["", "   ", "\t\u{3000}"] {
        let result = service.register_general_user(name, USER_CREDENTIAL).await;
        assert!(
            result.is_err(),
            "blank username {:?} must be rejected, got {:?}",
            name,
            result
        );
    }
    let users = service.list_users().await.unwrap();
    assert_eq!(users.len(), 1, "only the admin should exist: {:?}", users);
}

/// M13 — the rename paths (update_general_user / update_admin_user /
/// *_with_password) accept a whitespace-only name, locking the user (even
/// the admin) out with no recovery path.
/// Expected: every update path rejects a blank name and keeps the old name.
#[tokio::test]
#[ignore = "latent-audit M13"]
async fn latent_m13_update_rejects_blank_username() {
    let pool = setup_test_db().await;
    let admin_id = create_test_admin(&pool, "admin", ADMIN_CREDENTIAL).await;
    let service = UserManagementService::new(pool.clone());
    let user_id = service
        .register_general_user("alice", USER_CREDENTIAL)
        .await
        .unwrap();

    let r = service.update_general_user(user_id, Some("   ")).await;
    assert!(r.is_err(), "update_general_user must reject blank name: {:?}", r);

    let r = service
        .update_general_user_with_password(user_id, USER_CREDENTIAL, Some(""), None)
        .await;
    assert!(r.is_err(), "update_general_user_with_password must reject blank name: {:?}", r);

    let r = service.update_admin_user(admin_id, Some(" ")).await;
    assert!(r.is_err(), "update_admin_user must reject blank name: {:?}", r);

    assert_eq!(service.get_user(user_id).await.unwrap().name, "alice");
    assert_eq!(service.get_user(admin_id).await.unwrap().name, "admin");
}
