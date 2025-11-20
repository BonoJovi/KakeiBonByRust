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
