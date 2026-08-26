import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';
import { setupLanguageMenu, setupLanguageMenuHandlers } from './language-menu.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';
import { fitWindowToScreen } from './window-fit.js';
import { Modal } from './modal.js';
import { setupIndicators } from './indicators.js';
import { getCurrentSessionUser } from './session.js';
import { createMenuBar, handleLogout, handleQuit } from './menu.js';
import { clearValidationError, attachCharCounter } from './validation-display.js';
import { showToast } from './toast.js';
import { MAX_NAME_LEN, MAX_MEMO_LEN } from './consts.js';
import { escapeHtml } from './escape-html.js';
import { saveMasterEntry, API_ERROR_CODES, formatApiError } from './master-crud.js';

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

// When the user arrives here from the product modal via the
// "Open in manufacturer master" jump, this flag is set so saveManufacturer()
// can write the new manufacturer id back into the persisted product draft,
// and the "Back to product entry" button can be wired up.
let returnToProduct = false;
const PRODUCT_DRAFT_KEY = 'kakeibon.product_draft.v1';

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
        
        await i18n.init();
        console.log('i18n initialized:', i18n.initialized);
        currentLanguage = i18n.getCurrentLanguage();
        i18n.updateUI();

        // Setup menu handlers
        setupMenuHandlers();
        
        // Setup language and font size menus
        await setupLanguageMenu(loadManufacturers);
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

        // Wire up the side-trip back-button if we arrived from the product
        // modal. The product draft itself lives in sessionStorage; we only
        // need the flag here to (a) reveal the button and (b) let
        // saveManufacturer() stamp the new manufacturer id into the draft.
        const urlParams = new URLSearchParams(window.location.search);
        returnToProduct = urlParams.get('return_to_product') === '1';

        if (returnToProduct) {
            const backBtn = document.getElementById('back-to-product-btn');
            if (backBtn) {
                backBtn.style.display = '';
                backBtn.addEventListener('click', () => {
                    window.location.href = HTML_FILES.PRODUCT_MANAGEMENT + '?restore_product=1';
                });
            }
            // Auto-open the add modal so the user can immediately register a
            // manufacturer (matches the product-side prefill UX).
            openModal('add');
        }

        // Fit + center the window on this monitor
        await fitWindowToScreen();
    } catch (error) {
        console.error('Initialization error:', error);
        showToast(i18n.t('manufacturer_mgmt.failed_to_initialize'), { variant: 'error' });
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
    // PR13 (Fable-5 D8/D9): `deleteManufacturer` now returns a boolean
    // and the dead `confirmDeleteBtn.disabled` re-entry check is gone —
    // see product-management.js for the shared rationale.
    const confirmDeleteBtn = document.getElementById('confirm-delete-btn');
    confirmDeleteBtn.addEventListener('click', async () => {
        if (!manufacturerToDelete) return;
        confirmDeleteBtn.disabled = true;
        const ok = await deleteManufacturer(manufacturerToDelete.manufacturer_id);
        confirmDeleteBtn.disabled = false;
        if (ok) deleteModal.close();
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
        loading.textContent = i18n.t('manufacturer_mgmt.failed_to_load');
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
            nameCell.innerHTML = `<span style="color: #ffffff;">${escapeHtml(manufacturer.manufacturer_name)}</span>${badge}`;
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

// After a successful manufacturer add inside the product-side-trip flow,
// look the new row up by name from the freshly-reloaded `manufacturers`
// array and stamp its id into the persisted product draft. Best-effort: if
// the lookup fails for any reason, the draft is left untouched so the user
// can still come back and pick the manufacturer manually from the dropdown.
function linkNewManufacturerToProductDraft(manufacturerName) {
    try {
        const raw = sessionStorage.getItem(PRODUCT_DRAFT_KEY);
        if (!raw) return;
        const match = manufacturers.find(m => m.manufacturer_name === manufacturerName);
        if (!match) return;
        const draft = JSON.parse(raw);
        draft.manufacturer_id = String(match.manufacturer_id);
        sessionStorage.setItem(PRODUCT_DRAFT_KEY, JSON.stringify(draft));
    } catch (e) {
        console.warn('Could not link new manufacturer to product draft:', e);
    }
}

async function saveManufacturer() {
    clearErrors();

    const manufacturerNameInput = document.getElementById('manufacturer-name');
    const manufacturerMemoInput = document.getElementById('manufacturer-memo');
    const isDisabled = document.getElementById('manufacturer-is-disabled').checked ? 1 : 0;

    const result = await saveMasterEntry({
        nameInput: manufacturerNameInput,
        memoInput: manufacturerMemoInput,
        editingId: editingManufacturerId,
        findInCacheById: (id) => manufacturers.find(m => m.manufacturer_id === id) || null,
        invokeAdd: (name, memo) => invoke('add_manufacturer', {
            manufacturerName: name,
            memo,
            isDisabled: isDisabled === 1 ? isDisabled : null,
        }),
        invokeUpdate: (target, name, memo) => invoke('update_manufacturer', {
            manufacturerId: editingManufacturerId,
            manufacturerName: name,
            memo,
            displayOrder: target.display_order,
            isDisabled,
        }),
        i18nPrefix: 'manufacturer_mgmt',
        nameFieldI18nKey: 'manufacturer_mgmt.name',
        memoFieldI18nKey: 'manufacturer_mgmt.memo',
        nameMaxLen: MAX_NAME_LEN,
        memoMaxLen: MAX_MEMO_LEN,
        onNotFoundBeforeInvoke: async () => {
            showToast(i18n.t('manufacturer_mgmt.not_found'), { variant: 'error' });
            await loadManufacturers();
        },
    });

    if (result.mode === 'skip') {
        return;
    }

    // Save succeeded: failures past this point (list reload, side-trip
    // link) must not be reported as a failed save.
    await loadManufacturers();

    // If this add was the product-side trip, stamp the new manufacturer
    // id into the persisted product draft so the user resumes with it
    // already selected after "Back to product entry".
    if (result.mode === 'add' && returnToProduct) {
        const savedName = manufacturerNameInput.value.trim();
        linkNewManufacturerToProductDraft(savedName);
    }
}

/// Delete a manufacturer. Returns `true` on success (the confirmation
/// modal should close) or `false` on failure (the modal stays open so
/// the user can retry). PR13 (Fable-5 D8) — see product-management.js
/// for the shared rationale.
async function deleteManufacturer(manufacturerId) {
    try {
        await invoke('delete_manufacturer', {
            manufacturerId: manufacturerId
        });
        console.log('Manufacturer deleted successfully');
        await loadManufacturers();
        return true;
    } catch (error) {
        console.error('Failed to delete manufacturer:', error);
        // Delete-lock (master-delete-lock PR): reject-with-guidance when
        // any product still names this manufacturer.
        if (error?.code === API_ERROR_CODES.IN_USE) {
            showToast(i18n.t('manufacturer_mgmt.delete_in_use'), { variant: 'error' });
        } else {
            showToast(i18n.t('manufacturer_mgmt.failed_to_delete') + ': ' + formatApiError(error), { variant: 'error' });
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
        dropdownItems[0]?.addEventListener('click', () => {
            window.location.href = HTML_FILES.INDEX;
            fileDropdown.classList.remove('show');
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


