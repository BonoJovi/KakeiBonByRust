use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use crate::sql_queries;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Manufacturer {
    pub manufacturer_id: i64,
    pub user_id: i64,
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AddManufacturerRequest {
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub is_disabled: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateManufacturerRequest {
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
}

/// Get all manufacturers for a user
pub async fn get_manufacturers(pool: &SqlitePool, user_id: i64, include_disabled: bool) -> Result<Vec<Manufacturer>, String> {
    let query = if include_disabled {
        sql_queries::MANUFACTURER_GET_ALL_INCLUDING_DISABLED
    } else {
        sql_queries::MANUFACTURER_GET_ALL
    };

    let manufacturers = sqlx::query_as::<_, Manufacturer>(query)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to get manufacturers: {}", e))?;

    Ok(manufacturers)
}

/// Get a single manufacturer by ID
pub async fn get_manufacturer_by_id(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: i64,
) -> Result<Option<Manufacturer>, String> {
    let manufacturer = sqlx::query_as::<_, Manufacturer>(
        sql_queries::MANUFACTURER_GET_BY_ID
    )
    .bind(user_id)
    .bind(manufacturer_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to get manufacturer: {}", e))?;

    Ok(manufacturer)
}

/// Get next display order
async fn get_next_display_order(pool: &SqlitePool, user_id: i64) -> Result<i64, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::MANUFACTURER_GET_NEXT_DISPLAY_ORDER)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to get next display order: {}", e))?;

    Ok(result.0)
}

/// Check if manufacturer name is duplicate (for add)
async fn check_duplicate_for_add(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_name: &str,
) -> Result<bool, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::MANUFACTURER_CHECK_DUPLICATE_FOR_ADD)
        .bind(user_id)
        .bind(manufacturer_name)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check duplicate manufacturer name: {}", e))?;

    Ok(result.0 > 0)
}

/// Check if manufacturer name is duplicate (for update)
async fn check_duplicate_for_update(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_name: &str,
    manufacturer_id: i64,
) -> Result<bool, String> {
    let result: (i64,) = sqlx::query_as(sql_queries::MANUFACTURER_CHECK_DUPLICATE_FOR_UPDATE)
        .bind(user_id)
        .bind(manufacturer_name)
        .bind(manufacturer_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check duplicate manufacturer name: {}", e))?;

    Ok(result.0 > 0)
}

/// Add a new manufacturer
pub async fn add_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    request: AddManufacturerRequest,
) -> Result<String, String> {
    // Validate manufacturer name
    if request.manufacturer_name.trim().is_empty() {
        return Err("Manufacturer name cannot be empty".to_string());
    }

    // Check for duplicate manufacturer name
    if check_duplicate_for_add(pool, user_id, &request.manufacturer_name).await? {
        return Err("Manufacturer name already exists".to_string());
    }

    // Get next display order
    let display_order = get_next_display_order(pool, user_id).await?;

    // Get is_disabled value (default to 0)
    let is_disabled = request.is_disabled.unwrap_or(0);

    // Insert manufacturer
    sqlx::query(sql_queries::MANUFACTURER_INSERT)
        .bind(user_id)
        .bind(&request.manufacturer_name)
        .bind(&request.memo)
        .bind(display_order)
        .bind(is_disabled)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to add manufacturer: {}", e))?;

    Ok("Manufacturer added successfully".to_string())
}

/// Update a manufacturer
pub async fn update_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: i64,
    request: UpdateManufacturerRequest,
) -> Result<String, String> {
    // Validate manufacturer name
    if request.manufacturer_name.trim().is_empty() {
        return Err("Manufacturer name cannot be empty".to_string());
    }

    // Check if manufacturer exists
    get_manufacturer_by_id(pool, user_id, manufacturer_id)
        .await?
        .ok_or("Manufacturer not found")?;

    // Check for duplicate manufacturer name
    if check_duplicate_for_update(pool, user_id, &request.manufacturer_name, manufacturer_id).await? {
        return Err("Manufacturer name already exists".to_string());
    }

    // Update manufacturer
    sqlx::query(sql_queries::MANUFACTURER_UPDATE)
        .bind(&request.manufacturer_name)
        .bind(&request.memo)
        .bind(request.display_order)
        .bind(request.is_disabled)
        .bind(user_id)
        .bind(manufacturer_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update manufacturer: {}", e))?;

    Ok("Manufacturer updated successfully".to_string())
}

/// Delete a manufacturer (logical deletion)
pub async fn delete_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: i64,
) -> Result<String, String> {
    // Check if manufacturer exists
    get_manufacturer_by_id(pool, user_id, manufacturer_id)
        .await?
        .ok_or("Manufacturer not found")?;

    // Logical delete
    sqlx::query(sql_queries::MANUFACTURER_DELETE_LOGICAL)
        .bind(user_id)
        .bind(manufacturer_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete manufacturer: {}", e))?;

    Ok("Manufacturer deleted successfully".to_string())
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

        // Create MANUFACTURERS table
        sqlx::query(sql_queries::TEST_MANUFACTURER_CREATE_TABLE)
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
    async fn test_add_manufacturer() {
        let pool = setup_test_db().await;

        let request = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: Some("テストメモ".to_string()),
            is_disabled: None,
        };

        let result = add_manufacturer(&pool, 2, request).await;
        assert!(result.is_ok());

        let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
        assert_eq!(manufacturers.len(), 1);
        assert_eq!(manufacturers[0].manufacturer_name, "ニッスイ");
    }

    #[tokio::test]
    async fn test_update_manufacturer() {
        let pool = setup_test_db().await;

        // Add manufacturer first
        let add_request = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, add_request).await.unwrap();

        let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        // Update manufacturer
        let update_request = UpdateManufacturerRequest {
            manufacturer_name: "日本水産".to_string(),
            memo: Some("更新後メモ".to_string()),
            display_order: 1,
            is_disabled: 0,
        };

        let result = update_manufacturer(&pool, 2, manufacturer_id, update_request).await;
        assert!(result.is_ok());

        let manufacturer = get_manufacturer_by_id(&pool, 2, manufacturer_id).await.unwrap().unwrap();
        assert_eq!(manufacturer.manufacturer_name, "日本水産");
        assert_eq!(manufacturer.memo, Some("更新後メモ".to_string()));
    }

    #[tokio::test]
    async fn test_delete_manufacturer() {
        let pool = setup_test_db().await;

        // Add manufacturer first
        let request = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, request).await.unwrap();

        let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        // Delete manufacturer
        let result = delete_manufacturer(&pool, 2, manufacturer_id).await;
        assert!(result.is_ok());

        // Verify manufacturer is disabled
        let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
        assert_eq!(manufacturers.len(), 0);
    }

    #[tokio::test]
    async fn test_empty_manufacturer_name() {
        let pool = setup_test_db().await;

        let request = AddManufacturerRequest {
            manufacturer_name: "   ".to_string(),
            memo: None,
            is_disabled: None,
        };

        let result = add_manufacturer(&pool, 2, request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_add_duplicate_manufacturer() {
        let pool = setup_test_db().await;

        // Add first manufacturer
        let request1 = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        };
        let result1 = add_manufacturer(&pool, 2, request1).await;
        assert!(result1.is_ok());

        // Try to add duplicate manufacturer
        let request2 = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: Some("異なるメモ".to_string()),
            is_disabled: None,
        };
        let result2 = add_manufacturer(&pool, 2, request2).await;
        assert!(result2.is_err());
        assert!(result2.unwrap_err().contains("already exists"));
    }

    #[tokio::test]
    async fn test_update_to_duplicate_manufacturer_name() {
        let pool = setup_test_db().await;

        // Add two manufacturers
        let request1 = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, request1).await.unwrap();

        let request2 = AddManufacturerRequest {
            manufacturer_name: "マルハニチロ".to_string(),
            memo: None,
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, request2).await.unwrap();

        let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[1].manufacturer_id; // マルハニチロ

        // Try to update to existing name
        let update_request = UpdateManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            display_order: 1,
            is_disabled: 0,
        };
        let result = update_manufacturer(&pool, 2, manufacturer_id, update_request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[tokio::test]
    async fn test_update_same_manufacturer_name() {
        let pool = setup_test_db().await;

        // Add manufacturer
        let request = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: Some("元のメモ".to_string()),
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, request).await.unwrap();

        let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        // Update with same name (should succeed)
        let update_request = UpdateManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: Some("新しいメモ".to_string()),
            display_order: 1,
            is_disabled: 0,
        };
        let result = update_manufacturer(&pool, 2, manufacturer_id, update_request).await;
        assert!(result.is_ok());

        // Verify memo was updated
        let manufacturer = get_manufacturer_by_id(&pool, 2, manufacturer_id).await.unwrap().unwrap();
        assert_eq!(manufacturer.memo, Some("新しいメモ".to_string()));
    }

}
