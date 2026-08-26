use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use crate::api_error::ApiError;
use crate::services::master_data::{self, MasterCrudSpec};
use crate::sql_queries;
use crate::validation;

/// Full description of the Manufacturer master's SQL surface + labels.
/// Fable-5 review #26.
const SPEC: MasterCrudSpec = MasterCrudSpec {
    entity_label: "Manufacturer",
    name_label: "Manufacturer name",
    check_duplicate_for_add_sql: sql_queries::MANUFACTURER_CHECK_DUPLICATE_FOR_ADD,
    check_duplicate_for_update_sql: sql_queries::MANUFACTURER_CHECK_DUPLICATE_FOR_UPDATE,
    delete_logical_sql: sql_queries::MANUFACTURER_DELETE_LOGICAL,
};

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
pub async fn get_manufacturers(pool: &SqlitePool, user_id: i64, include_disabled: bool) -> Result<Vec<Manufacturer>, ApiError> {
    let query = if include_disabled {
        sql_queries::MANUFACTURER_GET_ALL_INCLUDING_DISABLED
    } else {
        sql_queries::MANUFACTURER_GET_ALL
    };

    let manufacturers = sqlx::query_as::<_, Manufacturer>(query)
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(manufacturers)
}

/// Get a single manufacturer by ID. Kept for module tests only after the
/// Fable-5 #26 refactor. See shop.rs::get_shop_by_id for rationale.
#[allow(dead_code)]
pub async fn get_manufacturer_by_id(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: i64,
) -> Result<Option<Manufacturer>, ApiError> {
    master_data::fetch_by_id(pool, sql_queries::MANUFACTURER_GET_BY_ID, user_id, manufacturer_id)
        .await
}

/// Add a new manufacturer
pub async fn add_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    request: AddManufacturerRequest,
) -> Result<String, ApiError> {
    validation::validate_master_name(SPEC.name_label, &request.manufacturer_name)
        .map_err(ApiError::validation)?;
    validation::validate_memo("Memo", request.memo.as_ref())
        .map_err(ApiError::validation)?;

    master_data::check_duplicate_for_add(&SPEC, pool, user_id, &request.manufacturer_name).await?;

    let display_order = master_data::fetch_next_display_order(
        pool,
        sql_queries::MANUFACTURER_GET_NEXT_DISPLAY_ORDER,
        user_id,
    )
    .await?;

    let is_disabled = request.is_disabled.unwrap_or(0);

    sqlx::query(sql_queries::MANUFACTURER_INSERT)
        .bind(user_id)
        .bind(&request.manufacturer_name)
        .bind(&request.memo)
        .bind(display_order)
        .bind(is_disabled)
        .execute(pool)
        .await?;

    Ok("Manufacturer added successfully".to_string())
}

/// Update a manufacturer
pub async fn update_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: i64,
    request: UpdateManufacturerRequest,
) -> Result<String, ApiError> {
    validation::validate_master_name(SPEC.name_label, &request.manufacturer_name)
        .map_err(ApiError::validation)?;
    validation::validate_memo("Memo", request.memo.as_ref())
        .map_err(ApiError::validation)?;

    master_data::check_duplicate_for_update(
        &SPEC,
        pool,
        user_id,
        manufacturer_id,
        &request.manufacturer_name,
    )
    .await?;

    // Pre-check + not_found is now derived from `rows_affected` on the
    // UPDATE itself (Fable-5 review #26). See shop.rs::update_shop for
    // rationale.
    let affected = sqlx::query(sql_queries::MANUFACTURER_UPDATE)
        .bind(&request.manufacturer_name)
        .bind(&request.memo)
        .bind(request.display_order)
        .bind(request.is_disabled)
        .bind(user_id)
        .bind(manufacturer_id)
        .execute(pool)
        .await?
        .rows_affected();
    master_data::ensure_update_affected_one(&SPEC, affected)?;

    Ok("Manufacturer updated successfully".to_string())
}

/// Delete a manufacturer (logical deletion). Rejected with
/// `ApiError::in_use("Manufacturer")` when any product row (active or
/// disabled) still points at it via `MANUFACTURER_ID`. See
/// `sql_queries::MANUFACTURER_CHECK_IN_USE`.
pub async fn delete_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: i64,
) -> Result<String, ApiError> {
    let (in_use,): (i64,) = sqlx::query_as(sql_queries::MANUFACTURER_CHECK_IN_USE)
        .bind(user_id)
        .bind(manufacturer_id)
        .fetch_one(pool)
        .await?;
    master_data::reject_if_in_use(SPEC.entity_label, in_use)?;
    master_data::run_delete_expect_one(&SPEC, pool, user_id, manufacturer_id).await?;
    Ok("Manufacturer deleted successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts;
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

        // PRODUCTS is created too so MANUFACTURER_CHECK_IN_USE has a table
        // to read against. `delete_manufacturer` runs the guard on every
        // call — including happy paths — so this must exist in every
        // manufacturer-service test.
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
    async fn test_empty_manufacturer_name_returns_validation_code() {
        let pool = setup_test_db().await;

        let request = AddManufacturerRequest {
            manufacturer_name: "   ".to_string(),
            memo: None,
            is_disabled: None,
        };

        let err = add_manufacturer(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_add_duplicate_manufacturer_returns_duplicate_name_code() {
        let pool = setup_test_db().await;

        let request1 = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        };
        let result1 = add_manufacturer(&pool, 2, request1).await;
        assert!(result1.is_ok());

        let request2 = AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: Some("異なるメモ".to_string()),
            is_disabled: None,
        };
        let err = add_manufacturer(&pool, 2, request2).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME);
        assert_eq!(err.entity.as_deref(), Some("manufacturer"));
    }

    #[tokio::test]
    async fn test_update_to_duplicate_manufacturer_name_returns_duplicate_name_code() {
        let pool = setup_test_db().await;

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
        let manufacturer_id = manufacturers[1].manufacturer_id;

        let update_request = UpdateManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            display_order: 1,
            is_disabled: 0,
        };
        let err = update_manufacturer(&pool, 2, manufacturer_id, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME);
    }

    #[tokio::test]
    async fn test_update_missing_manufacturer_returns_not_found_code() {
        let pool = setup_test_db().await;

        let update_request = UpdateManufacturerRequest {
            manufacturer_name: "存在しない".to_string(),
            memo: None,
            display_order: 1,
            is_disabled: 0,
        };
        let err = update_manufacturer(&pool, 2, 9999, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
        assert_eq!(err.entity.as_deref(), Some("manufacturer"));
    }

    #[tokio::test]
    async fn test_delete_missing_manufacturer_returns_not_found_code() {
        let pool = setup_test_db().await;
        let err = delete_manufacturer(&pool, 2, 9999).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_manufacturer_rejected_when_referenced_by_product() {
        let pool = setup_test_db().await;

        add_manufacturer(&pool, 2, AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        })
        .await
        .unwrap();
        let manufacturer_id = get_manufacturers(&pool, 2, false).await.unwrap()[0].manufacturer_id;

        sqlx::query("INSERT INTO PRODUCTS (USER_ID, PRODUCT_NAME, MANUFACTURER_ID, DISPLAY_ORDER) VALUES (?, ?, ?, 0)")
            .bind(2_i64)
            .bind("サバ缶")
            .bind(manufacturer_id)
            .execute(&pool)
            .await
            .unwrap();

        let err = delete_manufacturer(&pool, 2, manufacturer_id).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
        assert_eq!(err.entity.as_deref(), Some("manufacturer"));

        assert_eq!(get_manufacturers(&pool, 2, false).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_delete_manufacturer_rejected_when_only_disabled_products_reference() {
        let pool = setup_test_db().await;

        add_manufacturer(&pool, 2, AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        })
        .await
        .unwrap();
        let manufacturer_id = get_manufacturers(&pool, 2, false).await.unwrap()[0].manufacturer_id;

        // Even a disabled product still counts — the FK link exists and
        // the products screen still surfaces it under "Show disabled".
        sqlx::query("INSERT INTO PRODUCTS (USER_ID, PRODUCT_NAME, MANUFACTURER_ID, DISPLAY_ORDER, IS_DISABLED) VALUES (?, ?, ?, 0, 1)")
            .bind(2_i64)
            .bind("旧サバ缶")
            .bind(manufacturer_id)
            .execute(&pool)
            .await
            .unwrap();

        let err = delete_manufacturer(&pool, 2, manufacturer_id).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
    }

    #[tokio::test]
    async fn test_delete_manufacturer_ignores_other_users_references() {
        let pool = setup_test_db().await;

        add_manufacturer(&pool, 2, AddManufacturerRequest {
            manufacturer_name: "ニッスイ".to_string(),
            memo: None,
            is_disabled: None,
        })
        .await
        .unwrap();
        let manufacturer_id = get_manufacturers(&pool, 2, false).await.unwrap()[0].manufacturer_id;

        // User 1 uses the same manufacturer_id (cross-user id collision
        // is possible on AUTOINCREMENT); user 2's delete must still succeed.
        sqlx::query("INSERT INTO PRODUCTS (USER_ID, PRODUCT_NAME, MANUFACTURER_ID, DISPLAY_ORDER) VALUES (?, ?, ?, 0)")
            .bind(1_i64)
            .bind("他ユーザ商品")
            .bind(manufacturer_id)
            .execute(&pool)
            .await
            .unwrap();

        assert!(delete_manufacturer(&pool, 2, manufacturer_id).await.is_ok());
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

    // Issue #37 Phase 2-3 — bounded-field length checks must count
    // characters (not bytes). Japanese is 3 bytes per char in UTF-8.

    #[tokio::test]
    async fn test_add_manufacturer_accepts_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let request = AddManufacturerRequest {
            manufacturer_name: "あ".repeat(consts::MAX_NAME_LEN),
            memo: None,
            is_disabled: None,
        };
        let result = add_manufacturer(&pool, 2, request).await;
        assert!(result.is_ok(), "expected MAX_NAME_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_add_manufacturer_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let request = AddManufacturerRequest {
            manufacturer_name: "あ".repeat(consts::MAX_NAME_LEN + 1),
            memo: None,
            is_disabled: None,
        };
        let err = add_manufacturer(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_add_manufacturer_accepts_max_chars_of_multibyte_memo() {
        let pool = setup_test_db().await;

        let request = AddManufacturerRequest {
            manufacturer_name: "メーカー".to_string(),
            memo: Some("メ".repeat(consts::MAX_MEMO_LEN)),
            is_disabled: None,
        };
        let result = add_manufacturer(&pool, 2, request).await;
        assert!(result.is_ok(), "expected MAX_MEMO_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_add_manufacturer_rejects_over_max_chars_of_multibyte_memo() {
        let pool = setup_test_db().await;

        let request = AddManufacturerRequest {
            manufacturer_name: "メーカー".to_string(),
            memo: Some("メ".repeat(consts::MAX_MEMO_LEN + 1)),
            is_disabled: None,
        };
        let err = add_manufacturer(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_update_manufacturer_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let add_request = AddManufacturerRequest {
            manufacturer_name: "メーカー".to_string(),
            memo: None,
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, add_request).await.unwrap();
        let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        let update_request = UpdateManufacturerRequest {
            manufacturer_name: "あ".repeat(consts::MAX_NAME_LEN + 1),
            memo: None,
            display_order: 1,
            is_disabled: 0,
        };
        let err = update_manufacturer(&pool, 2, manufacturer_id, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_update_manufacturer_rejects_over_max_chars_of_multibyte_memo() {
        let pool = setup_test_db().await;

        let add_request = AddManufacturerRequest {
            manufacturer_name: "メーカー".to_string(),
            memo: None,
            is_disabled: None,
        };
        add_manufacturer(&pool, 2, add_request).await.unwrap();
        let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
        let manufacturer_id = manufacturers[0].manufacturer_id;

        let update_request = UpdateManufacturerRequest {
            manufacturer_name: "メーカー".to_string(),
            memo: Some("メ".repeat(consts::MAX_MEMO_LEN + 1)),
            display_order: 1,
            is_disabled: 0,
        };
        let err = update_manufacturer(&pool, 2, manufacturer_id, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }
}
