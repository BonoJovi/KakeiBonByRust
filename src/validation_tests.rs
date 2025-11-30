/// Reusable password validation test suite
/// 
/// This module provides a common set of password validation tests that can be
/// reused across different contexts (admin setup, user addition, password change, etc.).
/// Similar to the JavaScript password-validation-tests.js module.

#[cfg(test)]
pub mod password_tests {
    use crate::validation::{validate_password, validate_password_confirmation, validate_password_with_confirmation};

    /// Test suite for empty password validation
    pub fn test_empty_passwords() {
        // Completely empty
        assert!(validate_password("").is_err());
        assert_eq!(
            validate_password("").unwrap_err(),
            "Password cannot be empty!"
        );
    }

    /// Test suite for whitespace-only passwords
    pub fn test_whitespace_only_passwords() {
        // Spaces only
        assert!(validate_password("   ").is_err());
        assert_eq!(
            validate_password("   ").unwrap_err(),
            "Password cannot be empty!"
        );
        
        // Tabs only
        assert!(validate_password("\t\t").is_err());
        assert_eq!(
            validate_password("\t\t").unwrap_err(),
            "Password cannot be empty!"
        );
        
        // Mixed whitespace
        assert!(validate_password(" \t \n ").is_err());
        assert_eq!(
            validate_password(" \t \n ").unwrap_err(),
            "Password cannot be empty!"
        );
    }

    /// Test suite for short passwords
    pub fn test_short_passwords() {
        // Generic short password
        assert!(validate_password("short").is_err());
        assert_eq!(
            validate_password("short").unwrap_err(),
            "Password must be at least 16 characters long!"
        );
        
        // Single character
        assert!(validate_password("a").is_err());
        assert_eq!(
            validate_password("a").unwrap_err(),
            "Password must be at least 16 characters long!"
        );
    }

    /// Test suite for password length boundaries
    pub fn test_password_length_boundaries() {
        // Exactly 15 characters (should fail)
        let password = "123456789012345";
        assert_eq!(password.len(), 15);
        assert!(validate_password(password).is_err());
        assert_eq!(
            validate_password(password).unwrap_err(),
            "Password must be at least 16 characters long!"
        );
        
        // Exactly 16 characters (should pass)
        let password = "1234567890123456";
        assert_eq!(password.len(), 16);
        assert!(validate_password(password).is_ok());
        
        // More than 16 characters (should pass)
        let password = "thisIsAVerySecurePassword";
        assert!(password.len() > 16);
        assert!(validate_password(password).is_ok());
    }

    /// Test suite for valid passwords with various characters
    pub fn test_valid_password_variations() {
        // Password with spaces
        let password = "my secure password 16";
        assert!(password.len() >= 16);
        assert!(validate_password(password).is_ok());
        
        // Password with special characters
        let password = "p@ssw0rd!#$12345";
        assert!(password.len() >= 16);
        assert!(validate_password(password).is_ok());
        
        // Password with unicode
        let password = "パスワード1234567890";
        assert!(password.len() >= 16);
        assert!(validate_password(password).is_ok());
        
        // Very long password
        let password = "a".repeat(1000);
        assert!(validate_password(&password).is_ok());
        
        // Numeric password
        let password = "1234567890123456";
        assert!(validate_password(password).is_ok());
    }

    /// Test suite for password confirmation
    pub fn test_password_confirmation_logic() {
        // Matching passwords
        let password = "1234567890123456";
        assert!(validate_password_confirmation(password, password).is_ok());
        
        // Non-matching passwords
        let password1 = "1234567890123456";
        let password2 = "6543210987654321";
        assert!(validate_password_confirmation(password1, password2).is_err());
        assert_eq!(
            validate_password_confirmation(password1, password2).unwrap_err(),
            "Passwords do not match!"
        );
        
        // Case sensitivity
        let password1 = "Password12345678";
        let password2 = "password12345678";
        assert!(validate_password_confirmation(password1, password2).is_err());
    }

    /// Test suite for full validation with confirmation
    pub fn test_full_validation() {
        // Valid passwords
        let password = "securePassword123";
        assert!(validate_password_with_confirmation(password, password).is_ok());
        
        // Empty password
        let result = validate_password_with_confirmation("", "");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Password cannot be empty!");
        
        // Short password
        let password = "short";
        let result = validate_password_with_confirmation(password, password);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Password must be at least 16 characters long!"
        );
        
        // Non-matching passwords
        let password1 = "validPassword1234567";
        let password2 = "differentPassword123";
        let result = validate_password_with_confirmation(password1, password2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Passwords do not match!");
    }

    /// Test suite for error priority
    pub fn test_validation_error_priority() {
        // Empty password should be caught first
        let result = validate_password_with_confirmation("", "different");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Password cannot be empty!");
        
        // Short password should be caught before mismatch
        let result = validate_password_with_confirmation("short", "different");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Password must be at least 16 characters long!"
        );
    }

    /// Test suite for passwords with spaces
    pub fn test_passwords_with_spaces() {
        // Password with leading/trailing spaces
        let password = " passwordpassword ";
        assert!(password.len() >= 16);
        assert!(validate_password(password).is_ok());
        
        // Matching passwords with same spaces
        assert!(validate_password_confirmation(password, password).is_ok());
        
        // Non-matching due to different spacing
        let password2 = "passwordpassword";
        assert!(validate_password_confirmation(password, password2).is_err());
    }

    /// Test suite for boundary cases
    pub fn test_boundary_cases() {
        // Boundary - 1 (15 characters)
        assert!(validate_password(&"a".repeat(15)).is_err());
        
        // Boundary (16 characters)
        assert!(validate_password(&"a".repeat(16)).is_ok());
        
        // Boundary + 1 (17 characters)
        assert!(validate_password(&"a".repeat(17)).is_ok());
    }

    /// Run all password validation tests
    /// 
    /// This function runs all password tests in sequence, similar to
    /// runAllPasswordTests() in the JavaScript test suite
    pub fn run_all_tests() {
        test_empty_passwords();
        test_whitespace_only_passwords();
        test_short_passwords();
        test_password_length_boundaries();
        test_valid_password_variations();
        test_password_confirmation_logic();
        test_full_validation();
        test_validation_error_priority();
        test_passwords_with_spaces();
        test_boundary_cases();
    }
}

#[cfg(test)]
pub mod username_tests {
    // Placeholder for future username validation tests
    // Can be expanded when username validation rules are added
}
