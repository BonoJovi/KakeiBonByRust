//! Latent-audit 2026-09 regression tests for the Shop master (TDD red phase).
//! Wired from `src/services/shop.rs` via `#[path]`; `use super::*;` reaches
//! the parent module's private items.

use super::*;
use crate::test_helpers::database::{create_test_user, setup_test_db};

/// Build the test DB from the PRODUCTION schema (`res/sql/dbaccess.sql`),
/// which carries `UNIQUE(USER_ID, SHOP_NAME)` on SHOPS. The module-local
/// `TEST_SHOP_CREATE_TABLE` lacks that constraint and hides H6.
async fn setup_production_schema_db() -> (SqlitePool, i64) {
    let pool = setup_test_db().await;
    let user_id = create_test_user(&pool, "latent_h6_user", "latent-audit-password-16").await;
    (pool, user_id)
}

/// H6: the duplicate pre-check only looks at `IS_DISABLED = 0`, but the
/// production UNIQUE(USER_ID, SHOP_NAME) covers every row, so re-adding a
/// logically deleted shop name fails with a generic `database` error.
///
/// Expected: either success (revive/reuse the row) or a structured
/// `duplicate_name` error — never ApiError code `database`.
#[tokio::test]
async fn latent_h6_readd_deleted_shop_name_is_not_database_error() {
    let (pool, user_id) = setup_production_schema_db().await;

    add_shop(
        &pool,
        user_id,
        AddShopRequest { shop_name: "イオン".to_string(), memo: None },
    )
    .await
    .expect("initial add");
    let shop_id = get_shops(&pool, user_id).await.expect("list")[0].shop_id;
    delete_shop(&pool, user_id, shop_id).await.expect("logical delete");

    let result = add_shop(
        &pool,
        user_id,
        AddShopRequest { shop_name: "イオン".to_string(), memo: None },
    )
    .await;

    if let Err(err) = &result {
        assert_ne!(
            err.code,
            ApiError::CODE_DATABASE,
            "re-adding a deleted shop name must not surface a generic database error: {:?}",
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

/// H6 (chosen fix): re-adding a deleted shop name revives the original row
/// — same SHOP_ID, active again, carrying the new memo — rather than
/// inserting a second row.
#[tokio::test]
async fn latent_h6_readd_deleted_shop_name_revives_original_row() {
    let (pool, user_id) = setup_production_schema_db().await;

    add_shop(
        &pool,
        user_id,
        AddShopRequest { shop_name: "イオン".to_string(), memo: None },
    )
    .await
    .expect("initial add");
    let original_id = get_shops(&pool, user_id).await.expect("list")[0].shop_id;
    delete_shop(&pool, user_id, original_id).await.expect("logical delete");

    add_shop(
        &pool,
        user_id,
        AddShopRequest { shop_name: "イオン".to_string(), memo: Some("再登録".to_string()) },
    )
    .await
    .expect("re-adding a deleted shop name must succeed");

    let shops = get_shops(&pool, user_id).await.expect("list");
    let matching: Vec<_> = shops.iter().filter(|s| s.shop_name == "イオン").collect();
    assert_eq!(matching.len(), 1, "exactly one active shop with the name: {:?}", shops);
    assert_eq!(matching[0].shop_id, original_id, "the original row must be revived");
    assert_eq!(matching[0].memo.as_deref(), Some("再登録"));
}

/// H6 (chosen fix): renaming a shop onto a deleted shop's name is rejected
/// as a structured duplicate_name error, not a raw UNIQUE violation.
#[tokio::test]
async fn latent_h6_rename_onto_deleted_shop_name_is_duplicate_name() {
    let (pool, user_id) = setup_production_schema_db().await;

    for name in ["イオン", "ダイソー"] {
        add_shop(&pool, user_id, AddShopRequest { shop_name: name.to_string(), memo: None })
            .await
            .expect("add");
    }
    let shops = get_shops(&pool, user_id).await.expect("list");
    let id_of = |name: &str| shops.iter().find(|s| s.shop_name == name).expect(name).shop_id;
    let (aeon, daiso) = (id_of("イオン"), id_of("ダイソー"));
    delete_shop(&pool, user_id, aeon).await.expect("logical delete");

    let err = update_shop(
        &pool,
        user_id,
        daiso,
        UpdateShopRequest { shop_name: "イオン".to_string(), memo: None, display_order: 1 },
    )
    .await
    .expect_err("renaming onto a deleted shop's name must be rejected");
    assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME, "{:?}", err);
}

/// H6 follow-up (CodeRabbit #142): an INSERT that hits UNIQUE(USER_ID,
/// SHOP_NAME) after slipping past the pre-check — e.g. two concurrent adds
/// of the same name — is reported as duplicate_name, not `database`.
/// Simulated with a trigger that inserts the conflicting row first.
#[tokio::test]
async fn latent_h6_insert_unique_violation_maps_to_duplicate_name() {
    let (pool, user_id) = setup_production_schema_db().await;

    // The pre-check is bypassed by racing a trigger that inserts the same
    // name right before add_shop's INSERT lands. Not TEMP: a temp trigger
    // only exists on the connection that created it, and the pool may run
    // add_shop on another one (it did on CI).
    sqlx::query(
        "CREATE TRIGGER race_same_name BEFORE INSERT ON SHOPS \
         WHEN NEW.SHOP_NAME = 'イオン' AND NOT EXISTS (SELECT 1 FROM SHOPS WHERE SHOP_NAME = 'イオン') \
         BEGIN INSERT INTO SHOPS (USER_ID, SHOP_NAME, DISPLAY_ORDER) VALUES (NEW.USER_ID, NEW.SHOP_NAME, 0); END",
    )
    .execute(&pool)
    .await
    .expect("create race trigger");

    let err = add_shop(
        &pool,
        user_id,
        AddShopRequest { shop_name: "イオン".to_string(), memo: None },
    )
    .await
    .expect_err("the raced INSERT must be rejected");
    assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME, "{:?}", err);
}
