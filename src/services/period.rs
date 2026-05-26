use chrono::{Days, Months, NaiveDate};
use std::collections::HashSet;

use crate::services::holiday::{shift_for_holidays, HolidayShift};

pub fn end_of_month(year: i32, month: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, 1)
        .and_then(|d| d.checked_add_months(Months::new(1)))
        .and_then(|d| d.checked_sub_days(Days::new(1)))
        .expect("end_of_month: year/month must be valid")
}

pub fn resolve_day_or_end(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap_or_else(|| end_of_month(year, month))
}

pub fn monthly_period_bounds(year: i32, month: u32, start_day: u32) -> (NaiveDate, NaiveDate) {
    let start = resolve_day_or_end(year, month, start_day);
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let next_period_start = resolve_day_or_end(next_year, next_month, start_day);
    let end = next_period_start
        .pred_opt()
        .expect("predecessor of next period start must exist");
    (start, end)
}

/// v2.4.0: 月次起算日に休日シフトを適用したサイクル境界を返す。
/// 各月の境界候補日を独立に shift するため、前期終了日 = 次期開始日 - 1 で
/// 自動導出され、サイクル間で重複・欠落が構造的に発生しない（仕様案 D）。
/// `shift` が `HolidayShift::None` のときは `monthly_period_bounds` と同じ結果を返す。
pub fn monthly_period_bounds_with_shift(
    year: i32,
    month: u32,
    start_day: u32,
    shift: HolidayShift,
    holidays: &HashSet<NaiveDate>,
) -> (NaiveDate, NaiveDate) {
    let start_candidate = resolve_day_or_end(year, month, start_day);
    let start = shift_for_holidays(start_candidate, shift, holidays);

    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let next_start_candidate = resolve_day_or_end(next_year, next_month, start_day);
    let next_start = shift_for_holidays(next_start_candidate, shift, holidays);

    let end = next_start
        .pred_opt()
        .expect("predecessor of next period start must exist");
    (start, end)
}

pub fn yearly_period_bounds(
    year: i32,
    start_month: u32,
    start_day: u32,
) -> (NaiveDate, NaiveDate) {
    let start = resolve_day_or_end(year, start_month, start_day);
    let next_period_start = resolve_day_or_end(year + 1, start_month, start_day);
    let end = next_period_start
        .pred_opt()
        .expect("predecessor of next period start must exist");
    (start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn end_of_month_31_day_months() {
        assert_eq!(end_of_month(2026, 1), d(2026, 1, 31));
        assert_eq!(end_of_month(2026, 12), d(2026, 12, 31));
    }

    #[test]
    fn end_of_month_30_day_months() {
        assert_eq!(end_of_month(2026, 4), d(2026, 4, 30));
        assert_eq!(end_of_month(2026, 11), d(2026, 11, 30));
    }

    #[test]
    fn end_of_month_february_non_leap() {
        assert_eq!(end_of_month(2026, 2), d(2026, 2, 28));
    }

    #[test]
    fn end_of_month_february_leap() {
        assert_eq!(end_of_month(2024, 2), d(2024, 2, 29));
    }

    #[test]
    fn resolve_existing_day_returns_that_day() {
        assert_eq!(resolve_day_or_end(2026, 5, 15), d(2026, 5, 15));
    }

    #[test]
    fn resolve_nonexistent_day_falls_back_to_end_of_month() {
        assert_eq!(resolve_day_or_end(2026, 2, 31), d(2026, 2, 28));
        assert_eq!(resolve_day_or_end(2026, 2, 29), d(2026, 2, 28));
        assert_eq!(resolve_day_or_end(2024, 2, 29), d(2024, 2, 29));
        assert_eq!(resolve_day_or_end(2026, 4, 31), d(2026, 4, 30));
    }

    #[test]
    fn monthly_bounds_calendar_default() {
        assert_eq!(
            monthly_period_bounds(2026, 5, 1),
            (d(2026, 5, 1), d(2026, 5, 31))
        );
        assert_eq!(
            monthly_period_bounds(2026, 2, 1),
            (d(2026, 2, 1), d(2026, 2, 28))
        );
    }

    #[test]
    fn monthly_bounds_mid_month_start() {
        assert_eq!(
            monthly_period_bounds(2026, 5, 13),
            (d(2026, 5, 13), d(2026, 6, 12))
        );
        assert_eq!(
            monthly_period_bounds(2026, 5, 27),
            (d(2026, 5, 27), d(2026, 6, 26))
        );
    }

    #[test]
    fn monthly_bounds_year_rollover() {
        assert_eq!(
            monthly_period_bounds(2026, 12, 13),
            (d(2026, 12, 13), d(2027, 1, 12))
        );
        assert_eq!(
            monthly_period_bounds(2026, 12, 1),
            (d(2026, 12, 1), d(2026, 12, 31))
        );
    }

    #[test]
    fn monthly_bounds_end_of_month_clamp() {
        assert_eq!(
            monthly_period_bounds(2026, 1, 31),
            (d(2026, 1, 31), d(2026, 2, 27))
        );
        assert_eq!(
            monthly_period_bounds(2026, 1, 30),
            (d(2026, 1, 30), d(2026, 2, 27))
        );
        assert_eq!(
            monthly_period_bounds(2024, 1, 31),
            (d(2024, 1, 31), d(2024, 2, 28))
        );
    }

    #[test]
    fn monthly_bounds_start_in_february() {
        assert_eq!(
            monthly_period_bounds(2026, 2, 13),
            (d(2026, 2, 13), d(2026, 3, 12))
        );
        assert_eq!(
            monthly_period_bounds(2026, 2, 29),
            (d(2026, 2, 28), d(2026, 3, 28))
        );
    }

    #[test]
    fn yearly_bounds_calendar_default() {
        assert_eq!(
            yearly_period_bounds(2026, 1, 1),
            (d(2026, 1, 1), d(2026, 12, 31))
        );
    }

    #[test]
    fn yearly_bounds_fiscal_april() {
        assert_eq!(
            yearly_period_bounds(2026, 4, 1),
            (d(2026, 4, 1), d(2027, 3, 31))
        );
    }

    #[test]
    fn yearly_bounds_pension_revision_june() {
        assert_eq!(
            yearly_period_bounds(2026, 6, 15),
            (d(2026, 6, 15), d(2027, 6, 14))
        );
    }

    #[test]
    fn yearly_bounds_end_of_month_clamp_non_leap_to_non_leap() {
        assert_eq!(
            yearly_period_bounds(2026, 2, 29),
            (d(2026, 2, 28), d(2027, 2, 27))
        );
    }

    #[test]
    fn yearly_bounds_leap_year_start() {
        assert_eq!(
            yearly_period_bounds(2024, 2, 29),
            (d(2024, 2, 29), d(2025, 2, 27))
        );
    }

    // -------------------------------------------------------------------------
    // v2.4.0: monthly_period_bounds_with_shift tests
    // -------------------------------------------------------------------------

    use std::collections::HashSet;

    fn no_holidays() -> HashSet<NaiveDate> {
        HashSet::new()
    }

    #[test]
    fn monthly_with_shift_none_matches_plain_function() {
        let holidays = no_holidays();
        // 起算日 13 (May 2026 13日 = Wed)
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 5, 13, HolidayShift::None, &holidays),
            monthly_period_bounds(2026, 5, 13)
        );
        // 起算日 1 (calendar default)
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 5, 1, HolidayShift::None, &holidays),
            monthly_period_bounds(2026, 5, 1)
        );
        // 起算日 31 (clamp ケース)
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 1, 31, HolidayShift::None, &holidays),
            monthly_period_bounds(2026, 1, 31)
        );
    }

    #[test]
    fn monthly_with_shift_no_op_when_both_boundaries_are_business_days() {
        let holidays = no_holidays();
        // 2026-05-15 (Fri) と 2026-06-15 (Mon) はいずれも平日なので、
        // Prev/Next どちらでも plain と同じ結果になる
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 5, 15, HolidayShift::Prev, &holidays),
            (d(2026, 5, 15), d(2026, 6, 14))
        );
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 5, 15, HolidayShift::Next, &holidays),
            (d(2026, 5, 15), d(2026, 6, 14))
        );
    }

    /// 次月境界が休日に当たると、当月の start が平日でも end は次月 shift の影響を受ける。
    /// 2026-05-13 (Wed) は平日だが、2026-06-13 (Sat) なので Prev で 6/12 (Fri) に倒れ、
    /// その結果 5月期 end = 6/11 になる（5月期は1日縮む）。案 D の独立シフトの自然な帰結。
    #[test]
    fn monthly_with_shift_next_boundary_shift_affects_current_period_end() {
        let holidays = no_holidays();
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 5, 13, HolidayShift::Prev, &holidays),
            (d(2026, 5, 13), d(2026, 6, 11))
        );
    }

    /// Issue #57 のフラッグシップ例: 起算日 25 (給料日) が日曜の場合、
    /// Prev シフトで前営業日 23 (金) に倒れる。
    /// 2026-01-25 = Sun, 2026-02-25 = Wed (シフトなし)。
    /// 1月期 = 1/23 (Fri) 〜 2/24 (= 2/25 - 1)
    #[test]
    fn monthly_with_shift_prev_salary_day_25_on_sunday() {
        let holidays = no_holidays();
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 1, 25, HolidayShift::Prev, &holidays),
            (d(2026, 1, 23), d(2026, 2, 24))
        );
    }

    /// Next シフトでは 1/25 (Sun) → 1/26 (Mon)。
    /// 2月期は 2/25 (Wed, 変化なし)。1月期 = 1/26 〜 2/24
    #[test]
    fn monthly_with_shift_next_salary_day_25_on_sunday() {
        let holidays = no_holidays();
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 1, 25, HolidayShift::Next, &holidays),
            (d(2026, 1, 26), d(2026, 2, 24))
        );
    }

    /// 案 D シナリオ: 起算日 31 + Next で月跨ぎが起きるケース。
    /// 2026-01-31 (Sat) → 2/2 (Mon)、2026-02-28 (Sat、Feb は 31 がないので clamp) → 3/2 (Mon)。
    /// 1月期 = 2/2 〜 3/1
    #[test]
    fn monthly_with_shift_next_end_of_month_31_crosses_boundary() {
        let holidays = no_holidays();
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 1, 31, HolidayShift::Next, &holidays),
            (d(2026, 2, 2), d(2026, 3, 1))
        );
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 2, 31, HolidayShift::Next, &holidays),
            (d(2026, 3, 2), d(2026, 3, 30))
        );
    }

    /// 起算日 31 + Prev のケース。2026-01-31 (Sat) → 1/30 (Fri)、2026-02-28 (Sat) → 2/27 (Fri)。
    /// 1月期 = 1/30 〜 2/26
    #[test]
    fn monthly_with_shift_prev_end_of_month_31() {
        let holidays = no_holidays();
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 1, 31, HolidayShift::Prev, &holidays),
            (d(2026, 1, 30), d(2026, 2, 26))
        );
    }

    /// 祝日連鎖: 2026-05-05 (こどもの日, Tue) を起算日にすると、
    /// Prev シフトで 5/4 (みどりの日 Mon) もスキップして 5/1 (Fri) まで遡る。
    /// 6/5 は通常の Fri なので 6/4 で 5月期終了。
    /// 5月期 = 5/1 〜 6/4
    #[test]
    fn monthly_with_shift_prev_loops_through_consecutive_holidays() {
        let holidays: HashSet<NaiveDate> =
            [d(2026, 5, 4), d(2026, 5, 5)].into_iter().collect();
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 5, 5, HolidayShift::Prev, &holidays),
            (d(2026, 5, 1), d(2026, 6, 4))
        );
    }

    /// 年跨ぎ: 起算日 25 + Prev で 2026-12-25 (Fri, 平日) と 2027-01-25 (Mon, 平日)
    /// は両方ともシフトしない。12月期 = 12/25 〜 1/24
    #[test]
    fn monthly_with_shift_year_rollover_no_shift_needed() {
        let holidays = no_holidays();
        assert_eq!(
            monthly_period_bounds_with_shift(2026, 12, 25, HolidayShift::Prev, &holidays),
            (d(2026, 12, 25), d(2027, 1, 24))
        );
    }
}
