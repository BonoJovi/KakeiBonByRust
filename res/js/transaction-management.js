import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { setupLanguageMenuHandlers, setupLanguageMenu, handleLogout, handleQuit } from './menu.js';
import { HTML_FILES } from './html-files.js';
import { TAX_ROUND_DOWN, TAX_ROUND_HALF_UP, TAX_ROUND_UP, ROLE_ADMIN, ROLE_USER } from './consts.js';
import { Modal } from './modal.js';
import { getCurrentSessionUser, isSessionAuthenticated, setSessionSourceScreen, getSessionModalState, setSessionModalState, clearSessionModalState } from './session.js';
import { createMenuBar } from './menu.js';

let currentUserId = null;
let currentUserRole = null;

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
    
    // Create menu bar
    createMenuBar('management');
    try {
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
        
        currentUserId = user.user_id;
        currentUserRole = user.role;
        console.log(`Logged in as: ${user.name} (ID: ${currentUserId}, Role: ${currentUserRole})`);
        
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
        
        // Check if we need to restore modal state
        await restoreModalState();
        
        // Adjust window size after content is loaded
        await adjustWindowSize();
        
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
    // Toggle filter panel content
    const toggleFilterContentBtn = document.getElementById('toggle-filter-content-btn');
    const filterPanel = document.getElementById('filter-panel');
    const toggleArrow = toggleFilterContentBtn.querySelector('.toggle-arrow');
    
    toggleFilterContentBtn.addEventListener('click', () => {
        filterPanel.classList.toggle('hidden');
        // Change arrow direction
        toggleArrow.textContent = filterPanel.classList.contains('hidden') ? '▼' : '▲';
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

    // Add transaction button
    const addTransactionBtn = document.getElementById('add-transaction-btn');
    addTransactionBtn.addEventListener('click', openTransactionModal);
    
    // Initialize transaction modal with common Modal class
    initializeTransactionModal();
    
    // Setup spinner buttons for amount range filters
    setupAmountSpinners();
}

function setupAmountSpinners() {
    // Setup spinner for min amount
    const minAmountInput = document.getElementById('filter-min-amount');
    const minSpinnerContainer = minAmountInput.closest('.input-with-spinner');
    const minUpBtn = minSpinnerContainer.querySelector('.spinner-up');
    const minDownBtn = minSpinnerContainer.querySelector('.spinner-down');
    
    minUpBtn.addEventListener('click', () => {
        const currentValue = parseInt(minAmountInput.value) || 0;
        const step = 1000; // Increment by 1000
        minAmountInput.value = currentValue + step;
    });
    
    minDownBtn.addEventListener('click', () => {
        const currentValue = parseInt(minAmountInput.value) || 0;
        const step = 1000; // Decrement by 1000
        const newValue = Math.max(0, currentValue - step);
        minAmountInput.value = newValue;
    });
    
    // Setup spinner for max amount
    const maxAmountInput = document.getElementById('filter-max-amount');
    const maxSpinnerContainer = maxAmountInput.closest('.input-with-spinner');
    const maxUpBtn = maxSpinnerContainer.querySelector('.spinner-up');
    const maxDownBtn = maxSpinnerContainer.querySelector('.spinner-down');
    
    maxUpBtn.addEventListener('click', () => {
        const currentValue = parseInt(maxAmountInput.value) || 0;
        const step = 1000; // Increment by 1000
        maxAmountInput.value = currentValue + step;
    });
    
    maxDownBtn.addEventListener('click', () => {
        const currentValue = parseInt(maxAmountInput.value) || 0;
        const step = 1000; // Decrement by 1000
        const newValue = Math.max(0, currentValue - step);
        maxAmountInput.value = newValue;
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
    
    // Create content wrapper (for non-button content)
    const contentWrapper = document.createElement('div');
    contentWrapper.className = 'transaction-content';
    
    // Date (format: YYYY-MM-DD HH:MM:SS -> YYYY-MM-DD HH:MM)
    const dateDiv = document.createElement('div');
    dateDiv.className = 'transaction-date';
    const dateTime = transaction.transaction_date.substring(0, 16).replace(' ', ' '); // YYYY-MM-DD HH:MM
    dateDiv.textContent = dateTime;
    contentWrapper.appendChild(dateDiv);
    
    // Category (only category1)
    const categoryDiv = document.createElement('div');
    categoryDiv.className = 'transaction-category';
    categoryDiv.textContent = transaction.category1_name || transaction.category1_code;
    contentWrapper.appendChild(categoryDiv);
    
    // Accounts (FROM -> TO) - Display account names, fallback to codes if names not available
    const accountDiv = document.createElement('div');
    accountDiv.className = 'transaction-account';
    const fromAccountDisplay = transaction.from_account_name || transaction.from_account_code;
    const toAccountDisplay = transaction.to_account_name || transaction.to_account_code;
    accountDiv.textContent = `${fromAccountDisplay} → ${toAccountDisplay}`;
    contentWrapper.appendChild(accountDiv);
    
    // Amount
    const amountDiv = document.createElement('div');
    amountDiv.className = `transaction-amount ${transaction.category1_code.toLowerCase()}`;
    amountDiv.textContent = formatAmount(transaction.total_amount);
    contentWrapper.appendChild(amountDiv);
    
    // Memo (max 20 characters, show full text on hover)
    const memoDiv = document.createElement('div');
    memoDiv.className = 'transaction-memo';
    if (transaction.memo_text) {
        const memoText = transaction.memo_text;
        // Always set title for tooltip (useful when window is narrow)
        memoDiv.title = memoText;
        if (memoText.length > 20) {
            memoDiv.textContent = memoText.substring(0, 20) + '...';
        } else {
            memoDiv.textContent = memoText;
        }
    } else {
        memoDiv.textContent = '-';
    }
    contentWrapper.appendChild(memoDiv);
    
    item.appendChild(contentWrapper);
    
    // Actions
    const actionsDiv = document.createElement('div');
    actionsDiv.className = 'transaction-actions';
    
    const editBtn = document.createElement('button');
    editBtn.className = 'btn btn-secondary';
    editBtn.textContent = i18n.t('common.edit');
    editBtn.setAttribute('data-i18n', 'common.edit');
    editBtn.addEventListener('click', () => editTransaction(transaction.transaction_id));
    actionsDiv.appendChild(editBtn);
    
    const deleteBtn = document.createElement('button');
    deleteBtn.className = 'btn btn-danger';
    deleteBtn.textContent = i18n.t('common.delete');
    deleteBtn.setAttribute('data-i18n', 'common.delete');
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
    await openTransactionModal(transactionId);
}

async function deleteTransaction(transactionId) {
    const confirmMessage = i18n.t('transaction_mgmt.delete_confirm') || 
        'Are you sure you want to delete this transaction?';
    
    const confirmed = confirm(confirmMessage);
    if (!confirmed) {
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
        alert(i18n.t('transaction_mgmt.delete_error') || 'Failed to delete transaction: ' + error);
    }
}

// ============================================================================
// Transaction Modal Functions
// ============================================================================

let editingTransactionId = null;
let categories = [];
let accounts = [];
let transactionModal = null;

function initializeTransactionModal() {
    const category1Select = document.getElementById('category1');
    
    // Initialize Modal instance
    transactionModal = new Modal('transaction-modal', {
        formId: 'transaction-form',
        closeButtonId: 'close-transaction-modal',
        cancelButtonId: 'cancel-transaction-btn',
        onOpen: async (mode, data) => {
            // Set modal title
            const modalTitle = document.getElementById('transaction-modal-title');
            if (mode === 'edit' && data.transactionId) {
                modalTitle.textContent = i18n.t('transaction_mgmt.edit_transaction');
                editingTransactionId = data.transactionId;
            } else {
                modalTitle.textContent = i18n.t('transaction_mgmt.add_transaction');
                editingTransactionId = null;
            }
            
            // Load master data
            await loadCategoriesForModal();
            await loadAccountsForModal();
            await loadShopsForModal();

            // Reset form
            const form = document.getElementById('transaction-form');
            form.reset();
            
            // Set current date/time (round to hour, minutes=00, seconds=00)
            const now = new Date();
            now.setMinutes(0);
            now.setSeconds(0);
            now.setMilliseconds(0);
            // Format: YYYY-MM-DDTHH:mm (datetime-local format)
            const dateTimeStr = now.getFullYear() + '-' 
                + String(now.getMonth() + 1).padStart(2, '0') + '-'
                + String(now.getDate()).padStart(2, '0') + 'T'
                + String(now.getHours()).padStart(2, '0') + ':00';
            document.getElementById('transaction-date').value = dateTimeStr;
            
            // Reset detail count for new transaction
            const countElement = document.getElementById('detail-count-info');
            if (countElement && mode === 'add') {
                const itemsLabel = countElement.querySelector('[data-i18n="transaction_mgmt.detail_items"]');
                countElement.textContent = '0 ';
                if (itemsLabel) {
                    countElement.appendChild(itemsLabel);
                }
            }
            
            // If editing, load transaction data
            if (mode === 'edit' && data.transactionId && typeof data.transactionId === 'number') {
                await loadTransactionData(data.transactionId);
            }
        },
        onSave: async (formData) => {
            await handleTransactionSubmit(new Event('submit'));
        },
        onClose: () => {
            editingTransactionId = null;
        }
    });

    // Category1 change handler - control account field visibility
    category1Select.addEventListener('change', handleCategory1Change);

    // Manage shops button handler
    const manageShopsBtn = document.getElementById('manage-shops-btn');
    if (manageShopsBtn) {
        manageShopsBtn.addEventListener('click', async () => {
            // Save modal state before navigation
            await saveModalState();
            // Set caller screen in session
            await setSessionSourceScreen('transaction_mgmt');
            window.location.href = HTML_FILES.SHOP_MANAGEMENT;
        });
    }

    // Manage details button handler
    const manageDetailsBtn = document.getElementById('manage-details-btn');
    if (manageDetailsBtn) {
        manageDetailsBtn.addEventListener('click', () => {
            // Get transaction_id from the modal (when editing)
            if (editingTransactionId) {
                // Navigate to detail management screen with transaction_id
                window.location.href = `${HTML_FILES.TRANSACTION_DETAIL_MANAGEMENT}?transaction_id=${editingTransactionId}`;
            } else {
                // New transaction - need to save first
                alert(i18n.t('transaction_mgmt.save_before_details') || 'Please save the transaction first before managing details.');
            }
        });
    }
}

async function openTransactionModal(transactionId = null) {
    // Handle event object case (when called from button click)
    if (transactionId && typeof transactionId === 'object') {
        transactionId = null;
    }
    
    if (transactionId) {
        transactionModal.open('edit', { transactionId });
    } else {
        transactionModal.open('add', {});
    }
}

async function loadCategoriesForModal() {
    try {
        const categoryTree = await invoke('get_category_tree_with_lang', {
            userId: currentUserId,
            langCode: i18n.currentLanguage
        });
        
        console.log('Category tree received:', categoryTree);
        
        categories = categoryTree;
        
        // Populate category1 dropdown (keep the default option with data-i18n)
        const category1Select = document.getElementById('category1');
        // Clear only the dynamically added options (keep first option)
        while (category1Select.options.length > 1) {
            category1Select.remove(1);
        }
        
        categoryTree.forEach(cat1 => {
            console.log('Category1:', cat1);
            const option = document.createElement('option');
            option.value = cat1.category1.category1_code;
            option.textContent = cat1.category1.category1_name_i18n;
            category1Select.appendChild(option);
        });
        
    } catch (error) {
        console.error('Failed to load categories:', error);
    }
}

async function loadAccountsForModal() {
    try {
        accounts = await invoke('get_accounts', { 
            userId: currentUserId,
            userRole: currentUserRole
        });
        
        // Populate account dropdowns
        const fromAccountSelect = document.getElementById('from-account');
        const toAccountSelect = document.getElementById('to-account');
        
        // Clear ALL existing options first
        fromAccountSelect.innerHTML = '';
        toAccountSelect.innerHTML = '';
        
        // Add "NONE" option first
        const unspecifiedText = i18n.t('common.unspecified');
        
        const fromNoneOption = document.createElement('option');
        fromNoneOption.value = 'NONE';
        fromNoneOption.textContent = unspecifiedText;
        fromAccountSelect.appendChild(fromNoneOption);
        
        const toNoneOption = document.createElement('option');
        toNoneOption.value = 'NONE';
        toNoneOption.textContent = unspecifiedText;
        toAccountSelect.appendChild(toNoneOption);
        
        // Add actual accounts
        accounts.forEach(account => {
            if (account.account_code !== 'NONE') {
                const fromOption = document.createElement('option');
                fromOption.value = account.account_code;
                fromOption.textContent = account.account_name;
                fromAccountSelect.appendChild(fromOption);
                
                const toOption = document.createElement('option');
                toOption.value = account.account_code;
                toOption.textContent = account.account_name;
                toAccountSelect.appendChild(toOption);
            }
        });
        
    } catch (error) {
        console.error('Failed to load accounts:', error);
    }
}

async function loadShopsForModal() {
    try {
        const shops = await invoke('get_shops', {
            userId: currentUserId
        });

        // Populate shop dropdown
        const shopSelect = document.getElementById('shop');

        // Clear ALL existing options first
        shopSelect.innerHTML = '';

        // Add "Unspecified" option first
        const unspecifiedText = i18n.t('common.unspecified');
        const noneOption = document.createElement('option');
        noneOption.value = '';
        noneOption.textContent = unspecifiedText;
        shopSelect.appendChild(noneOption);

        // Add actual shops
        shops.forEach(shop => {
            const option = document.createElement('option');
            option.value = shop.shop_id;
            option.textContent = shop.shop_name;
            shopSelect.appendChild(option);
        });

    } catch (error) {
        console.error('Failed to load shops:', error);
    }
}

function handleCategory1Change(event) {
    const category1Code = event.target.value;
    const fromAccountGroup = document.getElementById('from-account-group');
    const toAccountGroup = document.getElementById('to-account-group');
    
    if (!category1Code) {
        // No category selected - hide both
        fromAccountGroup.style.display = 'none';
        toAccountGroup.style.display = 'none';
        // Reset both to NONE when hidden
        const fromAccountSelect = document.getElementById('from-account');
        const toAccountSelect = document.getElementById('to-account');
        if (fromAccountSelect) {
            fromAccountSelect.value = 'NONE';
        }
        if (toAccountSelect) {
            toAccountSelect.value = 'NONE';
        }
        return;
    }
    
    // Control visibility based on category1_code
    // Assuming: EXPENSE = show FROM, INCOME = show TO, TRANSFER = show both
    const category1 = categories.find(c => c.category1.category1_code === category1Code);
    
    if (!category1) {
        fromAccountGroup.style.display = 'none';
        toAccountGroup.style.display = 'none';
        // Reset both to NONE when hidden
        const fromAccountSelect = document.getElementById('from-account');
        const toAccountSelect = document.getElementById('to-account');
        if (fromAccountSelect) {
            fromAccountSelect.value = 'NONE';
        }
        if (toAccountSelect) {
            toAccountSelect.value = 'NONE';
        }
        return;
    }
    
    // Simple logic for now - adjust based on actual category codes
    const categoryName = category1.category1.category1_name_i18n.toLowerCase();
    const fromAccountSelect = document.getElementById('from-account');
    const toAccountSelect = document.getElementById('to-account');
    
    if (categoryName.includes('支出') || categoryName.includes('expense')) {
        fromAccountGroup.style.display = 'block';
        toAccountGroup.style.display = 'none';
        // Reset TO_ACCOUNT to NONE when hidden
        if (toAccountSelect) {
            toAccountSelect.value = 'NONE';
        }
    } else if (categoryName.includes('収入') || categoryName.includes('income')) {
        fromAccountGroup.style.display = 'none';
        toAccountGroup.style.display = 'block';
        // Reset FROM_ACCOUNT to NONE when hidden
        if (fromAccountSelect) {
            fromAccountSelect.value = 'NONE';
        }
    } else if (categoryName.includes('振替') || categoryName.includes('transfer')) {
        fromAccountGroup.style.display = 'block';
        toAccountGroup.style.display = 'block';
        // Both are visible, no reset needed
    } else {
        // Default: show both
        fromAccountGroup.style.display = 'block';
        toAccountGroup.style.display = 'block';
    }
    
    // Populate category2 dropdown (if exists in modal)
    const category2Select = document.getElementById('category2');
    if (category2Select) {
        category2Select.innerHTML = '<option value="">Select category</option>';
        
        if (category1.children && category1.children.length > 0) {
            category1.children.forEach(cat2 => {
                const option = document.createElement('option');
                option.value = cat2.category2.category2_code;
                option.textContent = cat2.category2.category2_name_i18n;
                category2Select.appendChild(option);
            });
        }
    }
}

async function handleTransactionSubmit(event) {
    event.preventDefault();
    
    const transactionDateInput = document.getElementById('transaction-date').value;
    const shopIdValue = document.getElementById('shop').value;
    const shopId = shopIdValue ? parseInt(shopIdValue) : null;
    const category1Code = document.getElementById('category1').value;
    const fromAccountCode = document.getElementById('from-account').value;
    const toAccountCode = document.getElementById('to-account').value;
    const totalAmount = parseInt(document.getElementById('total-amount').value);
    const taxRoundingValue = parseInt(document.getElementById('tax-rounding').value);
    const taxIncludedTypeValue = parseInt(document.getElementById('tax-included-type').value);
    const memoText = document.getElementById('transaction-memo').value.trim() || null;

    // Convert datetime-local format (YYYY-MM-DDTHH:mm) to SQLite DATETIME format (YYYY-MM-DD HH:MM:SS)
    const transactionDate = transactionDateInput.replace('T', ' ') + ':00';

    // Tax rounding value is already an integer (0, 1, or 2) from the select element
    const taxRoundingType = taxRoundingValue;
    const taxIncludedType = taxIncludedTypeValue;

    console.log('=== Transaction Data ===');
    console.log('shopId:', shopId);
    console.log('category1Code:', category1Code);
    console.log('fromAccountCode:', fromAccountCode);
    console.log('toAccountCode:', toAccountCode);
    console.log('transactionDate:', transactionDate);
    console.log('totalAmount:', totalAmount);
    console.log('taxRoundingType:', taxRoundingType);
    console.log('taxIncludedType:', taxIncludedType);
    console.log('memo:', memoText);
    
    try {
        if (editingTransactionId) {
            // Update existing transaction
            await invoke('update_transaction_header', {
                transactionId: editingTransactionId,
                shopId,
                category1Code,
                fromAccountCode,
                toAccountCode,
                transactionDate,
                totalAmount,
                taxRoundingType,
                taxIncludedType,
                memo: memoText
            });
        } else {
            // Create new transaction
            await invoke('save_transaction_header', {
                userId: currentUserId,
                shopId,
                category1Code,
                fromAccountCode,
                toAccountCode,
                transactionDate,
                totalAmount,
                taxRoundingType,
                taxIncludedType,
                memo: memoText
            });
        }
        
        // Close modal and reload list
        transactionModal.close();
        await loadTransactions();
        
    } catch (error) {
        console.error('Failed to save transaction:', error);
        alert('Failed to save transaction: ' + error);
    }
}

async function loadTransactionData(transactionId) {
    try {
        const transaction = await invoke('get_transaction_header', {
            transactionId: transactionId
        });
        
        // Convert SQLite DATETIME format (YYYY-MM-DD HH:MM:SS) to datetime-local format (YYYY-MM-DDTHH:mm)
        const dateTimeParts = transaction.transaction_date.split(' ');
        const datePart = dateTimeParts[0];  // YYYY-MM-DD
        const timePart = dateTimeParts[1] ? dateTimeParts[1].substring(0, 5) : '00:00';  // HH:MM
        const dateTimeLocal = datePart + 'T' + timePart;
        
        // Populate form fields
        document.getElementById('transaction-date').value = dateTimeLocal;
        document.getElementById('shop').value = transaction.shop_id || '';
        document.getElementById('category1').value = transaction.category1_code;
        document.getElementById('from-account').value = transaction.from_account_code || 'NONE';
        document.getElementById('to-account').value = transaction.to_account_code || 'NONE';
        document.getElementById('total-amount').value = transaction.total_amount;
        document.getElementById('tax-rounding').value = transaction.tax_rounding_type || 0;
        document.getElementById('tax-included-type').value = transaction.tax_included_type !== undefined ? transaction.tax_included_type : 1;
        document.getElementById('transaction-memo').value = transaction.memo || '';

        // Trigger category1 change to update account visibility
        handleCategory1Change({ target: document.getElementById('category1') });
        
        // Update detail count
        await updateDetailCount(transactionId);
        
    } catch (error) {
        console.error('Failed to load transaction:', error);
        alert('Failed to load transaction: ' + error);
    }
}

// Update detail count display
async function updateDetailCount(transactionId) {
    try {
        const details = await invoke('get_transaction_details', {
            transactionId: transactionId
        });
        
        const countElement = document.getElementById('detail-count-info');
        if (countElement) {
            const count = details.length;
            const itemsLabel = countElement.querySelector('[data-i18n="transaction_mgmt.detail_items"]');
            countElement.textContent = count + ' ';
            if (itemsLabel) {
                countElement.appendChild(itemsLabel);
            }
        }
    } catch (error) {
        console.error('Failed to load detail count:', error);
        // Don't show alert for count update failure, just log it
    }
}

// Modal state management functions
async function saveModalState() {
    const modalData = {
        modal_open: true,
        editing_transaction_id: editingTransactionId,
        transaction_date: document.getElementById('transaction-date')?.value,
        category1: document.getElementById('category1')?.value,
        category2: document.getElementById('category2')?.value,
        category3: document.getElementById('category3')?.value,
        account_id: document.getElementById('account')?.value,
        shop_id: document.getElementById('shop')?.value,
        from_account: document.getElementById('from-account')?.value,
        to_account: document.getElementById('to-account')?.value,
        total_amount: document.getElementById('total-amount')?.value,
        tax_rounding: document.getElementById('tax-rounding')?.value,
        tax_included_type: document.getElementById('tax-included-type')?.value,
        memo: document.getElementById('transaction-memo')?.value
    };
    
    await setSessionModalState(JSON.stringify(modalData));
}

async function restoreModalState() {
    const modalStateJson = await getSessionModalState();
    if (!modalStateJson) {
        return;
    }
    
    try {
        const modalData = JSON.parse(modalStateJson);
        
        // Clear session state
        await clearSessionModalState();
        
        // Open modal
        if (modalData.editing_transaction_id) {
            // Editing mode - open with transaction data
            await openTransactionModal(modalData.editing_transaction_id);
            
            // Wait for modal to be fully populated
            await new Promise(resolve => setTimeout(resolve, 200));
            
            // Override with saved values (in case user made changes before navigating away)
            if (modalData.transaction_date) {
                document.getElementById('transaction-date').value = modalData.transaction_date;
            }
            if (modalData.shop_id && modalData.shop_id !== 'null') {
                document.getElementById('shop').value = modalData.shop_id;
            }
            if (modalData.total_amount) {
                document.getElementById('total-amount').value = modalData.total_amount;
            }
            if (modalData.memo) {
                document.getElementById('transaction-memo').value = modalData.memo;
            }
        } else {
            // New transaction mode
            await openTransactionModal();
            
            // Restore form values
            if (modalData.transaction_date) {
                document.getElementById('transaction-date').value = modalData.transaction_date;
            }
            if (modalData.category1) {
                document.getElementById('category1').value = modalData.category1;
                // Trigger change to load dependent dropdowns
                await handleCategory1Change({ target: document.getElementById('category1') });
                
                // Wait a bit for category2 to load
                await new Promise(resolve => setTimeout(resolve, 100));
                
                if (modalData.category2) {
                    document.getElementById('category2').value = modalData.category2;
                    // Trigger change to load category3
                    document.getElementById('category2').dispatchEvent(new Event('change'));
                    
                    await new Promise(resolve => setTimeout(resolve, 100));
                    
                    if (modalData.category3) {
                        document.getElementById('category3').value = modalData.category3;
                    }
                }
            }
            if (modalData.account_id) {
                document.getElementById('account').value = modalData.account_id;
            }
            if (modalData.shop_id) {
                document.getElementById('shop').value = modalData.shop_id;
            }
            if (modalData.from_account) {
                document.getElementById('from-account').value = modalData.from_account;
            }
            if (modalData.to_account) {
                document.getElementById('to-account').value = modalData.to_account;
            }
            if (modalData.total_amount) {
                document.getElementById('total-amount').value = modalData.total_amount;
            }
            if (modalData.tax_rounding) {
                document.getElementById('tax-rounding').value = modalData.tax_rounding;
            }
            if (modalData.tax_included_type) {
                document.getElementById('tax-included-type').value = modalData.tax_included_type;
            }
            if (modalData.memo) {
                document.getElementById('transaction-memo').value = modalData.memo;
            }
        }
    } catch (error) {
        console.error('Failed to restore modal state:', error);
        await clearSessionModalState();
    }
}
