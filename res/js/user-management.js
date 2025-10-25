import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { ROLE_ADMIN, ROLE_USER } from './consts.js';
import { setupIndicators } from './indicators.js';

let currentUsers = [];
let editingUserId = null;

console.log('user-management.js loaded');

document.addEventListener('DOMContentLoaded', async function() {
    console.log('DOM loaded');
    
    await i18n.init();
    i18n.updateUI();
    
    await setupLanguageMenu();
    setupLanguageMenuHandlers();
    setupMenuHandlers();
    setupModalHandlers();
    setupIndicators();
    
    await loadUsers();
});

function setupMenuHandlers() {
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    if (fileMenu && fileDropdown) {
        fileMenu.addEventListener('click', function(e) {
            e.stopPropagation();
            fileDropdown.classList.toggle('show');
        });
        
        fileDropdown.addEventListener('click', function(e) {
            e.stopPropagation();
        });
        
        document.addEventListener('click', function() {
            document.querySelectorAll('.dropdown').forEach(dropdown => {
                dropdown.classList.remove('show');
            });
        });
        
        const dropdownItems = fileDropdown.querySelectorAll('.dropdown-item');
        dropdownItems[0]?.addEventListener('click', () => {
            window.location.href = 'index.html';
        });
        dropdownItems[1]?.addEventListener('click', handleLogout);
        dropdownItems[2]?.addEventListener('click', handleQuit);
    }
}

function setupLanguageMenuHandlers() {
    const languageMenu = document.getElementById('language-menu');
    const languageDropdown = document.getElementById('language-dropdown');
    
    if (!languageMenu || !languageDropdown) {
        console.error('Language menu elements not found');
        return;
    }
    
    languageMenu.addEventListener('click', function(e) {
        e.stopPropagation();
        
        document.querySelectorAll('.dropdown').forEach(dropdown => {
            if (dropdown !== languageDropdown) {
                dropdown.classList.remove('show');
            }
        });
        
        languageDropdown.classList.toggle('show');
    });
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

function setupModalHandlers() {
    const addUserBtn = document.getElementById('add-user-btn');
    const closeModalBtn = document.getElementById('close-modal');
    const cancelBtn = document.getElementById('cancel-btn');
    const userForm = document.getElementById('user-form');
    const closeDeleteModalBtn = document.getElementById('close-delete-modal');
    const cancelDeleteBtn = document.getElementById('cancel-delete');
    const confirmDeleteBtn = document.getElementById('confirm-delete');
    
    addUserBtn?.addEventListener('click', openAddUserModal);
    closeModalBtn?.addEventListener('click', closeUserModal);
    cancelBtn?.addEventListener('click', closeUserModal);
    userForm?.addEventListener('submit', handleUserFormSubmit);
    
    closeDeleteModalBtn?.addEventListener('click', closeDeleteModal);
    cancelDeleteBtn?.addEventListener('click', closeDeleteModal);
    confirmDeleteBtn?.addEventListener('click', handleDeleteConfirm);
    
    document.getElementById('user-modal')?.addEventListener('click', function(e) {
        if (e.target === this) {
            closeUserModal();
        }
    });
    
    document.getElementById('delete-modal')?.addEventListener('click', function(e) {
        if (e.target === this) {
            closeDeleteModal();
        }
    });
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
    editingUserId = null;
    
    const modal = document.getElementById('user-modal');
    const modalTitle = document.getElementById('modal-title');
    const userForm = document.getElementById('user-form');
    
    modalTitle.textContent = i18n.t('user_mgmt.add_user');
    userForm.reset();
    document.getElementById('user-id').value = '';
    
    const passwordInput = document.getElementById('password');
    const passwordConfirmInput = document.getElementById('password-confirm');
    passwordInput.required = true;
    passwordConfirmInput.required = true;
    
    showMessage('form-message', '', '');
    modal.classList.remove('hidden');
    
    // Focus on username field
    setTimeout(() => {
        document.getElementById('username').focus();
    }, 100);
}

function openEditUserModal(user) {
    editingUserId = user.user_id;
    
    const modal = document.getElementById('user-modal');
    const modalTitle = document.getElementById('modal-title');
    const userForm = document.getElementById('user-form');
    
    modalTitle.textContent = i18n.t('user_mgmt.edit_user');
    
    document.getElementById('user-id').value = user.user_id;
    document.getElementById('username').value = user.name;
    document.getElementById('password').value = '';
    document.getElementById('password-confirm').value = '';
    
    const passwordInput = document.getElementById('password');
    const passwordConfirmInput = document.getElementById('password-confirm');
    passwordInput.required = false;
    passwordConfirmInput.required = false;
    
    showMessage('form-message', '', '');
    modal.classList.remove('hidden');
    
    // Focus on username field
    setTimeout(() => {
        document.getElementById('username').focus();
    }, 100);
}

function closeUserModal() {
    const modal = document.getElementById('user-modal');
    modal.classList.add('hidden');
    editingUserId = null;
}

async function handleUserFormSubmit(e) {
    e.preventDefault();
    
    const userId = document.getElementById('user-id').value;
    const username = document.getElementById('username').value.trim();
    const password = document.getElementById('password').value;
    const passwordConfirm = document.getElementById('password-confirm').value;
    
    if (password && password !== passwordConfirm) {
        showMessage('form-message', i18n.t('error.password_mismatch'), 'error');
        return;
    }
    
    if (password && password.length < 16) {
        showMessage('form-message', i18n.t('error.password_too_short'), 'error');
        return;
    }
    
    try {
        if (userId) {
            await updateUser(parseInt(userId), username, password || null);
        } else {
            await createUser(username, password);
        }
        
        closeUserModal();
        await loadUsers();
    } catch (error) {
        console.error('Failed to save user:', error);
        showMessage('form-message', i18n.t('error.save_user_failed') + ': ' + error, 'error');
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
    const modal = document.getElementById('delete-modal');
    const usernameDisplay = document.getElementById('delete-username');
    
    usernameDisplay.textContent = user.name;
    modal.dataset.userId = user.user_id;
    
    showMessage('delete-result-message', '', '');
    modal.classList.remove('hidden');
}

function closeDeleteModal() {
    const modal = document.getElementById('delete-modal');
    modal.classList.add('hidden');
    delete modal.dataset.userId;
}

async function handleDeleteConfirm() {
    const modal = document.getElementById('delete-modal');
    const userId = parseInt(modal.dataset.userId);
    
    if (!userId) return;
    
    try {
        showMessage('delete-result-message', i18n.t('user_mgmt.deleting'), 'info');
        
        await invoke('delete_general_user_info', { userId: userId });
        
        showMessage('delete-result-message', i18n.t('user_mgmt.user_deleted'), 'success');
        
        setTimeout(async () => {
            closeDeleteModal();
            await loadUsers();
        }, 1500);
        
    } catch (error) {
        console.error('Failed to delete user:', error);
        showMessage('delete-result-message', i18n.t('error.delete_user_failed') + ': ' + error, 'error');
    }
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
