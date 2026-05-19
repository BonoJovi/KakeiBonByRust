import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { Modal } from './modal.js';
import { setupIndicators } from './indicators.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';
import { showValidationError, clearValidationError, showMaxLengthError, attachCharCounter } from './validation-display.js';
import { showToast } from './toast.js';
import { MAX_NAME_LEN, MAX_MEMO_LEN } from './consts.js';

console.log('=== MANUFACTURER-MANAGEMENT.JS LOADED ===');

let currentUserId = null;
let currentUserRole = null;

let currentLanguage = 'ja';
let manufacturers = [];
let editingManufacturerId = null;
let manufacturerModal = null;
let deleteModal = null;
let manufacturerToDelete = null;
let showDisabledItems = false;

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

        initManufacturerModal();
        initDeleteModal();
        setupIndicators();
        setupEventListeners();
        await loadManufacturers();

        // Adjust window size after content is loaded
        await adjustWindowSize();
    } catch (error) {
        console.error('Initialization error:', error);
        showToast(i18n.t('manufacturer_mgmt.failed_to_initialize') + ': ' + error, { variant: 'error' });
    }
});

function initManufacturerModal() {
    manufacturerModal = new Modal('manufacturer-modal', {
        formId: 'manufacturer-form',
        closeButtonId: 'close-modal',
        cancelButtonId: 'cancel-btn',
        onOpen: (mode, data) => {
            const modalTitle = document.getElementById('modal-title');
            const form = document.getElementById('manufacturer-form');
            const manufacturerNameInput = document.getElementById('manufacturer-name');
            const manufacturerMemoInput = document.getElementById('manufacturer-memo');

            // Clear form and errors
            form.reset();
            clearErrors();
            clearValidationError(manufacturerNameInput);
            clearValidationError(manufacturerMemoInput);

            if (mode === 'add') {
                modalTitle.setAttribute('data-i18n', 'manufacturer_mgmt.add');
                modalTitle.textContent = i18n.t('manufacturer_mgmt.add');
                editingManufacturerId = null;
                document.getElementById('manufacturer-is-disabled').checked = false;
            } else if (mode === 'edit') {
                modalTitle.setAttribute('data-i18n', 'manufacturer_mgmt.edit');
                modalTitle.textContent = i18n.t('manufacturer_mgmt.edit');

                // Populate form
                manufacturerNameInput.value = data.manufacturer_name;
                manufacturerMemoInput.value = data.memo || '';
                document.getElementById('manufacturer-is-disabled').checked = data.is_disabled === 1;

                editingManufacturerId = data.manufacturer_id;
            }

            // Refresh character counters after programmatic value changes
            // (form.reset() / direct .value assignments do not fire 'input').
            manufacturerNameInput?.dispatchEvent(new Event('input'));
            manufacturerMemoInput?.dispatchEvent(new Event('input'));
        },
        onSave: async (formData) => {
            await saveManufacturer();
        },
        onClose: () => {
            editingManufacturerId = null;
        }
    });
}

function initDeleteModal() {
    deleteModal = new Modal('delete-modal', {
        closeButtonId: 'close-delete-modal',
        cancelButtonId: 'cancel-delete-btn',
        onOpen: (mode, manufacturer) => {
            const manufacturerNameDisplay = document.getElementById('delete-manufacturer-name');
            manufacturerNameDisplay.textContent = manufacturer.manufacturer_name;
            manufacturerToDelete = manufacturer;
        },
        onClose: () => {
            manufacturerToDelete = null;
        }
    });

    // Confirm delete button
    document.getElementById('confirm-delete-btn').addEventListener('click', async () => {
        if (manufacturerToDelete) {
            await deleteManufacturer(manufacturerToDelete.manufacturer_id);
            deleteModal.close();
        }
    });
}

function setupEventListeners() {
    // Add manufacturer button
    document.getElementById('add-manufacturer-btn').addEventListener('click', () => {
        openModal('add');
    });

    // Toggle disabled items button
    document.getElementById('toggle-disabled-btn').addEventListener('click', () => {
        showDisabledItems = !showDisabledItems;
        updateToggleButton();
        loadManufacturers();
    });

    // Live-clear validation errors as the user edits
    const manufacturerNameInput = document.getElementById('manufacturer-name');
    const manufacturerMemoInput = document.getElementById('manufacturer-memo');
    manufacturerNameInput?.addEventListener('input', () => clearValidationError(manufacturerNameInput));
    manufacturerMemoInput?.addEventListener('input', () => clearValidationError(manufacturerMemoInput));

    // Live character counters (kept in sync with backend chars().count())
    if (manufacturerNameInput) attachCharCounter(manufacturerNameInput, MAX_NAME_LEN);
    if (manufacturerMemoInput) attachCharCounter(manufacturerMemoInput, MAX_MEMO_LEN);
}

function openModal(mode, data = null) {
    manufacturerModal.open(mode, data);
}

function openDeleteModal(manufacturer) {
    deleteModal.open('delete', manufacturer);
}

function clearErrors() {
    document.querySelectorAll('.error-message').forEach(el => {
        el.textContent = '';
    });
}

function updateToggleButton() {
    const btn = document.getElementById('toggle-disabled-btn');
    if (showDisabledItems) {
        btn.setAttribute('data-i18n', 'common.hide_disabled');
        btn.textContent = i18n.t('common.hide_disabled');
    } else {
        btn.setAttribute('data-i18n', 'common.show_disabled');
        btn.textContent = i18n.t('common.show_disabled');
    }
}

async function loadManufacturers() {
    const loading = document.getElementById('loading');
    const table = document.getElementById('manufacturers-table');

    try {
        loading.style.display = 'block';
        table.style.display = 'none';

        console.log('Loading manufacturers, includeDisabled:', showDisabledItems);
        manufacturers = await invoke('get_manufacturers', {
            includeDisabled: showDisabledItems
        });
        console.log('Loaded manufacturers:', manufacturers);

        renderManufacturers();

        loading.style.display = 'none';
        table.style.display = 'table';
    } catch (error) {
        console.error('Failed to load manufacturers:', error);
        loading.textContent = i18n.t('manufacturer_mgmt.failed_to_load') + ': ' + error;
    }
}

function renderManufacturers() {
    const tbody = document.getElementById('manufacturers-tbody');
    tbody.innerHTML = '';

    if (manufacturers.length === 0) {
        const row = tbody.insertRow();
        const cell = row.insertCell();
        cell.colSpan = 3;
        cell.style.textAlign = 'center';
        cell.style.padding = '20px';
        cell.style.color = '#999';
        cell.setAttribute('data-i18n', 'manufacturer_mgmt.no_data');
        cell.textContent = i18n.t('manufacturer_mgmt.no_data');
        return;
    }

    manufacturers.forEach(manufacturer => {
        const row = tbody.insertRow();

        // Apply styling for disabled items
        const isDisabled = manufacturer.is_disabled === 1;
        if (isDisabled) {
            row.style.backgroundColor = '#6c757d';  // Medium gray background
            // Note: No opacity - keeps buttons clearly visible
        }

        // Manufacturer Name
        const nameCell = row.insertCell();
        if (isDisabled) {
            // Add [非表示] badge for disabled items
            const badge = `<span style="color: #ffc107; font-weight: bold; margin-left: 8px;">[${i18n.t('common.disabled_label')}]</span>`;
            nameCell.innerHTML = `<span style="color: #ffffff;">${manufacturer.manufacturer_name}</span>${badge}`;
        } else {
            nameCell.textContent = manufacturer.manufacturer_name;
        }

        // Memo
        const memoCell = row.insertCell();
        memoCell.textContent = manufacturer.memo || '-';
        if (isDisabled) {
            memoCell.style.color = '#ffffff';  // White text for disabled items
        } else {
            memoCell.style.color = manufacturer.memo ? '#212529' : '#999';
        }

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
            openModal('edit', manufacturer);
        });
        actionsDiv.appendChild(editBtn);

        // Delete button
        const deleteBtn = document.createElement('button');
        deleteBtn.className = 'btn-small btn-delete';
        deleteBtn.setAttribute('data-i18n', 'common.delete');
        deleteBtn.textContent = i18n.t('common.delete');
        deleteBtn.addEventListener('click', () => {
            openDeleteModal(manufacturer);
        });
        actionsDiv.appendChild(deleteBtn);
        
        actionsCell.appendChild(actionsDiv);
    });
}

async function saveManufacturer() {
    const manufacturerNameInput = document.getElementById('manufacturer-name');
    const manufacturerMemoInput = document.getElementById('manufacturer-memo');
    const manufacturerName = manufacturerNameInput.value.trim();
    const memo = manufacturerMemoInput.value.trim();
    const isDisabled = document.getElementById('manufacturer-is-disabled').checked ? 1 : 0;

    // Clear previous errors
    clearErrors();
    clearValidationError(manufacturerNameInput);
    clearValidationError(manufacturerMemoInput);

    // Validation — empty name
    if (!manufacturerName) {
        showValidationError(manufacturerNameInput, i18n.t('manufacturer_mgmt.empty_name'));
        throw new Error('Validation error: empty manufacturer name');
    }

    // Validation — max length (mirrors Rust defense in src/services/manufacturer.rs)
    if ([...manufacturerName].length > MAX_NAME_LEN) {
        showMaxLengthError(manufacturerNameInput, i18n.t('manufacturer_mgmt.name'), MAX_NAME_LEN);
        throw new Error('Validation error: manufacturer name too long');
    }
    if (memo && [...memo].length > MAX_MEMO_LEN) {
        showMaxLengthError(manufacturerMemoInput, i18n.t('manufacturer_mgmt.memo'), MAX_MEMO_LEN);
        throw new Error('Validation error: memo too long');
    }

    try {
        if (editingManufacturerId) {
            // Update existing manufacturer
            const manufacturer = manufacturers.find(m => m.manufacturer_id === editingManufacturerId);
            await invoke('update_manufacturer', {
                manufacturerId: editingManufacturerId,
                manufacturerName: manufacturerName,
                memo: memo || null,
                displayOrder: manufacturer.display_order,
                isDisabled: isDisabled
            });
            console.log('Manufacturer updated successfully');
        } else {
            // Add new manufacturer
            await invoke('add_manufacturer', {
                manufacturerName: manufacturerName,
                memo: memo || null,
                isDisabled: isDisabled === 1 ? isDisabled : null
            });
            console.log('Manufacturer added successfully');
        }

        // Reload manufacturers list (modal will be closed by Modal class)
        await loadManufacturers();
    } catch (error) {
        console.error('Failed to save manufacturer:', error);

        // Map backend error messages to i18n resources / localized text
        const errorMessage = error.toString();
        let nameMessage = null;
        let memoMessage = null;

        if (errorMessage.includes('already exists')) {
            nameMessage = i18n.t('manufacturer_mgmt.duplicate_error');
        } else if (errorMessage.includes('Manufacturer name must be')) {
            // Defense-line trip: frontend max-length check should have caught
            // this, so use the same i18n message for parity.
            nameMessage = i18n.t('validation.max_length', {
                field: i18n.t('manufacturer_mgmt.name'),
                max: MAX_NAME_LEN,
                actual: [...manufacturerName].length,
            });
        } else if (errorMessage.includes('Memo must be')) {
            memoMessage = i18n.t('validation.max_length', {
                field: i18n.t('manufacturer_mgmt.memo'),
                max: MAX_MEMO_LEN,
                actual: [...memo].length,
            });
        } else if (errorMessage.includes('cannot be empty')) {
            nameMessage = i18n.t('manufacturer_mgmt.empty_name');
        } else {
            // Fallback to original error message on the name field
            nameMessage = errorMessage;
        }

        if (nameMessage) showValidationError(manufacturerNameInput, nameMessage);
        if (memoMessage) showValidationError(manufacturerMemoInput, memoMessage);

        // Re-throw error to prevent modal from closing
        throw error;
    }
}

async function deleteManufacturer(manufacturerId) {
    try {
        await invoke('delete_manufacturer', {
            manufacturerId: manufacturerId
        });
        console.log('Manufacturer deleted successfully');
        await loadManufacturers();
    } catch (error) {
        console.error('Failed to delete manufacturer:', error);
        showToast(i18n.t('manufacturer_mgmt.failed_to_delete') + ': ' + error, { variant: 'error' });
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
        await loadManufacturers();
    } catch (error) {
        console.error('Failed to change language:', error);
    }
}
