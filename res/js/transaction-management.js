import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';
import { setupLanguageMenuHandlers, setupLanguageMenu, handleLogout, handleQuit } from './menu.js';
import { HTML_FILES } from './html-files.js';

const currentUserId = 1; // TODO: Get from session/auth

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

document.addEventListener('DOMContentLoaded', async function() {
    try {
        // Initialize i18n
        await i18n.init();
        i18n.updateUI();
        
        // Setup menu handlers
        setupMenuHandlers();
        
        // Setup language and font size menus
        await setupLanguageMenu();
        setupLanguageMenuHandlers();
        
        setupFontSizeMenuHandlers();
        await setupFontSizeMenu();
        setupFontSizeModalHandlers();
        await applyFontSize();
        
        // Setup accessibility indicators
        setupIndicators();
        
        // Setup event listeners
        setupEventListeners();
        
        // Load category data for filters
        await loadCategoriesForFilter();
        
        // Load transactions
        await loadTransactions();
        
    } catch (error) {
        console.error('Failed to initialize:', error);
    }
});

function setupMenuHandlers() {
    // File menu handlers
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    if (fileMenu && fileDropdown) {
        fileMenu.addEventListener('click', (e) => {
            e.stopPropagation();
            fileDropdown.classList.toggle('show');
        });
        
        // Back to main
        const backToMainItem = fileDropdown.querySelector('[data-i18n="menu.back_to_main"]');
        if (backToMainItem) {
            backToMainItem.addEventListener('click', () => {
                window.location.href = HTML_FILES.INDEX;
            });
        }
        
        // Logout
        const logoutItem = fileDropdown.querySelector('[data-i18n="menu.logout"]');
        if (logoutItem) {
            logoutItem.addEventListener('click', handleLogout);
        }
        
        // Quit
        const quitItem = fileDropdown.querySelector('[data-i18n="menu.quit"]');
        if (quitItem) {
            quitItem.addEventListener('click', handleQuit);
        }
    }
    
    // Close dropdowns when clicking outside
    document.addEventListener('click', () => {
        document.querySelectorAll('.dropdown').forEach(dropdown => {
            dropdown.classList.remove('show');
        });
    });
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

    // Add transaction button (stub for now)
    const addTransactionBtn = document.getElementById('add-transaction-btn');
    addTransactionBtn.addEventListener('click', () => {
        alert(i18n.t('transaction_mgmt.coming_soon'));
    });
}

async function loadCategoriesForFilter() {
    try {
        // Get category tree
        const categoryTree = await invoke('get_category_tree_with_lang', {
            userId: currentUserId,
            langCode: i18n.currentLanguage
        });
        
        // Populate Category1 dropdown
        const category1Select = document.getElementById('filter-category1');
        if (!category1Select) {
            console.error('Category1 select element not found');
            return;
        }
        
        category1Select.innerHTML = '<option value="">' + i18n.t('common.all') + '</option>';
        
        categoryTree.forEach((cat1) => {
            const option = document.createElement('option');
            option.value = cat1.category1.category1_code;
            option.textContent = cat1.category1.category1_name_i18n;
            category1Select.appendChild(option);
        });
        
        // Handle Category1 change to populate Category2
        category1Select.addEventListener('change', () => {
            const selectedCat1 = category1Select.value;
            const category2Select = document.getElementById('filter-category2');
            const category3Select = document.getElementById('filter-category3');
            
            // Reset Category2 and Category3
            category2Select.innerHTML = '<option value="">' + i18n.t('common.all') + '</option>';
            category3Select.innerHTML = '<option value="">' + i18n.t('common.all') + '</option>';
            
            if (selectedCat1) {
                const cat1Data = categoryTree.find(c => c.category1.category1_code === selectedCat1);
                if (cat1Data && cat1Data.children) {
                    cat1Data.children.forEach(cat2 => {
                        const option = document.createElement('option');
                        option.value = cat2.category2.category2_code;
                        option.textContent = cat2.category2.category2_name_i18n;
                        category2Select.appendChild(option);
                    });
                }
            }
        });
        
        // Handle Category2 change to populate Category3
        const category2Select = document.getElementById('filter-category2');
        category2Select.addEventListener('change', () => {
            const selectedCat1 = category1Select.value;
            const selectedCat2 = category2Select.value;
            const category3Select = document.getElementById('filter-category3');
            
            // Reset Category3
            category3Select.innerHTML = '<option value="">' + i18n.t('common.all') + '</option>';
            
            if (selectedCat1 && selectedCat2) {
                const cat1Data = categoryTree.find(c => c.category1.category1_code === selectedCat1);
                if (cat1Data && cat1Data.children) {
                    const cat2Data = cat1Data.children.find(c => c.category2.category2_code === selectedCat2);
                    if (cat2Data && cat2Data.children) {
                        cat2Data.children.forEach(cat3 => {
                            const option = document.createElement('option');
                            option.value = cat3.category3_code;
                            option.textContent = cat3.category3_name_i18n;
                            category3Select.appendChild(option);
                        });
                    }
                }
            }
        });
        
        console.log('[loadCategoriesForFilter] Event listeners attached successfully');
        console.log('[loadCategoriesForFilter] END - SUCCESS');
        
    } catch (error) {
        console.error('Failed to load categories:', error);
    }
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
    majorSpan.textContent = transaction.category1_name || transaction.category1_code;
    categoryDiv.appendChild(majorSpan);
    
    const detailSpan = document.createElement('span');
    detailSpan.className = 'detail';
    const category2Name = transaction.category2_name || transaction.category2_code;
    const category3Name = transaction.category3_name || transaction.category3_code;
    detailSpan.textContent = `${category2Name} > ${category3Name}`;
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

async function applyFilters() {
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
    
    // Close filter panel
    const filterPanel = document.getElementById('filter-panel');
    filterPanel.classList.add('hidden');
    
    await loadTransactions();
}

async function clearFilters() {
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
    await loadTransactions();
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
