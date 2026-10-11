# User Management System

## Overview
Implementation of user registration, authentication, and management functions. Supports both administrator and general users with role-based access control.

## User Roles
- **Administrator (ROLE_ADMIN = 1)**: Full system access, can manage users
- **General User (ROLE_USER = 2)**: Can use basic household budget functions

## Main Features

### 1. Administrator Registration (`register_admin`)
Initial setup function to register the first administrator user.

**Conditions**:
- Can only be executed when no administrators exist
- Returns error if an administrator already exists

**Parameters**:
- `username`: Administrator username (16-128 characters)
- `password`: Password (16-128 characters, complexity required)

**Security**:
- Password hashing with Argon2id
- Encrypted master key saved in database

### 2. General User Registration (`register_user`, `create_general_user`)
Function to register general users.

**Two Registration Methods**:
1. `register_user`: First general user registration (when no general users exist)
2. `create_general_user`: Administrator registers general users

**Parameters**:
- `username`: Username (16-128 characters)
- `password`: Password (16-128 characters, complexity required)

**Security**:
- Password hashing with Argon2id
- Encrypted user-specific key saved in database

### 3. User Authentication (`login_user`)
Login function.

**Parameters**:
- `username`: Username
- `password`: Password

**Return Value**:
- User ID
- Username
- Role (1: Administrator, 2: General User)

**Error Cases**:
- Non-existent user
- Incorrect password

### 4. User List Retrieval (`list_users`)
Retrieves list of all users (administrator only).

**Return Value**: User list
- User ID
- Username
- Role
- Registration date
- Update date

### 5. User Information Retrieval (`get_user`)
Retrieves specific user information.

**Parameters**:
- `user_id`: User ID

**Return Value**:
- User ID
- Username
- Role
- Registration date
- Update date

### 6. General User Rename by an Administrator (`update_general_user_info`)
An administrator renames another general user. No password is taken or changed.

**Parameters**:
- `user_id`: The user to rename
- `username`: New username (optional)

**Permissions**:
- Administrator renaming another user only; renaming yourself is refused here
  because it needs your current password (use command 7)

### 7. Editing Your Own Account (`update_general_user_with_reencryption`)
Renames your own account, changes your password and re-encrypts all
encrypted fields, or both. The current password is always required.

**Parameters**:
- `old_password`: Current password (checked against the stored one)
- `username`: New username (optional)
- `new_password`: New password (optional; null renames only)

**Processing Flow (password change)**:
1. Verify current password
2. Decrypt all encrypted fields with old key
3. Generate new key from new password
4. Re-encrypt all data with new key
5. Update user information

**Security**:
- Performed within transaction
- Rollback on error

### 8. Administrator Rename (`update_admin_user_info`)
Same as command 6 for an administrator account. The administrator cannot
rename their own account through it.

### 9. Administrator Editing Their Own Account (`update_admin_user_with_reencryption`)
Same as command 7 for the administrator.

**Important Notes**:
- Only one administrator can exist in the system
- Re-encrypts all general users' encrypted data
- Re-encrypts administrator's own encrypted data

### 10. General User Deletion (`delete_general_user_info`)
Deletes general user.

**Parameters**:
- `user_id`: User ID to delete
- `admin_password`: Administrator password (for verification)

**Processing**:
- Deletes user record
- Deletes all encrypted data for that user
- Transaction processing

## Password Validation

### Password Requirements
- **Length**: 16-128 characters
- **Complexity**:
  - At least one uppercase letter (A-Z)
  - At least one lowercase letter (a-z)
  - At least one digit (0-9)
  - At least one special character (!@#$%^&*()_+-=[]{}|;:,.<>?)

### Validation Functions
- `validate_password_frontend`: Validates single password
- `validate_passwords_frontend`: Validates password and confirmation password match

## Security Features

### 1. Password Hashing
- Algorithm: Argon2id
- Default parameters
- Salt automatically generated

### 2. Encrypted Data Management
- Encryption key derived from user password
- User-specific encryption key
- Master key for administrator

### 3. Re-encryption System
- Re-encrypts all encrypted fields when password changes
- Transaction processing
- Rollback on error

## Testing

### Implemented Tests
1. Administrator registration
2. General user registration
3. User authentication
4. User list retrieval
5. User information update
6. User information update with re-encryption
7. Password validation

### Running Tests
```bash
cargo test services::user_management::tests --lib
cargo test services::auth::tests --lib
```

## Usage Example

### Frontend (JavaScript)
```javascript
// Administrator registration
await invoke('register_admin', {
  username: 'admin',
  password: 'SecurePassword123!'
});

// General user registration
await invoke('register_user', {
  username: 'user1',
  password: 'UserPassword123!'
});

// Login
const user = await invoke('login_user', {
  username: 'user1',
  password: 'UserPassword123!'
});

// Password change
await invoke('update_general_user_with_reencryption', {
  userId: user.user_id,
  newPassword: 'NewPassword123!',
  currentPassword: 'UserPassword123!'
});
```

## Database Schema

### USERS Table
```sql
CREATE TABLE USERS (
    USER_ID INTEGER NOT NULL,
    NAME VARCHAR(128) NOT NULL UNIQUE,
    PAW VARCHAR(128) NOT NULL,
    ROLE INTEGER NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID)
);
```

## Implementation Status
- ✅ Administrator registration
- ✅ General user registration
- ✅ User authentication
- ✅ User list retrieval
- ✅ User information update
- ✅ User information update with re-encryption
- ✅ User deletion
- ✅ Password validation
- ✅ All tests passing
