//! 繰り返し予定入出金（v2.1.0）の周期計算と DB アクセス。
//! 周期計算は純粋関数として切り出し、DB / Tauri コマンド層から独立してテスト可能にしている。

use chrono::{Datelike, Days, Months, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashSet;

use crate::services::period::end_of_month;
use crate::{sql_queries, consts};

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
///
/// holiday_shift が None 以外の場合、生成された日付に休日シフトを適用する。
/// シフト先が start..end の範囲外（例：1/1 → 12/31）になっても採用する
/// （カレンダー上の挙動を尊重）。シフト後にソートのみ行い、重複は除去しない
/// （同日に複数発生は意味のある情報として保持）。
pub fn generate_dates(
    spec: &CyclicSpec,
    start: NaiveDate,
    end: NaiveDate,
    holidays: &HashSet<NaiveDate>,
) -> Vec<NaiveDate> {
    if start > end {
        return Vec::new();
    }

    let base = match &spec.cycle {
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
    };

    if matches!(spec.holiday_shift, HolidayShift::None) {
        return base;
    }
    let mut shifted: Vec<NaiveDate> = base
        .into_iter()
        .map(|d| shift_for_holidays(d, spec.holiday_shift, holidays))
        .collect();
    shifted.sort();
    shifted
}

/// 土日 + 祝日テーブルに含まれる日を非平日とみなす。
fn is_non_business_day(d: NaiveDate, holidays: &HashSet<NaiveDate>) -> bool {
    matches!(d.weekday(), Weekday::Sat | Weekday::Sun) || holidays.contains(&d)
}

/// 休日シフト。指定方向に「平日にぶつかるまで」進める／遡る。
/// シフト結果が start..end の範囲外になっても採用する（呼び出し側で集約せず、
/// カレンダー上の挙動をそのまま返す）。
fn shift_for_holidays(
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
            MonthlyDayRule::DayOfMonthOrEnd { day } => Some(crate::services::period::resolve_day_or_end(year, month, day)),
            MonthlyDayRule::EndOfMonth => Some(end_of_month(year, month)),
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
            MonthlyDayRule::DayOfMonthOrEnd { day } => Some(crate::services::period::resolve_day_or_end(year, month, day)),
            MonthlyDayRule::EndOfMonth => Some(end_of_month(year, month)),
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

// ============================================================================
// DB row ↔ CyclicSpec 変換層
// ============================================================================

/// RECURRING_RULES の周期関連カラムと HOLIDAY_SHIFT_TYPE をまとめた中間表現。
/// SQLx のクエリ層と純粋関数 (CyclicSpec) を繋ぐ橋渡し。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleColumns {
    pub period_unit: String,
    pub period_interval: u32,
    pub anchor_date: Option<NaiveDate>,
    /// ISO 8601 番号: 1=Mon, 2=Tue, ..., 7=Sun。
    /// UI 層で WEEK_START_DAY を考慮した表示順に並べ替える前提（DB は曜日そのもの）。
    pub day_of_week: Option<u32>,
    pub month_day_rule_type: Option<String>,
    pub day_of_month: Option<u32>,
    pub week_of_month: Option<u32>,
    pub month_of_year: Option<u32>,
    pub holiday_shift_type: i32,
}

fn weekday_to_iso(w: Weekday) -> u32 {
    w.number_from_monday()
}

fn iso_to_weekday(n: u32) -> Option<Weekday> {
    match n {
        1 => Some(Weekday::Mon),
        2 => Some(Weekday::Tue),
        3 => Some(Weekday::Wed),
        4 => Some(Weekday::Thu),
        5 => Some(Weekday::Fri),
        6 => Some(Weekday::Sat),
        7 => Some(Weekday::Sun),
        _ => None,
    }
}

/// CyclicSpec を DB 行用カラムに変換する純粋関数。
pub fn cyclic_spec_to_columns(spec: &CyclicSpec) -> CycleColumns {
    use crate::consts::{
        HOLIDAY_SHIFT_NEXT, HOLIDAY_SHIFT_NONE, HOLIDAY_SHIFT_PREV, PERIOD_UNIT_DAY,
        PERIOD_UNIT_MONTH, PERIOD_UNIT_WEEK, PERIOD_UNIT_YEAR,
    };

    let holiday_shift_type = match spec.holiday_shift {
        HolidayShift::None => HOLIDAY_SHIFT_NONE,
        HolidayShift::Prev => HOLIDAY_SHIFT_PREV,
        HolidayShift::Next => HOLIDAY_SHIFT_NEXT,
    };

    match &spec.cycle {
        Cycle::Daily { interval, anchor } => CycleColumns {
            period_unit: PERIOD_UNIT_DAY.to_string(),
            period_interval: *interval,
            anchor_date: Some(*anchor),
            day_of_week: None,
            month_day_rule_type: None,
            day_of_month: None,
            week_of_month: None,
            month_of_year: None,
            holiday_shift_type,
        },
        Cycle::Weekly { interval, weekday } => CycleColumns {
            period_unit: PERIOD_UNIT_WEEK.to_string(),
            period_interval: *interval,
            anchor_date: None,
            day_of_week: Some(weekday_to_iso(*weekday)),
            month_day_rule_type: None,
            day_of_month: None,
            week_of_month: None,
            month_of_year: None,
            holiday_shift_type,
        },
        Cycle::Monthly { interval, day_rule } => {
            let (rule_type, day, week, dow) = monthly_rule_to_columns(day_rule);
            CycleColumns {
                period_unit: PERIOD_UNIT_MONTH.to_string(),
                period_interval: *interval,
                anchor_date: None,
                day_of_week: dow,
                month_day_rule_type: Some(rule_type.to_string()),
                day_of_month: day,
                week_of_month: week,
                month_of_year: None,
                holiday_shift_type,
            }
        }
        Cycle::Yearly { interval, month, day_rule } => {
            let (rule_type, day, week, dow) = monthly_rule_to_columns(day_rule);
            CycleColumns {
                period_unit: PERIOD_UNIT_YEAR.to_string(),
                period_interval: *interval,
                anchor_date: None,
                day_of_week: dow,
                month_day_rule_type: Some(rule_type.to_string()),
                day_of_month: day,
                week_of_month: week,
                month_of_year: Some(*month),
                holiday_shift_type,
            }
        }
    }
}

fn monthly_rule_to_columns(
    rule: &MonthlyDayRule,
) -> (&'static str, Option<u32>, Option<u32>, Option<u32>) {
    use crate::consts::{
        MONTH_DAY_RULE_TYPE_DAY, MONTH_DAY_RULE_TYPE_DAY_OR_END, MONTH_DAY_RULE_TYPE_END,
        MONTH_DAY_RULE_TYPE_NTH_WEEKDAY,
    };
    match *rule {
        MonthlyDayRule::DayOfMonth { day } => {
            (MONTH_DAY_RULE_TYPE_DAY, Some(day), None, None)
        }
        MonthlyDayRule::DayOfMonthOrEnd { day } => {
            (MONTH_DAY_RULE_TYPE_DAY_OR_END, Some(day), None, None)
        }
        MonthlyDayRule::EndOfMonth => (MONTH_DAY_RULE_TYPE_END, None, None, None),
        MonthlyDayRule::NthWeekday { week, weekday } => (
            MONTH_DAY_RULE_TYPE_NTH_WEEKDAY,
            None,
            Some(week),
            Some(weekday_to_iso(weekday)),
        ),
    }
}

/// CycleColumns から CyclicSpec を構築する純粋関数。値域・必須項目を検証する。
pub fn columns_to_cyclic_spec(cols: &CycleColumns) -> Result<CyclicSpec, String> {
    use crate::consts::{
        HOLIDAY_SHIFT_NEXT, HOLIDAY_SHIFT_NONE, HOLIDAY_SHIFT_PREV, PERIOD_UNIT_DAY,
        PERIOD_UNIT_MONTH, PERIOD_UNIT_WEEK, PERIOD_UNIT_YEAR,
    };

    let holiday_shift = match cols.holiday_shift_type {
        HOLIDAY_SHIFT_NONE => HolidayShift::None,
        HOLIDAY_SHIFT_PREV => HolidayShift::Prev,
        HOLIDAY_SHIFT_NEXT => HolidayShift::Next,
        v => return Err(format!("invalid HOLIDAY_SHIFT_TYPE: {}", v)),
    };

    if cols.period_interval < 1 {
        return Err(format!(
            "PERIOD_INTERVAL must be >= 1, got {}",
            cols.period_interval
        ));
    }

    let cycle = match cols.period_unit.as_str() {
        PERIOD_UNIT_DAY => {
            let anchor = cols
                .anchor_date
                .ok_or_else(|| "PERIOD_UNIT='DAY' requires ANCHOR_DATE".to_string())?;
            Cycle::Daily {
                interval: cols.period_interval,
                anchor,
            }
        }
        PERIOD_UNIT_WEEK => {
            let dow = cols
                .day_of_week
                .ok_or_else(|| "PERIOD_UNIT='WEEK' requires DAY_OF_WEEK".to_string())?;
            let weekday = iso_to_weekday(dow)
                .ok_or_else(|| format!("invalid DAY_OF_WEEK (must be 1..=7, got {})", dow))?;
            Cycle::Weekly {
                interval: cols.period_interval,
                weekday,
            }
        }
        PERIOD_UNIT_MONTH => {
            let day_rule = columns_to_monthly_rule(cols)?;
            Cycle::Monthly {
                interval: cols.period_interval,
                day_rule,
            }
        }
        PERIOD_UNIT_YEAR => {
            let month = cols
                .month_of_year
                .ok_or_else(|| "PERIOD_UNIT='YEAR' requires MONTH_OF_YEAR".to_string())?;
            if !(1..=12).contains(&month) {
                return Err(format!("MONTH_OF_YEAR must be 1..=12, got {}", month));
            }
            let day_rule = columns_to_monthly_rule(cols)?;
            Cycle::Yearly {
                interval: cols.period_interval,
                month,
                day_rule,
            }
        }
        other => return Err(format!("invalid PERIOD_UNIT: {}", other)),
    };

    Ok(CyclicSpec {
        cycle,
        holiday_shift,
    })
}

fn columns_to_monthly_rule(cols: &CycleColumns) -> Result<MonthlyDayRule, String> {
    use crate::consts::{
        MONTH_DAY_RULE_TYPE_DAY, MONTH_DAY_RULE_TYPE_DAY_OR_END, MONTH_DAY_RULE_TYPE_END,
        MONTH_DAY_RULE_TYPE_NTH_WEEKDAY,
    };
    let rule_type = cols
        .month_day_rule_type
        .as_deref()
        .ok_or_else(|| "Monthly/Yearly cycles require MONTH_DAY_RULE_TYPE".to_string())?;
    match rule_type {
        MONTH_DAY_RULE_TYPE_DAY => {
            let day = cols
                .day_of_month
                .ok_or_else(|| "MONTH_DAY_RULE_TYPE='DAY' requires DAY_OF_MONTH".to_string())?;
            if !(1..=31).contains(&day) {
                return Err(format!("DAY_OF_MONTH must be 1..=31, got {}", day));
            }
            Ok(MonthlyDayRule::DayOfMonth { day })
        }
        MONTH_DAY_RULE_TYPE_DAY_OR_END => {
            let day = cols
                .day_of_month
                .ok_or_else(|| "MONTH_DAY_RULE_TYPE='DAY_OR_END' requires DAY_OF_MONTH".to_string())?;
            if !(1..=31).contains(&day) {
                return Err(format!("DAY_OF_MONTH must be 1..=31, got {}", day));
            }
            Ok(MonthlyDayRule::DayOfMonthOrEnd { day })
        }
        MONTH_DAY_RULE_TYPE_END => Ok(MonthlyDayRule::EndOfMonth),
        MONTH_DAY_RULE_TYPE_NTH_WEEKDAY => {
            let week = cols
                .week_of_month
                .ok_or_else(|| "MONTH_DAY_RULE_TYPE='NTH_WEEKDAY' requires WEEK_OF_MONTH".to_string())?;
            if !(1..=5).contains(&week) {
                return Err(format!("WEEK_OF_MONTH must be 1..=5, got {}", week));
            }
            let dow = cols
                .day_of_week
                .ok_or_else(|| "MONTH_DAY_RULE_TYPE='NTH_WEEKDAY' requires DAY_OF_WEEK".to_string())?;
            let weekday = iso_to_weekday(dow)
                .ok_or_else(|| format!("invalid DAY_OF_WEEK (must be 1..=7, got {})", dow))?;
            Ok(MonthlyDayRule::NthWeekday { week, weekday })
        }
        other => Err(format!("invalid MONTH_DAY_RULE_TYPE: {}", other)),
    }
}

// ============================================================================
// RecurringService — DB アクセス層 (v2.1.0)
// ============================================================================

#[derive(Debug)]
pub enum RecurringError {
    Database(sqlx::Error),
    Validation(String),
}

impl std::fmt::Display for RecurringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecurringError::Database(e) => write!(f, "Database error: {}", e),
            RecurringError::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl From<sqlx::Error> for RecurringError {
    fn from(err: sqlx::Error) -> Self {
        RecurringError::Database(err)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SaveRecurringRuleRequest {
    pub rule_name: Option<String>,
    // Cycle definition (matches CycleColumns 1:1)
    pub period_unit: String,
    pub period_interval: u32,
    pub anchor_date: Option<String>,
    pub day_of_week: Option<u32>,
    pub month_day_rule_type: Option<String>,
    pub day_of_month: Option<u32>,
    pub week_of_month: Option<u32>,
    pub month_of_year: Option<u32>,
    pub holiday_shift_type: i32,
    // Period (YYYY-MM-DD)
    pub start_date: String,
    pub end_date: String,
    // HEADER template
    pub shop_id: Option<i64>,
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub total_amount: i64,
    pub tax_rounding_type: i64,
    pub tax_included_type: i64,
    pub header_memo: Option<String>,
    // DETAIL template (1:1)
    pub detail: SaveRecurringRuleDetailRequest,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SaveRecurringRuleDetailRequest {
    pub category1_code: String,
    pub category2_code: Option<String>,
    pub category3_code: Option<String>,
    pub item_name: String,
    pub amount: i64,
    pub tax_amount: i64,
    pub tax_rate: i32,
    pub amount_including_tax: Option<i64>,
    pub detail_memo: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateRecurringRuleResult {
    pub rule_id: i64,
    pub generated_count: usize,
    pub first_transaction_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct RecurringRuleSummary {
    pub rule_id: i64,
    pub rule_name: Option<String>,
    pub period_unit: String,
    pub period_interval: i64,
    pub start_date: String,
    pub end_date: String,
    pub total_amount: i64,
    pub holiday_shift_type: i32,
    pub occurrence_count: i64,
}

pub struct RecurringService {
    pool: SqlitePool,
}

impl RecurringService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Save a recurring rule and generate the matching IS_SCHEDULED=1 occurrences
    /// in a single transaction. Each generated TRANSACTIONS_HEADER row carries
    /// the new RULE_ID — that is the only marker tying occurrences back to the
    /// rule, so confirming or deleting individual rows leaves the rest untouched.
    /// Returns the new RULE_ID, the number of occurrences generated, and the
    /// first generated TRANSACTION_ID (if any) for callers that want to surface
    /// it in a result message.
    pub async fn create_rule_with_instances(
        &self,
        user_id: i64,
        request: SaveRecurringRuleRequest,
    ) -> Result<CreateRecurringRuleResult, RecurringError> {
        // ----- Parse + validate inputs -----
        let start = NaiveDate::parse_from_str(&request.start_date, "%Y-%m-%d")
            .map_err(|_| RecurringError::Validation(
                format!("Invalid start_date: {}", request.start_date)
            ))?;
        let end = NaiveDate::parse_from_str(&request.end_date, "%Y-%m-%d")
            .map_err(|_| RecurringError::Validation(
                format!("Invalid end_date: {}", request.end_date)
            ))?;
        if start > end {
            return Err(RecurringError::Validation(
                "start_date must be on or before end_date".to_string(),
            ));
        }
        if request.total_amount < 0 || request.total_amount > 999_999_999 {
            return Err(RecurringError::Validation(
                "TOTAL_AMOUNT must be between 0 and 999,999,999".to_string(),
            ));
        }
        if request.detail.item_name.trim().is_empty() {
            return Err(RecurringError::Validation(
                "DETAIL.item_name must not be empty".to_string(),
            ));
        }

        // Bounded-field length checks (Issue #37 Phase 2-3, character count).
        if let Some(rule_name) = &request.rule_name {
            if rule_name.chars().count() > consts::MAX_RULE_NAME_LEN {
                return Err(RecurringError::Validation(format!(
                    "Rule name must be {} characters or less",
                    consts::MAX_RULE_NAME_LEN
                )));
            }
        }
        if request.detail.item_name.chars().count() > consts::MAX_ITEM_NAME_LEN {
            return Err(RecurringError::Validation(format!(
                "Item name must be {} characters or less",
                consts::MAX_ITEM_NAME_LEN
            )));
        }
        if let Some(memo) = &request.header_memo {
            if memo.chars().count() > consts::MAX_MEMO_LEN {
                return Err(RecurringError::Validation(format!(
                    "Header memo must be {} characters or less",
                    consts::MAX_MEMO_LEN
                )));
            }
        }
        if let Some(memo) = &request.detail.detail_memo {
            if memo.chars().count() > consts::MAX_MEMO_LEN {
                return Err(RecurringError::Validation(format!(
                    "Detail memo must be {} characters or less",
                    consts::MAX_MEMO_LEN
                )));
            }
        }

        let anchor_date = match &request.anchor_date {
            Some(s) => Some(
                NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|_| {
                    RecurringError::Validation(format!("Invalid anchor_date: {}", s))
                })?,
            ),
            None => None,
        };
        let columns = CycleColumns {
            period_unit: request.period_unit.clone(),
            period_interval: request.period_interval,
            anchor_date,
            day_of_week: request.day_of_week,
            month_day_rule_type: request.month_day_rule_type.clone(),
            day_of_month: request.day_of_month,
            week_of_month: request.week_of_month,
            month_of_year: request.month_of_year,
            holiday_shift_type: request.holiday_shift_type,
        };
        let spec = columns_to_cyclic_spec(&columns).map_err(RecurringError::Validation)?;

        // ----- Fetch holidays (only when shift may apply) -----
        let holidays = if matches!(spec.holiday_shift, HolidayShift::None) {
            HashSet::new()
        } else {
            self.fetch_holidays_for(user_id, start, end).await?
        };

        let dates = generate_dates(&spec, start, end, &holidays);

        // ----- Persist rule + instances atomically -----
        let mut tx = self.pool.begin().await?;

        let header_memo_id = match &request.header_memo {
            Some(text) if !text.trim().is_empty() => {
                let r = sqlx::query(sql_queries::MEMO_INSERT)
                    .bind(user_id)
                    .bind(text)
                    .execute(&mut *tx)
                    .await?;
                Some(r.last_insert_rowid())
            }
            _ => None,
        };

        let detail_memo_id = match &request.detail.detail_memo {
            Some(text) if !text.trim().is_empty() => {
                let r = sqlx::query(sql_queries::MEMO_INSERT)
                    .bind(user_id)
                    .bind(text)
                    .execute(&mut *tx)
                    .await?;
                Some(r.last_insert_rowid())
            }
            _ => None,
        };

        let rule_result = sqlx::query(sql_queries::RECURRING_RULES_INSERT)
            .bind(user_id)
            .bind(&request.rule_name)
            .bind(&columns.period_unit)
            .bind(columns.period_interval as i64)
            .bind(columns.anchor_date.map(|d| d.format("%Y-%m-%d").to_string()))
            .bind(columns.day_of_week.map(|v| v as i64))
            .bind(&columns.month_day_rule_type)
            .bind(columns.day_of_month.map(|v| v as i64))
            .bind(columns.week_of_month.map(|v| v as i64))
            .bind(columns.month_of_year.map(|v| v as i64))
            .bind(columns.holiday_shift_type)
            .bind(start.format("%Y-%m-%d").to_string())
            .bind(end.format("%Y-%m-%d").to_string())
            .bind(request.shop_id)
            .bind(&request.category1_code)
            .bind(&request.from_account_code)
            .bind(&request.to_account_code)
            .bind(request.total_amount)
            .bind(request.tax_rounding_type)
            .bind(request.tax_included_type)
            .bind(header_memo_id)
            .execute(&mut *tx)
            .await?;
        let rule_id = rule_result.last_insert_rowid();

        sqlx::query(sql_queries::RECURRING_RULE_DETAILS_INSERT)
            .bind(rule_id)
            .bind(user_id)
            .bind(&request.detail.category1_code)
            .bind(&request.detail.category2_code)
            .bind(&request.detail.category3_code)
            .bind(&request.detail.item_name)
            .bind(request.detail.amount)
            .bind(request.detail.tax_amount)
            .bind(request.detail.tax_rate)
            .bind(request.detail.amount_including_tax)
            .bind(detail_memo_id)
            .execute(&mut *tx)
            .await?;

        let mut first_id: Option<i64> = None;

        for date in &dates {
            let datetime_str = format!("{} 00:00:00", date.format("%Y-%m-%d"));
            let header_result =
                sqlx::query(sql_queries::TRANSACTIONS_HEADER_INSERT_FOR_RECURRING)
                    .bind(user_id)
                    .bind(request.shop_id)
                    .bind(&datetime_str)
                    .bind(&request.category1_code)
                    .bind(&request.from_account_code)
                    .bind(&request.to_account_code)
                    .bind(request.total_amount)
                    .bind(request.tax_rounding_type)
                    .bind(request.tax_included_type)
                    .bind(header_memo_id)
                    .bind(rule_id)
                    .execute(&mut *tx)
                    .await?;
            let header_id = header_result.last_insert_rowid();
            if first_id.is_none() {
                first_id = Some(header_id);
            }

            sqlx::query(sql_queries::TRANSACTION_DETAIL_INSERT_FULL)
                .bind(header_id)
                .bind(user_id)
                .bind(&request.detail.category1_code)
                .bind(&request.detail.category2_code)
                .bind(&request.detail.category3_code)
                .bind(&request.detail.item_name)
                .bind(request.detail.amount)
                .bind(request.detail.tax_amount)
                .bind(request.detail.tax_rate)
                .bind(request.detail.amount_including_tax)
                .bind(detail_memo_id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(CreateRecurringRuleResult {
            rule_id,
            generated_count: dates.len(),
            first_transaction_id: first_id,
        })
    }

    /// List all active recurring rules for a user, with each rule's currently
    /// materialized occurrence count. Used by the rule list UI to show what
    /// the user has registered and how many TRANSACTIONS_HEADER rows each
    /// rule currently owns.
    pub async fn list_rules(
        &self,
        user_id: i64,
    ) -> Result<Vec<RecurringRuleSummary>, RecurringError> {
        use sqlx::Row;

        let rows = sqlx::query(sql_queries::RECURRING_RULES_LIST_BY_USER)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await?;

        let summaries = rows
            .into_iter()
            .map(|row| RecurringRuleSummary {
                rule_id: row.get::<i64, _>("RULE_ID"),
                rule_name: row.get::<Option<String>, _>("RULE_NAME"),
                period_unit: row.get::<String, _>("PERIOD_UNIT"),
                period_interval: row.get::<i64, _>("PERIOD_INTERVAL"),
                start_date: row.get::<String, _>("START_DATE"),
                end_date: row.get::<String, _>("END_DATE"),
                total_amount: row.get::<i64, _>("TOTAL_AMOUNT"),
                holiday_shift_type: row.get::<i32, _>("HOLIDAY_SHIFT_TYPE"),
                occurrence_count: row.get::<i64, _>("OCCURRENCE_COUNT"),
            })
            .collect();

        Ok(summaries)
    }

    /// Delete a recurring rule. The user picks one of two semantics in the UI:
    ///
    /// - `cascade = true`  → also drop every generated TRANSACTIONS_HEADER (and
    ///   their DETAILs via the existing FK) that points at this rule. Use when
    ///   the user is throwing the whole template away including its history.
    /// - `cascade = false` → keep the generated occurrences as standalone
    ///   scheduled transactions; only their `RULE_ID` is cleared so they no
    ///   longer reference the now-deleted rule.
    ///
    /// Either way the rule itself and its `RECURRING_RULE_DETAILS` row go away
    /// (the latter via `ON DELETE CASCADE`). All steps run inside one
    /// transaction so a partial failure leaves nothing dangling.
    pub async fn delete_rule(
        &self,
        user_id: i64,
        rule_id: i64,
        cascade: bool,
    ) -> Result<(), RecurringError> {
        let mut tx = self.pool.begin().await?;

        if cascade {
            sqlx::query(sql_queries::TRANSACTIONS_HEADER_DELETE_BY_RULE)
                .bind(rule_id)
                .bind(user_id)
                .execute(&mut *tx)
                .await?;
        } else {
            sqlx::query(sql_queries::TRANSACTIONS_HEADER_DETACH_FROM_RULE)
                .bind(rule_id)
                .bind(user_id)
                .execute(&mut *tx)
                .await?;
        }

        sqlx::query(sql_queries::RECURRING_RULES_DELETE)
            .bind(rule_id)
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    /// Fetch holidays applicable to this user within a window slightly wider than
    /// [start, end] — HolidayShift::Prev/Next can land outside the rule's period
    /// (e.g. Jan 1 holiday shifted back to Dec 31 of the previous year), so we
    /// pad ±14 days to cover any plausible chain of consecutive non-business days.
    async fn fetch_holidays_for(
        &self,
        user_id: i64,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<HashSet<NaiveDate>, RecurringError> {
        let locale: String = sqlx::query_scalar(
            "SELECT COALESCE(HOLIDAY_LOCALE, 'JP') FROM USERS WHERE USER_ID = ?",
        )
        .bind(user_id)
        .fetch_one(&self.pool)
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
        .fetch_all(&self.pool)
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
        .fetch_all(&self.pool)
        .await?;
        for d_str in custom_rows {
            if let Ok(d) = NaiveDate::parse_from_str(&d_str, "%Y-%m-%d") {
                holidays.insert(d);
            }
        }

        Ok(holidays)
    }
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

    fn spec_monthly_dom_with_shift(interval: u32, day: u32, shift: HolidayShift) -> CyclicSpec {
        CyclicSpec {
            cycle: Cycle::Monthly {
                interval,
                day_rule: MonthlyDayRule::DayOfMonth { day },
            },
            holiday_shift: shift,
        }
    }

    /// S❶ HolidayShift::None なら祝日が指定されていても無視（既存挙動の保証）。
    #[test]
    fn case_s1_shift_none_ignores_holidays() {
        let mut holidays = HashSet::new();
        holidays.insert(d(2026, 1, 15));
        let result = generate_dates(
            &spec_monthly_dom_with_shift(1, 15, HolidayShift::None),
            d(2026, 1, 1),
            d(2026, 1, 31),
            &holidays,
        );
        assert_eq!(result, vec![d(2026, 1, 15)]);
    }

    /// S❷ Prev：土曜の指定日 → 直前の金曜にシフト。
    #[test]
    fn case_s2_prev_saturday_to_friday() {
        // 2026-01-10 は土曜（2026-01-01 が木曜）
        let result = generate_dates(
            &spec_monthly_dom_with_shift(1, 10, HolidayShift::Prev),
            d(2026, 1, 1),
            d(2026, 1, 31),
            &no_holidays(),
        );
        assert_eq!(result, vec![d(2026, 1, 9)]);
    }

    /// S❸ Next：土曜の指定日 → 日曜をスキップして月曜へ（再帰ループ確認）。
    #[test]
    fn case_s3_next_saturday_to_monday_skips_sunday() {
        let result = generate_dates(
            &spec_monthly_dom_with_shift(1, 10, HolidayShift::Next),
            d(2026, 1, 1),
            d(2026, 1, 31),
            &no_holidays(),
        );
        assert_eq!(result, vec![d(2026, 1, 12)]);
    }

    /// S❹ Prev：祝日 + 土日の連続休日を超えて遡る（案A 確認）。
    #[test]
    fn case_s4_prev_chained_holidays() {
        // 2026-01-15(Thu) と 2026-01-14(Wed) を祝日にして、Prev で 1/13(Tue) まで遡らせる
        let mut holidays = HashSet::new();
        holidays.insert(d(2026, 1, 14));
        holidays.insert(d(2026, 1, 15));
        let result = generate_dates(
            &spec_monthly_dom_with_shift(1, 15, HolidayShift::Prev),
            d(2026, 1, 1),
            d(2026, 1, 31),
            &holidays,
        );
        assert_eq!(result, vec![d(2026, 1, 13)]);
    }

    /// S❺ Prev：1/1 が祝日 → 前年末にシフト（案I：範囲外でも採用）。
    #[test]
    fn case_s5_prev_falls_outside_range() {
        // 2026-01-01(Thu) を祝日として、Prev で 2025-12-31(Wed) にシフト
        let mut holidays = HashSet::new();
        holidays.insert(d(2026, 1, 1));
        let result = generate_dates(
            &spec_monthly_dom_with_shift(1, 1, HolidayShift::Prev),
            d(2026, 1, 1),
            d(2026, 1, 31),
            &holidays,
        );
        // 2/1 は無いことに注意（end=2026-01-31）。1/1 → 12/31 の 1 件だけ
        assert_eq!(result, vec![d(2025, 12, 31)]);
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

    // ========================================================================
    // CyclicSpec ↔ CycleColumns ラウンドトリップ
    // ========================================================================

    fn roundtrip(spec: CyclicSpec) {
        let cols = cyclic_spec_to_columns(&spec);
        let back = columns_to_cyclic_spec(&cols).expect("roundtrip should succeed");
        assert_eq!(back, spec, "roundtrip mismatch via columns: {:?}", cols);
    }

    #[test]
    fn roundtrip_daily() {
        roundtrip(CyclicSpec {
            cycle: Cycle::Daily { interval: 7, anchor: d(2026, 5, 1) },
            holiday_shift: HolidayShift::None,
        });
    }

    #[test]
    fn roundtrip_weekly_each_weekday() {
        for &wd in &[
            Weekday::Mon, Weekday::Tue, Weekday::Wed, Weekday::Thu,
            Weekday::Fri, Weekday::Sat, Weekday::Sun,
        ] {
            roundtrip(CyclicSpec {
                cycle: Cycle::Weekly { interval: 2, weekday: wd },
                holiday_shift: HolidayShift::Prev,
            });
        }
    }

    #[test]
    fn roundtrip_monthly_day_of_month() {
        roundtrip(CyclicSpec {
            cycle: Cycle::Monthly {
                interval: 1,
                day_rule: MonthlyDayRule::DayOfMonth { day: 25 },
            },
            holiday_shift: HolidayShift::Next,
        });
    }

    #[test]
    fn roundtrip_monthly_day_or_end() {
        roundtrip(CyclicSpec {
            cycle: Cycle::Monthly {
                interval: 1,
                day_rule: MonthlyDayRule::DayOfMonthOrEnd { day: 31 },
            },
            holiday_shift: HolidayShift::None,
        });
    }

    #[test]
    fn roundtrip_monthly_end_of_month() {
        roundtrip(CyclicSpec {
            cycle: Cycle::Monthly {
                interval: 2,
                day_rule: MonthlyDayRule::EndOfMonth,
            },
            holiday_shift: HolidayShift::None,
        });
    }

    #[test]
    fn roundtrip_monthly_nth_weekday() {
        roundtrip(CyclicSpec {
            cycle: Cycle::Monthly {
                interval: 1,
                day_rule: MonthlyDayRule::NthWeekday { week: 4, weekday: Weekday::Thu },
            },
            holiday_shift: HolidayShift::None,
        });
    }

    #[test]
    fn roundtrip_yearly_day_of_month() {
        roundtrip(CyclicSpec {
            cycle: Cycle::Yearly {
                interval: 1,
                month: 4,
                day_rule: MonthlyDayRule::DayOfMonth { day: 1 },
            },
            holiday_shift: HolidayShift::Next,
        });
    }

    #[test]
    fn roundtrip_yearly_nth_weekday() {
        roundtrip(CyclicSpec {
            cycle: Cycle::Yearly {
                interval: 1,
                month: 11,
                day_rule: MonthlyDayRule::NthWeekday { week: 5, weekday: Weekday::Fri },
            },
            holiday_shift: HolidayShift::Prev,
        });
    }

    // ========================================================================
    // 失敗ケース（不正な DB row → CyclicSpec の検証）
    // ========================================================================

    fn cols_minimal_daily() -> CycleColumns {
        cyclic_spec_to_columns(&CyclicSpec {
            cycle: Cycle::Daily { interval: 1, anchor: d(2026, 1, 1) },
            holiday_shift: HolidayShift::None,
        })
    }

    #[test]
    fn err_invalid_holiday_shift_type() {
        let mut cols = cols_minimal_daily();
        cols.holiday_shift_type = 99;
        assert!(columns_to_cyclic_spec(&cols).is_err());
    }

    #[test]
    fn err_daily_without_anchor() {
        let mut cols = cols_minimal_daily();
        cols.anchor_date = None;
        assert!(columns_to_cyclic_spec(&cols).is_err());
    }

    #[test]
    fn err_weekly_invalid_dow() {
        let mut cols = cyclic_spec_to_columns(&CyclicSpec {
            cycle: Cycle::Weekly { interval: 1, weekday: Weekday::Mon },
            holiday_shift: HolidayShift::None,
        });
        cols.day_of_week = Some(0);
        assert!(columns_to_cyclic_spec(&cols).is_err());
        cols.day_of_week = Some(8);
        assert!(columns_to_cyclic_spec(&cols).is_err());
    }

    #[test]
    fn err_monthly_missing_rule_type() {
        let mut cols = cyclic_spec_to_columns(&CyclicSpec {
            cycle: Cycle::Monthly { interval: 1, day_rule: MonthlyDayRule::DayOfMonth { day: 10 } },
            holiday_shift: HolidayShift::None,
        });
        cols.month_day_rule_type = None;
        assert!(columns_to_cyclic_spec(&cols).is_err());
    }

    #[test]
    fn err_yearly_invalid_month() {
        let mut cols = cyclic_spec_to_columns(&CyclicSpec {
            cycle: Cycle::Yearly {
                interval: 1, month: 6,
                day_rule: MonthlyDayRule::DayOfMonth { day: 1 },
            },
            holiday_shift: HolidayShift::None,
        });
        cols.month_of_year = Some(13);
        assert!(columns_to_cyclic_spec(&cols).is_err());
    }

    #[test]
    fn err_invalid_period_unit() {
        let mut cols = cols_minimal_daily();
        cols.period_unit = "WEIRD".to_string();
        assert!(columns_to_cyclic_spec(&cols).is_err());
    }

    #[test]
    fn iso_weekday_mapping() {
        // ISO 8601: Mon=1, Tue=2, ..., Sun=7
        assert_eq!(weekday_to_iso(Weekday::Mon), 1);
        assert_eq!(weekday_to_iso(Weekday::Sun), 7);
        assert_eq!(iso_to_weekday(1), Some(Weekday::Mon));
        assert_eq!(iso_to_weekday(7), Some(Weekday::Sun));
        assert_eq!(iso_to_weekday(0), None);
        assert_eq!(iso_to_weekday(8), None);
    }

    // Issue #37 Phase 2-3 — bounded-field length checks must count
    // characters (not bytes). The validation runs before any DB I/O, so
    // we exercise it against an empty in-memory pool.

    fn minimal_request() -> SaveRecurringRuleRequest {
        SaveRecurringRuleRequest {
            rule_name: None,
            period_unit: "DAY".to_string(),
            period_interval: 1,
            anchor_date: Some("2026-01-01".to_string()),
            day_of_week: None,
            month_day_rule_type: None,
            day_of_month: None,
            week_of_month: None,
            month_of_year: None,
            holiday_shift_type: 0,
            start_date: "2026-01-01".to_string(),
            end_date: "2026-01-01".to_string(),
            shop_id: None,
            category1_code: "EXPENSE".to_string(),
            from_account_code: "BANK".to_string(),
            to_account_code: "OUT".to_string(),
            total_amount: 100,
            tax_rounding_type: 0,
            tax_included_type: 1,
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
                detail_memo: None,
            },
        }
    }

    async fn empty_pool() -> SqlitePool {
        SqlitePool::connect(":memory:").await.unwrap()
    }

    #[tokio::test]
    async fn test_create_rule_rejects_over_max_chars_of_multibyte_rule_name() {
        let service = RecurringService::new(empty_pool().await);

        let mut request = minimal_request();
        request.rule_name = Some("あ".repeat(consts::MAX_RULE_NAME_LEN + 1));

        let err = service.create_rule_with_instances(2, request).await.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_RULE_NAME_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }

    #[tokio::test]
    async fn test_create_rule_rejects_over_max_chars_of_multibyte_item_name() {
        let service = RecurringService::new(empty_pool().await);

        let mut request = minimal_request();
        request.detail.item_name = "あ".repeat(consts::MAX_ITEM_NAME_LEN + 1);

        let err = service.create_rule_with_instances(2, request).await.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_ITEM_NAME_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }

    #[tokio::test]
    async fn test_create_rule_rejects_over_max_chars_of_multibyte_header_memo() {
        let service = RecurringService::new(empty_pool().await);

        let mut request = minimal_request();
        request.header_memo = Some("メ".repeat(consts::MAX_MEMO_LEN + 1));

        let err = service.create_rule_with_instances(2, request).await.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }

    #[tokio::test]
    async fn test_create_rule_rejects_over_max_chars_of_multibyte_detail_memo() {
        let service = RecurringService::new(empty_pool().await);

        let mut request = minimal_request();
        request.detail.detail_memo = Some("メ".repeat(consts::MAX_MEMO_LEN + 1));

        let err = service.create_rule_with_instances(2, request).await.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains(&consts::MAX_MEMO_LEN.to_string()),
            "error should reference the limit: {}", msg);
    }
}
