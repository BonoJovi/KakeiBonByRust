use chrono::{Days, Months, NaiveDate};

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
}
