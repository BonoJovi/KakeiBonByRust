use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use crate::api_error::ApiError;
use crate::services::like_escape::escape_like_pattern;
use crate::services::master_data::{self, MasterCrudSpec};
use crate::sql_queries;
use crate::validation;

/// Full description of the Product master's SQL surface + labels.
/// Fable-5 review #26.
const SPEC: MasterCrudSpec = MasterCrudSpec {
    entity_label: "Product",
    name_label: "Product name",
    check_duplicate_for_add_sql: sql_queries::PRODUCT_CHECK_DUPLICATE_FOR_ADD,
    check_duplicate_for_update_sql: sql_queries::PRODUCT_CHECK_DUPLICATE_FOR_UPDATE,
    delete_sql: sql_queries::PRODUCT_DELETE,
};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Product {
    pub product_id: i64,
    pub user_id: i64,
    pub product_name: String,
    pub manufacturer_id: Option<i64>,
    pub manufacturer_name: Option<String>,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddProductRequest {
    pub product_name: String,
    pub manufacturer_id: Option<i64>,
    pub memo: Option<String>,
    pub is_disabled: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub product_name: String,
    pub manufacturer_id: Option<i64>,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
}

/// Get all products for a user
pub async fn get_products(pool: &SqlitePool, user_id: i64, include_disabled: bool) -> Result<Vec<Product>, ApiError> {
    let query = if include_disabled {
        sql_queries::PRODUCT_GET_ALL_INCLUDING_DISABLED
    } else {
        sql_queries::PRODUCT_GET_ALL
    };

    let products = sqlx::query_as::<_, Product>(query)
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(products)
}

/// Product suggestions without a category. The Tauri command calls
/// `suggest_products` directly; this shorthand is kept for the module tests.
#[cfg(test)]
pub async fn search_products_by_name(
    pool: &SqlitePool,
    user_id: i64,
    query: &str,
) -> Result<Vec<Product>, ApiError> {
    suggest_products(pool, user_id, query, &SuggestCategory::default()).await
}

/// Category of the detail being entered, used to rank product suggestions.
#[derive(Debug, Default, Clone)]
pub struct SuggestCategory {
    pub category1_code: Option<String>,
    pub category2_code: Option<String>,
    pub category3_code: Option<String>,
}

/// Product suggestions for the detail item-name field.
///
/// Products whose name contains `query` (all products for an empty query),
/// ranked: used in details of `category` first, then other used products,
/// both by most recent use; never-used products last, by name. At most 20.
pub async fn suggest_products(
    pool: &SqlitePool,
    user_id: i64,
    query: &str,
    category: &SuggestCategory,
) -> Result<Vec<Product>, ApiError> {
    // Fable-5 review #23 — the previous form was
    // `format!("%{}%", trimmed)`, which passed `%` and `_` through as
    // LIKE wildcards. A product named "果汁100%ジュース" typed as
    // "100%ジ" then matched "100リンゴジュース" too. Escape the query
    // and pair with `LIKE ? ESCAPE '\'` in
    // `sql_queries::PRODUCT_SEARCH_BY_NAME` so the metacharacters
    // match literally.
    let pattern = format!("%{}%", escape_like_pattern(query.trim()));
    let category3 = category.category3_code.as_deref().filter(|c| !c.is_empty());

    let products = sqlx::query_as::<_, Product>(sql_queries::PRODUCT_SEARCH_BY_NAME)
        .bind(category.category1_code.as_deref())
        .bind(category.category2_code.as_deref())
        .bind(category3)
        .bind(category3)
        .bind(user_id)
        .bind(user_id)
        .bind(&pattern)
        .fetch_all(pool)
        .await?;

    Ok(products)
}

/// Get a single product by ID. Kept for module tests only after the
/// Fable-5 #26 refactor. See shop.rs::get_shop_by_id for rationale.
#[allow(dead_code)]
pub async fn get_product_by_id(
    pool: &SqlitePool,
    user_id: i64,
    product_id: i64,
) -> Result<Option<Product>, ApiError> {
    master_data::fetch_by_id(pool, sql_queries::PRODUCT_GET_BY_ID, user_id, product_id).await
}

/// Add a new product
pub async fn add_product(
    pool: &SqlitePool,
    user_id: i64,
    request: AddProductRequest,
) -> Result<String, ApiError> {
    validation::validate_master_name(SPEC.name_label, &request.product_name)
        .map_err(ApiError::validation)?;
    validation::validate_memo("Memo", request.memo.as_ref())
        .map_err(ApiError::validation)?;

    // Verify manufacturer ownership. PRODUCTS.MANUFACTURER_ID's FK only
    // enforces "some manufacturer row exists" — it does not check that
    // the manufacturer belongs to `user_id`. Without this guard, a direct
    // `invoke` from user B with a `manufacturer_id` that belongs to user A
    // would silently attach one of B's products to A's manufacturer,
    // and the PRODUCT_GET_ALL LEFT JOIN would then show A's manufacturer
    // name on B's product row (JOIN scope is fixed in the same PR).
    // Fable-5 review #13.
    verify_manufacturer_ownership(pool, user_id, request.manufacturer_id).await?;

    master_data::check_duplicate_for_add(&SPEC, pool, user_id, &request.product_name).await?;

    let display_order = master_data::fetch_next_display_order(
        pool,
        sql_queries::PRODUCT_GET_NEXT_DISPLAY_ORDER,
        user_id,
    )
    .await?;

    let is_disabled = request.is_disabled.unwrap_or(0);
    master_data::validate_is_disabled(is_disabled)?;

    // A disabled / logically deleted product with the same name still holds
    // the UNIQUE slot, so reuse that row instead of inserting
    // (latent-audit M6).
    let revived = sqlx::query(sql_queries::PRODUCT_REVIVE_DISABLED_BY_NAME)
        .bind(is_disabled)
        .bind(&request.manufacturer_id)
        .bind(&request.memo)
        .bind(display_order)
        .bind(user_id)
        .bind(&request.product_name)
        .execute(pool)
        .await?
        .rows_affected();

    if revived == 0 {
        sqlx::query(sql_queries::PRODUCT_INSERT)
            .bind(user_id)
            .bind(&request.product_name)
            .bind(&request.manufacturer_id)
            .bind(&request.memo)
            .bind(display_order)
            .bind(is_disabled)
            .execute(pool)
            .await
            .map_err(|e| master_data::map_insert_error(&SPEC, e))?;
    }

    Ok("Product added successfully".to_string())
}

/// If `manufacturer_id` is `Some`, confirm the manufacturer exists AND
/// belongs to `user_id`. Returns `ApiError::manufacturer_not_found()`
/// otherwise, which the frontend classifier maps to a specific "選択した
/// メーカーが見つかりません" toast rather than the generic
/// "保存失敗" — because the failure blames the manufacturer field, not
/// the product row. `None` (no manufacturer) is always OK — products can
/// be registered without a manufacturer.
async fn verify_manufacturer_ownership(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: Option<i64>,
) -> Result<(), ApiError> {
    let Some(id) = manufacturer_id else {
        return Ok(());
    };
    let exists: Option<i64> = sqlx::query_scalar(sql_queries::MANUFACTURER_EXISTS_FOR_USER)
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    if exists.is_none() {
        return Err(ApiError::manufacturer_not_found());
    }
    Ok(())
}

/// Update a product
pub async fn update_product(
    pool: &SqlitePool,
    user_id: i64,
    product_id: i64,
    request: UpdateProductRequest,
) -> Result<String, ApiError> {
    validation::validate_master_name(SPEC.name_label, &request.product_name)
        .map_err(ApiError::validation)?;
    validation::validate_memo("Memo", request.memo.as_ref())
        .map_err(ApiError::validation)?;
    master_data::validate_is_disabled(request.is_disabled)?;

    verify_manufacturer_ownership(pool, user_id, request.manufacturer_id).await?;

    master_data::check_duplicate_for_update(
        &SPEC,
        pool,
        user_id,
        product_id,
        &request.product_name,
    )
    .await?;

    // Pre-check + not_found is now derived from `rows_affected` on the
    // UPDATE itself (Fable-5 review #26). See shop.rs::update_shop for
    // rationale.
    let affected = sqlx::query(sql_queries::PRODUCT_UPDATE)
        .bind(&request.product_name)
        .bind(&request.manufacturer_id)
        .bind(&request.memo)
        .bind(request.display_order)
        .bind(request.is_disabled)
        .bind(user_id)
        .bind(product_id)
        .execute(pool)
        .await?
        .rows_affected();
    master_data::ensure_update_affected_one(&SPEC, affected)?;

    Ok("Product updated successfully".to_string())
}

/// Delete a product. Rejected with `ApiError::in_use("Product")` when any
/// transaction detail — from an active or disabled transaction — still
/// names this product (see `sql_queries::PRODUCT_CHECK_IN_USE`); otherwise
/// the row is removed. Hiding a product that is still in use is what
/// `IS_DISABLED` is for (latent-audit M7).
pub async fn delete_product(
    pool: &SqlitePool,
    user_id: i64,
    product_id: i64,
) -> Result<String, ApiError> {
    // Fable-5 review #14 — same TOCTOU-window closure as
    // `delete_shop` / `delete_manufacturer`. Check + delete now
    // run inside one transaction so the reference-count guard
    // and the delete can never be separated by a
    // concurrent transaction-detail insert.
    let mut tx = pool.begin().await?;
    let (in_use,): (i64,) = sqlx::query_as(sql_queries::PRODUCT_CHECK_IN_USE)
        .bind(user_id)
        .bind(product_id)
        .fetch_one(&mut *tx)
        .await?;
    master_data::reject_if_in_use(SPEC.entity_label, in_use)?;
    master_data::run_delete_expect_one_in_tx(&SPEC, &mut tx, user_id, product_id).await?;
    tx.commit().await?;
    Ok("Product deleted successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts;
    use crate::test_helpers::database::{init_db, TEST_DB_URL};
    use crate::services::manufacturer::{add_manufacturer, AddManufacturerRequest};

    async fn setup_test_db() -> SqlitePool {
        let pool = init_db(TEST_DB_URL).await.unwrap();

        // Create USERS table
        sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Create MANUFACTURERS table
        sqlx::query(sql_queries::TEST_MANUFACTURER_CREATE_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Create PRODUCTS table
        sqlx::query(sql_queries::TEST_PRODUCT_CREATE_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // TRANSACTIONS_HEADER + TRANSACTIONS_DETAIL so PRODUCT_CHECK_IN_USE
        // has tables to read against. `delete_product` runs the guard on
        // every call — including happy paths where the manufacturer test
        // also calls `delete_product` — so both tables must exist in
        // every product-service test.
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(sql_queries::TEST_CREATE_TRANSACTIONS_DETAIL_WITH_CATEGORY)
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

        pool
    }

    #[tokio::test]
    async fn test_add_product_without_manufacturer() {
        let pool = setup_test_db().await;

        let request = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: Some("テストメモ".to_string()),
            is_disabled: None,
        };

        let result = add_product(&pool, 2, request).await;
        assert!(result.is_ok());

        let products = get_products(&pool, 2, false).await.unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].product_name, "サバ缶");
        assert_eq!(products[0].manufacturer_id, None);
    }

    #[tokio::test]
    async fn test_add_product_with_manufacturer() {
        let pool = setup_test_db().await;

        // Add manufacturer first
        let manufacturer_request = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, manufacturer_request).await.unwrap();

        let manufacturers = crate::services::manufacturer::get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        // Add product
        let request = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: Some(manufacturer_id),
            memo: Some("テストメモ".to_string()),
            is_disabled: None,
        };

        let result = add_product(&pool, 2, request).await;
        assert!(result.is_ok());

        let products = get_products(&pool, 2, false).await.unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].product_name, "サバ缶");
        assert_eq!(products[0].manufacturer_id, Some(manufacturer_id));
        assert_eq!(products[0].manufacturer_name, Some("ニッスイ".to_string()));
    }

    #[tokio::test]
    async fn test_update_product() {
        let pool = setup_test_db().await;

        // Add product first
        let add_request = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        };
        add_product(&pool, 2, add_request).await.unwrap();

        let products = get_products(&pool, 2, false).await.unwrap();
        let product_id = products[0].product_id;

        // Update product
        let update_request = UpdateProductRequest {
            product_name: "サバの水煮缶".to_string(),
            manufacturer_id: None,
            memo: Some("更新後メモ".to_string()),
            display_order: 1,
            is_disabled: 0,
        };

        let result = update_product(&pool, 2, product_id, update_request).await;
        assert!(result.is_ok());

        let product = get_product_by_id(&pool, 2, product_id).await.unwrap().unwrap();
        assert_eq!(product.product_name, "サバの水煮缶");
        assert_eq!(product.memo, Some("更新後メモ".to_string()));
    }

    #[tokio::test]
    async fn test_delete_product() {
        let pool = setup_test_db().await;

        // Add product first
        let request = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        };
        add_product(&pool, 2, request).await.unwrap();

        let products = get_products(&pool, 2, false).await.unwrap();
        let product_id = products[0].product_id;

        // Delete product
        let result = delete_product(&pool, 2, product_id).await;
        assert!(result.is_ok());

        // An unused product is removed, not just hidden (latent-audit M7).
        let products = get_products(&pool, 2, true).await.unwrap();
        assert_eq!(products.len(), 0);
    }

    /// Latent-audit M7: a disabled product that nothing uses can still be
    /// deleted, which removes the row.
    #[tokio::test]
    async fn test_delete_disabled_product_removes_row() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: Some(1),
        })
        .await
        .unwrap();
        let product_id = get_products(&pool, 2, true).await.unwrap()[0].product_id;

        delete_product(&pool, 2, product_id).await.unwrap();

        assert!(get_products(&pool, 2, true).await.unwrap().is_empty());
    }

    /// Latent-audit M7: a product still named by a transaction detail cannot
    /// be deleted, but it can be disabled.
    #[tokio::test]
    async fn test_disable_product_allowed_while_referenced() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        })
        .await
        .unwrap();
        let product_id = get_products(&pool, 2, false).await.unwrap()[0].product_id;
        let transaction_id = sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_USER_ONLY)
            .bind(2_i64)
            .execute(&pool)
            .await
            .unwrap()
            .last_insert_rowid();
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_DETAIL_PRODUCT_REF)
            .bind(transaction_id)
            .bind(product_id)
            .execute(&pool)
            .await
            .unwrap();

        update_product(&pool, 2, product_id, UpdateProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: None,
            display_order: 1,
            is_disabled: 1,
        })
        .await
        .unwrap();

        assert!(get_products(&pool, 2, false).await.unwrap().is_empty());
        let all = get_products(&pool, 2, true).await.unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].is_disabled, 1);
    }

    #[tokio::test]
    async fn test_empty_product_name() {
        let pool = setup_test_db().await;

        let request = AddProductRequest {
            product_name: "   ".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        };

        let err = add_product(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_add_duplicate_product_returns_duplicate_name_code() {
        let pool = setup_test_db().await;

        // Add first product
        let request1 = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        };
        let result1 = add_product(&pool, 2, request1).await;
        assert!(result1.is_ok());

        // Try to add duplicate product
        let request2 = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: Some("異なるメモ".to_string()),
            is_disabled: None,
        };
        let err = add_product(&pool, 2, request2).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME);
        assert_eq!(err.entity.as_deref(), Some("product"));
    }

    #[tokio::test]
    async fn test_delete_product_rejected_when_referenced_by_transaction_detail() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        })
        .await
        .unwrap();
        let product_id = get_products(&pool, 2, false).await.unwrap()[0].product_id;

        // Insert a transaction owned by user 2 with a detail line naming
        // the product. Scoping runs through TRANSACTIONS_HEADER because
        // TRANSACTIONS_DETAIL has no USER_ID of its own.
        let transaction_id = sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_USER_ONLY)
            .bind(2_i64)
            .execute(&pool)
            .await
            .unwrap()
            .last_insert_rowid();
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_DETAIL_PRODUCT_REF)
            .bind(transaction_id)
            .bind(product_id)
            .execute(&pool)
            .await
            .unwrap();

        let err = delete_product(&pool, 2, product_id).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
        assert_eq!(err.entity.as_deref(), Some("product"));

        assert_eq!(get_products(&pool, 2, false).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_delete_product_ignores_other_users_transaction_details() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        })
        .await
        .unwrap();
        let product_id = get_products(&pool, 2, false).await.unwrap()[0].product_id;

        // Reference owned by user 1 — user 2's delete must still succeed.
        let transaction_id = sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_USER_ONLY)
            .bind(1_i64)
            .execute(&pool)
            .await
            .unwrap()
            .last_insert_rowid();
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_DETAIL_PRODUCT_REF)
            .bind(transaction_id)
            .bind(product_id)
            .execute(&pool)
            .await
            .unwrap();

        assert!(delete_product(&pool, 2, product_id).await.is_ok());
    }

    #[tokio::test]
    async fn test_manufacturer_deletion_rejected_while_product_references_it() {
        // Renamed from `test_manufacturer_deletion_sets_product_manufacturer_to_null`
        // when the master-delete-lock landed: the old test asserted the
        // fallback UX (logical delete succeeds, JOIN scrubs the name off
        // the list) — the new guard blocks the delete outright so the
        // user is steered to disable the manufacturer without orphaning
        // the FK on the product.
        let pool = setup_test_db().await;

        let manufacturer_request = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, manufacturer_request).await.unwrap();

        let manufacturers = crate::services::manufacturer::get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        let product_request = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: Some(manufacturer_id),
            memo: None,
            is_disabled: None,
        };
        add_product(&pool, 2, product_request).await.unwrap();

        let err = crate::services::manufacturer::delete_manufacturer(&pool, 2, manufacturer_id).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
        assert_eq!(err.entity.as_deref(), Some("manufacturer"));

        // Manufacturer still active — the guard aborted before the delete.
        let manufacturers = crate::services::manufacturer::get_manufacturers(&pool, 2, false).await.unwrap();
        assert_eq!(manufacturers.len(), 1);
        // And the product's manufacturer link is preserved.
        let products = get_products(&pool, 2, false).await.unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(products[0].manufacturer_id, Some(manufacturer_id));
    }

    // Issue #37 Phase 2-3 — bounded-field length checks must count
    // characters (not bytes). Japanese is 3 bytes per char in UTF-8.

    #[tokio::test]
    async fn test_add_product_accepts_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let request = AddProductRequest {
            product_name: "あ".repeat(consts::MAX_NAME_LEN),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        };
        let result = add_product(&pool, 2, request).await;
        assert!(result.is_ok(), "expected MAX_NAME_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_add_product_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let request = AddProductRequest {
            product_name: "あ".repeat(consts::MAX_NAME_LEN + 1),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        };
        let err = add_product(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_add_product_accepts_max_chars_of_multibyte_memo() {
        let pool = setup_test_db().await;

        let request = AddProductRequest {
            product_name: "商品".to_string(),
            manufacturer_id: None,
            memo: Some("メ".repeat(consts::MAX_MEMO_LEN)),
            is_disabled: None,
        };
        let result = add_product(&pool, 2, request).await;
        assert!(result.is_ok(), "expected MAX_MEMO_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_add_product_rejects_over_max_chars_of_multibyte_memo() {
        let pool = setup_test_db().await;

        let request = AddProductRequest {
            product_name: "商品".to_string(),
            manufacturer_id: None,
            memo: Some("メ".repeat(consts::MAX_MEMO_LEN + 1)),
            is_disabled: None,
        };
        let err = add_product(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_update_product_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let add_request = AddProductRequest {
            product_name: "商品".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        };
        add_product(&pool, 2, add_request).await.unwrap();
        let products = get_products(&pool, 2, false).await.unwrap();
        let product_id = products[0].product_id;

        let update_request = UpdateProductRequest {
            product_name: "あ".repeat(consts::MAX_NAME_LEN + 1),
            manufacturer_id: None,
            memo: None,
            display_order: 1,
            is_disabled: 0,
        };
        let err = update_product(&pool, 2, product_id, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_update_product_rejects_over_max_chars_of_multibyte_memo() {
        let pool = setup_test_db().await;

        let add_request = AddProductRequest {
            product_name: "商品".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        };
        add_product(&pool, 2, add_request).await.unwrap();
        let products = get_products(&pool, 2, false).await.unwrap();
        let product_id = products[0].product_id;

        let update_request = UpdateProductRequest {
            product_name: "商品".to_string(),
            manufacturer_id: None,
            memo: Some("メ".repeat(consts::MAX_MEMO_LEN + 1)),
            display_order: 1,
            is_disabled: 0,
        };
        let err = update_product(&pool, 2, product_id, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    // v2.6.0 autocomplete: search_products_by_name

    #[tokio::test]
    async fn test_search_products_substring_match() {
        let pool = setup_test_db().await;

        for name in ["セブンイレブン", "セブン店", "ファミリーマート"] {
            add_product(&pool, 2, AddProductRequest {
                product_name: name.to_string(),
                manufacturer_id: None,
                memo: None,
                is_disabled: None,
            }).await.unwrap();
        }

        let hits = search_products_by_name(&pool, 2, "セブン").await.unwrap();
        let names: Vec<String> = hits.iter().map(|p| p.product_name.clone()).collect();
        assert_eq!(hits.len(), 2);
        assert!(names.iter().any(|n| n == "セブンイレブン"));
        assert!(names.iter().any(|n| n == "セブン店"));
    }

    #[tokio::test]
    async fn test_search_products_excludes_other_users() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "私の商品".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        }).await.unwrap();

        // user_id=1 (admin) should not see user_id=2's products
        let hits = search_products_by_name(&pool, 1, "商品").await.unwrap();
        assert!(hits.is_empty());
    }

    #[tokio::test]
    async fn test_search_products_excludes_disabled() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "廃番商品".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        }).await.unwrap();
        let products = get_products(&pool, 2, false).await.unwrap();
        delete_product(&pool, 2, products[0].product_id).await.unwrap();

        let hits = search_products_by_name(&pool, 2, "廃番").await.unwrap();
        assert!(hits.is_empty());
    }

    #[tokio::test]
    async fn test_search_products_empty_query_lists_products() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "anything".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        }).await.unwrap();

        // The item-name field shows suggestions as soon as it gets focus, so
        // an empty query lists products (up to the suggestion limit).
        for query in ["", "   "] {
            let hits = search_products_by_name(&pool, 2, query).await.unwrap();
            assert_eq!(hits.len(), 1, "query {:?}", query);
            assert_eq!(hits[0].product_name, "anything");
        }
    }

    #[tokio::test]
    async fn test_search_products_includes_manufacturer_name() {
        let pool = setup_test_db().await;

        add_manufacturer(&pool, 2, AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        }).await.unwrap();
        let manufacturers = crate::services::manufacturer::get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        add_product(&pool, 2, AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: Some(manufacturer_id),
            memo: None,
            is_disabled: None,
        }).await.unwrap();

        let hits = search_products_by_name(&pool, 2, "サバ").await.unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].manufacturer_name.as_deref(), Some("ニッスイ"));
    }

    /// Fable-5 review #23 pin — the pre-fix `format!("%{}%", trimmed)`
    /// passed `%` and `_` through as LIKE wildcards, so "100%ジ"
    /// matched every "100…ジ" product, not just the one whose name
    /// literally contained "100%ジ". Escape the query and pair with
    /// `LIKE ? ESCAPE '\'` so metacharacters match literally.
    #[tokio::test]
    async fn test_search_products_escapes_percent_metacharacter() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "果汁100%ジュース".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        }).await.unwrap();
        add_product(&pool, 2, AddProductRequest {
            product_name: "果汁100リンゴジュース".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        }).await.unwrap();

        // Pre-fix: `LIKE '%100%ジ%'` matched both products because the
        // interior `%` was a wildcard. Post-fix: `LIKE '%100\%ジ%'
        // ESCAPE '\'` treats `\%` as a literal percent and only the
        // first product matches.
        let hits = search_products_by_name(&pool, 2, "100%ジ").await.unwrap();
        assert_eq!(hits.len(), 1, "only 果汁100%ジュース must match: {:?}",
                   hits.iter().map(|p| p.product_name.as_str()).collect::<Vec<_>>());
        assert_eq!(hits[0].product_name, "果汁100%ジュース");
    }

    #[tokio::test]
    async fn test_search_products_escapes_underscore_metacharacter() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "A_1".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        }).await.unwrap();
        add_product(&pool, 2, AddProductRequest {
            product_name: "AB1".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        }).await.unwrap();

        // Pre-fix: `LIKE '%A_1%'` matched both (underscore = any
        // single char). Post-fix: only the literal-underscore product.
        let hits = search_products_by_name(&pool, 2, "A_1").await.unwrap();
        assert_eq!(hits.len(), 1, "only A_1 must match: {:?}",
                   hits.iter().map(|p| p.product_name.as_str()).collect::<Vec<_>>());
        assert_eq!(hits[0].product_name, "A_1");
    }

    // Product suggestions for the detail item-name field: products used in
    // details of the selected category come first, then other used products,
    // both by most recent use; never-used products follow by name.

    async fn add_named_product(pool: &SqlitePool, name: &str) -> i64 {
        add_product(pool, 2, AddProductRequest {
            product_name: name.to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        }).await.unwrap();
        get_products(pool, 2, true).await.unwrap()
            .into_iter()
            .find(|p| p.product_name == name)
            .unwrap()
            .product_id
    }

    async fn use_product(pool: &SqlitePool, product_id: i64, category: (&str, &str, &str), date: &str) {
        add_detail(pool, Some(product_id), "item", category, date).await;
    }

    /// A detail typed by hand (no product link) under `item_name`.
    async fn use_free_text(pool: &SqlitePool, item_name: &str, category: (&str, &str, &str), date: &str) {
        add_detail(pool, None, item_name, category, date).await;
    }

    async fn add_detail(
        pool: &SqlitePool,
        product_id: Option<i64>,
        item_name: &str,
        category: (&str, &str, &str),
        date: &str,
    ) {
        let (c1, c2, c3) = category;
        let transaction_id = sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_DATED)
            .bind(2_i64)
            .bind(c1)
            .bind(date)
            .execute(pool)
            .await
            .unwrap()
            .last_insert_rowid();
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_DETAIL_WITH_PRODUCT)
            .bind(transaction_id)
            .bind(c2)
            .bind(c3)
            .bind(item_name)
            .bind(product_id)
            .execute(pool)
            .await
            .unwrap();
    }

    fn category(c1: &str, c2: &str, c3: Option<&str>) -> SuggestCategory {
        SuggestCategory {
            category1_code: Some(c1.to_string()),
            category2_code: Some(c2.to_string()),
            category3_code: c3.map(|c| c.to_string()),
        }
    }

    fn names(products: &[Product]) -> Vec<&str> {
        products.iter().map(|p| p.product_name.as_str()).collect()
    }

    const FOOD_SNACK: (&str, &str, &str) = ("EXPENSE", "C2_FOOD", "C3_SNACK");
    const FOOD_DRINK: (&str, &str, &str) = ("EXPENSE", "C2_FOOD", "C3_DRINK");
    const DAILY_SOAP: (&str, &str, &str) = ("EXPENSE", "C2_DAILY", "C3_SOAP");

    /// A: snack 01-10, B: soap 03-01, D: snack 02-01, C: never used.
    async fn seed_four_products(pool: &SqlitePool) {
        let a = add_named_product(pool, "Apple chips").await;
        let b = add_named_product(pool, "Bar soap").await;
        add_named_product(pool, "Cookies").await;
        let d = add_named_product(pool, "Dried mango").await;
        use_product(pool, a, FOOD_SNACK, "2026-01-10").await;
        use_product(pool, b, DAILY_SOAP, "2026-03-01").await;
        use_product(pool, d, FOOD_SNACK, "2026-02-01").await;
    }

    #[tokio::test]
    async fn test_suggest_products_ranks_selected_category_then_used_then_unused() {
        let pool = setup_test_db().await;
        seed_four_products(&pool).await;

        let hits = suggest_products(&pool, 2, "", &category("EXPENSE", "C2_FOOD", Some("C3_SNACK")))
            .await
            .unwrap();
        assert_eq!(names(&hits), ["Dried mango", "Apple chips", "Bar soap", "Cookies"]);
    }

    #[tokio::test]
    async fn test_suggest_products_without_category_ranks_by_recent_use() {
        let pool = setup_test_db().await;
        seed_four_products(&pool).await;

        let hits = suggest_products(&pool, 2, "", &SuggestCategory::default()).await.unwrap();
        assert_eq!(names(&hits), ["Bar soap", "Dried mango", "Apple chips", "Cookies"]);
    }

    #[tokio::test]
    async fn test_suggest_products_category3_must_match_when_given() {
        let pool = setup_test_db().await;
        let snack = add_named_product(&pool, "Snack").await;
        let drink = add_named_product(&pool, "Drink").await;
        use_product(&pool, snack, FOOD_SNACK, "2026-01-01").await;
        use_product(&pool, drink, FOOD_DRINK, "2026-05-01").await;

        // Medium and minor category selected: only the snack is in tier 1.
        let hits = suggest_products(&pool, 2, "", &category("EXPENSE", "C2_FOOD", Some("C3_SNACK")))
            .await
            .unwrap();
        assert_eq!(names(&hits), ["Snack", "Drink"]);

        // Only the medium category selected: both match, most recent first.
        let hits = suggest_products(&pool, 2, "", &category("EXPENSE", "C2_FOOD", None))
            .await
            .unwrap();
        assert_eq!(names(&hits), ["Drink", "Snack"]);
    }

    #[tokio::test]
    async fn test_suggest_products_uses_the_latest_use_of_each_product() {
        let pool = setup_test_db().await;
        let tea = add_named_product(&pool, "Tea").await;
        let milk = add_named_product(&pool, "Milk").await;
        use_product(&pool, tea, FOOD_DRINK, "2026-01-01").await;
        use_product(&pool, milk, FOOD_DRINK, "2026-02-01").await;
        use_product(&pool, tea, FOOD_DRINK, "2026-03-01").await;

        let hits = suggest_products(&pool, 2, "", &SuggestCategory::default()).await.unwrap();
        assert_eq!(names(&hits), ["Tea", "Milk"]);
    }

    #[tokio::test]
    async fn test_suggest_products_query_filters_and_keeps_the_ranking() {
        let pool = setup_test_db().await;
        seed_four_products(&pool).await;

        // "o" matches Bar soap, Cookies and Dried mango, not Apple chips.
        let hits = suggest_products(&pool, 2, "o", &category("EXPENSE", "C2_FOOD", Some("C3_SNACK")))
            .await
            .unwrap();
        assert_eq!(names(&hits), ["Dried mango", "Bar soap", "Cookies"]);
    }

    #[tokio::test]
    async fn test_suggest_products_counts_typed_details_with_the_same_name() {
        let pool = setup_test_db().await;
        add_named_product(&pool, "Apple chips").await;
        add_named_product(&pool, "Bar soap").await;
        add_named_product(&pool, "Cookies").await;
        // Details typed by hand before the product master was used: only an
        // exact name match counts as a use of that product.
        use_free_text(&pool, "Cookies", FOOD_SNACK, "2026-02-01").await;
        use_free_text(&pool, "Bar soap", DAILY_SOAP, "2026-03-01").await;
        use_free_text(&pool, "Apple chips and nuts", FOOD_SNACK, "2026-04-01").await;

        let hits = suggest_products(&pool, 2, "", &category("EXPENSE", "C2_FOOD", Some("C3_SNACK")))
            .await
            .unwrap();
        assert_eq!(names(&hits), ["Cookies", "Bar soap", "Apple chips"]);
    }

    #[tokio::test]
    async fn test_suggest_products_typed_detail_linked_elsewhere_is_not_counted_twice() {
        let pool = setup_test_db().await;
        let milk = add_named_product(&pool, "Milk").await;
        add_named_product(&pool, "Oat milk").await;
        // A detail linked to Milk but typed as "Oat milk" is a use of Milk only.
        add_detail(&pool, Some(milk), "Oat milk", FOOD_DRINK, "2026-05-01").await;

        let hits = suggest_products(&pool, 2, "", &SuggestCategory::default()).await.unwrap();
        assert_eq!(names(&hits), ["Milk", "Oat milk"]);
    }

    #[tokio::test]
    async fn test_suggest_products_returns_at_most_20() {
        let pool = setup_test_db().await;
        for i in 0..25 {
            add_named_product(&pool, &format!("Item {:02}", i)).await;
        }

        let hits = suggest_products(&pool, 2, "", &SuggestCategory::default()).await.unwrap();
        assert_eq!(hits.len(), 20);
        assert_eq!(hits[0].product_name, "Item 00");
    }

    /// Helper: seed a second general user (USER_ID = 3) alongside the
    /// standard admin (1) and general (2) already inserted by
    /// `setup_test_db`. Cross-owner tests need two general users to
    /// exercise the ownership boundary.
    async fn seed_second_general_user(pool: &SqlitePool) {
        sqlx::query(
            "INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) \
             VALUES (3, 'testuser2', 'dummy', 1, datetime('now'))",
        )
        .execute(pool)
        .await
        .unwrap();
    }

    /// Fable-5 review #13 — `add_product` used to bind whatever
    /// `manufacturer_id` the request supplied straight into
    /// PRODUCT_INSERT. The FK on PRODUCTS.MANUFACTURER_ID only enforces
    /// "some manufacturer row exists", not ownership, so a direct
    /// `invoke` from user B could attach one of B's products to a
    /// manufacturer_id that belongs to user A. This test locks in the
    /// new ownership check.
    #[tokio::test]
    async fn test_add_product_rejects_foreign_manufacturer_id() {
        let pool = setup_test_db().await;
        seed_second_general_user(&pool).await;

        // User 2 registers a manufacturer of theirs.
        add_manufacturer(
            &pool,
            2,
            AddManufacturerRequest {
                manufacturer_name: "ニッスイ".to_string(),
                memo: None,
                is_disabled: None,
            },
        )
        .await
        .unwrap();
        let user2_manufacturer_id =
            crate::services::manufacturer::get_manufacturers(&pool, 2, false)
                .await
                .unwrap()[0]
                .manufacturer_id;

        // User 3 tries to attach that manufacturer_id to a new product —
        // must fail before the INSERT touches PRODUCTS.
        let attempt = add_product(
            &pool,
            3,
            AddProductRequest {
                product_name: "サバ缶".to_string(),
                manufacturer_id: Some(user2_manufacturer_id),
                memo: None,
                is_disabled: None,
            },
        )
        .await;

        assert!(
            attempt.as_ref().is_err_and(|e| e.code == ApiError::CODE_MANUFACTURER_NOT_FOUND),
            "cross-owner manufacturer_id must be rejected: {:?}",
            attempt
        );

        // User 3 should have zero products (no leakage from the failed
        // request into PRODUCTS).
        assert!(get_products(&pool, 3, false).await.unwrap().is_empty());
    }

    /// Nonexistent manufacturer_id must also be rejected, even for the
    /// legitimate owner. Prevents the frontend from persisting a stale
    /// reference to a manufacturer the user just deleted.
    #[tokio::test]
    async fn test_add_product_rejects_nonexistent_manufacturer_id() {
        let pool = setup_test_db().await;

        let attempt = add_product(
            &pool,
            2,
            AddProductRequest {
                product_name: "サバ缶".to_string(),
                manufacturer_id: Some(999_999),
                memo: None,
                is_disabled: None,
            },
        )
        .await;

        assert!(
            attempt.as_ref().is_err_and(|e| e.code == ApiError::CODE_MANUFACTURER_NOT_FOUND),
            "nonexistent manufacturer_id must be rejected: {:?}",
            attempt
        );
    }

    /// Same guard on the update path: reassigning a product to another
    /// user's manufacturer must be rejected before the UPDATE runs.
    #[tokio::test]
    async fn test_update_product_rejects_foreign_manufacturer_id() {
        let pool = setup_test_db().await;
        seed_second_general_user(&pool).await;

        // Owner (user 2) creates a product without a manufacturer.
        add_product(
            &pool,
            2,
            AddProductRequest {
                product_name: "サバ缶".to_string(),
                manufacturer_id: None,
                memo: None,
                is_disabled: None,
            },
        )
        .await
        .unwrap();
        let product_id = get_products(&pool, 2, false).await.unwrap()[0].product_id;

        // User 3 registers a manufacturer of theirs.
        add_manufacturer(
            &pool,
            3,
            AddManufacturerRequest {
                manufacturer_name: "他ユーザーメーカー".to_string(),
                memo: None,
                is_disabled: None,
            },
        )
        .await
        .unwrap();
        let user3_manufacturer_id =
            crate::services::manufacturer::get_manufacturers(&pool, 3, false)
                .await
                .unwrap()[0]
                .manufacturer_id;

        // Owner tries to reassign their product to user 3's manufacturer.
        let attempt = update_product(
            &pool,
            2,
            product_id,
            UpdateProductRequest {
                product_name: "サバ缶".to_string(),
                manufacturer_id: Some(user3_manufacturer_id),
                memo: None,
                display_order: 1,
                is_disabled: 0,
            },
        )
        .await;

        assert!(
            attempt.as_ref().is_err_and(|e| e.code == ApiError::CODE_MANUFACTURER_NOT_FOUND),
            "cross-owner manufacturer_id must be rejected on update: {:?}",
            attempt
        );

        // The product row must be unchanged: still no manufacturer.
        let after = &get_products(&pool, 2, false).await.unwrap()[0];
        assert_eq!(after.manufacturer_id, None);
    }

    /// Defense-in-depth for the JOIN scope in
    /// PRODUCT_GET_ALL / _BY_ID / _INCLUDING_DISABLED / _SEARCH_BY_NAME.
    /// MANUFACTURERS.MANUFACTURER_ID is globally unique (AUTOINCREMENT
    /// PK), so a `p.MANUFACTURER_ID = m.MANUFACTURER_ID` join can only
    /// ever hit one row; the leak Fable-5 review #13 warns about is not
    /// a same-id collision but the case where a PRODUCTS row already
    /// references a manufacturer from another user (either from the
    /// pre-fix `add_product` gap or hand-crafted DB corruption). Without
    /// `m.USER_ID = p.USER_ID` on the JOIN, that other user's
    /// manufacturer name would show up on this user's product row.
    ///
    /// This test simulates that corrupt state by hand-inserting the
    /// cross-owner row into PRODUCTS (bypassing our newly-added
    /// verification), then confirms the JOIN returns NULL for
    /// `manufacturer_name` instead of leaking user 3's name to user 2.
    #[tokio::test]
    async fn test_product_join_scopes_manufacturer_by_user_id() {
        let pool = setup_test_db().await;
        seed_second_general_user(&pool).await;

        // User 3's manufacturer.
        add_manufacturer(
            &pool,
            3,
            AddManufacturerRequest {
                manufacturer_name: "他ユーザーメーカー".to_string(),
                memo: None,
                is_disabled: None,
            },
        )
        .await
        .unwrap();
        let user3_manufacturer_id =
            crate::services::manufacturer::get_manufacturers(&pool, 3, false)
                .await
                .unwrap()[0]
                .manufacturer_id;

        // Hand-insert a PRODUCT for user 2 that (incorrectly) references
        // user 3's manufacturer_id. This bypasses our new add-time check
        // to simulate legacy corruption / DB-level tampering.
        sqlx::query(
            "INSERT INTO PRODUCTS (USER_ID, PRODUCT_NAME, MANUFACTURER_ID, \
              MEMO, DISPLAY_ORDER, IS_DISABLED, ENTRY_DT) \
             VALUES (2, 'サバ缶', ?, NULL, 1, 0, datetime('now'))",
        )
        .bind(user3_manufacturer_id)
        .execute(&pool)
        .await
        .unwrap();

        // PRODUCT_GET_ALL scoping fix: user 2's product row must not
        // show user 3's manufacturer name.
        let products = get_products(&pool, 2, false).await.unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(
            products[0].manufacturer_name, None,
            "JOIN must NOT leak user 3's manufacturer name to user 2 \
             (got Some({:?}))",
            products[0].manufacturer_name,
        );

        // Same guard on PRODUCT_GET_BY_ID.
        let single = get_product_by_id(&pool, 2, products[0].product_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(single.manufacturer_name, None);

        // PRODUCT_SEARCH_BY_NAME goes through the same JOIN.
        let hits = search_products_by_name(&pool, 2, "サバ").await.unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].manufacturer_name, None);
    }
}

#[cfg(test)]
#[path = "latent_audit/product.rs"]
mod latent_audit;
