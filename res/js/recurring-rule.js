import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { setupIndicators } from './indicators.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar, setupLanguageMenu, setupLanguageMenuHandlers } from './menu.js';

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
        setupCategoryChainHandlers();
        setupFormSubmit();
        setupResetButton();
        setupDeleteModal();
        await loadRules();

        // Default start_date to today, end_date to one year out
        const today = new Date();
        const oneYearLater = new Date(today.getFullYear() + 1, today.getMonth(), today.getDate());
        document.getElementById('start-date').value = today.toISOString().slice(0, 10);
        document.getElementById('end-date').value = oneYearLater.toISOString().slice(0, 10);
        document.getElementById('anchor-date').value = today.toISOString().slice(0, 10);

        await adjustWindowSize();
        // Form is taller than the window; ensure the user starts at the top
        window.scrollTo(0, 0);
    } catch (err) {
        console.error('Initialization error:', err);
        showResult('error', String(err));
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
    const accounts = await invoke('get_accounts', {});
    const fromSel = document.getElementById('from-account');
    const toSel = document.getElementById('to-account');
    fromSel.innerHTML = '';
    toSel.innerHTML = '';
    accounts.forEach((acc) => {
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
    const shops = await invoke('get_shops', {});
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

// ----- Form submit -----

function setupFormSubmit() {
    document.getElementById('recurring-rule-form').addEventListener('submit', async (e) => {
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
                monthDayRuleType = 'DAY';
                dayOfMonth = parseInt(document.getElementById('day-of-month').value, 10);
            } else if (monthlyMode === 'NTH_WEEKDAY') {
                monthDayRuleType = 'NTH_WEEKDAY';
                weekOfMonth = parseInt(document.getElementById('week-of-month').value, 10);
                dayOfWeek = parseInt(document.getElementById('day-of-week').value, 10);
            }
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
            total_amount: parseInt(document.getElementById('total-amount').value, 10) || 0,
            tax_rounding_type: parseInt(document.getElementById('tax-rounding-type').value, 10),
            tax_included_type: parseInt(document.getElementById('tax-included-type').value, 10),
            header_memo: stringOrNull(document.getElementById('header-memo').value),

            detail: {
                category1_code: document.getElementById('category1').value,
                category2_code: stringOrNull(document.getElementById('category2').value),
                category3_code: stringOrNull(document.getElementById('category3').value),
                item_name: document.getElementById('item-name').value,
                amount: parseInt(document.getElementById('amount').value, 10) || 0,
                tax_amount: parseInt(document.getElementById('tax-amount').value, 10) || 0,
                tax_rate: parseInt(document.getElementById('tax-rate').value, 10) || 0,
                amount_including_tax: intOrNull(
                    document.getElementById('amount-including-tax').value
                ),
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
            const prefix = i18n.t('recurring_rule.create_failed') || 'Failed to create rule:';
            showResult('error', `${prefix} ${err}`);
        }
    });
}

function setupResetButton() {
    document.getElementById('reset-btn').addEventListener('click', () => {
        document.getElementById('recurring-rule-form').reset();
        hideResult();
    });
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
        const prefix = i18n.t('recurring_rule.delete_failed') || 'Failed to delete rule:';
        showResult('error', `${prefix} ${err}`);
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
