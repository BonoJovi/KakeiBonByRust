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

    fn d(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
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
}
