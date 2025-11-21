import { invoke } from '@tauri-apps/api/core';
import { HTML_FILES } from './html-files.js';
import i18n from './i18n.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';
import { ROLE_ADMIN, ROLE_USER } from './consts.js';
import { Modal } from './modal.js';
import { setupIndicators } from './indicators.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar } from './menu.js';

console.log('=== ACCOUNT-MANAGEMENT.JS LOADED - ALL imports enabled ===');
console.log('invoke:', typeof invoke);
console.log('i18n:', i18n);

let currentUserId = null;
let currentUserRole = null;

let currentLanguage = 'ja';
let templates = [];
let accounts = [];
let editingAccountCode = null;
let accountModal = null;

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
        
        initAccountModal();
        setupIndicators();
        setupEventListeners();
        await loadTemplates();
        await loadAccounts();
        
        // Adjust window size after content is loaded
        await adjustWindowSize();
    } catch (error) {
        console.error('Initialization error:', error);
        alert('Failed to initialize: ' + error);
    }
});

function initAccountModal() {
    accountModal = new Modal('account-modal', {
        formId: 'account-form',
        closeButtonId: 'close-modal',
        cancelButtonId: 'cancel-btn',
        onOpen: (mode, data) => {
            const modalTitle = document.getElementById('modal-title');
            const form = document.getElementById('account-form');
            const accountCodeInput = document.getElementById('account-code');
            
            // Clear form and errors
            form.reset();
            clearErrors();
            
            if (mode === 'add') {
                modalTitle.setAttribute('data-i18n', 'account_mgmt.modal_title_add');
                modalTitle.textContent = i18n.t('account_mgmt.modal_title_add');
                accountCodeInput.removeAttribute('readonly');
                editingAccountCode = null;
                
                // Focus on account code input after modal opens
                setTimeout(() => accountCodeInput.focus(), 0);
            } else if (mode === 'edit') {
                modalTitle.setAttribute('data-i18n', 'account_mgmt.modal_title_edit');
                modalTitle.textContent = i18n.t('account_mgmt.modal_title_edit');
                
                // Populate form
                accountCodeInput.value = data.account_code;
                accountCodeInput.setAttribute('readonly', 'true');
                document.getElementById('account-name').value = data.account_name;
                document.getElementById('template-code').value = data.template_code;
                document.getElementById('initial-balance').value = data.initial_balance;
                
                editingAccountCode = data.account_code;
            }
        },
        onSave: async (formData) => {
            await saveAccount();
        },
        onClose: () => {
            editingAccountCode = null;
        }
    });
}

function setupEventListeners() {
    // Add account button
    document.getElementById('add-account-btn').addEventListener('click', () => {
        openModal('add');
    });

    // Account code auto-uppercase
    document.getElementById('account-code').addEventListener('input', (e) => {
        e.target.value = e.target.value.toUpperCase();
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
        accounts = await invoke('get_accounts', {
            userId: currentUserId,
            userRole: currentUserRole
        });
        
        if (accounts.length === 0) {
            tbody.innerHTML = `<tr><td colspan="5" style="text-align: center; color: #999;">${i18n.t('account_mgmt.no_accounts')}</td></tr>`;
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

    // NONE account cannot be deleted
    const isNoneAccount = account.account_code === 'NONE';
    const deleteButtonHtml = isNoneAccount 
        ? `<button class="btn-small btn-delete" disabled style="opacity: 0.5; cursor: not-allowed;" data-i18n="common.delete">${i18n.t('common.delete')}</button>`
        : `<button class="btn-small btn-delete" data-code="${escapeHtml(account.account_code)}" data-i18n="common.delete">${i18n.t('common.delete')}</button>`;

    row.innerHTML = `
        <td>${escapeHtml(account.account_code)}</td>
        <td>${escapeHtml(account.account_name)}</td>
        <td>${escapeHtml(templateName)}</td>
        <td style="text-align: right;">${account.initial_balance.toLocaleString()}</td>
        <td class="actions">
            <button class="btn-small btn-edit" data-code="${escapeHtml(account.account_code)}" data-i18n="common.edit">${i18n.t('common.edit')}</button>
            ${deleteButtonHtml}
        </td>
    `;

    // Edit button
    row.querySelector('.btn-edit').addEventListener('click', () => {
        openModal('edit', account);
    });

    // Delete button (only if not NONE account)
    if (!isNoneAccount) {
        row.querySelector('.btn-delete').addEventListener('click', () => {
            deleteAccount(account.account_code, account.account_name);
        });
    }

    return row;
}

function openModal(mode, account = null) {
    if (mode === 'add') {
        accountModal.open('add', {});
    } else if (mode === 'edit') {
        accountModal.open('edit', account);
    }
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
                userId: currentUserId,
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
                userId: currentUserId,
                accountCode: accountCode,
                accountName: accountName,
                templateCode: templateCode,
                initialBalance: initialBalance
            });
            alert(i18n.t('common.success') || 'Account added successfully')
        }

        accountModal.close();
        await loadAccounts();
    } catch (error) {
        console.error('Failed to save account:', error);
        alert('Failed to save account: ' + error);
    }
}

async function deleteAccount(accountCode, accountName) {
    const confirmMessage = i18n.t('account_mgmt.delete_confirm') || 
        'Are you sure you want to delete this account?';
    const confirmed = confirm(
        `${confirmMessage}\n\n${accountName} (${accountCode})`
    );

    if (!confirmed) return;

    try {
        await invoke('delete_account', { 
            userId: currentUserId,
            accountCode: accountCode 
        });
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
        await loadAccounts();
    } catch (error) {
        console.error('Failed to change language:', error);
    }
}
