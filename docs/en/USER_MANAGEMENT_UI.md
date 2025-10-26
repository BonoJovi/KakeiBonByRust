# User Management UI Implementation Documentation

## Overview
Implemented the frontend for the user management screen. This web interface allows administrators to add, edit, and delete general users.

## Implementation Date
2025-10-25

## Implementation Files

### HTML
- `res/user-management.html` - Main HTML for user management screen

### CSS
- `res/css/user-management.css` - User management screen specific styles
- `res/css/indicators.css` - Common accessibility indicator styles (newly created)

### JavaScript
- `res/js/user-management.js` - User management screen logic
- `res/js/indicators.js` - Common indicator module (newly created)
- `res/js/consts.js` - Constants definition module

## Screen Layout

### Menu Bar
- **File Menu**
  - Back to Main: Return to main screen
  - Logout: Log out
  - Quit: Exit application
  
- **Language Menu**
  - Dynamically generated language selection menu
  - Indicator displayed for currently selected language

### User List Section
Displays user information in table format:
- User ID
- Username
- Role (Admin/User)
- Created At
- Updated At
- Actions (Edit/Delete buttons)

**Features**
- Admin users do not show delete button (cannot be deleted)
- Different colored badges for roles
- "No users found" message when list is empty

### User Add/Edit Modal
Modal dialog for user information input:

**Add Mode**
- Username - Required
- Password - Required, minimum 16 characters
- Password (Confirm) - Required

**Edit Mode**
- Username - Required, can be changed
- Password - Optional, only enter when changing
- Password (Confirm) - Required only when changing password

**Validation**
- Password must be at least 16 characters
- Password and confirmation must match
- Focus indicators displayed on input fields

### User Delete Confirmation Modal
Confirmation dialog before deletion:
- Explicitly shows username with emphasis (1.5x size, wrapped in double quotes)
- Delete/Cancel buttons
- Result message display

## Accessibility Features

### Focus Trap (Implemented 2025-10-26)

Proper keyboard focus control within modal dialogs:

**Features**
- Navigate forward with TAB, backward with SHIFT+TAB
- Loop between first and last elements in modal
- Prevents focus from escaping outside the modal

**Implementation Details**
- Addresses issue where SHIFT+TAB is reported as `"Unidentified"` in Tauri applications
- Intercepts events in capture phase
- Implemented in `setupFocusTrap()` function in `res/js/modal-utils.js`

### Focus Indicators

Visual indication of focused elements:

**Input Fields**
- Large green ● mark on the left when focused
- Black outline for color-blind users
- Controlled by `.form-group.active` class

**Buttons (Unified 2025-10-26)**
- Inactive state: 2px black border
- Focus state: 2px white + 4px black double box-shadow
- Unified design across all buttons
- High contrast ratio for accessibility

**Dropdown Items**
- Filled ○ mark for active items
- Black outline for better visibility

### Keyboard Support
- Tab key for focus navigation
- Enter key for form submission
- Esc key to close modal
- Focus trap within modals (implemented)

## Core Features

### Display User List (`loadUsers()`)
```javascript
// Fetch and display user list from backend
const users = await invoke('list_users');
```
- i18n-enabled message display
- Role-based badge coloring
- Hide delete button for administrators

### Add User (`createUser()`)
```javascript
const userId = await invoke('create_general_user', {
    username: username,
    password: password
});
```
- Password validation
- Success/failure message display
- Reload list after completion

### Edit User (`updateUser()`)
```javascript
// Update general user
await invoke('update_general_user_info', updateParams);

// Update admin user
await invoke('update_admin_user_info', updateParams);
```
- Use appropriate function based on role
- Pass null for unchanged fields
- Password is optional (only when changing)

### Delete User (`handleDeleteConfirm()`)
```javascript
await invoke('delete_general_user_info', { userId: userId });
```
- Show confirmation dialog
- Feedback on deletion result
- Auto-close modal after 1.5 seconds and reload list

### Language Switch (`handleLanguageChange()`)
```javascript
await i18n.setLanguage(langCode);
await setupLanguageMenu();
await loadUsers();
```
- Rebuild menu and list after language change
- Display indicator for current language

## Internationalization (i18n)

### Supported Keys
```javascript
// Menu
menu.file, menu.back_to_main, menu.logout, menu.quit
menu.language

// User Management
user_mgmt.title, user_mgmt.user_list
user_mgmt.user_id, user_mgmt.username, user_mgmt.role
user_mgmt.created_at, user_mgmt.updated_at, user_mgmt.actions
user_mgmt.add_user, user_mgmt.edit_user, user_mgmt.edit, user_mgmt.delete
user_mgmt.password, user_mgmt.password_confirm
user_mgmt.save, user_mgmt.cancel
user_mgmt.role_admin, user_mgmt.role_user
user_mgmt.no_users
user_mgmt.loading, user_mgmt.creating, user_mgmt.updating, user_mgmt.deleting
user_mgmt.user_created, user_mgmt.user_updated, user_mgmt.user_deleted
user_mgmt.delete_user, user_mgmt.delete_confirmation

// Errors
error.password_mismatch, error.password_too_short
error.load_users_failed, error.save_user_failed, error.delete_user_failed
```

## Code Refactoring

### Separation of indicators.css
**Purpose**: Improve maintainability

**Changes**:
- Separated indicator-related styles from `menu.css`
- Created as independent file `res/css/indicators.css`
- Include in each screen with `<link rel="stylesheet" href="css/indicators.css" />`

**Benefits**:
1. Localized changes - only one file to modify when updating indicators
2. Improved readability - clear functionality from filename
3. Reusability - easy to include in new screens

### indicators.js Modularization
**Purpose**: Eliminate code duplication and improve maintainability

**Exported Functions**:

#### `wrapInputFields()`
```javascript
// Wrap inputs in .form-group with .input-wrapper
// Build layout for indicator display
```

#### `setupInputIndicators()`
```javascript
// Set focus/blur event handlers on input, textarea, select
// Add .active class to .form-group on focus
```

#### `setupButtonIndicators()`
```javascript
// Add .focus-indicator class to all buttons
// Enable underline display on focus
```

#### `setupIndicators()`
```javascript
// Convenience function to execute all three functions above
// Call in each screen's DOMContentLoaded
```

**Usage**:
```javascript
import { setupIndicators } from './indicators.js';

document.addEventListener('DOMContentLoaded', async function() {
    // ... other initialization
    setupIndicators();  // Setup indicators
    // ...
});
```

**Applied Screens**:
- `res/index.html` (Login/Setup screen) - used by `menu.js`
- `res/user-management.html` (User management screen) - used by `user-management.js`

**Benefits**:
1. DRY principle compliance - removed duplicate code
2. Efficient bug fixes - one fix applies to all screens
3. Consistent UX - same behavior across all screens
4. Testability - can test by module

## Data Flow

### User Add Flow
1. Click "Add User" button → `openAddUserModal()`
2. Form input → Focus indicator display
3. Click "Save" button → `handleUserFormSubmit()`
4. Validation → Password length, match check
5. Backend call → `create_general_user`
6. Show success message → Close modal
7. Reload user list → `loadUsers()`

### User Edit Flow
1. Click "Edit" button → `openEditUserModal(user)`
2. Populate form with existing data
3. Edit form → Password is optional
4. Click "Save" button → `handleUserFormSubmit()`
5. Role determination → `update_general_user_info` or `update_admin_user_info`
6. Include only changed fields in parameters
7. Show success message → Close modal
8. Reload user list

### User Delete Flow
1. Click "Delete" button → `openDeleteModal(user)`
2. Show confirmation dialog → Display username
3. Click "Delete" button → `handleDeleteConfirm()`
4. Backend call → `delete_general_user_info`
5. Show success message → Wait 1.5 seconds
6. Close modal → Reload user list

## Security Considerations

### Password Handling
- Frontend sends plain text
- Backend hashes with Argon2
- Minimum 16 characters required

### XSS Prevention
```javascript
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
```
- HTML escape when displaying usernames
- Use `escapeHtml()` when rendering table

### CSRF Prevention
- Use Tauri's secure IPC communication
- No regular HTTP requests used

## Future Improvements

### Implemented Features (Updated 2025-10-26)
1. ✅ User add feature
2. ✅ User edit feature
3. ✅ User delete feature
4. ✅ Admin edit feature
5. ✅ Focus trap (within modals)
6. ✅ Unified button focus styles
7. ✅ Delete confirmation modal improvements

### UI/UX Improvements
- Pagination (for many users)
- Sort functionality (click column headers)
- Search/filter functionality
- Bulk operations

### Accessibility
- Add ARIA labels
- Screen reader support messages
- Add keyboard shortcuts

### Error Handling
- More detailed error messages
- Retry on network errors
- Offline behavior

## Test Status

### Completed Tests
- ✅ User list display
- ✅ Language switching
- ✅ User add modal display
- ✅ Focus indicator display (input)
- ✅ Focus indicator display (button: primary)
- ✅ Focus indicator display (button: secondary)
- ✅ Dropdown menu indicators

### Pending Tests
- ⚠ User add execution
- ⚠ User edit execution
- ⚠ User delete execution
- ⚠ Admin edit execution
- ⚠ Validation error handling
- ⚠ Error case behavior

## Related Documentation
- [User Management Implementation](./USER_MANAGEMENT.md) - Backend implementation
- [Accessibility Indicators](./ACCESSIBILITY_INDICATORS.md) - Indicator specification
- [Dynamic Language Menu](./DYNAMIC_LANGUAGE_MENU.md) - Language switching
- [I18N Implementation](./I18N_IMPLEMENTATION.md) - i18n infrastructure

## Reference Information

### Technologies Used
- HTML5
- CSS3 (Flexbox)
- JavaScript (ES6 Modules)
- Tauri IPC
- i18next (internationalization)

### Coding Conventions
- Semicolons required
- Async/await for asynchronous processing
- Error handling required
- Function names in camelCase
- Comments in Japanese/English

### File Organization Principles
- CSS: Split by functionality
- JS: Modularize for reusability
- HTML: Semantic markup
