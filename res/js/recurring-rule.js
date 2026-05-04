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
    const radios = document.querySelectorAll('input[name="cycle-kind"]');
    radios.forEach((r) =>
        r.addEventListener('change', () => {
            const kind = document.querySelector('input[name="cycle-kind"]:checked').value;
            document
                .getElementById('anchor-date-group')
                .classList.toggle('visible', kind === 'DAY');
            document
                .getElementById('day-of-month-group')
                .classList.toggle('visible', kind === 'MONTH');
        })
    );
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
        const periodInterval = parseInt(document.getElementById('period-interval').value, 10);

        const request = {
            rule_name: stringOrNull(document.getElementById('rule-name').value),
            period_unit: cycleKind,
            period_interval: periodInterval,
            anchor_date: cycleKind === 'DAY' ? document.getElementById('anchor-date').value : null,
            day_of_week: null,
            month_day_rule_type: cycleKind === 'MONTH' ? 'DAY' : null,
            day_of_month:
                cycleKind === 'MONTH'
                    ? parseInt(document.getElementById('day-of-month').value, 10)
                    : null,
            week_of_month: null,
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
        if (cycleKind === 'MONTH' && (!request.day_of_month || request.day_of_month < 1 || request.day_of_month > 31)) {
            showResult('error', i18n.t('recurring_rule.err_day_of_month_invalid') || 'Day of month must be 1–31.');
            return;
        }

        try {
            const result = await invoke('create_recurring_rule', { request });
            const tmpl = i18n.t('recurring_rule.create_success') || 'Created rule #{0} with {1} occurrences.';
            const msg = tmpl
                .replace('{0}', result.rule_id)
                .replace('{1}', result.generated_count);
            showResult('success', msg);
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
