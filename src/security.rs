use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Debug)]
pub enum SecurityError {
    HashError(String),
    VerifyError(String),
    DerivationError(String),
    InvalidPassword(String),
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::HashError(msg) => write!(f, "Hash error: {}", msg),
            SecurityError::VerifyError(msg) => write!(f, "Verify error: {}", msg),
            SecurityError::DerivationError(msg) => write!(f, "Key derivation error: {}", msg),
            SecurityError::InvalidPassword(msg) => write!(f, "Invalid password: {}", msg),
        }
    }
}

impl std::error::Error for SecurityError {}

/// Hash a password using Argon2id
///
/// # Arguments
/// * `password` - The plaintext password to hash
///
/// # Returns
/// * `Ok(String)` - The Argon2 hash string (PHC format)
/// * `Err(SecurityError)` - If hashing fails
pub fn hash_password(password: &str) -> Result<String, SecurityError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| SecurityError::HashError(e.to_string()))?;
    
    Ok(password_hash.to_string())
}

/// Verify a password against an Argon2 hash
///
/// # Arguments
/// * `password` - The plaintext password to verify
/// * `hash` - The Argon2 hash string to verify against
///
/// # Returns
/// * `Ok(true)` - Password matches the hash
/// * `Ok(false)` - Password does not match
/// * `Err(SecurityError)` - If verification fails
pub fn verify_password(password: &str, hash: &str) -> Result<bool, SecurityError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| SecurityError::VerifyError(e.to_string()))?;
    
    let argon2 = Argon2::default();
    
    match argon2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Derive a 256-bit encryption key from a password and salt using Argon2
///
/// # Arguments
/// * `password` - The password to derive the key from
/// * `salt` - The salt (must be at least 8 bytes)
///
/// # Returns
/// * `Ok([u8; 32])` - The derived 256-bit key
/// * `Err(SecurityError)` - If derivation fails
pub fn derive_encryption_key(password: &str, salt: &[u8]) -> Result<[u8; 32], SecurityError> {
    if salt.len() < 8 {
        return Err(SecurityError::DerivationError(
            "Salt must be at least 8 bytes".to_string(),
        ));
    }
    
    let mut output_key = [0u8; 32];
    let argon2 = Argon2::default();
    
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut output_key)
        .map_err(|e| SecurityError::DerivationError(e.to_string()))?;
    
    Ok(output_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "testPassword123";
        let hash = hash_password(password).expect("Failed to hash password");
        
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_verify_password_success() {
        let password = "testPassword123";
        let hash = hash_password(password).expect("Failed to hash password");
        
        let result = verify_password(password, &hash).expect("Failed to verify password");
        assert!(result);
    }

    #[test]
    fn test_verify_password_failure() {
        let password = "testPassword123";
        let wrong_password = "wrongPassword";
        let hash = hash_password(password).expect("Failed to hash password");
        
        let result = verify_password(wrong_password, &hash).expect("Failed to verify password");
        assert!(!result);
    }

    #[test]
    fn test_hash_uniqueness() {
        let password = "testPassword123";
        let hash1 = hash_password(password).expect("Failed to hash password");
        let hash2 = hash_password(password).expect("Failed to hash password");
        
        // Different salts should produce different hashes
        assert_ne!(hash1, hash2);
        
        // But both should verify correctly
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }

    #[test]
    fn test_derive_encryption_key() {
        let password = "testPassword123";
        let salt = b"testsalt12345678";
        
        let key = derive_encryption_key(password, salt).expect("Failed to derive key");
        
        assert_eq!(key.len(), 32);
    }

    #[test]
    fn test_derive_encryption_key_deterministic() {
        let password = "testPassword123";
        let salt = b"testsalt12345678";
        
        let key1 = derive_encryption_key(password, salt).expect("Failed to derive key");
        let key2 = derive_encryption_key(password, salt).expect("Failed to derive key");
        
        // Same password and salt should produce same key
        assert_eq!(key1, key2);
    }

    #[test]
    fn test_derive_encryption_key_different_passwords() {
        let salt = b"testsalt12345678";
        
        let key1 = derive_encryption_key("password1", salt).expect("Failed to derive key");
        let key2 = derive_encryption_key("password2", salt).expect("Failed to derive key");
        
        // Different passwords should produce different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_derive_encryption_key_different_salts() {
        let password = "testPassword123";
        
        let key1 = derive_encryption_key(password, b"salt1_1234567890")
            .expect("Failed to derive key");
        let key2 = derive_encryption_key(password, b"salt2_1234567890")
            .expect("Failed to derive key");
        
        // Different salts should produce different keys
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_derive_encryption_key_short_salt() {
        let password = "testPassword123";
        let short_salt = b"short";
        
        let result = derive_encryption_key(password, short_salt);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_password_hash() {
        let hash = hash_password("").expect("Failed to hash empty password");
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_long_password() {
        let long_password = "a".repeat(1000);
        let hash = hash_password(&long_password).expect("Failed to hash long password");
        
        assert!(verify_password(&long_password, &hash).unwrap());
    }

    #[test]
    fn test_unicode_password() {
        let password = "パスワード123";
        let hash = hash_password(password).expect("Failed to hash unicode password");
        
        assert!(verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_special_characters_password() {
        let password = "P@ssw0rd!#$%^&*()";
        let hash = hash_password(password).expect("Failed to hash password with special chars");
        
        assert!(verify_password(password, &hash).unwrap());
    }
}
