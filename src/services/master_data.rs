//! Shared query helpers for the per-user master-data services
//! (shops, manufacturers, products, accounts).
//!
//! Every master service runs the same three query shapes — fetch one row by
//! id, read the next display order, count duplicates of a name/code — so the
//! sqlx plumbing lives here and each service only supplies its SQL constant
//! from `sql_queries` plus the label used in error messages.

use sqlx::{sqlite::SqliteRow, FromRow, SqlitePool};

/// Fetch a single master row scoped to `user_id`, or `None` when it does not exist.
///
/// `query` must bind `user_id` first and the row id second.
pub async fn fetch_by_id<T>(
    pool: &SqlitePool,
    query: &str,
    user_id: i64,
    id: i64,
    entity: &str,
) -> Result<Option<T>, String>
where
    T: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
{
    sqlx::query_as::<_, T>(query)
        .bind(user_id)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Failed to get {}: {}", entity, e))
}

/// Read the display order to assign to the next row added by `user_id`.
pub async fn fetch_next_display_order(
    pool: &SqlitePool,
    query: &str,
    user_id: i64,
) -> Result<i64, String> {
    let result: (i64,) = sqlx::query_as(query)
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to get next display order: {}", e))?;

    Ok(result.0)
}

/// Whether `user_id` already has a row whose name/code equals `value`.
///
/// `entity` names the field in the error message (e.g. `"shop name"`).
pub async fn value_exists(
    pool: &SqlitePool,
    query: &str,
    user_id: i64,
    value: &str,
    entity: &str,
) -> Result<bool, String> {
    let result: (i64,) = sqlx::query_as(query)
        .bind(user_id)
        .bind(value)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check duplicate {}: {}", entity, e))?;

    Ok(result.0 > 0)
}

/// Same as [`value_exists`] but ignores the row being updated (`exclude_id`).
pub async fn value_exists_excluding(
    pool: &SqlitePool,
    query: &str,
    user_id: i64,
    value: &str,
    exclude_id: i64,
    entity: &str,
) -> Result<bool, String> {
    let result: (i64,) = sqlx::query_as(query)
        .bind(user_id)
        .bind(value)
        .bind(exclude_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check duplicate {}: {}", entity, e))?;

    Ok(result.0 > 0)
}

/// Execute a statement scoped to `(user_id, id)`, e.g. a logical delete.
pub async fn execute_by_id(
    pool: &SqlitePool,
    query: &str,
    user_id: i64,
    id: i64,
    action: &str,
) -> Result<(), String> {
    sqlx::query(query)
        .bind(user_id)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to {}: {}", action, e))?;

    Ok(())
}
