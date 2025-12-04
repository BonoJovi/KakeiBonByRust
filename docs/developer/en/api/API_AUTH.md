# Authentication & Setup API Reference

**Last Updated**: 2025-12-05 02:18 JST

## Overview

This document defines APIs used in the authentication and initial setup screen (index.html).

---

## Table of Contents

1. [Setup API](#setup-api)
2. [Authentication API](#authentication-api)
3. [Data Structures](#data-structures)

---

## Setup API

### check_admin_exists

Checks if an admin user exists.

**Parameters:** None

**Return Value:**
- `bool`: `true` if admin exists, `false` otherwise

**Usage Example:**
```javascript
const adminExists = await invoke('check_admin_exists');
if (!adminExists) {
    showAdminSetupForm();
}
```

---

### setup_admin

Creates the initial admin user.

**Parameters:**
- `username` (String): Admin username
- `password` (String): Admin password

**Return Value:**
- `String`: "Admin setup completed successfully"

**Usage Example:**
```javascript
try {
    await invoke('setup_admin', {
        username: 'admin',
        password: 'securePassword123456'
    });
    alert('Admin created successfully');
    window.location.reload();
} catch (error) {
    alert(`Setup failed: ${error}`);
}
```

**Validation:**
- Username: Required
- Password: Minimum 16 characters

**Automatic Processing:**
- Creates user with role = 0 (Admin)
- Initializes database tables if needed

---

## Authentication API

### login_user

Authenticates a user and creates a session.

**Parameters:**
- `username` (String): Username
- `password` (String): Password

**Return Value:**
- `User`: User information

**Usage Example:**
```javascript
try {
    const user = await invoke('login_user', {
        username,
        password
    });
    
    console.log(`Logged in as: ${user.name} (Role: ${user.role})`);
    window.location.href = 'user-management.html';
} catch (error) {
    alert(`Login failed: ${error}`);
}
```

**Error Messages:**
- `"User not found"`: Username doesn't exist
- `"Invalid password"`: Password incorrect
- `"Failed to verify password: ..."`: System error

---

### logout_user

Logs out the current user and clears the session.

**Parameters:** None

**Return Value:** None

**Usage Example:**
```javascript
await invoke('logout_user');
window.location.href = 'index.html';
```

**Note:**
- Equivalent to `clear_session` from Common API
- Clears all session data

---

## Data Structures

### User

```rust
pub struct User {
    pub user_id: i64,
    pub name: String,
    pub role: i64,  // 0: Admin, 1: General User
}
```

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"Admin user already exists"` | Admin already setup | Use login instead |
| `"User not found"` | Username doesn't exist | Check username |
| `"Invalid password"` | Password incorrect | Check password |
| `"Password too short"` | Password < 16 characters | Use longer password |
| `"Failed to verify password: ..."` | System error | Check database |

### Frontend Error Handling Example

```javascript
// Admin setup
async function handleAdminSetup(event) {
    event.preventDefault();
    
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    const confirmation = document.getElementById('confirmation').value;
    
    // Validate passwords match
    if (password !== confirmation) {
        alert('Passwords do not match');
        return;
    }
    
    try {
        await invoke('setup_admin', { username, password });
        alert('Admin created successfully');
        window.location.reload();
    } catch (error) {
        if (error.includes('already exists')) {
            alert('Admin already exists. Please login.');
        } else if (error.includes('too short')) {
            alert('Password must be at least 16 characters');
        } else {
            alert(`Setup failed: ${error}`);
        }
    }
}

// Login
async function handleLogin(event) {
    event.preventDefault();
    
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    
    try {
        const user = await invoke('login_user', { username, password });
        
        // Navigate based on role
        if (user.role === 0) {
            // Admin
            window.location.href = 'user-management.html';
        } else {
            // General user
            window.location.href = 'transaction-management.html';
        }
    } catch (error) {
        if (error.includes('User not found')) {
            alert('Username not found');
        } else if (error.includes('Invalid password')) {
            alert('Password incorrect');
        } else {
            alert(`Login failed: ${error}`);
        }
    }
}
```

---

## Usage Example: Login Screen Implementation

### Initial Check

```javascript
async function initializeLoginScreen() {
    try {
        const adminExists = await invoke('check_admin_exists');
        
        if (!adminExists) {
            // Show admin setup form
            document.getElementById('setup-form').style.display = 'block';
            document.getElementById('login-form').style.display = 'none';
        } else {
            // Show login form
            document.getElementById('setup-form').style.display = 'none';
            document.getElementById('login-form').style.display = 'block';
        }
    } catch (error) {
        console.error('Initialization error:', error);
    }
}

// Run on page load
document.addEventListener('DOMContentLoaded', initializeLoginScreen);
```

### Admin Setup Flow

```javascript
async function setupAdmin(username, password) {
    try {
        // Validate password first
        await invoke('validate_password_frontend', { password });
        
        // Create admin
        await invoke('setup_admin', { username, password });
        
        alert('Admin created successfully');
        window.location.reload();
    } catch (error) {
        alert(`Setup failed: ${error}`);
    }
}
```

### Login Flow

```javascript
async function login(username, password) {
    try {
        const user = await invoke('login_user', { username, password });
        
        // Store user info in session (already done by backend)
        const currentUser = await invoke('get_current_session_user');
        console.log('Logged in as:', currentUser);
        
        // Navigate to appropriate screen
        navigateAfterLogin(user.role);
    } catch (error) {
        handleLoginError(error);
    }
}

function navigateAfterLogin(role) {
    if (role === 0) {
        // Admin: Go to user management
        window.location.href = 'user-management.html';
    } else {
        // General user: Go to transaction management
        window.location.href = 'transaction-management.html';
    }
}

function handleLoginError(error) {
    if (error.includes('User not found')) {
        alert('Username not found. Please check your username.');
    } else if (error.includes('Invalid password')) {
        alert('Password incorrect. Please try again.');
    } else {
        alert(`Login failed: ${error}`);
    }
}
```

---

## Security Considerations

### Password Security

1. **Minimum length**: 16 characters enforced
2. **Hashing**: Argon2 used for password storage
3. **Never exposed**: Passwords never returned in responses

### Admin Setup

1. **One-time only**: Admin can only be created once
2. **Secure initial setup**: Ensure strong password
3. **Post-setup**: Additional admins added via user management

### Session Security

1. **Session creation**: Only on successful login
2. **Session clearing**: Always clear on logout
3. **Authentication check**: Required on all protected pages

---

## Test Coverage

**AuthService:**
- ✅ Admin existence check
- ✅ Admin setup (one-time only)
- ✅ Login authentication
- ✅ Password verification
- ✅ Session creation
- ✅ Logout and session clearing

---

## Related Documents

### Implementation Files

- Auth Service: `src/services/auth.rs`
- User Service: `src/services/user_management.rs`
- Security: `src/security.rs`
- Tauri Commands: `src/lib.rs`

### Other API References

- [Common API](./API_COMMON.md) - Session management
- [User Management API](./API_USER.md) - User CRUD operations

---

**Change History:**
- 2025-12-05: Created (based on implementation code)
