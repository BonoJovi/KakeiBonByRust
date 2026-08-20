//! Validation module
//!
//! Provides validation logic for passwords and for bounded text fields
//! (names, item names, memos) shared by every service.

use crate::consts;

/// Rejects text longer than `max_len` characters. Counts characters, not
/// bytes, so multibyte input (Japanese) is not implicitly clipped.
///
/// # Arguments
/// * `label` - Field name used in the error message (e.g. `"Shop name"`)
pub fn validate_max_chars(label: &str, value: &str, max_len: usize) -> Result<(), String> {
    if value.chars().count() > max_len {
        return Err(format!("{} must be {} characters or less", label, max_len));
    }
    Ok(())
}

/// Same as [`validate_max_chars`] but for optional fields; `None` is valid.
pub fn validate_optional_max_chars(
    label: &str,
    value: Option<&String>,
    max_len: usize,
) -> Result<(), String> {
    match value {
        Some(value) => validate_max_chars(label, value, max_len),
        None => Ok(()),
    }
}

/// Rejects an empty (or whitespace-only) required text field.
pub fn validate_not_empty(label: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{} cannot be empty", label));
    }
    Ok(())
}

/// Master-data name guard: required and at most [`consts::MAX_NAME_LEN`] characters.
pub fn validate_master_name(label: &str, name: &str) -> Result<(), String> {
    validate_not_empty(label, name)?;
    validate_max_chars(label, name, consts::MAX_NAME_LEN)
}

/// Memo guard shared by master data, transactions and recurring rules.
pub fn validate_memo(label: &str, memo: Option<&String>) -> Result<(), String> {
    validate_optional_max_chars(label, memo, consts::MAX_MEMO_LEN)
}

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

    // Count characters, not UTF-8 bytes. Using `password.len()` here would
    // let a 6-char Japanese password (18 bytes) satisfy the "16 characters"
    // rule while the frontend (which counts UTF-16 code units and matches
    // char count for BMP text) rejects the same input — a policy split the
    // user could stumble across only after a failed login. See
    // `consts::MIN_PASSWORD_LENGTH`.
    if password.chars().count() < consts::MIN_PASSWORD_LENGTH {
        return Err(format!(
            "Password must be at least {} characters long!",
            consts::MIN_PASSWORD_LENGTH
        ));
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

pub fn validate_period_start_day(day: i64) -> Result<u32, String> {
    if !(1..=31).contains(&day) {
        return Err(format!(
            "Period start day must be between 1 and 31 (got {})",
            day
        ));
    }
    Ok(day as u32)
}

pub fn validate_period_start_month(month: i64) -> Result<u32, String> {
    if !(1..=12).contains(&month) {
        return Err(format!(
            "Period start month must be between 1 and 12 (got {})",
            month
        ));
    }
    Ok(month as u32)
}

/// v2.4.0: 月次起算日の休日シフト設定 (0=None, 1=Prev, 2=Next) を検証する。
pub fn validate_month_period_holiday_shift(value: i64) -> Result<i32, String> {
    if !(0..=2).contains(&value) {
        return Err(format!(
            "Month period holiday shift must be 0 (None), 1 (Prev), or 2 (Next) (got {})",
            value
        ));
    }
    Ok(value as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation_tests::password_tests;

    // Use the common password test suite
    #[test]
    fn test_all_password_validations() {
        // Execute common test suite and verify it doesn't panic
        let result = std::panic::catch_unwind(|| {
            password_tests::run_all_tests();
        });
        assert!(result.is_ok(), "Common password test suite should not panic");
    }

    // Keep individual tests for backwards compatibility and granular failure reporting
    #[test]
    fn test_empty_password() {
        // Execute common test and verify it doesn't panic
        let result = std::panic::catch_unwind(|| {
            password_tests::test_empty_passwords();
        });
        assert!(result.is_ok(), "Empty password test should pass without panic");
    }

    #[test]
    fn test_whitespace_only_password() {
        // Execute common test and verify it doesn't panic
        let result = std::panic::catch_unwind(|| {
            password_tests::test_whitespace_only_passwords();
        });
        assert!(result.is_ok(), "Whitespace-only password test should pass without panic");
    }

    #[test]
    fn test_password_too_short() {
        // Execute common test and verify it doesn't panic
        let result = std::panic::catch_unwind(|| {
            password_tests::test_short_passwords();
        });
        assert!(result.is_ok(), "Short password test should pass without panic");
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
        // 16 chars of BMP Japanese input. `.chars().count()` == 16 here,
        // even though `.len()` (UTF-8 bytes) is much larger.
        let password = "パスワード12345678901";
        assert_eq!(password.chars().count(), 16);
        assert!(validate_password(password).is_ok());
    }

    /// Fable-5 review #9 — regression guard against `password.len()` (byte
    /// count) sneaking back in. A 15-char Japanese password is 15 chars
    /// but ~25 bytes; before the fix it satisfied `len() >= 16` and slipped
    /// through the backend gate while the frontend correctly rejected it.
    #[test]
    fn test_multibyte_password_below_min_length_rejected() {
        let password = "パスワード1234567890"; // 15 chars, ~25 UTF-8 bytes
        assert_eq!(password.chars().count(), 15);
        assert!(
            password.len() >= consts::MIN_PASSWORD_LENGTH,
            "test premise: byte count should be >= min length so a bytes-based check would incorrectly accept"
        );
        assert!(validate_password(password).is_err());
    }

    /// The boundary in Unicode scalar values: a 16-char JA password must
    /// pass (the mirror of the above test at the acceptance boundary).
    #[test]
    fn test_multibyte_password_at_min_length_accepted() {
        let password = "あ".repeat(consts::MIN_PASSWORD_LENGTH);
        assert_eq!(password.chars().count(), consts::MIN_PASSWORD_LENGTH);
        assert!(validate_password(&password).is_ok());
    }

    #[test]
    fn test_very_long_password() {
        let password = "a".repeat(1000);
        assert!(validate_password(&password).is_ok());
    }

    #[test]
    fn test_password_confirmation_matching() {
        // Execute common test and verify it doesn't panic
        let result = std::panic::catch_unwind(|| {
            password_tests::test_password_confirmation_logic();
        });
        assert!(result.is_ok(), "Password confirmation test should pass without panic");
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
        // Execute common test and verify it doesn't panic
        let result = std::panic::catch_unwind(|| {
            password_tests::test_validation_error_priority();
        });
        assert!(result.is_ok(), "Validation error priority test should pass without panic");
    }

    #[test]
    fn test_password_with_leading_trailing_spaces() {
        // Execute common test and verify it doesn't panic
        let result = std::panic::catch_unwind(|| {
            password_tests::test_passwords_with_spaces();
        });
        assert!(result.is_ok(), "Password with spaces test should pass without panic");
    }

    #[test]
    fn test_numeric_password() {
        let password = "1234567890123456";
        assert!(validate_password(password).is_ok());
    }

    #[test]
    fn test_password_boundary_cases() {
        // Execute common test and verify it doesn't panic
        let result = std::panic::catch_unwind(|| {
            password_tests::test_boundary_cases();
        });
        assert!(result.is_ok(), "Boundary cases test should pass without panic");
    }

    #[test]
    fn max_chars_counts_characters_not_bytes() {
        // Japanese is 3 bytes per char in UTF-8.
        let name = "あ".repeat(consts::MAX_NAME_LEN);
        assert!(validate_max_chars("Shop name", &name, consts::MAX_NAME_LEN).is_ok());

        let too_long = "あ".repeat(consts::MAX_NAME_LEN + 1);
        assert_eq!(
            validate_max_chars("Shop name", &too_long, consts::MAX_NAME_LEN).unwrap_err(),
            format!("Shop name must be {} characters or less", consts::MAX_NAME_LEN)
        );
    }

    #[test]
    fn optional_max_chars_accepts_none() {
        assert!(validate_optional_max_chars("Memo", None, consts::MAX_MEMO_LEN).is_ok());
        assert!(validate_memo("Memo", None).is_ok());
        assert!(validate_memo("Memo", Some(&"メ".repeat(consts::MAX_MEMO_LEN))).is_ok());
        assert!(validate_memo("Memo", Some(&"メ".repeat(consts::MAX_MEMO_LEN + 1))).is_err());
    }

    #[test]
    fn master_name_rejects_empty_and_whitespace_only() {
        assert_eq!(
            validate_master_name("Shop name", "   ").unwrap_err(),
            "Shop name cannot be empty"
        );
        assert!(validate_master_name("Shop name", "イオン新宿店").is_ok());
    }

    #[test]
    fn period_start_day_accepts_valid_range() {
        assert_eq!(validate_period_start_day(1).unwrap(), 1);
        assert_eq!(validate_period_start_day(15).unwrap(), 15);
        assert_eq!(validate_period_start_day(31).unwrap(), 31);
    }

    #[test]
    fn period_start_day_rejects_out_of_range() {
        assert!(validate_period_start_day(0).is_err());
        assert!(validate_period_start_day(32).is_err());
        assert!(validate_period_start_day(-1).is_err());
        assert!(validate_period_start_day(100).is_err());
    }

    #[test]
    fn period_start_month_accepts_valid_range() {
        assert_eq!(validate_period_start_month(1).unwrap(), 1);
        assert_eq!(validate_period_start_month(6).unwrap(), 6);
        assert_eq!(validate_period_start_month(12).unwrap(), 12);
    }

    #[test]
    fn period_start_month_rejects_out_of_range() {
        assert!(validate_period_start_month(0).is_err());
        assert!(validate_period_start_month(13).is_err());
        assert!(validate_period_start_month(-1).is_err());
        assert!(validate_period_start_month(100).is_err());
    }

    #[test]
    fn month_period_holiday_shift_accepts_valid_values() {
        assert_eq!(validate_month_period_holiday_shift(0).unwrap(), 0);
        assert_eq!(validate_month_period_holiday_shift(1).unwrap(), 1);
        assert_eq!(validate_month_period_holiday_shift(2).unwrap(), 2);
    }

    #[test]
    fn month_period_holiday_shift_rejects_out_of_range() {
        assert!(validate_month_period_holiday_shift(-1).is_err());
        assert!(validate_month_period_holiday_shift(3).is_err());
        assert!(validate_month_period_holiday_shift(100).is_err());
    }
}
