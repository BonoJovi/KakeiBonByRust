//! Latent-audit 2026-09 regression tests for `services::recurring`.
//!
//! TDD red phase: every test asserts the CORRECT behaviour and is expected to
//! FAIL against the current code. All are `#[ignore]`d so the normal suite
//! stays green; run them with `cargo test --lib latent_ -- --ignored`.
//!
//! Schema note: the shared `test_helpers::database::setup_test_db` schema
//! lacks `TRANSACTIONS_HEADER.IS_SCHEDULED` and the migrated DETAIL columns,
//! and the transaction-test schema lacks `RULE_ID`. These tests therefore
//! build a minimal FK-light in-memory schema that matches the columns the
//! recurring write path actually touches (test-only SQL kept local to this
//! audit module).

use super::*;
use chrono::Local;

const USER_ID: i64 = 2;

async fn setup_recurring_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    let ddl = [
        "CREATE TABLE USERS (
            USER_ID INTEGER PRIMARY KEY,
            NAME TEXT NOT NULL DEFAULT 'u',
            HOLIDAY_LOCALE TEXT DEFAULT 'JP'
        )",
        "CREATE TABLE MEMOS (
            MEMO_ID INTEGER PRIMARY KEY AUTOINCREMENT,
            USER_ID INTEGER NOT NULL,
            MEMO_TEXT TEXT NOT NULL,
            ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
            UPDATE_DT DATETIME
        )",
        "CREATE TABLE RECURRING_RULES (
            RULE_ID INTEGER PRIMARY KEY AUTOINCREMENT,
            USER_ID INTEGER NOT NULL,
            RULE_NAME TEXT,
            PERIOD_UNIT TEXT NOT NULL,
            PERIOD_INTERVAL INTEGER NOT NULL,
            ANCHOR_DATE DATE,
            DAY_OF_WEEK INTEGER,
            MONTH_DAY_RULE_TYPE TEXT,
            DAY_OF_MONTH INTEGER,
            WEEK_OF_MONTH INTEGER,
            MONTH_OF_YEAR INTEGER,
            HOLIDAY_SHIFT_TYPE INTEGER DEFAULT 0,
            START_DATE DATE NOT NULL,
            END_DATE DATE NOT NULL,
            SHOP_ID INTEGER,
            CATEGORY1_CODE VARCHAR(50) NOT NULL,
            FROM_ACCOUNT_CODE VARCHAR(50) NOT NULL,
            TO_ACCOUNT_CODE VARCHAR(50) NOT NULL,
            TOTAL_AMOUNT INTEGER NOT NULL,
            TAX_ROUNDING_TYPE INTEGER DEFAULT 0,
            TAX_INCLUDED_TYPE INTEGER DEFAULT 1 NOT NULL,
            MEMO_ID INTEGER,
            IS_DISABLED INTEGER DEFAULT 0,
            ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now', 'localtime')),
            UPDATE_DT DATETIME
        )",
        "CREATE TABLE RECURRING_RULE_DETAILS (
            RULE_DETAIL_ID INTEGER PRIMARY KEY AUTOINCREMENT,
            RULE_ID INTEGER NOT NULL UNIQUE,
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE VARCHAR(50) NOT NULL,
            CATEGORY2_CODE VARCHAR(50),
            CATEGORY3_CODE VARCHAR(50),
            ITEM_NAME TEXT NOT NULL,
            AMOUNT INTEGER NOT NULL,
            TAX_AMOUNT INTEGER DEFAULT 0,
            TAX_RATE INTEGER DEFAULT 8,
            AMOUNT_INCLUDING_TAX INTEGER,
            MEMO_ID INTEGER,
            ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
            UPDATE_DT DATETIME,
            FOREIGN KEY (RULE_ID) REFERENCES RECURRING_RULES(RULE_ID) ON DELETE CASCADE
        )",
        "CREATE TABLE TRANSACTIONS_HEADER (
            TRANSACTION_ID INTEGER PRIMARY KEY AUTOINCREMENT,
            USER_ID INTEGER NOT NULL,
            SHOP_ID INTEGER,
            CATEGORY1_CODE VARCHAR(50) NOT NULL,
            FROM_ACCOUNT_CODE VARCHAR(50) NOT NULL,
            TO_ACCOUNT_CODE VARCHAR(50) NOT NULL,
            TRANSACTION_DATE DATETIME NOT NULL,
            TOTAL_AMOUNT INTEGER NOT NULL,
            TAX_ROUNDING_TYPE INTEGER DEFAULT 0,
            TAX_INCLUDED_TYPE INTEGER DEFAULT 1 NOT NULL,
            MEMO_ID INTEGER,
            IS_DISABLED INTEGER DEFAULT 0,
            IS_SCHEDULED INTEGER DEFAULT 0,
            RULE_ID INTEGER,
            ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
            UPDATE_DT DATETIME,
            FOREIGN KEY (RULE_ID) REFERENCES RECURRING_RULES(RULE_ID) ON DELETE SET NULL
        )",
        "CREATE TABLE TRANSACTIONS_DETAIL (
            DETAIL_ID INTEGER PRIMARY KEY AUTOINCREMENT,
            TRANSACTION_ID INTEGER NOT NULL,
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE VARCHAR(50) NOT NULL,
            CATEGORY2_CODE VARCHAR(50),
            CATEGORY3_CODE VARCHAR(50),
            ITEM_NAME TEXT NOT NULL,
            AMOUNT INTEGER NOT NULL,
            TAX_AMOUNT INTEGER DEFAULT 0,
            TAX_RATE INTEGER DEFAULT 8,
            AMOUNT_INCLUDING_TAX INTEGER,
            PRODUCT_ID INTEGER,
            MEMO_ID INTEGER,
            ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
            UPDATE_DT DATETIME,
            FOREIGN KEY (TRANSACTION_ID) REFERENCES TRANSACTIONS_HEADER(TRANSACTION_ID) ON DELETE CASCADE
        )",
        sql_queries::CREATE_HOLIDAYS_STANDARD_TABLE,
        sql_queries::CREATE_HOLIDAYS_USER_CUSTOM_TABLE,
    ];
    for stmt in ddl {
        sqlx::query(stmt).execute(&pool).await.unwrap();
    }
    sqlx::query("INSERT INTO USERS (USER_ID, NAME) VALUES (?, 'testuser')")
        .bind(USER_ID)
        .execute(&pool)
        .await
        .unwrap();
    pool
}

/// A request the current code accepts end-to-end against `setup_recurring_db`
/// (DAY x1, 3 occurrences 2026-01-01..03).
fn valid_request() -> SaveRecurringRuleRequest {
    SaveRecurringRuleRequest {
        rule_name: None,
        period_unit: consts::PERIOD_UNIT_DAY.to_string(),
        period_interval: 1,
        anchor_date: Some("2026-01-01".to_string()),
        day_of_week: None,
        month_day_rule_type: None,
        day_of_month: None,
        week_of_month: None,
        month_of_year: None,
        holiday_shift_type: consts::HOLIDAY_SHIFT_NONE,
        start_date: "2026-01-01".to_string(),
        end_date: "2026-01-03".to_string(),
        shop_id: None,
        category1_code: "EXPENSE".to_string(),
        from_account_code: "BANK".to_string(),
        to_account_code: "NONE".to_string(),
        total_amount: 100,
        tax_rounding_type: consts::TAX_ROUND_DOWN,
        tax_included_type: consts::TAX_EXCLUDED,
        header_memo: None,
        detail: SaveRecurringRuleDetailRequest {
            category1_code: "EXPENSE".to_string(),
            category2_code: None,
            category3_code: None,
            item_name: "test".to_string(),
            amount: 100,
            tax_amount: 0,
            tax_rate: 0,
            amount_including_tax: Some(100),
            product_id: None,
            detail_memo: None,
        },
    }
}

/// Sanity guard: the unmodified request must succeed in this schema, so a
/// later `is_err()` on the mutated request is attributable to validation and
/// not to a broken fixture.
async fn assert_baseline_ok(service: &RecurringService) {
    let r = service
        .create_rule_with_instances(USER_ID, valid_request())
        .await;
    assert!(r.is_ok(), "fixture baseline must succeed, got {:?}", r.err());
}

async fn count_headers_for_rule(pool: &SqlitePool, rule_id: i64) -> i64 {
    sqlx::query_scalar("SELECT COUNT(*) FROM TRANSACTIONS_HEADER WHERE RULE_ID = ?")
        .bind(rule_id)
        .fetch_one(pool)
        .await
        .unwrap()
}

// ---------------------------------------------------------------------------
// H2
// ---------------------------------------------------------------------------

/// H2: `delete_rule(cascade = true)` deletes every TRANSACTIONS_HEADER with the
/// rule's RULE_ID regardless of IS_SCHEDULED, silently wiping confirmed
/// (IS_SCHEDULED = 0) actuals that were generated from the rule.
/// Expected: only IS_SCHEDULED = 1 occurrences are removed; confirmed rows
/// survive the cascade delete.
#[tokio::test]
#[ignore = "latent-audit H2"]
async fn latent_h2_cascade_delete_keeps_confirmed_headers() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());

    let created = service
        .create_rule_with_instances(USER_ID, valid_request())
        .await
        .expect("fixture rule creation must succeed");
    assert_eq!(created.generated_count, 3);
    assert_eq!(count_headers_for_rule(&pool, created.rule_id).await, 3);

    // Simulate the user confirming the first occurrence (scheduled -> actual).
    let confirmed_id = created.first_transaction_id.expect("first occurrence id");
    sqlx::query("UPDATE TRANSACTIONS_HEADER SET IS_SCHEDULED = 0 WHERE TRANSACTION_ID = ?")
        .bind(confirmed_id)
        .execute(&pool)
        .await
        .unwrap();

    service
        .delete_rule(USER_ID, created.rule_id, true)
        .await
        .expect("cascade delete must succeed");

    let confirmed_left: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM TRANSACTIONS_HEADER WHERE TRANSACTION_ID = ?",
    )
    .bind(confirmed_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        confirmed_left, 1,
        "confirmed (IS_SCHEDULED=0) header must survive cascade delete of its rule"
    );

    let scheduled_left: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM TRANSACTIONS_HEADER WHERE IS_SCHEDULED = 1")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(scheduled_left, 0, "scheduled occurrences must still be removed");
}

// ---------------------------------------------------------------------------
// M15 (仕様確認待ち)
// ---------------------------------------------------------------------------

/// M15 (proposal / 仕様確認待ち): holidays are only seeded for
/// [this year - 5, this year + 10]; a rule reaching beyond that range gets no
/// holiday data, so HolidayShift silently does nothing on e.g. New Year's Day
/// and the occurrence is fixed at creation time.
/// Expected (either is acceptable): creation is rejected with an error, OR the
/// out-of-range year's holidays are available so the shift is applied (no
/// occurrence on a known Japanese holiday).
#[tokio::test]
#[ignore = "latent-audit M15"]
async fn latent_m15_holiday_shift_applies_beyond_seeded_range() {
    use jpholiday::jpholiday::JPHoliday;

    let pool = setup_recurring_db().await;

    // Mirror production seeding (db.rs::seed_japanese_holidays).
    let jp = JPHoliday::new();
    let current_year = Local::now().year();
    for year in (current_year - 5)..=(current_year + 10) {
        for (date, name) in jp.year_holidays(year) {
            sqlx::query(
                "INSERT OR IGNORE INTO HOLIDAYS_STANDARD (LOCALE, HOLIDAY_DATE, HOLIDAY_NAME) \
                 VALUES ('JP', ?, ?)",
            )
            .bind(date.format("%Y-%m-%d").to_string())
            .bind(name)
            .execute(&pool)
            .await
            .unwrap();
        }
    }

    // First year outside the seeded range whose Jan 1 falls on a weekday, so
    // only the holiday table (not the weekend rule) can trigger the shift.
    let mut target_year = current_year + 11;
    while matches!(
        NaiveDate::from_ymd_opt(target_year, 1, 1).unwrap().weekday(),
        Weekday::Sat | Weekday::Sun
    ) {
        target_year += 1;
    }
    let new_year = NaiveDate::from_ymd_opt(target_year, 1, 1).unwrap();
    assert!(
        jp.year_holidays(target_year).iter().any(|(d, _)| *d == new_year),
        "precondition: {} must be a Japanese holiday",
        new_year
    );

    let mut request = valid_request();
    request.period_unit = consts::PERIOD_UNIT_MONTH.to_string();
    request.anchor_date = None;
    request.month_day_rule_type = Some(consts::MONTH_DAY_RULE_TYPE_DAY.to_string());
    request.day_of_month = Some(1);
    request.holiday_shift_type = consts::HOLIDAY_SHIFT_NEXT;
    request.start_date = format!("{}-01-01", target_year);
    request.end_date = format!("{}-01-31", target_year);

    let service = RecurringService::new(pool.clone());
    let result = service.create_rule_with_instances(USER_ID, request).await;

    if let Ok(created) = result {
        let dates: Vec<String> = sqlx::query_scalar(
            "SELECT substr(TRANSACTION_DATE, 1, 10) FROM TRANSACTIONS_HEADER WHERE RULE_ID = ?",
        )
        .bind(created.rule_id)
        .fetch_all(&pool)
        .await
        .unwrap();
        let holiday_str = new_year.format("%Y-%m-%d").to_string();
        assert!(
            !dates.contains(&holiday_str),
            "NextBusinessDay rule produced an occurrence on holiday {} (holiday data \
             missing beyond seeded range); generated dates = {:?}",
            holiday_str,
            dates
        );
    }
    // Err(_) is an acceptable outcome (range rejected up front).
}

// ---------------------------------------------------------------------------
// M16
// ---------------------------------------------------------------------------

/// M16: recurring creation bypasses the normal-transaction write validation;
/// a TRANSFER with FROM == TO is generated and later cannot be edited.
/// Expected: creation is rejected like `save_transaction_header` does.
#[tokio::test]
#[ignore = "latent-audit M16"]
async fn latent_m16_transfer_same_account_rejected() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    assert_baseline_ok(&service).await;

    let mut request = valid_request();
    request.category1_code = "TRANSFER".to_string();
    request.detail.category1_code = "TRANSFER".to_string();
    request.from_account_code = "NONE".to_string();
    request.to_account_code = "NONE".to_string();

    let result = service.create_rule_with_instances(USER_ID, request).await;
    assert!(
        result.is_err(),
        "TRANSFER with from_account == to_account must be rejected, got {:?}",
        result.ok()
    );
}

/// M16: recurring creation accepts any TAX_ROUNDING_TYPE value.
/// Expected: values outside {DOWN, HALF_UP, UP} are rejected.
#[tokio::test]
#[ignore = "latent-audit M16"]
async fn latent_m16_tax_rounding_type_out_of_range_rejected() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    assert_baseline_ok(&service).await;

    for bad in [-1_i64, 3, 99] {
        let mut request = valid_request();
        request.tax_rounding_type = bad;
        let result = service.create_rule_with_instances(USER_ID, request).await;
        assert!(
            result.is_err(),
            "tax_rounding_type={} must be rejected, got {:?}",
            bad,
            result.ok()
        );
    }
}

/// M16: recurring creation accepts any TAX_INCLUDED_TYPE value.
/// Expected: values outside {INCLUDED, EXCLUDED} are rejected.
#[tokio::test]
#[ignore = "latent-audit M16"]
async fn latent_m16_tax_included_type_out_of_range_rejected() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    assert_baseline_ok(&service).await;

    for bad in [-1_i64, 2, 99] {
        let mut request = valid_request();
        request.tax_included_type = bad;
        let result = service.create_rule_with_instances(USER_ID, request).await;
        assert!(
            result.is_err(),
            "tax_included_type={} must be rejected, got {:?}",
            bad,
            result.ok()
        );
    }
}

/// M16: the DETAIL amount is not range-checked on the recurring path.
/// Expected: amount outside 0..=999,999,999 is rejected (as for normal details).
#[tokio::test]
#[ignore = "latent-audit M16"]
async fn latent_m16_detail_amount_out_of_range_rejected() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    assert_baseline_ok(&service).await;

    for bad in [-1_i64, 1_000_000_000] {
        let mut request = valid_request();
        request.detail.amount = bad;
        let result = service.create_rule_with_instances(USER_ID, request).await;
        assert!(
            result.is_err(),
            "detail.amount={} must be rejected, got {:?}",
            bad,
            result.ok()
        );
    }
}

/// M16: the DETAIL tax_rate is not range-checked on the recurring path.
/// Expected: tax_rate outside 0..=100 is rejected (as for normal details).
#[tokio::test]
#[ignore = "latent-audit M16"]
async fn latent_m16_detail_tax_rate_out_of_range_rejected() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    assert_baseline_ok(&service).await;

    for bad in [-1_i32, 101] {
        let mut request = valid_request();
        request.detail.tax_rate = bad;
        let result = service.create_rule_with_instances(USER_ID, request).await;
        assert!(
            result.is_err(),
            "detail.tax_rate={} must be rejected, got {:?}",
            bad,
            result.ok()
        );
    }
}

// ---------------------------------------------------------------------------
// M18 (仕様確認待ち)
// ---------------------------------------------------------------------------

/// M18 (proposal / 仕様確認待ち): there is no cap on period length or
/// occurrence count; a year typo (e.g. end 9999-12-31 on a daily rule) makes
/// one transaction insert ~2.9 million rows while holding the DB, freezing
/// the app.
/// Expected: such an unreasonably large generation is rejected with an error
/// (the exact limit is left to the fix). A 10 s timeout turns "still
/// generating" into a failure instead of hanging the test run.
#[tokio::test]
#[ignore = "latent-audit M18"]
async fn latent_m18_huge_generation_rejected() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());

    let mut request = valid_request();
    request.start_date = "2026-01-01".to_string();
    request.end_date = "9999-12-31".to_string();

    let outcome = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        service.create_rule_with_instances(USER_ID, request),
    )
    .await;

    match outcome {
        Ok(Err(_)) => {}
        Ok(Ok(created)) => panic!(
            "daily rule 2026-01-01..9999-12-31 must be rejected, but generated {} rows",
            created.generated_count
        ),
        Err(_) => panic!(
            "daily rule 2026-01-01..9999-12-31 was not rejected: still generating after 10s"
        ),
    }
}
