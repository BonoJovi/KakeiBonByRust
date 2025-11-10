use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use crate::sql_queries;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Shop {
    pub shop_id: i64,
    pub user_id: i64,
    pub shop_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddShopRequest {
    pub shop_name: String,
    pub memo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateShopRequest {
    pub shop_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
}

/// Get all shops for a user
pub async fn get_shops(pool: &SqlitePool, user_id: i64) -> Result<Vec<Shop>, String> {
    let shops = sqlx::query_as::<_, Shop>(
        sql_queries::SHOP_GET_ALL
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to get shops: {}", e))?;

    Ok(shops)
}

/// Get a single shop by ID
pub async fn get_shop_by_id(
    pool: &SqlitePool,
    user_id: i64,
    shop_id: i64,
) -> Result<Option<Shop>, String> {
    let shop = sqlx::query_as::<_, Shop>(
        sql_queries::SHOP_GET_BY_ID
    )
    .bind(user_id)
    .bind(shop_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to get shop: {}", e))?;

    Ok(shop)
}

/// Get next display order
async fn get_next_display_order(pool: &SqlitePool, user_id: i64) -> Result<i64, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::SHOP_GET_NEXT_DISPLAY_ORDER)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to get next display order: {}", e))?;

    Ok(result.0)
}

/// Check if shop name is duplicate (for add)
async fn check_duplicate_for_add(
    pool: &SqlitePool,
    user_id: i64,
    shop_name: &str,
) -> Result<bool, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::SHOP_CHECK_DUPLICATE_FOR_ADD)
        .bind(user_id)
        .bind(shop_name)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check duplicate shop name: {}", e))?;

    Ok(result.0 > 0)
}

/// Check if shop name is duplicate (for update)
async fn check_duplicate_for_update(
    pool: &SqlitePool,
    user_id: i64,
    shop_name: &str,
    shop_id: i64,
) -> Result<bool, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::SHOP_CHECK_DUPLICATE_FOR_UPDATE)
        .bind(user_id)
        .bind(shop_name)
        .bind(shop_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check duplicate shop name: {}", e))?;

    Ok(result.0 > 0)
}

/// Add a new shop
pub async fn add_shop(
    pool: &SqlitePool,
    user_id: i64,
    request: AddShopRequest,
) -> Result<String, String> {
    // Validate shop name
    if request.shop_name.trim().is_empty() {
        return Err("Shop name cannot be empty".to_string());
    }

    // Check for duplicate shop name
    if check_duplicate_for_add(pool, user_id, &request.shop_name).await? {
        return Err("Shop name already exists".to_string());
    }

    // Get next display order
    let display_order = get_next_display_order(pool, user_id).await?;

    // Insert shop
    sqlx::query(sql_queries::SHOP_INSERT)
        .bind(user_id)
        .bind(&request.shop_name)
        .bind(&request.memo)
        .bind(display_order)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to add shop: {}", e))?;

    Ok("Shop added successfully".to_string())
}

/// Update a shop
pub async fn update_shop(
    pool: &SqlitePool,
    user_id: i64,
    shop_id: i64,
    request: UpdateShopRequest,
) -> Result<String, String> {
    // Validate shop name
    if request.shop_name.trim().is_empty() {
        return Err("Shop name cannot be empty".to_string());
    }

    // Check if shop exists
    get_shop_by_id(pool, user_id, shop_id)
        .await?
        .ok_or("Shop not found")?;

    // Check for duplicate shop name
    if check_duplicate_for_update(pool, user_id, &request.shop_name, shop_id).await? {
        return Err("Shop name already exists".to_string());
    }

    // Update shop
    sqlx::query(sql_queries::SHOP_UPDATE)
        .bind(&request.shop_name)
        .bind(&request.memo)
        .bind(request.display_order)
        .bind(user_id)
        .bind(shop_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update shop: {}", e))?;

    Ok("Shop updated successfully".to_string())
}

/// Delete a shop (logical deletion)
pub async fn delete_shop(
    pool: &SqlitePool,
    user_id: i64,
    shop_id: i64,
) -> Result<String, String> {
    // Check if shop exists
    get_shop_by_id(pool, user_id, shop_id)
        .await?
        .ok_or("Shop not found")?;

    // Logical delete
    sqlx::query(sql_queries::SHOP_DELETE_LOGICAL)
        .bind(user_id)
        .bind(shop_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete shop: {}", e))?;

    Ok("Shop deleted successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::database::{init_db, TEST_DB_URL};

    async fn setup_test_db() -> SqlitePool {
        let pool = init_db(TEST_DB_URL).await.unwrap();

        // Create USERS table
        sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // Create SHOPS table
        sqlx::query(sql_queries::TEST_SHOP_CREATE_TABLE)
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
    async fn test_add_shop() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: Some("テストメモ".to_string()),
        };

        let result = add_shop(&pool, 2, request).await;
        assert!(result.is_ok());

        let shops = get_shops(&pool, 2).await.unwrap();
        assert_eq!(shops.len(), 1);
        assert_eq!(shops[0].shop_name, "イオン新宿店");
    }

    #[tokio::test]
    async fn test_update_shop() {
        let pool = setup_test_db().await;

        // Add shop first
        let add_request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, add_request).await.unwrap();

        let shops = get_shops(&pool, 2).await.unwrap();
        let shop_id = shops[0].shop_id;

        // Update shop
        let update_request = UpdateShopRequest {
            shop_name: "イオン祇園店".to_string(),
            memo: Some("更新後メモ".to_string()),
            display_order: 1,
        };

        let result = update_shop(&pool, 2, shop_id, update_request).await;
        assert!(result.is_ok());

        let shop = get_shop_by_id(&pool, 2, shop_id).await.unwrap().unwrap();
        assert_eq!(shop.shop_name, "イオン祇園店");
        assert_eq!(shop.memo, Some("更新後メモ".to_string()));
    }

    #[tokio::test]
    async fn test_delete_shop() {
        let pool = setup_test_db().await;

        // Add shop first
        let request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, request).await.unwrap();

        let shops = get_shops(&pool, 2).await.unwrap();
        let shop_id = shops[0].shop_id;

        // Delete shop
        let result = delete_shop(&pool, 2, shop_id).await;
        assert!(result.is_ok());

        // Verify shop is disabled
        let shops = get_shops(&pool, 2).await.unwrap();
        assert_eq!(shops.len(), 0);
    }

    #[tokio::test]
    async fn test_empty_shop_name() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "   ".to_string(),
            memo: None,
        };

        let result = add_shop(&pool, 2, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_add_duplicate_shop() {
        let pool = setup_test_db().await;

        // Add first shop
        let request1 = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        let result1 = add_shop(&pool, 2, request1).await;
        assert!(result1.is_ok());

        // Try to add duplicate shop
        let request2 = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: Some("異なるメモ".to_string()),
        };
        let result2 = add_shop(&pool, 2, request2).await;
        assert!(result2.is_err());
        assert!(result2.unwrap_err().contains("already exists"));
    }

    #[tokio::test]
    async fn test_update_to_duplicate_shop_name() {
        let pool = setup_test_db().await;

        // Add two shops
        let request1 = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, request1).await.unwrap();

        let request2 = AddShopRequest {
            shop_name: "セブンイレブン".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, request2).await.unwrap();

        let shops = get_shops(&pool, 2).await.unwrap();
        let shop_id = shops[1].shop_id; // セブンイレブン

        // Try to update to existing name
        let update_request = UpdateShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
            display_order: 1,
        };
        let result = update_shop(&pool, 2, shop_id, update_request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[tokio::test]
    async fn test_update_same_shop_name() {
        let pool = setup_test_db().await;

        // Add shop
        let request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: Some("元のメモ".to_string()),
        };
        add_shop(&pool, 2, request).await.unwrap();

        let shops = get_shops(&pool, 2).await.unwrap();
        let shop_id = shops[0].shop_id;

        // Update with same name (should succeed)
        let update_request = UpdateShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: Some("新しいメモ".to_string()),
            display_order: 1,
        };
        let result = update_shop(&pool, 2, shop_id, update_request).await;
        assert!(result.is_ok());

        // Verify memo was updated
        let shop = get_shop_by_id(&pool, 2, shop_id).await.unwrap().unwrap();
        assert_eq!(shop.memo, Some("新しいメモ".to_string()));
    }
}
