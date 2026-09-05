//! Shared query helpers for the per-user master-data services
//! (shops, manufacturers, products, accounts).
//!
//! Every master service runs the same three query shapes — fetch one row by
//! id, read the next display order, count duplicates of a name/code — so the
//! sqlx plumbing lives here and each service only supplies its SQL constant
//! from `sql_queries` plus the label used in error messages.
//!
//! Fable-5 review #26 (PR3):
//! - Helpers return `ApiError` directly instead of `Result<_, String>`, so
//!   each service can `?`-propagate without the boilerplate `.map_err(
//!   ApiError::database)?` at every call site.
//! - `MasterCrudSpec` collects the six SQL constants plus the three labels
//!   each master service needs, so the shared prelude (validation +
//!   duplicate check) can be driven off a single value per screen.
//! - `run_delete_expect_one_in_tx` and `run_update_expect_one` return the row
//!   count and translate `0 → NotFound`, eliminating the pre-check
//!   `get_by_id().await?.ok_or(NotFound)?` boilerplate on 6 sites (and its
//!   TOCTOU window). Update paths accept the caller's `sqlx::query(...)
//!   .bind(...).bind(...)` because each table's column set differs; the
//!   helper only takes over the "execute + rows_affected" step.

use sqlx::{sqlite::SqliteRow, FromRow, Sqlite, SqlitePool, Transaction};

use crate::api_error::ApiError;

/// Static description of one master service's SQL surface + user-facing
/// labels. Kept as `&'static str` fields so it can live in a `const` at
/// each service module and be passed by reference into the helpers below —
/// no allocation, no lifetime plumbing.
///
/// Fields:
/// - `entity_label`      — capitalised English name for `ApiError` (`"Shop"`,
///   `"Manufacturer"`, ...). Serialised via `ApiError::not_found` /
///   `duplicate_name` and lowercased by those constructors before hitting
///   the wire as the classifier `entity` slug.
/// - `name_label`        — English label passed to `validation::validate_
///   master_name` (e.g. `"Shop name"`). Ends up in the validation error
///   text the frontend uses as the field-length message.
/// - `check_duplicate_for_add_sql` — `SELECT COUNT(*)` bound with
///   `(user_id, name)`.
/// - `check_duplicate_for_update_sql` — `SELECT COUNT(*)` bound with
///   `(user_id, name, exclude_id)`.
/// - `delete_logical_sql` — bound with `(user_id, id)`. Must use logical
///   delete semantics that update at most one row.
pub struct MasterCrudSpec {
    pub entity_label: &'static str,
    pub name_label: &'static str,
    pub check_duplicate_for_add_sql: &'static str,
    pub check_duplicate_for_update_sql: &'static str,
    pub delete_logical_sql: &'static str,
}

/// Fetch a single master row scoped to `user_id`, or `None` when it does not exist.
///
/// `query` must bind `user_id` first and the row id second. Only called from
/// each service's `get_X_by_id` helper (which the module tests still use as
/// a post-write probe), so the compiler flags it dead outside `cfg(test)` —
/// suppress here rather than at every consumer.
#[allow(dead_code)]
pub async fn fetch_by_id<T>(
    pool: &SqlitePool,
    query: &str,
    user_id: i64,
    id: i64,
) -> Result<Option<T>, ApiError>
where
    T: for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
{
    let row = sqlx::query_as::<_, T>(query)
        .bind(user_id)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

/// Read the display order to assign to the next row added by `user_id`.
pub async fn fetch_next_display_order(
    pool: &SqlitePool,
    query: &str,
    user_id: i64,
) -> Result<i64, ApiError> {
    let result: (i64,) = sqlx::query_as(query)
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    Ok(result.0)
}

/// Whether `user_id` already has a row whose name/code equals `value`.
/// Kept `pub` so `account.rs` — which checks duplicates against the CODE
/// column, not NAME, and is not yet on `MasterCrudSpec` — can still call
/// it directly until its own PR4 refactor.
pub async fn value_exists(
    pool: &SqlitePool,
    query: &str,
    user_id: i64,
    value: &str,
) -> Result<bool, ApiError> {
    let result: (i64,) = sqlx::query_as(query)
        .bind(user_id)
        .bind(value)
        .fetch_one(pool)
        .await?;
    Ok(result.0 > 0)
}

/// Same as [`value_exists`] but ignores the row being updated (`exclude_id`).
async fn value_exists_excluding(
    pool: &SqlitePool,
    query: &str,
    user_id: i64,
    value: &str,
    exclude_id: i64,
) -> Result<bool, ApiError> {
    let result: (i64,) = sqlx::query_as(query)
        .bind(user_id)
        .bind(value)
        .bind(exclude_id)
        .fetch_one(pool)
        .await?;
    Ok(result.0 > 0)
}

/// Run the shared prelude for a master **add** flow: duplicate name check.
/// Validation (name + memo) stays at the service so the caller can pick the
/// right validator for its bounded fields (product's manufacturer id, etc.).
/// Returns `Err(ApiError::duplicate_name(entity))` when the name is taken;
/// otherwise `Ok(())`.
pub async fn check_duplicate_for_add(
    spec: &MasterCrudSpec,
    pool: &SqlitePool,
    user_id: i64,
    name: &str,
) -> Result<(), ApiError> {
    if value_exists(pool, spec.check_duplicate_for_add_sql, user_id, name).await? {
        return Err(ApiError::duplicate_name(spec.entity_label));
    }
    Ok(())
}

/// Run the shared prelude for a master **update** flow: duplicate name
/// check excluding the row being edited. Returns
/// `Err(ApiError::duplicate_name(entity))` when the new name collides
/// with another row; otherwise `Ok(())`.
pub async fn check_duplicate_for_update(
    spec: &MasterCrudSpec,
    pool: &SqlitePool,
    user_id: i64,
    exclude_id: i64,
    name: &str,
) -> Result<(), ApiError> {
    if value_exists_excluding(
        pool,
        spec.check_duplicate_for_update_sql,
        user_id,
        name,
        exclude_id,
    )
    .await?
    {
        return Err(ApiError::duplicate_name(spec.entity_label));
    }
    Ok(())
}

/// Logical-delete a master row and treat `rows_affected == 0` as
/// `ApiError::not_found(entity)`.
///
/// Before Fable-5 review #26 every delete flow did two round-trips:
/// `get_by_id().await?.ok_or(NotFound)?` followed by an unchecked
/// `execute` that discarded `rows_affected`. That was both slower (2
/// queries when 1 would do) and TOCTOU-vulnerable — the row could be
/// deleted between the check and the execute, and the caller would still
/// report success. Threading the count back and mapping `0 → NotFound`
/// eliminates both.
/// Execute the caller's `spec.delete_logical_sql` on a caller-owned
/// transaction and translate `0 → NotFound`. Used by the per-master
/// delete paths (shop / product / manufacturer) so the `CHECK_IN_USE`
/// guard and the `IS_DISABLED=1` write share one transaction —
/// Fable-5 #14 closed the pre-fix window where the two ran on
/// independently pooled connections. Callers open the tx with
/// `pool.begin()` (sqlx default: `BEGIN DEFERRED`) and commit after
/// this helper returns.
pub async fn run_delete_expect_one_in_tx(
    spec: &MasterCrudSpec,
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    id: i64,
) -> Result<(), ApiError> {
    let affected = sqlx::query(spec.delete_logical_sql)
        .bind(user_id)
        .bind(id)
        .execute(&mut **tx)
        .await?
        .rows_affected();
    if affected == 0 {
        return Err(ApiError::not_found(spec.entity_label));
    }
    Ok(())
}

/// Assert that the update just executed by the caller actually touched a
/// row. Same rationale as [`run_delete_expect_one_in_tx`]: eliminates the
/// separate pre-check `get_by_id().await?.ok_or(NotFound)?` before an
/// UPDATE and its TOCTOU window. The caller performs the UPDATE itself
/// (each master's column set differs, so a generic helper would need a
/// bind-callback for little gain) and hands the `rows_affected` here.
pub fn ensure_update_affected_one(
    spec: &MasterCrudSpec,
    rows_affected: u64,
) -> Result<(), ApiError> {
    if rows_affected == 0 {
        return Err(ApiError::not_found(spec.entity_label));
    }
    Ok(())
}

/// Translate a `SELECT EXISTS(...)` result — bound and executed by the
/// caller because the referenced tables (and therefore the number of
/// `?` slots) differ per master — into
/// `Err(ApiError::in_use(entity_label))` when the row is still
/// referenced by other data. Kept as a plain function so each service's
/// delete path can stay a linear read: run the master's own
/// `CHECK_IN_USE` query, hand the flag here, then call
/// [`run_delete_expect_one_in_tx`].
pub fn reject_if_in_use(entity_label: &str, in_use_flag: i64) -> Result<(), ApiError> {
    if in_use_flag != 0 {
        return Err(ApiError::in_use(entity_label));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // A tiny stand-in spec used only in the pure-Rust helper tests below.
    // Every string is nonsense on purpose — these tests do not touch the
    // database, they only pin the two `expect_one` mappers.
    const TEST_SPEC: MasterCrudSpec = MasterCrudSpec {
        entity_label: "Shop",
        name_label: "Shop name",
        check_duplicate_for_add_sql: "SELECT 1",
        check_duplicate_for_update_sql: "SELECT 1",
        delete_logical_sql: "DELETE",
    };

    #[test]
    fn ensure_update_affected_one_maps_zero_to_not_found() {
        let err = ensure_update_affected_one(&TEST_SPEC, 0).unwrap_err();
        assert_eq!(err.code, ApiError::CODE_NOT_FOUND);
        assert_eq!(err.entity.as_deref(), Some("shop"));
    }

    #[test]
    fn ensure_update_affected_one_passes_positive_count() {
        assert!(ensure_update_affected_one(&TEST_SPEC, 1).is_ok());
        assert!(ensure_update_affected_one(&TEST_SPEC, 42).is_ok());
    }

    #[test]
    fn reject_if_in_use_maps_positive_flag_to_in_use() {
        let err = reject_if_in_use(TEST_SPEC.entity_label, 1).unwrap_err();
        assert_eq!(err.code, ApiError::CODE_IN_USE);
        assert_eq!(err.entity.as_deref(), Some("shop"));
    }

    #[test]
    fn reject_if_in_use_passes_when_flag_is_zero() {
        assert!(reject_if_in_use(TEST_SPEC.entity_label, 0).is_ok());
    }
}
