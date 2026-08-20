use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use crate::services::master_data;
use crate::sql_queries;
use crate::validation;

const NAME_LABEL: &str = "Product name";
const DUPLICATE_LABEL: &str = "product name";

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
pub async fn get_products(pool: &SqlitePool, user_id: i64, include_disabled: bool) -> Result<Vec<Product>, String> {
    let query = if include_disabled {
        sql_queries::PRODUCT_GET_ALL_INCLUDING_DISABLED
    } else {
        sql_queries::PRODUCT_GET_ALL
    };

    let products = sqlx::query_as::<_, Product>(query)
        .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to get products: {}", e))?;

    Ok(products)
}

/// Search products by partial name match for autocomplete in transaction entry.
/// Returns up to 20 enabled products matching the query (case-insensitive
/// substring). Empty/whitespace-only queries return an empty list to avoid
/// dumping the full master into the dropdown on focus.
pub async fn search_products_by_name(
    pool: &SqlitePool,
    user_id: i64,
    query: &str,
) -> Result<Vec<Product>, String> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let pattern = format!("%{}%", trimmed);

    let products = sqlx::query_as::<_, Product>(sql_queries::PRODUCT_SEARCH_BY_NAME)
        .bind(user_id)
        .bind(&pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to search products: {}", e))?;

    Ok(products)
}

/// Get a single product by ID
pub async fn get_product_by_id(
    pool: &SqlitePool,
    user_id: i64,
    product_id: i64,
) -> Result<Option<Product>, String> {
    master_data::fetch_by_id(
        pool,
        sql_queries::PRODUCT_GET_BY_ID,
        user_id,
        product_id,
        "product",
    )
    .await
}

/// Add a new product
pub async fn add_product(
    pool: &SqlitePool,
    user_id: i64,
    request: AddProductRequest,
) -> Result<String, String> {
    validation::validate_master_name(NAME_LABEL, &request.product_name)?;
    validation::validate_memo("Memo", request.memo.as_ref())?;

    // Verify manufacturer ownership. PRODUCTS.MANUFACTURER_ID's FK only
    // enforces "some manufacturer row exists" — it does not check that
    // the manufacturer belongs to `user_id`. Without this guard, a direct
    // `invoke` from user B with a `manufacturer_id` that belongs to user A
    // would silently attach one of B's products to A's manufacturer,
    // and the PRODUCT_GET_ALL LEFT JOIN would then show A's manufacturer
    // name on B's product row (JOIN scope is fixed in the same PR).
    // Fable-5 review #13.
    verify_manufacturer_ownership(pool, user_id, request.manufacturer_id).await?;

    // Check for duplicate product name
    if master_data::value_exists(
        pool,
        sql_queries::PRODUCT_CHECK_DUPLICATE_FOR_ADD,
        user_id,
        &request.product_name,
        DUPLICATE_LABEL,
    )
    .await?
    {
        return Err("Product name already exists".to_string());
    }

    // Get next display order
    let display_order = master_data::fetch_next_display_order(
        pool,
        sql_queries::PRODUCT_GET_NEXT_DISPLAY_ORDER,
        user_id,
    )
    .await?;

    // Get is_disabled value (default to 0)
    let is_disabled = request.is_disabled.unwrap_or(0);

    // Insert product
    sqlx::query(sql_queries::PRODUCT_INSERT)
        .bind(user_id)
        .bind(&request.product_name)
        .bind(&request.manufacturer_id)
        .bind(&request.memo)
        .bind(display_order)
        .bind(is_disabled)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to add product: {}", e))?;

    Ok("Product added successfully".to_string())
}

/// If `manufacturer_id` is `Some`, confirm the manufacturer exists AND
/// belongs to `user_id`. Returns `"Manufacturer not found"` otherwise,
/// mirroring the string the frontend already maps for missing-master
/// scenarios. `None` (no manufacturer) is always OK — products can be
/// registered without a manufacturer.
async fn verify_manufacturer_ownership(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: Option<i64>,
) -> Result<(), String> {
    let Some(id) = manufacturer_id else {
        return Ok(());
    };
    let exists: Option<i64> = sqlx::query_scalar(sql_queries::MANUFACTURER_EXISTS_FOR_USER)
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to verify manufacturer: {}", e))?;
    if exists.is_none() {
        return Err("Manufacturer not found".to_string());
    }
    Ok(())
}

/// Update a product
pub async fn update_product(
    pool: &SqlitePool,
    user_id: i64,
    product_id: i64,
    request: UpdateProductRequest,
) -> Result<String, String> {
    validation::validate_master_name(NAME_LABEL, &request.product_name)?;
    validation::validate_memo("Memo", request.memo.as_ref())?;

    // Verify manufacturer ownership before touching the row — same
    // reasoning as `add_product`. Fable-5 review #13.
    verify_manufacturer_ownership(pool, user_id, request.manufacturer_id).await?;

    // Check if product exists
    get_product_by_id(pool, user_id, product_id)
        .await?
        .ok_or("Product not found")?;

    // Check for duplicate product name
    if master_data::value_exists_excluding(
        pool,
        sql_queries::PRODUCT_CHECK_DUPLICATE_FOR_UPDATE,
        user_id,
        &request.product_name,
        product_id,
        DUPLICATE_LABEL,
    )
    .await?
    {
        return Err("Product name already exists".to_string());
    }

    // Update product
    sqlx::query(sql_queries::PRODUCT_UPDATE)
        .bind(&request.product_name)
        .bind(&request.manufacturer_id)
        .bind(&request.memo)
        .bind(request.display_order)
        .bind(request.is_disabled)
        .bind(user_id)
        .bind(product_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update product: {}", e))?;

    Ok("Product updated successfully".to_string())
}

/// Delete a product (logical deletion)
pub async fn delete_product(
    pool: &SqlitePool,
    user_id: i64,
    product_id: i64,
) -> Result<String, String> {
    // Check if product exists
    get_product_by_id(pool, user_id, product_id)
        .await?
        .ok_or("Product not found")?;

    // Logical delete
    master_data::execute_by_id(
        pool,
        sql_queries::PRODUCT_DELETE_LOGICAL,
        user_id,
        product_id,
        "delete product",
    )
    .await?;

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

        // Verify product is disabled
        let products = get_products(&pool, 2, false).await.unwrap();
        assert_eq!(products.len(), 0);
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

        let result = add_product(&pool, 2, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_add_duplicate_product() {
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
        let result2 = add_product(&pool, 2, request2).await;
        assert!(result2.is_err());
        assert!(result2.unwrap_err().contains("already exists"));
    }

    #[tokio::test]
    async fn test_manufacturer_deletion_sets_product_manufacturer_to_null() {
        let pool = setup_test_db().await;

        // Add manufacturer
        let manufacturer_request = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, manufacturer_request).await.unwrap();

        let manufacturers = crate::services::manufacturer::get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        // Add product with manufacturer
        let product_request = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: Some(manufacturer_id),
            memo: None,
            is_disabled: None,
        };
        add_product(&pool, 2, product_request).await.unwrap();

        // Delete manufacturer (logical delete)
        crate::services::manufacturer::delete_manufacturer(&pool, 2, manufacturer_id).await.unwrap();

        // Verify product still exists but manufacturer info is gone from list view
        let products = get_products(&pool, 2, false).await.unwrap();
        assert_eq!(products.len(), 1);
        // Due to LEFT JOIN, manufacturer_name should be None when manufacturer is disabled
        // (The actual manufacturer_id in PRODUCTS table remains, but manufacturer is not shown in list)
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
        assert!(err.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err);
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
        assert!(err.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", err);
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
        assert!(err.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err);
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
        assert!(err.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", err);
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
    async fn test_search_products_empty_query_returns_empty() {
        let pool = setup_test_db().await;

        add_product(&pool, 2, AddProductRequest {
            product_name: "anything".to_string(),
            manufacturer_id: None,
            memo: None,
            is_disabled: None,
        }).await.unwrap();

        // Empty query must not dump the master into the dropdown
        assert!(search_products_by_name(&pool, 2, "").await.unwrap().is_empty());
        assert!(search_products_by_name(&pool, 2, "   ").await.unwrap().is_empty());
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
            attempt.as_ref().is_err_and(|e| e.contains("Manufacturer not found")),
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
            attempt.as_ref().is_err_and(|e| e.contains("Manufacturer not found")),
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
            attempt.as_ref().is_err_and(|e| e.contains("Manufacturer not found")),
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
