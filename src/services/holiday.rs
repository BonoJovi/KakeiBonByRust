//! 休日シフトの共通ロジック。v2.1.0 の繰り返し予定（recurring.rs）と
//! v2.4.0 の集計起算日（period.rs）の両方から利用される。
//!
//! 祝日テーブル（HOLIDAYS_STANDARD + HOLIDAYS_USER_CUSTOM）の取得は呼び出し側の
//! 責任で、本モジュールは「与えられた祝日集合に対する平日判定とシフト計算」だけを担う。

use chrono::{Datelike, Days, NaiveDate, Weekday};
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
