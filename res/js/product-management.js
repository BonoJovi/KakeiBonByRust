import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';
import { adjustWindowSize } from './font-size.js';
import { Modal } from './modal.js';
import { setupIndicators } from './indicators.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';

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
        
        currentUserId = user.id;
        currentUserRole = user.role;
        console.log(`Logged in as: ${user.username} (ID: ${currentUserId}, Role: ${currentUserRole})`);
        
        await i18n.init();
        console.log('i18n initialized:', i18n.initialized);
        currentLanguage = i18n.getCurrentLanguage();
        i18n.updateUI();

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

            // Clear form and errors
            form.reset();
            clearErrors();
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
                document.getElementById('product-name').value = data.product_name;
                document.getElementById('product-manufacturer').value = data.manufacturer_id || '';
                document.getElementById('product-memo').value = data.memo || '';
                document.getElementById('product-is-disabled').checked = data.is_disabled === 1;

                editingProductId = data.product_id;
            }
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
    // Back button
    document.getElementById('back-btn').addEventListener('click', () => {
        window.location.href = HTML_FILES.INDEX;
    });

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
        console.log('Loading manufacturers for user:', currentUserId);
        manufacturers = await invoke('get_manufacturers', {
            userId: currentUserId,
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

        console.log('Loading products for user:', currentUserId, 'includeDisabled:', showDisabledItems);
        products = await invoke('get_products', {
            userId: currentUserId,
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
        actionsCell.className = 'actions';

        // Edit button
        const editBtn = document.createElement('button');
        editBtn.className = 'btn btn-warning';
        editBtn.setAttribute('data-i18n', 'common.edit');
        editBtn.textContent = i18n.t('common.edit');
        editBtn.addEventListener('click', () => {
            openModal('edit', product);
        });
        actionsCell.appendChild(editBtn);

        // Delete button
        const deleteBtn = document.createElement('button');
        deleteBtn.className = 'btn btn-danger';
        deleteBtn.setAttribute('data-i18n', 'common.delete');
        deleteBtn.textContent = i18n.t('common.delete');
        deleteBtn.addEventListener('click', () => {
            openDeleteModal(product);
        });
        actionsCell.appendChild(deleteBtn);
    });
}

async function saveProduct() {
    const productName = document.getElementById('product-name').value.trim();
    const manufacturerIdValue = document.getElementById('product-manufacturer').value;
    const manufacturerId = manufacturerIdValue ? parseInt(manufacturerIdValue) : null;
    const memo = document.getElementById('product-memo').value.trim();
    const isDisabled = document.getElementById('product-is-disabled').checked ? 1 : 0;

    // Clear previous errors
    clearErrors();

    // Validation
    if (!productName) {
        document.getElementById('product-name-error').textContent = i18n.t('product_mgmt.empty_name');
        throw new Error('Validation error: empty product name');
    }

    try {
        if (editingProductId) {
            // Update existing product
            const product = products.find(p => p.product_id === editingProductId);
            await invoke('update_product', {
                userId: currentUserId,
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
                userId: currentUserId,
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

        // Map backend error messages to i18n resources
        const errorMessage = error.toString();
        let displayMessage;

        if (errorMessage.includes('already exists')) {
            displayMessage = i18n.t('product_mgmt.duplicate_error');
        } else if (errorMessage.includes('cannot be empty')) {
            displayMessage = i18n.t('product_mgmt.empty_name');
        } else {
            // Fallback to original error message
            displayMessage = errorMessage;
        }

        document.getElementById('product-name-error').textContent = displayMessage;

        // Re-throw error to prevent modal from closing
        throw error;
    }
}

async function deleteProduct(productId) {
    try {
        await invoke('delete_product', {
            userId: currentUserId,
            productId: productId
        });
        console.log('Product deleted successfully');
        await loadProducts();
    } catch (error) {
        console.error('Failed to delete product:', error);
        alert(i18n.t('product_mgmt.failed_to_delete') + ': ' + error);
    }
}
