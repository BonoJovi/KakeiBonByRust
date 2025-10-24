import { invoke } from '@tauri-apps/api/core';

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

document.addEventListener('DOMContentLoaded', function() {
    console.log('DOM loaded');
    
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
            console.log('Document clicked, closing dropdown');
            fileDropdown.classList.remove('show');
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

async function handleAdminSetup(e) {
    e.preventDefault();
    console.log('Admin setup form submitted');
    
    const username = document.getElementById('admin-username').value;
    const password = document.getElementById('admin-password').value;
    const passwordConfirm = document.getElementById('admin-password-confirm').value;
    const messageDiv = document.getElementById('setup-message');
    
    if (!password || password.trim() === '') {
        messageDiv.textContent = 'Password cannot be empty!';
        messageDiv.className = 'message error';
        return;
    }
    
    if (password.length < 16) {
        messageDiv.textContent = 'Password must be at least 16 characters long!';
        messageDiv.className = 'message error';
        return;
    }
    
    if (password !== passwordConfirm) {
        messageDiv.textContent = 'Passwords do not match!';
        messageDiv.className = 'message error';
        return;
    }
    
    try {
        const result = await invoke('register_admin', {
            username: username,
            password: password
        });
        
        console.log('Admin registration result:', result);
        messageDiv.textContent = 'Administrator registered successfully! Please login.';
        messageDiv.className = 'message success';
        
        setTimeout(() => {
            document.getElementById('admin-setup').classList.add('hidden');
            document.getElementById('login-form').classList.remove('hidden');
        }, 2000);
        
    } catch (error) {
        console.error('Admin registration error:', error);
        messageDiv.textContent = 'Registration failed: ' + error;
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
        messageDiv.textContent = 'Password cannot be empty!';
        messageDiv.className = 'message error';
        return;
    }
    
    if (password.length < 16) {
        messageDiv.textContent = 'Password must be at least 16 characters long!';
        messageDiv.className = 'message error';
        return;
    }
    
    if (password !== passwordConfirm) {
        messageDiv.textContent = 'Passwords do not match!';
        messageDiv.className = 'message error';
        return;
    }
    
    try {
        const result = await invoke('register_user', {
            username: username,
            password: password
        });
        
        console.log('User registration result:', result);
        messageDiv.textContent = 'User registered successfully!';
        messageDiv.className = 'message success';
        
        setTimeout(() => {
            document.getElementById('user-setup').classList.add('hidden');
            document.getElementById('app-content').classList.remove('hidden');
        }, 2000);
        
    } catch (error) {
        console.error('User registration error:', error);
        messageDiv.textContent = 'Registration failed: ' + error;
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
        messageDiv.textContent = 'Login successful!';
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
        messageDiv.textContent = 'Login failed: ' + error;
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
