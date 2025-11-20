import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';
import * as session from './session.js';

let isLoggedIn = false;

console.log('menu.js loaded');

// Create menu bar dynamically based on page type
export function createMenuBar(pageType = 'management') {
    const menuBar = document.getElementById('menu-bar');
    if (!menuBar) return;
    
    let fileMenuItems = '';
    
    if (pageType === 'index') {
        fileMenuItems = `
            <div class="dropdown-item" data-i18n="menu.login">Login</div>
            <div class="dropdown-item" data-i18n="menu.logout">Logout</div>
            <div class="dropdown-separator"></div>
            <div class="dropdown-item" data-i18n="menu.quit">Quit</div>
        `;
    } else if (pageType === 'transaction-detail') {
        fileMenuItems = `
            <div class="dropdown-item" data-i18n="menu.back_to_transactions">Back to Transactions</div>
            <div class="dropdown-separator"></div>
            <div class="dropdown-item" data-i18n="menu.logout">Logout</div>
            <div class="dropdown-item" data-i18n="menu.quit">Quit</div>
        `;
    } else {
        // Default for management screens
        fileMenuItems = `
            <div class="dropdown-item" data-i18n="menu.back_to_main">Back to Main</div>
            <div class="dropdown-separator"></div>
            <div class="dropdown-item" data-i18n="menu.logout">Logout</div>
            <div class="dropdown-item" data-i18n="menu.quit">Quit</div>
        `;
    }
    
    const menuHTML = `
        <div id="file-menu" class="menu-item">
            <span data-i18n="menu.file">File</span>
            <div id="file-dropdown" class="dropdown">
                ${fileMenuItems}
            </div>
        </div>
        <div id="admin-menu" class="menu-item">
            <span data-i18n="menu.admin">Admin</span>
            <div id="admin-dropdown" class="dropdown">
                <div class="dropdown-item" data-i18n="menu.user_management">User Management</div>
                <div class="dropdown-item" data-i18n="menu.category_management">Category Management</div>
                <div class="dropdown-item" data-i18n="menu.account_management">Account Management</div>
                <div class="dropdown-item" data-i18n="menu.shop_management">Shop Management</div>
                <div class="dropdown-item" data-i18n="menu.manufacturer_management">Manufacturer Management</div>
                <div class="dropdown-item" data-i18n="menu.product_management">Product Management</div>
                <div class="dropdown-item" data-i18n="menu.transaction_management">Transaction Management</div>
                <div class="dropdown-item" data-i18n="menu.aggregation">Monthly Aggregation</div>
                <div class="dropdown-item" data-i18n="menu.aggregation_daily">Daily Aggregation</div>
            </div>
        </div>
        <div id="language-menu" class="menu-item">
            <span data-i18n="menu.language">Language</span>
            <div id="language-dropdown" class="dropdown">
                <!-- Language options will be populated dynamically -->
            </div>
        </div>
        <div id="font-size-menu" class="menu-item">
            <span data-i18n="menu.font_size">Font Size</span>
            <div id="font-size-dropdown" class="dropdown">
                <!-- Font size options will be populated dynamically -->
            </div>
        </div>
    `;
    
    menuBar.innerHTML = menuHTML;
}

// Add keyboard shortcut listener
document.addEventListener('keydown', function(e) {
    // Check for Ctrl+Q (or Cmd+Q on Mac)
    if ((e.ctrlKey || e.metaKey) && e.key === 'q') {
        e.preventDefault();
        console.log('Ctrl+Q pressed, quitting...');
        handleQuit();
    }
});

import { HTML_FILES } from './html-files.js';

document.addEventListener('DOMContentLoaded', async function() {
    console.log('DOM loaded');

    // Create menu bar dynamically for index page
    createMenuBar('index');

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
    setupFontSizeModalHandlers();
    
    // Apply saved font size
    await applyFontSize();
    
    // Setup accessibility indicators for form inputs
    setupIndicators();
    
    // Setup custom validation messages
    setupCustomValidationMessages();
    
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
            
            const isShown = adminDropdown.classList.contains('show');
            
            // Close all other dropdowns
            document.querySelectorAll('.dropdown').forEach(d => {
                if (d !== adminDropdown) {
                    d.classList.remove('show');
                }
            });
            
            // Toggle this dropdown
            if (!isShown) {
                adminDropdown.classList.add('show');
            }
        });
        
        adminDropdown.addEventListener('click', function(e) {
            e.stopPropagation();
        });
        
        const userMgmtItem = adminDropdown.querySelector('.dropdown-item:nth-child(1)');
        if (userMgmtItem) {
            userMgmtItem.addEventListener('click', function(e) {
                console.log('User Management item clicked');
                window.location.href = HTML_FILES.USER_MANAGEMENT;
                adminDropdown.classList.remove('show');
            });
        }
        
        const categoryMgmtItem = adminDropdown.querySelector('.dropdown-item:nth-child(2)');
        if (categoryMgmtItem) {
            categoryMgmtItem.addEventListener('click', function(e) {
                console.log('Category Management item clicked');
                window.location.href = HTML_FILES.CATEGORY_MANAGEMENT;
                adminDropdown.classList.remove('show');
            });
        }
        
        const accountMgmtItem = adminDropdown.querySelector('.dropdown-item:nth-child(3)');
        if (accountMgmtItem) {
            accountMgmtItem.addEventListener('click', function(e) {
                console.log('Account Management item clicked');
                window.location.href = HTML_FILES.ACCOUNT_MANAGEMENT;
                adminDropdown.classList.remove('show');
            });
        }

        const shopMgmtItem = adminDropdown.querySelector('.dropdown-item:nth-child(4)');
        if (shopMgmtItem) {
            shopMgmtItem.addEventListener('click', function(e) {
                console.log('Shop Management item clicked');
                window.location.href = HTML_FILES.SHOP_MANAGEMENT;
                adminDropdown.classList.remove('show');
            });
        }

        const manufacturerMgmtItem = adminDropdown.querySelector('.dropdown-item:nth-child(5)');
        if (manufacturerMgmtItem) {
            manufacturerMgmtItem.addEventListener('click', function(e) {
                console.log('Manufacturer Management item clicked');
                window.location.href = HTML_FILES.MANUFACTURER_MANAGEMENT;
                adminDropdown.classList.remove('show');
            });
        }

        const productMgmtItem = adminDropdown.querySelector('.dropdown-item:nth-child(6)');
        if (productMgmtItem) {
            productMgmtItem.addEventListener('click', function(e) {
                console.log('Product Management item clicked');
                window.location.href = HTML_FILES.PRODUCT_MANAGEMENT;
                adminDropdown.classList.remove('show');
            });
        }

        const transactionMgmtItem = adminDropdown.querySelector('.dropdown-item:nth-child(7)');
        if (transactionMgmtItem) {
            transactionMgmtItem.addEventListener('click', function(e) {
                console.log('Transaction Management item clicked');
                window.location.href = HTML_FILES.TRANSACTION_MANAGEMENT;
                adminDropdown.classList.remove('show');
            });
        }

        const aggregationItem = adminDropdown.querySelector('.dropdown-item:nth-child(8)');
        if (aggregationItem) {
            aggregationItem.addEventListener('click', function(e) {
                console.log('Aggregation item clicked');
                window.location.href = HTML_FILES.AGGREGATION;
                adminDropdown.classList.remove('show');
            });
        }

        const aggregationDailyItem = adminDropdown.querySelector('.dropdown-item:nth-child(9)');
        if (aggregationDailyItem) {
            aggregationDailyItem.addEventListener('click', function(e) {
                console.log('Daily Aggregation item clicked');
                window.location.href = HTML_FILES.AGGREGATION_DAILY;
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
    const adminSetup = document.getElementById('admin-setup');
    const loginForm = document.getElementById('login-form');
    const appContent = document.getElementById('app-content');
    
    // Only run on login page (index.html)
    if (!adminSetup || !loginForm || !appContent) {
        return;
    }
    
    try {
        const needsSetup = await invoke('check_needs_setup');
        console.log('Needs setup:', needsSetup);
        
        if (needsSetup) {
            adminSetup.classList.remove('hidden');
            loginForm.classList.add('hidden');
            appContent.classList.add('hidden');
        } else {
            adminSetup.classList.add('hidden');
            loginForm.classList.remove('hidden');
            appContent.classList.add('hidden');
        }
    } catch (error) {
        console.error('Failed to check setup status:', error);
        // On error, assume setup is needed
        adminSetup.classList.remove('hidden');
        loginForm.classList.add('hidden');
        appContent.classList.add('hidden');
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
        alert(i18n.t('error.language_change_failed') + ': ' + error);
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
        const user = await invoke('login_user', {
            username: username,
            password: password
        });
        
        console.log('Login result:', user);
        
        messageDiv.textContent = i18n.t('login.success') + ' Welcome, ' + user.name + '!';
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

async function handleLogout() {
    console.log('Logout clicked');
    
    try {
        // Clear session
        await session.clearSession();
        console.log('Session cleared');
    } catch (error) {
        console.error('Failed to clear session:', error);
    }
    
    isLoggedIn = false;
    
    // Check if we're on the index page
    const loginForm = document.getElementById('login-form');
    const appContent = document.getElementById('app-content');
    
    if (loginForm && appContent) {
        // Index page - clear login form and show it
        document.getElementById('username').value = '';
        document.getElementById('password').value = '';
        document.getElementById('login-message').textContent = '';
        loginForm.classList.remove('hidden');
        appContent.classList.add('hidden');
    } else {
        // Management page - redirect to index
        window.location.href = HTML_FILES.INDEX;
    }
}

function handleQuit() {
    console.log('Quit clicked');
    invoke('handle_quit');
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

// Export functions for use in other modules
export {
    setupLanguageMenuHandlers,
    setupLanguageMenu,
    handleLanguageChange,
    handleLogout,
    handleQuit,
    setupCustomValidationMessages
};
