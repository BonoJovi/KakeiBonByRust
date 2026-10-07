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

/// M8 follow-up (CodeRabbit on #146): enabling a CATEGORY2 that is already
/// enabled is a successful no-op and leaves its children alone — a CATEGORY3
/// hidden on its own must not be shown again by a repeated / stale enable.
#[tokio::test]
async fn latent_m8_enable_already_enabled_category2_keeps_hidden_children() {
    let (_pool, service, user_id) = setup().await;
    let food = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
    let rice = service.add_category3(user_id, "EXPENSE", &food, "米", "Rice").await.unwrap();

    // Hide only the child; the parent stays enabled.
    service.disable_category3(user_id, "EXPENSE", &food, &rice).await.unwrap();

    service
        .enable_category2(user_id, "EXPENSE", &food)
        .await
        .expect("enabling an already enabled CATEGORY2 must succeed");
    let tree = service.get_category_tree_all(user_id, "ja").await.unwrap();
    assert_eq!(
        cat3_is_disabled(&tree, "EXPENSE", &food, &rice),
        Some(1),
        "a CATEGORY3 hidden on its own must stay hidden (tree: {})",
        tree
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
async fn latent_l20_add_category2_multibyte_category1_code_does_not_panic() {
    let (_pool, service, user_id) = setup().await;
    let result = service.add_category2(user_id, "支出", "食費", "Food").await;
    assert!(result.is_err(), "unknown multibyte category1_code must be rejected, got {:?}", result);
}

/// L20: same slice in `add_category3`.
///
/// Expected: an `Err` (validation / not_found), never a panic.
#[tokio::test]
async fn latent_l20_add_category3_multibyte_category1_code_does_not_panic() {
    let (_pool, service, user_id) = setup().await;
    let result = service
        .add_category3(user_id, "支出", "C2_E_1", "米", "Rice")
        .await;
    assert!(result.is_err(), "unknown multibyte category1_code must be rejected, got {:?}", result);
}

// ---------------------------------------------------------------------------
// scan2 (2026-09-29 latent scan) — masters
// ---------------------------------------------------------------------------

/// CATEGORY2 codes under `cat1`, in the order `get_category_tree` lists them.
fn cat2_codes_in_tree(tree: &serde_json::Value, cat1: &str) -> Vec<String> {
    tree.as_array()
        .and_then(|a| a.iter().find(|n| n["category1"]["category1_code"] == cat1))
        .and_then(|n| n["children"].as_array())
        .map(|children| {
            children
                .iter()
                .filter_map(|c| c["category2"]["category2_code"].as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// scan2-M5: `swap_category2_with_sibling` swaps with the row at
/// DISPLAY_ORDER ± 1 regardless of IS_DISABLED. With children A(1),
/// B(2, hidden), C(3), "↑" on C swaps it with the hidden B, so the visible
/// order (and the picker order) stays A, C — the click appears to do nothing.
///
/// Expected: "↑" moves C above the nearest *enabled* sibling, so the
/// enabled tree lists C before A after one click.
#[tokio::test]
#[ignore = "latent-audit scan2-M5"]
async fn latent_scan2_m5_move_up_skips_hidden_sibling() {
    let (_pool, service, user_id) = setup().await;
    let a = service.add_category2(user_id, "EXPENSE", "食費", "Food").await.unwrap();
    let b = service.add_category2(user_id, "EXPENSE", "外食", "Dining").await.unwrap();
    let c = service.add_category2(user_id, "EXPENSE", "日用品", "Daily goods").await.unwrap();
    service.disable_category2(user_id, "EXPENSE", &b).await.unwrap();

    let before = cat2_codes_in_tree(&service.get_category_tree(user_id, "ja").await.unwrap(), "EXPENSE");
    assert_eq!(before, vec![a.clone(), c.clone()], "precondition: visible order is A, C");

    service.move_category2_up(user_id, "EXPENSE", &c).await.unwrap();

    let after = cat2_codes_in_tree(&service.get_category_tree(user_id, "ja").await.unwrap(), "EXPENSE");
    assert_eq!(
        after,
        vec![c.clone(), a.clone()],
        "one \"up\" click on C must move it above the enabled sibling A, skipping the hidden B"
    );
}

/// Minimal transaction schema (same as the transaction module tests) plus
/// the CATEGORY2/3 i18n tables, for USER 2 with EXPENSE / FOOD / GROCERY.
async fn setup_transaction_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    for stmt in [
        sql_queries::TEST_TRANSACTION_CREATE_USERS_TABLE,
        sql_queries::TEST_TRANSACTION_INSERT_USER,
        sql_queries::TEST_TRANSACTION_CREATE_MEMOS_TABLE,
        sql_queries::TEST_TRANSACTION_CREATE_CATEGORY1_TABLE,
        sql_queries::TEST_TRANSACTION_INSERT_CATEGORY1,
        sql_queries::TEST_TRANSACTION_CREATE_ACCOUNTS_TABLE,
        sql_queries::TEST_TRANSACTION_INSERT_ACCOUNT_CASH,
        sql_queries::TEST_TRANSACTION_INSERT_ACCOUNT_BANK,
        sql_queries::TEST_TRANSACTION_CREATE_HEADER_TABLE,
        sql_queries::TEST_TRANSACTION_CREATE_SHOPS_TABLE,
        sql_queries::TEST_TRANSACTION_CREATE_CATEGORY2_TABLE,
        sql_queries::TEST_TRANSACTION_INSERT_CATEGORY2,
        sql_queries::TEST_TRANSACTION_CREATE_CATEGORY3_TABLE,
        sql_queries::TEST_TRANSACTION_INSERT_CATEGORY3,
        sql_queries::TEST_MANUFACTURER_CREATE_TABLE,
        sql_queries::TEST_PRODUCT_CREATE_TABLE,
        sql_queries::TEST_TRANSACTION_CREATE_DETAIL_TABLE,
        sql_queries::CREATE_RECURRING_RULES_TABLE,
        sql_queries::CREATE_RECURRING_RULE_DETAILS_TABLE,
        sql_queries::TEST_TRANSACTION_CREATE_CATEGORY1_I18N_TABLE,
        sql_queries::TEST_TRANSACTION_CREATE_CATEGORY2_I18N_TABLE,
        sql_queries::TEST_TRANSACTION_CREATE_CATEGORY3_I18N_TABLE,
    ] {
        sqlx::query(stmt).execute(&pool).await.unwrap();
    }
    for (lang, name) in [("ja", "食費"), ("en", "Food")] {
        sqlx::query(sql_queries::CATEGORY2_I18N_INSERT)
            .bind(2_i64)
            .bind("EXPENSE")
            .bind("FOOD")
            .bind(lang)
            .bind(name)
            .execute(&pool)
            .await
            .unwrap();
    }
    for (lang, name) in [("ja", "食料品"), ("en", "Groceries")] {
        sqlx::query(sql_queries::CATEGORY3_I18N_INSERT)
            .bind(2_i64)
            .bind("EXPENSE")
            .bind("FOOD")
            .bind("GROCERY")
            .bind(lang)
            .bind(name)
            .execute(&pool)
            .await
            .unwrap();
    }
    pool
}

/// scan2-M3: renames only touch CATEGORY2_I18N / CATEGORY3_I18N, but
/// `TRANSACTION_DETAIL_GET_WITH_INFO` selects the base CATEGORY2_NAME /
/// CATEGORY3_NAME with no i18n join. After renaming 食費 → 食材 the pickers
/// and aggregation show the new name while the detail list keeps showing
/// 食費 (and base names are EN for user-added categories / JA for seeded
/// ones regardless of the UI language).
///
/// Expected: the detail list shows the CATEGORY1 name and the renamed
/// CATEGORY2 / CATEGORY3 names in the display language passed to
/// `get_transaction_details`, and the base names for a language with no
/// i18n row.
#[tokio::test]
async fn latent_scan2_m3_detail_list_shows_renamed_category_names() {
    use crate::services::transaction::{
        SaveTransactionDetailRequest, SaveTransactionRequest, TransactionService,
    };
    let user_id = 2;
    let pool = setup_transaction_db().await;
    let category = CategoryService::new(pool.clone());
    let transaction = TransactionService::new(pool.clone());

    let txn_id = transaction
        .save_transaction_header(
            user_id,
            SaveTransactionRequest {
                shop_id: None,
                category1_code: "EXPENSE".to_string(),
                from_account_code: "CASH".to_string(),
                to_account_code: "BANK".to_string(),
                transaction_date: "2024-01-01 10:00:00".to_string(),
                total_amount: 1000,
                tax_rounding_type: 0,
                tax_included_type: 0,
                memo: None,
                is_scheduled: None,
            },
        )
        .await
        .unwrap();
    transaction
        .add_transaction_detail(
            user_id,
            txn_id,
            SaveTransactionDetailRequest {
                detail_id: None,
                category1_code: "EXPENSE".to_string(),
                category2_code: Some("FOOD".to_string()),
                category3_code: Some("GROCERY".to_string()),
                item_name: "Rice".to_string(),
                amount: 1000,
                tax_rate: 8,
                tax_amount: 80,
                amount_including_tax: None,
                product_id: None,
                memo: None,
            },
        )
        .await
        .unwrap();

    category
        .update_category2_i18n(user_id, "EXPENSE", "FOOD", "食材", "Ingredients")
        .await
        .unwrap();
    category
        .update_category3_i18n(user_id, "EXPENSE", "FOOD", "GROCERY", "生鮮食品", "Fresh food")
        .await
        .unwrap();

    for (lang, name) in [("ja", "支出"), ("en", "Expense")] {
        sqlx::query(sql_queries::CATEGORY_INSERT_CATEGORY1_I18N)
            .bind(user_id)
            .bind("EXPENSE")
            .bind(lang)
            .bind(name)
            .bind("2024-01-01 00:00:00")
            .execute(&pool)
            .await
            .unwrap();
    }

    // "fr" has no i18n rows: the base names (支出 / 食費 / 食料品) are shown;
    // a rename only touches the i18n rows.
    for (lang, cat1, cat2, cat3) in [
        ("ja", "支出", "食材", "生鮮食品"),
        ("en", "Expense", "Ingredients", "Fresh food"),
        ("fr", "支出", "食費", "食料品"),
    ] {
        let details = transaction.get_transaction_details(user_id, txn_id, lang).await.unwrap();
        assert_eq!(details.len(), 1, "precondition: one detail");
        assert_eq!(
            details[0].category1_name.as_deref(),
            Some(cat1),
            "detail list must show the CATEGORY1 name in {}",
            lang
        );
        assert_eq!(
            details[0].category2_name.as_deref(),
            Some(cat2),
            "detail list must show the CATEGORY2 name in {}",
            lang
        );
        assert_eq!(
            details[0].category3_name.as_deref(),
            Some(cat3),
            "detail list must show the CATEGORY3 name in {}",
            lang
        );
    }
}

/// scan2-M8: `TRANSACTION_LIST_BASE` selected the base CATEGORY1_NAME (支出)
/// with no CATEGORY1_I18N join, so the English transaction list showed 支出
/// on every row.
///
/// Expected: the list shows the CATEGORY1 name of the display language, and
/// falls back to the base name when that language has no row.
#[tokio::test]
async fn latent_scan2_m8_transaction_list_category1_follows_language() {
    use crate::services::transaction::{SaveTransactionRequest, TransactionService};
    let user_id = 2;
    let pool = setup_transaction_db().await;
    let transaction = TransactionService::new(pool.clone());

    for (lang, name) in [("ja", "支出"), ("en", "Expense")] {
        sqlx::query(sql_queries::CATEGORY_INSERT_CATEGORY1_I18N)
            .bind(user_id)
            .bind("EXPENSE")
            .bind(lang)
            .bind(name)
            .bind("2024-01-01 00:00:00")
            .execute(&pool)
            .await
            .unwrap();
    }
    transaction
        .save_transaction_header(
            user_id,
            SaveTransactionRequest {
                shop_id: None,
                category1_code: "EXPENSE".to_string(),
                from_account_code: "CASH".to_string(),
                to_account_code: "BANK".to_string(),
                transaction_date: "2024-01-01 10:00:00".to_string(),
                total_amount: 1000,
                tax_rounding_type: 0,
                tax_included_type: 0,
                memo: None,
                is_scheduled: None,
            },
        )
        .await
        .unwrap();

    // "fr" has no CATEGORY1_I18N row: the base name (支出) is shown.
    for (lang, expected) in [("ja", "支出"), ("en", "Expense"), ("fr", "支出")] {
        let list = transaction
            .get_transactions(
                user_id, None, None, None, None, None, None, None, None, false, 1, 50, lang,
            )
            .await
            .unwrap();
        assert_eq!(list.transactions.len(), 1, "precondition: one transaction");
        assert_eq!(
            list.transactions[0].category1_name.as_deref(),
            Some(expected),
            "CATEGORY1 name in {}",
            lang
        );
    }
}
