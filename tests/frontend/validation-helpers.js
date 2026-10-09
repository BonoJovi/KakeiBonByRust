/**
 * Common Validation Helper Functions
 * 
 * These functions are shared across all validation tests
 */

/**
 * Validate password with confirmation
 * Used in admin-setup and user addition (without username validation)
 */
export function validatePassword(password, passwordConfirm) {
    if (!password || password.trim() === '') {
        return {
            valid: false,
            message: 'Password cannot be empty!'
        };
    }
    
    if (password.length < 16) {
        return {
            valid: false,
            message: 'Password must be at least 16 characters long!'
        };
    }
    
    if (password !== passwordConfirm) {
        return {
            valid: false,
            message: 'Passwords do not match!'
        };
    }
    
    return {
        valid: true,
        message: ''
    };
}

/**
 * Validate user addition (username + password)
 * Used in user-management forms
 */
export function validateUserAddition(username, password, passwordConfirm) {
    // Username validation
    if (!username || username.trim() === '') {
        return {
            valid: false,
            message: 'Username cannot be empty!'
        };
    }
    
    // Reuse password validation
    return validatePassword(password, passwordConfirm);
}

/**
 * Validate user edit (username + optional password)
 * Used in both admin and general user edit forms
 * 
 * @param {string} username - The username to validate
 * @param {string} password - The password (optional in edit mode)
 * @param {string} passwordConfirm - The password confirmation
 * @param {boolean} isEditMode - True if editing existing user, false if adding new user
 */
export function validateUserEdit(username, password, passwordConfirm, isEditMode = false) {
    // Username validation (always required)
    if (!username || username.trim() === '') {
        return {
            valid: false,
            message: 'Username cannot be empty!'
        };
    }
    
    // Password validation
    // In edit mode, password is optional (empty means no change)
    // But if provided, it must be valid
    if (isEditMode) {
        // If both password fields are empty, it's valid (no password change)
        if ((!password || password.trim() === '') && (!passwordConfirm || passwordConfirm.trim() === '')) {
            return {
                valid: true,
                message: ''
            };
        }
        
        // If password is provided, validate it
        if (password || passwordConfirm) {
            return validatePassword(password, passwordConfirm);
        }
    } else {
        // In add mode, password is required
        return validatePassword(password, passwordConfirm);
    }
    
    return {
        valid: true,
        message: ''
    };
}
