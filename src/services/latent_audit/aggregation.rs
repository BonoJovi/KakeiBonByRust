//! Latent-audit (2026-09) regression tests for `services::aggregation`.
//!
//! All tests are `#[ignore]`d TDD-red tests: they assert the CORRECT
//! behaviour and are expected to FAIL against the current code. Run with
//! `cargo test --lib latent_ -- --ignored`.

use super::*;
use chrono::Datelike;

// -----------------------------------------------------------------------------
// Fixtures (copied / extended from `super::tests`)
// -----------------------------------------------------------------------------

async fn setup_db() -> sqlx::SqlitePool {
    let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();

    let create_stmts = [
        "CREATE TABLE USERS (
            USER_ID INTEGER PRIMARY KEY AUTOINCREMENT,
            NAME TEXT NOT NULL UNIQUE,
            PAW TEXT NOT NULL,
            ROLE INTEGER NOT NULL,
            ENTRY_DT TEXT NOT NULL
        )",
        "CREATE TABLE CATEGORY1 (
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE TEXT NOT NULL,
            CATEGORY1_NAME TEXT,
            DISPLAY_ORDER INTEGER,
            PRIMARY KEY (USER_ID, CATEGORY1_CODE)
        )",
        "CREATE TABLE CATEGORY1_I18N (
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE TEXT NOT NULL,
            LANG_CODE TEXT NOT NULL,
            CATEGORY1_NAME_I18N TEXT,
            PRIMARY KEY (USER_ID, CATEGORY1_CODE, LANG_CODE)
        )",
        "CREATE TABLE CATEGORY2 (
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE TEXT NOT NULL,
            CATEGORY2_CODE TEXT NOT NULL,
            DISPLAY_ORDER INTEGER,
            CATEGORY2_NAME TEXT,
            PRIMARY KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE)
        )",
        "CREATE TABLE CATEGORY2_I18N (
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE TEXT NOT NULL,
            CATEGORY2_CODE TEXT NOT NULL,
            LANG_CODE TEXT NOT NULL,
            CATEGORY2_NAME_I18N TEXT,
            PRIMARY KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, LANG_CODE)
        )",
        "CREATE TABLE CATEGORY3 (
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE TEXT NOT NULL,
            CATEGORY2_CODE TEXT NOT NULL,
            CATEGORY3_CODE TEXT NOT NULL,
            DISPLAY_ORDER INTEGER,
            CATEGORY3_NAME TEXT,
            PRIMARY KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE)
        )",
        "CREATE TABLE CATEGORY3_I18N (
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE TEXT NOT NULL,
            CATEGORY2_CODE TEXT NOT NULL,
            CATEGORY3_CODE TEXT NOT NULL,
            LANG_CODE TEXT NOT NULL,
            CATEGORY3_NAME_I18N TEXT,
            PRIMARY KEY (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, LANG_CODE)
        )",
        "CREATE TABLE PRODUCTS (
            USER_ID INTEGER NOT NULL,
            PRODUCT_ID INTEGER NOT NULL,
            PRODUCT_NAME TEXT,
            PRIMARY KEY (USER_ID, PRODUCT_ID)
        )",
        "CREATE TABLE TRANSACTIONS_HEADER (
            USER_ID INTEGER NOT NULL,
            TRANSACTION_ID INTEGER PRIMARY KEY AUTOINCREMENT,
            SHOP_ID INTEGER,
            CATEGORY1_CODE TEXT NOT NULL,
            FROM_ACCOUNT_CODE TEXT,
            TO_ACCOUNT_CODE TEXT,
            TRANSACTION_DATE TEXT NOT NULL,
            TOTAL_AMOUNT INTEGER NOT NULL,
            TAX_ROUNDING_TYPE INTEGER,
            TAX_INCLUDED_TYPE INTEGER NOT NULL DEFAULT 1,
            IS_SCHEDULED INTEGER NOT NULL DEFAULT 0
        )",
        "CREATE TABLE TRANSACTIONS_DETAIL (
            USER_ID INTEGER NOT NULL,
            TRANSACTION_ID INTEGER NOT NULL,
            DETAIL_ID INTEGER NOT NULL,
            CATEGORY1_CODE TEXT NOT NULL,
            CATEGORY2_CODE TEXT,
            CATEGORY3_CODE TEXT,
            PRODUCT_ID INTEGER,
            ITEM_NAME TEXT,
            AMOUNT INTEGER NOT NULL,
            TAX_RATE INTEGER DEFAULT 8,
            TAX_AMOUNT INTEGER DEFAULT 0,
            AMOUNT_INCLUDING_TAX INTEGER,
            MEMO TEXT,
            PRIMARY KEY (USER_ID, TRANSACTION_ID, DETAIL_ID)
        )",
        "INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) \
         VALUES (1, 'tester', 'x', 1, '2024-01-01')",
        "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, CATEGORY1_NAME, DISPLAY_ORDER) \
         VALUES (1, 'EXPENSE', '支出', 1)",
        "INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, DISPLAY_ORDER, CATEGORY2_NAME) \
         VALUES (1, 'EXPENSE', 'FOOD', 1, '食費')",
        "INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, DISPLAY_ORDER, CATEGORY2_NAME) \
         VALUES (1, 'EXPENSE', 'DAILY', 2, '日用品')",
        "INSERT INTO CATEGORY3 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, DISPLAY_ORDER, CATEGORY3_NAME) \
         VALUES (1, 'EXPENSE', 'FOOD', 'RICE', 1, '米')",
        "INSERT INTO PRODUCTS (USER_ID, PRODUCT_ID, PRODUCT_NAME) VALUES (1, 1, '米5kg')",
    ];
    for stmt in create_stmts {
        sqlx::query(stmt).execute(&pool).await.unwrap();
    }
    pool
}

/// One EXPENSE header on 2024-06-15.
async fn insert_header(
    pool: &sqlx::SqlitePool,
    rounding_type: i64,
    tax_included_type: i64,
    total_amount: i64,
) -> i64 {
    use sqlx::Row;
    let row = sqlx::query(
        "INSERT INTO TRANSACTIONS_HEADER \
         (USER_ID, CATEGORY1_CODE, FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE, TRANSACTION_DATE, \
          TOTAL_AMOUNT, TAX_ROUNDING_TYPE, TAX_INCLUDED_TYPE, IS_SCHEDULED) \
         VALUES (1, 'EXPENSE', 'CASH', NULL, '2024-06-15', ?, ?, ?, 0) \
         RETURNING TRANSACTION_ID",
    )
    .bind(total_amount)
    .bind(rounding_type)
    .bind(tax_included_type)
    .fetch_one(pool)
    .await
    .unwrap();
    row.get::<i64, _>("TRANSACTION_ID")
}

#[allow(clippy::too_many_arguments)]
async fn insert_detail_full(
    pool: &sqlx::SqlitePool,
    txn_id: i64,
    detail_id: i64,
    category2: Option<&str>,
    category3: Option<&str>,
    product_id: Option<i64>,
    amount: i64,
    tax_rate: i64,
    amount_including_tax: Option<i64>,
) {
    sqlx::query(
        "INSERT INTO TRANSACTIONS_DETAIL \
         (USER_ID, TRANSACTION_ID, DETAIL_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, \
          PRODUCT_ID, ITEM_NAME, AMOUNT, TAX_RATE, AMOUNT_INCLUDING_TAX) \
         VALUES (1, ?, ?, 'EXPENSE', ?, ?, ?, 'item', ?, ?, ?)",
    )
    .bind(txn_id)
    .bind(detail_id)
    .bind(category2)
    .bind(category3)
    .bind(product_id)
    .bind(amount)
    .bind(tax_rate)
    .bind(amount_including_tax)
    .execute(pool)
    .await
    .unwrap();
}

fn june_2024_request(group_by: GroupBy) -> AggregationRequest {
    let from = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
    let to = NaiveDate::from_ymd_opt(2024, 6, 30).unwrap();
    AggregationRequest::new(1, AggregationFilter::new(DateFilter::Between(from, to)), group_by)
}

async fn run(
    pool: &sqlx::SqlitePool,
    group_by: GroupBy,
) -> Result<Vec<AggregationResult>, String> {
    execute_aggregation(pool, &june_2024_request(group_by), "ja").await
}

fn sum_total(rows: &[AggregationResult]) -> i64 {
    rows.iter().map(|r| r.total_amount).sum()
}

/// Seed: one fully-categorised txn (FOOD/RICE, 1000 @10% → 1100) and one
/// txn whose single detail has NULL CATEGORY2/3 (500 @10% → 550).
async fn seed_h1(pool: &sqlx::SqlitePool) {
    let t1 = insert_header(pool, 0, 1, 1100).await;
    insert_detail_full(pool, t1, 1, Some("FOOD"), Some("RICE"), Some(1), 1000, 10, Some(1100)).await;
    let t2 = insert_header(pool, 0, 1, 550).await;
    insert_detail_full(pool, t2, 1, None, None, None, 500, 10, Some(550)).await;
}

fn assert_unspecified_group(rows: &[AggregationResult], known_key: &str, expected: i64) {
    let known: Vec<_> = rows.iter().filter(|r| r.group_key == known_key).collect();
    assert_eq!(known.len(), 1, "expected one {} row: {:?}", known_key, rows);
    assert_eq!(known[0].total_amount, -1100, "categorised row total: {:?}", rows);
    let unspecified: Vec<_> = rows.iter().filter(|r| r.group_key != known_key).collect();
    assert_eq!(unspecified.len(), 1, "expected exactly one unspecified group: {:?}", rows);
    assert_eq!(
        unspecified[0].group_name, "",
        "unspecified group name must use the '' sentinel (frontend swaps in common.unspecified): {:?}",
        rows
    );
    assert_eq!(unspecified[0].total_amount, expected, "unspecified total: {:?}", rows);
    assert_eq!(sum_total(rows), -1100 + expected);
}

// -----------------------------------------------------------------------------
// H1
// -----------------------------------------------------------------------------
//
// NOTE (audit 2026-09-26): with sqlx 0.8.6 a NULL TEXT column decodes into
// `String` as "" (no decode error), so these H1 tests currently PASS — the
// crash reported from a raw sqlite reproduction does not surface through
// `execute_aggregation`. They are kept as regression guards pinning the
// unspecified-group behaviour ('' sentinel, totals preserved).

/// H1 — A detail with NULL CATEGORY2_CODE makes `group_key` (and
/// `group_name`) NULL, so `String` decoding fails and the whole Category2
/// aggregation errors out (dashboard charts included).
///
/// Expected: aggregation succeeds; the NULL-category rows land in a single
/// "unspecified" group whose `group_name` is `''` (same sentinel convention
/// as Shop / Product), and totals stay correct.
#[tokio::test]
async fn latent_h1_category2_null_code_goes_to_unspecified_group() {
    let pool = setup_db().await;
    seed_h1(&pool).await;

    let rows = run(&pool, GroupBy::Category2)
        .await
        .expect("Category2 aggregation must not fail when a detail has NULL CATEGORY2_CODE");
    assert_unspecified_group(&rows, "EXPENSE/FOOD", -550);
}

/// H1 — Same as above for Category3: NULL CATEGORY3_CODE (and/or NULL
/// CATEGORY2_CODE) breaks the whole Category3 aggregation.
///
/// Expected: aggregation succeeds with an unspecified group (`group_name ''`)
/// carrying the uncategorised amount; totals stay correct.
#[tokio::test]
async fn latent_h1_category3_null_code_goes_to_unspecified_group() {
    let pool = setup_db().await;
    seed_h1(&pool).await;

    let rows = run(&pool, GroupBy::Category3)
        .await
        .expect("Category3 aggregation must not fail when a detail has NULL CATEGORY3_CODE");
    assert_unspecified_group(&rows, "EXPENSE/FOOD/RICE", -550);
}

/// H1 — Partial case: CATEGORY2 set but CATEGORY3 NULL must not break
/// Category3 aggregation either.
///
/// Expected: Ok, and the grand total equals the categorised amounts.
#[tokio::test]
async fn latent_h1_category3_only_code3_null_does_not_fail() {
    let pool = setup_db().await;
    let t1 = insert_header(&pool, 0, 1, 1100).await;
    insert_detail_full(&pool, t1, 1, Some("FOOD"), None, None, 1000, 10, Some(1100)).await;

    let rows = run(&pool, GroupBy::Category3)
        .await
        .expect("Category3 aggregation must not fail when only CATEGORY3_CODE is NULL");
    assert_eq!(sum_total(&rows), -1100, "{:?}", rows);
}

// -----------------------------------------------------------------------------
// M10
// -----------------------------------------------------------------------------

/// M10 (仕様確認待ち: 明細なしヘッダーをどのグループで表現するかは仕様判断) —
/// A header with no details disappears from Category2/Category3/Product
/// aggregation because of the INNER JOIN on TRANSACTIONS_DETAIL, so the
/// trend chart (Category1) and pie chart (Category2) totals disagree.
///
/// Expected: for the same period, the sum over groups for Category2,
/// Category3 and Product each equals the Category1 aggregation total
/// (the detail-less header is counted somewhere, e.g. an unspecified group).
#[tokio::test]
#[ignore = "latent-audit M10"]
async fn latent_m10_detailless_header_counted_in_detail_groupings() {
    let pool = setup_db().await;
    // Fully categorised txn: 1000 @10% → 1100.
    let t1 = insert_header(&pool, 0, 1, 1100).await;
    insert_detail_full(&pool, t1, 1, Some("FOOD"), Some("RICE"), Some(1), 1000, 10, Some(1100)).await;
    // Header-only txn (no details): 500.
    let _t2 = insert_header(&pool, 0, 1, 500).await;

    let cat1 = run(&pool, GroupBy::Category1).await.expect("Category1");
    let cat1_total = sum_total(&cat1);
    assert_eq!(cat1_total, -1600, "sanity: Category1 sees both headers: {:?}", cat1);

    for gb in [GroupBy::Category2, GroupBy::Category3, GroupBy::Product] {
        let label = format!("{:?}", gb);
        let rows = run(&pool, gb).await.unwrap_or_else(|e| panic!("{}: {}", label, e));
        assert_eq!(
            sum_total(&rows),
            cat1_total,
            "{} total must equal Category1 total (detail-less header dropped?): {:?}",
            label,
            rows
        );
    }
}

// -----------------------------------------------------------------------------
// L9
// -----------------------------------------------------------------------------

/// L9 (仕様確認待ち: 修正ではなく差異をドキュメント化する選択肢もある) —
/// Category2 rounds per (txn × group × rate) slice, while the header total
/// rounds once per (txn × rate). Splitting one transaction across groups
/// makes the Category2 sum drift from Category1 by up to (#groups − 1) yen.
///
/// Scenario: floor rounding, 8%, FOOD 999 + DAILY 999.
/// Header recommended total = floor(1998 × 1.08) = 2157;
/// per-group rounding gives 1078 + 1078 = 2156.
///
/// Expected: sum of Category2 groups == Category1 total (== -2157).
#[tokio::test]
#[ignore = "latent-audit L9"]
async fn latent_l9_category2_sum_matches_header_total_across_groups() {
    let pool = setup_db().await;
    let t = insert_header(&pool, /*floor*/ 0, /*excluded*/ 1, 2157).await;
    insert_detail_full(&pool, t, 1, Some("FOOD"), Some("RICE"), Some(1), 999, 8, Some(1078)).await;
    insert_detail_full(&pool, t, 2, Some("DAILY"), Some("RICE"), Some(1), 999, 8, Some(1078)).await;

    let cat1 = run(&pool, GroupBy::Category1).await.expect("Category1");
    assert_eq!(sum_total(&cat1), -2157, "sanity: {:?}", cat1);

    let cat2 = run(&pool, GroupBy::Category2).await.expect("Category2");
    assert_eq!(cat2.len(), 2, "{:?}", cat2);
    assert_eq!(
        sum_total(&cat2),
        sum_total(&cat1),
        "Category2 groups must sum to the header-level total (per-group rounding drift): {:?}",
        cat2
    );
}

// -----------------------------------------------------------------------------
// L11
// -----------------------------------------------------------------------------

/// L11 — `calculate_week_range` (backing `get_weekly_aggregation`) starts
/// week 1 at the first Monday/Sunday ON OR AFTER Jan 1, so the leading days
/// of the year (e.g. 2026-01-01..04, Thu–Sun) belong to no week, and the
/// numbering disagrees with the frontend's ISO-8601 `getWeekNumber`.
///
/// Expected: 2026-01-01 lies inside week 1 (Monday start), and for every
/// day of 2026 whose ISO week-year is 2026, the Monday-start range for its
/// ISO week number contains that day.
#[test]
#[ignore = "latent-audit L11"]
fn latent_l11_weekly_week1_covers_jan1_and_matches_iso() {
    let jan1 = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let (s, e) = calculate_week_range(2026, 1, WeekStart::Monday).expect("week 1");
    assert!(
        s <= jan1 && jan1 <= e,
        "2026-01-01 must fall within week 1 (Monday start); got {}..{}",
        s,
        e
    );

    let mut d = jan1;
    while d.year() == 2026 {
        let iso = d.iso_week();
        if iso.year() == 2026 {
            let (s, e) = calculate_week_range(2026, iso.week(), WeekStart::Monday)
                .expect("week range");
            assert!(
                s <= d && d <= e,
                "{} is ISO week {} but backend week {} range is {}..{}",
                d,
                iso.week(),
                iso.week(),
                s,
                e
            );
        }
        d = d.succ_opt().unwrap();
    }
}

/// L11 — Coverage for both week-start modes: every day of the year must
/// belong to some week number 1..=53 returned by `weekly_aggregation`.
///
/// Expected: no day of 2026 is left uncovered (currently Jan 1–3/4 are).
#[test]
#[ignore = "latent-audit L11"]
fn latent_l11_weekly_every_day_of_year_is_covered() {
    for ws in [WeekStart::Monday, WeekStart::Sunday] {
        let ranges: Vec<(NaiveDate, NaiveDate)> = (1..=53)
            .filter_map(|w| calculate_week_range(2026, w, ws.clone()).ok())
            .collect();
        let mut d = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        while d.year() == 2026 {
            assert!(
                ranges.iter().any(|(s, e)| *s <= d && d <= *e),
                "{} ({:?} start) is not covered by any week 1..=53",
                d,
                ws
            );
            d = d.succ_opt().unwrap();
        }
    }
}

// -----------------------------------------------------------------------------
// H5 (aggregation side)
// -----------------------------------------------------------------------------

/// H5 (aggregation side) — Under a tax-included header (TAX_INCLUDED_TYPE=0)
/// the detail query treats AMOUNT as already tax-included, but the form
/// always saves AMOUNT tax-excluded, so Category2 under-reports by the tax.
///
/// Spec decided 2026-09-26: AMOUNT is ALWAYS tax-excluded; a tax-included
/// header's total = SUM(AMOUNT_INCLUDING_TAX).
///
/// Expected: AMOUNT=1000, AMOUNT_INCLUDING_TAX=1100, TAX_RATE=10 under an
/// included header aggregates as 1100 (EXPENSE → -1100) in Category2.
#[tokio::test]
#[ignore = "latent-audit H5"]
async fn latent_h5_included_header_category2_uses_amount_including_tax() {
    let pool = setup_db().await;
    let t = insert_header(&pool, 0, /*tax_included*/ 0, 1100).await;
    insert_detail_full(&pool, t, 1, Some("FOOD"), Some("RICE"), Some(1), 1000, 10, Some(1100)).await;

    let rows = run(&pool, GroupBy::Category2).await.expect("Category2");
    assert_eq!(rows.len(), 1, "{:?}", rows);
    assert_eq!(
        rows[0].total_amount, -1100,
        "included header must aggregate SUM(AMOUNT_INCLUDING_TAX), not tax-excluded AMOUNT"
    );
}
