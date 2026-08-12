//! 休日シフトの共通ロジック。v2.1.0 の繰り返し予定（recurring.rs）と
//! v2.4.0 の集計起算日（period.rs）の両方から利用される。
//!
//! 祝日テーブル（HOLIDAYS_STANDARD + HOLIDAYS_USER_CUSTOM）の取得は呼び出し側の
//! 責任で、本モジュールは「与えられた祝日集合に対する平日判定とシフト計算」だけを担う。

use chrono::{Datelike, Days, NaiveDate, Weekday};
use sqlx::SqlitePool;
use std::collections::HashSet;

/// 休日シフトの方向。
/// - DB 上は INTEGER (0/1/2) として `RECURRING_RULES.HOLIDAY_SHIFT_TYPE` /
///   `USERS.MONTH_PERIOD_HOLIDAY_SHIFT` に格納される。
/// - 変換は呼び出し側で行い、本モジュールは enum を直接受け取る。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolidayShift {
    /// 0: カレンダー通り（シフトなし）
    None,
    /// 1: 土日祝なら直前の平日（給料日想定）
    Prev,
    /// 2: 土日祝なら直後の平日（引落想定）
    Next,
}

impl HolidayShift {
    /// DB の i32 値 (0/1/2) を enum に変換する。範囲外は None。
    pub fn from_db_value(value: i32) -> Option<HolidayShift> {
        match value {
            0 => Some(HolidayShift::None),
            1 => Some(HolidayShift::Prev),
            2 => Some(HolidayShift::Next),
            _ => None,
        }
    }

    /// enum を DB の i32 値に変換する。
    pub fn to_db_value(self) -> i32 {
        match self {
            HolidayShift::None => 0,
            HolidayShift::Prev => 1,
            HolidayShift::Next => 2,
        }
    }
}

/// 土日 + 祝日テーブルに含まれる日を非平日とみなす。
pub fn is_non_business_day(d: NaiveDate, holidays: &HashSet<NaiveDate>) -> bool {
    matches!(d.weekday(), Weekday::Sat | Weekday::Sun) || holidays.contains(&d)
}

/// 休日シフト。指定方向に「平日にぶつかるまで」進める／遡る。
/// シフト結果が呼び出し側の意図する範囲外になっても採用する
/// （カレンダー上の挙動をそのまま返す）。
pub fn shift_for_holidays(
    d: NaiveDate,
    shift: HolidayShift,
    holidays: &HashSet<NaiveDate>,
) -> NaiveDate {
    let mut current = d;
    match shift {
        HolidayShift::None => current,
        HolidayShift::Prev => {
            while is_non_business_day(current, holidays) {
                match current.checked_sub_days(Days::new(1)) {
                    Some(prev) => current = prev,
                    None => break,
                }
            }
            current
        }
        HolidayShift::Next => {
            while is_non_business_day(current, holidays) {
                match current.checked_add_days(Days::new(1)) {
                    Some(next) => current = next,
                    None => break,
                }
            }
            current
        }
    }
}

/// HOLIDAYS_STANDARD と HOLIDAYS_USER_CUSTOM から、指定ウィンドウ ± 14 日の祝日を取得する。
///
/// ± 14 日のパディングは `HolidayShift::Prev` / `Next` が連休をまたいでサイクル外の日に
/// 落ちる可能性を吸収するため（例: 1/1 祝日が前年 12/31 に shift など）。
///
/// 呼び出し側で `HolidayShift::None` を判別して空集合で済ませる最適化は呼び側の責任。
pub async fn fetch_holidays(
    pool: &SqlitePool,
    user_id: i64,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<HashSet<NaiveDate>, sqlx::Error> {
    let locale: String = sqlx::query_scalar(
        "SELECT COALESCE(HOLIDAY_LOCALE, 'JP') FROM USERS WHERE USER_ID = ?",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    let widen = Days::new(14);
    let widened_start = start.checked_sub_days(widen).unwrap_or(start);
    let widened_end = end.checked_add_days(widen).unwrap_or(end);
    let ws = widened_start.format("%Y-%m-%d").to_string();
    let we = widened_end.format("%Y-%m-%d").to_string();

    let mut holidays = HashSet::new();

    let std_rows: Vec<String> = sqlx::query_scalar(
        "SELECT HOLIDAY_DATE FROM HOLIDAYS_STANDARD \
         WHERE LOCALE = ? AND HOLIDAY_DATE BETWEEN ? AND ?",
    )
    .bind(&locale)
    .bind(&ws)
    .bind(&we)
    .fetch_all(pool)
    .await?;
    for d_str in std_rows {
        if let Ok(d) = NaiveDate::parse_from_str(&d_str, "%Y-%m-%d") {
            holidays.insert(d);
        }
    }

    let custom_rows: Vec<String> = sqlx::query_scalar(
        "SELECT HOLIDAY_DATE FROM HOLIDAYS_USER_CUSTOM \
         WHERE USER_ID = ? AND HOLIDAY_DATE BETWEEN ? AND ?",
    )
    .bind(user_id)
    .bind(&ws)
    .bind(&we)
    .fetch_all(pool)
    .await?;
    for d_str in custom_rows {
        if let Ok(d) = NaiveDate::parse_from_str(&d_str, "%Y-%m-%d") {
            holidays.insert(d);
        }
    }

    Ok(holidays)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sql_queries;
    use crate::test_helpers::database::{init_db, TEST_DB_URL};

    fn d(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    /// USERS + both holiday tables, with `user_id` 1 on `locale`.
    async fn setup_holiday_db(locale: Option<&str>) -> SqlitePool {
        let pool = init_db(TEST_DB_URL).await.unwrap();

        for ddl in [
            sql_queries::TEST_HOLIDAY_CREATE_USERS_TABLE,
            sql_queries::CREATE_HOLIDAYS_STANDARD_TABLE,
            sql_queries::CREATE_HOLIDAYS_USER_CUSTOM_TABLE,
        ] {
            sqlx::query(ddl).execute(&pool).await.unwrap();
        }

        sqlx::query(sql_queries::TEST_HOLIDAY_INSERT_USER)
            .bind(1_i64)
            .bind("user1")
            .bind(locale)
            .execute(&pool)
            .await
            .unwrap();

        pool
    }

    async fn insert_standard(pool: &SqlitePool, locale: &str, date: &str) {
        sqlx::query(sql_queries::TEST_HOLIDAY_INSERT_STANDARD)
            .bind(locale)
            .bind(date)
            .bind("holiday")
            .execute(pool)
            .await
            .unwrap();
    }

    async fn insert_custom(pool: &SqlitePool, user_id: i64, date: &str) {
        sqlx::query(sql_queries::TEST_HOLIDAY_INSERT_CUSTOM)
            .bind(user_id)
            .bind(date)
            .bind("company holiday")
            .execute(pool)
            .await
            .unwrap();
    }

    #[test]
    fn weekday_is_business_day() {
        assert!(!is_non_business_day(d(2026, 5, 26), &HashSet::new())); // Tue
    }

    #[test]
    fn weekend_is_non_business_day() {
        assert!(is_non_business_day(d(2026, 5, 23), &HashSet::new())); // Sat
        assert!(is_non_business_day(d(2026, 5, 24), &HashSet::new())); // Sun
    }

    #[test]
    fn holiday_is_non_business_day() {
        let holidays: HashSet<NaiveDate> = [d(2026, 5, 5)].into_iter().collect(); // こどもの日 (Tue)
        assert!(is_non_business_day(d(2026, 5, 5), &holidays));
    }

    #[test]
    fn shift_none_returns_input() {
        let holidays = HashSet::new();
        assert_eq!(
            shift_for_holidays(d(2026, 5, 23), HolidayShift::None, &holidays),
            d(2026, 5, 23)
        );
    }

    #[test]
    fn shift_prev_skips_weekend() {
        let holidays = HashSet::new();
        // Sun 5/24 → Fri 5/22
        assert_eq!(
            shift_for_holidays(d(2026, 5, 24), HolidayShift::Prev, &holidays),
            d(2026, 5, 22)
        );
    }

    #[test]
    fn shift_next_skips_weekend() {
        let holidays = HashSet::new();
        // Sat 5/23 → Mon 5/25
        assert_eq!(
            shift_for_holidays(d(2026, 5, 23), HolidayShift::Next, &holidays),
            d(2026, 5, 25)
        );
    }

    #[test]
    fn shift_prev_loops_through_consecutive_non_business_days() {
        // GW 想定: 5/3(Sun)・5/4(Mon祝)・5/5(Tue祝) → 5/2(Sat) → 5/1(Fri)
        let holidays: HashSet<NaiveDate> =
            [d(2026, 5, 4), d(2026, 5, 5)].into_iter().collect();
        assert_eq!(
            shift_for_holidays(d(2026, 5, 5), HolidayShift::Prev, &holidays),
            d(2026, 5, 1)
        );
    }

    #[test]
    fn shift_next_loops_through_consecutive_non_business_days() {
        // 5/2(Sat)・5/3(Sun)・5/4(Mon祝)・5/5(Tue祝) → 5/6(Wed)
        let holidays: HashSet<NaiveDate> =
            [d(2026, 5, 4), d(2026, 5, 5)].into_iter().collect();
        assert_eq!(
            shift_for_holidays(d(2026, 5, 2), HolidayShift::Next, &holidays),
            d(2026, 5, 6)
        );
    }

    #[test]
    fn from_db_value_maps_known_values() {
        assert_eq!(HolidayShift::from_db_value(0), Some(HolidayShift::None));
        assert_eq!(HolidayShift::from_db_value(1), Some(HolidayShift::Prev));
        assert_eq!(HolidayShift::from_db_value(2), Some(HolidayShift::Next));
    }

    #[test]
    fn from_db_value_rejects_out_of_range_values() {
        assert_eq!(HolidayShift::from_db_value(3), None);
        assert_eq!(HolidayShift::from_db_value(-1), None);
    }

    #[test]
    fn db_value_round_trips() {
        for shift in [HolidayShift::None, HolidayShift::Prev, HolidayShift::Next] {
            assert_eq!(
                HolidayShift::from_db_value(shift.to_db_value()),
                Some(shift)
            );
        }
    }

    #[tokio::test]
    async fn fetch_holidays_merges_standard_and_custom() {
        let pool = setup_holiday_db(Some("JP")).await;
        insert_standard(&pool, "JP", "2026-05-05").await;
        insert_custom(&pool, 1, "2026-05-07").await;

        let holidays = fetch_holidays(&pool, 1, d(2026, 5, 1), d(2026, 5, 31))
            .await
            .unwrap();

        assert_eq!(holidays.len(), 2);
        assert!(holidays.contains(&d(2026, 5, 5)));
        assert!(holidays.contains(&d(2026, 5, 7)));
    }

    #[tokio::test]
    async fn fetch_holidays_filters_standard_by_user_locale() {
        let pool = setup_holiday_db(Some("US")).await;
        insert_standard(&pool, "JP", "2026-05-05").await;
        insert_standard(&pool, "US", "2026-05-25").await;

        let holidays = fetch_holidays(&pool, 1, d(2026, 5, 1), d(2026, 5, 31))
            .await
            .unwrap();

        assert_eq!(holidays, [d(2026, 5, 25)].into_iter().collect());
    }

    #[tokio::test]
    async fn fetch_holidays_defaults_locale_to_jp_when_null() {
        let pool = setup_holiday_db(None).await;
        insert_standard(&pool, "JP", "2026-05-05").await;

        let holidays = fetch_holidays(&pool, 1, d(2026, 5, 1), d(2026, 5, 31))
            .await
            .unwrap();

        assert!(holidays.contains(&d(2026, 5, 5)));
    }

    #[tokio::test]
    async fn fetch_holidays_ignores_other_users_custom_holidays() {
        let pool = setup_holiday_db(Some("JP")).await;
        sqlx::query(sql_queries::TEST_HOLIDAY_INSERT_USER)
            .bind(2_i64)
            .bind("user2")
            .bind("JP")
            .execute(&pool)
            .await
            .unwrap();
        insert_custom(&pool, 2, "2026-05-07").await;

        let holidays = fetch_holidays(&pool, 1, d(2026, 5, 1), d(2026, 5, 31))
            .await
            .unwrap();

        assert!(holidays.is_empty());
    }

    #[tokio::test]
    async fn fetch_holidays_widens_window_by_14_days_on_both_sides() {
        let pool = setup_holiday_db(Some("JP")).await;
        // 14 日パディングの内側（採用）と外側（除外）の境界
        insert_standard(&pool, "JP", "2026-05-18").await; // start - 14
        insert_standard(&pool, "JP", "2026-05-17").await; // start - 15
        insert_standard(&pool, "JP", "2026-07-14").await; // end + 14
        insert_standard(&pool, "JP", "2026-07-15").await; // end + 15

        let holidays = fetch_holidays(&pool, 1, d(2026, 6, 1), d(2026, 6, 30))
            .await
            .unwrap();
        assert!(holidays.contains(&d(2026, 5, 18)));
        assert!(!holidays.contains(&d(2026, 5, 17)));
        assert!(holidays.contains(&d(2026, 7, 14)));
        assert!(!holidays.contains(&d(2026, 7, 15)));
    }

    #[tokio::test]
    async fn fetch_holidays_skips_unparsable_dates() {
        let pool = setup_holiday_db(Some("JP")).await;
        insert_standard(&pool, "JP", "2026-05-05").await;
        insert_standard(&pool, "JP", "2026-05-XX").await;
        insert_custom(&pool, 1, "not-a-date").await;

        let holidays = fetch_holidays(&pool, 1, d(2026, 5, 1), d(2026, 5, 31))
            .await
            .unwrap();

        assert_eq!(holidays, [d(2026, 5, 5)].into_iter().collect());
    }

    #[tokio::test]
    async fn fetch_holidays_deduplicates_standard_and_custom_same_date() {
        let pool = setup_holiday_db(Some("JP")).await;
        insert_standard(&pool, "JP", "2026-05-05").await;
        insert_custom(&pool, 1, "2026-05-05").await;

        let holidays = fetch_holidays(&pool, 1, d(2026, 5, 1), d(2026, 5, 31))
            .await
            .unwrap();

        assert_eq!(holidays.len(), 1);
    }

    #[tokio::test]
    async fn fetch_holidays_errors_for_unknown_user() {
        let pool = setup_holiday_db(Some("JP")).await;

        let result = fetch_holidays(&pool, 999, d(2026, 5, 1), d(2026, 5, 31)).await;

        assert!(matches!(result, Err(sqlx::Error::RowNotFound)));
    }
}
