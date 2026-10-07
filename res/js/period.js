import { invoke } from '@tauri-apps/api/core';
import { formatLocalDate } from './format-local-date.js';

let cachedSettings = null;

export async function getPeriodSettings() {
    if (cachedSettings) return cachedSettings;
    try {
        const s = await invoke('get_user_period_settings');
        cachedSettings = {
            monthStartDay: Number(s.month_period_start_day) || 1,
            yearStartMonth: Number(s.year_period_start_month) || 1,
            yearStartDay: Number(s.year_period_start_day) || 1,
            monthHolidayShift: Number(s.month_period_holiday_shift) || 0,
        };
        return cachedSettings;
    } catch (e) {
        console.warn('Failed to load period settings, using defaults:', e);
        cachedSettings = { monthStartDay: 1, yearStartMonth: 1, yearStartDay: 1, monthHolidayShift: 0 };
        return cachedSettings;
    }
}

/// v2.4.0: 月次サイクル境界を backend から取得する（休日シフト適用済み）。
/// ローカルの monthlyPeriodBounds はカレンダー基準のみ。
export async function fetchMonthlyPeriodBounds(year, month) {
    const b = await invoke('get_monthly_period_bounds', { year, month });
    return {
        start: new Date(b.start + 'T00:00:00'),
        end: new Date(b.end + 'T00:00:00'),
    };
}

/// (year, month) of the user's monthly period that contains `date`. A period
/// is named by its start month, so with a custom start day or a holiday
/// shift the calendar month can name a period that starts after `date`
/// (start day 25, 09-10 -> the "September" period is 09-25..10-24) or ended
/// before it (start day 1 shifted back to 11-30 -> on 11-30 the period is
/// "December"). It can even be two months away: start day 31 with the next
/// business day, 2026-01-31 and 02-28 are Saturdays, so "January" runs
/// 02-02..03-01 and 03-01 belongs to it. So step one month at a time until
/// the bounds contain `date` (periods are contiguous, so this converges);
/// MAX_PERIOD_STEPS only guards against a backend that never answers with
/// a matching period (latent-scan2 A3). Falls back to the calendar month
/// when the backend cannot answer.
const MAX_PERIOD_STEPS = 6;

export async function findMonthlyPeriodContaining(date) {
    const day = formatLocalDate(date);
    const calendar = { year: date.getFullYear(), month: date.getMonth() + 1 };
    let { year, month } = calendar;
    try {
        for (let step = 0; step < MAX_PERIOD_STEPS; step++) {
            const b = await invoke('get_monthly_period_bounds', { year, month });
            if (day < b.start) {
                month -= 1;
                if (month === 0) {
                    month = 12;
                    year -= 1;
                }
            } else if (day > b.end) {
                month += 1;
                if (month === 13) {
                    month = 1;
                    year += 1;
                }
            } else {
                return { year, month };
            }
        }
        console.warn('No monthly period contains', day, '- using the calendar month');
    } catch (e) {
        console.warn('Failed to load period bounds, using the calendar month:', e);
    }
    return calendar;
}

/// Last day (YYYY-MM-DD) of the user's monthly period for (year, month),
/// following the custom start day and holiday shift (latent-audit L14). Falls
/// back to the calendar month end when the backend cannot answer.
export async function fetchMonthlyPeriodEndDate(year, month) {
    try {
        const b = await invoke('get_monthly_period_bounds', { year, month });
        return b.end;
    } catch (e) {
        console.warn('Failed to load period bounds, using the calendar month end:', e);
        // day 0 of the next month is the last day of this one (month is 1-based)
        const lastDay = new Date(year, month, 0).getDate();
        return `${year}-${String(month).padStart(2, '0')}-${String(lastDay).padStart(2, '0')}`;
    }
}

export function invalidatePeriodSettingsCache() {
    cachedSettings = null;
}

export function resolveDayOrEnd(year, month, day) {
    const endOfMonthDay = new Date(year, month, 0).getDate();
    return Math.min(day, endOfMonthDay);
}

export function monthlyPeriodBounds(year, month, startDay) {
    const startDate = new Date(year, month - 1, resolveDayOrEnd(year, month, startDay));
    let nextMonth = month + 1;
    let nextYear = year;
    if (nextMonth > 12) {
        nextMonth = 1;
        nextYear += 1;
    }
    const nextPeriodStart = new Date(nextYear, nextMonth - 1, resolveDayOrEnd(nextYear, nextMonth, startDay));
    const endDate = new Date(nextPeriodStart.getTime() - 24 * 60 * 60 * 1000);
    return { start: startDate, end: endDate };
}

export function yearlyPeriodBounds(year, startMonth, startDay) {
    const startDate = new Date(year, startMonth - 1, resolveDayOrEnd(year, startMonth, startDay));
    const nextPeriodStart = new Date(year + 1, startMonth - 1, resolveDayOrEnd(year + 1, startMonth, startDay));
    const endDate = new Date(nextPeriodStart.getTime() - 24 * 60 * 60 * 1000);
    return { start: startDate, end: endDate };
}

/// Year of the user's yearly period that contains `date`. A yearly period is
/// named by its start year, so with a start other than 01-01 the calendar
/// year can name a period that starts after `date` (start 04-01, 2026-02-10
/// -> the "2026" period is 2026-04-01..2027-03-31; the date is in "2025").
/// Yearly periods have no holiday shift today, so the answer is the calendar
/// year or the one before; the loop does not rely on that, so a future shift
/// keeps working (latent-scan2 A3). MAX_PERIOD_STEPS only guards the loop.
export function findYearlyPeriodContaining(date, startMonth, startDay) {
    const day = new Date(date.getFullYear(), date.getMonth(), date.getDate());
    let year = date.getFullYear();
    for (let step = 0; step < MAX_PERIOD_STEPS; step++) {
        const { start, end } = yearlyPeriodBounds(year, startMonth, startDay);
        if (day < start) {
            year -= 1;
        } else if (day > end) {
            year += 1;
        } else {
            return year;
        }
    }
    return date.getFullYear();
}

function fmtMonthDay(date, lang) {
    const m = date.getMonth() + 1;
    const d = date.getDate();
    return lang === 'en' ? `${m}/${d}` : `${m}/${d}`;
}

/// 月期の「ベースラベル」だけを返す（括弧の境界日表示なし）。
/// 月別推移など範囲表記の構成要素として使う。同期関数。
export function formatMonthlyPeriodBaseLabel(year, month, lang) {
    return lang === 'ja' ? `${year}年${month}月` : `${monthName(month, lang)} ${year}`;
}

export async function formatMonthlyPeriodLabel(year, month, startDay, lang) {
    const yearSuffix = lang === 'ja' ? '年' : '';
    const monthSuffix = lang === 'ja' ? '月' : '';
    const baseLabel = lang === 'ja'
        ? `${year}${yearSuffix}${month}${monthSuffix}`
        : `${monthName(month, lang)} ${year}`;

    const settings = await getPeriodSettings();
    // shift None かつ起算日が 1 日 → 境界 = 当月そのもの、括弧表示不要
    if (settings.monthHolidayShift === 0 && startDay === 1) return baseLabel;

    let start, end;
    try {
        ({ start, end } = await fetchMonthlyPeriodBounds(year, month));
    } catch (e) {
        console.warn('Failed to fetch shift-aware bounds, falling back to calendar:', e);
        ({ start, end } = monthlyPeriodBounds(year, month, startDay));
    }
    const rangeOpen = lang === 'ja' ? '（' : ' (';
    const rangeClose = lang === 'ja' ? '）' : ')';
    const separator = lang === 'ja' ? '〜' : ' – ';
    return `${baseLabel}${rangeOpen}${fmtMonthDay(start, lang)}${separator}${fmtMonthDay(end, lang)}${rangeClose}`;
}

export function formatYearlyPeriodLabel(year, startMonth, startDay, lang) {
    const baseLabel = lang === 'ja' ? `${year}年` : `${year}`;
    if (startMonth === 1 && startDay === 1) return baseLabel;
    const { start, end } = yearlyPeriodBounds(year, startMonth, startDay);
    const rangeOpen = lang === 'ja' ? '（' : ' (';
    const rangeClose = lang === 'ja' ? '）' : ')';
    const separator = lang === 'ja' ? '〜' : ' – ';
    const fmtDate = (d) => `${d.getFullYear()}/${d.getMonth() + 1}/${d.getDate()}`;
    return `${baseLabel}${rangeOpen}${fmtDate(start)}${separator}${fmtDate(end)}${rangeClose}`;
}

function monthName(month, lang) {
    const names = {
        en: ['January', 'February', 'March', 'April', 'May', 'June',
             'July', 'August', 'September', 'October', 'November', 'December'],
    };
    const list = names[lang] || names.en;
    return list[month - 1] || String(month);
}
