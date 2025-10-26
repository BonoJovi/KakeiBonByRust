/// Password validation module
/// 
/// Provides validation logic for user passwords to ensure security requirements are met.

/// Validates a password according to security requirements
/// 
/// # Arguments
/// * `password` - The password string to validate
/// 
/// # Returns
/// * `Ok(())` if the password is valid
/// * `Err(String)` with an error message if validation fails
pub fn validate_password(password: &str) -> Result<(), String> {
    // Check if password is empty or only whitespace
    if password.trim().is_empty() {
        return Err("Password cannot be empty!".to_string());
    }
    
    // Check minimum length (16 characters)
    if password.len() < 16 {
        return Err("Password must be at least 16 characters long!".to_string());
    }
    
    Ok(())
}

/// Validates password confirmation
/// 
/// # Arguments
/// * `password` - The original password
/// * `password_confirm` - The confirmation password
/// 
/// # Returns
/// * `Ok(())` if passwords match
/// * `Err(String)` with an error message if they don't match
pub fn validate_password_confirmation(password: &str, password_confirm: &str) -> Result<(), String> {
    if password != password_confirm {
        return Err("Passwords do not match!".to_string());
    }
    Ok(())
}

/// Validates both password and confirmation together
/// 
/// # Arguments
/// * `password` - The password to validate
/// * `password_confirm` - The confirmation password
/// 
/// # Returns
/// * `Ok(())` if all validations pass
/// * `Err(String)` with the first error encountered
#[allow(dead_code)]
pub fn validate_password_with_confirmation(password: &str, password_confirm: &str) -> Result<(), String> {
    validate_password(password)?;
    validate_password_confirmation(password, password_confirm)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation_tests::password_tests;

    // Use the common password test suite
    #[test]
    fn test_all_password_validations() {
        password_tests::run_all_tests();
    }

    // Keep individual tests for backwards compatibility and granular failure reporting
    #[test]
    fn test_empty_password() {
        password_tests::test_empty_passwords();
    }

    #[test]
    fn test_whitespace_only_password() {
        password_tests::test_whitespace_only_passwords();
    }

    #[test]
    fn test_password_too_short() {
        password_tests::test_short_passwords();
    }

    #[test]
    fn test_single_character_password() {
        assert!(validate_password("a").is_err());
        assert_eq!(
            validate_password("a").unwrap_err(),
            "Password must be at least 16 characters long!"
        );
    }

    #[test]
    fn test_password_exactly_15_characters() {
        let password = "123456789012345"; // 15 characters
        assert_eq!(password.len(), 15);
        assert!(validate_password(password).is_err());
        assert_eq!(
            validate_password(password).unwrap_err(),
            "Password must be at least 16 characters long!"
        );
    }

    #[test]
    fn test_password_exactly_16_characters() {
        let password = "1234567890123456"; // 16 characters
        assert_eq!(password.len(), 16);
        assert!(validate_password(password).is_ok());
    }

    #[test]
    fn test_password_more_than_16_characters() {
        let password = "thisIsAVerySecurePassword";
        assert!(password.len() > 16);
        assert!(validate_password(password).is_ok());
    }

    #[test]
    fn test_password_with_spaces() {
        let password = "my secure password 16";
        assert!(password.len() >= 16);
        assert!(validate_password(password).is_ok());
    }

    #[test]
    fn test_password_with_special_characters() {
        let password = "p@ssw0rd!#$12345";
        assert!(password.len() >= 16);
        assert!(validate_password(password).is_ok());
    }

    #[test]
    fn test_password_with_unicode() {
        let password = "パスワード1234567890";
        assert!(password.len() >= 16);
        assert!(validate_password(password).is_ok());
    }

    #[test]
    fn test_very_long_password() {
        let password = "a".repeat(1000);
        assert!(validate_password(&password).is_ok());
    }

    #[test]
    fn test_password_confirmation_matching() {
        password_tests::test_password_confirmation_logic();
    }

    #[test]
    fn test_password_confirmation_not_matching() {
        let password1 = "1234567890123456";
        let password2 = "6543210987654321";
        assert!(validate_password_confirmation(password1, password2).is_err());
        assert_eq!(
            validate_password_confirmation(password1, password2).unwrap_err(),
            "Passwords do not match!"
        );
    }

    #[test]
    fn test_password_confirmation_case_sensitive() {
        let password1 = "Password12345678";
        let password2 = "password12345678";
        assert!(validate_password_confirmation(password1, password2).is_err());
    }

    #[test]
    fn test_full_validation_with_valid_passwords() {
        let password = "securePassword123";
        assert!(validate_password_with_confirmation(password, password).is_ok());
    }

    #[test]
    fn test_full_validation_with_empty_password() {
        let result = validate_password_with_confirmation("", "");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Password cannot be empty!");
    }

    #[test]
    fn test_full_validation_with_short_password() {
        let password = "short";
        let result = validate_password_with_confirmation(password, password);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Password must be at least 16 characters long!"
        );
    }

    #[test]
    fn test_full_validation_with_non_matching_passwords() {
        let password1 = "validPassword1234567";
        let password2 = "differentPassword123";
        let result = validate_password_with_confirmation(password1, password2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Passwords do not match!");
    }

    #[test]
    fn test_full_validation_error_priority() {
        password_tests::test_validation_error_priority();
    }

    #[test]
    fn test_password_with_leading_trailing_spaces() {
        password_tests::test_passwords_with_spaces();
    }

    #[test]
    fn test_numeric_password() {
        let password = "1234567890123456";
        assert!(validate_password(password).is_ok());
    }

    #[test]
    fn test_password_boundary_cases() {
        password_tests::test_boundary_cases();
    }
}
