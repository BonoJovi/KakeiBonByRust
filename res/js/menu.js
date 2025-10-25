import { invoke } from '@tauri-apps/api/core';
import i18n from './i18n.js';
import { setupIndicators } from './indicators.js';

let currentFontSize = 16;
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
    
    // Setup accessibility indicators for form inputs
    setupIndicators();
    
    // Check if initial setup is needed
    checkSetupNeeded();
    
    const fileMenu = document.getElementById('file-menu');
    const fileDropdown = document.getElementById('file-dropdown');
    
    console.log('fileMenu:', fileMenu);
    console.log('fileDropdown:', fileDropdown);
    
    if (fileMenu && fileDropdown) {
        console.log('Adding click event listener to file menu');
        fileMenu.addEventListener('click', function(e) {
            console.log('File menu clicked!');
            e.stopPropagation();
            const isShown = fileDropdown.classList.contains('show');
            console.log('Before toggle - has show class:', isShown);
            fileDropdown.classList.toggle('show');
            console.log('After toggle - classes:', fileDropdown.className);
            console.log('Computed display style:', window.getComputedStyle(fileDropdown).display);
        });
        
        // Prevent dropdown from closing when clicking inside it
        fileDropdown.addEventListener('click', function(e) {
            e.stopPropagation();
        });
        
        document.addEventListener('click', function() {
            console.log('Document clicked, closing all dropdowns');
            document.querySelectorAll('.dropdown').forEach(dropdown => {
                dropdown.classList.remove('show');
            });
        });
        
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
    
    // Setup dropdown toggle - only once
    languageMenu.addEventListener('click', function(e) {
        console.log('Language menu clicked');
        e.stopPropagation();
        
        // Close other dropdowns
        document.querySelectorAll('.dropdown').forEach(dropdown => {
            if (dropdown !== languageDropdown) {
                dropdown.classList.remove('show');
            }
        });
        
        languageDropdown.classList.toggle('show');
        console.log('Language dropdown toggled, show class:', languageDropdown.classList.contains('show'));
    });
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

function increaseFontSize() {
    currentFontSize += 2;
    applyFontSize();
}

function decreaseFontSize() {
    if (currentFontSize > 10) {
        currentFontSize -= 2;
        applyFontSize();
    }
}

function applyFontSize() {
    document.documentElement.style.fontSize = currentFontSize + 'px';
}
