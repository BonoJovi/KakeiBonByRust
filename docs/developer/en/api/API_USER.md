# User Management API Reference

**Last Updated**: 2025-12-05 02:21 JST

## Overview

This document defines APIs used in the user management screen (user-management.html). It provides functionality for administrators to manage general users and for users to edit their own information.

---

## Table of Contents

1. [User List and Retrieval API](#user-list-and-retrieval-api)
2. [User Creation API](#user-creation-api)
3. [User Update API](#user-update-api)
4. [User Deletion API](#user-deletion-api)
5. [User Settings API](#user-settings-api)
6. [Data Structures](#data-structures)

---

## User List and Retrieval API

### list_users

Retrieves a list of all users.

**Parameters:** None

**Return Value:**
- `Vec<UserInfo>`: Array of user information

**UserInfo Structure:**
```javascript
{
    user_id: number,
    name: string,
    role: number,       // 0: Admin, 1: General User
    entry_dt: string,   // "YYYY-MM-DD HH:MM:SS"
    update_dt: string | null
}
```

**Usage Example:**
```javascript
const users = await invoke('list_users');
users.forEach(user => {
    console.log(`${user.name} (${user.role === 0 ? 'Admin' : 'User'})`);
});
```

**Permissions:**
- Admin only (session check recommended)

---

### get_user

Retrieves specific user information.

**Parameters:**
- `user_id` (i64): User ID

**Return Value:**
- `UserInfo`: User information

**Usage Example:**
```javascript
const user = await invoke('get_user', { userId: 2 });
console.log(user.name);
```

**Note:**
- Used when admin queries other users
- Use `get_current_session_user` to get own information

**Error:**
- `"Failed to get user: User not found"`: User doesn't exist

---

## User Creation API

### create_general_user

Creates a new general user.

**Parameters:**
- `username` (String): Username
- `password` (String): Password (minimum 16 characters)

**Return Value:**
- `i64`: Created user ID

**Usage Example:**
```javascript
try {
    const userId = await invoke('create_general_user', {
        username: 'user01',
        password: 'SecurePassword123456'
    });
    console.log(`Created user ID ${userId}`);
    
    // Reload user list
    await loadUsers();
} catch (error) {
    alert(`User creation failed: ${error}`);
}
```

**Automatic Processing:**
1. Argon2 password hashing
2. Encryption key generation
3. Default category insertion (20 category2, 126 category3)
4. Role automatically set to general user (ROLE_USER = 1)

**Validation:**
- Password must be 16+ characters
- Username uniqueness check

**Errors:**
- `"Password must be at least 16 characters long"`: Password too short
- `"Username already exists"`: Duplicate username
- `"Failed to create user: ..."`: Database error

---

## User Update API

### update_general_user_info

Logged-in general user updates their own information (without password change).

**Parameters:**
- `username` (Option\<String\>): New username (if changing)
- `password` (Option\<String\>): New password (if changing)

**Return Value:** None

**Usage Example:**
```javascript
// Change username only
await invoke('update_general_user_info', {
    username: 'new_username',
    password: null
});

// Change password only
await invoke('update_general_user_info', {
    username: null,
    password: 'NewPassword123456'
});
```

**Note:**
- Session user ID automatically retrieved (`get_session_user_id`)
- When changing password, encrypted data is **NOT re-encrypted**
- Use `update_general_user_with_reencryption` if encrypted data exists

**Permissions:**
- General user updating themselves

---

### update_general_user_with_reencryption

Changes password and re-encrypts encrypted data simultaneously.

**Parameters:**
- `old_password` (String): Current password (for authentication)
- `username` (Option\<String\>): New username
- `new_password` (Option\<String\>): New password

**Return Value:** None

**Usage Example:**
```javascript
try {
    await invoke('update_general_user_with_reencryption', {
        oldPassword: 'CurrentPassword123456',
        username: null,
        newPassword: 'NewPassword123456'
    });
    alert('Password changed. Encrypted data re-encrypted.');
} catch (error) {
    alert(`Update failed: ${error}`);
}
```

**Processing Flow:**
1. Authenticate with current password
2. Decrypt encrypted data (old password)
3. Update user information
4. Re-encrypt data (new password)

**Errors:**
- `"Invalid password"`: Current password incorrect
- `"Password must be at least 16 characters long"`: New password too short
- `"Failed to update user: ..."`: Update error

**Important:**
- Must use this API if encrypted data exists
- Use only when changing password (use `update_general_user_info` for username only)

---

### update_admin_user_info

Logged-in administrator updates their own information (without password change).

**Parameters:**
- `username` (Option\<String\>): New username
- `password` (Option\<String\>): New password

**Return Value:** None

**Usage Example:**
```javascript
await invoke('update_admin_user_info', {
    username: 'new_admin_name',
    password: null
});
```

**Note:**
- Same behavior as `update_general_user_info` for general users
- Admin only (internal role check exists)

---

### update_admin_user_with_reencryption

Admin password change with encrypted data re-encryption.

**Parameters:**
- `old_password` (String): Current password
- `username` (Option\<String\>): New username
- `new_password` (Option\<String\>): New password

**Return Value:** None

**Usage Example:**
```javascript
await invoke('update_admin_user_with_reencryption', {
    oldPassword: 'CurrentAdminPassword',
    username: null,
    newPassword: 'NewAdminPassword123'
});
```

**Note:**
- Same behavior as `update_general_user_with_reencryption`
- Admin only

---

## User Deletion API

### delete_general_user_info

Deletes a general user.

**Parameters:**
- `user_id` (i64): User ID to delete

**Return Value:** None

**Usage Example:**
```javascript
if (confirm('Are you sure you want to delete this user?')) {
    try {
        await invoke('delete_general_user_info', { userId: 3 });
        alert('User deleted');
        await loadUsers(); // Reload list
    } catch (error) {
        alert(`Deletion failed: ${error}`);
    }
}
```

**Constraints:**
- Admin users cannot be deleted
- Physical deletion (not logical)

**Errors:**
- `"Admin user cannot be deleted"`: Attempted to delete admin
- `"User not found"`: User doesn't exist
- `"Failed to delete user: ..."`: Deletion error

**Cascade Deletion:**
- User-related data (transactions, categories, etc.) also deleted
- Automatically deleted by foreign key constraints

---

## User Settings API

### get_user_settings

Retrieves current user settings.

**Parameters:** None

**Return Value:**
```javascript
{
    language: string,    // "ja" | "en"
    font_size: string    // "small" | "medium" | "large"
}
```

**Usage Example:**
```javascript
const settings = await invoke('get_user_settings');
console.log(`Language: ${settings.language}, Font size: ${settings.font_size}`);
```

**Default Values:**
- `language`: "ja" (LANG_DEFAULT)
- `font_size`: "medium" (FONT_SIZE_DEFAULT)

---

### update_user_settings

Updates user settings.

**Parameters:**
- `settings` (Object): Map of setting keys and values

**Usage Example:**
```javascript
await invoke('update_user_settings', {
    settings: {
        language: 'en',
        font_size: 'large'
    }
});
```

**Note:**
- Can update partial settings (only specified keys)
- Settings saved to application configuration file

---

## Data Structures

### UserInfo

```rust
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub user_id: i64,
    pub name: String,
    pub role: i64,          // 0: Admin, 1: General User
    pub entry_dt: String,   // "YYYY-MM-DD HH:MM:SS"
    pub update_dt: Option<String>,
}
```

**Role Constants:**
- `ROLE_ADMIN = 0`: Administrator
- `ROLE_USER = 1`: General User

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"User not authenticated"` | Session not authenticated | Login required |
| `"Password must be at least 16 characters long"` | Password too short | Use 16+ characters |
| `"Username already exists"` | Duplicate username | Use different name |
| `"User not found"` | User doesn't exist | Specify correct ID |
| `"Admin user cannot be deleted"` | Cannot delete admin | Only general users can be deleted |
| `"Invalid password"` | Current password incorrect | Enter correct password |
| `"Failed to ... user: ..."` | Database error | Check database |

### Frontend Error Handling Example

```javascript
// User creation
async function createUser(username, password) {
    try {
        // Validation
        await invoke('validate_password_frontend', { password });
        
        // Create
        const userId = await invoke('create_general_user', { 
            username, 
            password 
        });
        
        alert(`Created user ID ${userId}`);
        return userId;
    } catch (error) {
        if (error.includes('already exists')) {
            alert('This username is already taken');
        } else if (error.includes('16 characters')) {
            alert('Password must be at least 16 characters');
        } else {
            alert(`Error: ${error}`);
        }
        return null;
    }
}

// Password change (with re-encryption)
async function changePassword(oldPassword, newPassword) {
    try {
        await invoke('update_general_user_with_reencryption', {
            oldPassword,
            username: null,
            newPassword
        });
        
        alert('Password changed');
        return true;
    } catch (error) {
        if (error.includes('Invalid password')) {
            alert('Current password is incorrect');
        } else {
            alert(`Error: ${error}`);
        }
        return false;
    }
}
```

---

## Security Considerations

### Password Management

1. **Minimum length**: 16 characters (MIN_PASSWORD_LENGTH)
2. **Hashing**: Argon2 (automatic salt generation)
3. **Storage**: Only hashed passwords stored in DB
4. **Re-encryption**: Must use re-encryption API when changing password

### Permission Management

1. **Administrator**:
   - Can view, create, and delete all users
   - Can update own information
2. **General User**:
   - Can only update own information
   - Cannot view or modify other users

### Encrypted Data

1. **Encryption key**: Derived from user password
2. **Re-encryption**: Required when changing password
3. **Target data**: Transaction memos, etc.

**Important:** If re-encryption is forgotten during password change, encrypted data becomes unrecoverable.

---

## Usage Example: User Management Screen Implementation

### User List Display

```javascript
async function loadUsers() {
    try {
        const users = await invoke('list_users');
        
        const tbody = document.getElementById('user-table-body');
        tbody.innerHTML = '';
        
        users.forEach(user => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td>${user.user_id}</td>
                <td>${user.name}</td>
                <td>${user.role === 0 ? 'Admin' : 'General User'}</td>
                <td>${user.entry_dt}</td>
                <td>
                    ${user.role === 1 ? `
                        <button onclick="deleteUser(${user.user_id})">Delete</button>
                    ` : ''}
                </td>
            `;
            tbody.appendChild(row);
        });
    } catch (error) {
        console.error('User list loading error:', error);
    }
}
```

### User Creation Form

```javascript
async function handleCreateUser(event) {
    event.preventDefault();
    
    const username = document.getElementById('new-username').value;
    const password = document.getElementById('new-password').value;
    const confirmPassword = document.getElementById('confirm-password').value;
    
    // Client-side validation
    if (password !== confirmPassword) {
        alert('Passwords do not match');
        return;
    }
    
    try {
        // Server-side validation
        await invoke('validate_passwords_frontend', { 
            password, 
            passwordConfirm: confirmPassword 
        });
        
        // Create user
        const userId = await invoke('create_general_user', { 
            username, 
            password 
        });
        
        alert(`Created user "${username}" (ID: ${userId})`);
        
        // Reset form
        event.target.reset();
        
        // Reload list
        await loadUsers();
    } catch (error) {
        alert(`Error: ${error}`);
    }
}
```

---

## Test Coverage

**UserManagementService:**
- ✅ User list retrieval test
- ✅ User creation test
- ✅ User update test (username/password)
- ✅ User deletion test
- ✅ Duplicate username check
- ✅ Re-encryption test on password change
- ✅ Admin deletion prevention test

**Validation:**
- ✅ Password length check
- ✅ Password match check

---

## Related Documents

### Implementation Files

- User Management Service: `src/services/user_management.rs`
- Authentication Service: `src/services/auth.rs`
- Encryption Service: `src/services/encryption.rs`
- Security: `src/security.rs`
- Validation: `src/validation.rs`
- Tauri Commands: `src/lib.rs`

### Other API References

- [Common API](./API_COMMON.md) - Session management, validation
- [Authentication & Setup API](./API_AUTH.md) - Login, initial setup
- [Settings API](./API_SETTINGS.md) - Application settings

---

**Change History:**
- 2025-12-05: Created (based on implementation code)
