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

fn add_req(name: &str, memo: Option<&str>) -> AddProductRequest {
    AddProductRequest {
        product_name: name.to_string(),
        manufacturer_id: None,
        memo: memo.map(str::to_string),
        is_disabled: None,
    }
}

/// M6 (chosen fix, same as shops / H6): adding the name of a disabled /
/// deleted product reuses that row — same PRODUCT_ID (so existing detail
/// links stay valid), enabled again, carrying the new memo.
#[tokio::test]
async fn latent_m6_readd_deleted_product_name_revives_original_row() {
    let pool = setup_test_db().await;
    let user_id = 2;

    add_product(&pool, user_id, add_req("サバ缶", None)).await.expect("initial add");
    let original_id = get_products(&pool, user_id, true).await.expect("list")[0].product_id;
    delete_product(&pool, user_id, original_id).await.expect("logical delete");

    add_product(&pool, user_id, add_req("サバ缶", Some("再登録")))
        .await
        .expect("re-adding a deleted product name must succeed");

    let all = get_products(&pool, user_id, true).await.expect("list");
    let matching: Vec<_> = all.iter().filter(|p| p.product_name == "サバ缶").collect();
    assert_eq!(matching.len(), 1, "exactly one row with the name: {:?}", all);
    assert_eq!(matching[0].product_id, original_id, "the original row must be reused");
    assert_eq!(matching[0].is_disabled, 0, "the reused row must be enabled");
    assert_eq!(matching[0].memo.as_deref(), Some("再登録"));
}

/// M6 (chosen fix): renaming onto a disabled product's name is rejected as a
/// structured duplicate_name error, not a raw UNIQUE violation.
#[tokio::test]
async fn latent_m6_rename_onto_disabled_product_name_is_duplicate_name() {
    let pool = setup_test_db().await;
    let user_id = 2;

    for name in ["サバ缶", "イワシ缶"] {
        add_product(&pool, user_id, add_req(name, None)).await.expect("add");
    }
    let all = get_products(&pool, user_id, true).await.expect("list");
    let id_of = |name: &str| all.iter().find(|p| p.product_name == name).expect(name).product_id;
    let (saba, iwashi) = (id_of("サバ缶"), id_of("イワシ缶"));
    delete_product(&pool, user_id, saba).await.expect("logical delete");

    let err = update_product(
        &pool,
        user_id,
        iwashi,
        UpdateProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: None,
            display_order: 1,
            is_disabled: 0,
        },
    )
    .await
    .expect_err("renaming onto a disabled product's name must be rejected");
    assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME, "{:?}", err);
}
