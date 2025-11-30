import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { HTML_FILES } from './html-files.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';

console.log('aggregation.js loaded');

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

    // Initialize filter defaults
    initializeFilterDefaults();

    // Setup event handlers
    setupEventHandlers();

    // Adjust window size
    await adjustWindowSize();

    console.log('[DOMContentLoaded] Initialization complete');
});

function initializeFilterDefaults() {
    const now = new Date();
    const yearInput = document.getElementById('year');
    const monthSelect = document.getElementById('month');

    // Set current year
    yearInput.value = now.getFullYear();

    // Set current month
    monthSelect.value = now.getMonth() + 1;
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

        // Also toggle when clicking the header
        filterHeader.addEventListener('click', (e) => {
            // Don't toggle if clicking directly on the button
            if (e.target !== toggleFilterBtn) {
                filterFormContent.classList.toggle('collapsed');
                toggleFilterBtn.classList.toggle('collapsed');
            }
        });
    }

    // Execute button
    const executeBtn = document.getElementById('execute-btn');
    executeBtn.addEventListener('click', executeAggregation);

    // Enter key in year input
    const yearInput = document.getElementById('year');
    yearInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            executeAggregation();
        }
    });

    // Year spinner buttons
    const yearDownBtn = document.getElementById('year-down');
    const yearUpBtn = document.getElementById('year-up');

    yearDownBtn.addEventListener('click', () => {
        const currentYear = parseInt(yearInput.value) || new Date().getFullYear();
        if (currentYear > 1900) {
            yearInput.value = currentYear - 1;
        }
    });

    yearUpBtn.addEventListener('click', () => {
        const currentYear = parseInt(yearInput.value) || new Date().getFullYear();
        if (currentYear < 2100) {
            yearInput.value = currentYear + 1;
        }
    });
}

async function executeAggregation() {
    const user = await getCurrentSessionUser();
    if (!user) {
        showMessage('error', 'User not authenticated');
        return;
    }

    const yearInput = document.getElementById('year');
    const monthSelect = document.getElementById('month');
    const groupBySelect = document.getElementById('group-by');

    const year = parseInt(yearInput.value);
    const month = parseInt(monthSelect.value);
    const groupBy = groupBySelect.value;

    // Validate inputs
    if (isNaN(year) || year < 1900 || year > 2100) {
        showMessage('error', i18n.t('aggregation.error_invalid_year') || 'Invalid year');
        return;
    }

    if (isNaN(month) || month < 1 || month > 12) {
        showMessage('error', i18n.t('aggregation.error_invalid_month') || 'Invalid month');
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
        console.log(`Executing aggregation: user_id=${user.user_id}, year=${year}, month=${month}, group_by=${groupBy}`);

        const results = await invoke('get_monthly_aggregation', {
            year: year,
            month: month,
            groupBy: groupBy
        });

        console.log('Aggregation results:', results);
        displayResults(results);

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

function displayResults(results) {
    const tbody = document.getElementById('results-list');
    const tfoot = document.getElementById('results-footer');

    // Clear previous results
    tbody.innerHTML = '';
    tfoot.innerHTML = '';

    if (results.length === 0) {
        tbody.innerHTML = `
            <tr>
                <td colspan="4" class="empty-state">
                    ${i18n.t('aggregation.no_results') || 'No results found'}
                </td>
            </tr>
        `;
        return;
    }

    // Calculate totals
    let totalAmount = 0;
    let totalCount = 0;

    // Display each result
    results.forEach(result => {
        const tr = document.createElement('tr');

        tr.innerHTML = `
            <td>${escapeHtml(result.group_name)}</td>
            <td class="amount">${formatAmount(result.total_amount)}</td>
            <td class="amount">${result.count.toLocaleString()}</td>
            <td class="amount">${formatAmount(result.avg_amount)}</td>
        `;

        tbody.appendChild(tr);

        totalAmount += result.total_amount;
        totalCount += result.count;
    });

    // Calculate overall average
    const avgAmount = totalCount > 0 ? Math.round(totalAmount / totalCount) : 0;

    // Display totals in footer
    const footerTr = document.createElement('tr');
    footerTr.innerHTML = `
        <td>${i18n.t('aggregation.total') || 'Total'}</td>
        <td class="amount">${formatAmount(totalAmount)}</td>
        <td class="amount">${totalCount.toLocaleString()}</td>
        <td class="amount">${formatAmount(avgAmount)}</td>
    `;
    tfoot.appendChild(footerTr);
}

function clearResults() {
    const tbody = document.getElementById('results-list');
    const tfoot = document.getElementById('results-footer');
    const resultCount = document.getElementById('result-count');

    tbody.innerHTML = '';
    tfoot.innerHTML = '';
    resultCount.textContent = '';
}

function formatAmount(amount) {
    // Format as Japanese yen style
    return '¥' + amount.toLocaleString();
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function showMessage(type, message) {
    const messageEl = document.getElementById('results-message');
    messageEl.textContent = message;
    messageEl.className = `message ${type}`;
}

function clearMessage() {
    const messageEl = document.getElementById('results-message');
    messageEl.textContent = '';
    messageEl.className = 'message';
}

// Menu handlers (copied from other management pages)
function setupMenuHandlers() {
    // Close all dropdowns when clicking outside
    document.addEventListener('click', (e) => {
        if (!e.target.closest('.menu-item')) {
            closeAllDropdowns();
        }
    });
}

function closeAllDropdowns() {
    document.querySelectorAll('.dropdown-menu.show').forEach(menu => {
        menu.classList.remove('show');
    });
}

async function setupLanguageMenu() {
    try {
        const languages = await invoke('get_language_names');
        const languageMenu = document.querySelector('.language-dropdown');
        if (!languageMenu) return;

        languageMenu.innerHTML = '';

        for (const [code, name] of Object.entries(languages)) {
            const item = document.createElement('a');
            item.href = '#';
            item.className = 'dropdown-item';
            item.dataset.lang = code;
            item.textContent = name;
            languageMenu.appendChild(item);
        }

        // Mark current language
        const currentLang = await invoke('get_language');
        const currentItem = languageMenu.querySelector(`[data-lang="${currentLang}"]`);
        if (currentItem) {
            currentItem.classList.add('active');
        }
    } catch (error) {
        console.error('Failed to setup language menu:', error);
    }
}

function setupLanguageMenuHandlers() {
    const languageMenu = document.querySelector('.language-dropdown');
    if (!languageMenu) return;

    languageMenu.addEventListener('click', async (e) => {
        e.preventDefault();
        const item = e.target.closest('.dropdown-item');
        if (!item) return;

        const lang = item.dataset.lang;

        try {
            await invoke('set_language', { language: lang });
            await i18n.init();
            i18n.updateUI();

            // Update active state
            languageMenu.querySelectorAll('.dropdown-item').forEach(el => {
                el.classList.remove('active');
            });
            item.classList.add('active');

            closeAllDropdowns();
        } catch (error) {
            console.error('Failed to set language:', error);
        }
    });
}
