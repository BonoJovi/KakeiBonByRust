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
            ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
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
// M15
// ---------------------------------------------------------------------------

/// M15: holidays are only seeded for [this year - 5, this year + 10]; a rule
/// reaching beyond that range got no holiday data, so HolidayShift silently
/// did nothing on e.g. New Year's Day and the occurrence was fixed at
/// creation time.
/// Spec (2026-09-28): a rule may only span the seeded years, so creating one
/// beyond them is rejected (`PeriodOutOfRange`); the test also still accepts
/// the alternative outcome (no occurrence on a known holiday).
#[tokio::test]
async fn latent_m15_holiday_shift_applies_beyond_seeded_range() {
    use jpholiday::JPHoliday;

    let pool = setup_recurring_db().await;

    // Mirror production seeding (db.rs::seed_japanese_holidays), which
    // covers one year more than a rule may (scan2-R7).
    let jp = JPHoliday::new();
    let current_year = Local::now().year();
    for year in (current_year - 5)..=(current_year + 11) {
        for holiday in jp.year_holidays(year) {
            sqlx::query(
                "INSERT OR IGNORE INTO HOLIDAYS_STANDARD (LOCALE, HOLIDAY_DATE, HOLIDAY_NAME) \
                 VALUES ('JP', ?, ?)",
            )
            .bind(holiday.date.to_string())
            .bind(holiday.name)
            .execute(&pool)
            .await
            .unwrap();
        }
    }

    // First year outside the seeded range whose Jan 1 falls on a weekday, so
    // only the holiday table (not the weekend rule) can trigger the shift.
    let mut target_year = current_year + 12;
    while matches!(
        NaiveDate::from_ymd_opt(target_year, 1, 1).unwrap().weekday(),
        Weekday::Sat | Weekday::Sun
    ) {
        target_year += 1;
    }
    let new_year = NaiveDate::from_ymd_opt(target_year, 1, 1).unwrap();
    assert!(
        jp.year_holidays(target_year)
            .iter()
            .any(|h| h.date.to_string() == new_year.format("%Y-%m-%d").to_string()),
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

    assert!(
        matches!(result, Err(RecurringError::PeriodOutOfRange { .. }) | Ok(_)),
        "unexpected error: {:?}",
        result.as_ref().err()
    );
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

/// A rule whose category needs an account (EXPENSE: FROM, INCOME: TO,
/// TRANSFER: both) is refused when that side is the NONE account, as
/// `save_transaction_header` refuses it; otherwise every generated
/// occurrence would count as an expense or income on no account.
#[tokio::test]
async fn recurring_rule_rejects_missing_account_when_category_needs_it() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    assert_baseline_ok(&service).await;

    for (category1, from, to) in [
        ("EXPENSE", "NONE", "BANK"),
        ("INCOME", "BANK", "NONE"),
        ("TRANSFER", "NONE", "BANK"),
        ("TRANSFER", "BANK", "NONE"),
    ] {
        let mut request = valid_request();
        request.category1_code = category1.to_string();
        request.detail.category1_code = category1.to_string();
        request.from_account_code = from.to_string();
        request.to_account_code = to.to_string();

        let result = service.create_rule_with_instances(USER_ID, request).await;
        let code = result.err().map(|e| ApiError::from(e).code);
        assert_eq!(
            code.as_deref(),
            Some("account_required"),
            "{} with FROM={} TO={} must be refused with account_required",
            category1, from, to
        );
    }
}

/// M16: recurring creation accepts any TAX_ROUNDING_TYPE value.
/// Expected: values outside {DOWN, HALF_UP, UP} are rejected.
#[tokio::test]
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
// M18
// ---------------------------------------------------------------------------

/// M18: there was no cap on period length or occurrence count; a year typo
/// (e.g. end 9999-12-31 on a daily rule) made one transaction insert ~2.9
/// million rows while holding the DB, freezing the app.
/// Spec (2026-09-28): the end date may be at most Dec 31 of (this year + 10),
/// so such a rule is rejected (`PeriodOutOfRange`). A 10 s timeout turns
/// "still generating" into a failure instead of hanging the test run.
#[tokio::test]
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
        Ok(Err(RecurringError::PeriodOutOfRange { .. })) => {}
        Ok(Err(e)) => panic!("expected PeriodOutOfRange, got {:?}", e),
        Ok(Ok(created)) => panic!(
            "daily rule 2026-01-01..9999-12-31 must be rejected, but generated {} rows",
            created.generated_count
        ),
        Err(_) => panic!(
            "daily rule 2026-01-01..9999-12-31 was not rejected: still generating after 10s"
        ),
    }
}

/// M15 / M18: the allowed period is exactly the seeded holiday years — its
/// first and last day are accepted, one day outside is rejected.
#[tokio::test]
async fn latent_m15_m18_period_limits_are_inclusive() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    let (first, last) = recurring_period_limits(Local::now().date_naive());
    let day = |d: NaiveDate| d.format("%Y-%m-%d").to_string();

    for (start, end) in [(first, first), (last, last)] {
        let mut request = valid_request();
        request.start_date = day(start);
        request.end_date = day(end);
        // The fixture's daily anchor (2026-01-01) must not lie after the end
        // date (latent-scan2 R2).
        request.anchor_date = Some(day(start));
        let result = service.create_rule_with_instances(USER_ID, request).await;
        assert!(result.is_ok(), "{}..{} must be accepted: {:?}", start, end, result.err());
    }
    for (start, end) in [
        (first.pred_opt().unwrap(), first),
        (last, last.succ_opt().unwrap()),
    ] {
        let mut request = valid_request();
        request.start_date = day(start);
        request.end_date = day(end);
        let result = service.create_rule_with_instances(USER_ID, request).await;
        assert!(
            matches!(result, Err(RecurringError::PeriodOutOfRange { .. })),
            "{}..{} must be rejected: {:?}",
            start,
            end,
            result.map(|r| r.generated_count)
        );
    }
}

/// M15: an app left running across New Year has one seeded year less ahead
/// than the date-based limits assume; the rule must stay within the years
/// actually seeded. Since scan2-R7 it also stops one year before the last
/// seeded year, so a "next business day" shift from Dec 31 still finds the
/// January holidays.
#[tokio::test]
async fn latent_m15_period_limit_follows_seeded_holidays() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    let (_, last) = recurring_period_limits(Local::now().date_naive());
    // Seeded only up to the year before the date-based last year (and the
    // year before that, so the window is not empty).
    let seeded_last_year = last.year() - 1;
    for y in [seeded_last_year - 1, seeded_last_year] {
        sqlx::query(
            "INSERT INTO HOLIDAYS_STANDARD (LOCALE, HOLIDAY_DATE, HOLIDAY_NAME) VALUES ('JP', ?, '元日')",
        )
        .bind(format!("{}-01-01", y))
        .execute(&pool)
        .await
        .unwrap();
    }

    let mut request = valid_request();
    request.start_date = format!("{}-01-01", last.year());
    request.end_date = format!("{}-01-03", last.year());
    let result = service.create_rule_with_instances(USER_ID, request).await;
    assert!(
        matches!(result, Err(RecurringError::PeriodOutOfRange { .. })),
        "a year without seeded holidays must be rejected: {:?}",
        result.map(|r| r.generated_count)
    );

    let mut request = valid_request();
    request.start_date = format!("{}-12-29", seeded_last_year);
    request.end_date = format!("{}-12-31", seeded_last_year);
    let result = service.create_rule_with_instances(USER_ID, request).await;
    assert!(
        matches!(result, Err(RecurringError::PeriodOutOfRange { .. })),
        "the last seeded year is rejected: the January after it is not seeded (scan2-R7): {:?}",
        result.map(|r| r.generated_count)
    );

    let mut request = valid_request();
    request.start_date = format!("{}-12-29", seeded_last_year - 1);
    request.end_date = format!("{}-12-31", seeded_last_year - 1);
    let result = service.create_rule_with_instances(USER_ID, request).await;
    assert!(
        result.is_ok(),
        "the year before the last seeded one is allowed: {:?}",
        result.err()
    );
}

/// M15 / M18: `period_limits` (what the screen shows and create enforces)
/// is the date-based window clamped to the seeded years (ending one year
/// before the last seeded year, scan2-R7), and is the plain date-based
/// window when no holidays are seeded.
#[tokio::test]
async fn latent_m15_m18_period_limits_service_clamps_to_seeded_years() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    let date_based = recurring_period_limits(Local::now().date_naive());
    assert_eq!(service.period_limits().await.unwrap(), date_based);

    let year = Local::now().year();
    for y in [year - 2, year + 3] {
        sqlx::query(
            "INSERT INTO HOLIDAYS_STANDARD (LOCALE, HOLIDAY_DATE, HOLIDAY_NAME) VALUES ('JP', ?, '元日')",
        )
        .bind(format!("{}-01-01", y))
        .execute(&pool)
        .await
        .unwrap();
    }
    assert_eq!(
        service.period_limits().await.unwrap(),
        (
            NaiveDate::from_ymd_opt(year - 2, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(year + 2, 12, 31).unwrap(),
        )
    );
}

/// M15 / M18: the limits follow the holiday seeding window.
#[test]
fn latent_m15_m18_period_limits_follow_seeded_years() {
    let today = NaiveDate::from_ymd_opt(2026, 9, 28).unwrap();
    let (first, last) = recurring_period_limits(today);
    assert_eq!(first, NaiveDate::from_ymd_opt(2021, 1, 1).unwrap());
    assert_eq!(last, NaiveDate::from_ymd_opt(2036, 12, 31).unwrap());
}

// ---------------------------------------------------------------------------
// M17
// ---------------------------------------------------------------------------

/// M17: the rule's total was typed separately (defaulting to 0) and never
/// checked against the detail, so 0-yen occurrences could be generated.
/// Spec (2026-09-28): the total is derived from the single detail, like a
/// transaction's recommended total.
#[tokio::test]
async fn latent_m17_total_is_derived_from_the_detail() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());

    // (tax_included_type, rounding, amount, rate, amount_including_tax, expected total)
    let cases = [
        // tax-excluded: 1005 @ 8% = 1085.4 → floor 1085 / half-up 1085 / ceil 1086
        (consts::TAX_EXCLUDED, consts::TAX_ROUND_DOWN, 1005, 8, Some(1085), 1085),
        (consts::TAX_EXCLUDED, consts::TAX_ROUND_UP, 1005, 8, Some(1086), 1086),
        // tax-included: the detail's own tax-included price
        (consts::TAX_INCLUDED, consts::TAX_ROUND_DOWN, 1000, 10, Some(1100), 1100),
        // tax-included without a price: derived from amount and rate
        (consts::TAX_INCLUDED, consts::TAX_ROUND_DOWN, 1000, 10, None, 1100),
    ];
    for (included, rounding, amount, rate, including, expected) in cases {
        let mut request = valid_request();
        request.tax_included_type = included;
        request.tax_rounding_type = rounding;
        request.detail.amount = amount;
        request.detail.tax_rate = rate;
        request.detail.amount_including_tax = including;
        let created = service
            .create_rule_with_instances(USER_ID, request)
            .await
            .expect("create rule");

        let totals: Vec<i64> = sqlx::query_scalar(
            "SELECT TOTAL_AMOUNT FROM TRANSACTIONS_HEADER WHERE RULE_ID = ?",
        )
        .bind(created.rule_id)
        .fetch_all(&pool)
        .await
        .unwrap();
        assert!(!totals.is_empty());
        assert!(
            totals.iter().all(|t| *t == expected),
            "included={} rounding={} amount={} rate={} → expected {}, got {:?}",
            included, rounding, amount, rate, expected, totals
        );
        let rule_total: i64 = sqlx::query_scalar("SELECT TOTAL_AMOUNT FROM RECURRING_RULES WHERE RULE_ID = ?")
            .bind(created.rule_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(rule_total, expected);
    }
}

// ---------------------------------------------------------------------------
// L13
// ---------------------------------------------------------------------------

/// L13: a daily rule with a holiday shift produced two occurrences on the
/// same day, or one past the end date. Spec (2026-09-28): a daily rule has
/// no holiday shift; other cycles keep it.
#[tokio::test]
async fn latent_l13_daily_rule_rejects_holiday_shift() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());

    for shift in [consts::HOLIDAY_SHIFT_PREV, consts::HOLIDAY_SHIFT_NEXT] {
        let mut request = valid_request();
        request.period_unit = consts::PERIOD_UNIT_DAY.to_string();
        request.holiday_shift_type = shift;
        let result = service.create_rule_with_instances(USER_ID, request).await;
        assert!(
            matches!(result, Err(RecurringError::Validation(_))),
            "daily rule with shift {} must be rejected: {:?}",
            shift,
            result.map(|r| r.generated_count)
        );
    }

    let mut daily = valid_request();
    daily.period_unit = consts::PERIOD_UNIT_DAY.to_string();
    daily.holiday_shift_type = consts::HOLIDAY_SHIFT_NONE;
    let result = service.create_rule_with_instances(USER_ID, daily).await;
    assert!(result.is_ok(), "daily rule without shift: {:?}", result.err());

    let mut monthly = valid_request();
    monthly.period_unit = consts::PERIOD_UNIT_MONTH.to_string();
    monthly.anchor_date = None;
    monthly.month_day_rule_type = Some(consts::MONTH_DAY_RULE_TYPE_DAY_OR_END.to_string());
    monthly.day_of_month = Some(31);
    monthly.holiday_shift_type = consts::HOLIDAY_SHIFT_NEXT;
    let result = service.create_rule_with_instances(USER_ID, monthly).await;
    assert!(result.is_ok(), "monthly rule keeps its holiday shift: {:?}", result.err());
}

// ---------------------------------------------------------------------------
// L2 (recurring path)
// ---------------------------------------------------------------------------

/// L2 — the recurring template's SHOP_ID / PRODUCT_ID must belong to the
/// user, like a normal transaction's; otherwise every generated occurrence
/// links another user's shop / product.
#[tokio::test]
async fn latent_l2_recurring_rejects_foreign_shop_and_product() {
    let pool = setup_recurring_db().await;
    for ddl in [
        "CREATE TABLE SHOPS (SHOP_ID INTEGER PRIMARY KEY, USER_ID INTEGER NOT NULL, SHOP_NAME TEXT NOT NULL)",
        "CREATE TABLE PRODUCTS (PRODUCT_ID INTEGER PRIMARY KEY, USER_ID INTEGER NOT NULL, PRODUCT_NAME TEXT NOT NULL)",
        "INSERT INTO SHOPS (SHOP_ID, USER_ID, SHOP_NAME) VALUES (10, 2, 'own shop'), (11, 3, 'foreign shop')",
        "INSERT INTO PRODUCTS (PRODUCT_ID, USER_ID, PRODUCT_NAME) VALUES (20, 2, 'own product'), (21, 3, 'foreign product')",
    ] {
        sqlx::query(ddl).execute(&pool).await.expect(ddl);
    }
    let service = RecurringService::new(pool);

    // Own shop + own product: accepted.
    let mut own = valid_request();
    own.shop_id = Some(10);
    own.detail.product_id = Some(20);
    let r = service.create_rule_with_instances(USER_ID, own).await;
    assert!(r.is_ok(), "own shop / product must be accepted: {:?}", r.err());

    let mut foreign_shop = valid_request();
    foreign_shop.shop_id = Some(11);
    assert!(
        matches!(
            service.create_rule_with_instances(USER_ID, foreign_shop).await,
            Err(RecurringError::Validation(_))
        ),
        "another user's SHOP_ID must be rejected"
    );

    let mut foreign_product = valid_request();
    foreign_product.detail.product_id = Some(21);
    assert!(
        matches!(
            service.create_rule_with_instances(USER_ID, foreign_product).await,
            Err(RecurringError::Validation(_))
        ),
        "another user's PRODUCT_ID must be rejected"
    );
}

// ---------------------------------------------------------------------------
// L10 follow-up (recurring path)
// ---------------------------------------------------------------------------

/// Run `f` on its own thread and give up after `secs`: an infinite loop in
/// the code under test fails the test instead of hanging the whole run.
fn finishes_within<T: Send + 'static>(secs: u64, f: impl FnOnce() -> T + Send + 'static) -> Option<T> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(f());
    });
    rx.recv_timeout(std::time::Duration::from_secs(secs)).ok()
}

/// L10 follow-up (CodeRabbit on #153): once the period helpers stopped
/// panicking, monthly / yearly generation near the end of chrono's range
/// could loop forever — the loop only stopped when the next month's 1st was
/// past `end`, and that date could no longer be built. It must terminate.
#[test]
fn latent_l10_generation_terminates_at_the_end_of_the_date_range() {
    use chrono::Datelike;
    let last_year = NaiveDate::MAX.year();
    let start = NaiveDate::from_ymd_opt(last_year, 11, 1).unwrap();
    let end = NaiveDate::MAX;

    for rule in [MonthlyDayRule::EndOfMonth, MonthlyDayRule::DayOfMonth { day: 15 }] {
        let monthly = finishes_within(5, move || generate_monthly(1, &rule, start, start, end));
        assert!(monthly.is_some(), "monthly generation with {:?} must terminate", rule);
        let yearly = finishes_within(5, move || generate_yearly(1, 12, &rule, start, start, end));
        assert!(yearly.is_some(), "yearly generation with {:?} must terminate", rule);
    }
}

// ===========================================================================
// Latent-bug scan 2 (2026-09-29): R1, R2, R6, R7
// (R3 lives in src/latent_audit/db.rs; R4, R5, R8 are Jest tests under
// tests/frontend/latent-audit/scan2-r*.test.js)
// ===========================================================================

async fn occurrence_dates(pool: &SqlitePool, rule_id: i64) -> Vec<String> {
    sqlx::query_scalar(
        "SELECT substr(TRANSACTION_DATE, 1, 10) FROM TRANSACTIONS_HEADER \
         WHERE RULE_ID = ? ORDER BY TRANSACTION_DATE",
    )
    .bind(rule_id)
    .fetch_all(pool)
    .await
    .unwrap()
}

fn monthly_day_request(day: u32, shift: i32, start: &str, end: &str) -> SaveRecurringRuleRequest {
    let mut request = valid_request();
    request.period_unit = consts::PERIOD_UNIT_MONTH.to_string();
    request.anchor_date = None;
    request.month_day_rule_type = Some(consts::MONTH_DAY_RULE_TYPE_DAY.to_string());
    request.day_of_month = Some(day);
    request.holiday_shift_type = shift;
    request.start_date = start.to_string();
    request.end_date = end.to_string();
    request
}

/// scan2-R1: the period filter runs on the calendar date BEFORE the holiday
/// shift, so an occurrence whose calendar date is just outside the period
/// but whose shifted date is inside it is dropped.
/// Expected: Monthly day 25 / Next, start Mon 2026-10-26 → the October debit
/// (Sun 10-25 → Mon 10-26) is generated; Monthly day 25 / Prev, end
/// 2027-04-24 → the April payday (Sun 04-25 → Fri 04-23) is generated.
#[tokio::test]
async fn latent_scan2_r1_shifted_date_inside_period_is_kept() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());

    // Start boundary.
    let created = service
        .create_rule_with_instances(
            USER_ID,
            monthly_day_request(25, consts::HOLIDAY_SHIFT_NEXT, "2026-10-26", "2027-10-26"),
        )
        .await
        .expect("create rule (Next)");
    let dates = occurrence_dates(&pool, created.rule_id).await;
    assert!(
        dates.contains(&"2026-10-26".to_string()),
        "Next shift of Sun 2026-10-25 lands on 2026-10-26 (inside the period) and must be \
         generated; got {:?}",
        dates
    );

    // End boundary.
    let created = service
        .create_rule_with_instances(
            USER_ID,
            monthly_day_request(25, consts::HOLIDAY_SHIFT_PREV, "2026-05-01", "2027-04-24"),
        )
        .await
        .expect("create rule (Prev)");
    let dates = occurrence_dates(&pool, created.rule_id).await;
    assert!(
        dates.contains(&"2027-04-23".to_string()),
        "Prev shift of Sun 2027-04-25 lands on 2027-04-23 (inside the period) and must be \
         generated; got {:?}",
        dates
    );
}

/// scan2-R2: a daily rule's anchor is never checked against the period.
/// An anchor after the end date creates a rule with 0 occurrences as a
/// success, and a blank anchor (the UI sends "" after the field is cleared)
/// fails with the raw `Invalid anchor_date: `.
/// Expected (bug-list fix direction): a blank anchor falls back to the start
/// date, and an anchor after the end date is rejected as a validation error.
#[tokio::test]
async fn latent_scan2_r2_daily_anchor_is_checked_against_the_period() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    assert_baseline_ok(&service).await;

    // Blank anchor → start_date (2026-01-01..03, interval 1 → 3 occurrences).
    let mut blank = valid_request();
    blank.anchor_date = Some(String::new());
    let result = service.create_rule_with_instances(USER_ID, blank).await;
    match result {
        Ok(created) => {
            let dates = occurrence_dates(&pool, created.rule_id).await;
            assert_eq!(
                dates,
                vec!["2026-01-01", "2026-01-02", "2026-01-03"],
                "a blank anchor must fall back to the start date"
            );
        }
        Err(e) => panic!("a blank anchor must fall back to the start date, got error {:?}", e),
    }

    // Anchor after the end date → rejected, not a silent 0-occurrence rule.
    let mut late = valid_request();
    late.anchor_date = Some("2026-02-01".to_string());
    let result = service.create_rule_with_instances(USER_ID, late).await;
    assert!(
        matches!(result, Err(RecurringError::Validation(_))),
        "an anchor after end_date must be rejected; got {:?}",
        result.map(|r| r.generated_count)
    );
}

/// scan2-R6: the recurring path skips the M2 guard, so a detail whose
/// CATEGORY1 differs from the header's (header INCOME, detail EXPENSE) is
/// stored in every occurrence; the regular edit path later rejects it.
/// Expected: rejected as a validation error, or the detail is stored with
/// the header's CATEGORY1.
#[tokio::test]
async fn latent_scan2_r6_detail_category1_must_match_header() {
    let pool = setup_recurring_db().await;
    let service = RecurringService::new(pool.clone());
    assert_baseline_ok(&service).await;

    let mut request = valid_request();
    request.category1_code = "INCOME".to_string();
    request.from_account_code = "NONE".to_string();
    request.to_account_code = "BANK".to_string();
    request.detail.category1_code = "EXPENSE".to_string();
    let result = service.create_rule_with_instances(USER_ID, request).await;

    match result {
        Err(RecurringError::Validation(_)) => {}
        Err(e) => panic!("unexpected error: {:?}", e),
        Ok(created) => {
            let mismatched: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM TRANSACTIONS_DETAIL d \
                 JOIN TRANSACTIONS_HEADER h ON h.TRANSACTION_ID = d.TRANSACTION_ID \
                 WHERE h.RULE_ID = ? AND d.CATEGORY1_CODE <> h.CATEGORY1_CODE",
            )
            .bind(created.rule_id)
            .fetch_one(&pool)
            .await
            .unwrap();
            let rule_detail: String = sqlx::query_scalar(
                "SELECT CATEGORY1_CODE FROM RECURRING_RULE_DETAILS WHERE RULE_ID = ?",
            )
            .bind(created.rule_id)
            .fetch_one(&pool)
            .await
            .unwrap();
            assert_eq!(
                mismatched, 0,
                "{} of {} occurrences store a detail CATEGORY1 that differs from the header's",
                mismatched, created.generated_count
            );
            assert_eq!(rule_detail, "INCOME", "rule detail CATEGORY1 must follow the header");
        }
    }
}

/// scan2-R7: `period_limits` lets a rule end on Dec 31 of the last seeded
/// year, and a Next shift from a weekend Dec 31 walks into January of the
/// following, unseeded year, where 元日 counts as a business day.
/// The DB here is seeded through 2028 only — the state production reaches
/// when the last seeded year is the last allowed year (e.g. an app left
/// running across New Year, see `period_limits`). 2028-12-31 is a Sunday
/// and 2029-01-01 (Mon) is 元日, so the correct date is 2029-01-02.
/// Expected: either the period is rejected (the limit leaves room for the
/// shift) or the occurrence does not land on a holiday / weekend.
#[tokio::test]
async fn latent_scan2_r7_next_shift_past_the_last_seeded_year() {
    use jpholiday::JPHoliday;

    let pool = setup_recurring_db().await;
    let jp = JPHoliday::new();
    let last_seeded = 2028;
    for year in (Local::now().year() - consts::HOLIDAY_SEED_YEARS_BACK)..=last_seeded {
        for holiday in jp.year_holidays(year) {
            sqlx::query(
                "INSERT OR IGNORE INTO HOLIDAYS_STANDARD (LOCALE, HOLIDAY_DATE, HOLIDAY_NAME) \
                 VALUES ('JP', ?, ?)",
            )
            .bind(holiday.date.to_string())
            .bind(holiday.name)
            .execute(&pool)
            .await
            .unwrap();
        }
    }
    assert_eq!(
        NaiveDate::from_ymd_opt(2028, 12, 31).unwrap().weekday(),
        Weekday::Sun,
        "precondition"
    );

    let mut request = valid_request();
    request.period_unit = consts::PERIOD_UNIT_MONTH.to_string();
    request.anchor_date = None;
    request.month_day_rule_type = Some(consts::MONTH_DAY_RULE_TYPE_END.to_string());
    request.holiday_shift_type = consts::HOLIDAY_SHIFT_NEXT;
    request.start_date = "2028-12-01".to_string();
    request.end_date = "2028-12-31".to_string();

    let service = RecurringService::new(pool.clone());
    match service.create_rule_with_instances(USER_ID, request).await {
        Err(RecurringError::PeriodOutOfRange { .. }) => {}
        Err(e) => panic!("unexpected error: {:?}", e),
        Ok(created) => {
            let dates = occurrence_dates(&pool, created.rule_id).await;
            let holidays: Vec<String> = jp
                .year_holidays(2029)
                .iter()
                .map(|h| h.date.to_string())
                .collect();
            for d in &dates {
                let date = NaiveDate::parse_from_str(d, "%Y-%m-%d").unwrap();
                assert!(
                    !holidays.contains(d)
                        && !matches!(date.weekday(), Weekday::Sat | Weekday::Sun),
                    "Next shift from Sun 2028-12-31 landed on non-business day {} \
                     (2029 holidays are not seeded); generated dates = {:?}",
                    d,
                    dates
                );
            }
        }
    }
}
