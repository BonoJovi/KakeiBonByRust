import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { setupLanguageMenuHandlers, setupLanguageMenu, handleLogout, handleQuit } from './menu.js';
import { HTML_FILES } from './html-files.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';

let currentUserId = null;
let currentUserRole = null;
let transactionId = null;
let category1Code = null; // Store CATEGORY1_CODE from transaction header
let lastTaxInputField = null; // Track which amount field was last edited: 'excluding' or 'including'
let taxRoundingType = 0; // Store TAX_ROUNDING_TYPE from transaction header (0: floor, 1: half-up, 2: ceil)

document.addEventListener('DOMContentLoaded', async function() {
    
    // Create menu bar
    createMenuBar('transaction-detail');
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

/**
 * Setup tax calculation listeners for automatic calculation
 * between tax-excluded and tax-included amounts
 */
function setupTaxCalculationListeners() {
    const taxRate = document.getElementById('tax-rate');
    const amountExcludingTax = document.getElementById('amount-excluding-tax');
    const amountIncludingTax = document.getElementById('amount-including-tax');
    const taxAmount = document.getElementById('tax-amount');
    
    if (!taxRate || !amountExcludingTax || !amountIncludingTax || !taxAmount) {
        return;
    }
    
    // Calculate tax-included amount when tax-excluded amount is entered
    amountExcludingTax.addEventListener('input', () => {
        calculateFromExcludingTax(
            amountExcludingTax,
            taxRate,
            taxAmount,
            amountIncludingTax
        );
    });
    
    // Calculate tax-excluded amount when tax-included amount is entered
    amountIncludingTax.addEventListener('input', () => {
        calculateFromIncludingTax(
            amountIncludingTax,
            taxRate,
            taxAmount,
            amountExcludingTax
        );
    });
    
    // Recalculate when tax rate changes
    taxRate.addEventListener('change', () => {
        // Recalculate based on which field was last edited
        if (lastTaxInputField === 'including' && amountIncludingTax.value) {
            // User last edited tax-included amount, so recalculate tax-excluded
            calculateFromIncludingTax(
                amountIncludingTax,
                taxRate,
                taxAmount,
                amountExcludingTax
            );
        } else if (lastTaxInputField === 'excluding' && amountExcludingTax.value) {
            // User last edited tax-excluded amount, so recalculate tax-included
            calculateFromExcludingTax(
                amountExcludingTax,
                taxRate,
                taxAmount,
                amountIncludingTax
            );
        } else if (amountExcludingTax.value) {
            // Default to tax-excluded if no flag is set
            calculateFromExcludingTax(
                amountExcludingTax,
                taxRate,
                taxAmount,
                amountIncludingTax
            );
        } else if (amountIncludingTax.value) {
            // Fall back to tax-included
            calculateFromIncludingTax(
                amountIncludingTax,
                taxRate,
                taxAmount,
                amountExcludingTax
            );
        }
    });
}

/**
 * Apply rounding based on tax rounding type
 * @param {number} value - Value to round
 * @param {number} roundingType - 0: floor, 1: half-up, 2: ceil
 * @returns {number} Rounded value
 */
function applyTaxRounding(value, roundingType = 0) {
    switch (roundingType) {
        case 0: // Round down (切り捨て)
            return Math.floor(value);
        case 1: // Round half-up (四捨五入)
            return Math.round(value);
        case 2: // Round up (切り上げ)
            return Math.ceil(value);
        default:
            return Math.floor(value);
    }
}

/**
 * Calculate tax-included amount from tax-excluded amount
 * @param {HTMLInputElement} excludingTaxInput - Tax-excluded amount input
 * @param {HTMLSelectElement} taxRateSelect - Tax rate select
 * @param {HTMLInputElement} taxAmountInput - Tax amount input (readonly)
 * @param {HTMLInputElement} includingTaxInput - Tax-included amount input
 */
function calculateFromExcludingTax(excludingTaxInput, taxRateSelect, taxAmountInput, includingTaxInput) {
    const excluded = parseFloat(excludingTaxInput.value) || 0;
    const rate = parseFloat(taxRateSelect.value) || 0;
    
    // Clear any previous warning (user is now entering tax-excluded amount)
    clearRoundingWarning();
    
    // Mark that tax-excluded amount was last edited
    lastTaxInputField = 'excluding';
    
    // Calculate tax amount using the configured rounding type
    const tax = applyTaxRounding(excluded * rate / 100, taxRoundingType);
    
    // Calculate including tax amount
    const included = excluded + tax;
    
    // Update fields
    taxAmountInput.value = tax;
    includingTaxInput.value = included || '';
}

/**
 * Calculate tax-excluded amount from tax-included amount
 * @param {HTMLInputElement} includingTaxInput - Tax-included amount input
 * @param {HTMLSelectElement} taxRateSelect - Tax rate select
 * @param {HTMLInputElement} taxAmountInput - Tax amount input (readonly)
 * @param {HTMLInputElement} excludingTaxInput - Tax-excluded amount input
 */
function calculateFromIncludingTax(includingTaxInput, taxRateSelect, taxAmountInput, excludingTaxInput) {
    const included = parseFloat(includingTaxInput.value) || 0;
    const rate = parseFloat(taxRateSelect.value) || 0;
    
    // Clear any previous warning
    clearRoundingWarning();
    
    // Mark that tax-included amount was last edited
    lastTaxInputField = 'including';
    
    if (!included) {
        excludingTaxInput.value = '';
        taxAmountInput.value = 0;
        return;
    }
    
    if (rate === 0) {
        // No tax
        excludingTaxInput.value = included || '';
        taxAmountInput.value = 0;
        return;
    }
    
    // Calculate tax-excluded amount using the configured rounding type
    const excluded = applyTaxRounding(included / (1 + rate / 100), taxRoundingType);
    
    // Calculate tax amount
    const tax = included - excluded;
    
    // Verify by reverse calculation
    const taxReverse = applyTaxRounding(excluded * rate / 100, taxRoundingType);
    const includedReverse = excluded + taxReverse;
    
    // Check if there's a rounding discrepancy
    if (includedReverse !== included) {
        showRoundingWarning(included, includedReverse);
    }
    
    // Update fields
    taxAmountInput.value = tax;
    excludingTaxInput.value = excluded || '';
}

/**
 * Show warning message for rounding discrepancy
 */
function showRoundingWarning(userInput, calculated) {
    const warningDiv = document.getElementById('rounding-warning');
    if (!warningDiv) {
        // Create warning element if it doesn't exist
        const detailForm = document.getElementById('detail-form');
        const newWarning = document.createElement('div');
        newWarning.id = 'rounding-warning';
        newWarning.className = 'rounding-warning';
        newWarning.style.cssText = 'background-color: #fff3cd; color: #856404; padding: 10px; margin: 10px 0; border: 1px solid #ffc107; border-radius: 4px;';
        
        const amountInputs = detailForm.querySelector('#amount-including-tax').closest('.form-group');
        amountInputs.parentNode.insertBefore(newWarning, amountInputs.nextSibling);
    }
    
    const warning = document.getElementById('rounding-warning');
    const diff = Math.abs(userInput - calculated);
    warning.innerHTML = `
        <strong>⚠️ ${i18n.t('detail_mgmt.rounding_warning_title')}</strong><br>
        ${i18n.t('detail_mgmt.rounding_warning_message')
            .replace('{userInput}', userInput.toLocaleString())
            .replace('{calculated}', calculated.toLocaleString())
            .replace('{diff}', diff)}
    `;
    warning.style.display = 'block';
}

/**
 * Clear rounding warning message
 */
function clearRoundingWarning() {
    const warningDiv = document.getElementById('rounding-warning');
    if (warningDiv) {
        warningDiv.style.display = 'none';
    }
}

function setupEventListeners() {
    // Add detail button
    const addDetailBtn = document.getElementById('add-detail-btn');
    if (addDetailBtn) {
        addDetailBtn.addEventListener('click', async () => {
            await openDetailModal();
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
    
    // Detail modal backdrop click and ESC key
    const detailModal = document.getElementById('detail-modal');
    if (detailModal) {
        detailModal.addEventListener('click', (e) => {
            if (e.target === detailModal) {
                closeDetailModal();
            }
        });
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
    
    // Delete modal backdrop click and ESC key
    const deleteModal = document.getElementById('delete-modal');
    if (deleteModal) {
        deleteModal.addEventListener('click', (e) => {
            if (e.target === deleteModal) {
                closeDeleteModal();
            }
        });
    }
    
    // Global ESC key handler for modals
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            const detailModal = document.getElementById('detail-modal');
            const deleteModal = document.getElementById('delete-modal');
            
            if (detailModal && !detailModal.classList.contains('hidden')) {
                closeDetailModal();
            } else if (deleteModal && !deleteModal.classList.contains('hidden')) {
                closeDeleteModal();
            }
        }
    });
    
    // Tax calculation listeners
    setupTaxCalculationListeners();
    
    // Category2 change listener
    const category2Select = document.getElementById('category2-code');
    if (category2Select) {
        category2Select.addEventListener('change', async (e) => {
            await loadCategory3Options(e.target.value);
        });
    }
}

/**
 * Load category dropdowns based on CATEGORY1_CODE from header
 */
async function loadCategoryDropdowns() {
    console.log('loadCategoryDropdowns called, category1Code:', category1Code);
    
    if (!category1Code) {
        console.error('category1Code is not set yet');
        showMessage('error', 'Category code not loaded. Please refresh the page.');
        return;
    }
    
    try {
        const categoryTree = await invoke('get_category_tree_with_lang', {
            userId: currentUserId,
            langCode: i18n.getCurrentLanguage()
        });
        
        console.log('Category tree loaded:', categoryTree);
        console.log('Looking for category1Code:', category1Code, 'type:', typeof category1Code);
        
        // Find the category1 node matching our category1Code
        const category1Node = categoryTree.find(cat1 => cat1.category1?.category1_code === category1Code);
        
        if (!category1Node) {
            console.error('Category1 not found:', category1Code);
            return;
        }
        
        console.log('Category1 node found:', category1Node);
        
        // Populate CATEGORY2 dropdown
        const category2Select = document.getElementById('category2-code');
        if (category2Select) {
            category2Select.innerHTML = '<option value="">' + i18n.t('common.select') + '</option>';
            
            if (category1Node.children && category1Node.children.length > 0) {
                category1Node.children.forEach(cat2Item => {
                    const cat2 = cat2Item.category2;
                    const option = document.createElement('option');
                    option.value = cat2.category2_code;
                    option.textContent = cat2.category2_name_i18n;
                    category2Select.appendChild(option);
                });
            }
        }
        
        // Clear CATEGORY3 dropdown (will be populated when CATEGORY2 is selected)
        const category3Select = document.getElementById('category3-code');
        if (category3Select) {
            category3Select.innerHTML = '<option value="">' + i18n.t('common.select') + '</option>';
        }
        
    } catch (error) {
        console.error('Failed to load categories:', error);
        showMessage('error', `Failed to load categories: ${error.message}`);
    }
}

/**
 * Load CATEGORY3 options based on selected CATEGORY2
 */
async function loadCategory3Options(category2Code) {
    if (!category2Code) {
        const category3Select = document.getElementById('category3-code');
        if (category3Select) {
            category3Select.innerHTML = '<option value="">' + i18n.t('common.select') + '</option>';
        }
        return;
    }
    
    try {
        const categoryTree = await invoke('get_category_tree_with_lang', {
            userId: currentUserId,
            langCode: i18n.getCurrentLanguage()
        });
        
        // Find the category1 and category2 nodes
        const category1Node = categoryTree.find(cat1 => cat1.category1?.category1_code === category1Code);
        if (!category1Node) return;
        
        const category2Node = category1Node.children?.find(cat2Item => cat2Item.category2?.category2_code === category2Code);
        if (!category2Node) return;
        
        // Populate CATEGORY3 dropdown
        const category3Select = document.getElementById('category3-code');
        if (category3Select) {
            category3Select.innerHTML = '<option value="">' + i18n.t('common.select') + '</option>';
            
            if (category2Node.children && category2Node.children.length > 0) {
                category2Node.children.forEach(cat3 => {
                    const option = document.createElement('option');
                    option.value = cat3.category3_code;
                    option.textContent = cat3.category3_name_i18n;
                    category3Select.appendChild(option);
                });
            }
        }
        
    } catch (error) {
        console.error('Failed to load category3 options:', error);
    }
}

async function loadTransactionHeader() {
    try {
        console.log('Loading transaction header for ID:', transactionId);
        
        // Get transaction header from backend
        const header = await invoke('get_transaction_header', {
            transactionId: parseInt(transactionId)
        });
        
        if (!header) {
            throw new Error('Transaction header not found');
        }
        
        // Store CATEGORY1_CODE and TAX_ROUNDING_TYPE for detail operations
        category1Code = header.category1_code;
        taxRoundingType = header.tax_rounding_type ?? 0; // Default to floor (0) if not set
        
        // Display header information
        document.getElementById('header-transaction-date').textContent = header.transaction_date || '-';
        document.getElementById('header-account').textContent = header.account_name || '-';
        document.getElementById('header-shop').textContent = header.shop_name || '-';
        document.getElementById('header-total-amount').textContent = header.amount 
            ? `¥${header.amount.toLocaleString()}` 
            : '¥0';
        
        console.log('Transaction header loaded successfully, CATEGORY1_CODE:', category1Code, 'TAX_ROUNDING_TYPE:', taxRoundingType);
    } catch (error) {
        console.error('Failed to load transaction header:', error);
        showMessage('error', `Failed to load transaction header: ${error.message}`);
    }
}

async function loadDetails() {
    try {
        console.log('Loading details for transaction ID:', transactionId);
        
        const detailList = document.getElementById('detail-list');
        if (!detailList) return;
        
        // Show loading state
        detailList.innerHTML = `
            <tr class="loading-row">
                <td colspan="5" class="loading" style="text-align: center; padding: 20px;">
                    ${i18n.t('common.loading')}
                </td>
            </tr>
        `;
        
        // Get transaction details from backend
        const details = await invoke('get_transaction_details', {
            userId: currentUserId,
            transactionId: parseInt(transactionId)
        });
        
        if (!details || details.length === 0) {
            // No details found
            detailList.innerHTML = `
                <tr>
                    <td colspan="5" style="text-align: center; padding: 40px; color: #757575;">
                        ${i18n.t('common.no_data')}
                    </td>
                </tr>
            `;
            return;
        }
        
        // Display details
        detailList.innerHTML = details.map((detail, index) => `
            <tr data-detail-id="${detail.detail_id}">
                <td>${escapeHtml(detail.item_name)}</td>
                <td>${escapeHtml(detail.category2_name)} / ${escapeHtml(detail.category3_name)}</td>
                <td style="text-align: right;">¥${detail.amount_including_tax?.toLocaleString() || detail.amount.toLocaleString()}</td>
                <td style="text-align: right;">¥${detail.tax_amount.toLocaleString()}</td>
                <td class="actions">
                    <button class="btn btn-small btn-secondary edit-detail-btn" data-detail-id="${detail.detail_id}" data-i18n="common.edit">Edit</button>
                    <button class="btn btn-small btn-danger delete-detail-btn" data-detail-id="${detail.detail_id}" data-i18n="common.delete">Delete</button>
                </td>
            </tr>
        `).join('');
        
        // Add event listeners to dynamically created buttons
        document.querySelectorAll('.edit-detail-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const detailId = parseInt(e.target.dataset.detailId);
                editDetail(detailId);
            });
        });
        
        document.querySelectorAll('.delete-detail-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const detailId = parseInt(e.target.dataset.detailId);
                confirmDeleteDetail(detailId);
            });
        });
        
        // Update i18n for dynamically added elements
        i18n.updateUI();
        
    } catch (error) {
        console.error('Failed to load transaction details:', error);
        const detailList = document.getElementById('detail-list');
        if (detailList) {
            detailList.innerHTML = `
                <tr>
                    <td colspan="5" style="text-align: center; padding: 40px; color: #d32f2f;">
                        ${i18n.t('common.error')}: ${error.message}
                    </td>
                </tr>
            `;
        }
    }
}

/**
 * Escape HTML to prevent XSS
 */
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

async function openDetailModal(detail = null) {
    const modal = document.getElementById('detail-modal');
    const modalTitle = document.getElementById('modal-title');
    const detailForm = document.getElementById('detail-form');
    
    if (!modal || !modalTitle || !detailForm) return;
    
    // Reset form
    detailForm.reset();
    document.getElementById('detail-id').value = '';
    
    // Set CATEGORY1_CODE from header (always needed for both add and edit)
    document.getElementById('category1-code').value = category1Code || '';
    
    // Load category dropdowns
    await loadCategoryDropdowns();
    
    if (detail) {
        // Edit mode
        modalTitle.setAttribute('data-i18n', 'detail_mgmt.edit_detail');
        modalTitle.textContent = i18n.t('detail_mgmt.edit_detail');
        
        // Populate form with detail data
        document.getElementById('detail-id').value = detail.detail_id;
        document.getElementById('item-name').value = detail.item_name;
        
        // Set category values after dropdowns are loaded
        document.getElementById('category2-code').value = detail.category2_code;
        await loadCategory3Options(detail.category2_code);
        document.getElementById('category3-code').value = detail.category3_code;
        
        document.getElementById('amount-excluding-tax').value = detail.amount;
        document.getElementById('tax-rate').value = detail.tax_rate;
        document.getElementById('tax-amount').value = detail.tax_amount;
        document.getElementById('amount-including-tax').value = detail.amount_including_tax;
        if (detail.memo_text) {
            document.getElementById('memo').value = detail.memo_text;
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
    const amountExcludingTax = parseInt(document.getElementById('amount-excluding-tax').value) || 0;
    const amountIncludingTax = parseInt(document.getElementById('amount-including-tax').value) || 0;
    const taxRate = parseInt(document.getElementById('tax-rate').value) || 0;
    const taxAmount = parseInt(document.getElementById('tax-amount').value) || 0;
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
    
    if (amountExcludingTax < 0 || amountIncludingTax < 0) {
        showMessage('error', i18n.t('detail_mgmt.error_invalid_amount'));
        return;
    }
    
    try {
        if (detailId) {
            // Update existing detail
            await invoke('update_transaction_detail', {
                detailId: parseInt(detailId),
                category1Code: category1Code,
                category2Code: category2Code,
                category3Code: category3Code,
                itemName: itemName,
                amount: amountExcludingTax,
                amountIncludingTax: amountIncludingTax,
                taxRate: taxRate,
                taxAmount: taxAmount,
                memo: memo || null
            });
        } else {
            // Add new detail
            await invoke('add_transaction_detail', {
                transactionId: parseInt(transactionId),
                category1Code: category1Code,
                category2Code: category2Code,
                category3Code: category3Code,
                itemName: itemName,
                amount: amountExcludingTax,
                amountIncludingTax: amountIncludingTax,
                taxRate: taxRate,
                taxAmount: taxAmount,
                memo: memo || null
            });
        }
        
        closeDetailModal();
        showMessage('success', i18n.t('detail_mgmt.save_success'));
        await loadDetails();
        await loadTransactionHeader(); // Refresh total amount
        
    } catch (error) {
        console.error('Failed to save detail:', error);
        showMessage('error', `${i18n.t('detail_mgmt.save_error')}: ${error.message}`);
    }
}

/**
 * Edit an existing detail
 */
async function editDetail(detailId) {
    try {
        // Get all details
        const details = await invoke('get_transaction_details', {
            userId: currentUserId,
            transactionId: parseInt(transactionId)
        });
        
        // Find the detail to edit
        const detail = details.find(d => d.detail_id === detailId);
        if (!detail) {
            showMessage('error', 'Detail not found');
            return;
        }
        
        // Open modal with detail data
        await openDetailModal(detail);
        
    } catch (error) {
        console.error('Failed to load detail for editing:', error);
        showMessage('error', `Failed to load detail: ${error.message}`);
    }
}

/**
 * Confirm deletion of a detail
 */
async function confirmDeleteDetail(detailId) {
    try {
        // Get all details
        const details = await invoke('get_transaction_details', {
            userId: currentUserId,
            transactionId: parseInt(transactionId)
        });
        
        // Find the detail to delete
        const detail = details.find(d => d.detail_id === detailId);
        if (!detail) {
            showMessage('error', 'Detail not found');
            return;
        }
        
        // Open delete confirmation modal
        openDeleteModal(detail);
        
    } catch (error) {
        console.error('Failed to load detail for deletion:', error);
        showMessage('error', `Failed to load detail: ${error.message}`);
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
        await invoke('delete_transaction_detail', {
            userId: currentUserId,
            detailId: parseInt(detailId)
        });
        
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

// Export functions for testing
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        applyTaxRounding
    };
}
