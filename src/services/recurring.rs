//! 繰り返し予定入出金（v2.1.0）の周期計算。純粋関数として切り出し、
//! DB / Tauri コマンド層から独立してテスト可能にする。

use chrono::{Datelike, Days, NaiveDate, Weekday};
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
/// DDL 対応:
/// - DayOfMonth → DAY_OF_MONTH
/// - EndOfMonth → IS_END_OF_MONTH=1
/// - NthWeekday → WEEK_OF_MONTH + DAY_OF_WEEK（KakeiBon の差別化要因）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonthlyDayRule {
    DayOfMonth { day: u32 },
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
        Cycle::Weekly { .. } => Vec::new(),  // Task #4
        Cycle::Yearly { .. } => Vec::new(),  // Task #4
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

fn generate_monthly(
    interval: u32,
    rule: &MonthlyDayRule,
    start: NaiveDate,
    end: NaiveDate,
) -> Vec<NaiveDate> {
    let day = match *rule {
        MonthlyDayRule::DayOfMonth { day } => day,
        // EndOfMonth / NthWeekday は Task #4 で実装
        _ => return Vec::new(),
    };

    let mut out = Vec::new();
    let mut year = start.year();
    let mut month = start.month();

    loop {
        if let Some(d) = NaiveDate::from_ymd_opt(year, month, day) {
            if d > end {
                break;
            }
            if d >= start {
                out.push(d);
            }
        }
        // 短い月（day が存在しない月）は黙ってスキップして次へ進む。
        // ただし「その月の 1 日が end を超えていれば break」で無限ループを防ぐ。
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
