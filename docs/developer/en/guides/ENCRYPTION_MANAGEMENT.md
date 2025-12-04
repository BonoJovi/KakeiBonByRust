# Encryption Management System

## Overview
System for managing encrypted fields in the database and re-encrypting data when user passwords change.

## Main Features

### 1. Encrypted Field Registration
Manages which table columns are encrypted in a centralized manner.

**ENCRYPTED_FIELDS Table**:
```sql
CREATE TABLE ENCRYPTED_FIELDS (
    FIELD_ID INTEGER NOT NULL,
    TABLE_NAME VARCHAR(128) NOT NULL,
    COLUMN_NAME VARCHAR(128) NOT NULL,
    DESCRIPTION VARCHAR(256),
    IS_ACTIVE INTEGER NOT NULL DEFAULT 1,
    ENTRY_DT DATETIME NOT NULL,
    PRIMARY KEY(FIELD_ID),
    UNIQUE(TABLE_NAME, COLUMN_NAME)
);
```

### 2. Encrypted Field List Retrieval (`list_encrypted_fields`)
Retrieves list of all encrypted fields.

**Return Value**:
- Field ID
- Table name
- Column name
- Description
- Active status
- Registration date

### 3. Encrypted Field Registration (`register_encrypted_field`)
Registers new encrypted fields.

**Parameters**:
- `table_name`: Table name
- `column_name`: Column name
- `description`: Description (optional)

### 4. Re-encryption Processing
Automatically re-encrypts all encrypted fields when user password changes.

**Processing Flow**:
1. Retrieve encrypted field list
2. Decrypt with old key
3. Encrypt with new key
4. Update database

**Target Tables**:
- All tables registered in ENCRYPTED_FIELDS table

## Encryption Specifications

### Encryption Algorithm
- **Algorithm**: AES-256-GCM
- **Key Derivation**: From user password (Argon2id output)
- **IV (Initialization Vector)**: Randomly generated for each encryption

### Key Management
- **General Users**: User-specific encryption key
- **Administrator**: Master key (can decrypt all users' data)

## Security Features

### 1. Automatic Re-encryption
- Automatically re-encrypts when password changes
- Transaction processing
- Rollback on error

### 2. Centralized Management
- Manages encrypted fields in single table
- Easy to add new encrypted fields
- Flexible for schema changes

### 3. Field-level Encryption
- Only specific fields encrypted
- Performance optimization

## Usage Example

### Backend (Rust)
```rust
use crate::services::encryption::EncryptionService;

let encryption = EncryptionService::new(pool);

// Register encrypted field
encryption.register_encrypted_field(
    "TRANSACTIONS",
    "MEMO",
    Some("Transaction memo field")
).await?;

// Get encrypted field list
let fields = encryption.list_encrypted_fields().await?;

// Re-encrypt all data (automatically called when password changes)
encryption.reencrypt_all_user_data(
    user_id,
    &old_key,
    &new_key
).await?;
```

## Testing

### Implemented Tests
1. Encrypted field registration
2. Encrypted field list retrieval
3. Duplicate registration prevention
4. Re-encryption processing

### Running Tests
```bash
cargo test services::encryption::tests --lib
```

## Implementation Status
- ✅ Encrypted field management table
- ✅ Encrypted field registration
- ✅ Encrypted field list retrieval
- ✅ Re-encryption system
- ✅ Password change integration
- ✅ All tests passing

## Future Enhancements
- Encryption key rotation
- Encrypted field search
- Partial re-encryption (specified tables only)
- Encryption performance monitoring
