use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, FromRow};
use crate::api_error::ApiError;
use crate::services::master_data::{self, MasterCrudSpec};
use crate::sql_queries;
use crate::validation;

/// Full description of the Shop master's SQL surface + labels. Consumed by
/// the generic `master_data` helpers so the shared prelude (duplicate check
/// + not-found mapping) stays out of this module. Fable-5 review #26.
const SPEC: MasterCrudSpec = MasterCrudSpec {
    entity_label: "Shop",
    name_label: "Shop name",
    check_duplicate_for_add_sql: sql_queries::SHOP_CHECK_DUPLICATE_FOR_ADD,
    check_duplicate_for_update_sql: sql_queries::SHOP_CHECK_DUPLICATE_FOR_UPDATE,
    delete_logical_sql: sql_queries::SHOP_DELETE_LOGICAL,
};

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
pub async fn get_shops(pool: &SqlitePool, user_id: i64) -> Result<Vec<Shop>, ApiError> {
    let shops = sqlx::query_as::<_, Shop>(sql_queries::SHOP_GET_ALL)
        .bind(user_id)
        .fetch_all(pool)
        .await?;
    Ok(shops)
}

/// Get a single shop by ID. No longer called from production paths after
/// the Fable-5 #26 refactor (delete/update now derive not_found from
/// `rows_affected` rather than a pre-check), but kept because the module
/// tests still exercise it as a "did the write actually persist?" probe.
#[allow(dead_code)]
pub async fn get_shop_by_id(
    pool: &SqlitePool,
    user_id: i64,
    shop_id: i64,
) -> Result<Option<Shop>, ApiError> {
    master_data::fetch_by_id(pool, sql_queries::SHOP_GET_BY_ID, user_id, shop_id).await
}

/// Add a new shop
pub async fn add_shop(
    pool: &SqlitePool,
    user_id: i64,
    request: AddShopRequest,
) -> Result<String, ApiError> {
    validation::validate_master_name(SPEC.name_label, &request.shop_name)
        .map_err(ApiError::validation)?;
    validation::validate_memo("Memo", request.memo.as_ref())
        .map_err(ApiError::validation)?;

    master_data::check_duplicate_for_add(&SPEC, pool, user_id, &request.shop_name).await?;

    let display_order = master_data::fetch_next_display_order(
        pool,
        sql_queries::SHOP_GET_NEXT_DISPLAY_ORDER,
        user_id,
    )
    .await?;

    sqlx::query(sql_queries::SHOP_INSERT)
        .bind(user_id)
        .bind(&request.shop_name)
        .bind(&request.memo)
        .bind(display_order)
        .execute(pool)
        .await?;

    Ok("Shop added successfully".to_string())
}

/// Update a shop
pub async fn update_shop(
    pool: &SqlitePool,
    user_id: i64,
    shop_id: i64,
    request: UpdateShopRequest,
) -> Result<String, ApiError> {
    validation::validate_master_name(SPEC.name_label, &request.shop_name)
        .map_err(ApiError::validation)?;
    validation::validate_memo("Memo", request.memo.as_ref())
        .map_err(ApiError::validation)?;

    master_data::check_duplicate_for_update(&SPEC, pool, user_id, shop_id, &request.shop_name)
        .await?;

    // Pre-check `get_shop_by_id().ok_or(NotFound)?` was removed here
    // (Fable-5 review #26): rows_affected from the UPDATE tells us the
    // same thing in one round-trip instead of two, and it closes the
    // TOCTOU window where the row could vanish between the pre-check
    // and the update. See master_data::ensure_update_affected_one.
    let affected = sqlx::query(sql_queries::SHOP_UPDATE)
        .bind(&request.shop_name)
        .bind(&request.memo)
        .bind(request.display_order)
        .bind(user_id)
        .bind(shop_id)
        .execute(pool)
        .await?
        .rows_affected();
    master_data::ensure_update_affected_one(&SPEC, affected)?;

    Ok("Shop updated successfully".to_string())
}

/// Delete a shop (logical deletion). Rejected with
/// `ApiError::in_use("Shop")` when any transaction or recurring rule
/// still names this shop; the frontend surfaces that as the "still in
/// use" toast and steers the user to disable instead. See
/// `sql_queries::SHOP_CHECK_IN_USE` for the exact scope.
pub async fn delete_shop(
    pool: &SqlitePool,
    user_id: i64,
    shop_id: i64,
) -> Result<String, ApiError> {
    let (in_use,): (i64,) = sqlx::query_as(sql_queries::SHOP_CHECK_IN_USE)
        .bind(user_id)
        .bind(shop_id)
        .bind(user_id)
        .bind(shop_id)
        .fetch_one(pool)
        .await?;
    master_data::reject_if_in_use(SPEC.entity_label, in_use)?;
    master_data::run_delete_expect_one(&SPEC, pool, user_id, shop_id).await?;
    Ok("Shop deleted successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consts;
    use crate::test_helpers::database::{init_db, TEST_DB_URL};

    async fn setup_test_db() -> SqlitePool {
        let pool = init_db(TEST_DB_URL).await.unwrap();

        sqlx::query(sql_queries::TEST_CREATE_USERS_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query(sql_queries::TEST_SHOP_CREATE_TABLE)
            .execute(&pool)
            .await
            .unwrap();

        // TRANSACTIONS_HEADER / RECURRING_RULES so SHOP_CHECK_IN_USE has
        // tables to read against. `delete_shop` runs the guard on every
        // call — including happy paths — so these must exist in every
        // shop-service test.
        sqlx::query(sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(sql_queries::TEST_CREATE_RECURRING_RULES_MINIMAL)
            .execute(&pool)
            .await
            .unwrap();

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

        let add_request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, add_request).await.unwrap();

        let shops = get_shops(&pool, 2).await.unwrap();
        let shop_id = shops[0].shop_id;

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

        let request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, request).await.unwrap();

        let shops = get_shops(&pool, 2).await.unwrap();
        let shop_id = shops[0].shop_id;

        let result = delete_shop(&pool, 2, shop_id).await;
        assert!(result.is_ok());

        let shops = get_shops(&pool, 2).await.unwrap();
        assert_eq!(shops.len(), 0);
    }

    #[tokio::test]
    async fn test_empty_shop_name_returns_validation_code() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "   ".to_string(),
            memo: None,
        };

        let err = add_shop(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_add_duplicate_shop_returns_duplicate_name_code() {
        let pool = setup_test_db().await;

        let request1 = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, request1).await.unwrap();

        let request2 = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: Some("異なるメモ".to_string()),
        };
        let err = add_shop(&pool, 2, request2).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME);
        assert_eq!(err.entity.as_deref(), Some("shop"));
    }

    #[tokio::test]
    async fn test_update_to_duplicate_shop_name_returns_duplicate_name_code() {
        let pool = setup_test_db().await;

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
        let shop_id = shops[1].shop_id;

        let update_request = UpdateShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
            display_order: 1,
        };
        let err = update_shop(&pool, 2, shop_id, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_DUPLICATE_NAME);
    }

    #[tokio::test]
    async fn test_update_missing_shop_returns_not_found_code() {
        let pool = setup_test_db().await;

        let update_request = UpdateShopRequest {
            shop_name: "存在しない".to_string(),
            memo: None,
            display_order: 1,
        };
        let err = update_shop(&pool, 2, 9999, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
        assert_eq!(err.entity.as_deref(), Some("shop"));
    }

    #[tokio::test]
    async fn test_delete_missing_shop_returns_not_found_code() {
        let pool = setup_test_db().await;
        let err = delete_shop(&pool, 2, 9999).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_shop_rejected_when_referenced_by_transaction() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, request).await.unwrap();
        let shop_id = get_shops(&pool, 2).await.unwrap()[0].shop_id;

        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_SHOP_REF)
            .bind(2_i64)
            .bind(shop_id)
            .execute(&pool)
            .await
            .unwrap();

        let err = delete_shop(&pool, 2, shop_id).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
        assert_eq!(err.entity.as_deref(), Some("shop"));

        // Shop still visible — the guard aborted before the logical delete.
        assert_eq!(get_shops(&pool, 2).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_delete_shop_rejected_when_referenced_by_recurring_rule() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, request).await.unwrap();
        let shop_id = get_shops(&pool, 2).await.unwrap()[0].shop_id;

        sqlx::query(sql_queries::TEST_INSERT_RECURRING_RULES_SHOP_REF)
            .bind(2_i64)
            .bind(shop_id)
            .execute(&pool)
            .await
            .unwrap();

        let err = delete_shop(&pool, 2, shop_id).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
    }

    #[tokio::test]
    async fn test_delete_shop_ignores_other_users_references() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, request).await.unwrap();
        let shop_id = get_shops(&pool, 2).await.unwrap()[0].shop_id;

        // Reference belongs to user 1, so user 2's delete must succeed.
        sqlx::query(sql_queries::TEST_INSERT_TRANSACTIONS_HEADER_SHOP_REF)
            .bind(1_i64)
            .bind(shop_id)
            .execute(&pool)
            .await
            .unwrap();

        let result = delete_shop(&pool, 2, shop_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_same_shop_name() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: Some("元のメモ".to_string()),
        };
        add_shop(&pool, 2, request).await.unwrap();

        let shops = get_shops(&pool, 2).await.unwrap();
        let shop_id = shops[0].shop_id;

        let update_request = UpdateShopRequest {
            shop_name: "イオン新宿店".to_string(),
            memo: Some("新しいメモ".to_string()),
            display_order: 1,
        };
        let result = update_shop(&pool, 2, shop_id, update_request).await;
        assert!(result.is_ok());

        let shop = get_shop_by_id(&pool, 2, shop_id).await.unwrap().unwrap();
        assert_eq!(shop.memo, Some("新しいメモ".to_string()));
    }

    // Issue #37 Phase 2-2 — bounded-field length checks must count
    // characters (not bytes). Japanese is 3 bytes per char in UTF-8.

    #[tokio::test]
    async fn test_add_shop_accepts_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "あ".repeat(consts::MAX_NAME_LEN),
            memo: None,
        };
        let result = add_shop(&pool, 2, request).await;
        assert!(result.is_ok(), "expected MAX_NAME_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_add_shop_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "あ".repeat(consts::MAX_NAME_LEN + 1),
            memo: None,
        };
        let err = add_shop(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_add_shop_accepts_max_chars_of_multibyte_memo() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "店".to_string(),
            memo: Some("メ".repeat(consts::MAX_MEMO_LEN)),
        };
        let result = add_shop(&pool, 2, request).await;
        assert!(result.is_ok(), "expected MAX_MEMO_LEN multibyte chars to be accepted: {:?}", result.err());
    }

    #[tokio::test]
    async fn test_add_shop_rejects_over_max_chars_of_multibyte_memo() {
        let pool = setup_test_db().await;

        let request = AddShopRequest {
            shop_name: "店".to_string(),
            memo: Some("メ".repeat(consts::MAX_MEMO_LEN + 1)),
        };
        let err = add_shop(&pool, 2, request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_update_shop_rejects_over_max_chars_of_multibyte_name() {
        let pool = setup_test_db().await;

        let add_request = AddShopRequest {
            shop_name: "店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, add_request).await.unwrap();
        let shops = get_shops(&pool, 2).await.unwrap();
        let shop_id = shops[0].shop_id;

        let update_request = UpdateShopRequest {
            shop_name: "あ".repeat(consts::MAX_NAME_LEN + 1),
            memo: None,
            display_order: 1,
        };
        let err = update_shop(&pool, 2, shop_id, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_NAME_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }

    #[tokio::test]
    async fn test_update_shop_rejects_over_max_chars_of_multibyte_memo() {
        let pool = setup_test_db().await;

        let add_request = AddShopRequest {
            shop_name: "店".to_string(),
            memo: None,
        };
        add_shop(&pool, 2, add_request).await.unwrap();
        let shops = get_shops(&pool, 2).await.unwrap();
        let shop_id = shops[0].shop_id;

        let update_request = UpdateShopRequest {
            shop_name: "店".to_string(),
            memo: Some("メ".repeat(consts::MAX_MEMO_LEN + 1)),
            display_order: 1,
        };
        let err = update_shop(&pool, 2, shop_id, update_request).await.unwrap_err();
        assert_eq!(err.code, ApiError::CODE_VALIDATION);
        assert!(err.message.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", err.message);
    }
}
