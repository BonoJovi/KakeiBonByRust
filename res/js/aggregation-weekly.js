import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { setupLanguageMenuHandlers, setupLanguageMenu } from './menu.js';
import { HTML_FILES } from './html-files.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';
import * as AggCommon from './aggregation-common.js';

console.log('aggregation-weekly.js loaded');

document.addEventListener('DOMContentLoaded', async function() {
    console.log('[DOMContentLoaded] DOM loaded');

    createMenuBar('management');

    if (!await isSessionAuthenticated()) {
        console.error('Not authenticated, redirecting to login');
        window.location.href = HTML_FILES.INDEX;
        return;
    }

    const user = await getCurrentSessionUser();
    if (!user) {
        console.error('Failed to get user info, redirecting to login');
        window.location.href = HTML_FILES.INDEX;
        return;
    }

    console.log(`Logged in as: ${user.name} (ID: ${user.user_id}, Role: ${user.role})`);

    await i18n.init();
    i18n.updateUI();

    setupMenuHandlers();
    await setupLanguageMenu();
    setupLanguageMenuHandlers();
    setupFontSizeMenuHandlers();
    await setupFontSizeMenu();
    setupFontSizeModalHandlers();
    await applyFontSize();
    setupIndicators();

    initializeFilterDefaults();
    setupEventHandlers();
    await adjustWindowSize();

    console.log('[DOMContentLoaded] Initialization complete');
});

function initializeFilterDefaults() {
    const dateInput = document.getElementById('reference-date');

    // Set today's date
    const today = AggCommon.getCurrentDate();
    dateInput.value = AggCommon.formatDate(today);

    updateWeekRangeDisplay();
}

function setupEventHandlers() {
    const toggleFilterBtn = document.getElementById('toggle-filter-btn');
    const filterFormContent = document.getElementById('filter-form-content');
    const filterHeader = document.getElementById('filter-header');

    if (toggleFilterBtn && filterFormContent) {
        toggleFilterBtn.addEventListener('click', () => {
            filterFormContent.classList.toggle('collapsed');
            toggleFilterBtn.classList.toggle('collapsed');
        });

        filterHeader.addEventListener('click', (e) => {
            if (e.target !== toggleFilterBtn) {
                filterFormContent.classList.toggle('collapsed');
                toggleFilterBtn.classList.toggle('collapsed');
            }
        });
    }

    const executeBtn = document.getElementById('execute-btn');
    executeBtn.addEventListener('click', executeAggregation);

    const dateInput = document.getElementById('reference-date');
    dateInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') executeAggregation();
    });
    dateInput.addEventListener('change', updateWeekRangeDisplay);

    const weekStartSelect = document.getElementById('week-start');
    weekStartSelect.addEventListener('change', updateWeekRangeDisplay);
}

function updateWeekRangeDisplay() {
    const display = document.getElementById('week-range-display');
    if (!display) return;

    const dateStr = document.getElementById('reference-date').value;
    const weekStart = document.getElementById('week-start').value;

    if (!dateStr) {
        display.textContent = '';
        return;
    }

    const refDate = new Date(dateStr + 'T00:00:00');
    const dayOfWeek = refDate.getDay(); // 0=Sun, 1=Mon, ..., 6=Sat

    // Calculate days back to the start of the week
    let startOffset;
    if (weekStart === 'monday') {
        // Monday=0, Tue=1, ..., Sun=6
        startOffset = (dayOfWeek + 6) % 7;
    } else {
        // Sunday=0, Mon=1, ..., Sat=6
        startOffset = dayOfWeek;
    }

    const startDate = new Date(refDate);
    startDate.setDate(refDate.getDate() - startOffset);
    const endDate = new Date(startDate);
    endDate.setDate(startDate.getDate() + 6);

    const dayNames = {
        en: ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'],
        ja: ['日', '月', '火', '水', '木', '金', '土']
    };
    const lang = i18n.getCurrentLanguage ? i18n.getCurrentLanguage() : 'en';
    const names = dayNames[lang] || dayNames['en'];

    const fmt = (d) => {
        const y = d.getFullYear();
        const m = String(d.getMonth() + 1).padStart(2, '0');
        const day = String(d.getDate()).padStart(2, '0');
        const dn = names[d.getDay()];
        return `${y}/${m}/${day}(${dn})`;
    };

    const label = i18n.t('aggregation.week_range_label') || 'Target week';
    display.textContent = `${label}: ${fmt(startDate)} ~ ${fmt(endDate)}`;
}

function setupMenuHandlers() {
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    if (fileMenu && fileDropdown) {
        fileMenu.addEventListener('click', (e) => {
            e.stopPropagation();
            fileDropdown.classList.toggle('show');
        });
        
        document.addEventListener('click', () => {
            document.querySelectorAll('.dropdown').forEach(dropdown => {
                dropdown.classList.remove('show');
            });
        });
    }
}

async function executeAggregation() {
    const user = await getCurrentSessionUser();
    if (!user) {
        showMessage('error', i18n.t('common.not_authenticated') || 'Not authenticated');
        return;
    }

    const dateStr = document.getElementById('reference-date').value;
    const weekStart = document.getElementById('week-start').value;
    const groupBy = document.getElementById('group-by').value;

    if (!dateStr) {
        showMessage('error', i18n.t('aggregation.error_no_date') || 'Please select a date');
        return;
    }

    const resultsContainer = document.getElementById('results-container');
    resultsContainer.classList.add('loading');
    clearMessage();
    
    const accountNote = document.getElementById('account-note');
    if (accountNote) {
        accountNote.style.display = (groupBy === 'account') ? 'block' : 'none';
    }

    try {
        console.log(`Executing weekly aggregation: user_id=${user.user_id}, reference_date=${dateStr}, week_start=${weekStart}, group_by=${groupBy}`);

        const includeScheduled = document.getElementById('filter-include-scheduled').checked;
        const results = await invoke('get_weekly_aggregation_by_date', {
            referenceDate: dateStr,
            weekStart: weekStart,
            groupBy: groupBy,
            includeScheduled: includeScheduled
        });

        console.log('Aggregation results:', results);
        
        const tbody = document.getElementById('results-list');
        const tfoot = document.getElementById('results-footer');
        AggCommon.renderResults(results, tbody, tfoot);

        const resultCount = document.getElementById('result-count');
        resultCount.textContent = `(${results.length} ${i18n.t('aggregation.items') || 'items'})`;

    } catch (error) {
        console.error('Aggregation error:', error);
        showMessage('error', AggCommon.translateAggregationError(error));
        clearResults();
    } finally {
        resultsContainer.classList.remove('loading');
    }
}

function clearResults() {
    const tbody = document.getElementById('results-list');
    const tfoot = document.getElementById('results-footer');
    const resultCount = document.getElementById('result-count');

    tbody.innerHTML = '';
    tfoot.innerHTML = '';
    resultCount.textContent = '';
}

function showMessage(type, message) {
    const messageEl = document.getElementById('results-message');
    messageEl.textContent = message;
    messageEl.className = `message ${type}`;
    messageEl.style.display = 'block';
}

function clearMessage() {
    const messageEl = document.getElementById('results-message');
    messageEl.textContent = '';
    messageEl.style.display = 'none';
}
