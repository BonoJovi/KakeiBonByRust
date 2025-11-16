import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { setupLanguageMenuHandlers, setupLanguageMenu, handleLogout, handleQuit } from './menu.js';
import { HTML_FILES } from './html-files.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';

let currentUserId = null;
let currentUserRole = null;
let transactionId = null;
let category1Code = null; // Store CATEGORY1_CODE from transaction header

document.addEventListener('DOMContentLoaded', async function() {
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
        
        // Get transaction ID from URL parameters
        const urlParams = new URLSearchParams(window.location.search);
        transactionId = urlParams.get('transaction_id');
        
        if (!transactionId) {
            console.error('No transaction ID provided');
            showMessage('error', 'No transaction ID provided');
            return;
        }
        
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
        
        // Load transaction header info
        await loadTransactionHeader();
        
        // Load transaction details
        await loadDetails();
        
        // Adjust window size after content is loaded
        await adjustWindowSize();
        
    } catch (error) {
        console.error('Failed to initialize:', error);
        showMessage('error', `Initialization failed: ${error.message}`);
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
        
        // Back to transactions
        const backToTransactionsItem = fileDropdown.querySelector('[data-i18n="menu.back_to_transactions"]');
        if (backToTransactionsItem) {
            backToTransactionsItem.addEventListener('click', () => {
                window.location.href = HTML_FILES.TRANSACTION_MANAGEMENT;
            });
        }
        
        // Logout handler
        const logoutItem = fileDropdown.querySelector('[data-i18n="menu.logout"]');
        if (logoutItem) {
            logoutItem.addEventListener('click', handleLogout);
        }
        
        // Quit handler
        const quitItem = fileDropdown.querySelector('[data-i18n="menu.quit"]');
        if (quitItem) {
            quitItem.addEventListener('click', handleQuit);
        }
    }
    
    // Close dropdown when clicking outside
    document.addEventListener('click', () => {
        if (fileDropdown) {
            fileDropdown.classList.remove('show');
        }
    });
}

function setupEventListeners() {
    // Add detail button
    const addDetailBtn = document.getElementById('add-detail-btn');
    if (addDetailBtn) {
        addDetailBtn.addEventListener('click', () => {
            openDetailModal();
        });
    }
    
    // Modal close buttons
    const closeModalBtn = document.getElementById('close-modal');
    if (closeModalBtn) {
        closeModalBtn.addEventListener('click', closeDetailModal);
    }
    
    const cancelBtn = document.getElementById('cancel-btn');
    if (cancelBtn) {
        cancelBtn.addEventListener('click', closeDetailModal);
    }
    
    // Detail form submit
    const detailForm = document.getElementById('detail-form');
    if (detailForm) {
        detailForm.addEventListener('submit', handleDetailFormSubmit);
    }
    
    // Delete modal handlers
    const closeDeleteModalBtn = document.getElementById('close-delete-modal');
    if (closeDeleteModalBtn) {
        closeDeleteModalBtn.addEventListener('click', closeDeleteModal);
    }
    
    const cancelDeleteBtn = document.getElementById('cancel-delete-btn');
    if (cancelDeleteBtn) {
        cancelDeleteBtn.addEventListener('click', closeDeleteModal);
    }
    
    const confirmDeleteBtn = document.getElementById('confirm-delete-btn');
    if (confirmDeleteBtn) {
        confirmDeleteBtn.addEventListener('click', handleDeleteConfirm);
    }
}

async function loadTransactionHeader() {
    try {
        console.log('Loading transaction header for ID:', transactionId);
        
        // Get transaction header from backend
        const header = await invoke('get_transaction_header', {
            userId: currentUserId,
            transactionId: parseInt(transactionId)
        });
        
        if (!header) {
            throw new Error('Transaction header not found');
        }
        
        // Store CATEGORY1_CODE for detail operations
        category1Code = header.category1_code;
        
        // Display header information
        document.getElementById('header-transaction-date').textContent = header.transaction_date || '-';
        document.getElementById('header-account').textContent = header.account_name || '-';
        document.getElementById('header-shop').textContent = header.shop_name || '-';
        document.getElementById('header-total-amount').textContent = header.amount 
            ? `¥${header.amount.toLocaleString()}` 
            : '¥0';
        
        console.log('Transaction header loaded successfully, CATEGORY1_CODE:', category1Code);
    } catch (error) {
        console.error('Failed to load transaction header:', error);
        showMessage('error', `Failed to load transaction header: ${error.message}`);
    }
}

async function loadDetails() {
    // Placeholder: Load transaction details
    console.log('Loading details for transaction ID:', transactionId);
    
    const detailList = document.getElementById('detail-list');
    if (!detailList) return;
    
    // TODO: Implement backend call to get transaction details
    // For now, show "no data" message
    detailList.innerHTML = `
        <tr>
            <td colspan="5" style="text-align: center; padding: 40px; color: #757575;">
                ${i18n.t('common.no_data')}
            </td>
        </tr>
    `;
}

function openDetailModal(detail = null) {
    const modal = document.getElementById('detail-modal');
    const modalTitle = document.getElementById('modal-title');
    const detailForm = document.getElementById('detail-form');
    
    if (!modal || !modalTitle || !detailForm) return;
    
    // Reset form
    detailForm.reset();
    document.getElementById('detail-id').value = '';
    
    // Set CATEGORY1_CODE from header (always needed for both add and edit)
    document.getElementById('category1-code').value = category1Code || '';
    
    if (detail) {
        // Edit mode
        modalTitle.setAttribute('data-i18n', 'detail_mgmt.edit_detail');
        modalTitle.textContent = i18n.t('detail_mgmt.edit_detail');
        
        // Populate form with detail data
        document.getElementById('detail-id').value = detail.detail_id;
        document.getElementById('item-name').value = detail.item_name;
        document.getElementById('category2-code').value = detail.category2_code;
        document.getElementById('category3-code').value = detail.category3_code;
        document.getElementById('amount').value = detail.amount;
        document.getElementById('tax-rate').value = detail.tax_rate;
        document.getElementById('tax-amount').value = detail.tax_amount;
        if (detail.memo) {
            document.getElementById('memo').value = detail.memo;
        }
    } else {
        // Add mode
        modalTitle.setAttribute('data-i18n', 'detail_mgmt.add_detail');
        modalTitle.textContent = i18n.t('detail_mgmt.add_detail');
    }
    
    modal.classList.remove('hidden');
}

function closeDetailModal() {
    const modal = document.getElementById('detail-modal');
    if (modal) {
        modal.classList.add('hidden');
    }
}

async function handleDetailFormSubmit(event) {
    event.preventDefault();
    
    const detailId = document.getElementById('detail-id').value;
    const itemName = document.getElementById('item-name').value.trim();
    const category1Code = document.getElementById('category1-code').value;
    const category2Code = document.getElementById('category2-code').value;
    const category3Code = document.getElementById('category3-code').value;
    const amount = parseInt(document.getElementById('amount').value);
    const taxRate = parseInt(document.getElementById('tax-rate').value);
    const taxAmount = parseInt(document.getElementById('tax-amount').value);
    const memo = document.getElementById('memo').value.trim();
    
    // Validation
    if (!itemName) {
        showMessage('error', i18n.t('detail_mgmt.error_item_name_required'));
        return;
    }
    
    if (!category1Code || !category2Code || !category3Code) {
        showMessage('error', i18n.t('detail_mgmt.error_category_required'));
        return;
    }
    
    if (isNaN(amount) || amount < 0) {
        showMessage('error', i18n.t('detail_mgmt.error_invalid_amount'));
        return;
    }
    
    try {
        // TODO: Implement backend call to save detail
        console.log('Saving detail:', {
            detailId,
            transactionId,
            itemName,
            category1Code,
            category2Code,
            category3Code,
            amount,
            taxRate,
            taxAmount,
            memo
        });
        
        closeDetailModal();
        showMessage('success', i18n.t('detail_mgmt.save_success'));
        await loadDetails();
        await loadTransactionHeader(); // Refresh total amount
        
    } catch (error) {
        console.error('Failed to save detail:', error);
        showMessage('error', `${i18n.t('detail_mgmt.save_error')}: ${error.message}`);
    }
}

function openDeleteModal(detail) {
    const modal = document.getElementById('delete-modal');
    const detailName = document.getElementById('delete-detail-name');
    
    if (!modal || !detailName) return;
    
    detailName.textContent = detail.item_name;
    modal.dataset.detailId = detail.detail_id;
    modal.classList.remove('hidden');
}

function closeDeleteModal() {
    const modal = document.getElementById('delete-modal');
    if (modal) {
        modal.classList.add('hidden');
        delete modal.dataset.detailId;
    }
}

async function handleDeleteConfirm() {
    const modal = document.getElementById('delete-modal');
    const detailId = modal.dataset.detailId;
    
    if (!detailId) return;
    
    try {
        // TODO: Implement backend call to delete detail
        console.log('Deleting detail:', detailId);
        
        closeDeleteModal();
        showMessage('success', i18n.t('detail_mgmt.delete_success'));
        await loadDetails();
        await loadTransactionHeader(); // Refresh total amount
        
    } catch (error) {
        console.error('Failed to delete detail:', error);
        showMessage('error', `${i18n.t('detail_mgmt.delete_error')}: ${error.message}`);
    }
}

function showMessage(type, text) {
    const messageDiv = document.getElementById('detail-message');
    if (!messageDiv) return;
    
    messageDiv.className = `message ${type}`;
    messageDiv.textContent = text;
    messageDiv.style.display = 'block';
    
    setTimeout(() => {
        messageDiv.style.display = 'none';
    }, 5000);
}
