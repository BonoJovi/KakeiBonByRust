import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { setupLanguageMenuHandlers, setupLanguageMenu } from './menu.js';
import { HTML_FILES } from './html-files.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';
import * as AggCommon from './aggregation-common.js';

console.log('aggregation-daily.js loaded');

document.addEventListener('DOMContentLoaded', async function() {
    console.log('[DOMContentLoaded] DOM loaded');

    // Create menu bar
    createMenuBar('management');

    // Check session authentication
    if (!await isSessionAuthenticated()) {
        console.error('Not authenticated, redirecting to login');
        window.location.href = HTML_FILES.INDEX;
        return;
    }

    // Get current user info
    const user = await getCurrentSessionUser();
    if (!user) {
        console.error('Failed to get user info, redirecting to login');
        window.location.href = HTML_FILES.INDEX;
        return;
    }

    console.log(`Logged in as: ${user.name} (ID: ${user.user_id}, Role: ${user.role})`);

    await i18n.init();
    i18n.updateUI();

    // Setup menu handlers
    setupMenuHandlers();

    // Setup language menu
    await setupLanguageMenu();
    setupLanguageMenuHandlers();

    // Setup font size
    setupFontSizeMenuHandlers();
    await setupFontSizeMenu();
    setupFontSizeModalHandlers();
    await applyFontSize();

    // Setup accessibility indicators
    setupIndicators();

    // Initialize filter defaults
    initializeFilterDefaults();

    // Setup event handlers
    setupEventHandlers();

    // Adjust window size
    await adjustWindowSize();

    console.log('[DOMContentLoaded] Initialization complete');
});

function initializeFilterDefaults() {
    const dateInput = document.getElementById('date');
    
    // Set today's date
    const today = AggCommon.getCurrentDate();
    dateInput.value = AggCommon.formatDate(today);
}

function setupEventHandlers() {
    // Toggle filter section
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

    // Execute button
    const executeBtn = document.getElementById('execute-btn');
    executeBtn.addEventListener('click', executeAggregation);

    // Enter key in date input
    const dateInput = document.getElementById('date');
    dateInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            executeAggregation();
        }
    });
}

function setupMenuHandlers() {
    // File menu handlers
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    if (fileMenu && fileDropdown) {
        fileMenu.addEventListener('click', (e) => {
            e.stopPropagation();
            fileDropdown.classList.toggle('show');
        });
        
        // Close dropdowns when clicking outside
        document.addEventListener('click', () => {
            document.querySelectorAll('.dropdown').forEach(dropdown => {
                dropdown.classList.remove('show');
            });
        });
    }
}

async function executeAggregation() {
    // Get current user
    const user = await getCurrentSessionUser();
    if (!user) {
        showMessage('error', 'Not authenticated');
        return;
    }

    // Get filter values
    const dateStr = document.getElementById('date').value;
    const groupBy = document.getElementById('group-by').value;

    if (!dateStr) {
        showMessage('error', i18n.t('aggregation.error_no_date') || 'Please select a date');
        return;
    }

    // Show loading state
    const resultsContainer = document.getElementById('results-container');
    resultsContainer.classList.add('loading');
    clearMessage();
    
    // Show/hide account note based on grouping
    const accountNote = document.getElementById('account-note');
    if (accountNote) {
        accountNote.style.display = (groupBy === 'account') ? 'block' : 'none';
    }

    try {
        console.log(`Executing daily aggregation: user_id=${user.user_id}, date=${dateStr}, group_by=${groupBy}`);

        const results = await invoke('get_daily_aggregation', {
            userId: user.user_id,
            date: dateStr,
            groupBy: groupBy
        });

        console.log('Aggregation results:', results);
        
        // Use common rendering function
        const tbody = document.getElementById('results-list');
        const tfoot = document.getElementById('results-footer');
        AggCommon.renderResults(results, tbody, tfoot);

        // Update result count
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

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
