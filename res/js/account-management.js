// DEBUGGING: Testing i18n import
import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';

console.log('=== ACCOUNT-MANAGEMENT.JS LOADED - ALL imports enabled ===');
console.log('invoke:', typeof invoke);
console.log('i18n:', i18n);

let currentLanguage = 'ja';
let templates = [];
let accounts = [];
let editingAccountCode = null;

// Initialize
document.addEventListener('DOMContentLoaded', async () => {
    console.log('DOMContentLoaded fired');
    try {
        await i18n.init();
        console.log('i18n initialized:', i18n.initialized);
        currentLanguage = i18n.getCurrentLanguage();
        i18n.updateUI();
        
        setupEventListeners();
        await loadTemplates();
        await loadAccounts();
    } catch (error) {
        console.error('Initialization error:', error);
        alert('Failed to initialize: ' + error);
    }
});

function setupEventListeners() {
    // Back button
    document.getElementById('back-btn').addEventListener('click', () => {
        window.location.href = HTML_FILES.INDEX;
    });

    // Add account button
    document.getElementById('add-account-btn').addEventListener('click', () => {
        openModal('add');
    });

    // Account code auto-uppercase
    document.getElementById('account-code').addEventListener('input', (e) => {
        e.target.value = e.target.value.toUpperCase();
    });

    // Modal close buttons
    document.getElementById('close-modal').addEventListener('click', closeModal);
    document.getElementById('cancel-btn').addEventListener('click', closeModal);

    // Modal overlay click - use mousedown to prevent accidental close when dragging
    const modal = document.getElementById('account-modal');
    modal.addEventListener('mousedown', (e) => {
        if (e.target.id === 'account-modal') {
            closeModal();
        }
    });

    // Form submit
    document.getElementById('account-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        await saveAccount();
    });
}

// Load account templates
async function loadTemplates() {
    try {
        templates = await invoke('get_account_templates');
        populateTemplateDropdown();
    } catch (error) {
        console.error('Failed to load templates:', error);
        alert('Failed to load templates: ' + error);
    }
}

function populateTemplateDropdown() {
    const select = document.getElementById('template-code');
    
    // Clear existing options except the first one
    while (select.options.length > 1) {
        select.remove(1);
    }

    // Add template options
    templates.forEach(template => {
        const option = document.createElement('option');
        option.value = template.template_code;
        
        // Use language-specific template name
        const templateName = currentLanguage === 'ja' 
            ? template.template_name_ja 
            : template.template_name_en;
        
        option.textContent = templateName;
        select.appendChild(option);
    });
}

// Load accounts
async function loadAccounts() {
    const loading = document.getElementById('loading');
    const tbody = document.getElementById('accounts-tbody');

    loading.style.display = 'block';
    tbody.innerHTML = '';

    try {
        accounts = await invoke('get_accounts');
        
        if (accounts.length === 0) {
            tbody.innerHTML = `<tr><td colspan="5" style="text-align: center; color: #999;">No accounts found</td></tr>`;
        } else {
            accounts.forEach(account => {
                const row = createAccountRow(account);
                tbody.appendChild(row);
            });
        }
    } catch (error) {
        console.error('Failed to load accounts:', error);
        tbody.innerHTML = `<tr><td colspan="5" style="text-align: center; color: #dc3545;">Error loading accounts: ${error}</td></tr>`;
    } finally {
        loading.style.display = 'none';
    }
}

function createAccountRow(account) {
    const row = document.createElement('tr');
    
    // Find template name
    const template = templates.find(t => t.template_code === account.template_code);
    const templateName = template 
        ? (currentLanguage === 'ja' ? template.template_name_ja : template.template_name_en)
        : account.template_code;

    row.innerHTML = `
        <td>${escapeHtml(account.account_code)}</td>
        <td>${escapeHtml(account.account_name)}</td>
        <td>${escapeHtml(templateName)}</td>
        <td style="text-align: right;">${account.initial_balance.toLocaleString()}</td>
        <td class="actions">
            <button class="btn btn-warning edit-btn" data-code="${escapeHtml(account.account_code)}">Edit</button>
            <button class="btn btn-danger delete-btn" data-code="${escapeHtml(account.account_code)}">Delete</button>
        </td>
    `;

    // Edit button
    row.querySelector('.edit-btn').addEventListener('click', () => {
        openModal('edit', account);
    });

    // Delete button
    row.querySelector('.delete-btn').addEventListener('click', () => {
        deleteAccount(account.account_code, account.account_name);
    });

    return row;
}

function openModal(mode, account = null) {
    const modal = document.getElementById('account-modal');
    const modalTitle = document.getElementById('modal-title');
    const form = document.getElementById('account-form');
    
    // Clear form
    form.reset();
    clearErrors();

    if (mode === 'add') {
        modalTitle.setAttribute('data-i18n', 'account_mgmt.modal_title_add');
        modalTitle.textContent = i18n.t('account_mgmt.modal_title_add');
        document.getElementById('account-code').removeAttribute('readonly');
        editingAccountCode = null;
    } else {
        modalTitle.setAttribute('data-i18n', 'account_mgmt.modal_title_edit');
        modalTitle.textContent = i18n.t('account_mgmt.modal_title_edit');
        
        // Populate form
        document.getElementById('account-code').value = account.account_code;
        document.getElementById('account-code').setAttribute('readonly', 'true');
        document.getElementById('account-name').value = account.account_name;
        document.getElementById('template-code').value = account.template_code;
        document.getElementById('initial-balance').value = account.initial_balance;
        
        editingAccountCode = account.account_code;
    }

    modal.classList.add('show');
}

function closeModal() {
    const modal = document.getElementById('account-modal');
    modal.classList.remove('show');
    editingAccountCode = null;
}

async function saveAccount() {
    clearErrors();

    const accountCode = document.getElementById('account-code').value.trim();
    const accountName = document.getElementById('account-name').value.trim();
    const templateCode = document.getElementById('template-code').value;
    const initialBalance = parseInt(document.getElementById('initial-balance').value);

    // Validation
    if (!accountCode) {
        showError('account-code-error', 'Account code is required');
        return;
    }

    if (!accountName) {
        showError('account-name-error', 'Account name is required');
        return;
    }

    if (!templateCode) {
        showError('template-code-error', 'Template is required');
        return;
    }

    if (isNaN(initialBalance)) {
        showError('initial-balance-error', 'Initial balance must be a number');
        return;
    }

    try {
        if (editingAccountCode) {
            // Update existing account
            const displayOrder = accounts.find(a => a.account_code === editingAccountCode).display_order;
            await invoke('update_account', {
                accountCode: accountCode,
                accountName: accountName,
                templateCode: templateCode,
                initialBalance: initialBalance,
                displayOrder: displayOrder
            });
            alert(i18n.t('common.success') || 'Account updated successfully')
        } else {
            // Add new account
            await invoke('add_account', {
                accountCode: accountCode,
                accountName: accountName,
                templateCode: templateCode,
                initialBalance: initialBalance
            });
            alert(i18n.t('common.success') || 'Account added successfully')
        }

        closeModal();
        await loadAccounts();
    } catch (error) {
        console.error('Failed to save account:', error);
        alert('Failed to save account: ' + error);
    }
}

async function deleteAccount(accountCode, accountName) {
    const confirmed = confirm(
        `Are you sure you want to delete account "${accountName}" (${accountCode})?`
    );

    if (!confirmed) return;

    try {
        await invoke('delete_account', { accountCode: accountCode });
        alert(i18n.t('common.success') || 'Account deleted successfully')
        await loadAccounts();
    } catch (error) {
        console.error('Failed to delete account:', error);
        alert('Failed to delete account: ' + error);
    }
}

function showError(elementId, message) {
    const errorElement = document.getElementById(elementId);
    if (errorElement) {
        errorElement.textContent = message;
    }
}

function clearErrors() {
    document.querySelectorAll('.error-message').forEach(el => {
        el.textContent = '';
    });
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
