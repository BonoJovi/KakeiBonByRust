//! Latent-audit 2026-09 regression tests for the Category master (TDD red phase).

use super::*;
use crate::test_helpers::database::setup_test_db;

/// Copy of the module-test `setup_category1` fixture (EXPENSE / INCOME /
/// TRANSFER with ja+en i18n rows).
async fn setup_category1(pool: &SqlitePool, user_id: i64) {
    for (code, order, name, ja) in [
        ("EXPENSE", 1, "Expense", "支出"),
        ("INCOME", 2, "Income", "収入"),
        ("TRANSFER", 3, "Transfer", "振替"),
    ] {
        sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1)
            .bind(user_id)
            .bind(code)
            .bind(order)
            .bind(name)
            .bind(0)
            .execute(pool)
            .await
            .unwrap();
        for (lang, label) in [("ja", ja), ("en", name)] {
            sqlx::query(sql_queries::TEST_CATEGORY_INSERT_CATEGORY1_I18N)
                .bind(user_id)
                .bind(code)
                .bind(lang)
                .bind(label)
                .execute(pool)
                .await
                .unwrap();
        }
    }
}

async fn setup() -> (SqlitePool, CategoryService, i64) {
    let pool = setup_test_db().await;
    let user_id = 1;
    setup_category1(&pool, user_id).await;
    let service = CategoryService::new(pool.clone());
    (pool, service, user_id)
}

fn is_not_found(result: Result<(), CategoryError>) -> (bool, String) {
    match result {
        Ok(()) => (false, "Ok(())".to_string()),
        Err(e) => {
            let dbg = format!("{:?}", e);
            let api: ApiError = e.into();
            (api.code == ApiError::CODE_NOT_FOUND, format!("{} -> {:?}", dbg, api))
        }
    }
}

/// Find `is_disabled` of a CATEGORY3 node in `get_category_tree_all`.
fn cat3_is_disabled(tree: &serde_json::Value, cat1: &str, cat2: &str, cat3: &str) -> Option<i64> {
    let expense = tree
        .as_array()?
        .iter()
        .find(|n| n["category1"]["category1_code"] == cat1)?;
    let cat2_node = expense["children"]
        .as_array()?
        .iter()
        .find(|n| n["category2"]["category2_code"] == cat2)?;
    let cat3_node = cat2_node["children"]
        .as_array()?
        .iter()
        .find(|n| n["category3_code"] == cat3)?;
    cat3_node["is_disabled"].as_i64()
}

// ---------------------------------------------------------------------------
// M8
// ---------------------------------------------------------------------------

/// M8: `disable_category2` cascades IS_DISABLED=1 onto its CATEGORY3
/// children, but `enable_category2` only flips the CATEGORY2 row (despite
/// its doc comment "and its child CATEGORY3 entries").
///
/// Expected: a disable → enable round trip restores the children that the
/// disable cascaded to.
#[tokio::test]
#[ignore = "latent-audit M8"]
async fn latent_m8_enable_category2_restores_cascaded_category3() {
    let (_pool, service, user_id) = setup().await;
    let food = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
    let rice = service.add_category3(user_id, "EXPENSE", &food, "米", "Rice").await.unwrap();

    service.disable_category2(user_id, "EXPENSE", &food).await.unwrap();
    let tree = service.get_category_tree_all(user_id, "ja").await.unwrap();
    assert_eq!(
        cat3_is_disabled(&tree, "EXPENSE", &food, &rice),
        Some(1),
        "precondition: disable_category2 cascades to the child (tree: {})",
        tree
    );

    service.enable_category2(user_id, "EXPENSE", &food).await.unwrap();
    let tree = service.get_category_tree_all(user_id, "ja").await.unwrap();
    assert_eq!(
        cat3_is_disabled(&tree, "EXPENSE", &food, &rice),
        Some(0),
        "enable_category2 must re-enable the CATEGORY3 child the disable cascaded to"
    );
}

// ---------------------------------------------------------------------------
// L18
// ---------------------------------------------------------------------------

/// L18: `update_category2_i18n` never checks `rows_affected`, so updating a
/// non-existent CATEGORY2 silently returns `Ok(())`.
///
/// Expected: `not_found`.
#[tokio::test]
#[ignore = "latent-audit L18"]
async fn latent_l18_update_missing_category2_returns_not_found() {
    let (_pool, service, user_id) = setup().await;
    let result = service
        .update_category2_i18n(user_id, "EXPENSE", "C2_E_999", "存在しない", "Missing")
        .await;
    let (ok, detail) = is_not_found(result);
    assert!(ok, "updating a missing CATEGORY2 must return not_found, got {}", detail);
}

/// L18: same as above for `update_category3_i18n`.
///
/// Expected: `not_found`.
#[tokio::test]
#[ignore = "latent-audit L18"]
async fn latent_l18_update_missing_category3_returns_not_found() {
    let (_pool, service, user_id) = setup().await;
    let food = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
    let result = service
        .update_category3_i18n(user_id, "EXPENSE", &food, "C3_E_1_999", "存在しない", "Missing")
        .await;
    let (ok, detail) = is_not_found(result);
    assert!(ok, "updating a missing CATEGORY3 must return not_found, got {}", detail);
}

/// L18: `add_category2` runs the CATEGORY2 INSERT and the two i18n INSERTs
/// on independent pool connections (no tx). If a later INSERT fails the
/// CATEGORY2 row is left behind without its i18n names.
///
/// The failure is injected deterministically with a test-only trigger that
/// aborts the `en` i18n INSERT.
///
/// Expected: the add fails AND leaves no partial CATEGORY2 row.
#[tokio::test]
#[ignore = "latent-audit L18"]
async fn latent_l18_add_category2_is_atomic_on_i18n_failure() {
    let (pool, service, user_id) = setup().await;

    // Test-only fault injection (not a production query).
    sqlx::query(
        "CREATE TRIGGER latent_l18_fail_en_i18n BEFORE INSERT ON CATEGORY2_I18N \
         WHEN NEW.LANG_CODE = 'en' \
         BEGIN SELECT RAISE(ABORT, 'latent-audit L18 injected failure'); END",
    )
    .execute(&pool)
    .await
    .unwrap();

    let before: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_COUNT_BY_USER_AND_CATEGORY1)
        .bind(user_id)
        .bind("EXPENSE")
        .fetch_one(&pool)
        .await
        .unwrap();

    let result = service.add_category2(user_id, "EXPENSE", "食費", "Food").await;
    assert!(result.is_err(), "precondition: injected failure must surface as Err");

    let after: i64 = sqlx::query_scalar(sql_queries::CATEGORY2_COUNT_BY_USER_AND_CATEGORY1)
        .bind(user_id)
        .bind("EXPENSE")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        after, before,
        "a failed add_category2 must not leave a partial CATEGORY2 row behind"
    );
}

// ---------------------------------------------------------------------------
// L19
// ---------------------------------------------------------------------------

/// L19: `move_category2_up` on a missing CATEGORY2 fails the `fetch_one`
/// order lookup with sqlx `RowNotFound`, surfacing as a raw English
/// `database` error.
///
/// Expected: structured `not_found`.
#[tokio::test]
#[ignore = "latent-audit L19"]
async fn latent_l19_move_missing_category2_returns_not_found() {
    let (_pool, service, user_id) = setup().await;
    let result = service.move_category2_up(user_id, "EXPENSE", "C2_E_999").await;
    let (ok, detail) = is_not_found(result);
    assert!(ok, "moving a missing CATEGORY2 must return not_found, got {}", detail);
}

/// L19: same for `move_category3_down`.
///
/// Expected: structured `not_found`.
#[tokio::test]
#[ignore = "latent-audit L19"]
async fn latent_l19_move_missing_category3_returns_not_found() {
    let (_pool, service, user_id) = setup().await;
    let food = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
    let result = service
        .move_category3_down(user_id, "EXPENSE", &food, "C3_E_1_999")
        .await;
    let (ok, detail) = is_not_found(result);
    assert!(ok, "moving a missing CATEGORY3 must return not_found, got {}", detail);
}

/// L19: `enable_category2` ignores `rows_affected`, so enabling a missing
/// CATEGORY2 reports success.
///
/// Expected: structured `not_found`.
#[tokio::test]
#[ignore = "latent-audit L19"]
async fn latent_l19_enable_missing_category2_returns_not_found() {
    let (_pool, service, user_id) = setup().await;
    let result = service.enable_category2(user_id, "EXPENSE", "C2_E_999").await;
    let (ok, detail) = is_not_found(result);
    assert!(ok, "enabling a missing CATEGORY2 must return not_found, got {}", detail);
}

/// L19: same for `enable_category3`.
///
/// Expected: structured `not_found`.
#[tokio::test]
#[ignore = "latent-audit L19"]
async fn latent_l19_enable_missing_category3_returns_not_found() {
    let (_pool, service, user_id) = setup().await;
    let food = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
    let result = service
        .enable_category3(user_id, "EXPENSE", &food, "C3_E_1_999")
        .await;
    let (ok, detail) = is_not_found(result);
    assert!(ok, "enabling a missing CATEGORY3 must return not_found, got {}", detail);
}

// ---------------------------------------------------------------------------
// L20
// ---------------------------------------------------------------------------

/// L20: CATEGORY3 codes are generated as
/// `C3_{cat1[0]}_{last char of cat2 code}_{count+1}`. Two CATEGORY2 parents
/// whose codes end in the same digit (C2_E_1 and C2_E_11) therefore produce
/// the same CATEGORY3 code (C3_E_1_1), and category3 lookups keyed on the
/// code (transaction.rs search) hit the wrong row.
///
/// Expected: generated CATEGORY3 codes are unique per user across the tree.
#[tokio::test]
#[ignore = "latent-audit L20"]
async fn latent_l20_category3_code_unique_across_category2_parents() {
    let (_pool, service, user_id) = setup().await;

    let mut cat2_codes = Vec::new();
    for i in 1..=11 {
        let code = service
            .add_category2(user_id, "EXPENSE", &format!("中分類{}", i), &format!("Mid {}", i))
            .await
            .unwrap();
        cat2_codes.push(code);
    }
    let first = cat2_codes[0].clone();
    let eleventh = cat2_codes[10].clone();
    assert_ne!(first, eleventh, "precondition: distinct CATEGORY2 parents");

    let a = service
        .add_category3(user_id, "EXPENSE", &first, "小分類A", "Minor A")
        .await
        .unwrap();
    let b = service
        .add_category3(user_id, "EXPENSE", &eleventh, "小分類B", "Minor B")
        .await
        .unwrap();

    assert_ne!(
        a, b,
        "CATEGORY3 codes under different parents ({} / {}) must not collide",
        first, eleventh
    );
}

/// L20: `add_category2` slices `&category1_code[0..1]`, which panics on an
/// empty category1_code.
///
/// Expected: an `Err` (validation / not_found), never a panic.
#[tokio::test]
#[ignore = "latent-audit L20"]
async fn latent_l20_add_category2_empty_category1_code_does_not_panic() {
    let (_pool, service, user_id) = setup().await;
    let result = service.add_category2(user_id, "", "食費", "Food").await;
    assert!(result.is_err(), "empty category1_code must be rejected, got {:?}", result);
}

/// L20: `&category1_code[0..1]` panics when the first char is multibyte
/// (byte index 1 is not a char boundary).
///
/// Expected: an `Err` (validation / not_found), never a panic.
#[tokio::test]
#[ignore = "latent-audit L20"]
async fn latent_l20_add_category2_multibyte_category1_code_does_not_panic() {
    let (_pool, service, user_id) = setup().await;
    let result = service.add_category2(user_id, "支出", "食費", "Food").await;
    assert!(result.is_err(), "unknown multibyte category1_code must be rejected, got {:?}", result);
}

/// L20: same slice in `add_category3`.
///
/// Expected: an `Err` (validation / not_found), never a panic.
#[tokio::test]
#[ignore = "latent-audit L20"]
async fn latent_l20_add_category3_multibyte_category1_code_does_not_panic() {
    let (_pool, service, user_id) = setup().await;
    let result = service
        .add_category3(user_id, "支出", "C2_E_1", "米", "Rice")
        .await;
    assert!(result.is_err(), "unknown multibyte category1_code must be rejected, got {:?}", result);
}
