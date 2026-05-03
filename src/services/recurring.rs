//! 繰り返し予定入出金（v2.1.0）の周期計算。純粋関数として切り出し、
//! DB / Tauri コマンド層から独立してテスト可能にする。

use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
use std::collections::HashSet;

/// 周期と起点を一体で表現する。`unit` と「いつ発生するか」のアンカー情報を
/// バリアントごとに固定することで、不正な組み合わせ（例: Day なのに DayOfMonth が
/// 指定される）を型レベルで排除する。
///
/// DDL 対応:
/// - Daily   → PERIOD_UNIT='DAY',   PERIOD_INTERVAL, ANCHOR_DATE
/// - Weekly  → PERIOD_UNIT='WEEK',  PERIOD_INTERVAL, DAY_OF_WEEK
/// - Monthly → PERIOD_UNIT='MONTH', PERIOD_INTERVAL, day_rule（複数カラム）
/// - Yearly  → PERIOD_UNIT='YEAR',  PERIOD_INTERVAL, MONTH_OF_YEAR, day_rule
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cycle {
    Daily {
        interval: u32,
        anchor: NaiveDate,
    },
    Weekly {
        interval: u32,
        weekday: Weekday,
    },
    Monthly {
        interval: u32,
        day_rule: MonthlyDayRule,
    },
    Yearly {
        interval: u32,
        /// MONTH_OF_YEAR: 1..=12
        month: u32,
        day_rule: MonthlyDayRule,
    },
}

/// 月内のどの日に発生するかの規則。Monthly / Yearly で共有。
///
/// DDL 対応（v2.1.0 で要再設計、Task #6 参照）:
/// - DayOfMonth      → 指定日固定、無い月はスキップ
/// - DayOfMonthOrEnd → 指定日希望、無い月は月末で代用（クレカ 29 日引落など）
/// - EndOfMonth      → 常に月末
/// - NthWeekday      → 第N週の指定曜日（KakeiBon の差別化要因）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthlyDayRule {
    DayOfMonth { day: u32 },
    DayOfMonthOrEnd { day: u32 },
    EndOfMonth,
    /// 第N週の指定曜日。week=5 は最終週として扱う。
    NthWeekday { week: u32, weekday: Weekday },
}

/// HOLIDAY_SHIFT_TYPE。RECURRING_RULES.HOLIDAY_SHIFT_TYPE に対応。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HolidayShift {
    /// HOLIDAY_SHIFT_NONE = 0
    None,
    /// HOLIDAY_SHIFT_PREV = 1（給料日想定：直前の平日）
    Prev,
    /// HOLIDAY_SHIFT_NEXT = 2（引落想定：直後の平日）
    Next,
}

/// 周期仕様。DB row や HEADER テンプレ部分は含めず、本モジュールは「いつ発生するか」
/// の計算だけに責任を持つ。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyclicSpec {
    pub cycle: Cycle,
    pub holiday_shift: HolidayShift,
}

/// 周期仕様と期間（START_DATE, END_DATE; 両端含む）から発生日を列挙する。
/// 起点（anchor / day_rule）は cycle 側に閉じ込め、`start..=end` は期間フィルタ専用。
/// 祝日は呼び出し側が用意し、純粋関数として保つ。
pub fn generate_dates(
    spec: &CyclicSpec,
    start: NaiveDate,
    end: NaiveDate,
    holidays: &HashSet<NaiveDate>,
) -> Vec<NaiveDate> {
    let _ = holidays; // Task #5 で使用

    if start > end {
        return Vec::new();
    }

    match &spec.cycle {
        Cycle::Daily { interval, anchor } => {
            debug_assert!(*interval >= 1, "interval must be >= 1");
            generate_daily(*interval, *anchor, start, end)
        }
        Cycle::Monthly { interval, day_rule } => {
            debug_assert!(*interval >= 1, "interval must be >= 1");
            generate_monthly(*interval, day_rule, start, end)
        }
        Cycle::Weekly { interval, weekday } => {
            debug_assert!(*interval >= 1, "interval must be >= 1");
            generate_weekly(*interval, *weekday, start, end)
        }
        Cycle::Yearly { interval, month, day_rule } => {
            debug_assert!(*interval >= 1, "interval must be >= 1");
            debug_assert!(*month >= 1 && *month <= 12, "month must be 1..=12");
            generate_yearly(*interval, *month, day_rule, start, end)
        }
    }
}

fn generate_daily(
    interval: u32,
    anchor: NaiveDate,
    start: NaiveDate,
    end: NaiveDate,
) -> Vec<NaiveDate> {
    let step = Days::new(interval as u64);
    let mut out = Vec::new();
    let mut d = anchor;

    // anchor < start なら start まで早送り
    while d < start {
        match d.checked_add_days(step) {
            Some(next) => d = next,
            None => return out,
        }
    }
    while d <= end {
        out.push(d);
        match d.checked_add_days(step) {
            Some(next) => d = next,
            None => break,
        }
    }
    out
}

/// start 以降の最初の指定曜日を返す（start 当日が一致すれば start 自身）。
/// Weekly の anchor を導出する純粋関数。
fn derive_weekly_anchor(start: NaiveDate, weekday: Weekday) -> NaiveDate {
    let start_w = start.weekday().num_days_from_monday();
    let target_w = weekday.num_days_from_monday();
    let diff = (target_w + 7 - start_w) % 7;
    start + Days::new(diff as u64)
}

fn generate_weekly(
    interval: u32,
    weekday: Weekday,
    start: NaiveDate,
    end: NaiveDate,
) -> Vec<NaiveDate> {
    let anchor = derive_weekly_anchor(start, weekday);
    let step = Days::new(interval as u64 * 7);
    let mut out = Vec::new();
    let mut d = anchor;
    while d <= end {
        out.push(d);
        match d.checked_add_days(step) {
            Some(next) => d = next,
            None => break,
        }
    }
    out
}

/// 指定した年月の月末日を返す（翌月 1 日 - 1 日）。
fn end_of_month(year: i32, month: u32) -> Option<NaiveDate> {
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    first
        .checked_add_months(Months::new(1))?
        .checked_sub_days(Days::new(1))
}

/// 指定した年月の第N週の指定曜日を返す。week=5 は「最終出現の指定曜日」として
/// 扱う（4 回しかない月は 4 回目で代用）。指定曜日がその月に N 回未満（かつ week<5）
/// の場合は None。
fn nth_weekday_of_month(
    year: i32,
    month: u32,
    week: u32,
    weekday: Weekday,
) -> Option<NaiveDate> {
    let first = NaiveDate::from_ymd_opt(year, month, 1)?;
    let first_w = first.weekday().num_days_from_monday() as i32;
    let target_w = weekday.num_days_from_monday() as i32;
    let diff = ((target_w - first_w) + 7) % 7;
    let mut d = first.checked_add_days(Days::new(diff as u64))?;

    let mut candidates: Vec<NaiveDate> = Vec::with_capacity(5);
    while d.month() == month {
        candidates.push(d);
        match d.checked_add_days(Days::new(7)) {
            Some(next) => d = next,
            None => break,
        }
    }

    if week == 5 {
        candidates.last().copied()
    } else {
        candidates.get(week.saturating_sub(1) as usize).copied()
    }
}

fn generate_monthly(
    interval: u32,
    rule: &MonthlyDayRule,
    start: NaiveDate,
    end: NaiveDate,
) -> Vec<NaiveDate> {
    let mut out = Vec::new();
    let mut year = start.year();
    let mut month = start.month();

    loop {
        let candidate = match *rule {
            MonthlyDayRule::DayOfMonth { day } => NaiveDate::from_ymd_opt(year, month, day),
            MonthlyDayRule::DayOfMonthOrEnd { day } => NaiveDate::from_ymd_opt(year, month, day)
                .or_else(|| end_of_month(year, month)),
            MonthlyDayRule::EndOfMonth => end_of_month(year, month),
            MonthlyDayRule::NthWeekday { week, weekday } => {
                nth_weekday_of_month(year, month, week, weekday)
            }
        };
        if let Some(d) = candidate {
            if d > end {
                break;
            }
            if d >= start {
                out.push(d);
            }
        }
        // 候補日が無い月（DayOfMonth=31 の 2 月など）は黙ってスキップ。
        // 「その月の 1 日が end を超えていれば break」で無限ループを防ぐ。
        match NaiveDate::from_ymd_opt(year, month, 1) {
            Some(first) if first > end => break,
            _ => {}
        }
        // interval ヶ月進める。month は 1..=12、m_zero は 0 起点で計算してから戻す。
        let m_zero = month as i32 - 1 + interval as i32;
        year += m_zero / 12;
        month = (m_zero % 12 + 1) as u32;
    }
    out
}

fn generate_yearly(
    interval: u32,
    month: u32,
    rule: &MonthlyDayRule,
    start: NaiveDate,
    end: NaiveDate,
) -> Vec<NaiveDate> {
    let mut out = Vec::new();
    let mut year = start.year();

    loop {
        let candidate = match *rule {
            MonthlyDayRule::DayOfMonth { day } => NaiveDate::from_ymd_opt(year, month, day),
            MonthlyDayRule::DayOfMonthOrEnd { day } => NaiveDate::from_ymd_opt(year, month, day)
                .or_else(|| end_of_month(year, month)),
            MonthlyDayRule::EndOfMonth => end_of_month(year, month),
            MonthlyDayRule::NthWeekday { week, weekday } => {
                nth_weekday_of_month(year, month, week, weekday)
            }
        };
        if let Some(d) = candidate {
            if d > end {
                break;
            }
            if d >= start {
                out.push(d);
            }
        }
        // 候補が無い年（うるう年以外の 2/29 など）は年だけ進める。
        // 「指定月の 1 日が end を超えていれば break」で無限ループを防ぐ。
        match NaiveDate::from_ymd_opt(year, month, 1) {
            Some(first) if first > end => break,
            _ => {}
        }
        year += interval as i32;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn no_holidays() -> HashSet<NaiveDate> {
        HashSet::new()
    }

    fn spec_daily(interval: u32, anchor: NaiveDate) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Daily { interval, anchor },
            holiday_shift: HolidayShift::None,
        }
    }

    fn spec_monthly_dom(interval: u32, day: u32) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Monthly {
                interval,
                day_rule: MonthlyDayRule::DayOfMonth { day },
            },
            holiday_shift: HolidayShift::None,
        }
    }

    fn spec_weekly(interval: u32, weekday: Weekday) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Weekly { interval, weekday },
            holiday_shift: HolidayShift::None,
        }
    }

    fn spec_monthly_eom(interval: u32) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Monthly {
                interval,
                day_rule: MonthlyDayRule::EndOfMonth,
            },
            holiday_shift: HolidayShift::None,
        }
    }

    fn spec_monthly_nth(interval: u32, week: u32, weekday: Weekday) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Monthly {
                interval,
                day_rule: MonthlyDayRule::NthWeekday { week, weekday },
            },
            holiday_shift: HolidayShift::None,
        }
    }

    fn spec_monthly_dom_or_end(interval: u32, day: u32) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Monthly {
                interval,
                day_rule: MonthlyDayRule::DayOfMonthOrEnd { day },
            },
            holiday_shift: HolidayShift::None,
        }
    }

    fn spec_yearly_dom(interval: u32, month: u32, day: u32) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Yearly {
                interval,
                month,
                day_rule: MonthlyDayRule::DayOfMonth { day },
            },
            holiday_shift: HolidayShift::None,
        }
    }

    fn spec_yearly_dom_or_end(interval: u32, month: u32, day: u32) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Yearly {
                interval,
                month,
                day_rule: MonthlyDayRule::DayOfMonthOrEnd { day },
            },
            holiday_shift: HolidayShift::None,
        }
    }

    fn spec_yearly_eom(interval: u32, month: u32) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Yearly {
                interval,
                month,
                day_rule: MonthlyDayRule::EndOfMonth,
            },
            holiday_shift: HolidayShift::None,
        }
    }

    fn spec_yearly_nth(interval: u32, month: u32, week: u32, weekday: Weekday) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Yearly {
                interval,
                month,
                day_rule: MonthlyDayRule::NthWeekday { week, weekday },
            },
            holiday_shift: HolidayShift::None,
        }
    }

    /// ❶ n 日毎（最も単純）。anchor=start、両端含む。
    #[test]
    fn case1_daily_every_7_days_inclusive() {
        let result = generate_dates(
            &spec_daily(7, d(2026, 1, 1)),
            d(2026, 1, 1),
            d(2026, 1, 22),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 1), d(2026, 1, 8), d(2026, 1, 15), d(2026, 1, 22)]
        );
    }

    /// ❶' anchor が start より前にある場合、start 以降のみ採用。
    #[test]
    fn case1b_daily_anchor_before_start() {
        // anchor=1/1 で 7 日刻み、ただし期間は 1/15 から
        let result = generate_dates(
            &spec_daily(7, d(2026, 1, 1)),
            d(2026, 1, 15),
            d(2026, 1, 30),
            &no_holidays(),
        );
        assert_eq!(result, vec![d(2026, 1, 15), d(2026, 1, 22), d(2026, 1, 29)]);
    }

    /// ❷ n 月毎・同日（毎月 25 日）。
    #[test]
    fn case2_monthly_same_day_25th() {
        let result = generate_dates(
            &spec_monthly_dom(1, 25),
            d(2026, 1, 1),
            d(2026, 4, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 25), d(2026, 2, 25), d(2026, 3, 25), d(2026, 4, 25)]
        );
    }

    /// ❸ start 日が指定日より後 → 同月をスキップ。
    #[test]
    fn case3_monthly_start_after_target_day() {
        let result = generate_dates(
            &spec_monthly_dom(1, 25),
            d(2026, 1, 26),
            d(2026, 4, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 2, 25), d(2026, 3, 25), d(2026, 4, 25)]
        );
    }

    /// ❹ 隔月（interval=2）は start を含む月から起算（案α）。
    #[test]
    fn case4_bimonthly_starts_from_start_month() {
        let result = generate_dates(
            &spec_monthly_dom(2, 25),
            d(2026, 1, 1),
            d(2026, 6, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 25), d(2026, 3, 25), d(2026, 5, 25)]
        );
    }

    /// ❺ DAY_OF_MONTH=31 は存在しない月をスキップ（案A）。
    #[test]
    fn case5_day_31_skips_short_months() {
        let result = generate_dates(
            &spec_monthly_dom(1, 31),
            d(2026, 1, 1),
            d(2026, 4, 30),
            &no_holidays(),
        );
        assert_eq!(result, vec![d(2026, 1, 31), d(2026, 3, 31)]);
    }

    /// W❶ 毎週月曜（start=木曜 → 最初の月曜は 1/5）。
    #[test]
    fn case_w1_weekly_monday_from_thursday() {
        let result = generate_dates(
            &spec_weekly(1, Weekday::Mon),
            d(2026, 1, 1),
            d(2026, 1, 31),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 5), d(2026, 1, 12), d(2026, 1, 19), d(2026, 1, 26)]
        );
    }

    /// W❷ 隔週月曜（interval=2、start=木曜 → 最初の月曜は 1/5）。
    #[test]
    fn case_w2_biweekly_monday_from_thursday() {
        let result = generate_dates(
            &spec_weekly(2, Weekday::Mon),
            d(2026, 1, 1),
            d(2026, 2, 28),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 5), d(2026, 1, 19), d(2026, 2, 2), d(2026, 2, 16)]
        );
    }

    /// W❸ start が指定曜日と一致（start 自身を anchor 採用）。
    #[test]
    fn case_w3_weekly_thursday_from_thursday() {
        let result = generate_dates(
            &spec_weekly(1, Weekday::Thu),
            d(2026, 1, 1),
            d(2026, 1, 22),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 1), d(2026, 1, 8), d(2026, 1, 15), d(2026, 1, 22)]
        );
    }

    /// W❹ 隔週で start が指定曜日と一致。
    #[test]
    fn case_w4_biweekly_thursday_from_thursday() {
        let result = generate_dates(
            &spec_weekly(2, Weekday::Thu),
            d(2026, 1, 1),
            d(2026, 2, 28),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![
                d(2026, 1, 1),
                d(2026, 1, 15),
                d(2026, 1, 29),
                d(2026, 2, 12),
                d(2026, 2, 26)
            ]
        );
    }

    /// EOM❶ 毎月末。
    #[test]
    fn case_eom1_monthly_end_of_month() {
        let result = generate_dates(
            &spec_monthly_eom(1),
            d(2026, 1, 1),
            d(2026, 4, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 31), d(2026, 2, 28), d(2026, 3, 31), d(2026, 4, 30)]
        );
    }

    /// EOM❷ 隔月末。
    #[test]
    fn case_eom2_bimonthly_end_of_month() {
        let result = generate_dates(
            &spec_monthly_eom(2),
            d(2026, 1, 1),
            d(2026, 6, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 31), d(2026, 3, 31), d(2026, 5, 31)]
        );
    }

    /// EOM❸ うるう年 2 月（2024 年）。
    #[test]
    fn case_eom3_leap_year_february() {
        let result = generate_dates(
            &spec_monthly_eom(1),
            d(2024, 2, 1),
            d(2024, 4, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2024, 2, 29), d(2024, 3, 31), d(2024, 4, 30)]
        );
    }

    /// EOM❹ start が直前の月末より後（その月をスキップせず、月末は含まれる）。
    #[test]
    fn case_eom4_start_after_previous_month_end() {
        let result = generate_dates(
            &spec_monthly_eom(1),
            d(2026, 3, 1),
            d(2026, 5, 31),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 3, 31), d(2026, 4, 30), d(2026, 5, 31)]
        );
    }

    /// NW❶ 第4木曜日（4 回月と 5 回月が混在）。
    #[test]
    fn case_nw1_4th_thursday_mixed_4_and_5_occurrences() {
        let result = generate_dates(
            &spec_monthly_nth(1, 4, Weekday::Thu),
            d(2026, 1, 1),
            d(2026, 4, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 22), d(2026, 2, 26), d(2026, 3, 26), d(2026, 4, 23)]
        );
    }

    /// NW❷ week=5 は最終出現で代用（5 回月はそのまま、4 回月は 4 回目）。
    #[test]
    fn case_nw2_5th_week_falls_back_to_last() {
        let result = generate_dates(
            &spec_monthly_nth(1, 5, Weekday::Thu),
            d(2026, 1, 1),
            d(2026, 4, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 29), d(2026, 2, 26), d(2026, 3, 26), d(2026, 4, 30)]
        );
    }

    /// NW❸ 第1日曜（月初が日曜の月を含む）。
    #[test]
    fn case_nw3_1st_sunday_including_first_of_month() {
        let result = generate_dates(
            &spec_monthly_nth(1, 1, Weekday::Sun),
            d(2026, 1, 1),
            d(2026, 4, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 4), d(2026, 2, 1), d(2026, 3, 1), d(2026, 4, 5)]
        );
    }

    /// NW❹ 隔月の第2金曜。
    #[test]
    fn case_nw4_bimonthly_2nd_friday() {
        let result = generate_dates(
            &spec_monthly_nth(2, 2, Weekday::Fri),
            d(2026, 1, 1),
            d(2026, 6, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 9), d(2026, 3, 13), d(2026, 5, 8)]
        );
    }

    /// case5b: DayOfMonthOrEnd=31 は短い月で月末代用される（case5 のスキップ版と対）。
    #[test]
    fn case5b_dom31_or_end_clamps_to_eom() {
        let result = generate_dates(
            &spec_monthly_dom_or_end(1, 31),
            d(2026, 1, 1),
            d(2026, 4, 30),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 1, 31), d(2026, 2, 28), d(2026, 3, 31), d(2026, 4, 30)]
        );
    }

    /// Y❶ 毎年 5/10（DayOfMonth）。
    #[test]
    fn case_y1_yearly_may_10th() {
        let result = generate_dates(
            &spec_yearly_dom(1, 5, 10),
            d(2026, 1, 1),
            d(2028, 12, 31),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 5, 10), d(2027, 5, 10), d(2028, 5, 10)]
        );
    }

    /// Y❷ 隔年 12月末（EndOfMonth）。
    #[test]
    fn case_y2_biyearly_december_end() {
        let result = generate_dates(
            &spec_yearly_eom(2, 12),
            d(2026, 1, 1),
            d(2030, 12, 31),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 12, 31), d(2028, 12, 31), d(2030, 12, 31)]
        );
    }

    /// Y❸ 毎年 11月第4木曜（NthWeekday、米国 Thanksgiving パターン）。
    #[test]
    fn case_y3_yearly_4th_thursday_of_november() {
        let result = generate_dates(
            &spec_yearly_nth(1, 11, 4, Weekday::Thu),
            d(2026, 1, 1),
            d(2028, 12, 31),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![d(2026, 11, 26), d(2027, 11, 25), d(2028, 11, 23)]
        );
    }

    /// Y❹ 毎年 2/29（DayOfMonth）→ うるう年以外はスキップ。
    #[test]
    fn case_y4_yearly_feb_29_skips_non_leap() {
        let result = generate_dates(
            &spec_yearly_dom(1, 2, 29),
            d(2024, 1, 1),
            d(2028, 12, 31),
            &no_holidays(),
        );
        assert_eq!(result, vec![d(2024, 2, 29), d(2028, 2, 29)]);
    }

    /// Y❹b 毎年 2/29（DayOfMonthOrEnd）→ 平年は 2/28 で代用。
    #[test]
    fn case_y4b_yearly_feb_29_or_end_clamps_in_non_leap() {
        let result = generate_dates(
            &spec_yearly_dom_or_end(1, 2, 29),
            d(2024, 1, 1),
            d(2028, 12, 31),
            &no_holidays(),
        );
        assert_eq!(
            result,
            vec![
                d(2024, 2, 29),
                d(2025, 2, 28),
                d(2026, 2, 28),
                d(2027, 2, 28),
                d(2028, 2, 29),
            ]
        );
    }

    /// 入力境界：start > end は空。
    #[test]
    fn case_empty_when_start_after_end() {
        let result = generate_dates(
            &spec_daily(1, d(2026, 5, 10)),
            d(2026, 5, 10),
            d(2026, 5, 1),
            &no_holidays(),
        );
        assert!(result.is_empty());
    }
}
