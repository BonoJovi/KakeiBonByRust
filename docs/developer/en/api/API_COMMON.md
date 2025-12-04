# Common API Reference

**Last Updated**: 2025-12-05 02:51 JST

## Overview

This document defines APIs that are commonly used across multiple screens.

---

## Table of Contents

1. [Session Management API](#session-management-api)
2. [Internationalization (I18n) API](#internationalization-i18n-api)
3. [Encryption Management API](#encryption-management-api)
4. [System API](#system-api)
5. [Validation API](#validation-api)

---

## Session Management API

### Data Structures

#### User

```rust
pub struct User {
    pub user_id: i64,
    pub name: String,
    pub role: i64,  // 0: Admin, 1: General User
}
```

#### SessionInfo

```rust
pub struct SessionInfo {
    pub source_screen: Option<String>,    // Source screen name
    pub category1_code: Option<String>,   // Selected category1 code
    pub modal_state: Option<String>,      // Modal state
}
```

---

### get_current_session_user

Retrieves the currently logged-in user information.

**Parameters:** None

**Return Value:**
- `Option<User>`: User information if logged in, `null` if not authenticated

**Usage Example:**
```javascript
const user = await invoke('get_current_session_user');
if (user) {
    console.log(`User: ${user.name}, Role: ${user.role}`);
}
```

---

### is_session_authenticated

Checks if the session is authenticated.

**Parameters:** None

**Return Value:**
- `bool`: `true` if authenticated, `false` if not

**Usage Example:**
```javascript
const isAuth = await invoke('is_session_authenticated');
if (!isAuth) {
    window.location.href = 'index.html';
}
```

---

### clear_session

Clears all session information (logout process).

**Parameters:** None

**Return Value:** None

**Usage Example:**
```javascript
await invoke('clear_session');
window.location.href = 'index.html';
```

**Note:**
- Clears both user information and all `SessionInfo` fields

---

### set_session_source_screen

Stores the source screen name in the session.

**Parameters:**
- `source_screen` (String): Screen name (e.g., "transaction-management")

**Return Value:** None

**Usage Example:**
```javascript
await invoke('set_session_source_screen', { 
    sourceScreen: 'transaction-management' 
});
```

---

### get_session_source_screen

Retrieves the source screen name stored in the session.

**Parameters:** None

**Return Value:**
- `Option<String>`: Screen name, or `null` if not set

**Usage Example:**
```javascript
const sourceScreen = await invoke('get_session_source_screen');
if (sourceScreen) {
    window.location.href = `${sourceScreen}.html`;
}
```

---

### clear_session_source_screen

Clears the source screen name from the session.

**Parameters:** None

**Return Value:** None

---

### set_session_category1_code

Stores the selected category1 code in the session.

**Parameters:**
- `category1_code` (String): Category1 code (e.g., "EXPENSE")

**Return Value:** None

**Usage Example:**
```javascript
await invoke('set_session_category1_code', { 
    category1Code: 'EXPENSE' 
});
```

---

### get_session_category1_code

Retrieves the category1 code stored in the session.

**Parameters:** None

**Return Value:**
- `Option<String>`: Category1 code, or `null` if not set

**Usage Example:**
```javascript
const category1Code = await invoke('get_session_category1_code');
```

---

### clear_session_category1_code

Clears the category1 code from the session.

**Parameters:** None

**Return Value:** None

---

### set_session_modal_state

Stores the modal state in the session.

**Parameters:**
- `modal_state` (String): Modal state string

**Return Value:** None

**Usage Example:**
```javascript
await invoke('set_session_modal_state', { 
    modalState: 'category-selection-open' 
});
```

---

### get_session_modal_state

Retrieves the modal state stored in the session.

**Parameters:** None

**Return Value:**
- `Option<String>`: Modal state, or `null` if not set

---

### clear_session_modal_state

Clears the modal state from the session.

**Parameters:** None

**Return Value:** None

---

## Internationalization (I18n) API

### get_translations

Retrieves all translations for the specified language.

**Parameters:**
- `language` (String): Language code ("ja" or "en")

**Return Value:**
- `HashMap<String, String>`: Key-value pairs of translations

**Usage Example:**
```javascript
const translations = await invoke('get_translations', { language: 'ja' });
document.getElementById('title').textContent = translations['app.title'];
```

**Use Cases:**
- Loading translations on page load
- Dynamic language switching

---

### get_i18n_resource

Retrieves a single translation resource.

**Parameters:**
- `key` (String): Translation key (e.g., "app.title")
- `lang` (String): Language code

**Return Value:**
- `String`: Translated text

**Usage Example:**
```javascript
const title = await invoke('get_i18n_resource', { 
    key: 'app.title', 
    lang: 'ja' 
});
```

**Error:**
- Returns error if key doesn't exist

---

### get_i18n_resources_by_category

Retrieves translations by category prefix.

**Parameters:**
- `category` (String): Category prefix (e.g., "button", "label")
- `lang` (String): Language code

**Return Value:**
- `HashMap<String, String>`: Translations with matching prefix

**Usage Example:**
```javascript
const buttonLabels = await invoke('get_i18n_resources_by_category', { 
    category: 'button', 
    lang: 'ja' 
});
// { "button.save": "保存", "button.cancel": "キャンセル", ... }
```

**Use Cases:**
- Loading all button labels
- Loading form field labels

---

### get_available_languages

Retrieves the list of available languages.

**Parameters:** None

**Return Value:**
- `Vec<String>`: Array of language codes

**Usage Example:**
```javascript
const languages = await invoke('get_available_languages');
// ["ja", "en"]
```

---

### get_language_names

Retrieves language code and display name pairs.

**Parameters:** None

**Return Value:**
- `Vec<(String, String)>`: Array of (code, name) tuples

**Usage Example:**
```javascript
const languageNames = await invoke('get_language_names');
// [["ja", "日本語"], ["en", "English"]]

languageNames.forEach(([code, name]) => {
    const option = document.createElement('option');
    option.value = code;
    option.textContent = name;
    selectElement.appendChild(option);
});
```

---

## Encryption Management API

### list_encrypted_fields

Retrieves a list of encrypted fields.

**Parameters:** None

**Return Value:**
- `Vec<String>`: Array of field names that are encrypted

**Usage Example:**
```javascript
const encryptedFields = await invoke('list_encrypted_fields');
console.log('Encrypted fields:', encryptedFields);
```

**Use Cases:**
- Identifying which fields are encrypted
- Data migration
- Security audits

---

### register_encrypted_field

Registers a field as encrypted.

**Parameters:**
- `field_name` (String): Field name to register

**Return Value:** None

**Usage Example:**
```javascript
await invoke('register_encrypted_field', { 
    fieldName: 'sensitive_data' 
});
```

**Note:**
- For managing encryption metadata
- Usually not called from frontend

---

## System API

### test_db_connection

Tests the database connection.

**Parameters:** None

**Return Value:**
- `bool`: `true` if connection successful, `false` otherwise

**Usage Example:**
```javascript
const isConnected = await invoke('test_db_connection');
if (!isConnected) {
    alert('Database connection failed');
}
```

**Use Cases:**
- Health checks
- Startup diagnostics
- Error troubleshooting

---

### adjust_window_size

Adjusts the window size.

**Parameters:**
- `width` (f64): Width in pixels
- `height` (f64): Height in pixels

**Return Value:** None

**Usage Example:**
```javascript
await invoke('adjust_window_size', {
    width: 1280,
    height: 720
});
```

---

### handle_quit

Handles application quit process.

**Parameters:** None

**Return Value:** None

**Usage Example:**
```javascript
await invoke('handle_quit');
```

**Note:**
- Performs cleanup before closing
- Usually called by window close handler

---

## Validation API

### validate_password_frontend

Validates a single password (frontend validation).

**Parameters:**
- `password` (String): Password to validate

**Return Value:**
- `Result<(), Vec<String>>`: Empty on success, array of error messages on failure

**Usage Example:**
```javascript
try {
    await invoke('validate_password_frontend', { password });
    // Validation passed
} catch (errors) {
    // Display errors
    errors.forEach(error => {
        console.error('Validation error:', error);
    });
}
```

**Validation Rules:**
- Minimum 16 characters
- Additional rules defined in validation service

---

### validate_passwords_frontend

Validates password and confirmation match (frontend validation).

**Parameters:**
- `password` (String): Password
- `password_confirmation` (String): Confirmation password

**Return Value:**
- `Result<(), Vec<String>>`: Empty on success, array of error messages on failure

**Usage Example:**
```javascript
try {
    await invoke('validate_passwords_frontend', { 
        password, 
        passwordConfirmation 
    });
    // Passwords are valid and match
} catch (errors) {
    errors.forEach(error => {
        alert(error);
    });
}
```

**Validation Rules:**
- Both passwords must meet minimum requirements
- Passwords must match

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"User not authenticated"` | Session not authenticated | Login required |
| `"Failed to get i18n resource: ..."` | Translation key not found | Check key name |
| `"Database connection failed"` | Database connection error | Check database |
| `"Password too short"` | Password < 16 characters | Use longer password |
| `"Passwords do not match"` | Confirmation doesn't match | Re-enter passwords |

### Frontend Error Handling Example

```javascript
// Session authentication check
async function checkAuth() {
    try {
        const isAuth = await invoke('is_session_authenticated');
        if (!isAuth) {
            window.location.href = 'index.html';
        }
    } catch (error) {
        console.error('Auth check failed:', error);
        window.location.href = 'index.html';
    }
}

// Translation loading
async function loadTranslations(lang) {
    try {
        const translations = await invoke('get_translations', { language: lang });
        applyTranslations(translations);
    } catch (error) {
        console.error('Failed to load translations:', error);
        // Fallback to default language
        const defaultTrans = await invoke('get_translations', { language: 'ja' });
        applyTranslations(defaultTrans);
    }
}

// Password validation
async function validatePasswordInput(password, confirmation) {
    try {
        await invoke('validate_passwords_frontend', { 
            password, 
            passwordConfirmation: confirmation 
        });
        return true;
    } catch (errors) {
        // Display all validation errors
        const errorList = document.getElementById('error-list');
        errorList.innerHTML = '';
        errors.forEach(error => {
            const li = document.createElement('li');
            li.textContent = error;
            errorList.appendChild(li);
        });
        return false;
    }
}
```

---

## Usage Examples

### Session Management

```javascript
// Login flow
async function handleLogin(username, password) {
    try {
        const user = await invoke('login_user', { username, password });
        
        // Check if logged in
        const currentUser = await invoke('get_current_session_user');
        console.log('Logged in as:', currentUser.name);
        
        // Navigate to main screen
        window.location.href = 'transaction-management.html';
    } catch (error) {
        alert(`Login failed: ${error}`);
    }
}

// Logout
async function handleLogout() {
    await invoke('clear_session');
    window.location.href = 'index.html';
}

// Screen navigation with source tracking
async function navigateToCategoryManagement() {
    await invoke('set_session_source_screen', { 
        sourceScreen: 'transaction-management' 
    });
    window.location.href = 'category-management.html';
}

// Return to source screen
async function returnToSource() {
    const sourceScreen = await invoke('get_session_source_screen');
    if (sourceScreen) {
        await invoke('clear_session_source_screen');
        window.location.href = `${sourceScreen}.html`;
    } else {
        window.location.href = 'transaction-management.html'; // Default
    }
}
```

### Password Validation

```javascript
// Form validation
async function handlePasswordChange(event) {
    event.preventDefault();
    
    const password = document.getElementById('password').value;
    const confirmation = document.getElementById('password-confirmation').value;
    
    try {
        await invoke('validate_passwords_frontend', { 
            password, 
            passwordConfirmation: confirmation 
        });
        
        // Validation passed - proceed with password change
        await invoke('change_password', { password });
        alert('Password changed successfully');
    } catch (errors) {
        alert(`Validation failed:\n${errors.join('\n')}`);
    }
}
```

### Encryption

```javascript
// Check if field is encrypted
async function isFieldEncrypted(fieldName) {
    const encryptedFields = await invoke('list_encrypted_fields');
    return encryptedFields.includes(fieldName);
}

// Display encrypted data warning
async function showEncryptedFieldWarning() {
    const encryptedFields = await invoke('list_encrypted_fields');
    if (encryptedFields.length > 0) {
        console.log('Encrypted fields:', encryptedFields.join(', '));
    }
}
```

---

## Security Considerations

### Session Management

1. **Authentication check**: Always verify on page load
2. **Automatic logout**: Clear session on browser close
3. **Timeout**: Consider implementing session timeout

### Password Handling

1. **Minimum length**: 16 characters enforced
2. **Never log passwords**: Don't output to console
3. **Server-side validation**: Frontend validation is supplementary

### Encryption

1. **Encrypted fields**: Memos and sensitive data
2. **Field registration**: Managed by system
3. **Audit trail**: Track encrypted field access

---

## Test Coverage

**SessionService:**
- ✅ User session management
- ✅ SessionInfo management
- ✅ Authentication check
- ✅ Session clear

**I18nService:**
- ✅ Translation retrieval
- ✅ Multi-language support
- ✅ Category-based retrieval
- ✅ Available languages

**ValidationService:**
- ✅ Password validation
- ✅ Password confirmation matching
- ✅ Minimum length enforcement

---

## Related Documents

### Implementation Files

- Session Service: `src/services/session.rs`
- I18n Service: `src/services/i18n.rs`
- Validation Service: `src/validation.rs`
- Tauri Commands: `src/lib.rs`

### Other API References

- [Authentication API](./API_AUTH.md) - Login/Setup
- [User Management API](./API_USER.md) - User CRUD operations
- [Settings API](./API_SETTINGS.md) - Language and font settings

---

**Change History:**
- 2025-12-05: Created (based on implementation code)
