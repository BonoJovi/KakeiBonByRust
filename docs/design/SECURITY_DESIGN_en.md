# Security Design Specification

**Last Updated**: 2024-12-05 03:56 JST  
**Target Version**: In Development

---

## Table of Contents

1. [Overview](#overview)
2. [Password Management](#password-management)
3. [Data Encryption](#data-encryption)
4. [Authentication & Authorization](#authentication--authorization)
5. [Security Boundaries](#security-boundaries)
6. [Threat Model](#threat-model)

---

## Overview

KakeiБon is designed as a desktop application handling personal financial data, based on the following security principles:

### Security Principles

1. **Data Protection**: Store sensitive data encrypted
2. **Least Privilege**: Role-based access control
3. **Defense in Depth**: Multi-layered security measures
4. **Transparency**: Auditable security implementation

### Security Scope

**Protected**:
- User passwords
- Memo fields (managed by ENCRYPTED_FIELDS)
- Session information

**Out of Scope**:
- Network communication (desktop app)
- External API integration (not yet implemented)

---

## Password Management

### Hash Algorithm: Argon2id

**Selection Rationale**:
- Latest OWASP-recommended password hashing algorithm
- Resistant to GPU/ASIC attacks
- Resistant to side-channel attacks
- Memory-hard function preventing parallel attacks

**Implementation**: `src/security.rs`

**Parameters**:
```rust
m_cost: 19456,      // Memory usage: 19MB
t_cost: 2,          // Iterations: 2
p_cost: 1,          // Parallelism: 1 thread
output_len: 32      // Hash output: 32 bytes
```

**Parameter Rationale**:
- **m_cost (19456 KiB ≈ 19MB)**: Balance between security and UX
  - Sufficient for attack prevention
  - Acceptable latency on consumer hardware (~100-200ms)
- **t_cost (2)**: OWASP minimum recommendation
- **p_cost (1)**: Single-threaded to avoid resource competition

### Password Policy

**Minimum Length**: 16 characters

**Rationale**:
- Entropy > 80 bits (sufficient against brute force)
- No complexity requirements (length is more important)
- Encourages passphrases over complex passwords

**Validation**: `src/validation.rs::validate_password()`

---

## Data Encryption

### Algorithm: AES-256-GCM

**Selection Rationale**:
- Industry-standard authenticated encryption
- Hardware acceleration (AES-NI) support
- Built-in authentication (prevents tampering)

**Implementation**: `src/crypto.rs`

### Key Derivation: Argon2id

**Process**:
1. User enters password
2. Argon2id derives 256-bit key from password
3. Key used for AES-256-GCM encryption/decryption

**Key Storage**: Never stored (derived on-demand)

### Encrypted Fields Management

**Database Table**: `ENCRYPTED_FIELDS`

**Schema**:
```sql
CREATE TABLE ENCRYPTED_FIELDS (
    field_id INTEGER PRIMARY KEY,
    table_name TEXT NOT NULL,      -- Target table
    record_id INTEGER NOT NULL,    -- Target record ID
    field_name TEXT NOT NULL,      -- Target field name
    encrypted_value TEXT NOT NULL, -- AES-256-GCM encrypted data
    nonce TEXT NOT NULL,           -- GCM nonce (96-bit)
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);
```

**Key Design Points**:
- Separate nonce per field (GCM security requirement)
- Metadata stored for field tracking
- Flexible design (any table/field can be encrypted)

**Target Fields**:
- `TRANSACTIONS.memo`
- Future extensible to other sensitive fields

---

## Authentication & Authorization

### Authentication Flow

**Login Process** (`index.html`):
1. User enters username and password
2. Frontend: `auth.login(username, password)`
3. Backend: `verify_login()` verifies credentials
4. Session established on success

**Session Management**:
- In-memory session (desktop app scope)
- No persistent token (cleared on app close)

### Authorization Model

**Role Definition** (`src/consts.rs`):
```rust
pub const ROLE_ADMIN: i32 = 0;  // Administrator
pub const ROLE_USER: i32 = 1;   // General user
```

**Permission Matrix**:

| Feature | Admin | User |
|---------|-------|------|
| User management | ✓ | ✗ |
| Data encryption key | ✓ | ✗ |
| Transaction CRUD | ✓ | ✓ |
| Category management | ✓ | ✓ |
| Reports viewing | ✓ | ✓ |

**Validation**: `src/services/auth.rs::check_admin_permission()`

---

## Security Boundaries

### Trust Boundaries

**Trusted**:
- Application code (Rust backend)
- SQLite database (local file system)
- OS file system permissions

**Untrusted**:
- User input (all validated)
- External clipboard data

### Input Validation

**Validation Layers**:
1. **Frontend** (`res/js/validation-helpers.js`): UX immediate feedback
2. **Backend** (`src/validation.rs`): Security enforcement layer

**Validation Rules**:
- Password: ≥16 characters
- Username: 1-50 characters, alphanumeric + `_-.@`
- All string inputs: Length and format validation

**Implementation**: Defense in depth (frontend + backend)

---

## Threat Model

### Attack Surface Analysis

**Desktop Application Characteristics**:
- ✅ No network exposure (local only)
- ✅ OS-level file system protection
- ⚠️ Physical access risk (device theft/loss)
- ⚠️ Memory attack risk (debugging/dump)

### Threat Scenarios

#### 1. Password Brute Force Attack

**Threat**: Attacker attempts multiple password guesses

**Mitigation**:
- Argon2id makes each attempt computationally expensive
- 16-character minimum length requirement
- No account lockout (desktop app, no remote attack vector)

**Residual Risk**: Low (Argon2id provides sufficient protection)

#### 2. Database File Theft

**Threat**: Attacker copies `kakeibo.db` file

**Mitigation**:
- Passwords hashed (Argon2id, not reversible)
- Sensitive fields encrypted (AES-256-GCM)
- Encryption key derived from password (not in file)

**Residual Risk**: Medium (requires strong password from user)

#### 3. Memory Dump Attack

**Threat**: Attacker dumps application memory to extract keys

**Mitigation**:
- Keys derived on-demand, not stored long-term
- Rust's memory safety prevents unintended leaks
- Session cleared on app close

**Residual Risk**: Medium (requires elevated privileges + app running)

#### 4. SQL Injection

**Threat**: Malicious SQL via user input

**Mitigation**:
- sqlx parameterized queries (prevents injection)
- Input validation (additional defense)

**Residual Risk**: Very Low (sqlx provides strong protection)

#### 5. Path Traversal

**Threat**: Unauthorized file access via path manipulation

**Mitigation**:
- No user-controlled file paths
- Database path fixed in configuration

**Residual Risk**: Very Low (no attack vector)

---

## Security Testing

### Test Coverage

**Password Hashing** (`src/security.rs`):
- ✅ Hash generation and verification
- ✅ Invalid password rejection
- ✅ Hash format validation

**Encryption** (`src/crypto.rs`):
- ✅ Encryption/decryption round-trip
- ✅ Nonce uniqueness
- ✅ Wrong password detection

**Input Validation** (`src/validation.rs`):
- ✅ Password length validation
- ✅ Username format validation
- ✅ SQL injection pattern detection

**Total Security Tests**: 50+ tests

---

## Future Enhancements

### Short-term (Phase 1)
- [ ] Session timeout implementation
- [ ] Audit logging for sensitive operations
- [ ] Password change flow with re-encryption

### Medium-term (Phase 2)
- [ ] Master password key rotation
- [ ] Backup encryption
- [ ] Security event monitoring

### Long-term (Phase 3)
- [ ] Hardware security module (HSM) support
- [ ] Biometric authentication (OS integration)
- [ ] Zero-knowledge backup option

---

## References

- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [RFC 9106: Argon2 Memory-Hard Function](https://datatracker.ietf.org/doc/html/rfc9106)
- [NIST SP 800-38D: GCM Mode](https://csrc.nist.gov/publications/detail/sp/800-38d/final)

---

**Document Status**: ✅ Complete  
**Review Required**: Before public release
