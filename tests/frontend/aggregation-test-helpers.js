/**
 * Common test helpers for aggregation screens
 */

/**
 * Mock Tauri invoke for testing
 */
export function createMockInvoke() {
    return async function mockInvoke(command, args) {
        console.log(`Mock invoke: ${command}`, args);
        
        // Return mock aggregation results
        if (command.includes('aggregation')) {
            return [
                {
                    group_name: 'Food',
                    total_amount: 50000,
                    count: 10,
                    avg_amount: 5000
                },
                {
                    group_name: 'Transportation',
                    total_amount: -20000,
                    count: 5,
                    avg_amount: -4000
                }
            ];
        }
        
        throw new Error(`Unknown command: ${command}`);
    };
}

/**
 * Mock session user
 */
export function createMockSessionUser() {
    return {
        user_id: 1,
        name: 'Test User',
        role: 1
    };
}

/**
 * Create mock i18n
 */
export function createMockI18n() {
    const translations = {
        'aggregation.title': 'Aggregation',
        'aggregation.title_daily': 'Daily Aggregation',
        'aggregation.title_weekly': 'Weekly Aggregation',
        'aggregation.title_yearly': 'Yearly Aggregation',
        'aggregation.filter': 'Filter',
        'aggregation.execute': 'Execute',
        'aggregation.results': 'Results',
        'aggregation.no_results': 'No results found',
        'aggregation.items': 'items',
        'aggregation.total': 'Total',
        'aggregation.year': 'Year',
        'aggregation.month': 'Month',
        'aggregation.date': 'Date',
        'aggregation.week': 'Week',
        'aggregation.week_start': 'Week Start',
        'aggregation.reference_date': 'Reference Date',
        'aggregation.year_start': 'Year Start',
        'aggregation.group_by': 'Group By',
        'aggregation.category1': 'Category 1',
        'aggregation.category2': 'Category 2',
        'aggregation.category3': 'Category 3',
        'aggregation.account': 'Account',
        'aggregation.shop': 'Shop',
        'aggregation.sunday': 'Sunday',
        'aggregation.monday': 'Monday',
        'aggregation.january': 'January (Calendar Year)',
        'aggregation.april': 'April (Fiscal Year)',
        'aggregation.error_invalid_year': 'Invalid year',
        'aggregation.error_invalid_month': 'Invalid month',
        'aggregation.error_no_date': 'Please select a date',
        'aggregation.error_invalid_week': 'Invalid week',
        'aggregation.account_note': '※ Account aggregation includes transfers'
    };
    
    return {
        t: (key) => translations[key] || key,
        updateUI: () => {},
        init: async () => {}
    };
}

/**
 * Wait for async operations
 */
export function waitFor(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Simulate user input
 */
export function setInputValue(selector, value) {
    const element = document.querySelector(selector);
    if (!element) {
        throw new Error(`Element not found: ${selector}`);
    }
    element.value = value;
    element.dispatchEvent(new Event('input', { bubbles: true }));
    element.dispatchEvent(new Event('change', { bubbles: true }));
}

/**
 * Simulate button click
 */
export function clickButton(selector) {
    const element = document.querySelector(selector);
    if (!element) {
        throw new Error(`Element not found: ${selector}`);
    }
    element.click();
}

/**
 * Get table data
 */
export function getTableData(tableSelector) {
    const tbody = document.querySelector(`${tableSelector} tbody`);
    if (!tbody) return [];
    
    const rows = Array.from(tbody.querySelectorAll('tr'));
    return rows.map(row => {
        const cells = Array.from(row.querySelectorAll('td'));
        return cells.map(cell => cell.textContent.trim());
    });
}

/**
 * Check if element exists
 */
export function elementExists(selector) {
    return document.querySelector(selector) !== null;
}

/**
 * Check if element is visible
 */
export function isVisible(selector) {
    const element = document.querySelector(selector);
    if (!element) return false;
    
    const style = window.getComputedStyle(element);
    return style.display !== 'none' && style.visibility !== 'hidden';
}

/**
 * Setup monthly aggregation DOM
 */
export function setupMonthlyAggregationDOM() {
    const currentDate = new Date();
    const currentYear = currentDate.getFullYear();
    const currentMonth = currentDate.getMonth() + 1;
    
    document.body.innerHTML = `
        <div class="aggregation-container">
            <h1>Aggregation</h1>
            <div class="section filter-section">
                <div class="section-header collapsible-header" id="filter-header">
                    <h2>Filter</h2>
                    <button id="toggle-filter-btn" class="toggle-btn">▲</button>
                </div>
                <div class="filter-form" id="filter-form-content">
                    <div class="filter-row">
                        <div class="form-group">
                            <label for="year">Year:</label>
                            <div class="input-with-spinner">
                                <input type="number" id="year" name="year" min="1900" max="2100" value="${currentYear}" />
                                <div class="spinner-buttons">
                                    <button type="button" class="spinner-btn spinner-down" id="year-down">◀</button>
                                    <button type="button" class="spinner-btn spinner-up" id="year-up">▶</button>
                                </div>
                            </div>
                        </div>
                        <div class="form-group">
                            <label for="month">Month:</label>
                            <select id="month" name="month">
                                ${Array.from({length: 12}, (_, i) => i + 1).map(m => 
                                    `<option value="${m}" ${m === currentMonth ? 'selected' : ''}>${m}</option>`
                                ).join('')}
                            </select>
                        </div>
                        <div class="form-group">
                            <label for="group-by">Group By:</label>
                            <select id="group-by" name="group-by">
                                <option value="category1" selected>Category 1</option>
                                <option value="category2">Category 2</option>
                                <option value="category3">Category 3</option>
                                <option value="account">Account</option>
                                <option value="shop">Shop</option>
                            </select>
                        </div>
                    </div>
                    <div class="filter-actions">
                        <button id="execute-btn" class="btn-primary">Execute</button>
                    </div>
                </div>
            </div>
            <div id="results-section" style="display:none;">
                <div id="result-count"></div>
                <table id="results-table">
                    <thead><tr><th>Group</th><th>Amount</th><th>Count</th></tr></thead>
                    <tbody></tbody>
                </table>
            </div>
            <div id="account-note" style="display:none;">※ Account aggregation includes transfers</div>
            <div id="loading-indicator" style="display:none;">Loading...</div>
            <div id="error-message" class="message error" style="display:none;"></div>
        </div>
    `;
    
    // Add event listeners for testing
    setupAggregationEventListeners();
}

/**
 * Setup daily aggregation DOM
 */
export function setupDailyAggregationDOM() {
    const today = new Date().toISOString().split('T')[0];
    
    document.body.innerHTML = `
        <div class="aggregation-container">
            <h1>Daily Aggregation</h1>
            <div class="section filter-section">
                <div class="section-header collapsible-header" id="filter-header">
                    <h2>Filter</h2>
                    <button id="toggle-filter-btn" class="toggle-btn">▲</button>
                </div>
                <div class="filter-form" id="filter-form-content">
                    <div class="filter-row">
                        <div class="form-group">
                            <label for="date">Date:</label>
                            <input type="date" id="date" name="date" value="${today}" />
                        </div>
                        <div class="form-group">
                            <label for="group-by">Group By:</label>
                            <select id="group-by" name="group-by">
                                <option value="category1" selected>Category 1</option>
                                <option value="category2">Category 2</option>
                                <option value="category3">Category 3</option>
                                <option value="account">Account</option>
                                <option value="shop">Shop</option>
                            </select>
                        </div>
                    </div>
                    <div class="filter-actions">
                        <button id="execute-btn" class="btn-primary">Execute</button>
                    </div>
                </div>
            </div>
            <div id="results-section" style="display:none;">
                <div id="result-count"></div>
                <table id="results-table">
                    <thead><tr><th>Group</th><th>Amount</th><th>Count</th></tr></thead>
                    <tbody></tbody>
                </table>
            </div>
            <div id="account-note" style="display:none;">※ Account aggregation includes transfers</div>
            <div id="loading-indicator" style="display:none;">Loading...</div>
            <div id="error-message" class="message error" style="display:none;"></div>
        </div>
    `;
    
    setupDailyEventListeners();
}

/**
 * Setup weekly aggregation DOM
 */
export function setupWeeklyAggregationDOM() {
    const today = new Date().toISOString().split('T')[0];
    
    document.body.innerHTML = `
        <div class="aggregation-container">
            <h1>Weekly Aggregation</h1>
            <div class="section filter-section">
                <div class="section-header collapsible-header" id="filter-header">
                    <h2>Filter</h2>
                    <button id="toggle-filter-btn" class="toggle-btn">▲</button>
                </div>
                <div class="filter-form" id="filter-form-content">
                    <div class="filter-row">
                        <div class="form-group">
                            <label for="reference-date">Reference Date:</label>
                            <input type="date" id="reference-date" name="reference-date" value="${today}" />
                        </div>
                        <div class="form-group">
                            <label for="week-start">Week Start:</label>
                            <select id="week-start" name="week-start">
                                <option value="0" selected>Sunday</option>
                                <option value="1">Monday</option>
                            </select>
                        </div>
                        <div class="form-group">
                            <label for="group-by">Group By:</label>
                            <select id="group-by" name="group-by">
                                <option value="category1" selected>Category 1</option>
                                <option value="category2">Category 2</option>
                                <option value="category3">Category 3</option>
                                <option value="account">Account</option>
                                <option value="shop">Shop</option>
                            </select>
                        </div>
                    </div>
                    <div class="filter-actions">
                        <button id="execute-btn" class="btn-primary">Execute</button>
                    </div>
                </div>
            </div>
            <div id="results-section" style="display:none;">
                <div id="result-count"></div>
                <table id="results-table">
                    <thead><tr><th>Group</th><th>Amount</th><th>Count</th></tr></thead>
                    <tbody></tbody>
                </table>
            </div>
            <div id="account-note" style="display:none;">※ Account aggregation includes transfers</div>
            <div id="loading-indicator" style="display:none;">Loading...</div>
            <div id="error-message" class="message error" style="display:none;"></div>
        </div>
    `;
    
    setupWeeklyEventListeners();
}

/**
 * Setup yearly aggregation DOM
 */
export function setupYearlyAggregationDOM() {
    const currentYear = new Date().getFullYear();
    
    document.body.innerHTML = `
        <div class="aggregation-container">
            <h1>Yearly Aggregation</h1>
            <div class="section filter-section">
                <div class="section-header collapsible-header" id="filter-header">
                    <h2>Filter</h2>
                    <button id="toggle-filter-btn" class="toggle-btn">▲</button>
                </div>
                <div class="filter-form" id="filter-form-content">
                    <div class="filter-row">
                        <div class="form-group">
                            <label for="year">Year:</label>
                            <div class="input-with-spinner">
                                <input type="number" id="year" name="year" min="1900" max="2100" value="${currentYear}" />
                                <div class="spinner-buttons">
                                    <button type="button" class="spinner-btn spinner-down" id="year-down">◀</button>
                                    <button type="button" class="spinner-btn spinner-up" id="year-up">▶</button>
                                </div>
                            </div>
                        </div>
                        <div class="form-group">
                            <label for="year-start">Year Start:</label>
                            <select id="year-start" name="year-start">
                                <option value="1" selected>January (Calendar Year)</option>
                                <option value="4">April (Fiscal Year)</option>
                            </select>
                        </div>
                        <div class="form-group">
                            <label for="group-by">Group By:</label>
                            <select id="group-by" name="group-by">
                                <option value="category1" selected>Category 1</option>
                                <option value="category2">Category 2</option>
                                <option value="category3">Category 3</option>
                                <option value="account">Account</option>
                                <option value="shop">Shop</option>
                            </select>
                        </div>
                    </div>
                    <div class="filter-actions">
                        <button id="execute-btn" class="btn-primary">Execute</button>
                    </div>
                </div>
            </div>
            <div id="results-section" style="display:none;">
                <div id="result-count"></div>
                <table id="results-table">
                    <thead><tr><th>Group</th><th>Amount</th><th>Count</th></tr></thead>
                    <tbody></tbody>
                </table>
            </div>
            <div id="account-note" style="display:none;">※ Account aggregation includes transfers</div>
            <div id="loading-indicator" style="display:none;">Loading...</div>
            <div id="error-message" class="message error" style="display:none;"></div>
        </div>
    `;
    
    setupYearlyEventListeners();
}

/**
 * Setup period aggregation DOM
 */
export function setupPeriodAggregationDOM() {
    const today = new Date();
    const todayStr = today.toISOString().split('T')[0];
    const firstDayOfMonth = new Date(today.getFullYear(), today.getMonth(), 1);
    const firstDayStr = firstDayOfMonth.toISOString().split('T')[0];
    
    document.body.innerHTML = `
        <div class="aggregation-container">
            <h1>Period Aggregation</h1>
            <div class="section filter-section">
                <div class="section-header collapsible-header" id="filter-header">
                    <h2>Filter</h2>
                    <button id="toggle-filter-btn" class="toggle-btn">▲</button>
                </div>
                <div class="filter-form" id="filter-form-content">
                    <div class="filter-row">
                        <div class="form-group">
                            <label for="start-date">Start Date:</label>
                            <input type="date" id="start-date" name="start-date" value="${firstDayStr}" />
                        </div>
                        <div class="form-group">
                            <label for="end-date">End Date:</label>
                            <input type="date" id="end-date" name="end-date" value="${todayStr}" />
                        </div>
                        <div class="form-group">
                            <label for="group-by">Group By:</label>
                            <select id="group-by" name="group-by">
                                <option value="category1" selected>Category 1</option>
                                <option value="category2">Category 2</option>
                                <option value="category3">Category 3</option>
                                <option value="account">Account</option>
                                <option value="shop">Shop</option>
                            </select>
                        </div>
                    </div>
                    <div class="filter-actions">
                        <button id="execute-btn" class="btn-primary">Execute</button>
                    </div>
                </div>
            </div>
            <div id="results-section" style="display:none;">
                <div id="result-count"></div>
                <table id="results-table">
                    <thead><tr><th>Group</th><th>Amount</th><th>Count</th></tr></thead>
                    <tbody></tbody>
                </table>
            </div>
            <div id="account-note" style="display:none;">※ Account aggregation includes transfers</div>
            <div id="loading-indicator" style="display:none;">Loading...</div>
            <div id="error-message" class="message error" style="display:none;"></div>
        </div>
    `;
    
    setupPeriodEventListeners();
}

/**
 * Setup event listeners for aggregation functionality
 */
function setupAggregationEventListeners() {
    const executeBtn = document.querySelector('#execute-btn');
    const yearInput = document.querySelector('#year');
    const monthSelect = document.querySelector('#month');
    const groupBySelect = document.querySelector('#group-by');
    const filterHeader = document.querySelector('#filter-header');
    const toggleBtn = document.querySelector('#toggle-filter-btn');
    
    // Execute button handler
    if (executeBtn) {
        executeBtn.addEventListener('click', async () => {
            const year = parseInt(yearInput.value);
            const month = parseInt(monthSelect.value);
            const groupBy = groupBySelect.value;
            
            // Validation
            const errorMsg = document.querySelector('#error-message');
            if (year < 1900 || year > 2100) {
                errorMsg.textContent = 'Invalid year';
                errorMsg.style.display = 'block';
                return;
            }
            
            // Check for future date
            const now = new Date();
            const inputDate = new Date(year, month - 1);
            if (inputDate > now) {
                errorMsg.textContent = 'Cannot aggregate future dates';
                errorMsg.style.display = 'block';
                return;
            }
            
            errorMsg.style.display = 'none';
            
            // Show loading
            const loadingIndicator = document.querySelector('#loading-indicator');
            if (loadingIndicator) {
                loadingIndicator.style.display = 'block';
            }
            
            try {
                // Call backend
                const results = await window.__TAURI__?.core?.invoke('get_monthly_aggregation', {
                    year,
                    month,
                    groupBy
                });
                
                // Hide loading
                if (loadingIndicator) {
                    loadingIndicator.style.display = 'none';
                }
                
                // Display results
                displayResults(results, groupBy);
            } catch (error) {
                if (loadingIndicator) {
                    loadingIndicator.style.display = 'none';
                }
                errorMsg.textContent = error.message || 'An error occurred';
                errorMsg.style.display = 'block';
            }
        });
    }
    
    // Group-by change handler (show/hide account note)
    if (groupBySelect) {
        groupBySelect.addEventListener('change', () => {
            const accountNote = document.querySelector('#account-note');
            if (accountNote) {
                accountNote.style.display = groupBySelect.value === 'account' ? 'block' : 'none';
            }
        });
    }
    
    // Filter toggle handlers
    if (filterHeader) {
        filterHeader.addEventListener('click', toggleFilterSection);
    }
    if (toggleBtn) {
        toggleBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            toggleFilterSection();
        });
    }
}

/**
 * Toggle filter section visibility
 */
function toggleFilterSection() {
    const filterContent = document.querySelector('#filter-form-content');
    const toggleBtn = document.querySelector('#toggle-filter-btn');
    
    if (filterContent && toggleBtn) {
        const isVisible = filterContent.style.display !== 'none';
        filterContent.style.display = isVisible ? 'none' : 'block';
        toggleBtn.textContent = isVisible ? '▼' : '▲';
    }
}

/**
 * Display aggregation results
 */
function displayResults(results, groupBy) {
    const resultsSection = document.querySelector('#results-section');
    const resultCount = document.querySelector('#result-count');
    const tbody = document.querySelector('#results-table tbody');
    
    if (!resultsSection || !tbody) return;
    
    // Clear previous results
    tbody.innerHTML = '';
    
    if (!results || results.length === 0) {
        resultCount.textContent = 'No results found';
        resultsSection.style.display = 'block';
        return;
    }
    
    // Update result count
    resultCount.textContent = `${results.length} items`;
    
    // Add rows
    results.forEach(row => {
        const tr = document.createElement('tr');
        tr.innerHTML = `
            <td>${row.group_name || ''}</td>
            <td>${formatAmount(row.total_amount || 0)}</td>
            <td>${row.count || 0}</td>
        `;
        tbody.appendChild(tr);
    });
    
    resultsSection.style.display = 'block';
}

/**
 * Format amount with thousand separators
 */
function formatAmount(amount) {
    return amount.toLocaleString('en-US');
}

/**
 * Setup event listeners for daily aggregation
 */
function setupDailyEventListeners() {
    const executeBtn = document.querySelector('#execute-btn');
    const dateInput = document.querySelector('#date');
    const groupBySelect = document.querySelector('#group-by');
    const filterHeader = document.querySelector('#filter-header');
    const toggleBtn = document.querySelector('#toggle-filter-btn');
    
    if (executeBtn) {
        executeBtn.addEventListener('click', async () => {
            const date = dateInput.value;
            const groupBy = groupBySelect.value;
            
            const errorMsg = document.querySelector('#error-message');
            if (!date) {
                errorMsg.textContent = 'Please select a date';
                errorMsg.style.display = 'block';
                return;
            }
            
            // Check for future date
            const now = new Date();
            const inputDate = new Date(date);
            if (inputDate > now) {
                errorMsg.textContent = 'Cannot aggregate future dates';
                errorMsg.style.display = 'block';
                return;
            }
            
            errorMsg.style.display = 'none';
            
            const loadingIndicator = document.querySelector('#loading-indicator');
            if (loadingIndicator) loadingIndicator.style.display = 'block';
            
            try {
                const results = await window.__TAURI__?.core?.invoke('get_daily_aggregation', {
                    date,
                    groupBy
                });
                
                if (loadingIndicator) loadingIndicator.style.display = 'none';
                displayResults(results, groupBy);
            } catch (error) {
                if (loadingIndicator) loadingIndicator.style.display = 'none';
                errorMsg.textContent = error.message || 'An error occurred';
                errorMsg.style.display = 'block';
            }
        });
    }
    
    // Add Enter key listener for date input
    if (dateInput) {
        dateInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                executeBtn?.click();
            }
        });
    }
    
    if (groupBySelect) {
        groupBySelect.addEventListener('change', () => {
            const accountNote = document.querySelector('#account-note');
            if (accountNote) {
                accountNote.style.display = groupBySelect.value === 'account' ? 'block' : 'none';
            }
        });
    }
    
    if (filterHeader) filterHeader.addEventListener('click', toggleFilterSection);
    if (toggleBtn) toggleBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        toggleFilterSection();
    });
}

/**
 * Setup event listeners for weekly aggregation
 */
function setupWeeklyEventListeners() {
    const executeBtn = document.querySelector('#execute-btn');
    const referenceDateInput = document.querySelector('#reference-date');
    const weekStartSelect = document.querySelector('#week-start');
    const groupBySelect = document.querySelector('#group-by');
    const filterHeader = document.querySelector('#filter-header');
    const toggleBtn = document.querySelector('#toggle-filter-btn');
    
    if (executeBtn) {
        executeBtn.addEventListener('click', async () => {
            const referenceDate = referenceDateInput.value;
            const weekStart = parseInt(weekStartSelect.value);
            const groupBy = groupBySelect.value;
            
            const errorMsg = document.querySelector('#error-message');
            if (!referenceDate) {
                errorMsg.textContent = 'Please select a reference date';
                errorMsg.style.display = 'block';
                return;
            }
            
            // Check for future date
            const now = new Date();
            const inputDate = new Date(referenceDate);
            if (inputDate > now) {
                errorMsg.textContent = 'Cannot aggregate future dates';
                errorMsg.style.display = 'block';
                return;
            }
            
            errorMsg.style.display = 'none';
            
            const loadingIndicator = document.querySelector('#loading-indicator');
            if (loadingIndicator) loadingIndicator.style.display = 'block';
            
            try {
                const results = await window.__TAURI__?.core?.invoke('get_weekly_aggregation', {
                    referenceDate,
                    weekStart,
                    groupBy
                });
                
                if (loadingIndicator) loadingIndicator.style.display = 'none';
                displayResults(results, groupBy);
            } catch (error) {
                if (loadingIndicator) loadingIndicator.style.display = 'none';
                errorMsg.textContent = error.message || 'An error occurred';
                errorMsg.style.display = 'block';
            }
        });
    }
    
    // Add Enter key listener for reference date input
    if (referenceDateInput) {
        referenceDateInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                executeBtn?.click();
            }
        });
    }
    
    if (groupBySelect) {
        groupBySelect.addEventListener('change', () => {
            const accountNote = document.querySelector('#account-note');
            if (accountNote) {
                accountNote.style.display = groupBySelect.value === 'account' ? 'block' : 'none';
            }
        });
    }
    
    if (filterHeader) filterHeader.addEventListener('click', toggleFilterSection);
    if (toggleBtn) toggleBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        toggleFilterSection();
    });
}

/**
 * Setup event listeners for yearly aggregation
 */
function setupYearlyEventListeners() {
    const executeBtn = document.querySelector('#execute-btn');
    const yearInput = document.querySelector('#year');
    const yearStartSelect = document.querySelector('#year-start');
    const groupBySelect = document.querySelector('#group-by');
    const filterHeader = document.querySelector('#filter-header');
    const toggleBtn = document.querySelector('#toggle-filter-btn');
    const yearUpBtn = document.querySelector('#year-up');
    const yearDownBtn = document.querySelector('#year-down');
    
    if (executeBtn) {
        executeBtn.addEventListener('click', async () => {
            const year = parseInt(yearInput.value);
            const yearStart = parseInt(yearStartSelect.value);
            const groupBy = groupBySelect.value;
            
            const errorMsg = document.querySelector('#error-message');
            if (year < 1900 || year > 2100) {
                errorMsg.textContent = 'Invalid year';
                errorMsg.style.display = 'block';
                return;
            }
            
            // Check for future date
            const now = new Date();
            const inputDate = new Date(year, yearStart - 1);
            if (inputDate > now) {
                errorMsg.textContent = 'Cannot aggregate future dates';
                errorMsg.style.display = 'block';
                return;
            }
            
            errorMsg.style.display = 'none';
            
            const loadingIndicator = document.querySelector('#loading-indicator');
            if (loadingIndicator) loadingIndicator.style.display = 'block';
            
            try {
                const results = await window.__TAURI__?.core?.invoke('get_yearly_aggregation', {
                    year,
                    yearStartMonth: yearStart,
                    groupBy
                });
                
                if (loadingIndicator) loadingIndicator.style.display = 'none';
                displayResults(results, groupBy);
            } catch (error) {
                if (loadingIndicator) loadingIndicator.style.display = 'none';
                errorMsg.textContent = error.message || 'An error occurred';
                errorMsg.style.display = 'block';
            }
        });
    }
    
    // Add spinner button listeners
    if (yearUpBtn) {
        yearUpBtn.addEventListener('click', () => {
            const current = parseInt(yearInput.value);
            if (current < 2100) {
                yearInput.value = current + 1;
            }
        });
    }
    
    if (yearDownBtn) {
        yearDownBtn.addEventListener('click', () => {
            const current = parseInt(yearInput.value);
            if (current > 1900) {
                yearInput.value = current - 1;
            }
        });
    }
    
    // Add Enter key listener for year input
    if (yearInput) {
        yearInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                executeBtn?.click();
            }
        });
    }
    
    if (groupBySelect) {
        groupBySelect.addEventListener('change', () => {
            const accountNote = document.querySelector('#account-note');
            if (accountNote) {
                accountNote.style.display = groupBySelect.value === 'account' ? 'block' : 'none';
            }
        });
    }
    
    if (filterHeader) filterHeader.addEventListener('click', toggleFilterSection);
    if (toggleBtn) toggleBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        toggleFilterSection();
    });
}

/**
 * Setup event listeners for period aggregation
 */
function setupPeriodEventListeners() {
    const executeBtn = document.querySelector('#execute-btn');
    const startDateInput = document.querySelector('#start-date');
    const endDateInput = document.querySelector('#end-date');
    const groupBySelect = document.querySelector('#group-by');
    const filterHeader = document.querySelector('#filter-header');
    const toggleBtn = document.querySelector('#toggle-filter-btn');
    
    if (executeBtn) {
        executeBtn.addEventListener('click', async () => {
            const startDate = startDateInput.value;
            const endDate = endDateInput.value;
            const groupBy = groupBySelect.value;
            
            const errorMsg = document.querySelector('#error-message');
            if (!startDate || !endDate) {
                errorMsg.textContent = 'Please select start and end dates';
                errorMsg.style.display = 'block';
                return;
            }
            
            if (new Date(startDate) > new Date(endDate)) {
                errorMsg.textContent = 'Start date must be before end date';
                errorMsg.style.display = 'block';
                return;
            }
            
            // Check for future date
            const now = new Date();
            const inputEndDate = new Date(endDate);
            if (inputEndDate > now) {
                errorMsg.textContent = 'Cannot aggregate future dates';
                errorMsg.style.display = 'block';
                return;
            }
            
            errorMsg.style.display = 'none';
            
            const loadingIndicator = document.querySelector('#loading-indicator');
            if (loadingIndicator) loadingIndicator.style.display = 'block';
            
            try {
                const results = await window.__TAURI__?.core?.invoke('get_period_aggregation', {
                    startDate,
                    endDate,
                    groupBy
                });
                
                if (loadingIndicator) loadingIndicator.style.display = 'none';
                displayResults(results, groupBy);
            } catch (error) {
                if (loadingIndicator) loadingIndicator.style.display = 'none';
                errorMsg.textContent = error.message || 'An error occurred';
                errorMsg.style.display = 'block';
            }
        });
    }
    
    // Add Enter key listeners for date inputs
    if (startDateInput) {
        startDateInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                executeBtn?.click();
            }
        });
    }
    
    if (endDateInput) {
        endDateInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                executeBtn?.click();
            }
        });
    }
    
    if (groupBySelect) {
        groupBySelect.addEventListener('change', () => {
            const accountNote = document.querySelector('#account-note');
            if (accountNote) {
                accountNote.style.display = groupBySelect.value === 'account' ? 'block' : 'none';
            }
        });
    }
    
    if (filterHeader) filterHeader.addEventListener('click', toggleFilterSection);
    if (toggleBtn) toggleBtn.addEventListener('click', (e) => {
        e.stopPropagation();
        toggleFilterSection();
    });
}
