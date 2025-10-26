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
