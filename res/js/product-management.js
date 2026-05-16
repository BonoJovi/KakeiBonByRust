import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { Modal } from './modal.js';
import { setupIndicators } from './indicators.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';
import { showValidationError, clearValidationError, showMaxLengthError, attachCharCounter } from './validation-display.js';
import { MAX_NAME_LEN, MAX_MEMO_LEN } from './consts.js';

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

        initProductModal();
        initDeleteModal();
        setupIndicators();
        setupEventListeners();
        await loadManufacturers();
        await loadProducts();

        // Adjust window size after content is loaded
        await adjustWindowSize();
    } catch (error) {
        console.error('Initialization error:', error);
        alert(i18n.t('product_mgmt.failed_to_initialize') + ': ' + error);
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
    document.getElementById('confirm-delete-btn').addEventListener('click', async () => {
        if (productToDelete) {
            await deleteProduct(productToDelete.product_id);
            deleteModal.close();
        }
    });
}

function setupEventListeners() {
    // Add product button
    document.getElementById('add-product-btn').addEventListener('click', () => {
        openModal('add');
    });

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
        loading.textContent = i18n.t('product_mgmt.failed_to_load') + ': ' + error;
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
            nameCell.innerHTML = `<span style="color: #ffffff;">${product.product_name}</span>${badge}`;
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
    const productNameInput = document.getElementById('product-name');
    const productMemoInput = document.getElementById('product-memo');
    const productName = productNameInput.value.trim();
    const manufacturerIdValue = document.getElementById('product-manufacturer').value;
    const manufacturerId = manufacturerIdValue ? parseInt(manufacturerIdValue) : null;
    const memo = productMemoInput.value.trim();
    const isDisabled = document.getElementById('product-is-disabled').checked ? 1 : 0;

    // Clear previous errors
    clearErrors();
    clearValidationError(productNameInput);
    clearValidationError(productMemoInput);

    // Validation — empty name
    if (!productName) {
        showValidationError(productNameInput, i18n.t('product_mgmt.empty_name'));
        throw new Error('Validation error: empty product name');
    }

    // Validation — max length (mirrors Rust defense in src/services/product.rs)
    if ([...productName].length > MAX_NAME_LEN) {
        showMaxLengthError(productNameInput, i18n.t('product_mgmt.name'), MAX_NAME_LEN);
        throw new Error('Validation error: product name too long');
    }
    if (memo && [...memo].length > MAX_MEMO_LEN) {
        showMaxLengthError(productMemoInput, i18n.t('product_mgmt.memo'), MAX_MEMO_LEN);
        throw new Error('Validation error: memo too long');
    }

    try {
        if (editingProductId) {
            // Update existing product
            const product = products.find(p => p.product_id === editingProductId);
            await invoke('update_product', {
                productId: editingProductId,
                productName: productName,
                manufacturerId: manufacturerId,
                memo: memo || null,
                displayOrder: product.display_order,
                isDisabled: isDisabled
            });
            console.log('Product updated successfully');
        } else {
            // Add new product
            await invoke('add_product', {
                productName: productName,
                manufacturerId: manufacturerId,
                memo: memo || null,
                isDisabled: isDisabled === 1 ? isDisabled : null
            });
            console.log('Product added successfully');
        }

        // Reload products list (modal will be closed by Modal class)
        await loadProducts();
    } catch (error) {
        console.error('Failed to save product:', error);

        // Map backend error messages to i18n resources / localized text
        const errorMessage = error.toString();
        let nameMessage = null;
        let memoMessage = null;

        if (errorMessage.includes('already exists')) {
            nameMessage = i18n.t('product_mgmt.duplicate_error');
        } else if (errorMessage.includes('Product name must be')) {
            // Defense-line trip: frontend max-length check should have caught
            // this, so use the same i18n message for parity.
            nameMessage = i18n.t('validation.max_length', {
                field: i18n.t('product_mgmt.name'),
                max: MAX_NAME_LEN,
                actual: [...productName].length,
            });
        } else if (errorMessage.includes('Memo must be')) {
            memoMessage = i18n.t('validation.max_length', {
                field: i18n.t('product_mgmt.memo'),
                max: MAX_MEMO_LEN,
                actual: [...memo].length,
            });
        } else if (errorMessage.includes('cannot be empty')) {
            nameMessage = i18n.t('product_mgmt.empty_name');
        } else {
            // Fallback to original error message on the name field
            nameMessage = errorMessage;
        }

        if (nameMessage) showValidationError(productNameInput, nameMessage);
        if (memoMessage) showValidationError(productMemoInput, memoMessage);

        // Re-throw error to prevent modal from closing
        throw error;
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
        alert(i18n.t('product_mgmt.failed_to_delete') + ': ' + error);
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
        await loadProducts();
    } catch (error) {
        console.error('Failed to change language:', error);
    }
}
