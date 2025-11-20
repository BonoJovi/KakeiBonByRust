/**
 * Common functions for all aggregation screens
 */
import i18n from './i18n.js';

/**
 * Render aggregation results to table
 * @param {Array} results - Aggregation results
 * @param {HTMLElement} tbody - Table body element
 * @param {HTMLElement} tfoot - Table footer element
 */
export function renderResults(results, tbody, tfoot) {
    // Clear existing rows
    tbody.innerHTML = '';
    tfoot.innerHTML = '';
    
    if (!results || results.length === 0) {
        tbody.innerHTML = `
            <tr>
                <td colspan="4" class="empty-state" data-i18n="aggregation.no_results">
                    No results found
                </td>
            </tr>
        `;
        i18n.updateUI();
        return;
    }
    
    let totalAmount = 0;
    let totalCount = 0;
    
    // Render each result row
    results.forEach(result => {
        const row = document.createElement('tr');
        
        // Group name
        const nameCell = document.createElement('td');
        nameCell.textContent = result.group_name;
        row.appendChild(nameCell);
        
        // Total amount
        const amountCell = document.createElement('td');
        amountCell.className = 'amount';
        if (result.total_amount >= 0) {
            amountCell.classList.add('amount-positive');
            amountCell.textContent = `+${formatAmount(result.total_amount)}`;
        } else {
            amountCell.classList.add('amount-negative');
            amountCell.textContent = formatAmount(result.total_amount);
        }
        row.appendChild(amountCell);
        
        // Count
        const countCell = document.createElement('td');
        countCell.textContent = result.count;
        row.appendChild(countCell);
        
        // Average amount
        const avgCell = document.createElement('td');
        avgCell.className = 'amount';
        avgCell.textContent = formatAmount(result.avg_amount);
        row.appendChild(avgCell);
        
        tbody.appendChild(row);
        
        totalAmount += result.total_amount;
        totalCount += result.count;
    });
    
    // Render total row
    const avgAmount = totalCount > 0 ? Math.round(totalAmount / totalCount) : 0;
    
    const totalRow = document.createElement('tr');
    totalRow.innerHTML = `
        <td data-i18n="aggregation.total">Total</td>
        <td class="amount ${totalAmount >= 0 ? 'amount-positive' : 'amount-negative'}">
            ${totalAmount >= 0 ? '+' : ''}${formatAmount(totalAmount)}
        </td>
        <td>${totalCount}</td>
        <td class="amount">${formatAmount(avgAmount)}</td>
    `;
    tfoot.appendChild(totalRow);
    
    i18n.updateUI();
}

/**
 * Format amount with thousand separators
 * @param {number} amount - Amount to format
 * @returns {string} Formatted amount
 */
export function formatAmount(amount) {
    return amount.toLocaleString('ja-JP');
}

/**
 * Show error message
 * @param {string} message - Error message
 * @param {HTMLElement} messageElement - Message element
 */
export function showError(message, messageElement) {
    messageElement.textContent = message;
    messageElement.className = 'message error';
    messageElement.style.display = 'block';
}

/**
 * Show success message
 * @param {string} message - Success message
 * @param {HTMLElement} messageElement - Message element
 */
export function showSuccess(message, messageElement) {
    messageElement.textContent = message;
    messageElement.className = 'message success';
    messageElement.style.display = 'block';
}

/**
 * Clear message
 * @param {HTMLElement} messageElement - Message element
 */
export function clearMessage(messageElement) {
    messageElement.textContent = '';
    messageElement.style.display = 'none';
}

/**
 * Get current year
 * @returns {number} Current year
 */
export function getCurrentYear() {
    return new Date().getFullYear();
}

/**
 * Get current month (1-12)
 * @returns {number} Current month
 */
export function getCurrentMonth() {
    return new Date().getMonth() + 1;
}

/**
 * Get current date
 * @returns {Date} Current date
 */
export function getCurrentDate() {
    return new Date();
}

/**
 * Format date as YYYY-MM-DD
 * @param {Date} date - Date to format
 * @returns {string} Formatted date
 */
export function formatDate(date) {
    const year = date.getFullYear();
    const month = String(date.getMonth() + 1).padStart(2, '0');
    const day = String(date.getDate()).padStart(2, '0');
    return `${year}-${month}-${day}`;
}

/**
 * Get current week number (ISO 8601)
 * @param {Date} date - Date
 * @returns {number} Week number (1-53)
 */
export function getWeekNumber(date) {
    const d = new Date(Date.UTC(date.getFullYear(), date.getMonth(), date.getDate()));
    const dayNum = d.getUTCDay() || 7;
    d.setUTCDate(d.getUTCDate() + 4 - dayNum);
    const yearStart = new Date(Date.UTC(d.getUTCFullYear(), 0, 1));
    return Math.ceil((((d - yearStart) / 86400000) + 1) / 7);
}

/**
 * Parse group_by value
 * @param {string} groupBy - Group by value
 * @returns {string} Parsed value
 */
export function parseGroupBy(groupBy) {
    return groupBy;
}
