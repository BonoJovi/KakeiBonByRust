//! Latent-audit regression tests (Opus 5.5, 2026-09-26) — transaction domain.
//!
//! Every test here asserts the CORRECT behaviour for a latent bug from
//! `work/bug_list_opus_5_5_latent_audit_2026-09-26.md`, so it FAILS on the
//! current code (TDD red phase) and is `#[ignore]`d until the fix lands.
//! Run with: `cargo test --lib latent_ -- --ignored`.

use super::*;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use crate::{consts, sql_queries};

// ============================================================================
// Helpers (copied from the private `mod tests` in transaction.rs)
// ============================================================================

const USER: i64 = 2;
const OTHER_USER: i64 = 3;

/// Build the same minimal schema as `tests::setup_test_db` on `pool`.
async fn init_schema(pool: &SqlitePool) {
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
    ] {
        sqlx::query(stmt).execute(pool).await.unwrap();
    }
}

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("Failed to create test database");
    init_schema(&pool).await;
    pool
}

async fn setup_test_db_with_foreign_keys() -> SqlitePool {
    let pool = setup_test_db().await;
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .unwrap();
    pool
}

fn header_request(total: i64, rounding: i64, included: i64) -> SaveTransactionRequest {
    SaveTransactionRequest {
        shop_id: None,
        category1_code: "EXPENSE".to_string(),
        from_account_code: "CASH".to_string(),
        to_account_code: "BANK".to_string(),
        transaction_date: "2024-01-01 10:00:00".to_string(),
        total_amount: total,
        tax_rounding_type: rounding,
        tax_included_type: included,
        memo: None,
        is_scheduled: None,
    }
}

fn detail_request(amount: i64, tax_rate: i32, tax: i64, incl: Option<i64>) -> SaveTransactionDetailRequest {
    SaveTransactionDetailRequest {
        detail_id: None,
        category1_code: "EXPENSE".to_string(),
        category2_code: None,
        category3_code: None,
        item_name: "Item".to_string(),
        amount,
        tax_rate,
        tax_amount: tax,
        amount_including_tax: incl,
        product_id: None,
        memo: None,
    }
}

fn d(amount: i64, including: Option<i64>, rate: i64) -> DetailForRecalc {
    DetailForRecalc {
        amount,
        amount_including_tax: including,
        tax_rate: rate,
    }
}

async fn header_cols(pool: &SqlitePool, txn_id: i64) -> (i64, i64, i64) {
    let row = sqlx::query(
        "SELECT TOTAL_AMOUNT, TAX_ROUNDING_TYPE, TAX_INCLUDED_TYPE FROM TRANSACTIONS_HEADER WHERE TRANSACTION_ID = ?",
    )
    .bind(txn_id)
    .fetch_one(pool)
    .await
    .unwrap();
    (row.get(0), row.get(1), row.get(2))
}

// ----------------------------------------------------------------------------
// HOME sandbox for the bulk-recalc / restore paths.
//
// `recalculate_all_transaction_totals` copies `crate::db::get_db_path()`
// (derived from $HOME) to a backup file, and `restore_totals_from_backup`
// only accepts backups next to that path. To keep the tests off the real
// ~/.kakeibon, point HOME at a fresh temp dir and connect a single-connection
// pool to the DB file living there. HOME is process-global, so every test
// that touches it serialises on HOME_LOCK and restores the original value.
// ----------------------------------------------------------------------------

static HOME_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct HomeSandbox {
    _lock: std::sync::MutexGuard<'static, ()>,
    original_home: Option<std::ffi::OsString>,
    _dir: tempfile::TempDir,
    db_dir: std::path::PathBuf,
}

impl Drop for HomeSandbox {
    fn drop(&mut self) {
        match &self.original_home {
            Some(h) => std::env::set_var("HOME", h),
            None => std::env::remove_var("HOME"),
        }
    }
}

/// Returns the sandbox guard plus a 1-connection pool on
/// `$HOME/.kakeibon/KakeiBonDB.sqlite3` with the test schema applied.
async fn sandboxed_file_db() -> (HomeSandbox, SqlitePool) {
    let lock = HOME_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original_home = std::env::var_os("HOME");
    let dir = tempfile::tempdir().expect("tempdir");
    std::env::set_var("HOME", dir.path());
    let db_path = crate::db::get_db_path();
    let db_dir = db_path.parent().unwrap().to_path_buf();
    std::fs::create_dir_all(&db_dir).unwrap();
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!("sqlite://{}?mode=rwc", db_path.display()))
        .await
        .expect("connect sandbox db");
    init_schema(&pool).await;
    (
        HomeSandbox {
            _lock: lock,
            original_home,
            _dir: dir,
            db_dir,
        },
        pool,
    )
}

// ============================================================================
// H4
// ============================================================================

/// H4: 一括再計算で明細なしヘッダーの TOTAL_AMOUNT が 0 に上書きされる。
/// Expected: 明細が 1 件も無いヘッダーは再計算対象外 (TOTAL_AMOUNT はユーザー入力値のまま)。
#[tokio::test]
async fn latent_h4_bulk_recalc_keeps_total_without_details() {
    let (_home, pool) = sandboxed_file_db().await;
    let service = TransactionService::new(pool.clone());
    let txn_id = service
        .save_transaction_header(USER, header_request(5000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();

    service.recalculate_all_transaction_totals(USER).await.unwrap();

    let (total, _, _) = header_cols(&pool, txn_id).await;
    assert_eq!(total, 5000, "header without details must keep its TOTAL_AMOUNT after bulk recalc");
}

// ============================================================================
// H5 / L1 — AMOUNT is always tax-excluded (owner decision 2026-09-26)
// ============================================================================

/// H5: 内税ヘッダー用の合計計算が明細 AMOUNT (税抜) をそのまま SUM している。
/// Expected: 内税ヘッダー (TAX_INCLUDED_TYPE=0) の合計 = SUM(AMOUNT_INCLUDING_TAX)。
#[test]
fn latent_h5_included_header_total_sums_amount_including_tax() {
    // AMOUNT is tax-excluded; AMOUNT_INCLUDING_TAX carries the per-row gross.
    let details = vec![d(1000, Some(1080), 8), d(500, Some(550), 10), d(333, Some(359), 8)];
    assert_eq!(
        calculate_recommended_total_with_settings(&details, consts::TAX_ROUND_DOWN, consts::TAX_INCLUDED),
        1080 + 550 + 359,
    );
}

/// H5: compute_recommended_total がヘッダーの TAX_INCLUDED_TYPE を見ず常に gross-up する。
/// Expected: 内税ヘッダーでは SUM(AMOUNT_INCLUDING_TAX) を返す (333+333 @8% → 359+359=718, gross-up だと 719)。
#[tokio::test]
async fn latent_h5_compute_recommended_total_honours_included_header() {
    let pool = setup_test_db().await;
    let service = TransactionService::new(pool);
    let txn_id = service
        .save_transaction_header(USER, header_request(718, consts::TAX_ROUND_DOWN, consts::TAX_INCLUDED))
        .await
        .unwrap();
    for _ in 0..2 {
        service
            .add_transaction_detail(USER, txn_id, detail_request(333, 8, 26, Some(359)))
            .await
            .unwrap();
    }

    let recommended = service.compute_recommended_total(USER, txn_id).await.unwrap();
    assert_eq!(recommended, Some(718), "tax-included header total must be SUM(AMOUNT_INCLUDING_TAX)");
}

/// H5: 一括再計算が内税ヘッダーを「外税」と誤判定し TAX_INCLUDED_TYPE を黙って書き換える。
/// Expected: TOTAL_AMOUNT = SUM(AMOUNT_INCLUDING_TAX) の整合した内税ヘッダーは変更されない。
#[tokio::test]
async fn latent_h5_bulk_recalc_keeps_consistent_included_header() {
    let (_home, pool) = sandboxed_file_db().await;
    let service = TransactionService::new(pool.clone());
    let txn_id = service
        .save_transaction_header(USER, header_request(1100, consts::TAX_ROUND_DOWN, consts::TAX_INCLUDED))
        .await
        .unwrap();
    service
        .add_transaction_detail(USER, txn_id, detail_request(1000, 10, 100, Some(1100)))
        .await
        .unwrap();

    let summary = service.recalculate_all_transaction_totals(USER).await.unwrap();

    let (total, rounding, included) = header_cols(&pool, txn_id).await;
    assert_eq!(
        (total, rounding, included),
        (1100, consts::TAX_ROUND_DOWN, consts::TAX_INCLUDED),
        "consistent tax-included header must be left untouched (changes: {:?})",
        summary.changes
    );
}

/// L1: 「AMOUNT == AMOUNT_INCLUDING_TAX なら税込扱い」の判定が、税額 0 円の少額明細を誤分類する。
/// Expected: 外税ヘッダーは税率ごとに SUM(AMOUNT) を gross-up して 1 回丸める
/// (5+10 @8% = 15×1.08 = 16.2 → floor 16。現状は両明細を税込扱いして 15)。
#[test]
fn latent_l1_small_detail_with_zero_tax_is_still_grossed_up() {
    let details = vec![d(5, Some(5), 8), d(10, Some(10), 8)];
    assert_eq!(
        calculate_recommended_total_with_settings(&details, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED),
        16,
    );
}

// ============================================================================
// M1
// ============================================================================

/// M1: TRANSACTION_HEADER_UPDATE に IS_SCHEDULED が無く、予定チェックの変更が黙って捨てられる。
/// Expected: update_transaction_header(is_scheduled=Some(1)) 後、ヘッダーの IS_SCHEDULED は 1。
#[tokio::test]
#[ignore = "latent-audit M1"]
async fn latent_m1_update_header_persists_is_scheduled() {
    let pool = setup_test_db().await;
    let service = TransactionService::new(pool);
    let txn_id = service
        .save_transaction_header(USER, header_request(1000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();

    let mut req = header_request(1000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED);
    req.is_scheduled = Some(1);
    service.update_transaction_header(USER, txn_id, req).await.unwrap();

    let header = service.get_transaction_header(USER, txn_id).await.unwrap();
    assert_eq!(header.is_scheduled, 1, "IS_SCHEDULED change on update must be persisted");
}

// ============================================================================
// M2 (仕様確認待ち)
// ============================================================================

/// M2 (仕様確認待ち): ヘッダーの CATEGORY1 変更が明細の CATEGORY1_CODE に追従しない。
/// Expected: 更新後、ヘッダーと全明細の CATEGORY1_CODE が一致する
/// (明細へ伝播する / 明細があれば更新を拒否する、のどちらでも可。不整合状態だけを拒否)。
#[tokio::test]
#[ignore = "latent-audit M2"]
async fn latent_m2_header_category1_change_keeps_details_consistent() {
    let pool = setup_test_db().await;
    sqlx::query("INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, CATEGORY1_NAME) VALUES (2, 'INCOME', '収入')")
        .execute(&pool)
        .await
        .unwrap();
    let service = TransactionService::new(pool.clone());
    let txn_id = service
        .save_transaction_header(USER, header_request(1080, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();
    service
        .add_transaction_detail(USER, txn_id, detail_request(1000, 8, 80, Some(1080)))
        .await
        .unwrap();

    let mut req = header_request(1080, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED);
    req.category1_code = "INCOME".to_string();
    let _ = service.update_transaction_header(USER, txn_id, req).await; // Ok or Err both acceptable

    let header = service.get_transaction_header(USER, txn_id).await.unwrap();
    let detail_codes: Vec<String> =
        sqlx::query_scalar("SELECT CATEGORY1_CODE FROM TRANSACTIONS_DETAIL WHERE TRANSACTION_ID = ?")
            .bind(txn_id)
            .fetch_all(&pool)
            .await
            .unwrap();
    assert!(!detail_codes.is_empty());
    for code in &detail_codes {
        assert_eq!(
            code, &header.category1_code,
            "detail CATEGORY1_CODE must match header after category1 update"
        );
    }
}

// ============================================================================
// M9
// ============================================================================

/// M9: 一括再計算のロールバックが TOTAL_AMOUNT しか戻さず、再計算で変更された税設定が残る。
/// Expected: restore 後、再計算で書き換えられた TAX_ROUNDING_TYPE / TAX_INCLUDED_TYPE も元に戻る。
#[tokio::test]
#[ignore = "latent-audit M9"]
async fn latent_m9_restore_reverts_tax_settings_changed_by_recalc() {
    let (_home, pool) = sandboxed_file_db().await;
    let service = TransactionService::new(pool.clone());
    // 105 @10% = 115.5 → FLOOR 115 / HALF_UP 116. Saved as FLOOR with 116,
    // so recalc "corrects" the rounding to HALF_UP.
    let txn_id = service
        .save_transaction_header(USER, header_request(116, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();
    service
        .add_transaction_detail(USER, txn_id, detail_request(105, 10, 10, Some(115)))
        .await
        .unwrap();

    let summary = service.recalculate_all_transaction_totals(USER).await.unwrap();
    let (_, rounding_after_recalc, _) = header_cols(&pool, txn_id).await;
    assert_eq!(
        rounding_after_recalc,
        consts::TAX_ROUND_HALF_UP,
        "precondition: recalc should have corrected the rounding type"
    );

    service
        .restore_totals_from_backup(USER, &summary.backup_path)
        .await
        .unwrap();

    let (total, rounding, included) = header_cols(&pool, txn_id).await;
    assert_eq!(
        (total, rounding, included),
        (116, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED),
        "rollback must restore the tax settings the recalc changed"
    );
}

/// M9: ロールバックが全ヘッダーの TOTAL_AMOUNT をバックアップ値で上書きし、再計算後の手修正を消す。
/// Expected: 再計算で変更されなかったヘッダーに再計算後に加えた修正は、restore 後も保持される
/// (restore_totals_from_backup の doc コメント「再計算後の入力を消さない」の契約)。
#[tokio::test]
#[ignore = "latent-audit M9"]
async fn latent_m9_restore_keeps_edits_made_after_recalc() {
    let (_home, pool) = sandboxed_file_db().await;
    let service = TransactionService::new(pool.clone());
    // Consistent header: 100 @10% = 110 → skipped by recalc.
    let txn_id = service
        .save_transaction_header(USER, header_request(110, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();
    service
        .add_transaction_detail(USER, txn_id, detail_request(100, 10, 10, Some(110)))
        .await
        .unwrap();

    let summary = service.recalculate_all_transaction_totals(USER).await.unwrap();
    assert!(summary.changes.is_empty(), "precondition: recalc should not touch this header");

    // User edits the total after the recalc.
    service
        .update_transaction_header_total(USER, txn_id, 999)
        .await
        .unwrap();

    service
        .restore_totals_from_backup(USER, &summary.backup_path)
        .await
        .unwrap();

    let (total, _, _) = header_cols(&pool, txn_id).await;
    assert_eq!(total, 999, "rollback must not overwrite edits made after the recalc");
}

// ============================================================================
// L2
// ============================================================================

/// L2: ヘッダー保存時に SHOP_ID の所有者検証が無く、他ユーザーの店舗を紐付けられる。
/// Expected: 他ユーザーの SHOP_ID を指定した save は Err。
#[tokio::test]
#[ignore = "latent-audit L2"]
async fn latent_l2_save_header_rejects_foreign_shop_id() {
    let pool = setup_test_db().await;
    let foreign_shop: i64 = sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (?, '他人の店')")
        .bind(OTHER_USER)
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let service = TransactionService::new(pool);

    let mut req = header_request(1000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED);
    req.shop_id = Some(foreign_shop);
    let result = service.save_transaction_header(USER, req).await;
    assert!(result.is_err(), "another user's SHOP_ID must be rejected, got {:?}", result);
}

/// L2: ヘッダー更新時に SHOP_ID の所有者検証が無い。
/// Expected: 他ユーザーの SHOP_ID を指定した update は Err。
#[tokio::test]
#[ignore = "latent-audit L2"]
async fn latent_l2_update_header_rejects_foreign_shop_id() {
    let pool = setup_test_db().await;
    let foreign_shop: i64 = sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (?, '他人の店')")
        .bind(OTHER_USER)
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let service = TransactionService::new(pool);
    let txn_id = service
        .save_transaction_header(USER, header_request(1000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();

    let mut req = header_request(1000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED);
    req.shop_id = Some(foreign_shop);
    let result = service.update_transaction_header(USER, txn_id, req).await;
    assert!(result.is_err(), "another user's SHOP_ID must be rejected on update");
}

/// L2: 明細追加時に PRODUCT_ID の所有者検証が無く、他ユーザーの商品を紐付けられる。
/// Expected: 他ユーザーの PRODUCT_ID を指定した add_transaction_detail は Err。
#[tokio::test]
#[ignore = "latent-audit L2"]
async fn latent_l2_add_detail_rejects_foreign_product_id() {
    let pool = setup_test_db().await;
    sqlx::query("INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) VALUES (?, 'otheruser', 'hash', 1, datetime('now'))")
        .bind(OTHER_USER)
        .execute(&pool)
        .await
        .unwrap();
    let foreign_product: i64 =
        sqlx::query("INSERT INTO PRODUCTS (USER_ID, PRODUCT_NAME) VALUES (?, '他人の商品')")
            .bind(OTHER_USER)
            .execute(&pool)
            .await
            .unwrap()
            .last_insert_rowid();
    let service = TransactionService::new(pool);
    let txn_id = service
        .save_transaction_header(USER, header_request(1080, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();

    let mut req = detail_request(1000, 8, 80, Some(1080));
    req.product_id = Some(foreign_product);
    let result = service.add_transaction_detail(USER, txn_id, req).await;
    assert!(result.is_err(), "another user's PRODUCT_ID must be rejected, got {:?}", result);
}

/// L2: TRANSACTION_HEADER_GET_WITH_INFO の SHOPS JOIN に USER_ID 条件が無く、他ユーザーの店舗名が漏れる。
/// Expected: ヘッダーの SHOP_ID が他ユーザーの店舗を指していても shop_name は None。
#[tokio::test]
#[ignore = "latent-audit L2"]
async fn latent_l2_header_with_info_does_not_leak_foreign_shop_name() {
    let pool = setup_test_db().await;
    let foreign_shop: i64 = sqlx::query("INSERT INTO SHOPS (USER_ID, SHOP_NAME) VALUES (?, '他人の店')")
        .bind(OTHER_USER)
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let service = TransactionService::new(pool.clone());
    let txn_id = service
        .save_transaction_header(USER, header_request(1000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();
    // Simulate a legacy / direct-invoke row pointing at the other user's shop.
    sqlx::query("UPDATE TRANSACTIONS_HEADER SET SHOP_ID = ? WHERE TRANSACTION_ID = ?")
        .bind(foreign_shop)
        .bind(txn_id)
        .execute(&pool)
        .await
        .unwrap();

    let info = service.get_transaction_header_with_info(USER, txn_id).await.unwrap();
    assert_eq!(info.shop_name, None, "another user's SHOP_NAME must not be exposed");
}

// ============================================================================
// L3
// ============================================================================

async fn memo_text_of_detail(pool: &SqlitePool, detail_id: i64) -> Option<String> {
    sqlx::query_scalar(
        "SELECT m.MEMO_TEXT FROM TRANSACTIONS_DETAIL d LEFT JOIN MEMOS m ON m.MEMO_ID = d.MEMO_ID WHERE d.DETAIL_ID = ?",
    )
    .bind(detail_id)
    .fetch_one(pool)
    .await
    .unwrap()
}

/// L3: update_transaction_detail のメモ更新 (MEMO_UPDATE) が DETAIL 更新の tx 外で先に確定する。
/// Expected: DETAIL 更新が失敗したら、メモ本文の変更もロールバックされる (元の本文のまま)。
#[tokio::test]
#[ignore = "latent-audit L3"]
async fn latent_l3_failed_detail_update_rolls_back_memo_change() {
    let pool = setup_test_db_with_foreign_keys().await;
    let service = TransactionService::new(pool.clone());
    let txn_id = service
        .save_transaction_header(USER, header_request(1080, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();
    let mut add = detail_request(1000, 8, 80, Some(1080));
    add.memo = Some("original".to_string());
    let detail_id = service.add_transaction_detail(USER, txn_id, add).await.unwrap();

    // CATEGORY1_CODE that is not seeded → composite FK makes DETAIL_UPDATE fail.
    let mut upd = detail_request(1000, 8, 80, Some(1080));
    upd.category1_code = "NO_SUCH_CATEGORY".to_string();
    upd.memo = Some("changed".to_string());
    let result = service.update_transaction_detail(USER, detail_id, upd).await;
    assert!(result.is_err(), "precondition: detail update must fail on FK");

    assert_eq!(
        memo_text_of_detail(&pool, detail_id).await.as_deref(),
        Some("original"),
        "memo text must not change when the detail update fails"
    );
}

/// L3: in-place の MEMO_UPDATE が trim していない値を書く (新規作成経路は trim する)。
/// Expected: 前後空白付きのメモで更新しても、保存される MEMO_TEXT は trim 済み。
#[tokio::test]
#[ignore = "latent-audit L3"]
async fn latent_l3_in_place_memo_update_is_trimmed() {
    let pool = setup_test_db().await;
    let service = TransactionService::new(pool.clone());
    let txn_id = service
        .save_transaction_header(USER, header_request(1080, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();
    let mut add = detail_request(1000, 8, 80, Some(1080));
    add.memo = Some("original".to_string());
    let detail_id = service.add_transaction_detail(USER, txn_id, add).await.unwrap();

    let mut upd = detail_request(1000, 8, 80, Some(1080));
    upd.memo = Some("  updated  ".to_string());
    service.update_transaction_detail(USER, detail_id, upd).await.unwrap();

    assert_eq!(
        memo_text_of_detail(&pool, detail_id).await.as_deref(),
        Some("updated"),
        "memo text must be stored trimmed, like the create path"
    );
}

// ============================================================================
// L4
// ============================================================================

/// L4: restore_totals_from_backup の UPDATE 失敗時に DETACH されず、接続に recalc_backup が残る
/// (同じ接続での次回 restore は ATTACH 失敗)。
/// Expected: UPDATE が失敗しても recalc_backup は DETACH 済みで、接続に残らない。
#[tokio::test]
#[ignore = "latent-audit L4"]
async fn latent_l4_restore_detaches_backup_when_update_fails() {
    let (home, pool) = sandboxed_file_db().await;
    let service = TransactionService::new(pool.clone());
    // An empty file ATTACHes fine but has no TRANSACTIONS_HEADER → UPDATE fails.
    let bogus = home.db_dir.join("KakeiBonDB.sqlite3.backup_bogus");
    std::fs::write(&bogus, b"").unwrap();

    let result = service
        .restore_totals_from_backup(USER, bogus.to_str().unwrap())
        .await;
    assert!(result.is_err(), "precondition: restore from a table-less backup must fail");

    // Single-connection pool → this runs on the connection the restore used.
    let names: Vec<String> = sqlx::query("PRAGMA database_list")
        .fetch_all(&pool)
        .await
        .unwrap()
        .iter()
        .map(|r| r.get::<String, _>("name"))
        .collect();
    assert!(
        !names.iter().any(|n| n == "recalc_backup"),
        "recalc_backup must be detached after a failed restore, attached: {:?}",
        names
    );
}

// ============================================================================
// L8
// ============================================================================

/// L8: 取引日時のバックエンド検証が「長さ 19」だけで、不正な日時文字列を保存できる。
/// Expected: 19 文字でも YYYY-MM-DD HH:MM:SS として不正な値は save で Err。
#[tokio::test]
#[ignore = "latent-audit L8"]
async fn latent_l8_save_header_rejects_malformed_datetime() {
    let pool = setup_test_db().await;
    let service = TransactionService::new(pool);
    for bad in ["2024-13-45 99:99:99", "abcdefghijklmnopqrs", "                   "] {
        let mut req = header_request(1000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED);
        req.transaction_date = bad.to_string();
        let result = service.save_transaction_header(USER, req).await;
        assert!(result.is_err(), "malformed datetime {:?} must be rejected on save", bad);
    }
}

/// L8: update_transaction_header も長さ 19 のみの検証。
/// Expected: 19 文字でも不正な日時文字列は update で Err。
#[tokio::test]
#[ignore = "latent-audit L8"]
async fn latent_l8_update_header_rejects_malformed_datetime() {
    let pool = setup_test_db().await;
    let service = TransactionService::new(pool);
    let txn_id = service
        .save_transaction_header(USER, header_request(1000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();
    let mut req = header_request(1000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED);
    req.transaction_date = "2024-02-30 25:61:61".to_string();
    let result = service.update_transaction_header(USER, txn_id, req).await;
    assert!(result.is_err(), "malformed datetime must be rejected on update");
}

/// H4 (frontend contract): `compute_recommended_total` reports "nothing to
/// recommend" (`None` → JSON `null`) for a header without details instead of
/// `0`, so the edit flow never offers to overwrite the total with ¥0.
#[tokio::test]
async fn latent_h4_compute_recommended_total_is_none_without_details() {
    let pool = setup_test_db().await;
    let service = TransactionService::new(pool);
    let txn_id = service
        .save_transaction_header(USER, header_request(5000, consts::TAX_ROUND_DOWN, consts::TAX_EXCLUDED))
        .await
        .unwrap();

    assert_eq!(service.compute_recommended_total(USER, txn_id).await.unwrap(), None);
}
