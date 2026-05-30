import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';
import { fitWindowToScreen } from './window-fit.js';
import { setupLanguageMenuHandlers, setupLanguageMenu, handleLogout, handleQuit } from './menu.js';
import { HTML_FILES } from './html-files.js';
import { ROLE_ADMIN, MAX_ITEM_NAME_LEN, MAX_MEMO_LEN } from './consts.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';
import { applyHeaderRecalculationPrompt } from './header-recalc.js';
import { setupTaxCalculationListeners } from './detail-tax-calc.js';
import { showValidationError, clearValidationError, showMaxLengthError, attachCharCounter } from './validation-display.js';

let currentUserId = null;
let currentUserRole = null;
let transactionId = null;
let category1Code = null; // Store CATEGORY1_CODE from transaction header
let taxRoundingType = 0; // Store TAX_ROUNDING_TYPE from transaction header (0: floor, 1: half-up, 2: ceil)
let currentHeaderTotal = 0; // Cached TOTAL_AMOUNT from the loaded header; used to detect drift after a detail edit

// Product autocomplete state (v2.6.0 master integration).
// PRODUCT_ID is held off-DOM so a user can type a custom item name on top of
// a previous selection without us accidentally still sending the stale id.
// Any keystroke after a selection clears it; only a fresh pick re-sets it.
const autocompleteState = {
    selectedProductId: null,
    candidates: [],
    activeIndex: -1,
    debounceTimer: null,
    // Token guards against an older fetch's results overwriting a newer one
    // when the user types fast. Each new fetch bumps the token; the response
    // only renders if its token still matches.
    requestToken: 0,
};

// Detail-form draft persisted across the round-trip to product-management.
// When the user clicks "Open in product master" mid-edit, the in-flight form
// state is serialized to sessionStorage so it can be restored when the user
// comes back via the "Back to detail entry" button. The v1 suffix lets us
// evolve the schema later without crashing on stale drafts in a session.
const DETAIL_DRAFT_KEY = 'kakeibon.detail_draft.v1';

export function buildDraftFromForm() {
    return {
        transaction_id: transactionId,
        detail_id: document.getElementById('detail-id')?.value || null,
        item_name: document.getElementById('item-name')?.value || '',
        category2_code: document.getElementById('category2-code')?.value || '',
        category3_code: document.getElementById('category3-code')?.value || '',
        tax_rate: document.getElementById('tax-rate')?.value || '10',
        amount_excluding_tax: document.getElementById('amount-excluding-tax')?.value || '',
        amount_including_tax: document.getElementById('amount-including-tax')?.value || '',
        tax_amount: document.getElementById('tax-amount')?.value || '',
        memo: document.getElementById('memo')?.value || '',
        selected_product_id: autocompleteState.selectedProductId,
    };
}

export function persistDraft(draft) {
    sessionStorage.setItem(DETAIL_DRAFT_KEY, JSON.stringify(draft));
}

export function consumeDraft() {
    const raw = sessionStorage.getItem(DETAIL_DRAFT_KEY);
    if (!raw) return null;
    try {
        return JSON.parse(raw);
    } catch (e) {
        console.error('Failed to parse detail draft, discarding', e);
        sessionStorage.removeItem(DETAIL_DRAFT_KEY);
        return null;
    }
}

export function clearDraft() {
    sessionStorage.removeItem(DETAIL_DRAFT_KEY);
}

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

        // Check if user is admin - admin cannot access transaction detail management
        if (currentUserRole === ROLE_ADMIN) {
            console.log('Admin user detected, transaction detail management access denied');
            // alert() is intentional here: see Issue #50 comment on
            // navigation-bound access-denied notices.
            alert(i18n.t('transaction.admin_access_denied') || 'Transaction management is not available for administrator accounts. Please login as a regular user.');
            window.location.href = HTML_FILES.INDEX;
            return;
        }

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

        // If we just came back from product-management via the "Back to detail
        // entry" button, restore the half-filled modal. Only consume the draft
        // on a successful restore; otherwise leave it for the user to retry.
        if (urlParams.get('restore') === '1') {
            const draft = consumeDraft();
            if (draft) {
                try {
                    await restoreDraftIntoModal(draft);
                    clearDraft();
                } catch (e) {
                    console.error('Failed to restore detail draft', e);
                }
            }
        }

        // Fit + center the window on this monitor
        await fitWindowToScreen();
        
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
 * Wire up tax-calculation listeners using the shared module.
 */
function installTaxCalculationListeners() {
    const taxRate = document.getElementById('tax-rate');
    const amountExcludingTax = document.getElementById('amount-excluding-tax');
    const amountIncludingTax = document.getElementById('amount-including-tax');
    const taxAmount = document.getElementById('tax-amount');
    if (!taxRate || !amountExcludingTax || !amountIncludingTax || !taxAmount) {
        return;
    }
    setupTaxCalculationListeners(
        { taxRate, amountExcludingTax, amountIncludingTax, taxAmount },
        {
            getRoundingType: () => taxRoundingType,
            onRoundingDiscrepancy: ({ userInput, calculated }) =>
                showRoundingWarning(userInput, calculated),
            onCalculationCleared: clearRoundingWarning,
        }
    );
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
    // Back button
    const backBtn = document.getElementById('back-btn');
    if (backBtn) {
        backBtn.addEventListener('click', () => {
            window.location.href = HTML_FILES.TRANSACTION_MANAGEMENT;
        });
    }

    // Add detail button
    const addDetailBtn = document.getElementById('add-detail-btn');
    if (addDetailBtn) {
        addDetailBtn.addEventListener('click', async () => {
            await openDetailModal();
        });
    }

    // "Open in product master" button: jumps to product master while
    // preserving the in-flight modal state via sessionStorage so the user can
    // come back to the half-filled detail row after registering the product.
    const openProductMasterBtn = document.getElementById('open-product-master-btn');
    if (openProductMasterBtn) {
        openProductMasterBtn.addEventListener('click', () => {
            const draft = buildDraftFromForm();
            persistDraft(draft);
            const params = new URLSearchParams();
            if (draft.item_name) params.set('prefill_name', draft.item_name);
            if (transactionId) params.set('return_to', transactionId);
            const qs = params.toString();
            window.location.href = HTML_FILES.PRODUCT_MANAGEMENT + (qs ? '?' + qs : '');
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
    
    // Tax calculation listeners (shared module)
    installTaxCalculationListeners();

    // Live-clear validation errors as the user edits + character counters
    const itemNameInput = document.getElementById('item-name');
    const memoInput = document.getElementById('memo');
    itemNameInput?.addEventListener('input', () => clearValidationError(itemNameInput));
    memoInput?.addEventListener('input', () => clearValidationError(memoInput));
    if (itemNameInput) attachCharCounter(itemNameInput, MAX_ITEM_NAME_LEN);
    if (memoInput) attachCharCounter(memoInput, MAX_MEMO_LEN);

    // Product autocomplete (v2.6.0)
    installProductAutocomplete();
    
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

        // Get transaction header with related info (account names, shop name)
        const header = await invoke('get_transaction_header_with_info', {
            transactionId: parseInt(transactionId)
        });

        if (!header) {
            throw new Error('Transaction header not found');
        }

        // Store CATEGORY1_CODE and TAX_ROUNDING_TYPE for detail operations
        category1Code = header.category1_code;
        taxRoundingType = header.tax_rounding_type ?? 0; // Default to floor (0) if not set
        currentHeaderTotal = header.total_amount ?? 0; // Track for drift detection on detail edits

        // Display header information
        document.getElementById('header-transaction-date').textContent = header.transaction_date || '-';

        // Display account based on category1_code
        // EXPENSE: from_account (出金元), INCOME: to_account (入金先), TRANSFER: from → to
        let accountDisplay = '-';
        if (header.category1_code === 'TRANSFER') {
            const from = header.from_account_name || header.from_account_code || '-';
            const to = header.to_account_name || header.to_account_code || '-';
            accountDisplay = `${from} → ${to}`;
        } else if (header.category1_code === 'INCOME') {
            accountDisplay = header.to_account_name || header.to_account_code || '-';
        } else {
            // EXPENSE or other
            accountDisplay = header.from_account_name || header.from_account_code || '-';
        }
        document.getElementById('header-account').textContent = accountDisplay;

        document.getElementById('header-shop').textContent = header.shop_name || '-';
        document.getElementById('header-total-amount').textContent = header.total_amount
            ? `¥${header.total_amount.toLocaleString()}`
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

// ============================================================================
// Product autocomplete (v2.6.0 master integration)
// ============================================================================

const AUTOCOMPLETE_DEBOUNCE_MS = 180;

function installProductAutocomplete() {
    const input = document.getElementById('item-name');
    const dropdown = document.getElementById('product-autocomplete-dropdown');
    if (!input || !dropdown) return;

    input.addEventListener('input', handleAutocompleteInput);
    input.addEventListener('keydown', handleAutocompleteKeydown);
    input.addEventListener('blur', handleAutocompleteBlur);
    // mousedown (not click) so the input's blur doesn't fire first and hide
    // the dropdown before we can read which item was clicked.
    dropdown.addEventListener('mousedown', handleAutocompleteMousedown);
}

function handleAutocompleteInput(event) {
    // Any keystroke after a selection means the user is overwriting the
    // master link; demote back to free-text until they pick again.
    autocompleteState.selectedProductId = setHiddenProductId(null);

    const query = event.target.value.trim();
    if (!query) {
        hideAutocomplete();
        return;
    }

    clearTimeout(autocompleteState.debounceTimer);
    autocompleteState.debounceTimer = setTimeout(() => fetchAndRenderCandidates(query), AUTOCOMPLETE_DEBOUNCE_MS);
}

async function fetchAndRenderCandidates(query) {
    const token = ++autocompleteState.requestToken;
    try {
        const results = await invoke('search_products_by_name', { query });
        if (token !== autocompleteState.requestToken) return; // stale
        renderAutocomplete(results || []);
    } catch (err) {
        console.error('search_products_by_name failed:', err);
        if (token === autocompleteState.requestToken) hideAutocomplete();
    }
}

function renderAutocomplete(candidates) {
    const dropdown = document.getElementById('product-autocomplete-dropdown');
    if (!dropdown) return;

    autocompleteState.candidates = candidates;
    autocompleteState.activeIndex = -1;

    if (candidates.length === 0) {
        dropdown.innerHTML = `<div class="product-autocomplete-empty">${escapeHtml(i18n.t('detail_mgmt.autocomplete_no_match'))}</div>`;
        dropdown.classList.remove('hidden');
        return;
    }

    dropdown.innerHTML = candidates.map((c, idx) => {
        const manuf = c.manufacturer_name
            ? `<span class="product-autocomplete-item__manufacturer">(${escapeHtml(c.manufacturer_name)})</span>`
            : '';
        return `<div class="product-autocomplete-item" data-idx="${idx}" role="option">
            <span class="product-autocomplete-item__name">${escapeHtml(c.product_name)}</span>${manuf}
        </div>`;
    }).join('');
    dropdown.classList.remove('hidden');
}

function handleAutocompleteKeydown(event) {
    const dropdown = document.getElementById('product-autocomplete-dropdown');
    if (!dropdown || dropdown.classList.contains('hidden')) return;
    if (autocompleteState.candidates.length === 0) return;

    switch (event.key) {
        case 'ArrowDown':
            event.preventDefault();
            moveActive(1);
            break;
        case 'ArrowUp':
            event.preventDefault();
            moveActive(-1);
            break;
        case 'Enter':
            if (autocompleteState.activeIndex >= 0) {
                event.preventDefault();
                selectCandidate(autocompleteState.activeIndex);
            }
            break;
        case 'Escape':
            event.preventDefault();
            hideAutocomplete();
            break;
    }
}

function moveActive(delta) {
    const n = autocompleteState.candidates.length;
    if (n === 0) return;
    let next = autocompleteState.activeIndex + delta;
    if (next < 0) next = n - 1;
    if (next >= n) next = 0;
    autocompleteState.activeIndex = next;

    const dropdown = document.getElementById('product-autocomplete-dropdown');
    if (!dropdown) return;
    dropdown.querySelectorAll('.product-autocomplete-item').forEach((el, idx) => {
        el.classList.toggle('product-autocomplete-item--active', idx === next);
        if (idx === next) el.scrollIntoView({ block: 'nearest' });
    });
}

function handleAutocompleteMousedown(event) {
    const target = event.target.closest('.product-autocomplete-item');
    if (!target) return;
    event.preventDefault(); // keep input focus
    const idx = parseInt(target.dataset.idx, 10);
    if (!Number.isNaN(idx)) selectCandidate(idx);
}

function selectCandidate(index) {
    const candidate = autocompleteState.candidates[index];
    if (!candidate) return;

    const input = document.getElementById('item-name');
    if (input) {
        input.value = candidate.product_name;
        clearValidationError(input);
        // Notify char counter / validators of the programmatic change
        input.dispatchEvent(new Event('input', { bubbles: false }));
    }
    autocompleteState.selectedProductId = setHiddenProductId(candidate.product_id);
    hideAutocomplete();
}

function handleAutocompleteBlur() {
    // Defer hide so a mousedown on a dropdown item still fires before we
    // tear the dropdown down. mousedown handler also preventDefaults blur.
    setTimeout(hideAutocomplete, 120);
}

function hideAutocomplete() {
    const dropdown = document.getElementById('product-autocomplete-dropdown');
    if (dropdown) {
        dropdown.classList.add('hidden');
        dropdown.innerHTML = '';
    }
    autocompleteState.candidates = [];
    autocompleteState.activeIndex = -1;
}

function resetAutocompleteState() {
    clearTimeout(autocompleteState.debounceTimer);
    autocompleteState.debounceTimer = null;
    autocompleteState.requestToken++;
    autocompleteState.selectedProductId = setHiddenProductId(null);
    hideAutocomplete();
}

// Mirror the PRODUCT_ID into the hidden input. Returns the value so callers
// can also keep the module variable in sync in a single expression.
function setHiddenProductId(value) {
    const hidden = document.getElementById('product-id');
    if (hidden) hidden.value = value == null ? '' : String(value);
    return value;
}

async function openDetailModal(detail = null) {
    const modal = document.getElementById('detail-modal');
    const modalTitle = document.getElementById('modal-title');
    const detailForm = document.getElementById('detail-form');
    
    if (!modal || !modalTitle || !detailForm) return;
    
    // Reset form
    detailForm.reset();
    document.getElementById('detail-id').value = '';
    resetAutocompleteState();

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
        // Restore PRODUCT_ID link if this row was tied to a master entry
        if (detail.product_id != null) {
            autocompleteState.selectedProductId = setHiddenProductId(detail.product_id);
        }
        
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

    // Reset modal content scroll so the form header is visible on every open.
    // .modal-content has `overflow-y: auto`, and `.hidden` only toggles
    // display:none — the inner scrollTop survives close/re-open, which made
    // the 2nd+ open inherit the previous scroll position.
    const modalContent = modal.querySelector('.modal-content');
    if (modalContent) modalContent.scrollTop = 0;

    // Clear validation errors and refresh char counters after programmatic
    // value changes (form.reset() / direct .value assignments do not fire 'input').
    const itemNameInput = document.getElementById('item-name');
    const memoInput = document.getElementById('memo');
    clearValidationError(itemNameInput);
    clearValidationError(memoInput);
    itemNameInput?.dispatchEvent(new Event('input'));
    memoInput?.dispatchEvent(new Event('input'));

    // Focus on item name input after modal opens (preventScroll to avoid modal shifting)
    setTimeout(() => document.getElementById('item-name')?.focus({ preventScroll: true }), 0);
}

function closeDetailModal() {
    const modal = document.getElementById('detail-modal');
    if (modal) {
        modal.classList.add('hidden');
    }
}

// Populate the detail modal from a draft saved before the user jumped to
// product-management. Mirrors the value assignments in openDetailModal()'s
// edit branch but works for both new and existing-row drafts.
export async function restoreDraftIntoModal(draft) {
    await openDetailModal();

    if (draft.detail_id) {
        document.getElementById('detail-id').value = draft.detail_id;
        const modalTitle = document.getElementById('modal-title');
        if (modalTitle) {
            modalTitle.setAttribute('data-i18n', 'detail_mgmt.edit_detail');
            modalTitle.textContent = i18n.t('detail_mgmt.edit_detail');
        }
    }

    document.getElementById('item-name').value = draft.item_name || '';
    if (draft.selected_product_id != null) {
        autocompleteState.selectedProductId = setHiddenProductId(draft.selected_product_id);
    }
    if (draft.category2_code) {
        document.getElementById('category2-code').value = draft.category2_code;
        await loadCategory3Options(draft.category2_code);
        document.getElementById('category3-code').value = draft.category3_code || '';
    }
    document.getElementById('tax-rate').value = draft.tax_rate || '10';
    document.getElementById('amount-excluding-tax').value = draft.amount_excluding_tax || '';
    document.getElementById('amount-including-tax').value = draft.amount_including_tax || '';
    document.getElementById('tax-amount').value = draft.tax_amount || '';
    document.getElementById('memo').value = draft.memo || '';
}

async function handleDetailFormSubmit(event) {
    event.preventDefault();

    const detailId = document.getElementById('detail-id').value;
    const itemNameInput = document.getElementById('item-name');
    const memoInput = document.getElementById('memo');
    const itemName = itemNameInput.value.trim();
    const category1Code = document.getElementById('category1-code').value;
    const category2Code = document.getElementById('category2-code').value;
    const category3Code = document.getElementById('category3-code').value;
    const amountExcludingTax = parseInt(document.getElementById('amount-excluding-tax').value) || 0;
    const amountIncludingTax = parseInt(document.getElementById('amount-including-tax').value) || 0;
    const taxRate = parseInt(document.getElementById('tax-rate').value) || 0;
    const taxAmount = parseInt(document.getElementById('tax-amount').value) || 0;
    const memo = memoInput.value.trim();

    clearValidationError(itemNameInput);
    clearValidationError(memoInput);

    // Validation
    if (!itemName) {
        showValidationError(itemNameInput, i18n.t('detail_mgmt.error_item_name_required'));
        return;
    }

    // Validation — max length (mirrors Rust defense in src/services/transaction.rs)
    if ([...itemName].length > MAX_ITEM_NAME_LEN) {
        showMaxLengthError(itemNameInput, i18n.t('detail_mgmt.item_name'), MAX_ITEM_NAME_LEN);
        return;
    }
    if (memo && [...memo].length > MAX_MEMO_LEN) {
        showMaxLengthError(memoInput, i18n.t('detail_mgmt.memo'), MAX_MEMO_LEN);
        return;
    }

    if (!category1Code) {
        showMessage('error', i18n.t('detail_mgmt.error_category_required'));
        return;
    }

    if (amountExcludingTax < 0 || amountIncludingTax < 0) {
        showMessage('error', i18n.t('detail_mgmt.error_invalid_amount'));
        return;
    }
    
    try {
        const productId = autocompleteState.selectedProductId;
        if (detailId) {
            // Update existing detail
            await invoke('update_transaction_detail', {
                detailId: parseInt(detailId),
                category1Code: category1Code,
                category2Code: category2Code || null,
                category3Code: category3Code || null,
                itemName: itemName,
                amount: amountExcludingTax,
                amountIncludingTax: amountIncludingTax,
                taxRate: taxRate,
                taxAmount: taxAmount,
                productId: productId,
                memo: memo || null
            });
        } else {
            // Add new detail
            await invoke('add_transaction_detail', {
                transactionId: parseInt(transactionId),
                category1Code: category1Code,
                category2Code: category2Code || null,
                category3Code: category3Code || null,
                itemName: itemName,
                amount: amountExcludingTax,
                amountIncludingTax: amountIncludingTax,
                taxRate: taxRate,
                taxAmount: taxAmount,
                productId: productId,
                memo: memo || null
            });
        }

        // Reconcile cached header total with the new detail set. Prompts the
        // user if a drift was introduced; either way, the subsequent
        // loadTransactionHeader() refreshes the display.
        const recalcResult = await applyHeaderRecalculationPrompt(transactionId, currentHeaderTotal);
        if (recalcResult.applied && recalcResult.recommended !== null) {
            currentHeaderTotal = recalcResult.recommended;
        }

        closeDetailModal();
        showMessage('success', i18n.t('detail_mgmt.save_success'));
        await loadDetails();
        await loadTransactionHeader(); // Refresh total amount
        
    } catch (error) {
        console.error('Failed to save detail:', error);

        // Map backend error messages to i18n resources / localized text.
        // Rust defense line for bounded fields (src/services/transaction.rs).
        const errorMessage = error.toString();
        if (errorMessage.includes('Item name must be')) {
            showValidationError(itemNameInput, i18n.t('validation.max_length', {
                field: i18n.t('detail_mgmt.item_name'),
                max: MAX_ITEM_NAME_LEN,
                actual: [...itemName].length,
            }));
            return;
        }
        if (errorMessage.includes('Memo must be')) {
            showValidationError(memoInput, i18n.t('validation.max_length', {
                field: i18n.t('detail_mgmt.memo'),
                max: MAX_MEMO_LEN,
                actual: [...memo].length,
            }));
            return;
        }

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
            detailId: parseInt(detailId)
        });

        // Same recalculation handshake the save flow uses — a delete can also
        // change the recommended header total.
        const recalcResult = await applyHeaderRecalculationPrompt(transactionId, currentHeaderTotal);
        if (recalcResult.applied && recalcResult.recommended !== null) {
            currentHeaderTotal = recalcResult.recommended;
        }

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
