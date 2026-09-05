/**
 * Format a Date as `YYYY-MM-DD` in the browser's local timezone
 * (Fable-5 #13).
 *
 * The tempting one-liner `new Date().toISOString().slice(0, 10)`
 * uses UTC, so a JST user who opens a form before 09:00 JST sees
 * yesterday's date in every "today" default (`new Date()` in JST
 * 06:30 is 21:30 UTC on the previous day → `toISOString()` starts
 * with the previous day's string). That surfaced in the recurring-
 * rule modal, where start-date / end-date / anchor-date were all
 * one day behind, and a Daily-interval-1 rule wrote a spurious
 * occurrence on the wrong day.
 *
 * `transaction-management.js` had already fixed this for its
 * datetime-local field with the same local-getter pattern; this
 * module is the shared version so the fix stays consistent across
 * every future caller.
 *
 * @param {Date} date - the Date to format.
 * @returns {string} `YYYY-MM-DD` in the browser's local timezone.
 */
export function formatLocalDate(date) {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
}
