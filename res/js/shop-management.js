import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { Modal } from './modal.js';
import { setupIndicators } from './indicators.js';
import { getCurrentSessionUser, isSessionAuthenticated, getSessionSourceScreen, clearSessionSourceScreen } from './session.js';
import { createMenuBar } from './menu.js';
import { showValidationError, clearValidationError, showMaxLengthError, attachCharCounter } from './validation-display.js';
import { showToast } from './toast.js';
import { MAX_NAME_LEN, MAX_MEMO_LEN } from './consts.js';

console.log('=== SHOP-MANAGEMENT.JS LOADED ===');

let currentUserId = null;
let currentUserRole = null;

let currentLanguage = 'ja';
let shops = [];
let editingShopId = null;
let shopModal = null;
let deleteModal = null;
let shopToDelete = null;

// Initialize
document.addEventListener('DOMContentLoaded', async () => {
    
    // Create menu bar
    createMenuBar('management');
    console.log('DOMContentLoaded fired');
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
        
        await i18n.init();
        console.log('i18n initialized:', i18n.initialized);
        currentLanguage = i18n.getCurrentLanguage();
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

        initShopModal();
        initDeleteModal();
        setupIndicators();
        setupEventListeners();
        await loadShops();

        // Adjust window size after content is loaded
        await adjustWindowSize();
    } catch (error) {
        console.error('Initialization error:', error);
        showToast(i18n.t('shop_mgmt.failed_to_initialize') + ': ' + error, { variant: 'error' });
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
    document.getElementById('confirm-delete-btn').addEventListener('click', async () => {
        if (shopToDelete) {
            await deleteShop(shopToDelete.shop_id);
            deleteModal.close();
        }
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
        loading.textContent = i18n.t('shop_mgmt.failed_to_load') + ': ' + error;
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
    const shopNameInput = document.getElementById('shop-name');
    const memoInput = document.getElementById('shop-memo');
    const shopName = shopNameInput.value.trim();
    const memo = memoInput.value.trim();

    // Clear previous errors
    clearErrors();
    clearValidationError(shopNameInput);
    clearValidationError(memoInput);

    // Validation — empty name
    if (!shopName) {
        showValidationError(shopNameInput, i18n.t('shop_mgmt.empty_name'));
        throw new Error('Validation error: empty shop name');
    }

    // Validation — max length (mirrors Rust defense in src/services/shop.rs)
    if ([...shopName].length > MAX_NAME_LEN) {
        showMaxLengthError(shopNameInput, i18n.t('shop_mgmt.shop_name'), MAX_NAME_LEN);
        throw new Error('Validation error: shop name too long');
    }
    if (memo && [...memo].length > MAX_MEMO_LEN) {
        showMaxLengthError(memoInput, i18n.t('shop_mgmt.memo'), MAX_MEMO_LEN);
        throw new Error('Validation error: memo too long');
    }

    try {
        if (editingShopId) {
            // Update existing shop
            const shop = shops.find(s => s.shop_id === editingShopId);
            await invoke('update_shop', {
                shopId: editingShopId,
                shopName: shopName,
                memo: memo || null,
                displayOrder: shop.display_order
            });
            console.log('Shop updated successfully');
        } else {
            // Add new shop
            await invoke('add_shop', {
                shopName: shopName,
                memo: memo || null
            });
            console.log('Shop added successfully');
        }

        // Reload shops list (modal will be closed by Modal class)
        await loadShops();
        
        // Check if called from another screen
        const callerScreen = await getSessionSourceScreen();
        if (callerScreen) {
            // Clear session and return to caller screen
            await clearSessionSourceScreen();
            window.location.href = HTML_FILES.TRANSACTION_MANAGEMENT;
        }
    } catch (error) {
        console.error('Failed to save shop:', error);

        // Map backend error messages to i18n resources / localized text
        const errorMessage = error.toString();
        let nameMessage = null;
        let memoMessage = null;

        if (errorMessage.includes('already exists')) {
            nameMessage = i18n.t('shop_mgmt.duplicate_error');
        } else if (errorMessage.includes('Shop name must be')) {
            // Defense-line trip: frontend max-length check should have caught
            // this, so use the same i18n message for parity.
            nameMessage = i18n.t('validation.max_length', {
                field: i18n.t('shop_mgmt.shop_name'),
                max: MAX_NAME_LEN,
                actual: [...shopName].length,
            });
        } else if (errorMessage.includes('Memo must be')) {
            memoMessage = i18n.t('validation.max_length', {
                field: i18n.t('shop_mgmt.memo'),
                max: MAX_MEMO_LEN,
                actual: [...memo].length,
            });
        } else if (errorMessage.includes('cannot be empty')) {
            nameMessage = i18n.t('shop_mgmt.empty_name');
        } else {
            // Fallback to original error message on the name field
            nameMessage = errorMessage;
        }

        if (nameMessage) showValidationError(shopNameInput, nameMessage);
        if (memoMessage) showValidationError(memoInput, memoMessage);

        // Re-throw error to prevent modal from closing
        throw error;
    }
}

async function deleteShop(shopId) {
    try {
        await invoke('delete_shop', {
            shopId: shopId
        });
        console.log('Shop deleted successfully');
        await loadShops();
    } catch (error) {
        console.error('Failed to delete shop:', error);
        showToast(i18n.t('shop_mgmt.failed_to_delete') + ': ' + error, { variant: 'error' });
    }
}

// Menu handlers
function setupMenuHandlers() {
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    if (fileMenu && fileDropdown) {
        if (fileMenu.dataset.initialized === 'true') {
            return;
        }
        
        fileMenu.addEventListener('click', function(e) {
            e.stopPropagation();
            const isShown = fileDropdown.classList.contains('show');
            document.querySelectorAll('.dropdown').forEach(d => {
                if (d !== fileDropdown) d.classList.remove('show');
            });
            if (!isShown) fileDropdown.classList.add('show');
        });
        
        fileDropdown.addEventListener('click', function(e) {
            e.stopPropagation();
        });
        
        const dropdownItems = fileDropdown.querySelectorAll('.dropdown-item');
        dropdownItems[0]?.addEventListener('click', () => {
            window.location.href = HTML_FILES.INDEX;
            fileDropdown.classList.remove('show');
        });
        dropdownItems[1]?.addEventListener('click', async () => {
            await invoke('logout');
            window.location.href = HTML_FILES.INDEX;
            fileDropdown.classList.remove('show');
        });
        dropdownItems[2]?.addEventListener('click', async () => {
            await invoke('quit_app');
            fileDropdown.classList.remove('show');
        });
        
        fileMenu.dataset.initialized = 'true';
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

function setupLanguageMenuHandlers() {
    const languageMenu = document.getElementById('language-menu');
    const languageDropdown = document.getElementById('language-dropdown');
    
    if (!languageMenu || !languageDropdown) return;
    if (languageMenu.dataset.initialized === 'true') return;
    
    languageMenu.addEventListener('click', function(e) {
        e.stopPropagation();
        const isShown = languageDropdown.classList.contains('show');
        document.querySelectorAll('.dropdown').forEach(d => {
            if (d !== languageDropdown) d.classList.remove('show');
        });
        if (!isShown) languageDropdown.classList.add('show');
    });
    
    languageDropdown.addEventListener('click', function(e) {
        e.stopPropagation();
    });
    
    languageMenu.dataset.initialized = 'true';
}

async function setupLanguageMenu() {
    try {
        const languageNames = await invoke('get_language_names');
        const currentLang = i18n.getCurrentLanguage();
        const languageDropdown = document.getElementById('language-dropdown');
        
        if (!languageDropdown) return;
        
        languageDropdown.innerHTML = '';
        
        for (const [langCode, langName] of languageNames) {
            const item = document.createElement('div');
            item.className = 'dropdown-item';
            item.textContent = langName;
            item.dataset.langCode = langCode;
            
            if (langCode === currentLang) {
                item.classList.add('active');
            }
            
            item.addEventListener('click', async function(e) {
                e.stopPropagation();
                await handleLanguageChange(langCode);
                languageDropdown.classList.remove('show');
            });
            
            languageDropdown.appendChild(item);
        }
    } catch (error) {
        console.error('Failed to setup language menu:', error);
    }
}

async function handleLanguageChange(langCode) {
    try {
        await i18n.setLanguage(langCode);
        await setupLanguageMenu();
        // Font Size submenu items are built via textContent (no data-i18n),
        // so an explicit redraw is needed after language change.
        await setupFontSizeMenu();
        await loadShops();
    } catch (error) {
        console.error('Failed to change language:', error);
    }
}
