import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';

let isLoggedIn = false;

console.log('menu.js loaded');

// Add keyboard shortcut listener
document.addEventListener('keydown', function(e) {
    // Check for Ctrl+Q (or Cmd+Q on Mac)
    if ((e.ctrlKey || e.metaKey) && e.key === 'q') {
        e.preventDefault();
        console.log('Ctrl+Q pressed, quitting...');
        handleQuit();
    }
});

document.addEventListener('DOMContentLoaded', async function() {
    console.log('DOM loaded');
    
    // Initialize i18n
    await i18n.init();
    
    // Update UI with translations
    i18n.updateUI();
    
    // Setup language menu event handlers first (once)
    setupLanguageMenuHandlers();
    
    // Then load and populate language menu items
    await setupLanguageMenu();
    
    // Setup font size menu
    setupFontSizeMenuHandlers();
    await setupFontSizeMenu();
    
    // Apply saved font size
    await applyFontSize();
    
    // Setup accessibility indicators for form inputs
    setupIndicators();
    
    // Check if initial setup is needed
    checkSetupNeeded();
    
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    console.log('fileMenu:', fileMenu);
    console.log('fileDropdown:', fileDropdown);
    
    if (fileMenu && fileDropdown) {
        // Check if already initialized
        if (fileMenu.dataset.initialized === 'true') {
            console.log('File menu already initialized, skipping');
        } else {
            console.log('Adding click event listener to file menu');
            fileMenu.addEventListener('click', function(e) {
                console.log('File menu clicked!');
                e.stopPropagation();
                const isShown = fileDropdown.classList.contains('show');
                console.log('Before toggle - has show class:', isShown);
                
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
                
                console.log('After toggle - classes:', fileDropdown.className);
                console.log('Computed display style:', window.getComputedStyle(fileDropdown).display);
            });
            
            // Prevent dropdown from closing when clicking inside it
            fileDropdown.addEventListener('click', function(e) {
                e.stopPropagation();
            });
            
            // Mark as initialized
            fileMenu.dataset.initialized = 'true';
        }
        
        // Global click handler to close all dropdowns (only register once)
        if (!document.body.dataset.globalClickHandlerInitialized) {
            console.log('Adding global click handler');
            document.addEventListener('click', function(e) {
                console.log('Document clicked, target:', e.target);
                // Don't close if clicking on a menu item or dropdown
                if (e.target.closest('.menu-item') || e.target.closest('.dropdown')) {
                    console.log('Click was on menu or dropdown, ignoring');
                    return;
                }
                console.log('Closing all dropdowns');
                document.querySelectorAll('.dropdown').forEach(dropdown => {
                    dropdown.classList.remove('show');
                });
            });
            document.body.dataset.globalClickHandlerInitialized = 'true';
            console.log('Global click handler registered');
        } else {
            console.log('Global click handler already registered, skipping');
        }
        
        // Add event listeners to dropdown items
        const loginItem = fileDropdown.querySelector('.dropdown-item:nth-child(1)');
        const logoutItem = fileDropdown.querySelector('.dropdown-item:nth-child(2)');
        const quitItem = fileDropdown.querySelector('.dropdown-item:nth-child(4)');
        
        if (loginItem) {
            loginItem.addEventListener('click', function(e) {
                console.log('Login item clicked');
                handleLoginMenu();
                fileDropdown.classList.remove('show');
            });
        }
        if (logoutItem) {
            logoutItem.addEventListener('click', function(e) {
                console.log('Logout item clicked');
                handleLogout();
                fileDropdown.classList.remove('show');
            });
        }
        if (quitItem) {
            quitItem.addEventListener('click', function(e) {
                console.log('Quit item clicked');
                handleQuit();
                fileDropdown.classList.remove('show');
            });
        }
    } else {
        console.error('Elements not found!');
    }
    
    // Setup admin menu
    const adminMenu = document.getElementById('admin-menu');
    const adminDropdown = document.getElementById('admin-dropdown');
    
    if (adminMenu && adminDropdown) {
        adminMenu.addEventListener('click', function(e) {
            console.log('Admin menu clicked');
            e.stopPropagation();
            adminDropdown.classList.toggle('show');
        });
        
        adminDropdown.addEventListener('click', function(e) {
            e.stopPropagation();
        });
        
        const userMgmtItem = adminDropdown.querySelector('.dropdown-item:nth-child(1)');
        if (userMgmtItem) {
            userMgmtItem.addEventListener('click', function(e) {
                console.log('User Management item clicked');
                window.location.href = 'user-management.html';
                adminDropdown.classList.remove('show');
            });
        }
    }
    
    // Setup login form
    const loginForm = document.getElementById('login-form-element');
    if (loginForm) {
        loginForm.addEventListener('submit', handleLoginSubmit);
    }
    
    // Setup admin registration form
    const adminSetupForm = document.getElementById('admin-setup-form');
    if (adminSetupForm) {
        adminSetupForm.addEventListener('submit', handleAdminSetup);
    }
    
    // Setup user registration form
    const userSetupForm = document.getElementById('user-setup-form');
    if (userSetupForm) {
        userSetupForm.addEventListener('submit', handleUserSetup);
    }
    
    applyFontSize();
});

async function checkSetupNeeded() {
    try {
        const needsSetup = await invoke('check_needs_setup');
        console.log('Needs setup:', needsSetup);
        
        if (needsSetup) {
            document.getElementById('admin-setup').classList.remove('hidden');
            document.getElementById('login-form').classList.add('hidden');
            document.getElementById('app-content').classList.add('hidden');
        } else {
            document.getElementById('admin-setup').classList.add('hidden');
            document.getElementById('login-form').classList.remove('hidden');
            document.getElementById('app-content').classList.add('hidden');
        }
    } catch (error) {
        console.error('Failed to check setup status:', error);
        // On error, assume setup is needed
        document.getElementById('admin-setup').classList.remove('hidden');
        document.getElementById('login-form').classList.add('hidden');
        document.getElementById('app-content').classList.add('hidden');
    }
}

function setupLanguageMenuHandlers() {
    const languageMenu = document.getElementById('language-menu');
    const languageDropdown = document.getElementById('language-dropdown');
    
    if (!languageMenu || !languageDropdown) {
        console.error('Language menu elements not found');
        return;
    }
    
    // Check if already initialized
    if (languageMenu.dataset.initialized === 'true') {
        console.log('[setupLanguageMenuHandlers] Already initialized, skipping');
        return;
    }
    
    // Setup dropdown toggle - only once
    languageMenu.addEventListener('click', function(e) {
        console.log('Language menu clicked');
        e.stopPropagation();
        
        const isShown = languageDropdown.classList.contains('show');
        
        // Close all other dropdowns
        document.querySelectorAll('.dropdown').forEach(dropdown => {
            if (dropdown !== languageDropdown) {
                dropdown.classList.remove('show');
            }
        });
        
        // Toggle this dropdown
        if (!isShown) {
            languageDropdown.classList.add('show');
        }
        
        console.log('Language dropdown toggled, show class:', languageDropdown.classList.contains('show'));
    });
    
    // Mark as initialized
    languageMenu.dataset.initialized = 'true';
}

async function setupLanguageMenu() {
    try {
        // Get language names (localized in current language) as array of [code, name]
        const languageNames = await invoke('get_language_names');
        console.log('Available languages:', languageNames);
        
        // Get current language
        const currentLang = i18n.getCurrentLanguage();
        
        // Get dropdown container
        const languageDropdown = document.getElementById('language-dropdown');
        
        if (!languageDropdown) {
            console.error('Language dropdown not found');
            return;
        }
        
        // Clear existing items only (not event listeners on parent)
        languageDropdown.innerHTML = '';
        
        // Add language items - languageNames is already sorted
        for (const [langCode, langName] of languageNames) {
            const item = document.createElement('div');
            item.className = 'dropdown-item';
            item.textContent = langName;
            item.dataset.langCode = langCode;
            
            // Mark current language with active class (shows filled circle)
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
        console.log('Changing language to:', langCode);
        await i18n.setLanguage(langCode);
        
        // Reload language menu items to update display names and selection
        await setupLanguageMenu();
        
        console.log('Language changed successfully');
    } catch (error) {
        console.error('Failed to change language:', error);
        alert('Failed to change language: ' + error);
    }
}

async function handleAdminSetup(e) {
    e.preventDefault();
    console.log('Admin setup form submitted');
    
    const username = document.getElementById('admin-username').value;
    const password = document.getElementById('admin-password').value;
    const passwordConfirm = document.getElementById('admin-password-confirm').value;
    const messageDiv = document.getElementById('setup-message');
    
    if (!password || password.trim() === '') {
        messageDiv.textContent = i18n.t('error.password_empty');
        messageDiv.className = 'message error';
        return;
    }
    
    if (password.length < 16) {
        messageDiv.textContent = i18n.t('error.password_too_short');
        messageDiv.className = 'message error';
        return;
    }
    
    if (password !== passwordConfirm) {
        messageDiv.textContent = i18n.t('error.password_mismatch');
        messageDiv.className = 'message error';
        return;
    }
    
    try {
        const result = await invoke('register_admin', {
            username: username,
            password: password
        });
        
        console.log('Admin registration result:', result);
        messageDiv.textContent = i18n.t('admin.registration_success');
        messageDiv.className = 'message success';
        
        setTimeout(() => {
            document.getElementById('admin-setup').classList.add('hidden');
            document.getElementById('login-form').classList.remove('hidden');
        }, 2000);
        
    } catch (error) {
        console.error('Admin registration error:', error);
        messageDiv.textContent = i18n.t('error.registration_failed') + ': ' + error;
        messageDiv.className = 'message error';
    }
}

async function handleUserSetup(e) {
    e.preventDefault();
    console.log('User setup form submitted');
    
    const username = document.getElementById('user-username').value;
    const password = document.getElementById('user-password').value;
    const passwordConfirm = document.getElementById('user-password-confirm').value;
    const messageDiv = document.getElementById('user-setup-message');
    
    if (!password || password.trim() === '') {
        messageDiv.textContent = i18n.t('error.password_empty');
        messageDiv.className = 'message error';
        return;
    }
    
    if (password.length < 16) {
        messageDiv.textContent = i18n.t('error.password_too_short');
        messageDiv.className = 'message error';
        return;
    }
    
    if (password !== passwordConfirm) {
        messageDiv.textContent = i18n.t('error.password_mismatch');
        messageDiv.className = 'message error';
        return;
    }
    
    try {
        const result = await invoke('register_user', {
            username: username,
            password: password
        });
        
        console.log('User registration result:', result);
        messageDiv.textContent = i18n.t('user.registration_success');
        messageDiv.className = 'message success';
        
        setTimeout(() => {
            document.getElementById('user-setup').classList.add('hidden');
            document.getElementById('app-content').classList.remove('hidden');
        }, 2000);
        
    } catch (error) {
        console.error('User registration error:', error);
        messageDiv.textContent = i18n.t('error.registration_failed') + ': ' + error;
        messageDiv.className = 'message error';
    }
}

function handleLoginMenu() {
    console.log('Login menu clicked');
    const loginContainer = document.getElementById('login-form');
    const appContent = document.getElementById('app-content');
    
    if (!isLoggedIn) {
        loginContainer.classList.remove('hidden');
        appContent.classList.add('hidden');
    }
}

async function handleLoginSubmit(e) {
    e.preventDefault();
    console.log('Login form submitted');
    
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const messageDiv = document.getElementById('login-message');
    
    try {
        const result = await invoke('login_user', {
            username: username,
            password: password
        });
        
        console.log('Login result:', result);
        messageDiv.textContent = i18n.t('login.success');
        messageDiv.className = 'message success';
        
        isLoggedIn = true;
        
        // Check if user setup is needed
        const needsUserSetup = await invoke('check_needs_user_setup');
        console.log('Needs user setup:', needsUserSetup);
        
        if (needsUserSetup) {
            // Show user registration form
            setTimeout(() => {
                document.getElementById('login-form').classList.add('hidden');
                document.getElementById('user-setup').classList.remove('hidden');
            }, 1000);
        } else {
            // Show app content
            setTimeout(() => {
                document.getElementById('login-form').classList.add('hidden');
                document.getElementById('app-content').classList.remove('hidden');
            }, 1000);
        }
        
    } catch (error) {
        console.error('Login error:', error);
        messageDiv.textContent = i18n.t('error.login_failed') + ': ' + error;
        messageDiv.className = 'message error';
    }
}

function handleLogout() {
    console.log('Logout clicked');
    isLoggedIn = false;
    
    // Clear login form
    document.getElementById('username').value = '';
    document.getElementById('password').value = '';
    document.getElementById('login-message').textContent = '';
    
    // Show login form and hide app content
    document.getElementById('login-form').classList.remove('hidden');
    document.getElementById('app-content').classList.add('hidden');
}

function handleQuit() {
    console.log('Quit clicked');
    invoke('handle_quit');
}

// Font size menu handlers
function setupFontSizeMenuHandlers() {
    console.log('[setupFontSizeMenuHandlers] Starting setup');
    const fontSizeMenu = document.getElementById('font-size-menu');
    const fontSizeDropdown = document.getElementById('font-size-dropdown');
    
    if (!fontSizeMenu || !fontSizeDropdown) {
        console.warn('Font size menu elements not found');
        return;
    }
    
    // Check if already initialized
    if (fontSizeMenu.dataset.initialized === 'true') {
        return;
    }
    
    fontSizeMenu.addEventListener('click', function(e) {
        e.stopPropagation();
        
        const isShown = fontSizeDropdown.classList.contains('show');
        
        // Close all other dropdowns
        document.querySelectorAll('.dropdown').forEach(d => {
            if (d !== fontSizeDropdown) {
                d.classList.remove('show');
            }
        });
        
        // Toggle this dropdown
        if (!isShown) {
            fontSizeDropdown.classList.add('show');
        }
    });
    
    // Prevent dropdown from closing when clicking inside it
    fontSizeDropdown.addEventListener('click', function(e) {
        e.stopPropagation();
    });
    
    // Mark as initialized
    fontSizeMenu.dataset.initialized = 'true';
    console.log('[setupFontSizeMenuHandlers] Marked as initialized');
}

async function setupFontSizeMenu() {
    try {
        // Get current font size
        const currentSize = await invoke('get_font_size');
        
        // Get dropdown container
        const fontSizeDropdown = document.getElementById('font-size-dropdown');
        
        if (!fontSizeDropdown) {
            console.error('Font size dropdown not found');
            return;
        }
        
        // Clear existing items
        fontSizeDropdown.innerHTML = '';
        
        // Define font size options
        const fontSizes = [
            { code: 'small', key: 'font_size.small' },
            { code: 'medium', key: 'font_size.medium' },
            { code: 'large', key: 'font_size.large' },
            { code: 'custom', key: 'font_size.custom', action: 'modal' }
        ];
        
        // Add font size items
        for (const size of fontSizes) {
            const item = document.createElement('div');
            item.className = 'dropdown-item';
            item.textContent = i18n.t(size.key);
            item.dataset.sizeCode = size.code;
            
            // Mark current size with active class
            if (size.code === currentSize) {
                item.classList.add('active');
            }
            
            item.addEventListener('click', async function(e) {
                e.stopPropagation();
                if (size.action === 'modal') {
                    // Open modal for custom settings
                    openFontSizeModal();
                } else {
                    await handleFontSizeChange(size.code);
                }
                fontSizeDropdown.classList.remove('show');
            });
            
            fontSizeDropdown.appendChild(item);
        }
        
    } catch (error) {
        console.error('Failed to setup font size menu:', error);
    }
}

async function handleFontSizeChange(sizeCode) {
    try {
        console.log('Changing font size to:', sizeCode);
        await invoke('set_font_size', { fontSize: sizeCode });
        
        // Apply the new font size
        await applyFontSize();
        
        // Reload font size menu to update selection
        await setupFontSizeMenu();
        
        console.log('Font size changed successfully');
    } catch (error) {
        console.error('Failed to change font size:', error);
        alert('Failed to change font size: ' + error);
    }
}

let resizeInProgress = false;
let lastResizeRequest = null;

async function applyFontSize() {
    try {
        const fontSize = await invoke('get_font_size');
        
        // Map size code to CSS variable
        const sizeMap = {
            'small': 'var(--font-size-small)',
            'medium': 'var(--font-size-medium)',
            'large': 'var(--font-size-large)'
        };
        
        let cssValue;
        
        if (sizeMap[fontSize]) {
            // It's a preset size
            cssValue = sizeMap[fontSize];
        } else {
            // It's a custom percentage value
            const percent = parseInt(fontSize);
            if (!isNaN(percent)) {
                cssValue = percent + '%';
            } else {
                cssValue = sizeMap['medium']; // fallback
            }
        }
        
        document.documentElement.style.setProperty('--base-font-size', cssValue);
        
        console.log('Applied font size:', fontSize, '→', cssValue);
        
        // Prevent multiple simultaneous resize attempts
        if (resizeInProgress) {
            return;
        }
        
        resizeInProgress = true;
        
        // Cancel any pending resize
        if (lastResizeRequest) {
            clearTimeout(lastResizeRequest);
        }
        
        try {
            // Wait for layout to update using requestAnimationFrame
            // This ensures the browser has completed the layout recalculation
            await new Promise(resolve => {
                requestAnimationFrame(() => {
                    requestAnimationFrame(() => {
                        // Wait one more frame for stability
                        requestAnimationFrame(resolve);
                    });
                });
            });
            
            // Adjust window size based on content (only once)
            await adjustWindowSize();
        } finally {
            resizeInProgress = false;
        }
    } catch (error) {
        console.error('Failed to apply font size:', error);
        resizeInProgress = false;
    }
}

async function adjustWindowSize() {
    try {
        // First, shrink window to minimum size to get natural content size
        const minWidth = 400;
        const minHeight = 300;
        
        await invoke('adjust_window_size', { 
            width: minWidth, 
            height: minHeight 
        });
        
        // Wait for layout to update after resize
        await new Promise(resolve => {
            requestAnimationFrame(() => {
                requestAnimationFrame(resolve);
            });
        });
        
        // Now measure the natural content size
        const mainContent = document.getElementById('main-content');
        const menuBar = document.getElementById('menu-bar');
        
        // Calculate total content height by getting bounding rectangles
        let maxWidth = 0;
        let maxHeight = 0;
        
        // Check all visible elements
        const elements = [menuBar, mainContent];
        for (const el of elements) {
            if (el && !el.classList.contains('hidden')) {
                const rect = el.getBoundingClientRect();
                maxWidth = Math.max(maxWidth, rect.right);
                maxHeight = Math.max(maxHeight, rect.bottom);
            }
        }
        
        // Add some padding
        const paddingWidth = 40;
        const paddingHeight = 40;
        
        const newWidth = maxWidth + paddingWidth;
        const newHeight = maxHeight + paddingHeight;
        
        // Resize to final size
        await invoke('adjust_window_size', { 
            width: newWidth, 
            height: newHeight 
        });
    } catch (error) {
        console.error('[adjustWindowSize] Failed to adjust window size:', error);
    }
}

// Font size modal functions
async function openFontSizeModal() {
    const modal = document.getElementById('font-size-modal');
    if (modal) {
        modal.classList.remove('hidden');
        
        // Load current font size settings
        await loadCurrentFontSizeSettings();
        
        // Setup modal event handlers if not already done
        setupFontSizeModalHandlers();
        
        // Setup click outside to close
        setupModalClickOutside();
        
        // Setup indicators for modal elements
        import('./indicators.js').then(module => {
            module.setupIndicators();
            
            // Set focus to preset dropdown after indicators are setup
            setTimeout(() => {
                const presetSelect = document.getElementById('font-size-preset');
                if (presetSelect) {
                    presetSelect.focus();
                    // Manually trigger active class for indicator
                    const formGroup = presetSelect.closest('.form-group');
                    if (formGroup) {
                        formGroup.classList.add('active');
                    }
                }
            }, 0);
        });
    }
}

async function loadCurrentFontSizeSettings() {
    try {
        // Get current font size from backend
        const currentSize = await invoke('get_font_size');
        console.log('[loadCurrentFontSizeSettings] Current size:', currentSize);
        
        const presetSelect = document.getElementById('font-size-preset');
        const percentInput = document.getElementById('font-size-percent');
        
        if (!presetSelect || !percentInput) {
            console.error('[loadCurrentFontSizeSettings] Elements not found');
            return;
        }
        
        // Map size code to percentage
        const sizeToPercent = {
            'small': 85,
            'medium': 100,
            'large': 115
        };
        
        if (sizeToPercent[currentSize]) {
            // It's a preset size
            presetSelect.value = currentSize;
            percentInput.value = sizeToPercent[currentSize];
        } else {
            // It's a custom percentage value (for future implementation)
            presetSelect.value = 'custom';
            // Parse percentage from currentSize (e.g., "120" or "120%")
            const percent = parseInt(currentSize);
            percentInput.value = isNaN(percent) ? 100 : percent;
        }
        
        console.log('[loadCurrentFontSizeSettings] Set preset:', presetSelect.value, 'percent:', percentInput.value);
    } catch (error) {
        console.error('[loadCurrentFontSizeSettings] Failed to load settings:', error);
        // Set defaults on error
        document.getElementById('font-size-preset').value = 'medium';
        document.getElementById('font-size-percent').value = 100;
    }
}

function closeFontSizeModal() {
    const modal = document.getElementById('font-size-modal');
    if (modal) {
        modal.classList.add('hidden');
        // Remove click outside handler
        removeModalClickOutside();
    }
}

function setupModalClickOutside() {
    const modal = document.getElementById('font-size-modal');
    if (!modal) return;
    
    const handleClickOutside = function(e) {
        // Check if click is on the modal backdrop (not on modal content)
        if (e.target === modal) {
            closeFontSizeModal();
        }
    };
    
    // Store handler reference for cleanup
    modal._clickOutsideHandler = handleClickOutside;
    modal.addEventListener('click', handleClickOutside);
}

function removeModalClickOutside() {
    const modal = document.getElementById('font-size-modal');
    if (modal && modal._clickOutsideHandler) {
        modal.removeEventListener('click', modal._clickOutsideHandler);
        delete modal._clickOutsideHandler;
    }
}

function setupFontSizeModalHandlers() {
    const modal = document.getElementById('font-size-modal');
    if (!modal || modal.dataset.handlersInitialized === 'true') {
        return;
    }
    
    // Close button
    const closeBtn = document.getElementById('font-size-modal-close');
    closeBtn?.addEventListener('click', closeFontSizeModal);
    
    // Cancel button
    const cancelBtn = document.getElementById('font-size-cancel');
    cancelBtn?.addEventListener('click', closeFontSizeModal);
    
    // Apply button
    const applyBtn = document.getElementById('font-size-apply');
    applyBtn?.addEventListener('click', handleFontSizeModalApply);
    
    // ESC key to close modal
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape' && !modal.classList.contains('hidden')) {
            closeFontSizeModal();
        }
    });
    
    // Focus trap
    const focusableElements = modal.querySelectorAll(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    const firstFocusable = focusableElements[0];
    const lastFocusable = focusableElements[focusableElements.length - 1];
    
    modal.addEventListener('keydown', function(e) {
        // Handle Tab key (with or without Shift modifier)
        // Also handle "Unidentified" key when Shift is pressed (Tauri-specific issue)
        const isTab = e.key === 'Tab' || (e.shiftKey && (e.key === 'Unidentified' || e.code === 'Tab'));
        
        if (!isTab) return;
        
        if (e.shiftKey) {
            // SHIFT + TAB
            if (document.activeElement === firstFocusable) {
                e.preventDefault();
                lastFocusable.focus();
            }
        } else {
            // TAB
            if (document.activeElement === lastFocusable) {
                e.preventDefault();
                firstFocusable.focus();
            }
        }
    });
    
    // Spinner buttons
    const spinnerUp = modal.querySelector('.spinner-up');
    const spinnerDown = modal.querySelector('.spinner-down');
    const percentInput = document.getElementById('font-size-percent');
    
    // Add focus/blur handlers for spinner buttons to maintain indicator
    const percentFormGroup = percentInput?.closest('.form-group');
    
    spinnerUp?.addEventListener('focus', () => {
        if (percentFormGroup) {
            percentFormGroup.classList.add('active');
        }
    });
    
    spinnerUp?.addEventListener('blur', () => {
        if (percentFormGroup && !percentInput.matches(':focus') && !spinnerDown.matches(':focus')) {
            percentFormGroup.classList.remove('active');
        }
    });
    
    spinnerDown?.addEventListener('focus', () => {
        if (percentFormGroup) {
            percentFormGroup.classList.add('active');
        }
    });
    
    spinnerDown?.addEventListener('blur', () => {
        if (percentFormGroup && !percentInput.matches(':focus') && !spinnerUp.matches(':focus')) {
            percentFormGroup.classList.remove('active');
        }
    });
    
    spinnerUp?.addEventListener('click', () => {
        const current = parseInt(percentInput.value) || 100;
        const step = parseInt(percentInput.step) || 5;
        const max = parseInt(percentInput.max) || 200;
        percentInput.value = Math.min(current + step, max);
        // Auto-switch to custom when spinner is used
        presetSelect.value = 'custom';
    });
    
    spinnerDown?.addEventListener('click', () => {
        const current = parseInt(percentInput.value) || 100;
        const step = parseInt(percentInput.step) || 5;
        const min = parseInt(percentInput.min) || 50;
        percentInput.value = Math.max(current - step, min);
        // Auto-switch to custom when spinner is used
        presetSelect.value = 'custom';
    });
    
    // Preset dropdown change
    const presetSelect = document.getElementById('font-size-preset');
    presetSelect?.addEventListener('change', (e) => {
        const value = e.target.value;
        if (value === 'small') {
            percentInput.value = 85;
        } else if (value === 'medium') {
            percentInput.value = 100;
        } else if (value === 'large') {
            percentInput.value = 115;
        }
    });
    
    // Percentage input change
    percentInput?.addEventListener('input', () => {
        presetSelect.value = 'custom';
    });
    
    modal.dataset.handlersInitialized = 'true';
}

async function handleFontSizeModalApply() {
    try {
        const presetSelect = document.getElementById('font-size-preset');
        const percentInput = document.getElementById('font-size-percent');
        const presetValue = presetSelect.value;
        const percentValue = parseInt(percentInput.value) || 100;
        
        console.log('[handleFontSizeModalApply] Applying:', presetValue, percentValue);
        
        let sizeToSave;
        
        if (presetValue === 'custom') {
            // Save as custom percentage
            sizeToSave = percentValue.toString();
            console.log('[handleFontSizeModalApply] Saving custom percentage:', sizeToSave);
        } else {
            // Save as preset (small/medium/large)
            sizeToSave = presetValue;
            console.log('[handleFontSizeModalApply] Saving preset:', sizeToSave);
        }
        
        // Save to backend
        await invoke('set_font_size', { fontSize: sizeToSave });
        
        // Apply the new font size to UI
        await applyFontSize();
        
        // Reload font size menu to update selection
        await setupFontSizeMenu();
        
        console.log('[handleFontSizeModalApply] Font size applied successfully');
        
        // Close modal
        closeFontSizeModal();
    } catch (error) {
        console.error('[handleFontSizeModalApply] Failed to apply font size:', error);
        alert('Failed to apply font size: ' + error);
    }
}

// Export font size functions for use in other pages
export { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize };

