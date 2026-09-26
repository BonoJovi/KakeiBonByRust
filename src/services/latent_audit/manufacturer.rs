//! Latent-audit 2026-09 regression tests for the Manufacturer master (TDD red phase).

use super::*;
use crate::test_helpers::database::{init_db, TEST_DB_URL};

/// Copy of the module-test fixture (`TEST_MANUFACTURER_CREATE_TABLE` already
/// carries the production `UNIQUE(USER_ID, MANUFACTURER_NAME)`).
async fn setup_test_db() -> SqlitePool {
    let pool = init_db(TEST_DB_URL).await.unwrap();
    sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_MANUFACTURER_CREATE_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_PRODUCT_CREATE_TABLE).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_INSERT_USER_ADMIN).execute(&pool).await.unwrap();
    sqlx::query(sql_queries::TEST_INSERT_USER_GENERAL).execute(&pool).await.unwrap();
    pool
}

/// M6: same as H6 for manufacturers — the duplicate pre-check only sees
/// `IS_DISABLED = 0` rows while UNIQUE(USER_ID, MANUFACTURER_NAME) covers
/// all rows, so re-adding a deleted name fails with a generic `database`
/// error.
///
/// Expected: success (revive/reuse) or structured `duplicate_name`, never
/// ApiError code `database`.
#[tokio::test]
async fn latent_m6_readd_deleted_manufacturer_name_is_not_database_error() {
    let pool = setup_test_db().await;
    let user_id = 2;

    add_manufacturer(
        &pool,
        user_id,
        AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        },
    )
    .await
    .expect("initial add");
    let id = get_manufacturers(&pool, user_id, true).await.expect("list")[0].manufacturer_id;
    delete_manufacturer(&pool, user_id, id).await.expect("logical delete");

    let result = add_manufacturer(
        &pool,
        user_id,
        AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        },
    )
    .await;

    if let Err(err) = &result {
        assert_ne!(
            err.code,
            ApiError::CODE_DATABASE,
            "re-adding a deleted manufacturer name must not surface a generic database error: {:?}",
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

fn add_req(name: &str, memo: Option<&str>) -> AddManufacturerRequest {
    AddManufacturerRequest {
        manufacturer_name: name.to_string(),
        memo: memo.map(str::to_string),
        is_disabled: None,
    }
}

/// M6 (chosen fix, same as shops / H6): adding the name of a disabled /
/// deleted manufacturer reuses that row — same MANUFACTURER_ID, enabled
/// again, carrying the new memo — instead of inserting a second row.
#[tokio::test]
async fn latent_m6_readd_deleted_manufacturer_name_revives_original_row() {
    let pool = setup_test_db().await;
    let user_id = 2;

    add_manufacturer(&pool, user_id, add_req("ニッスイ", None)).await.expect("initial add");
    let original_id = get_manufacturers(&pool, user_id, true).await.expect("list")[0].manufacturer_id;
    delete_manufacturer(&pool, user_id, original_id).await.expect("logical delete");

    add_manufacturer(&pool, user_id, add_req("ニッスイ", Some("再登録")))
        .await
        .expect("re-adding a deleted manufacturer name must succeed");

    let all = get_manufacturers(&pool, user_id, true).await.expect("list");
    let matching: Vec<_> = all.iter().filter(|m| m.manufacturer_name == "ニッスイ").collect();
    assert_eq!(matching.len(), 1, "exactly one row with the name: {:?}", all);
    assert_eq!(matching[0].manufacturer_id, original_id, "the original row must be reused");
    assert_eq!(matching[0].is_disabled, 0, "the reused row must be enabled");
    assert_eq!(matching[0].memo.as_deref(), Some("再登録"));
}

/// M6 (chosen fix): renaming onto a disabled manufacturer's name is rejected
/// as a structured duplicate_name error, not a raw UNIQUE violation.
#[tokio::test]
async fn latent_m6_rename_onto_disabled_manufacturer_name_is_duplicate_name() {
    let pool = setup_test_db().await;
    let user_id = 2;

    for name in ["ニッスイ", "マルハ"] {
        add_manufacturer(&pool, user_id, add_req(name, None)).await.expect("add");
    }
    let all = get_manufacturers(&pool, user_id, true).await.expect("list");
    let id_of = |name: &str| all.iter().find(|m| m.manufacturer_name == name).expect(name).manufacturer_id;
    let (nissui, maruha) = (id_of("ニッスイ"), id_of("マルハ"));
    delete_manufacturer(&pool, user_id, nissui).await.expect("logical delete");

    let err = update_manufacturer(
        &pool,
        user_id,
        maruha,
        UpdateManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            display_order: 1,
            is_disabled: 0,
        },
    )
    .await
    .expect_err("renaming onto a disabled manufacturer's name must be rejected");
    assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME, "{:?}", err);
}
