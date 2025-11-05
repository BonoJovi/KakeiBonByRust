import { invoke } from '@tauri-apps/api/core';
import { I18n } from './i18n.js';
import { setupMenuHandlers, setupLanguageMenu, setupFontSizeMenu } from './menu.js';

// Initialize i18n
const i18n = new I18n();
let currentUserId = null;

// Pagination state
let currentPage = 1;
const perPage = 50;

// Filter state
let currentFilters = {
    startDate: null,
    endDate: null,
    category1Code: null,
    category2Code: null,
    category3Code: null,
    minAmount: null,
    maxAmount: null,
    keyword: null
};

async function initialize() {
    try {
        // Get current user
        const storedUserId = localStorage.getItem('currentUserId');
        if (!storedUserId) {
            window.location.href = 'index.html';
            return;
        }
        currentUserId = parseInt(storedUserId);

        // Initialize i18n
        await i18n.initialize();
        
        // Setup menu handlers
        setupMenuHandlers();
        await setupLanguageMenu(i18n);
        await setupFontSizeMenu(i18n);
        
        // Setup event listeners
        setupEventListeners();
        
        // Load transactions
        await loadTransactions();
        
    } catch (error) {
        console.error('Failed to initialize:', error);
    }
}

function setupEventListeners() {
    // Toggle filter panel
    const toggleFilterBtn = document.getElementById('toggle-filter-btn');
    const filterPanel = document.getElementById('filter-panel');
    toggleFilterBtn.addEventListener('click', () => {
        filterPanel.classList.toggle('hidden');
    });

    // Apply filter
    const applyFilterBtn = document.getElementById('apply-filter-btn');
    applyFilterBtn.addEventListener('click', applyFilters);

    // Clear filter
    const clearFilterBtn = document.getElementById('clear-filter-btn');
    clearFilterBtn.addEventListener('click', clearFilters);

    // Pagination
    const prevPageBtn = document.getElementById('prev-page-btn');
    const nextPageBtn = document.getElementById('next-page-btn');
    
    prevPageBtn.addEventListener('click', () => {
        if (currentPage > 1) {
            currentPage--;
            loadTransactions();
        }
    });
    
    nextPageBtn.addEventListener('click', () => {
        currentPage++;
        loadTransactions();
    });

    // Add transaction button (placeholder for now)
    const addTransactionBtn = document.getElementById('add-transaction-btn');
    addTransactionBtn.addEventListener('click', () => {
        alert('Add transaction functionality coming soon!');
    });
}

async function loadTransactions() {
    try {
        const listContainer = document.getElementById('transaction-list');
        listContainer.innerHTML = '<div class="loading" data-i18n="common.loading">Loading...</div>';
        i18n.updateUI();

        const response = await invoke('get_transactions', {
            userId: currentUserId,
            startDate: currentFilters.startDate,
            endDate: currentFilters.endDate,
            category1Code: currentFilters.category1Code,
            category2Code: currentFilters.category2Code,
            category3Code: currentFilters.category3Code,
            minAmount: currentFilters.minAmount,
            maxAmount: currentFilters.maxAmount,
            keyword: currentFilters.keyword,
            page: currentPage,
            perPage: perPage
        });

        console.log('Transactions loaded:', response);

        renderTransactions(response.transactions);
        updatePagination(response);

    } catch (error) {
        console.error('Failed to load transactions:', error);
        const listContainer = document.getElementById('transaction-list');
        listContainer.innerHTML = `<div class="error">Failed to load transactions: ${error}</div>`;
    }
}

function renderTransactions(transactions) {
    const listContainer = document.getElementById('transaction-list');
    
    if (!transactions || transactions.length === 0) {
        listContainer.innerHTML = '<div class="empty-state" data-i18n="transaction_mgmt.no_transactions">No transactions found</div>';
        i18n.updateUI();
        return;
    }

    listContainer.innerHTML = '';
    
    transactions.forEach(transaction => {
        const item = createTransactionItem(transaction);
        listContainer.appendChild(item);
    });
}

function createTransactionItem(transaction) {
    const item = document.createElement('div');
    item.className = 'transaction-item';
    
    // Date
    const dateDiv = document.createElement('div');
    dateDiv.className = 'transaction-date';
    dateDiv.textContent = transaction.transaction_date;
    item.appendChild(dateDiv);
    
    // Category
    const categoryDiv = document.createElement('div');
    categoryDiv.className = 'transaction-category';
    
    const majorSpan = document.createElement('span');
    majorSpan.className = 'major';
    majorSpan.textContent = getCategoryLabel(transaction.category1_code);
    categoryDiv.appendChild(majorSpan);
    
    const detailSpan = document.createElement('span');
    detailSpan.className = 'detail';
    detailSpan.textContent = `${transaction.category2_code} > ${transaction.category3_code}`;
    categoryDiv.appendChild(detailSpan);
    
    item.appendChild(categoryDiv);
    
    // Description
    const descriptionDiv = document.createElement('div');
    descriptionDiv.className = 'transaction-description';
    descriptionDiv.textContent = transaction.description || '-';
    item.appendChild(descriptionDiv);
    
    // Amount
    const amountDiv = document.createElement('div');
    amountDiv.className = `transaction-amount ${transaction.category1_code.toLowerCase()}`;
    amountDiv.textContent = formatAmount(transaction.amount);
    item.appendChild(amountDiv);
    
    // Empty column for memo (not displayed in list)
    const emptyDiv = document.createElement('div');
    item.appendChild(emptyDiv);
    
    // Actions
    const actionsDiv = document.createElement('div');
    actionsDiv.className = 'transaction-actions';
    
    const editBtn = document.createElement('button');
    editBtn.className = 'btn btn-secondary btn-icon';
    editBtn.textContent = '✏️';
    editBtn.title = 'Edit';
    editBtn.addEventListener('click', () => editTransaction(transaction.transaction_id));
    actionsDiv.appendChild(editBtn);
    
    const deleteBtn = document.createElement('button');
    deleteBtn.className = 'btn btn-danger btn-icon';
    deleteBtn.textContent = '🗑️';
    deleteBtn.title = 'Delete';
    deleteBtn.addEventListener('click', () => deleteTransaction(transaction.transaction_id));
    actionsDiv.appendChild(deleteBtn);
    
    item.appendChild(actionsDiv);
    
    return item;
}

function getCategoryLabel(category1Code) {
    const labels = {
        'EXPENSE': '支出 / Expense',
        'INCOME': '収入 / Income',
        'TRANSFER': '振替 / Transfer'
    };
    return labels[category1Code] || category1Code;
}

function formatAmount(amount) {
    return '¥' + amount.toLocaleString('ja-JP');
}

function updatePagination(response) {
    document.getElementById('total-count').textContent = response.total_count;
    document.getElementById('current-page').textContent = response.page;
    document.getElementById('total-pages').textContent = response.total_pages;
    
    const prevBtn = document.getElementById('prev-page-btn');
    const nextBtn = document.getElementById('next-page-btn');
    
    prevBtn.disabled = response.page <= 1;
    nextBtn.disabled = response.page >= response.total_pages;
}

function applyFilters() {
    currentFilters.startDate = document.getElementById('filter-start-date').value || null;
    currentFilters.endDate = document.getElementById('filter-end-date').value || null;
    currentFilters.category1Code = document.getElementById('filter-category1').value || null;
    currentFilters.category2Code = document.getElementById('filter-category2').value || null;
    currentFilters.category3Code = document.getElementById('filter-category3').value || null;
    
    const minAmountInput = document.getElementById('filter-min-amount').value;
    const maxAmountInput = document.getElementById('filter-max-amount').value;
    
    currentFilters.minAmount = minAmountInput ? parseInt(minAmountInput) : null;
    currentFilters.maxAmount = maxAmountInput ? parseInt(maxAmountInput) : null;
    currentFilters.keyword = document.getElementById('filter-keyword').value || null;
    
    currentPage = 1; // Reset to first page
    loadTransactions();
}

function clearFilters() {
    document.getElementById('filter-start-date').value = '';
    document.getElementById('filter-end-date').value = '';
    document.getElementById('filter-category1').value = '';
    document.getElementById('filter-category2').value = '';
    document.getElementById('filter-category3').value = '';
    document.getElementById('filter-min-amount').value = '';
    document.getElementById('filter-max-amount').value = '';
    document.getElementById('filter-keyword').value = '';
    
    currentFilters = {
        startDate: null,
        endDate: null,
        category1Code: null,
        category2Code: null,
        category3Code: null,
        minAmount: null,
        maxAmount: null,
        keyword: null
    };
    
    currentPage = 1;
    loadTransactions();
}

async function editTransaction(transactionId) {
    // Placeholder for edit functionality
    alert(`Edit transaction ID: ${transactionId} - Coming soon!`);
}

async function deleteTransaction(transactionId) {
    if (!confirm('Are you sure you want to delete this transaction?')) {
        return;
    }
    
    try {
        await invoke('delete_transaction', {
            userId: currentUserId,
            transactionId: transactionId
        });
        
        // Reload transactions
        await loadTransactions();
        
    } catch (error) {
        console.error('Failed to delete transaction:', error);
        alert('Failed to delete transaction: ' + error);
    }
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', initialize);
