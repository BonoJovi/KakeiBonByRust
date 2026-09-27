//! Latent-audit 2026-09 regression tests for `services::period` and its
//! monthly entry point `crate::monthly_bounds_with_shift_for` (lib.rs).
//!
//! TDD red phase: every test asserts the CORRECT behaviour and is expected to
//! FAIL against the current code. All are `#[ignore]`d; run them with
//! `cargo test --lib latent_ -- --ignored`.

use super::*;
use crate::services::holiday::HolidayShift;

/// Years chrono's `NaiveDate` cannot represent, plus the last representable
/// year (`NaiveDate::MAX.year()`), where rolling December into "next month"
/// overflows.
fn out_of_range_years() -> [i32; 3] {
    use chrono::Datelike;
    [300_000, -300_000, NaiveDate::MAX.year()]
}

/// Run `monthly_bounds_with_shift_for` on its own task so a panic surfaces as
/// a `JoinError` instead of aborting the test body.
async fn call_monthly_bounds(
    year: i32,
    month: u32,
    shift: HolidayShift,
) -> Result<Result<(NaiveDate, NaiveDate), String>, tokio::task::JoinError> {
    let pool = sqlx::SqlitePool::connect(":memory:").await.unwrap();
    tokio::spawn(async move {
        crate::monthly_bounds_with_shift_for(&pool, 1, year, month, 1, shift).await
    })
    .await
}

/// L10: `monthly_bounds_with_shift_for` validates `month` but not `year`; on
/// the `HolidayShift::None` fast path an out-of-range year reaches
/// `period::end_of_month`'s `.expect` and panics the backend thread
/// (reachable via direct invoke of the monthly period/aggregation commands).
/// Expected: returns `Err` for out-of-range years instead of panicking.
#[tokio::test]
async fn latent_l10_monthly_bounds_rejects_out_of_range_year_without_shift() {
    for year in out_of_range_years() {
        let outcome = call_monthly_bounds(year, 12, HolidayShift::None).await;
        match outcome {
            Ok(Err(_)) => {}
            Ok(Ok(bounds)) => panic!("year={} must be rejected, got {:?}", year, bounds),
            Err(e) => panic!("year={} panicked instead of returning Err: {}", year, e),
        }
    }
}

/// L10: on the holiday-shift path `NaiveDate::from_ymd_opt` catches years
/// beyond chrono's range, but the last representable year with month 12
/// still reaches `end_of_month(year + 1, 1)` and panics.
/// Expected: returns `Err` instead of panicking.
#[tokio::test]
async fn latent_l10_monthly_bounds_rejects_out_of_range_year_with_shift() {
    for year in out_of_range_years() {
        let outcome = call_monthly_bounds(year, 12, HolidayShift::Next).await;
        match outcome {
            Ok(Err(_)) => {}
            Ok(Ok(bounds)) => panic!("year={} must be rejected, got {:?}", year, bounds),
            Err(e) => panic!("year={} panicked instead of returning Err: {}", year, e),
        }
    }
}

/// L10: the public helpers in `services::period` panicked via `.expect` on
/// out-of-range years. They now return `None` there (callers turn that into
/// an "invalid year" error).
#[test]
fn latent_l10_period_helpers_return_none_on_out_of_range_year() {
    let holidays: HashSet<NaiveDate> = HashSet::new();
    let year = 300_000;
    let last = {
        use chrono::Datelike;
        NaiveDate::MAX.year()
    };
    assert_eq!(end_of_month(year, 1), None);
    assert_eq!(resolve_day_or_end(year, 1, 31), None);
    assert_eq!(monthly_period_bounds(last, 12, 1), None);
    assert_eq!(monthly_period_bounds_with_shift(last, 12, 1, HolidayShift::Next, &holidays), None);
    assert_eq!(yearly_period_bounds(last, 1, 1), None);
    // In range they still resolve.
    assert!(monthly_period_bounds(2026, 12, 1).is_some());
}
