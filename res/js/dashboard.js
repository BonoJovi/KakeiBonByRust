import { invoke } from '@tauri-apps/api/core';
import { Chart } from 'chart.js';
import i18n from './i18n.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { HTML_FILES } from './html-files.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';

console.log('dashboard.js loaded');

// Chart instances
let charts = {
    expensePie: null,
    categoryBar: null,
    monthlyTrend: null
};

// Color palette for charts
const CHART_COLORS = [
    '#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0',
    '#9966FF', '#FF9F40', '#E7E9ED', '#C9CBCF',
    '#7CB342', '#F06292', '#4DD0E1', '#FFD54F'
];

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

    // Initialize i18n first for error messages
    await i18n.init();
    i18n.updateUI();

    // Check if user is admin (role 0) - admin cannot access dashboard for privacy reasons
    const ROLE_ADMIN = 0;
    if (user.role === ROLE_ADMIN) {
        console.log('Admin user detected, dashboard access denied');
        alert(i18n.t('dashboard.admin_access_denied') || 'Dashboard is not available for administrator accounts. Please login as a regular user.');
        window.location.href = HTML_FILES.INDEX;
        return;
    }

    // Setup menu handlers
    setupMenuHandlers();

    // Setup language menu (handlers first, then populate items)
    setupLanguageMenuHandlers();
    await setupLanguageMenu();

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

    // Load initial dashboard data
    await loadDashboardData();

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

        filterHeader.addEventListener('click', (e) => {
            if (e.target !== toggleFilterBtn) {
                filterFormContent.classList.toggle('collapsed');
                toggleFilterBtn.classList.toggle('collapsed');
            }
        });
    }

    // Execute button
    const executeBtn = document.getElementById('execute-btn');
    executeBtn.addEventListener('click', loadDashboardData);

    // Enter key in year input
    const yearInput = document.getElementById('year');
    yearInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            loadDashboardData();
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

async function loadDashboardData() {
    const user = await getCurrentSessionUser();
    if (!user) {
        showMessage('error', 'User not authenticated');
        return;
    }

    const yearInput = document.getElementById('year');
    const monthSelect = document.getElementById('month');
    const trendMonthsSelect = document.getElementById('trend-months');

    const year = parseInt(yearInput.value);
    const month = parseInt(monthSelect.value);
    const trendMonths = parseInt(trendMonthsSelect.value);

    // Validate inputs
    if (isNaN(year) || year < 1900 || year > 2100) {
        showMessage('error', i18n.t('dashboard.error_invalid_year') || 'Invalid year');
        return;
    }

    if (isNaN(month) || month < 1 || month > 12) {
        showMessage('error', i18n.t('dashboard.error_invalid_month') || 'Invalid month');
        return;
    }

    clearMessage();

    try {
        // Load data in parallel
        const [categoryData, monthlyTrendData] = await Promise.all([
            invoke('get_monthly_aggregation', {
                userId: user.user_id,
                year: year,
                month: month,
                groupBy: 'category2',
                lang: i18n.getCurrentLanguage()
            }),
            getMonthlyTrendData(user.user_id, year, month, trendMonths)
        ]);

        console.log('Category data:', categoryData);
        console.log('Monthly trend data:', monthlyTrendData);

        // Calculate period strings for chart titles
        const monthPeriod = `${year}${i18n.t('dashboard.year_suffix') || '年'}${month}${i18n.t('dashboard.month_suffix') || '月'}`;

        // Calculate trend period (from oldest to newest month)
        let trendPeriod = '';
        if (monthlyTrendData.length > 0) {
            const oldest = monthlyTrendData[0];
            const newest = monthlyTrendData[monthlyTrendData.length - 1];
            const oldestStr = `${oldest.year}${i18n.t('dashboard.year_suffix') || '年'}${oldest.month}${i18n.t('dashboard.month_suffix') || '月'}`;
            const newestStr = `${newest.year}${i18n.t('dashboard.year_suffix') || '年'}${newest.month}${i18n.t('dashboard.month_suffix') || '月'}`;
            trendPeriod = `${oldestStr}〜${newestStr}`;
        }

        // Update chart titles with period
        updateChartTitles(monthPeriod, trendPeriod);

        // Create color map for consistent colors across charts
        const expenses = categoryData.filter(d => d.group_key && d.group_key.startsWith('EXPENSE/'));
        const expenseColorMap = createCategoryColorMap(expenses);

        // Update charts with shared color map
        updateExpensePieChart(categoryData, expenseColorMap);
        updateCategoryBarChart(categoryData, expenseColorMap);
        updateMonthlyTrendChart(monthlyTrendData);

    } catch (error) {
        console.error('Failed to load dashboard data:', error);
        showMessage('error', error.toString());
    }
}

async function getMonthlyTrendData(userId, year, month, count) {
    const results = [];
    const lang = i18n.getCurrentLanguage();

    for (let i = count - 1; i >= 0; i--) {
        let y = year;
        let m = month - i;

        // Handle year rollover
        while (m <= 0) {
            m += 12;
            y--;
        }

        try {
            const data = await invoke('get_monthly_aggregation', {
                userId: userId,
                year: y,
                month: m,
                groupBy: 'category1',
                lang: lang
            });

            results.push({
                year: y,
                month: m,
                label: `${y}/${m}`,
                data: data
            });
        } catch (error) {
            console.error(`Failed to get data for ${y}/${m}:`, error);
            results.push({
                year: y,
                month: m,
                label: `${y}/${m}`,
                data: []
            });
        }
    }

    return results;
}

function updateExpensePieChart(data, colorMap) {
    const ctx = document.getElementById('expense-pie-chart');
    const noDataEl = document.getElementById('expense-pie-no-data');

    // Filter expenses only (group_key starts with "EXPENSE/")
    const expenses = data.filter(d => d.group_key && d.group_key.startsWith('EXPENSE/'));

    if (expenses.length === 0) {
        if (charts.expensePie) {
            charts.expensePie.destroy();
            charts.expensePie = null;
        }
        ctx.style.display = 'none';
        noDataEl.classList.remove('hidden');
        return;
    }

    ctx.style.display = 'block';
    noDataEl.classList.add('hidden');

    // Destroy existing chart
    if (charts.expensePie) {
        charts.expensePie.destroy();
    }

    // Use color map for consistent colors
    const colors = expenses.map(d => colorMap[d.group_key] || CHART_COLORS[0]);

    charts.expensePie = new Chart(ctx, {
        type: 'pie',
        data: {
            labels: expenses.map(d => d.group_name),
            datasets: [{
                data: expenses.map(d => Math.abs(d.total_amount)),
                backgroundColor: colors,
                borderWidth: 1,
                borderColor: '#fff'
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    position: 'right',
                    labels: {
                        boxWidth: 12,
                        padding: 10
                    }
                },
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            const value = context.raw;
                            const total = context.dataset.data.reduce((a, b) => a + b, 0);
                            const percentage = ((value / total) * 100).toFixed(1);
                            return `${context.label}: ${formatAmount(value)} (${percentage}%)`;
                        }
                    }
                }
            }
        }
    });
}

function updateCategoryBarChart(data, colorMap) {
    const ctx = document.getElementById('category-bar-chart');
    const noDataEl = document.getElementById('category-bar-no-data');

    // Separate expenses and income by group_key prefix
    const expenses = data.filter(d => d.group_key && d.group_key.startsWith('EXPENSE/'));
    const income = data.filter(d => d.group_key && d.group_key.startsWith('INCOME/'));

    if (expenses.length === 0 && income.length === 0) {
        if (charts.categoryBar) {
            charts.categoryBar.destroy();
            charts.categoryBar = null;
        }
        ctx.style.display = 'none';
        noDataEl.classList.remove('hidden');
        return;
    }

    ctx.style.display = 'block';
    noDataEl.classList.add('hidden');

    // Destroy existing chart
    if (charts.categoryBar) {
        charts.categoryBar.destroy();
    }

    // Sort expenses by amount (largest first, descending order)
    const sortedExpenses = [...expenses].sort((a, b) => b.total_amount - a.total_amount);
    const top10Expenses = sortedExpenses.slice(0, 10);

    // Use color map for consistent colors with pie chart
    const colors = top10Expenses.map(d => colorMap[d.group_key] || CHART_COLORS[0]);

    charts.categoryBar = new Chart(ctx, {
        type: 'bar',
        data: {
            labels: top10Expenses.map(d => truncateLabel(d.group_name, 15)),
            datasets: [{
                label: i18n.t('dashboard.expense') || 'Expense',
                data: top10Expenses.map(d => Math.abs(d.total_amount)),
                backgroundColor: colors,
                borderColor: colors.map(c => darkenColor(c)),
                borderWidth: 1
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            indexAxis: 'y',
            plugins: {
                legend: {
                    display: false
                },
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            return formatAmount(context.raw);
                        }
                    }
                }
            },
            scales: {
                x: {
                    beginAtZero: true,
                    ticks: {
                        callback: function(value) {
                            return formatAmountShort(value);
                        }
                    }
                },
                y: {
                    ticks: {
                        autoSkip: false
                    }
                }
            }
        }
    });
}

function updateMonthlyTrendChart(monthlyData) {
    const ctx = document.getElementById('monthly-trend-chart');
    const noDataEl = document.getElementById('monthly-trend-no-data');

    if (!monthlyData || monthlyData.length === 0) {
        if (charts.monthlyTrend) {
            charts.monthlyTrend.destroy();
            charts.monthlyTrend = null;
        }
        ctx.style.display = 'none';
        noDataEl.classList.remove('hidden');
        return;
    }

    ctx.style.display = 'block';
    noDataEl.classList.add('hidden');

    // Extract expense and income totals for each month
    const labels = monthlyData.map(m => m.label);
    const expenseData = monthlyData.map(m => {
        const expense = m.data.find(d => d.group_key === 'EXPENSE');
        return expense ? Math.abs(expense.total_amount) : 0;
    });
    const incomeData = monthlyData.map(m => {
        const income = m.data.find(d => d.group_key === 'INCOME');
        return income ? income.total_amount : 0;
    });
    const balanceData = monthlyData.map((m, i) => incomeData[i] - expenseData[i]);

    // Destroy existing chart
    if (charts.monthlyTrend) {
        charts.monthlyTrend.destroy();
    }

    charts.monthlyTrend = new Chart(ctx, {
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                {
                    label: i18n.t('dashboard.expense') || 'Expense',
                    data: expenseData,
                    borderColor: '#e74c3c',
                    backgroundColor: 'rgba(231, 76, 60, 0.1)',
                    fill: false,
                    tension: 0.1
                },
                {
                    label: i18n.t('dashboard.income') || 'Income',
                    data: incomeData,
                    borderColor: '#27ae60',
                    backgroundColor: 'rgba(39, 174, 96, 0.1)',
                    fill: false,
                    tension: 0.1
                },
                {
                    label: i18n.t('dashboard.balance') || 'Balance',
                    data: balanceData,
                    borderColor: '#3498db',
                    backgroundColor: 'rgba(52, 152, 219, 0.1)',
                    fill: false,
                    tension: 0.1,
                    borderDash: [5, 5]
                }
            ]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    position: 'top'
                },
                tooltip: {
                    callbacks: {
                        label: function(context) {
                            return `${context.dataset.label}: ${formatAmount(context.raw)}`;
                        }
                    }
                }
            },
            scales: {
                y: {
                    beginAtZero: true,
                    ticks: {
                        callback: function(value) {
                            return formatAmountShort(value);
                        }
                    }
                }
            }
        }
    });
}

// Utility functions
function updateChartTitles(monthPeriod, trendPeriod) {
    // Update pie chart title
    const pieTitle = document.querySelector('#expense-pie-card h3');
    if (pieTitle) {
        const baseTitle = i18n.t('dashboard.expense_by_category') || 'Expense by Category';
        pieTitle.textContent = `${baseTitle} (${monthPeriod})`;
    }

    // Update bar chart title
    const barTitle = document.querySelector('#category-bar-card h3');
    if (barTitle) {
        const baseTitle = i18n.t('dashboard.category_comparison') || 'Category Comparison';
        barTitle.textContent = `${baseTitle} (${monthPeriod})`;
    }

    // Update line chart title
    const lineTitle = document.querySelector('#monthly-trend-card h3');
    if (lineTitle) {
        const baseTitle = i18n.t('dashboard.monthly_trend') || 'Monthly Trend';
        lineTitle.textContent = `${baseTitle} (${trendPeriod})`;
    }
}

function generateColors(count) {
    return Array.from({length: count}, (_, i) => CHART_COLORS[i % CHART_COLORS.length]);
}

function createCategoryColorMap(categories) {
    const colorMap = {};
    categories.forEach((item, index) => {
        colorMap[item.group_key] = CHART_COLORS[index % CHART_COLORS.length];
    });
    return colorMap;
}

function darkenColor(hex) {
    // Convert hex to RGB, darken by 20%, convert back to hex
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    const factor = 0.8;
    const newR = Math.floor(r * factor);
    const newG = Math.floor(g * factor);
    const newB = Math.floor(b * factor);
    return `#${newR.toString(16).padStart(2, '0')}${newG.toString(16).padStart(2, '0')}${newB.toString(16).padStart(2, '0')}`;
}

function formatAmount(amount) {
    return '¥' + Math.abs(amount).toLocaleString('ja-JP');
}

function formatAmountShort(amount) {
    if (amount >= 1000000) {
        return '¥' + (amount / 1000000).toFixed(1) + 'M';
    } else if (amount >= 1000) {
        return '¥' + (amount / 1000).toFixed(0) + 'K';
    }
    return '¥' + amount.toLocaleString('ja-JP');
}

function truncateLabel(label, maxLength) {
    if (label.length <= maxLength) return label;
    return label.substring(0, maxLength - 2) + '...';
}

function showMessage(type, message) {
    const messageEl = document.getElementById('dashboard-message');
    if (messageEl) {
        messageEl.className = `message ${type}`;
        messageEl.textContent = message;
    }
}

function clearMessage() {
    const messageEl = document.getElementById('dashboard-message');
    if (messageEl) {
        messageEl.className = 'message';
        messageEl.textContent = '';
    }
}

// Menu handlers
function setupMenuHandlers() {
    console.log('[setupMenuHandlers] Setting up menu handlers');

    // File menu
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    const adminMenu = document.getElementById('admin-menu');
    const adminDropdown = document.getElementById('admin-dropdown');

    console.log('[setupMenuHandlers] fileMenu:', fileMenu);
    console.log('[setupMenuHandlers] fileDropdown:', fileDropdown);

    // Toggle dropdowns on click
    if (fileMenu && fileDropdown) {
        fileMenu.addEventListener('click', (e) => {
            e.stopPropagation();
            fileDropdown.classList.toggle('show');
            if (adminDropdown) adminDropdown.classList.remove('show');
        });

        // Prevent clicks inside dropdown from closing it immediately
        fileDropdown.addEventListener('click', (e) => {
            e.stopPropagation();
        });

        // File menu items for management pages
        // Structure: Back to Main, separator, Logout, Quit
        const backToMainItem = fileDropdown.querySelector('[data-i18n="menu.back_to_main"]');
        const logoutItem = fileDropdown.querySelector('[data-i18n="menu.logout"]');
        const quitItem = fileDropdown.querySelector('[data-i18n="menu.quit"]');

        console.log('[setupMenuHandlers] backToMainItem:', backToMainItem);
        console.log('[setupMenuHandlers] logoutItem:', logoutItem);
        console.log('[setupMenuHandlers] quitItem:', quitItem);

        if (backToMainItem) {
            backToMainItem.addEventListener('click', function(e) {
                e.stopPropagation();
                console.log('Back to Main clicked');
                window.location.href = HTML_FILES.INDEX;
            });
        }

        if (logoutItem) {
            console.log('[setupMenuHandlers] Adding logout handler');
            logoutItem.addEventListener('click', async function(e) {
                e.preventDefault();
                e.stopPropagation();
                console.log('Logout clicked!');
                fileDropdown.classList.remove('show');
                try {
                    // Use session.clearSession() instead of invoke('logout')
                    const session = await import('./session.js');
                    await session.clearSession();
                    console.log('Session cleared, redirecting to login');
                    window.location.href = HTML_FILES.INDEX;
                } catch (error) {
                    console.error('Logout failed:', error);
                }
            });
        } else {
            console.error('[setupMenuHandlers] logoutItem NOT FOUND!');
        }

        if (quitItem) {
            quitItem.addEventListener('click', async function(e) {
                e.preventDefault();
                e.stopPropagation();
                console.log('Quit clicked');
                try {
                    await invoke('handle_quit');
                } catch (error) {
                    console.error('Quit failed:', error);
                }
            });
        }
    } else {
        console.error('[setupMenuHandlers] fileMenu or fileDropdown NOT FOUND!');
    }

    if (adminMenu && adminDropdown) {
        adminMenu.addEventListener('click', (e) => {
            e.stopPropagation();
            adminDropdown.classList.toggle('show');
            if (fileDropdown) fileDropdown.classList.remove('show');
        });
    }

    // Close dropdowns when clicking outside
    document.addEventListener('click', (e) => {
        // Don't close if clicking inside a dropdown
        if (e.target.closest('.dropdown')) {
            return;
        }
        if (fileDropdown) fileDropdown.classList.remove('show');
        if (adminDropdown) adminDropdown.classList.remove('show');
    });

    console.log('[setupMenuHandlers] Setup complete');
}

async function handleMenuClick(menuKey) {
    switch (menuKey) {
        case 'menu.back_to_main':
            window.location.href = HTML_FILES.INDEX;
            break;
        case 'menu.logout':
            try {
                const session = await import('./session.js');
                await session.clearSession();
                window.location.href = HTML_FILES.INDEX;
            } catch (error) {
                console.error('Logout failed:', error);
            }
            break;
        case 'menu.quit':
            try {
                await invoke('handle_quit');
            } catch (error) {
                console.error('Quit failed:', error);
            }
            break;
        case 'menu.user_management':
            window.location.href = HTML_FILES.USER_MANAGEMENT;
            break;
        case 'menu.category_management':
            window.location.href = HTML_FILES.CATEGORY_MANAGEMENT;
            break;
        case 'menu.account_management':
            window.location.href = HTML_FILES.ACCOUNT_MANAGEMENT;
            break;
        case 'menu.shop_management':
            window.location.href = HTML_FILES.SHOP_MANAGEMENT;
            break;
        case 'menu.manufacturer_management':
            window.location.href = HTML_FILES.MANUFACTURER_MANAGEMENT;
            break;
        case 'menu.product_management':
            window.location.href = HTML_FILES.PRODUCT_MANAGEMENT;
            break;
        case 'menu.transaction_management':
            window.location.href = HTML_FILES.TRANSACTION_MANAGEMENT;
            break;
        case 'menu.aggregation':
            window.location.href = HTML_FILES.AGGREGATION;
            break;
        case 'menu.aggregation_daily':
            window.location.href = HTML_FILES.AGGREGATION_DAILY;
            break;
        case 'menu.aggregation_weekly':
            window.location.href = HTML_FILES.AGGREGATION_WEEKLY;
            break;
        case 'menu.aggregation_yearly':
            window.location.href = HTML_FILES.AGGREGATION_YEARLY;
            break;
        case 'menu.aggregation_period':
            window.location.href = HTML_FILES.AGGREGATION_PERIOD;
            break;
        case 'menu.dashboard':
            window.location.href = HTML_FILES.DASHBOARD;
            break;
    }
}

async function setupLanguageMenu() {
    const languageDropdown = document.getElementById('language-dropdown');
    if (!languageDropdown) return;

    try {
        // Fixed language labels (not localized, so users can always understand)
        const languages = [
            { code: 'en', label: 'English' },
            { code: 'ja', label: '日本語' }
        ];

        // Get current language
        const currentLang = i18n.getCurrentLanguage();

        // Clear existing items
        languageDropdown.innerHTML = '';

        // Add language items with individual click handlers
        for (const lang of languages) {
            const item = document.createElement('div');
            item.className = 'dropdown-item';
            item.textContent = lang.label;
            item.dataset.langCode = lang.code;

            // Mark current language with active class
            if (lang.code === currentLang) {
                item.classList.add('active');
            }

            // Add click handler for this language item
            item.addEventListener('click', async function(e) {
                e.stopPropagation();
                console.log('Language selected:', lang.code);
                await changeLanguage(lang.code);
                languageDropdown.classList.remove('show');
            });

            languageDropdown.appendChild(item);
        }
    } catch (error) {
        console.error('Failed to setup language menu:', error);
    }
}

function setupLanguageMenuHandlers() {
    const languageMenu = document.getElementById('language-menu');
    const languageDropdown = document.getElementById('language-dropdown');

    if (!languageMenu || !languageDropdown) {
        console.error('Language menu elements not found');
        return;
    }

    // Remove any existing click handler by cloning the element
    const newLanguageMenu = languageMenu.cloneNode(true);
    languageMenu.parentNode.replaceChild(newLanguageMenu, languageMenu);

    // Re-get references after cloning
    const freshLanguageMenu = document.getElementById('language-menu');
    const freshLanguageDropdown = document.getElementById('language-dropdown');

    // Setup dropdown toggle
    freshLanguageMenu.addEventListener('click', function(e) {
        console.log('Language menu clicked');
        e.stopPropagation();

        const isShown = freshLanguageDropdown.classList.contains('show');

        // Close all other dropdowns first
        document.querySelectorAll('.dropdown').forEach(dropdown => {
            if (dropdown !== freshLanguageDropdown) {
                dropdown.classList.remove('show');
            }
        });

        // Toggle this dropdown
        if (!isShown) {
            freshLanguageDropdown.classList.add('show');
        } else {
            freshLanguageDropdown.classList.remove('show');
        }
    });
}

async function changeLanguage(lang) {
    try {
        await i18n.setLanguage(lang);
        await setupLanguageMenu();

        // Reload dashboard data with new language
        await loadDashboardData();
    } catch (error) {
        console.error('Failed to change language:', error);
    }
}
