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
import { saveMasterEntry } from './master-crud.js';

console.log('=== PRODUCT-MANAGEMENT.JS LOADED ===');

let currentUserId = null;
let currentUserRole = null;

let currentLanguage = 'ja';
let products = [];
let manufacturers = [];
let editingProductId = null;
let productModal = null;
let deleteModal = null;
let productToDelete = null;
let showDisabledItems = false;

// When the user lands here via the "Open in product master" jump from the
// transaction detail modal, the URL carries `return_to=<transaction_id>` and
// optionally `prefill_name=<typed text>`. We keep the return target on a
// module-level variable so saveProduct() can update the persisted detail
// draft with the newly created product id.
let returnToTransactionId = null;
const DETAIL_DRAFT_KEY = 'kakeibon.detail_draft.v1';

// Holds the in-flight product-modal state while the user side-jumps to
// manufacturer-management to register a new manufacturer. The return trip
// (manufacturer save → "Back to product entry") lands on
// product-management.html?restore_product=1, which consumes this draft to
// re-open the product modal in its original state — including the
// just-registered manufacturer pre-selected and the original
// "Back to detail entry" target if the user originally came from the
// detail modal.
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
        await setupLanguageMenu(loadProducts);
        setupLanguageMenuHandlers();
        
        setupFontSizeMenuHandlers();
        await setupFontSizeMenu();
        setupFontSizeModalHandlers();
        await applyFontSize();

        initProductModal();
        initDeleteModal();
        setupIndicators();
        setupEventListeners();
        await loadManufacturers();
        await loadProducts();

        // Handle the cross-page round-trips that arrive at product-management:
        //  - From detail modal: ?prefill_name=...&return_to=<tid>
        //  - From manufacturer modal: ?restore_product=1 (consume product_draft)
        // When restoring from the manufacturer side trip, return_to_transaction_id
        // inside the draft is what re-wires the "Back to detail entry" button.
        const urlParams = new URLSearchParams(window.location.search);
        const shouldRestoreProduct = urlParams.get('restore_product') === '1';

        if (shouldRestoreProduct) {
            const draft = consumeProductDraft();
            if (draft) {
                returnToTransactionId = draft.return_to_transaction_id || null;
                openModal('add');
                document.getElementById('product-name').value = draft.product_name || '';
                document.getElementById('product-manufacturer').value = draft.manufacturer_id || '';
                document.getElementById('product-memo').value = draft.memo || '';
                document.getElementById('product-is-disabled').checked = !!draft.is_disabled;
                clearProductDraft();
            }
        } else {
            returnToTransactionId = urlParams.get('return_to') || null;
        }

        if (returnToTransactionId) {
            const backBtn = document.getElementById('back-to-detail-btn');
            if (backBtn) {
                backBtn.style.display = '';
                backBtn.addEventListener('click', () => {
                    const params = new URLSearchParams();
                    params.set('transaction_id', returnToTransactionId);
                    params.set('restore', '1');
                    window.location.href = HTML_FILES.TRANSACTION_DETAIL_MANAGEMENT + '?' + params.toString();
                });
            }
        }

        const prefillName = urlParams.get('prefill_name') || '';
        if (prefillName && !shouldRestoreProduct) {
            openModal('add');
            const productNameInput = document.getElementById('product-name');
            if (productNameInput) {
                productNameInput.value = prefillName;
            }
        }

        // Fit + center the window on this monitor
        await fitWindowToScreen();
    } catch (error) {
        console.error('Initialization error:', error);
        showToast(i18n.t('product_mgmt.failed_to_initialize'), { variant: 'error' });
    }
});

function initProductModal() {
    productModal = new Modal('product-modal', {
        formId: 'product-form',
        closeButtonId: 'close-modal',
        cancelButtonId: 'cancel-btn',
        onOpen: (mode, data) => {
            const modalTitle = document.getElementById('modal-title');
            const form = document.getElementById('product-form');
            const productNameInput = document.getElementById('product-name');
            const productMemoInput = document.getElementById('product-memo');

            // Clear form and errors
            form.reset();
            clearErrors();
            clearValidationError(productNameInput);
            clearValidationError(productMemoInput);
            populateManufacturerDropdown();

            if (mode === 'add') {
                modalTitle.setAttribute('data-i18n', 'product_mgmt.add');
                modalTitle.textContent = i18n.t('product_mgmt.add');
                editingProductId = null;
                document.getElementById('product-is-disabled').checked = false;
            } else if (mode === 'edit') {
                modalTitle.setAttribute('data-i18n', 'product_mgmt.edit');
                modalTitle.textContent = i18n.t('product_mgmt.edit');

                // Populate form
                productNameInput.value = data.product_name;
                document.getElementById('product-manufacturer').value = data.manufacturer_id || '';
                productMemoInput.value = data.memo || '';
                document.getElementById('product-is-disabled').checked = data.is_disabled === 1;

                editingProductId = data.product_id;
            }

            // Refresh character counters after programmatic value changes
            // (form.reset() / direct .value assignments do not fire 'input').
            productNameInput?.dispatchEvent(new Event('input'));
            productMemoInput?.dispatchEvent(new Event('input'));
        },
        onSave: async (formData) => {
            await saveProduct();
        },
        onClose: () => {
            editingProductId = null;
        }
    });
}

function initDeleteModal() {
    deleteModal = new Modal('delete-modal', {
        closeButtonId: 'close-delete-modal',
        cancelButtonId: 'cancel-delete-btn',
        onOpen: (mode, product) => {
            const productNameDisplay = document.getElementById('delete-product-name');
            productNameDisplay.textContent = product.product_name;
            productToDelete = product;
        },
        onClose: () => {
            productToDelete = null;
        }
    });

    // Confirm delete button
    const confirmDeleteBtn = document.getElementById('confirm-delete-btn');
    confirmDeleteBtn.addEventListener('click', async () => {
        if (!productToDelete || confirmDeleteBtn.disabled) return;
        confirmDeleteBtn.disabled = true;
        try {
            await deleteProduct(productToDelete.product_id);
            deleteModal.close();
        } catch (error) {
            // Keep the confirmation modal open so the user can retry or cancel
        } finally {
            confirmDeleteBtn.disabled = false;
        }
    });
}

function setupEventListeners() {
    // Add product button
    document.getElementById('add-product-btn').addEventListener('click', () => {
        openModal('add');
    });

    // "Open in manufacturer master" jump from inside the product modal.
    // Persists the current product form to sessionStorage and navigates to
    // manufacturer-management. The user comes back via the "Back to product
    // entry" button on that page, which lands on ?restore_product=1 here.
    const openManufacturerBtn = document.getElementById('open-manufacturer-master-btn');
    if (openManufacturerBtn) {
        openManufacturerBtn.addEventListener('click', () => {
            const draft = buildProductDraftFromForm();
            persistProductDraft(draft);
            window.location.href = HTML_FILES.MANUFACTURER_MANAGEMENT + '?return_to_product=1';
        });
    }

    // Toggle disabled items button
    document.getElementById('toggle-disabled-btn').addEventListener('click', () => {
        showDisabledItems = !showDisabledItems;
        updateToggleButton();
        loadProducts();
    });

    // Live-clear validation errors as the user edits
    const productNameInput = document.getElementById('product-name');
    const productMemoInput = document.getElementById('product-memo');
    productNameInput?.addEventListener('input', () => clearValidationError(productNameInput));
    productMemoInput?.addEventListener('input', () => clearValidationError(productMemoInput));

    // Live character counters (kept in sync with backend chars().count())
    if (productNameInput) attachCharCounter(productNameInput, MAX_NAME_LEN);
    if (productMemoInput) attachCharCounter(productMemoInput, MAX_MEMO_LEN);
}

function openModal(mode, data = null) {
    productModal.open(mode, data);
}

function openDeleteModal(product) {
    deleteModal.open('delete', product);
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
    try {
        console.log('Loading manufacturers');
        manufacturers = await invoke('get_manufacturers', {
            includeDisabled: false
        });
        console.log('Loaded manufacturers:', manufacturers);
    } catch (error) {
        console.error('Failed to load manufacturers:', error);
        // Don't block the page, but show warning
        console.warn('Manufacturer list will be empty');
        manufacturers = [];
    }
}

function populateManufacturerDropdown() {
    const select = document.getElementById('product-manufacturer');
    select.innerHTML = '';

    // Add empty option
    const emptyOption = document.createElement('option');
    emptyOption.value = '';
    emptyOption.textContent = i18n.t('product_mgmt.manufacturer_none');
    select.appendChild(emptyOption);

    // Add manufacturer options
    manufacturers.forEach(manufacturer => {
        const option = document.createElement('option');
        option.value = manufacturer.manufacturer_id;
        option.textContent = manufacturer.manufacturer_name;
        select.appendChild(option);
    });
}

async function loadProducts() {
    const loading = document.getElementById('loading');
    const table = document.getElementById('products-table');

    try {
        loading.style.display = 'block';
        table.style.display = 'none';

        console.log('Loading products, includeDisabled:', showDisabledItems);
        products = await invoke('get_products', {
            includeDisabled: showDisabledItems
        });
        console.log('Loaded products:', products);

        renderProducts();

        loading.style.display = 'none';
        table.style.display = 'table';
    } catch (error) {
        console.error('Failed to load products:', error);
        loading.textContent = i18n.t('product_mgmt.failed_to_load');
    }
}

function renderProducts() {
    const tbody = document.getElementById('products-tbody');
    tbody.innerHTML = '';

    if (products.length === 0) {
        const row = tbody.insertRow();
        const cell = row.insertCell();
        cell.colSpan = 4;
        cell.style.textAlign = 'center';
        cell.style.padding = '20px';
        cell.style.color = '#999';
        cell.setAttribute('data-i18n', 'product_mgmt.no_data');
        cell.textContent = i18n.t('product_mgmt.no_data');
        return;
    }

    products.forEach(product => {
        const row = tbody.insertRow();

        // Apply styling for disabled items
        const isDisabled = product.is_disabled === 1;
        if (isDisabled) {
            row.style.backgroundColor = '#6c757d';  // Medium gray background
            // Note: No opacity - keeps buttons clearly visible
        }

        // Product Name
        const nameCell = row.insertCell();
        if (isDisabled) {
            // Add [非表示] badge for disabled items
            const badge = `<span style="color: #ffc107; font-weight: bold; margin-left: 8px;">[${i18n.t('common.disabled_label')}]</span>`;
            nameCell.innerHTML = `<span style="color: #ffffff;">${escapeHtml(product.product_name)}</span>${badge}`;
        } else {
            nameCell.textContent = product.product_name;
        }

        // Manufacturer
        const manufacturerCell = row.insertCell();
        manufacturerCell.textContent = product.manufacturer_name || i18n.t('product_mgmt.manufacturer_none');
        if (isDisabled) {
            manufacturerCell.style.color = '#ffffff';  // White text for disabled items
        } else {
            manufacturerCell.style.color = product.manufacturer_name ? '#212529' : '#999';
        }

        // Memo
        const memoCell = row.insertCell();
        memoCell.textContent = product.memo || '-';
        if (isDisabled) {
            memoCell.style.color = '#ffffff';  // White text for disabled items
        } else {
            memoCell.style.color = product.memo ? '#212529' : '#999';
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
            openModal('edit', product);
        });
        actionsDiv.appendChild(editBtn);

        // Delete button
        const deleteBtn = document.createElement('button');
        deleteBtn.className = 'btn-small btn-delete';
        deleteBtn.setAttribute('data-i18n', 'common.delete');
        deleteBtn.textContent = i18n.t('common.delete');
        deleteBtn.addEventListener('click', () => {
            openDeleteModal(product);
        });
        actionsDiv.appendChild(deleteBtn);
        
        actionsCell.appendChild(actionsDiv);
    });
}

async function saveProduct() {
    clearErrors();

    const productNameInput = document.getElementById('product-name');
    const productMemoInput = document.getElementById('product-memo');
    const manufacturerIdValue = document.getElementById('product-manufacturer').value;
    const manufacturerId = manufacturerIdValue ? parseInt(manufacturerIdValue) : null;
    const isDisabled = document.getElementById('product-is-disabled').checked ? 1 : 0;

    const result = await saveMasterEntry({
        nameInput: productNameInput,
        memoInput: productMemoInput,
        editingId: editingProductId,
        findInCacheById: (id) => products.find(p => p.product_id === id) || null,
        invokeAdd: (name, memo) => invoke('add_product', {
            productName: name,
            manufacturerId,
            memo,
            isDisabled: isDisabled === 1 ? isDisabled : null,
        }),
        invokeUpdate: (target, name, memo) => invoke('update_product', {
            productId: editingProductId,
            productName: name,
            manufacturerId,
            memo,
            displayOrder: target.display_order,
            isDisabled,
        }),
        i18nPrefix: 'product_mgmt',
        nameFieldI18nKey: 'product_mgmt.name',
        memoFieldI18nKey: 'product_mgmt.memo',
        nameMaxLen: MAX_NAME_LEN,
        memoMaxLen: MAX_MEMO_LEN,
        onNotFoundBeforeInvoke: async () => {
            showToast(i18n.t('product_mgmt.not_found'), { variant: 'error' });
            await loadProducts();
        },
    });

    if (result.mode === 'skip') {
        return;
    }

    // Save succeeded: failures past this point (detail-draft link, list
    // reload) must not be reported as a failed save.
    const savedName = productNameInput.value.trim();
    if (result.mode === 'add' && returnToTransactionId) {
        // If this add was triggered by the detail-jump flow, look up the
        // new product by name and stamp its id into the persisted draft
        // so the user returns to the detail modal with the new master
        // entry already selected (canonicalizing the item name too).
        await linkNewProductToDraft(savedName);
    }

    // Reload products list (modal will be closed by Modal class)
    await loadProducts();
}

// Snapshot the product modal's current inputs so the user can step out to
// manufacturer-management and come back to a pre-filled product modal. We
// carry return_to_transaction_id forward so the eventual "Back to detail
// entry" path still works two hops down.
function buildProductDraftFromForm() {
    return {
        product_name: document.getElementById('product-name')?.value || '',
        manufacturer_id: document.getElementById('product-manufacturer')?.value || '',
        memo: document.getElementById('product-memo')?.value || '',
        is_disabled: !!document.getElementById('product-is-disabled')?.checked,
        return_to_transaction_id: returnToTransactionId,
    };
}

function persistProductDraft(draft) {
    sessionStorage.setItem(PRODUCT_DRAFT_KEY, JSON.stringify(draft));
}

function consumeProductDraft() {
    const raw = sessionStorage.getItem(PRODUCT_DRAFT_KEY);
    if (!raw) return null;
    try {
        return JSON.parse(raw);
    } catch (e) {
        console.error('Failed to parse product draft, discarding', e);
        sessionStorage.removeItem(PRODUCT_DRAFT_KEY);
        return null;
    }
}

function clearProductDraft() {
    sessionStorage.removeItem(PRODUCT_DRAFT_KEY);
}

// Update the in-flight detail draft so the user resumes with the newly
// registered product selected. Best-effort: any error here just leaves the
// draft as-is — the user can still come back and pick the product manually
// from the autocomplete.
async function linkNewProductToDraft(productName) {
    try {
        const raw = sessionStorage.getItem(DETAIL_DRAFT_KEY);
        if (!raw) return;
        const candidates = await invoke('search_products_by_name', { query: productName });
        if (!candidates || candidates.length === 0) return;
        const match = candidates.find(c => c.product_name === productName) || candidates[0];
        const draft = JSON.parse(raw);
        draft.selected_product_id = match.product_id;
        draft.item_name = match.product_name;
        sessionStorage.setItem(DETAIL_DRAFT_KEY, JSON.stringify(draft));
    } catch (e) {
        console.warn('Could not link new product to detail draft:', e);
    }
}

async function deleteProduct(productId) {
    try {
        await invoke('delete_product', {
            productId: productId
        });
        console.log('Product deleted successfully');
        await loadProducts();
    } catch (error) {
        console.error('Failed to delete product:', error);
        showToast(i18n.t('product_mgmt.failed_to_delete'), { variant: 'error' });
        throw error;
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


