import { invoke } from '@tauri-apps/api/core';

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
