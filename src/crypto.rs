use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;

#[derive(Debug)]
pub enum CryptoError {
    EncryptionError(String),
    DecryptionError(String),
    EncodingError(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            CryptoError::DecryptionError(msg) => write!(f, "Decryption error: {}", msg),
            CryptoError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}

/// AES-256-GCM encryption/decryption handler
pub struct Crypto {
    cipher: Aes256Gcm,
}

impl Crypto {
    /// Create a new Crypto instance with a 256-bit key
    ///
    /// # Arguments
    /// * `key` - A 32-byte (256-bit) encryption key
    pub fn new(key: [u8; 32]) -> Self {
        let cipher = Aes256Gcm::new(&key.into());
        Self { cipher }
    }

    /// Encrypt plaintext and return Base64-encoded ciphertext with nonce
    ///
    /// Format: Base64(nonce || ciphertext)
    ///
    /// # Arguments
    /// * `plaintext` - The text to encrypt
    ///
    /// # Returns
    /// * `Ok(String)` - Base64-encoded encrypted data
    /// * `Err(CryptoError)` - If encryption fails
    pub fn encrypt(&self, plaintext: &str) -> Result<String, CryptoError> {
        // Generate random 96-bit nonce
        let mut rng = rand::thread_rng();
        let nonce_bytes: [u8; 12] = rng.gen();
        let nonce = nonce_bytes.into();

        // Encrypt
        let ciphertext = self
            .cipher
            .encrypt(&nonce, plaintext.as_bytes())
            .map_err(|e| CryptoError::EncryptionError(e.to_string()))?;

        // Combine nonce + ciphertext
        let mut combined = Vec::with_capacity(12 + ciphertext.len());
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);

        // Encode to Base64
        Ok(general_purpose::STANDARD.encode(&combined))
    }

    /// Decrypt Base64-encoded ciphertext
    ///
    /// # Arguments
    /// * `encoded` - Base64-encoded encrypted data (nonce || ciphertext)
    ///
    /// # Returns
    /// * `Ok(String)` - Decrypted plaintext
    /// * `Err(CryptoError)` - If decryption fails
    pub fn decrypt(&self, encoded: &str) -> Result<String, CryptoError> {
        // Decode from Base64
        let combined = general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| CryptoError::EncodingError(e.to_string()))?;

        if combined.len() < 12 {
            return Err(CryptoError::DecryptionError(
                "Invalid ciphertext: too short".to_string(),
            ));
        }

        // Split nonce and ciphertext
        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce: [u8; 12] = nonce_bytes.try_into()
            .map_err(|_| CryptoError::DecryptionError("Invalid nonce size".to_string()))?;
        let nonce = nonce.into();

        // Decrypt
        let plaintext_bytes = self
            .cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|e| CryptoError::DecryptionError(e.to_string()))?;

        // Convert to UTF-8 string
        String::from_utf8(plaintext_bytes)
            .map_err(|e| CryptoError::DecryptionError(format!("Invalid UTF-8: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_key() -> [u8; 32] {
        [42u8; 32] // Test key
    }

    #[test]
    fn test_encrypt_decrypt_basic() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "Hello, World!";

        let encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_encrypt_produces_different_outputs() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "test message";

        let encrypted1 = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let encrypted2 = crypto.encrypt(plaintext).expect("Failed to encrypt");

        // Different nonces should produce different ciphertexts
        assert_ne!(encrypted1, encrypted2);

        // But both should decrypt to the same plaintext
        assert_eq!(crypto.decrypt(&encrypted1).unwrap(), plaintext);
        assert_eq!(crypto.decrypt(&encrypted2).unwrap(), plaintext);
    }

    #[test]
    fn test_empty_string() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "";

        let encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_long_string() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "a".repeat(10000);

        let encrypted = crypto.encrypt(&plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_unicode_text() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "日本語テスト 🎉";

        let encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_special_characters() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

        let encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_newlines_and_whitespace() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "Line 1\nLine 2\r\nLine 3\tTabbed";

        let encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_different_keys_produce_different_results() {
        let key1 = [1u8; 32];
        let key2 = [2u8; 32];
        let crypto1 = Crypto::new(key1);
        let crypto2 = Crypto::new(key2);
        let plaintext = "test message";

        let encrypted1 = crypto1.encrypt(plaintext).expect("Failed to encrypt");
        let encrypted2 = crypto2.encrypt(plaintext).expect("Failed to encrypt");

        assert_ne!(encrypted1, encrypted2);
    }

    #[test]
    fn test_wrong_key_fails_decryption() {
        let key1 = [1u8; 32];
        let key2 = [2u8; 32];
        let crypto1 = Crypto::new(key1);
        let crypto2 = Crypto::new(key2);
        let plaintext = "test message";

        let encrypted = crypto1.encrypt(plaintext).expect("Failed to encrypt");
        let result = crypto2.decrypt(&encrypted);

        assert!(result.is_err());
    }

    #[test]
    fn test_corrupted_ciphertext() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "test message";

        let mut encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        
        // Corrupt the ciphertext
        if let Some(last_char) = encrypted.pop() {
            encrypted.push(if last_char == 'A' { 'B' } else { 'A' });
        }

        let result = crypto.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_base64() {
        let crypto = Crypto::new(get_test_key());
        let invalid = "not valid base64!!!";

        let result = crypto.decrypt(invalid);
        assert!(result.is_err());
    }

    #[test]
    fn test_too_short_ciphertext() {
        let crypto = Crypto::new(get_test_key());
        // Base64 encoded data that's too short (less than 12 bytes when decoded)
        let short = general_purpose::STANDARD.encode(b"short");

        let result = crypto.decrypt(&short);
        assert!(result.is_err());
    }

    #[test]
    fn test_numeric_strings() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "1234567890";

        let encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_json_like_string() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = r#"{"name":"John","age":30}"#;

        let encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_sql_like_string() {
        let crypto = Crypto::new(get_test_key());
        let plaintext = "SELECT * FROM users WHERE id = 1";

        let encrypted = crypto.encrypt(plaintext).expect("Failed to encrypt");
        let decrypted = crypto.decrypt(&encrypted).expect("Failed to decrypt");

        assert_eq!(plaintext, decrypted);
    }
}
