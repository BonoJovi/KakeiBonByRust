//! Latent-audit 2026-09 regression tests for the Product master (TDD red phase).

use super::*;
use crate::test_helpers::database::{init_db, TEST_DB_URL};

/// Copy of the module-test fixture (`TEST_PRODUCT_CREATE_TABLE` already
/// carries the production `UNIQUE(USER_ID, PRODUCT_NAME)`).
async fn setup_test_db() -> SqlitePool {
    let pool = init_db(TEST_DB_URL).await.unwrap();
    sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_MANUFACTURER_CREATE_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_PRODUCT_CREATE_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_CREATE_TRANSACTIONS_DETAIL_MINIMAL).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_INSERT_USER_ADMIN).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_INSERT_USER_GENERAL).execute(&pool).await.unwrap();
    pool
}

/// M6: same as H6 for products — the duplicate pre-check only sees
/// `IS_DISABLED = 0` rows while UNIQUE(USER_ID, PRODUCT_NAME) covers all
/// rows, so re-adding a deleted name fails with a generic `database` error.
///
/// Expected: success (revive/reuse) or structured `duplicate_name`, never
/// ApiError code `database`.
#[tokio::test]
#[ignore = "latent-audit M6"]
async fn latent_m6_readd_deleted_product_name_is_not_database_error() {
    let pool = setup_test_db().await;
    let user_id = 2;

    add_product(
        &pool,
        user_id,
        AddProductRequest {
            product_name: "牛乳".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        },
    )
    .await
    .expect("initial add");
    let id = get_products(&pool, user_id, true).await.expect("list")[0].product_id;
    delete_product(&pool, user_id, id).await.expect("logical delete");

    let result = add_product(
        &pool,
        user_id,
        AddProductRequest {
            product_name: "牛乳".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        },
    )
    .await;

    if let Err(err) = &result {
        assert_ne!(
            err.code,
            ApiError::CODE_DATABASE,
            "re-adding a deleted product name must not surface a generic database error: {:?}",
            err
        );
        assert_eq!(
            err.code,
            ApiError::CODE_DUPLICATE_NAME,
            "if rejected, it must be a structured duplicate_name error: {:?}",
            err
        );
    }
}
