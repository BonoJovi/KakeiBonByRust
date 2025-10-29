import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { ROLE_ADMIN, ROLE_USER } from './consts.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers, adjustWindowSize } from './font-size.js';

let currentUsers = [];
let editingUserId = null;

// Modal instances
let userModal;
let deleteModal;

console.log('user-management.js loaded');

document.addEventListener('DOMContentLoaded', async function() {
    console.log('[DOMContentLoaded] DOM loaded');
    
    await i18n.init();
    i18n.updateUI();
    
    // Setup menu handlers first (includes global click handler)
    console.log('[DOMContentLoaded] Setting up menu handlers');
    setupMenuHandlers();
    
    // Then setup language and font size menus
    console.log('[DOMContentLoaded] Setting up language menu');
    await setupLanguageMenu();
    setupLanguageMenuHandlers();
    
    console.log('[DOMContentLoaded] Setting up font size menu');
    setupFontSizeMenuHandlers();
    await setupFontSizeMenu();
    setupFontSizeModalHandlers();
    await applyFontSize();
    
    console.log('[DOMContentLoaded] Setting up modal and indicators');
    initModals();
    setupModalEventHandlers();
    setupIndicators();
    
    console.log('[DOMContentLoaded] Loading users');
    await loadUsers();
    
    console.log('[DOMContentLoaded] Adjusting window size for modals');
    await adjustWindowSize();
    
    console.log('[DOMContentLoaded] Initialization complete');
});

function initModals() {
    // Initialize User Modal
    userModal = new Modal('user-modal', {
        formId: 'user-form',
        closeButtonId: 'close-modal',
        cancelButtonId: 'cancel-btn',
        onOpen: (mode, data) => {
            const title = document.getElementById('modal-title');
            const passwordGroup = document.getElementById('password-group');
            const passwordConfirmGroup = document.getElementById('password-confirm-group');
            const passwordInput = document.getElementById('password');
            const passwordConfirmInput = document.getElementById('password-confirm');
            
            if (mode === 'add') {
                title.textContent = i18n.t('user_mgmt.add_user');
                passwordGroup.style.display = 'block';
                passwordConfirmGroup.style.display = 'block';
                passwordInput.required = true;
                passwordConfirmInput.required = true;
                editingUserId = null;
            } else if (mode === 'edit') {
                title.textContent = i18n.t('user_mgmt.edit_user');
                passwordGroup.style.display = 'none';
                passwordConfirmGroup.style.display = 'none';
                passwordInput.required = false;
                passwordConfirmInput.required = false;
                editingUserId = data.userId;
                
                // Set form values
                document.getElementById('username').value = data.username;
            }
        },
        onSave: async (formData) => {
            await handleUserSave();
        }
    });
    
    // Initialize Delete Modal
    deleteModal = new Modal('delete-modal', {
        closeButtonId: 'close-delete-modal',
        cancelButtonId: 'cancel-delete',
        saveButtonId: 'confirm-delete',
        onOpen: (mode, data) => {
            document.getElementById('delete-username').textContent = data.username;
        },
        onSave: async (formData) => {
            await handleUserDelete(formData.userId);
        }
    });
}


function setupMenuHandlers() {
    console.log('[setupMenuHandlers] Starting setup');
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    console.log('[setupMenuHandlers] fileMenu:', fileMenu);
    console.log('[setupMenuHandlers] fileDropdown:', fileDropdown);
    
    if (fileMenu && fileDropdown) {
        // Check if already initialized
        if (fileMenu.dataset.initialized === 'true') {
            console.log('[setupMenuHandlers] File menu already initialized, skipping');
            return;
        }
        
        console.log('[setupMenuHandlers] Adding click listener to fileMenu');
        fileMenu.addEventListener('click', function(e) {
            console.log('[fileMenu clicked]');
            e.stopPropagation();
            
            const isShown = fileDropdown.classList.contains('show');
            
            // Close all other dropdowns
            document.querySelectorAll('.dropdown').forEach(d => {
                if (d !== fileDropdown) {
                    d.classList.remove('show');
                }
            });
            
            // Toggle this dropdown
            if (!isShown) {
                fileDropdown.classList.add('show');
            }
            
            console.log('[fileMenu] Toggled show class, current classes:', fileDropdown.className);
        });
        
        // Prevent dropdown from closing when clicking inside it
        fileDropdown.addEventListener('click', function(e) {
            console.log('[fileDropdown clicked]');
            e.stopPropagation();
        });
        
        const dropdownItems = fileDropdown.querySelectorAll('.dropdown-item');
        dropdownItems[0]?.addEventListener('click', () => {
            console.log('[fileDropdown] Back to main clicked');
            window.location.href = 'index.html';
            fileDropdown.classList.remove('show');
        });
        dropdownItems[1]?.addEventListener('click', () => {
            console.log('[fileDropdown] Logout clicked');
            handleLogout();
            fileDropdown.classList.remove('show');
        });
        dropdownItems[2]?.addEventListener('click', () => {
            console.log('[fileDropdown] Quit clicked');
            handleQuit();
            fileDropdown.classList.remove('show');
        });
        
        // Mark as initialized
        fileMenu.dataset.initialized = 'true';
        console.log('[setupMenuHandlers] File menu marked as initialized');
    }
    
    // Global click handler to close all dropdowns (only register once)
    if (!document.body.dataset.globalClickHandlerInitialized) {
        console.log('[setupMenuHandlers] Adding global click handler');
        document.addEventListener('click', function(e) {
            console.log('[document clicked] Target:', e.target);
            // Don't close if clicking on a menu item or dropdown
            if (e.target.closest('.menu-item') || e.target.closest('.dropdown')) {
                console.log('[document clicked] Click was on menu or dropdown, ignoring');
                return;
            }
            console.log('[document clicked] Closing all dropdowns');
            document.querySelectorAll('.dropdown').forEach(dropdown => {
                dropdown.classList.remove('show');
            });
        });
        document.body.dataset.globalClickHandlerInitialized = 'true';
        console.log('[setupMenuHandlers] Global click handler registered');
    } else {
        console.log('[setupMenuHandlers] Global click handler already registered, skipping');
    }
}

function setupLanguageMenuHandlers() {
    console.log('[setupLanguageMenuHandlers] Starting setup');
    const languageMenu = document.getElementById('language-menu');
    const languageDropdown = document.getElementById('language-dropdown');
    
    console.log('[setupLanguageMenuHandlers] languageMenu:', languageMenu);
    console.log('[setupLanguageMenuHandlers] languageDropdown:', languageDropdown);
    
    if (!languageMenu || !languageDropdown) {
        console.error('Language menu elements not found');
        return;
    }
    
    // Check if already initialized
    if (languageMenu.dataset.initialized === 'true') {
        console.log('[setupLanguageMenuHandlers] Already initialized, skipping');
        return;
    }
    
    console.log('[setupLanguageMenuHandlers] Adding click listener to languageMenu');
    languageMenu.addEventListener('click', function(e) {
        console.log('[languageMenu clicked]');
        e.stopPropagation();
        
        const isShown = languageDropdown.classList.contains('show');
        
        // Close all other dropdowns
        document.querySelectorAll('.dropdown').forEach(d => {
            if (d !== languageDropdown) {
                d.classList.remove('show');
            }
        });
        
        // Toggle this dropdown
        if (!isShown) {
            languageDropdown.classList.add('show');
        }
        
        console.log('[languageMenu] Toggled show class, current classes:', languageDropdown.className);
    });
    
    // Prevent dropdown from closing when clicking inside it
    languageDropdown.addEventListener('click', function(e) {
        console.log('[languageDropdown clicked]');
        e.stopPropagation();
    });
    
    // Mark as initialized
    languageMenu.dataset.initialized = 'true';
    console.log('[setupLanguageMenuHandlers] Marked as initialized');
}

async function setupLanguageMenu() {
    try {
        const languageNames = await invoke('get_language_names');
        const currentLang = i18n.getCurrentLanguage();
        const languageDropdown = document.getElementById('language-dropdown');
        
        if (!languageDropdown) {
            console.error('Language dropdown not found');
            return;
        }
        
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
        await loadUsers();
    } catch (error) {
        console.error('Failed to change language:', error);
        showMessage('user-list-message', 'Failed to change language: ' + error, 'error');
    }
}

function setupModalEventHandlers() {
    const addUserBtn = document.getElementById('add-user-btn');
    addUserBtn?.addEventListener('click', openAddUserModal);
    
    // Note: Modal class handles close, cancel, and save button events
}

async function loadUsers() {
    try {
        showMessage('user-list-message', i18n.t('user_mgmt.loading'), 'info');
        
        const users = await invoke('list_users');
        currentUsers = users;
        
        const userList = document.getElementById('user-list');
        if (!userList) return;
        
        userList.innerHTML = '';
        
        if (users.length === 0) {
            userList.innerHTML = `<tr><td colspan="6" class="empty-state">${i18n.t('user_mgmt.no_users')}</td></tr>`;
            showMessage('user-list-message', '', '');
            return;
        }
        
        for (const user of users) {
            const row = createUserRow(user);
            userList.appendChild(row);
        }
        
        showMessage('user-list-message', '', '');
    } catch (error) {
        console.error('Failed to load users:', error);
        showMessage('user-list-message', i18n.t('error.load_users_failed') + ': ' + error, 'error');
    }
}

function createUserRow(user) {
    const row = document.createElement('tr');
    
    const roleText = user.role === ROLE_ADMIN ? 
        i18n.t('user_mgmt.role_admin') : 
        i18n.t('user_mgmt.role_user');
    const roleClass = user.role === ROLE_ADMIN ? 'role-admin' : 'role-user';
    
    const isAdmin = user.role === ROLE_ADMIN;
    const editText = i18n.t('user_mgmt.edit');
    const deleteText = i18n.t('user_mgmt.delete');
    
    const deleteButton = isAdmin ? '' : 
        `<button class="btn-small btn-delete" data-user-id="${user.user_id}">${deleteText}</button>`;
    
    row.innerHTML = `
        <td>${user.user_id}</td>
        <td>${escapeHtml(user.name)}</td>
        <td><span class="role-badge ${roleClass}">${roleText}</span></td>
        <td>${formatDateTime(user.entry_dt)}</td>
        <td>${user.update_dt ? formatDateTime(user.update_dt) : '-'}</td>
        <td>
            <div class="action-buttons">
                <button class="btn-small btn-edit" data-user-id="${user.user_id}">${editText}</button>
                ${deleteButton}
            </div>
        </td>
    `;
    
    const editBtn = row.querySelector('.btn-edit');
    const deleteBtn = row.querySelector('.btn-delete');
    
    editBtn?.addEventListener('click', () => openEditUserModal(user));
    deleteBtn?.addEventListener('click', () => openDeleteModal(user));
    
    return row;
}

function openAddUserModal() {
    showMessage('form-message', '', '');
    userModal.open('add', {});
}

function openEditUserModal(user) {
    showMessage('form-message', '', '');
    userModal.open('edit', { userId: user.user_id, username: user.name });
}

function closeUserModal() {
    userModal.close();
}

async function handleUserSave() {
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value;
    const passwordConfirm = document.getElementById('password-confirm').value;
    
    if (password && password !== passwordConfirm) {
        showMessage('form-message', i18n.t('error.password_mismatch'), 'error');
        throw new Error('Password mismatch');
    }
    
    if (password && password.length < 16) {
        showMessage('form-message', i18n.t('error.password_too_short'), 'error');
        throw new Error('Password too short');
    }
    
    try {
        if (editingUserId) {
            await updateUser(editingUserId, username, password || null);
        } else {
            await createUser(username, password);
        }
        
        await loadUsers();
    } catch (error) {
        console.error('Failed to save user:', error);
        showMessage('form-message', i18n.t('error.save_user_failed') + ': ' + error, 'error');
        throw error;
    }
}

async function createUser(username, password) {
    showMessage('form-message', i18n.t('user_mgmt.creating'), 'info');
    
    const userId = await invoke('create_general_user', {
        username: username,
        password: password
    });
    
    showMessage('form-message', i18n.t('user_mgmt.user_created'), 'success');
    return userId;
}

async function updateUser(userId, username, password) {
    showMessage('form-message', i18n.t('user_mgmt.updating'), 'info');
    
    const user = currentUsers.find(u => u.user_id === userId);
    if (!user) {
        throw new Error('User not found');
    }
    
    const updateParams = {
        userId: userId,
        username: username !== user.name ? username : null,
        password: password
    };
    
    if (user.role === ROLE_ADMIN) {
        await invoke('update_admin_user_info', updateParams);
    } else {
        await invoke('update_general_user_info', updateParams);
    }
    
    showMessage('form-message', i18n.t('user_mgmt.user_updated'), 'success');
}

function openDeleteModal(user) {
    showMessage('delete-result-message', '', '');
    deleteModal.open('delete', { userId: user.user_id, username: user.name });
}

function closeDeleteModal() {
    deleteModal.close();
}

async function handleUserDelete(userId) {
    if (!userId) return;
    
    showMessage('delete-result-message', i18n.t('user_mgmt.deleting'), 'info');
    
    await invoke('delete_general_user_info', { userId: userId });
    
    showMessage('delete-result-message', i18n.t('user_mgmt.user_deleted'), 'success');
    
    setTimeout(async () => {
        deleteModal.close();
        await loadUsers();
    }, 1500);
}

function handleLogout() {
    console.log('Logout clicked');
    window.location.href = 'index.html';
}

function handleQuit() {
    console.log('Quit clicked');
    invoke('handle_quit');
}

function showMessage(elementId, message, type) {
    const element = document.getElementById(elementId);
    if (!element) return;
    
    element.textContent = message;
    element.className = 'message';
    
    if (type) {
        element.classList.add(type);
    }
}

function formatDateTime(dateTimeStr) {
    if (!dateTimeStr) return '-';
    return dateTimeStr.replace('T', ' ').substring(0, 19);
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
