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
#[ignore = "latent-audit H6"]
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
