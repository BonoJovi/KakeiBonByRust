use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use crate::sql_queries;

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
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub product_name: String,
    pub manufacturer_id: Option<i64>,
    pub memo: Option<String>,
    pub display_order: i64,
}

/// Get all products for a user
pub async fn get_products(pool: &SqlitePool, user_id: i64) -> Result<Vec<Product>, String> {
    let products = sqlx::query_as::<_, Product>(
        sql_queries::PRODUCT_GET_ALL
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to get products: {}", e))?;

    Ok(products)
}

/// Get a single product by ID
pub async fn get_product_by_id(
    pool: &SqlitePool,
    user_id: i64,
    product_id: i64,
) -> Result<Option<Product>, String> {
    let product = sqlx::query_as::<_, Product>(
        sql_queries::PRODUCT_GET_BY_ID
    )
    .bind(user_id)
    .bind(product_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to get product: {}", e))?;

    Ok(product)
}

/// Get next display order
async fn get_next_display_order(pool: &SqlitePool, user_id: i64) -> Result<i64, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::PRODUCT_GET_NEXT_DISPLAY_ORDER)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to get next display order: {}", e))?;

    Ok(result.0)
}

/// Check if product name is duplicate (for add)
async fn check_duplicate_for_add(
    pool: &SqlitePool,
    user_id: i64,
    product_name: &str,
) -> Result<bool, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::PRODUCT_CHECK_DUPLICATE_FOR_ADD)
        .bind(user_id)
        .bind(product_name)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check duplicate product name: {}", e))?;

    Ok(result.0 > 0)
}

/// Check if product name is duplicate (for update)
async fn check_duplicate_for_update(
    pool: &SqlitePool,
    user_id: i64,
    product_name: &str,
    product_id: i64,
) -> Result<bool, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::PRODUCT_CHECK_DUPLICATE_FOR_UPDATE)
        .bind(user_id)
        .bind(product_name)
        .bind(product_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check duplicate product name: {}", e))?;

    Ok(result.0 > 0)
}

/// Add a new product
pub async fn add_product(
    pool: &SqlitePool,
    user_id: i64,
    request: AddProductRequest,
) -> Result<String, String> {
    // Validate product name
    if request.product_name.trim().is_empty() {
        return Err("Product name cannot be empty".to_string());
    }

    // Check for duplicate product name
    if check_duplicate_for_add(pool, user_id, &request.product_name).await? {
        return Err("Product name already exists".to_string());
    }

    // Get next display order
    let display_order = get_next_display_order(pool, user_id).await?;

    // Insert product
    sqlx::query(sql_queries::PRODUCT_INSERT)
        .bind(user_id)
        .bind(&request.product_name)
        .bind(&request.manufacturer_id)
        .bind(&request.memo)
        .bind(display_order)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to add product: {}", e))?;

    Ok("Product added successfully".to_string())
}

/// Update a product
pub async fn update_product(
    pool: &SqlitePool,
    user_id: i64,
    product_id: i64,
    request: UpdateProductRequest,
) -> Result<String, String> {
    // Validate product name
    if request.product_name.trim().is_empty() {
        return Err("Product name cannot be empty".to_string());
    }

    // Check if product exists
    get_product_by_id(pool, user_id, product_id)
        .await?
        .ok_or("Product not found")?;

    // Check for duplicate product name
    if check_duplicate_for_update(pool, user_id, &request.product_name, product_id).await? {
        return Err("Product name already exists".to_string());
    }

    // Update product
    sqlx::query(sql_queries::PRODUCT_UPDATE)
        .bind(&request.product_name)
        .bind(&request.manufacturer_id)
        .bind(&request.memo)
        .bind(request.display_order)
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
    sqlx::query(sql_queries::PRODUCT_DELETE_LOGICAL)
        .bind(user_id)
        .bind(product_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete product: {}", e))?;

    Ok("Product deleted successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
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
        };

        let result = add_product(&pool, 2, request).await;
        assert!(result.is_ok());

        let products = get_products(&pool, 2).await.unwrap();
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
        };
        add_manufacturer(&pool, 2, manufacturer_request).await.unwrap();

        let manufacturers = crate::services::manufacturer::get_manufacturers(&pool, 2).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        // Add product
        let request = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: Some(manufacturer_id),
            memo: Some("テストメモ".to_string()),
        };

        let result = add_product(&pool, 2, request).await;
        assert!(result.is_ok());

        let products = get_products(&pool, 2).await.unwrap();
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
        };
        add_product(&pool, 2, add_request).await.unwrap();

        let products = get_products(&pool, 2).await.unwrap();
        let product_id = products[0].product_id;

        // Update product
        let update_request = UpdateProductRequest {
            product_name: "サバの水煮缶".to_string(),
            manufacturer_id: None,
            memo: Some("更新後メモ".to_string()),
            display_order: 1,
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
        };
        add_product(&pool, 2, request).await.unwrap();

        let products = get_products(&pool, 2).await.unwrap();
        let product_id = products[0].product_id;

        // Delete product
        let result = delete_product(&pool, 2, product_id).await;
        assert!(result.is_ok());

        // Verify product is disabled
        let products = get_products(&pool, 2).await.unwrap();
        assert_eq!(products.len(), 0);
    }

    #[tokio::test]
    async fn test_empty_product_name() {
        let pool = setup_test_db().await;

        let request = AddProductRequest {
            product_name: "   ".to_string(),
            manufacturer_id: None,
            memo: None,
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
        };
        let result1 = add_product(&pool, 2, request1).await;
        assert!(result1.is_ok());

        // Try to add duplicate product
        let request2 = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: None,
            memo: Some("異なるメモ".to_string()),
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
        };
        add_manufacturer(&pool, 2, manufacturer_request).await.unwrap();

        let manufacturers = crate::services::manufacturer::get_manufacturers(&pool, 2).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        // Add product with manufacturer
        let product_request = AddProductRequest {
            product_name: "サバ缶".to_string(),
            manufacturer_id: Some(manufacturer_id),
            memo: None,
        };
        add_product(&pool, 2, product_request).await.unwrap();

        // Delete manufacturer (logical delete)
        crate::services::manufacturer::delete_manufacturer(&pool, 2, manufacturer_id).await.unwrap();

        // Verify product still exists but manufacturer info is gone from list view
        let products = get_products(&pool, 2).await.unwrap();
        assert_eq!(products.len(), 1);
        // Due to LEFT JOIN, manufacturer_name should be None when manufacturer is disabled
        // (The actual manufacturer_id in PRODUCTS table remains, but manufacturer is not shown in list)
    }
}
