import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { setupLanguageMenuHandlers, setupLanguageMenu } from './menu.js';
import { HTML_FILES } from './html-files.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';
import * as AggCommon from './aggregation-common.js';

console.log('aggregation-yearly.js loaded');

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
    const yearInput = document.getElementById('year');
    yearInput.value = AggCommon.getCurrentYear();
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

    const yearInput = document.getElementById('year');
    yearInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') executeAggregation();
    });

    const yearDownBtn = document.getElementById('year-down');
    const yearUpBtn = document.getElementById('year-up');

    yearDownBtn.addEventListener('click', () => {
        const currentYear = parseInt(yearInput.value) || AggCommon.getCurrentYear();
        if (currentYear > 1900) {
            yearInput.value = currentYear - 1;
        }
    });

    yearUpBtn.addEventListener('click', () => {
        const currentYear = parseInt(yearInput.value) || AggCommon.getCurrentYear();
        if (currentYear < 2100) {
            yearInput.value = currentYear + 1;
        }
    });
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
        showMessage('error', 'Not authenticated');
        return;
    }

    const year = parseInt(document.getElementById('year').value);
    const yearStart = document.getElementById('year-start').value;
    const groupBy = document.getElementById('group-by').value;

    if (!year || year < 1900 || year > 2100) {
        showMessage('error', i18n.t('aggregation.error_invalid_year') || 'Please enter a valid year');
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
        console.log(`Executing yearly aggregation: user_id=${user.user_id}, year=${year}, year_start=${yearStart}, group_by=${groupBy}`);

        const results = await invoke('get_yearly_aggregation', {
            year: year,
            yearStart: yearStart,
            groupBy: groupBy
        });

        console.log('Aggregation results:', results);
        
        const tbody = document.getElementById('results-list');
        const tfoot = document.getElementById('results-footer');
        AggCommon.renderResults(results, tbody, tfoot);

        const resultCount = document.getElementById('result-count');
        resultCount.textContent = `(${results.length} ${i18n.t('aggregation.items') || 'items'})`;

    } catch (error) {
        console.error('Aggregation error:', error);
        showMessage('error', error.toString());
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
