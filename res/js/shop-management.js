import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';
import { setupLanguageMenu, setupLanguageMenuHandlers } from './language-menu.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';
import { fitWindowToScreen } from './window-fit.js';
import { Modal } from './modal.js';
import { setupIndicators } from './indicators.js';
import { getCurrentSessionUser, getSessionSourceScreen, clearSessionSourceScreen, clearSessionModalState, clearSessionCategory1Code } from './session.js';
import { createMenuBar, handleLogout, handleQuit } from './menu.js';
import { clearValidationError, attachCharCounter } from './validation-display.js';
import { showToast } from './toast.js';
import { MAX_NAME_LEN, MAX_MEMO_LEN, SOURCE_SCREEN_TRANSACTION_MGMT } from './consts.js';
import { saveMasterEntry, API_ERROR_CODES, formatApiError } from './master-crud.js';

console.log('=== SHOP-MANAGEMENT.JS LOADED ===');

let currentUserId = null;
let currentUserRole = null;

let currentLanguage = 'ja';
let shops = [];
let editingShopId = null;
let shopModal = null;
let deleteModal = null;
let shopToDelete = null;
// Screen that side-tripped here (captured once at load, cleared from the
// session immediately so it cannot go stale if the user leaves without saving)
let sideTripSource = null;

// Initialize
document.addEventListener('DOMContentLoaded', async () => {
    
    // Create menu bar
    createMenuBar('management');
    console.log('DOMContentLoaded fired');
    try {
        // Check session authentication and get user info in a single call
        const user = await getCurrentSessionUser();
        if (!user) {
            console.error('Not authenticated, redirecting to login');
            window.location.href = HTML_FILES.INDEX;
            return;
        }
        
        currentUserId = user.user_id;
        currentUserRole = user.role;
        console.log(`Logged in as: ${user.name} (ID: ${currentUserId}, Role: ${currentUserRole})`);
        
        sideTripSource = await getSessionSourceScreen();
        if (sideTripSource) {
            await clearSessionSourceScreen();
        }
        
        await i18n.init();
        console.log('i18n initialized:', i18n.initialized);
        currentLanguage = i18n.getCurrentLanguage();
        i18n.updateUI();

        // Setup menu handlers
        setupMenuHandlers();
        
        // Setup language and font size menus
        await setupLanguageMenu(loadShops);
        setupLanguageMenuHandlers();
        
        setupFontSizeMenuHandlers();
        await setupFontSizeMenu();
        setupFontSizeModalHandlers();
        await applyFontSize();

        initShopModal();
        initDeleteModal();
        setupIndicators();
        setupEventListeners();
        await loadShops();

        // Fit + center the window on this monitor
        await fitWindowToScreen();
    } catch (error) {
        console.error('Initialization error:', error);
        showToast(i18n.t('shop_mgmt.failed_to_initialize'), { variant: 'error' });
    }
});

function initShopModal() {
    shopModal = new Modal('shop-modal', {
        formId: 'shop-form',
        closeButtonId: 'close-modal',
        cancelButtonId: 'cancel-btn',
        onOpen: (mode, data) => {
            const modalTitle = document.getElementById('modal-title');
            const form = document.getElementById('shop-form');

            // Clear form and errors
            form.reset();
            clearErrors();
            const shopNameEl = document.getElementById('shop-name');
            const shopMemoEl = document.getElementById('shop-memo');
            clearValidationError(shopNameEl);
            clearValidationError(shopMemoEl);

            if (mode === 'add') {
                modalTitle.setAttribute('data-i18n', 'shop_mgmt.modal_title_add');
                modalTitle.textContent = i18n.t('shop_mgmt.modal_title_add');
                editingShopId = null;
            } else if (mode === 'edit') {
                modalTitle.setAttribute('data-i18n', 'shop_mgmt.modal_title_edit');
                modalTitle.textContent = i18n.t('shop_mgmt.modal_title_edit');

                // Populate form
                shopNameEl.value = data.shop_name;
                shopMemoEl.value = data.memo || '';

                editingShopId = data.shop_id;
            }

            // Refresh character counters after programmatic value changes
            // (form.reset() / direct .value assignments do not fire 'input').
            shopNameEl?.dispatchEvent(new Event('input'));
            shopMemoEl?.dispatchEvent(new Event('input'));
        },
        onSave: async (formData) => {
            await saveShop();
        },
        onClose: () => {
            editingShopId = null;
        }
    });
}

function initDeleteModal() {
    deleteModal = new Modal('delete-modal', {
        closeButtonId: 'close-delete-modal',
        cancelButtonId: 'cancel-delete-btn',
        onOpen: (mode, shop) => {
            const shopNameDisplay = document.getElementById('delete-shop-name');
            shopNameDisplay.textContent = shop.shop_name;
            shopToDelete = shop;
        },
        onClose: () => {
            shopToDelete = null;
        }
    });

    // Confirm delete button
    // PR13 (Fable-5 D8/D9): `deleteShop` now returns a boolean and the
    // dead `confirmDeleteBtn.disabled` re-entry check is gone —
    // see product-management.js for the shared rationale.
    const confirmDeleteBtn = document.getElementById('confirm-delete-btn');
    confirmDeleteBtn.addEventListener('click', async () => {
        if (!shopToDelete) return;
        confirmDeleteBtn.disabled = true;
        const ok = await deleteShop(shopToDelete.shop_id);
        confirmDeleteBtn.disabled = false;
        if (ok) deleteModal.close();
    });
}

function setupEventListeners() {
    // Add shop button
    document.getElementById('add-shop-btn').addEventListener('click', () => {
        openModal('add');
    });

    // Live-clear validation errors as the user edits
    const shopNameInput = document.getElementById('shop-name');
    const memoInput = document.getElementById('shop-memo');
    shopNameInput?.addEventListener('input', () => clearValidationError(shopNameInput));
    memoInput?.addEventListener('input', () => clearValidationError(memoInput));

    // Live character counters (kept in sync with backend chars().count())
    if (shopNameInput) attachCharCounter(shopNameInput, MAX_NAME_LEN);
    if (memoInput) attachCharCounter(memoInput, MAX_MEMO_LEN);
}

function openModal(mode, data = null) {
    shopModal.open(mode, data);
}

function openDeleteModal(shop) {
    deleteModal.open('delete', shop);
}

function clearErrors() {
    document.querySelectorAll('.error-message').forEach(el => {
        el.textContent = '';
    });
}

async function loadShops() {
    const loading = document.getElementById('loading');
    const table = document.getElementById('shops-table');

    try {
        loading.style.display = 'block';
        table.style.display = 'none';

        console.log('Loading shops');
        shops = await invoke('get_shops', {});
        console.log('Loaded shops:', shops);

        renderShops();

        loading.style.display = 'none';
        table.style.display = 'table';
    } catch (error) {
        console.error('Failed to load shops:', error);
        loading.textContent = i18n.t('shop_mgmt.failed_to_load');
    }
}

function renderShops() {
    const tbody = document.getElementById('shops-tbody');
    tbody.innerHTML = '';

    if (shops.length === 0) {
        const row = tbody.insertRow();
        const cell = row.insertCell();
        cell.colSpan = 3;
        cell.style.textAlign = 'center';
        cell.style.padding = '20px';
        cell.style.color = '#999';
        cell.setAttribute('data-i18n', 'shop_mgmt.no_shops');
        cell.textContent = i18n.t('shop_mgmt.no_shops');
        return;
    }

    shops.forEach(shop => {
        const row = tbody.insertRow();

        // Shop Name
        const nameCell = row.insertCell();
        nameCell.textContent = shop.shop_name;

        // Memo
        const memoCell = row.insertCell();
        memoCell.textContent = shop.memo || '-';
        memoCell.style.color = shop.memo ? '#212529' : '#999';

        // Actions
        const actionsCell = row.insertCell();
        const actionsDiv = document.createElement('div');
        actionsDiv.className = 'action-buttons';

        // Edit button
        const editBtn = document.createElement('button');
        editBtn.className = 'btn-small btn-edit';
        editBtn.setAttribute('data-i18n', 'common.edit');
        editBtn.textContent = i18n.t('common.edit');
        editBtn.addEventListener('click', () => {
            openModal('edit', shop);
        });
        actionsDiv.appendChild(editBtn);

        // Delete button
        const deleteBtn = document.createElement('button');
        deleteBtn.className = 'btn-small btn-delete';
        deleteBtn.setAttribute('data-i18n', 'common.delete');
        deleteBtn.textContent = i18n.t('common.delete');
        deleteBtn.addEventListener('click', () => {
            openDeleteModal(shop);
        });
        actionsDiv.appendChild(deleteBtn);
        
        actionsCell.appendChild(actionsDiv);
    });
}

async function saveShop() {
    // Clear "top of form" error area kept for legacy fallbacks; per-field
    // errors are cleared inside saveMasterEntry via clearValidationError.
    clearErrors();

    const shopNameInput = document.getElementById('shop-name');
    const memoInput = document.getElementById('shop-memo');

    const result = await saveMasterEntry({
        nameInput: shopNameInput,
        memoInput,
        editingId: editingShopId,
        findInCacheById: (id) => shops.find(s => s.shop_id === id) || null,
        invokeAdd: (name, memo) => invoke('add_shop', {
            shopName: name,
            memo,
        }),
        invokeUpdate: (target, name, memo) => invoke('update_shop', {
            shopId: editingShopId,
            shopName: name,
            memo,
            displayOrder: target.display_order,
        }),
        i18nPrefix: 'shop_mgmt',
        nameFieldI18nKey: 'shop_mgmt.shop_name',
        memoFieldI18nKey: 'shop_mgmt.memo',
        nameMaxLen: MAX_NAME_LEN,
        memoMaxLen: MAX_MEMO_LEN,
        onNotFoundBeforeInvoke: async () => {
            showToast(i18n.t('shop_mgmt.not_found'), { variant: 'error' });
            await loadShops();
        },
    });

    if (result.mode === 'skip') {
        return;
    }

    // Save succeeded: failures past this point (list reload, side-trip
    // return) must not be reported as a failed save.
    await loadShops();

    if (sideTripSource === SOURCE_SCREEN_TRANSACTION_MGMT) {
        window.location.href = HTML_FILES.TRANSACTION_MANAGEMENT;
    }
}

/// Delete a shop. Returns `true` on success (the confirmation modal
/// should close) or `false` on failure (the modal stays open so the
/// user can retry). PR13 (Fable-5 D8) — see product-management.js
/// for the shared rationale.
async function deleteShop(shopId) {
    try {
        await invoke('delete_shop', {
            shopId: shopId
        });
        console.log('Shop deleted successfully');
        await loadShops();
        return true;
    } catch (error) {
        console.error('Failed to delete shop:', error);
        // Delete-lock: master-delete-lock PR. When the backend reports the
        // shop is still referenced by a transaction or a recurring rule,
        // surface the dedicated "disable instead" toast so the user knows
        // why the removal is refused. Everything else keeps the generic
        // failure toast + `formatApiError` fallback used elsewhere.
        if (error?.code === API_ERROR_CODES.IN_USE) {
            showToast(i18n.t('shop_mgmt.delete_in_use'), { variant: 'error' });
        } else {
            showToast(i18n.t('shop_mgmt.failed_to_delete') + ': ' + formatApiError(error), { variant: 'error' });
        }
        return false;
    }
}

// Menu handlers
function setupMenuHandlers() {
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    if (fileMenu && fileDropdown) {
        if (fileMenu.dataset.initialized !== 'true') {
            fileMenu.addEventListener('click', function(e) {
                e.stopPropagation();
                const isShown = fileDropdown.classList.contains('show');
                document.querySelectorAll('.dropdown').forEach(d => {
                    if (d !== fileDropdown) d.classList.remove('show');
                });
                fileDropdown.classList.toggle('show', !isShown);
            });

            fileDropdown.addEventListener('click', function(e) {
                e.stopPropagation();
            });

            fileMenu.dataset.initialized = 'true';
        }

        if (fileDropdown.dataset.itemsInitialized === 'true') {
            return;
        }
        fileDropdown.dataset.itemsInitialized = 'true';

        const dropdownItems = fileDropdown.querySelectorAll('.dropdown-item');
        dropdownItems[0]?.addEventListener('click', async () => {
            fileDropdown.classList.remove('show');
            if (sideTripSource) {
                // Leaving without saving abandons the side-trip: drop the
                // caller's saved modal state so it is not restored later
                try {
                    await clearSessionModalState();
                    await clearSessionCategory1Code();
                } catch (error) {
                    console.error('Failed to clear side-trip state:', error);
                }
            }
            window.location.href = HTML_FILES.INDEX;
        });
        dropdownItems[1]?.addEventListener('click', () => {
            fileDropdown.classList.remove('show');
            handleLogout();
        });
        dropdownItems[2]?.addEventListener('click', () => {
            fileDropdown.classList.remove('show');
            handleQuit();
        });
    }
    
    if (!document.body.dataset.globalClickHandlerInitialized) {
        document.addEventListener('click', function(e) {
            if (e.target.closest('.menu-item') || e.target.closest('.dropdown')) {
                return;
            }
            document.querySelectorAll('.dropdown').forEach(dropdown => {
                dropdown.classList.remove('show');
            });
        });
        document.body.dataset.globalClickHandlerInitialized = 'true';
    }
}


