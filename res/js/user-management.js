import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { ROLE_ADMIN, ROLE_USER, MAX_NAME_LEN } from './consts.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';
import { HTML_FILES } from './html-files.js';
import { Modal } from './modal.js';
import { getCurrentSessionUser, isSessionAuthenticated } from './session.js';
import { createMenuBar, handleLogout, handleQuit } from './menu.js';
import { showValidationError, clearValidationError, showMaxLengthError, attachCharCounter } from './validation-display.js';
import { invalidatePeriodSettingsCache } from './period.js';
import { fitWindowToScreen } from './window-fit.js';
import { showToast } from './toast.js';
import { mapMasterErrorCode, API_ERROR_CODES, formatApiError } from './master-crud.js';

let currentUsers = [];
let editingUserId = null;

// Modal instances
let userModal;
let deleteModal;
let periodSettingsModal;

console.log('user-management.js loaded');

document.addEventListener('DOMContentLoaded', async function() {
    console.log('[DOMContentLoaded] DOM loaded');
    
    // Create menu bar
    createMenuBar('management');
    
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
    
    console.log(`Logged in as: ${user.name} (ID: ${user.user_id}, Role: ${user.role})`);
    
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
    
    console.log('[DOMContentLoaded] Setting up custom validation messages');
    setupCustomValidationMessages();
    
    console.log('[DOMContentLoaded] Loading users');
    await loadUsers();
    
    console.log('[DOMContentLoaded] Fitting window to screen');
    await fitWindowToScreen();

    console.log('[DOMContentLoaded] Initialization complete');
});

function initModals() {
    // Wire bounded-field counter + live error clearing for the username input.
    setupUsernameCounter();

    // Initialize User Modal
    userModal = new Modal('user-modal', {
        formId: 'user-form',
        closeButtonId: 'close-modal',
        cancelButtonId: 'cancel-btn',
        onOpen: (mode, data) => {
            const title = document.getElementById('modal-title');
            const oldPasswordGroup = document.getElementById('old-password-group');
            const passwordGroup = document.getElementById('password-group');
            const passwordConfirmGroup = document.getElementById('password-confirm-group');
            const oldPasswordInput = document.getElementById('old-password');
            const passwordInput = document.getElementById('password');
            const passwordConfirmInput = document.getElementById('password-confirm');
            const usernameInput = document.getElementById('username');

            // Clear password fields on every open so a previous session's
            // typed value can't leak into the next edit. The inline
            // validation error attached to the current-password input
            // (set by handleUserSave when the backend returns
            // `old_password_incorrect`) also has to go, otherwise the
            // wording sticks to the empty field on the next open and
            // points at input the user has not typed yet (CodeRabbit
            // review on #123).
            oldPasswordInput.value = '';
            passwordInput.value = '';
            passwordConfirmInput.value = '';
            clearValidationError(oldPasswordInput);

            if (mode === 'add') {
                title.textContent = i18n.t('user_mgmt.add_user');
                // New users have no encrypted data yet, so the "current
                // password" prompt would be meaningless — hide it.
                oldPasswordGroup.style.display = 'none';
                oldPasswordInput.required = false;
                passwordGroup.style.display = 'block';
                passwordConfirmGroup.style.display = 'block';
                passwordInput.required = true;
                passwordConfirmInput.required = true;
                editingUserId = null;
                usernameInput.value = '';
            } else if (mode === 'edit') {
                title.textContent = i18n.t('user_mgmt.edit_user');
                // Password fields are optional at the browser level —
                // JS requires `old-password` only when the user actually
                // types a new password (see handleUserSave).
                oldPasswordGroup.style.display = 'block';
                passwordGroup.style.display = 'block';
                passwordConfirmGroup.style.display = 'block';
                oldPasswordInput.required = false;
                passwordInput.required = false;
                passwordConfirmInput.required = false;
                editingUserId = data.userId;

                // Set form values
                usernameInput.value = data.username;
            }

            // Clear validation errors and refresh counter after programmatic value changes.
            clearValidationError(usernameInput);
            usernameInput.dispatchEvent(new Event('input'));
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

    // Initialize Period Settings Modal
    periodSettingsModal = new Modal('period-settings-modal', {
        formId: 'period-settings-form',
        closeButtonId: 'close-period-settings-modal',
        cancelButtonId: 'period-settings-cancel',
        onOpen: async () => {
            showMessage('period-settings-message', '', '');
            try {
                const settings = await invoke('get_user_period_settings');
                document.getElementById('month-period-start-day').value = settings.month_period_start_day;
                document.getElementById('year-period-start-month').value = settings.year_period_start_month;
                document.getElementById('year-period-start-day').value = settings.year_period_start_day;
                const shift = Number(settings.month_period_holiday_shift) || 0;
                const shiftInput = document.querySelector(
                    `input[name="month-period-holiday-shift"][value="${shift}"]`
                );
                if (shiftInput) shiftInput.checked = true;
            } catch (error) {
                console.error('Failed to load period settings:', error);
                showMessage('period-settings-message', i18n.t('user_mgmt.period_settings_load_failed') + ': ' + error, 'error');
            }
        },
        onSave: async () => {
            await handlePeriodSettingsSave();
        }
    });
}

function setupUsernameCounter() {
    const usernameInput = document.getElementById('username');
    if (!usernameInput) return;
    attachCharCounter(usernameInput, MAX_NAME_LEN);
    usernameInput.addEventListener('input', () => clearValidationError(usernameInput));
}


function setupMenuHandlers() {
    console.log('[setupMenuHandlers] Starting setup');
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    console.log('[setupMenuHandlers] fileMenu:', fileMenu);
    console.log('[setupMenuHandlers] fileDropdown:', fileDropdown);
    
    if (fileMenu && fileDropdown) {
        // The toggle may already be wired by menu.js; the items are only wired here
        if (fileMenu.dataset.initialized === 'true') {
            console.log('[setupMenuHandlers] File menu toggle already initialized, skipping');
        } else {
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
                fileDropdown.classList.toggle('show', !isShown);

                console.log('[fileMenu] Toggled show class, current classes:', fileDropdown.className);
            });

            // Prevent dropdown from closing when clicking inside it
            fileDropdown.addEventListener('click', function(e) {
                console.log('[fileDropdown clicked]');
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
            console.log('[fileDropdown] Back to main clicked');
            window.location.href = HTML_FILES.INDEX;
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
        console.log('[setupMenuHandlers] File menu items wired');
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
        // Font Size submenu items are built via textContent (no data-i18n),
        // so an explicit redraw is needed after language change.
        await setupFontSizeMenu();
        await loadUsers();
    } catch (error) {
        console.error('Failed to change language:', error);
        showMessage('user-list-message', i18n.t('error.language_change_failed') + ': ' + error, 'error');
    }
}

function setupModalEventHandlers() {
    const addUserBtn = document.getElementById('add-user-btn');
    addUserBtn?.addEventListener('click', openAddUserModal);

    const periodSettingsBtn = document.getElementById('open-period-settings-btn');
    periodSettingsBtn?.addEventListener('click', () => periodSettingsModal.open('edit', {}));

    // Note: Modal class handles close, cancel, and save button events
}

async function handlePeriodSettingsSave() {
    const monthStartDay = parseInt(document.getElementById('month-period-start-day').value, 10);
    const yearStartMonth = parseInt(document.getElementById('year-period-start-month').value, 10);
    const yearStartDay = parseInt(document.getElementById('year-period-start-day').value, 10);
    const shiftRadio = document.querySelector('input[name="month-period-holiday-shift"]:checked');
    const monthHolidayShift = shiftRadio ? parseInt(shiftRadio.value, 10) : 0;

    if (!Number.isInteger(monthStartDay) || monthStartDay < 1 || monthStartDay > 31) {
        showMessage('period-settings-message', i18n.t('validation.invalid_period_start_day'), 'error');
        throw new Error('Validation error: invalid month period start day');
    }
    if (!Number.isInteger(yearStartMonth) || yearStartMonth < 1 || yearStartMonth > 12) {
        showMessage('period-settings-message', i18n.t('validation.invalid_period_start_month'), 'error');
        throw new Error('Validation error: invalid year period start month');
    }
    if (!Number.isInteger(yearStartDay) || yearStartDay < 1 || yearStartDay > 31) {
        showMessage('period-settings-message', i18n.t('validation.invalid_period_start_day'), 'error');
        throw new Error('Validation error: invalid year period start day');
    }
    if (!Number.isInteger(monthHolidayShift) || monthHolidayShift < 0 || monthHolidayShift > 2) {
        showMessage('period-settings-message', i18n.t('validation.invalid_month_period_holiday_shift'), 'error');
        throw new Error('Validation error: invalid month period holiday shift');
    }

    try {
        await invoke('update_user_period_settings', {
            monthPeriodStartDay: monthStartDay,
            yearPeriodStartMonth: yearStartMonth,
            yearPeriodStartDay: yearStartDay,
            monthPeriodHolidayShift: monthHolidayShift,
        });
        invalidatePeriodSettingsCache();
        showToast(i18n.t('user_mgmt.period_settings_saved'), { variant: 'success' });
    } catch (error) {
        console.error('Failed to save period settings:', error);
        showMessage('period-settings-message', i18n.t('user_mgmt.period_settings_save_failed') + ': ' + error, 'error');
        throw error;
    }
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
        // formatApiError unwraps the { code, message, entity } object
        // shape that list_users (migrated to ApiError) now returns.
        showMessage('user-list-message', i18n.t('error.load_users_failed') + ': ' + formatApiError(error), 'error');
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
    const usernameInput = document.getElementById('username');
    const username = usernameInput.value.trim();
    const oldPassword = document.getElementById('old-password').value;
    const password = document.getElementById('password').value;
    const passwordConfirm = document.getElementById('password-confirm').value;

    clearValidationError(usernameInput);

    // Validation — max length (mirrors Rust defense in src/services/user_management.rs)
    if ([...username].length > MAX_NAME_LEN) {
        showMaxLengthError(usernameInput, i18n.t('user_mgmt.username'), MAX_NAME_LEN);
        throw new Error('Validation error: username too long');
    }

    if (password && password !== passwordConfirm) {
        showMessage('form-message', i18n.t('error.password_mismatch'), 'error');
        throw new Error('Password mismatch');
    }

    if (password && password.length < 16) {
        showMessage('form-message', i18n.t('error.password_too_short'), 'error');
        throw new Error('Password too short');
    }

    // Fable-5 review #1 — changing the password requires the caller to
    // prove knowledge of the current one so the backend can decrypt
    // every encrypted field with the old key before re-encrypting with
    // the new one. Without this guard the frontend could send the new
    // hash while every ciphertext still needs the old key, orphaning
    // the data forever (see change_password_in_tx). Only enforced in
    // edit mode — add mode has no encrypted data yet and no old
    // password to check against.
    if (editingUserId && password && !oldPassword) {
        showMessage('form-message', i18n.t('error.current_password_required'), 'error');
        throw new Error('Current password required');
    }

    try {
        if (editingUserId) {
            await updateUser(editingUserId, username, password || null, oldPassword || null);
        } else {
            await createUser(username, password);
        }

        await loadUsers();
    } catch (error) {
        console.error('Failed to save user:', error);

        // Password-change specific: the current-password field is where
        // the user can act on this failure, so surface the error inline
        // there instead of dropping the generic classifier's toast.
        // Handled before mapMasterErrorCode because the shared
        // classifier has no concept of the current-password input.
        if (error && typeof error === 'object'
            && error.code === API_ERROR_CODES.OLD_PASSWORD_INCORRECT) {
            const oldPasswordInput = document.getElementById('old-password');
            showValidationError(oldPasswordInput, i18n.t('error.old_password_incorrect'));
            showMessage('form-message', '', '');
            throw error;
        }

        // Backend not_found means the edit target was deleted between
        // load and save. Reload the list so the "the list has been
        // reloaded" wording on `user_mgmt.not_found` matches what the
        // user actually sees on screen — same fix as saveMasterEntry's
        // backend-not_found path (Devin review on #97, extended to
        // #100 for the standalone user-management save flow).
        const isBackendNotFound = error && typeof error === 'object'
            && error.code === API_ERROR_CODES.NOT_FOUND;

        // Route through the shared classifier so `duplicate_name`
        // (username taken) surfaces as an inline error under the
        // username input via `user_mgmt.duplicate_error`, validation
        // trips (username too long) reuse the shared
        // `validation.max_length` template, and admin_protected
        // pops as its own toast rather than the generic form message.
        // No more `errStr.includes('Username must be')` substring
        // classification.
        const mapped = mapMasterErrorCode(error, {
            i18nPrefix: 'user_mgmt',
            nameFieldI18nKey: 'user_mgmt.username',
            memoFieldI18nKey: 'user_mgmt.username',
            nameMaxLen: MAX_NAME_LEN,
            memoMaxLen: MAX_NAME_LEN,
            actualNameLen: [...username].length,
            actualMemoLen: 0,
        });

        if (mapped.nameMessage || mapped.toastMessage) {
            // Clear the stale "creating.../updating..." progress note
            // that createUser/updateUser set on the form-message slot;
            // otherwise the info line stays on top of the inline error
            // or toast and reads as "still saving" while the classifier
            // has already announced the failure (Devin review on #100).
            showMessage('form-message', '', '');
        }

        if (mapped.toastMessage) {
            showToast(mapped.toastMessage, { variant: 'error' });
        }
        if (mapped.nameMessage) {
            // Username-scoped errors (duplicate / too long) surface
            // inline next to the input, matching the shape shop /
            // manufacturer / product use for name errors.
            showValidationError(usernameInput, mapped.nameMessage);
        } else if (!mapped.toastMessage) {
            // Fully unknown error — replace the progress note with the
            // legacy form message and a properly-unwrapped body.
            showMessage('form-message', i18n.t('error.save_user_failed') + ': ' + formatApiError(error), 'error');
        }

        // Fire the actual reload the not_found toast promises. Done
        // after the toast/inline error so those messages remain
        // visible during the async reload.
        if (isBackendNotFound) {
            await loadUsers();
        }

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

async function updateUser(userId, username, password, oldPassword) {
    showMessage('form-message', i18n.t('user_mgmt.updating'), 'info');

    const user = currentUsers.find(u => u.user_id === userId);
    if (!user) {
        throw new Error('User not found');
    }

    // Only send the username field when it actually changed — a NULL
    // username on the backend means "leave it alone" and lets the
    // rename-only vs. password-only paths share one shape.
    const nextUsername = username !== user.name ? username : null;

    if (password) {
        // Password change: route through the `_with_reencryption`
        // command so re-encrypting every ENCRYPTED_FIELDS row and
        // updating USERS.PAW commit as one atomic step. The `_info`
        // command below deliberately can no longer accept a password
        // argument (Fable-5 review #1, #5).
        const params = {
            oldPassword,
            username: nextUsername,
            newPassword: password,
        };
        if (user.role === ROLE_ADMIN) {
            await invoke('update_admin_user_with_reencryption', params);
        } else {
            await invoke('update_general_user_with_reencryption', params);
        }
    } else {
        // Rename-only path.
        const params = { username: nextUsername };
        if (user.role === ROLE_ADMIN) {
            await invoke('update_admin_user_info', params);
        } else {
            await invoke('update_general_user_info', params);
        }
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
    try {
        await invoke('delete_general_user_info', { userId: userId });
        showToast(i18n.t('user_mgmt.user_deleted'), { variant: 'success' });
        await loadUsers();
    } catch (error) {
        // admin_protected → dedicated wording; anything else → generic
        // failure message with a properly-unwrapped body.
        if (error && typeof error === 'object' && error.code === API_ERROR_CODES.ADMIN_PROTECTED) {
            showToast(i18n.t('user_mgmt.admin_protected'), { variant: 'error' });
        } else {
            showToast(i18n.t('error.delete_user_failed') + ': ' + formatApiError(error), { variant: 'error' });
        }
        throw error;
    }
}

function setupCustomValidationMessages() {
    // Set custom validation messages for required fields
    const requiredInputs = document.querySelectorAll('input[required], select[required], textarea[required]');
    
    requiredInputs.forEach(input => {
        input.addEventListener('invalid', function(e) {
            if (this.validity.valueMissing) {
                this.setCustomValidity(i18n.t('validation.required'));
            }
        });
        
        // Clear custom validity on input
        input.addEventListener('input', function() {
            this.setCustomValidity('');
        });
    });
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
