import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';
import { fitWindowToScreen } from './window-fit.js';
import { setupIndicators } from './indicators.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar, setupLanguageMenu, setupLanguageMenuHandlers } from './menu.js';
import { setupTaxCalculationListeners } from './detail-tax-calc.js';
import { showValidationError, clearValidationError, showMaxLengthError, attachCharCounter } from './validation-display.js';
import {
    MAX_RULE_NAME_LEN, MAX_ITEM_NAME_LEN, MAX_MEMO_LEN,
    HOLIDAY_SEED_YEARS_BACK, HOLIDAY_SEED_YEARS_AHEAD,
} from './consts.js';
import { formatApiError, API_ERROR_CODES } from './master-crud.js';
import { singleFlight } from './single-flight.js';
import { parseAmountStrict } from './parse-amount-strict.js';
import { formatLocalDate } from './format-local-date.js';
import { calculateRecommendedTotal } from './tax-calc.js';

console.log('=== RECURRING-RULE.JS LOADED ===');

let currentUserId = null;
let categoryTree = [];
let pendingDeleteRule = null; // { rule_id, occurrence_count, name } awaiting modal choice

document.addEventListener('DOMContentLoaded', async () => {
    createMenuBar('management');

    try {
        if (!await isSessionAuthenticated()) {
            window.location.href = HTML_FILES.INDEX;
            return;
        }
        const user = await getCurrentSessionUser();
        if (!user) {
            window.location.href = HTML_FILES.INDEX;
            return;
        }
        currentUserId = user.user_id;

        await i18n.init();
        i18n.updateUI();

        await setupLanguageMenu();
        setupLanguageMenuHandlers();
        setupFontSizeMenuHandlers();
        await setupFontSizeMenu();
        setupFontSizeModalHandlers();
        await applyFontSize();
        setupIndicators();

        await loadCategoryTree();
        await loadAccounts();
        await loadShops();

        setupCycleKindToggle();
        setupAnchorFollowsStart();
        setupCategoryChainHandlers();
        setupDetailTaxCalculation();
        setupDerivedTotal();
        setupBoundedFieldCounters();
        setupFormSubmit();
        setupResetButton();
        setupDeleteModal();
        await loadRules();

        // Default start_date to today, end_date to one year out.
        // Fable-5 review #13 — the pre-fix defaults used
        // `new Date().toISOString().slice(0, 10)` which yields the UTC
        // date. A JST user opening the modal before 09:00 JST saw
        // yesterday in all three fields (Daily-interval-1 then wrote
        // a spurious occurrence, Monthly-day-of-month could skip a
        // cycle). `formatLocalDate` uses the browser's local getters
        // so the default matches the user's wall clock.
        const today = new Date();
        const oneYearLater = new Date(today.getFullYear() + 1, today.getMonth(), today.getDate());
        document.getElementById('start-date').value = formatLocalDate(today);
        document.getElementById('end-date').value = formatLocalDate(oneYearLater);
        // A rule may only span the years with holiday data (latent-audit
        // M15 / M18); the date pickers stop at the bounds the backend enforces.
        applyPeriodLimits(await loadPeriodLimits());
        document.getElementById('anchor-date').value = document.getElementById('start-date').value;

        await fitWindowToScreen();
        // Form is taller than the window; ensure the user starts at the top
        window.scrollTo(0, 0);
    } catch (err) {
        console.error('Initialization error:', err);
        // formatApiError unwraps the { code, message, entity } object
        // shape that get_accounts / get_shops (migrated to ApiError in
        // PR #97/#99) now return; a bare String(err) would render
        // "[object Object]" here (Devin review on #99).
        showResult('error', formatApiError(err));
    }
});

// ----- Data loading -----

async function loadCategoryTree() {
    categoryTree = await invoke('get_category_tree_with_lang', {
        langCode: i18n.currentLanguage,
    });
    const cat1Select = document.getElementById('category1');
    cat1Select.innerHTML = '';
    const placeholder = document.createElement('option');
    placeholder.value = '';
    placeholder.textContent = i18n.t('common.unspecified') || '-- select --';
    cat1Select.appendChild(placeholder);
    categoryTree.forEach((cat1) => {
        const opt = document.createElement('option');
        opt.value = cat1.category1.category1_code;
        opt.textContent = cat1.category1.category1_name_i18n;
        cat1Select.appendChild(opt);
    });
}

async function loadAccounts() {
    const accounts = await invoke('get_accounts', { includeDisabled: false });
    const fromSel = document.getElementById('from-account');
    const toSel = document.getElementById('to-account');
    fromSel.innerHTML = '';
    toSel.innerHTML = '';

    // Fable-5 review #22 — the NONE account's persisted name is
    // Japanese ("指定なし") from `initialize_none_account`. Mirror the
    // pattern already used in `transaction-management.js::loadAccounts
    // ForModal`: add the NONE option first with the localised
    // `common.unspecified` label, then skip account_code === 'NONE'
    // when appending the rest so it isn't duplicated.
    const unspecifiedText = i18n.t('common.unspecified');
    const fromNoneOpt = document.createElement('option');
    fromNoneOpt.value = 'NONE';
    fromNoneOpt.textContent = unspecifiedText;
    fromSel.appendChild(fromNoneOpt);
    const toNoneOpt = document.createElement('option');
    toNoneOpt.value = 'NONE';
    toNoneOpt.textContent = unspecifiedText;
    toSel.appendChild(toNoneOpt);

    accounts.forEach((acc) => {
        if (acc.account_code === 'NONE') return;
        const fromOpt = document.createElement('option');
        fromOpt.value = acc.account_code;
        fromOpt.textContent = acc.account_name;
        fromSel.appendChild(fromOpt);

        const toOpt = document.createElement('option');
        toOpt.value = acc.account_code;
        toOpt.textContent = acc.account_name;
        toSel.appendChild(toOpt);
    });
}

async function loadShops() {
    const shops = await invoke('get_shops', { includeDisabled: false });
    const shopSel = document.getElementById('shop');
    shopSel.innerHTML = '';
    const noneOpt = document.createElement('option');
    noneOpt.value = '';
    noneOpt.textContent = i18n.t('common.unspecified') || '(none)';
    shopSel.appendChild(noneOpt);
    shops.forEach((s) => {
        if (s.is_disabled) return;
        const opt = document.createElement('option');
        opt.value = s.shop_id;
        opt.textContent = s.shop_name;
        shopSel.appendChild(opt);
    });
}

// ----- Daily anchor follows the start date -----

// The anchor (起点日) of a daily rule starts out as the start date and keeps
// following it until the user edits the anchor. It used to default to today
// on its own, so moving the start date back silently skipped the days before
// today, and a past period produced no occurrences (latent-scan2 R2).
let anchorEditedByUser = false;

function setupAnchorFollowsStart() {
    const start = document.getElementById('start-date');
    const anchor = document.getElementById('anchor-date');
    if (!start || !anchor) return;
    const follow = () => {
        if (!anchorEditedByUser) anchor.value = start.value;
    };
    start.addEventListener('input', follow);
    start.addEventListener('change', follow);
    anchor.addEventListener('input', () => { anchorEditedByUser = true; });
}

// ----- Cycle kind: show/hide anchor vs day-of-month -----

function setupCycleKindToggle() {
    const cycleRadios = document.querySelectorAll('input[name="cycle-kind"]');
    const monthlyModeRadios = document.querySelectorAll('input[name="monthly-mode"]');
    cycleRadios.forEach((r) => r.addEventListener('change', updateCycleVisibility));
    monthlyModeRadios.forEach((r) => r.addEventListener('change', updateCycleVisibility));
    updateCycleVisibility();
}

function updateCycleVisibility() {
    const kind = document.querySelector('input[name="cycle-kind"]:checked').value;
    const monthlyMode =
        document.querySelector('input[name="monthly-mode"]:checked')?.value || 'DAY';
    const isDay = kind === 'DAY';
    const isMonth = kind === 'MONTH';
    const isMonthDay = isMonth && monthlyMode === 'DAY';
    const isMonthNth = isMonth && monthlyMode === 'NTH_WEEKDAY';

    document.getElementById('anchor-date-group').classList.toggle('visible', isDay);
    // Latent-audit L13 — a daily rule has no holiday shift: moving a daily
    // occurrence off a holiday lands it on a day that already has one.
    const holidayShift = document.getElementById('holiday-shift-type');
    if (holidayShift) {
        if (isDay) holidayShift.value = '0';
        holidayShift.disabled = isDay;
    }
    document.getElementById('monthly-mode-group').classList.toggle('visible', isMonth);
    document.getElementById('day-of-month-group').classList.toggle('visible', isMonthDay);
    document.getElementById('week-of-month-group').classList.toggle('visible', isMonthNth);
    document.getElementById('day-of-week-group').classList.toggle('visible', isMonthNth);
}

// ----- Category 1 → 2 → 3 dependent dropdowns -----

function setupCategoryChainHandlers() {
    const cat1 = document.getElementById('category1');
    const cat2 = document.getElementById('category2');
    const cat3 = document.getElementById('category3');

    cat1.addEventListener('change', () => {
        cat2.innerHTML = '';
        cat3.innerHTML = '';
        const placeholder = document.createElement('option');
        placeholder.value = '';
        placeholder.textContent = i18n.t('common.unspecified') || '(none)';
        cat2.appendChild(placeholder);

        const cat3Placeholder = document.createElement('option');
        cat3Placeholder.value = '';
        cat3Placeholder.textContent = i18n.t('common.unspecified') || '(none)';
        cat3.appendChild(cat3Placeholder);

        const selected = cat1.value;
        if (!selected) return;
        const node = categoryTree.find((c) => c.category1.category1_code === selected);
        if (!node || !node.children) return;
        node.children.forEach((c2) => {
            const opt = document.createElement('option');
            opt.value = c2.category2.category2_code;
            opt.textContent = c2.category2.category2_name_i18n;
            cat2.appendChild(opt);
        });
    });

    cat2.addEventListener('change', () => {
        cat3.innerHTML = '';
        const placeholder = document.createElement('option');
        placeholder.value = '';
        placeholder.textContent = i18n.t('common.unspecified') || '(none)';
        cat3.appendChild(placeholder);

        const cat1Code = cat1.value;
        const cat2Code = cat2.value;
        if (!cat1Code || !cat2Code) return;
        const cat1Node = categoryTree.find((c) => c.category1.category1_code === cat1Code);
        if (!cat1Node || !cat1Node.children) return;
        const cat2Node = cat1Node.children.find(
            (c) => c.category2.category2_code === cat2Code
        );
        if (!cat2Node || !cat2Node.children) return;
        cat2Node.children.forEach((c3) => {
            const opt = document.createElement('option');
            opt.value = c3.category3_code;
            opt.textContent = c3.category3_name_i18n;
            cat3.appendChild(opt);
        });
    });
}

// ----- Detail tax auto-calculation (shared module) -----

function setupDetailTaxCalculation() {
    const taxRate = document.getElementById('tax-rate');
    const amountExcludingTax = document.getElementById('amount-excluding-tax');
    const amountIncludingTax = document.getElementById('amount-including-tax');
    const taxAmount = document.getElementById('tax-amount');
    if (!taxRate || !amountExcludingTax || !amountIncludingTax || !taxAmount) {
        return;
    }
    setupTaxCalculationListeners(
        { taxRate, amountExcludingTax, amountIncludingTax, taxAmount },
        {
            getRoundingType: () =>
                parseInt(document.getElementById('tax-rounding-type').value, 10) || 0,
        }
    );
}

// ----- Total derived from the detail (latent-audit M17) -----

// The rule has exactly one detail, so its total is that detail's price under
// the header's rounding / tax-included settings — the same value the Rust
// side stores. The field is read-only; it used to be typed separately,
// defaulted to 0, and was never checked against the detail.
function updateDerivedTotal() {
    const amount = parseAmountStrict(document.getElementById('amount-excluding-tax').value);
    const taxRate = parseAmountStrict(document.getElementById('tax-rate').value);
    const includingRaw = document.getElementById('amount-including-tax').value.trim();
    const including = includingRaw === '' ? null : parseAmountStrict(includingRaw);
    const totalInput = document.getElementById('total-amount');
    if (amount === null || taxRate === null || (includingRaw !== '' && including === null)) {
        totalInput.value = '';
        return;
    }
    totalInput.value = calculateRecommendedTotal(
        [{ amount, amount_including_tax: including, tax_rate: taxRate }],
        parseInt(document.getElementById('tax-rounding-type').value, 10) || 0,
        parseInt(document.getElementById('tax-included-type').value, 10),
    );
}

function setupDerivedTotal() {
    // Registered after setupDetailTaxCalculation(), so the tax fields it
    // fills in are already updated when these handlers run.
    for (const id of ['amount-excluding-tax', 'amount-including-tax', 'tax-amount', 'tax-rate']) {
        document.getElementById(id)?.addEventListener('input', updateDerivedTotal);
    }
    for (const id of ['tax-rate', 'tax-rounding-type', 'tax-included-type']) {
        document.getElementById(id)?.addEventListener('change', updateDerivedTotal);
    }
    updateDerivedTotal();
}

// ----- Form submit -----

function setupFormSubmit() {
    // singleFlight: a double click / repeated Enter while the rule is being
    // created must not create it (and its occurrences) twice (latent-audit M19).
    document.getElementById('recurring-rule-form').addEventListener('submit', singleFlight(async (e) => {
        e.preventDefault();
        hideResult();

        const cycleKind = document.querySelector('input[name="cycle-kind"]:checked').value;
        const monthlyMode =
            document.querySelector('input[name="monthly-mode"]:checked')?.value || 'DAY';
        const periodInterval = parseInt(document.getElementById('period-interval').value, 10);

        let monthDayRuleType = null;
        let dayOfMonth = null;
        let weekOfMonth = null;
        let dayOfWeek = null;
        if (cycleKind === 'MONTH') {
            if (monthlyMode === 'DAY') {
                // Latent-audit M14 — a day the month does not have (29–31)
                // falls on the month's last day instead of being skipped.
                monthDayRuleType = 'DAY_OR_END';
                dayOfMonth = parseInt(document.getElementById('day-of-month').value, 10);
            } else if (monthlyMode === 'END') {
                monthDayRuleType = 'END';
            } else if (monthlyMode === 'NTH_WEEKDAY') {
                monthDayRuleType = 'NTH_WEEKDAY';
                weekOfMonth = parseInt(document.getElementById('week-of-month').value, 10);
                dayOfWeek = parseInt(document.getElementById('day-of-week').value, 10);
            }
        }

        // Fable-5 review #10 — money fields on this form used to be
        // read with `parseInt(el.value) || 0` (or `intOrNull`), which
        // silently truncated decimals and locale-comma inputs. The
        // money / rate fields are now parsed with the strict
        // helper up front; a rejection surfaces a field-level error
        // and aborts before we build the request.
        const amountExcludingTaxInput = document.getElementById('amount-excluding-tax');
        const taxAmountInput = document.getElementById('tax-amount');
        const taxRateInput = document.getElementById('tax-rate');
        const amountIncludingTaxInput = document.getElementById('amount-including-tax');
        clearValidationError(amountExcludingTaxInput);
        clearValidationError(taxAmountInput);
        clearValidationError(taxRateInput);
        clearValidationError(amountIncludingTaxInput);

        const amountExcludingTax = parseAmountStrict(amountExcludingTaxInput.value);
        const taxAmount = parseAmountStrict(taxAmountInput.value);
        const taxRate = parseAmountStrict(taxRateInput.value);
        // amount_including_tax stays optional (null when empty) — the
        // strict helper returns 0 for empty, so keep the explicit
        // empty check that `intOrNull` used to provide.
        const amountIncludingTaxRaw = amountIncludingTaxInput.value.trim();
        const amountIncludingTax = amountIncludingTaxRaw === ''
            ? null
            : parseAmountStrict(amountIncludingTaxRaw);

        if (amountExcludingTax === null) {
            showValidationError(amountExcludingTaxInput, i18n.t('common.error_amount_not_integer'));
            return;
        }
        if (taxAmount === null) {
            showValidationError(taxAmountInput, i18n.t('common.error_amount_not_integer'));
            return;
        }
        if (taxRate === null) {
            showValidationError(taxRateInput, i18n.t('common.error_amount_not_integer'));
            return;
        }
        // `amountIncludingTax === null` from a non-empty raw input
        // means strict rejection; from an empty raw input it just
        // means "user didn't fill it in" (legal — server accepts null).
        if (amountIncludingTax === null && amountIncludingTaxRaw !== '') {
            showValidationError(amountIncludingTaxInput, i18n.t('common.error_amount_not_integer'));
            return;
        }

        // Latent-audit M16 — a TRANSFER from an account to itself is
        // rejected by the transaction screen and by the Rust side; catch it
        // here with the same message before any occurrence is generated.
        if (document.getElementById('category1').value === 'TRANSFER'
            && document.getElementById('from-account').value
                === document.getElementById('to-account').value) {
            showResult('error', i18n.t('transaction_mgmt.transfer_same_account'));
            return;
        }

        // Latent-audit M15 / M18 — the period must stay within the years
        // with holiday data; the Rust side enforces the same bounds.
        const limits = await loadPeriodLimits();
        applyPeriodLimits(limits);
        const startDate = document.getElementById('start-date').value;
        const endDate = document.getElementById('end-date').value;
        if (startDate < limits.min || endDate > limits.max) {
            showResult('error', periodOutOfRangeMessage(limits.min, limits.max));
            return;
        }

        const request = {
            rule_name: stringOrNull(document.getElementById('rule-name').value),
            period_unit: cycleKind,
            period_interval: periodInterval,
            anchor_date: cycleKind === 'DAY' ? document.getElementById('anchor-date').value : null,
            day_of_week: dayOfWeek,
            month_day_rule_type: monthDayRuleType,
            day_of_month: dayOfMonth,
            week_of_month: weekOfMonth,
            month_of_year: null,
            holiday_shift_type: parseInt(document.getElementById('holiday-shift-type').value, 10),

            start_date: document.getElementById('start-date').value,
            end_date: document.getElementById('end-date').value,

            shop_id: intOrNull(document.getElementById('shop').value),
            category1_code: document.getElementById('category1').value,
            from_account_code: document.getElementById('from-account').value,
            to_account_code: document.getElementById('to-account').value,
            tax_rounding_type: parseInt(document.getElementById('tax-rounding-type').value, 10),
            tax_included_type: parseInt(document.getElementById('tax-included-type').value, 10),
            header_memo: stringOrNull(document.getElementById('header-memo').value),

            detail: {
                category1_code: document.getElementById('category1').value,
                category2_code: stringOrNull(document.getElementById('category2').value),
                category3_code: stringOrNull(document.getElementById('category3').value),
                item_name: document.getElementById('item-name').value,
                amount: amountExcludingTax,
                tax_amount: taxAmount,
                tax_rate: taxRate,
                amount_including_tax: amountIncludingTax,
                detail_memo: stringOrNull(document.getElementById('detail-memo').value),
            },
        };

        // Client-side guards (server validates anyway; this is just UX)
        if (!request.category1_code) {
            showResult('error', i18n.t('recurring_rule.err_category1_required') || 'Category 1 is required.');
            return;
        }
        if (!request.detail.item_name.trim()) {
            showResult('error', i18n.t('recurring_rule.err_item_name_required') || 'Item name is required.');
            return;
        }
        if (cycleKind === 'MONTH' && monthlyMode === 'DAY' &&
            (!request.day_of_month || request.day_of_month < 1 || request.day_of_month > 31)) {
            showResult('error', i18n.t('recurring_rule.err_day_of_month_invalid') || 'Day of month must be 1–31.');
            return;
        }
        if (cycleKind === 'MONTH' && monthlyMode === 'NTH_WEEKDAY' &&
            (!request.week_of_month || !request.day_of_week)) {
            showResult('error', i18n.t('recurring_rule.err_nth_weekday_invalid') || 'Week and weekday must be selected.');
            return;
        }

        // Bounded-field max-length checks (mirror Rust defense in
        // src/services/recurring.rs::create_rule_with_instances).
        const ruleNameInput = document.getElementById('rule-name');
        const headerMemoInput = document.getElementById('header-memo');
        const itemNameInput = document.getElementById('item-name');
        const detailMemoInput = document.getElementById('detail-memo');
        clearValidationError(ruleNameInput);
        clearValidationError(headerMemoInput);
        clearValidationError(itemNameInput);
        clearValidationError(detailMemoInput);

        const ruleName = request.rule_name || '';
        const headerMemo = request.header_memo || '';
        const itemName = request.detail.item_name;
        const detailMemo = request.detail.detail_memo || '';

        if ([...ruleName].length > MAX_RULE_NAME_LEN) {
            showMaxLengthError(ruleNameInput, i18n.t('recurring_rule.rule_name'), MAX_RULE_NAME_LEN);
            return;
        }
        if ([...itemName].length > MAX_ITEM_NAME_LEN) {
            showMaxLengthError(itemNameInput, i18n.t('recurring_rule.item_name'), MAX_ITEM_NAME_LEN);
            return;
        }
        if ([...headerMemo].length > MAX_MEMO_LEN) {
            showMaxLengthError(headerMemoInput, i18n.t('recurring_rule.header_memo'), MAX_MEMO_LEN);
            return;
        }
        if ([...detailMemo].length > MAX_MEMO_LEN) {
            showMaxLengthError(detailMemoInput, i18n.t('recurring_rule.detail_memo'), MAX_MEMO_LEN);
            return;
        }

        try {
            const result = await invoke('create_recurring_rule', { request });
            const tmpl = i18n.t('recurring_rule.create_success') || 'Created rule #{0} with {1} occurrences.';
            const msg = tmpl
                .replace('{0}', result.rule_id)
                .replace('{1}', result.generated_count);
            showResult('success', msg);
            await loadRules();
        } catch (err) {
            console.error('create_recurring_rule failed:', err);

            // Map bounded-field validation errors back to the inline
            // message on the offending input. The Rust side emits a
            // structured `ApiError { code: 'validation', message: '...' }`
            // (PR2a); the `code === 'validation'` gate scopes the
            // message-substring lookup so a generic database error can
            // never accidentally match one of the field needles below.
            const isValidation = err
                && typeof err === 'object'
                && err.code === API_ERROR_CODES.VALIDATION;
            if (isValidation) {
                const message = String(err.message || '');
                const map = [
                    ['Rule name must be', ruleNameInput, 'recurring_rule.rule_name', MAX_RULE_NAME_LEN, ruleName],
                    ['Item name must be', itemNameInput, 'recurring_rule.item_name', MAX_ITEM_NAME_LEN, itemName],
                    ['Header memo must be', headerMemoInput, 'recurring_rule.header_memo', MAX_MEMO_LEN, headerMemo],
                    ['Detail memo must be', detailMemoInput, 'recurring_rule.detail_memo', MAX_MEMO_LEN, detailMemo],
                ];
                for (const [needle, input, fieldKey, max, value] of map) {
                    if (input && message.startsWith(needle)) {
                        showValidationError(input, i18n.t('validation.max_length', {
                            field: i18n.t(fieldKey),
                            max,
                            actual: [...value].length,
                        }));
                        return;
                    }
                }
            }

            if (err && typeof err === 'object'
                && err.code === API_ERROR_CODES.TRANSFER_SAME_ACCOUNT) {
                showResult('error', i18n.t('transaction_mgmt.transfer_same_account'));
                return;
            }

            if (err && typeof err === 'object'
                && err.code === API_ERROR_CODES.RECURRING_PERIOD_OUT_OF_RANGE) {
                const limits = await loadPeriodLimits();
                applyPeriodLimits(limits);
                showResult('error', periodOutOfRangeMessage(limits.min, limits.max));
                return;
            }

            if (err && typeof err === 'object'
                && err.code === API_ERROR_CODES.RECURRING_HOLIDAY_SHIFT_TOO_LONG) {
                showResult('error', i18n.t('recurring_rule.holiday_shift_too_long'));
                return;
            }

            const prefix = i18n.t('recurring_rule.create_failed') || 'Failed to create rule:';
            showResult('error', `${prefix} ${formatApiError(err)}`);
        }
    }));
}

// First and last date (YYYY-MM-DD) a recurring rule may cover, as the
// backend enforces them (latent-audit M15 / M18): the seeded holiday years,
// which can lag the calendar when the app runs across New Year. Asked on
// every use so the pickers, the pre-check and the message stay in step with
// the backend; the local calculation is only a fallback.
async function loadPeriodLimits() {
    try {
        const limits = await invoke('get_recurring_period_limits');
        return { min: limits.first, max: limits.last };
    } catch (error) {
        console.warn('Failed to load recurring period limits, using the local calculation:', error);
        return recurringPeriodLimits(new Date());
    }
}

// Keep the date pickers on the latest bounds, so a date they offer is never
// one the pre-check or the backend then rejects.
function applyPeriodLimits(limits) {
    for (const id of ['start-date', 'end-date']) {
        const input = document.getElementById(id);
        input.min = limits.min;
        input.max = limits.max;
    }
}

// Local fallback for loadPeriodLimits(): the years whose holidays are seeded
// at startup, counted from this year.
function recurringPeriodLimits(today) {
    const year = today.getFullYear();
    return {
        min: `${year - HOLIDAY_SEED_YEARS_BACK}-01-01`,
        max: `${year + HOLIDAY_SEED_YEARS_AHEAD}-12-31`,
    };
}

function periodOutOfRangeMessage(min, max) {
    return i18n.t('recurring_rule.period_out_of_range', { start: min, end: max });
}

function setupResetButton() {
    document.getElementById('reset-btn').addEventListener('click', () => {
        document.getElementById('recurring-rule-form').reset();
        anchorEditedByUser = false;
        hideResult();
        updateDerivedTotal();
        // form.reset() does not fire 'input', so refresh counters manually.
        ['rule-name', 'header-memo', 'item-name', 'detail-memo'].forEach((id) => {
            const el = document.getElementById(id);
            if (el) {
                clearValidationError(el);
                el.dispatchEvent(new Event('input'));
            }
        });
    });
}

function setupBoundedFieldCounters() {
    const ruleName = document.getElementById('rule-name');
    const headerMemo = document.getElementById('header-memo');
    const itemName = document.getElementById('item-name');
    const detailMemo = document.getElementById('detail-memo');

    if (ruleName) {
        attachCharCounter(ruleName, MAX_RULE_NAME_LEN);
        ruleName.addEventListener('input', () => clearValidationError(ruleName));
    }
    if (headerMemo) {
        attachCharCounter(headerMemo, MAX_MEMO_LEN);
        headerMemo.addEventListener('input', () => clearValidationError(headerMemo));
    }
    if (itemName) {
        attachCharCounter(itemName, MAX_ITEM_NAME_LEN);
        itemName.addEventListener('input', () => clearValidationError(itemName));
    }
    if (detailMemo) {
        attachCharCounter(detailMemo, MAX_MEMO_LEN);
        detailMemo.addEventListener('input', () => clearValidationError(detailMemo));
    }
}

// ----- Rule list -----

async function loadRules() {
    try {
        const rules = await invoke('list_recurring_rules', {});
        renderRules(rules);
    } catch (err) {
        console.error('list_recurring_rules failed:', err);
    }
}

function renderRules(rules) {
    const tbody = document.getElementById('rules-tbody');
    const emptyMsg = document.getElementById('no-rules-message');
    const table = document.getElementById('rules-table');
    tbody.innerHTML = '';

    if (!rules || rules.length === 0) {
        table.classList.add('hidden');
        emptyMsg.classList.remove('hidden');
        return;
    }
    table.classList.remove('hidden');
    emptyMsg.classList.add('hidden');

    rules.forEach((r) => {
        const tr = document.createElement('tr');

        const tdName = document.createElement('td');
        tdName.textContent = r.rule_name || `#${r.rule_id}`;
        tr.appendChild(tdName);

        const tdCycle = document.createElement('td');
        tdCycle.textContent = formatCycle(r.period_unit, r.period_interval, r.holiday_shift_type);
        tr.appendChild(tdCycle);

        const tdRange = document.createElement('td');
        tdRange.textContent = `${r.start_date} 〜 ${r.end_date}`;
        tr.appendChild(tdRange);

        const tdAmount = document.createElement('td');
        tdAmount.className = 'col-amount';
        tdAmount.textContent = r.total_amount.toLocaleString();
        tr.appendChild(tdAmount);

        const tdCount = document.createElement('td');
        tdCount.className = 'col-count';
        tdCount.textContent = r.occurrence_count;
        tr.appendChild(tdCount);

        const tdActions = document.createElement('td');
        const delBtn = document.createElement('button');
        delBtn.type = 'button';
        delBtn.className = 'btn-danger';
        delBtn.textContent = i18n.t('recurring_rule.delete') || 'Delete';
        delBtn.addEventListener('click', () => openDeleteModal(r));
        tdActions.appendChild(delBtn);
        tr.appendChild(tdActions);

        tbody.appendChild(tr);
    });
}

function formatCycle(unit, interval, shiftType) {
    const unitLabel = {
        DAY: i18n.t('recurring_rule.cycle_daily') || 'Daily',
        WEEK: 'Weekly',
        MONTH: i18n.t('recurring_rule.cycle_monthly') || 'Monthly',
        YEAR: 'Yearly',
    }[unit] || unit;
    const shiftLabel = [
        i18n.t('recurring_rule.holiday_shift_none') || 'no shift',
        i18n.t('recurring_rule.holiday_shift_prev') || 'prev BD',
        i18n.t('recurring_rule.holiday_shift_next') || 'next BD',
    ][shiftType] || '';
    return `${unitLabel} × ${interval} (${shiftLabel})`;
}

// ----- Delete modal -----

function setupDeleteModal() {
    document.getElementById('delete-cancel-btn').addEventListener('click', closeDeleteModal);
    document.getElementById('delete-modal-close').addEventListener('click', closeDeleteModal);
    document.getElementById('delete-detach-btn').addEventListener('click', () => deleteRule(false));
    document.getElementById('delete-cascade-btn').addEventListener('click', () => deleteRule(true));
}

function openDeleteModal(rule) {
    pendingDeleteRule = rule;
    const tmpl = i18n.t('recurring_rule.delete_confirm_message')
        || 'Delete rule "{0}"? It currently has {1} generated occurrence(s).';
    const name = rule.rule_name || `#${rule.rule_id}`;
    document.getElementById('delete-modal-message').textContent = tmpl
        .replace('{0}', name)
        .replace('{1}', rule.occurrence_count);
    document.getElementById('delete-rule-modal').classList.remove('hidden');
}

function closeDeleteModal() {
    pendingDeleteRule = null;
    document.getElementById('delete-rule-modal').classList.add('hidden');
}

async function deleteRule(cascade) {
    if (!pendingDeleteRule) return;
    const ruleId = pendingDeleteRule.rule_id;
    closeDeleteModal();
    try {
        await invoke('delete_recurring_rule', { ruleId, cascade });
        const tmpl = i18n.t('recurring_rule.delete_success') || 'Rule #{0} deleted.';
        showResult('success', tmpl.replace('{0}', ruleId));
        await loadRules();
    } catch (err) {
        console.error('delete_recurring_rule failed:', err);

        // Target rule was removed by another window (or a stale rule_id):
        // surface the dedicated not_found message and refresh the list so
        // the phantom row disappears — matches the Shop/Product/Manufacturer/
        // Category master-audit contract (PR #75/#76/#77/#83). Now gated on
        // the stable `err.code` returned by `Result<_, ApiError>` (PR2a)
        // instead of the previous English message substring; a French /
        // localised sqlx error text can no longer accidentally match.
        const isNotFound = err
            && typeof err === 'object'
            && err.code === API_ERROR_CODES.NOT_FOUND;
        if (isNotFound) {
            const notFoundMsg = i18n.t('recurring_rule.not_found') ||
                'Recurring rule not found. The list has been reloaded.';
            showResult('error', notFoundMsg);
            await loadRules();
            return;
        }

        const prefix = i18n.t('recurring_rule.delete_failed') || 'Failed to delete rule:';
        showResult('error', `${prefix} ${formatApiError(err)}`);
    }
}

// ----- Result helpers -----

function showResult(kind, message) {
    const box = document.getElementById('result-box');
    box.className = `result-box ${kind}`;
    box.textContent = message;
    box.classList.remove('hidden');
}

function hideResult() {
    const box = document.getElementById('result-box');
    box.classList.add('hidden');
    box.textContent = '';
}

function stringOrNull(s) {
    if (s === null || s === undefined) return null;
    const t = String(s).trim();
    return t === '' ? null : t;
}

function intOrNull(s) {
    if (s === null || s === undefined || s === '') return null;
    const n = parseInt(s, 10);
    return Number.isNaN(n) ? null : n;
}
