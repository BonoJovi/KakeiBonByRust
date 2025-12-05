# Backend Test Index

This document provides a complete index of all backend tests implemented in Rust.

**Last Updated**: 2025-12-06 06:45 JST  
**Total Tests**: 201

---

## Table of Contents

- [Common Test Suites](#common-test-suites)
  - [validation_tests.rs](#validation_testsrs)
  - [test_helpers.rs](#test_helpersrs)
  - [font_size_tests.rs](#font_size_testsrs)
- [Inline Tests](#inline-tests)
  - [validation.rs](#validationrs)
  - [security.rs](#securityrs)
  - [crypto.rs](#cryptors)
  - [db.rs](#dbrs)
  - [settings.rs](#settingsrs)
  - [services/auth.rs](#servicesauthrs)
  - [services/user_management.rs](#servicesuser_managementrs)
  - [services/encryption.rs](#servicesencryptionrs)
  - [services/category.rs](#servicescategoryrs)
  - [services/manufacturer.rs](#servicesmanufacturerrs)
  - [services/product.rs](#servicesproductrs)
  - [services/shop.rs](#servicesshoprs)
  - [services/transaction.rs](#servicestransactionrs)
  - [services/aggregation.rs](#servicesaggregationrs)
  - [services/session.rs](#servicessessionrs)
  - [services/i18n.rs](#servicesi18nrs)

---

## Common Test Suites

### validation_tests.rs

Reusable test suite for password validation.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_empty_passwords` | Reject completely empty and whitespace-only passwords | src/validation_tests.rs | 12 |
| `test_whitespace_only_passwords` | Reject passwords with only spaces, tabs, newlines | src/validation_tests.rs | 22 |
| `test_short_passwords` | Reject passwords shorter than 16 characters | src/validation_tests.rs | 46 |
| `test_password_length_boundaries` | Boundary value test: 15 (reject), 16 (accept), 17 (accept) | src/validation_tests.rs | 63 |
| `test_valid_password_variations` | Accept valid passwords with special chars, Unicode, spaces | src/validation_tests.rs | 85 |
| `test_password_confirmation_logic` | Test password confirmation match/mismatch/case sensitivity | src/validation_tests.rs | 111 |
| `test_full_validation` | Full validation combining password and confirmation | src/validation_tests.rs | 132 |
| `test_validation_error_priority` | Test error priority when multiple errors exist | src/validation_tests.rs | 160 |
| `test_passwords_with_spaces` | Handle passwords with leading/trailing/internal spaces | src/validation_tests.rs | 176 |
| `test_boundary_cases` | Test very long passwords and special strings | src/validation_tests.rs | 191 |

**Total**: 10 tests

### test_helpers.rs

Test helper functions (no test functions, utilities only)

### font_size_tests.rs

Test suite for font size settings functionality.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_font_size_default` | Verify default font size | src/font_size_tests.rs | 39 |
| `test_set_font_size_small` | Test setting Small size | src/font_size_tests.rs | 59 |
| `test_set_font_size_medium` | Test setting Medium size | src/font_size_tests.rs | 74 |
| `test_set_font_size_large` | Test setting Large size | src/font_size_tests.rs | 89 |
| `test_validate_font_size_preset` | Validate preset sizes | src/font_size_tests.rs | 104 |
| `test_validate_font_size_custom_percentage` | Validate custom percentage | src/font_size_tests.rs | 120 |
| `test_invalid_font_size_custom_percentage` | Reject invalid custom percentage | src/font_size_tests.rs | 135 |
| `test_invalid_font_size_string` | Reject invalid strings | src/font_size_tests.rs | 151 |
| `test_font_size_persistence` | Test font size persistence | src/font_size_tests.rs | 167 |
| `test_font_size_custom_percentage_persistence` | Test custom percentage persistence | src/font_size_tests.rs | 185 |
| `test_font_size_boundary_values` | Test boundary values (50%, 200%) | src/font_size_tests.rs | 203 |
| `test_font_size_overwrite` | Test font size overwrite | src/font_size_tests.rs | 228 |
| `test_font_size_constants` | Verify font size constants | src/font_size_tests.rs | 250 |

**Total**: 13 tests

---

## Inline Tests

Tests implemented in `#[cfg(test)]` blocks within each functional module.

### validation.rs

Password validation logic tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_all_password_validations` | Run all password validation tests | src/validation.rs | 66 |
| `test_empty_password` | Reject empty password | src/validation.rs | 76 |
| `test_whitespace_only_password` | Reject whitespace-only password | src/validation.rs | 85 |
| `test_password_too_short` | Reject password that's too short (5 chars) | src/validation.rs | 94 |
| `test_single_character_password` | Reject single character password | src/validation.rs | 103 |
| `test_password_exactly_15_characters` | Reject exactly 15 character password | src/validation.rs | 112 |
| `test_password_exactly_16_characters` | Accept exactly 16 character password | src/validation.rs | 123 |
| `test_password_more_than_16_characters` | Accept 16+ character password | src/validation.rs | 130 |
| `test_password_with_spaces` | Accept password with spaces | src/validation.rs | 137 |
| `test_password_with_special_characters` | Accept password with special characters | src/validation.rs | 144 |
| `test_password_with_unicode` | Accept password with Unicode characters | src/validation.rs | 151 |
| `test_very_long_password` | Accept very long password (128 chars) | src/validation.rs | 158 |
| `test_password_confirmation_matching` | Test password confirmation match | src/validation.rs | 164 |
| `test_password_confirmation_not_matching` | Test password confirmation mismatch | src/validation.rs | 173 |
| `test_password_confirmation_case_sensitive` | Test case sensitivity in confirmation | src/validation.rs | 184 |
| `test_full_validation_with_valid_passwords` | Full validation (valid) | src/validation.rs | 191 |
| `test_full_validation_with_empty_password` | Full validation (empty password) | src/validation.rs | 197 |
| `test_full_validation_with_short_password` | Full validation (short password) | src/validation.rs | 204 |
| `test_full_validation_with_non_matching_passwords` | Full validation (mismatch) | src/validation.rs | 215 |
| `test_full_validation_error_priority` | Test error priority | src/validation.rs | 224 |
| `test_password_with_leading_trailing_spaces` | Password with leading/trailing spaces | src/validation.rs | 233 |
| `test_numeric_password` | Numeric-only password | src/validation.rs | 242 |
| `test_password_boundary_cases` | Boundary cases (15, 16, 17 chars) | src/validation.rs | 248 |

**Total**: 23 tests

### security.rs

Security functionality tests (password hashing, encryption key derivation).

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_hash_password` | Test password hashing | src/security.rs | 99 |
| `test_verify_password_success` | Test successful password verification | src/security.rs | 108 |
| `test_verify_password_failure` | Test failed password verification | src/security.rs | 117 |
| `test_hash_uniqueness` | Test hash uniqueness (same password, different hash) | src/security.rs | 127 |
| `test_derive_encryption_key` | Test encryption key derivation | src/security.rs | 141 |
| `test_derive_encryption_key_deterministic` | Test key derivation determinism (same input, same key) | src/security.rs | 151 |
| `test_derive_encryption_key_different_passwords` | Generate different keys for different passwords | src/security.rs | 163 |
| `test_derive_encryption_key_different_salts` | Generate different keys for different salts | src/security.rs | 174 |
| `test_derive_encryption_key_short_salt` | Error handling for short salt | src/security.rs | 187 |
| `test_empty_password_hash` | Hash empty password | src/security.rs | 197 |
| `test_long_password` | Hash long password | src/security.rs | 203 |
| `test_unicode_password` | Hash Unicode password | src/security.rs | 211 |
| `test_special_characters_password` | Hash password with special characters | src/security.rs | 219 |

**Total**: 13 tests

### crypto.rs

AES-256-GCM encryption/decryption tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_encrypt_decrypt_basic` | Basic encryption/decryption | src/crypto.rs | 120 |
| `test_encrypt_produces_different_outputs` | Same plaintext produces different ciphertexts | src/crypto.rs | 131 |
| `test_empty_string` | Encrypt empty string | src/crypto.rs | 147 |
| `test_long_string` | Encrypt long string | src/crypto.rs | 158 |
| `test_unicode_text` | Encrypt Unicode string | src/crypto.rs | 169 |
| `test_special_characters` | Encrypt special characters | src/crypto.rs | 180 |
| `test_newlines_and_whitespace` | Encrypt string with newlines and whitespace | src/crypto.rs | 191 |
| `test_different_keys_produce_different_results` | Different keys produce different ciphertexts | src/crypto.rs | 202 |
| `test_wrong_key_fails_decryption` | Decryption fails with wrong key | src/crypto.rs | 216 |
| `test_corrupted_ciphertext` | Decryption fails with corrupted ciphertext | src/crypto.rs | 230 |
| `test_invalid_base64` | Decryption fails with invalid Base64 | src/crypto.rs | 246 |
| `test_too_short_ciphertext` | Decryption fails with too short ciphertext | src/crypto.rs | 255 |
| `test_numeric_strings` | Encrypt numeric strings | src/crypto.rs | 265 |
| `test_json_like_string` | Encrypt JSON-like string | src/crypto.rs | 276 |
| `test_sql_like_string` | Encrypt SQL-like string | src/crypto.rs | 287 |

**Total**: 15 tests

### db.rs

Database initialization and migration tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_wal_mode_enabled` | Verify WAL mode is enabled | src/db.rs | 187 |
| `test_transactions_detail_migration` | Test transactions_detail table migration | src/db.rs | 217 |

**Total**: 2 tests

### settings.rs

Settings management functionality tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_settings_manager_creation` | Test SettingsManager creation | src/settings.rs | 188 |
| `test_get_and_set_string` | Get and set string values | src/settings.rs | 198 |
| `test_get_and_set_int` | Get and set integer values | src/settings.rs | 214 |
| `test_get_and_set_bool` | Get and set boolean values | src/settings.rs | 227 |
| `test_save_and_reload` | Save and reload settings | src/settings.rs | 240 |
| `test_remove_entry` | Remove entry | src/settings.rs | 258 |
| `test_entry_not_found` | Error handling for non-existent entry | src/settings.rs | 273 |
| `test_complex_type` | Save and retrieve complex types (JSON) | src/settings.rs | 289 |
| `test_keys_list` | Retrieve keys list | src/settings.rs | 315 |

**Total**: 9 tests

### services/auth.rs

Authentication service tests (user registration, login).

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_register_admin_user` | Test admin user registration | src/services/auth.rs | 246 |
| `test_authenticate_user_success` | Test successful authentication | src/services/auth.rs | 264 |
| `test_authenticate_user_wrong_password` | Test authentication failure with wrong password | src/services/auth.rs | 284 |
| `test_authenticate_user_nonexistent` | Test authentication failure with non-existent user | src/services/auth.rs | 300 |
| `test_has_users_empty` | Check user existence on empty DB | src/services/auth.rs | 312 |
| `test_has_users_with_user` | Check user existence with users | src/services/auth.rs | 322 |
| `test_password_is_hashed` | Verify password is hashed | src/services/auth.rs | 334 |
| `test_admin_role_assigned` | Verify admin role assignment | src/services/auth.rs | 355 |
| `test_multiple_authentication_attempts` | Test multiple authentication attempts | src/services/auth.rs | 372 |
| `test_special_characters_in_credentials` | Test special characters in credentials | src/services/auth.rs | 387 |
| `test_unicode_credentials` | Test Unicode in credentials | src/services/auth.rs | 402 |
| `test_role_constants_values` | Verify role constant values | src/services/auth.rs | 417 |
| `test_role_constants_uniqueness` | Verify role constant uniqueness | src/services/auth.rs | 425 |

**Total**: 13 tests

### services/user_management.rs

User management service tests (CRUD operations).

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_register_general_user` | Test general user registration | src/services/user_management.rs | 354 |
| `test_update_general_user` | Test general user update | src/services/user_management.rs | 371 |
| `test_update_general_user_username_only` | Update username only | src/services/user_management.rs | 396 |
| `test_update_general_user_password_only` | Update password only | src/services/user_management.rs | 417 |
| `test_update_general_user_username_and_password` | Update both username and password | src/services/user_management.rs | 447 |
| `test_update_admin_user` | Test admin user update | src/services/user_management.rs | 477 |
| `test_update_admin_user_username_only` | Update admin username only | src/services/user_management.rs | 493 |
| `test_update_admin_user_password_only` | Update admin password only | src/services/user_management.rs | 511 |
| `test_update_admin_user_username_and_password` | Update admin username and password | src/services/user_management.rs | 538 |
| `test_delete_general_user` | Test general user deletion | src/services/user_management.rs | 565 |
| `test_cannot_delete_admin_user` | Prevent admin user deletion | src/services/user_management.rs | 581 |
| `test_duplicate_username` | Test duplicate username error | src/services/user_management.rs | 592 |
| `test_list_users` | Test user list retrieval | src/services/user_management.rs | 606 |

**Total**: 13 tests

### services/encryption.rs

Encryption service tests (field encryption, re-encryption).

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_register_encrypted_field` | Test encrypted field registration | src/services/encryption.rs | 285 |
| `test_encrypt_decrypt_field` | Test field encryption/decryption | src/services/encryption.rs | 304 |
| `test_re_encrypt_user_data` | Test user data re-encryption | src/services/encryption.rs | 326 |
| `test_decrypt_with_wrong_password_fails` | Decryption fails with wrong password | src/services/encryption.rs | 380 |

**Total**: 4 tests

### services/category.rs

Category management service tests (3-tier category CRUD).

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_populate_default_categories` | Register default categories | src/services/category.rs | 1087 |
| `test_get_category1_list` | Get major category list | src/services/category.rs | 1155 |
| `test_add_category2` | Add medium category | src/services/category.rs | 1198 |
| `test_add_category2_duplicate_name` | Medium category duplicate name error | src/services/category.rs | 1234 |
| `test_add_category3` | Add minor category | src/services/category.rs | 1267 |
| `test_add_category3_duplicate_name` | Minor category duplicate name error | src/services/category.rs | 1301 |
| `test_move_category2_order` | Change medium category display order | src/services/category.rs | 1342 |
| `test_move_category3_order` | Change minor category display order | src/services/category.rs | 1426 |
| `test_update_category2` | Update medium category | src/services/category.rs | 1503 |
| `test_update_category3` | Update minor category | src/services/category.rs | 1527 |
| `test_update_category2_duplicate_name` | Medium category duplicate name update error | src/services/category.rs | 1552 |
| `test_move_category2_boundary` | Medium category boundary value move test | src/services/category.rs | 1571 |
| `test_get_category_for_edit` | Get category info for editing | src/services/category.rs | 1623 |

**Total**: 13 tests

### services/manufacturer.rs

Manufacturer management service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_add_manufacturer` | Test manufacturer addition | src/services/manufacturer.rs | 243 |
| `test_update_manufacturer` | Test manufacturer update | src/services/manufacturer.rs | 261 |
| `test_delete_manufacturer` | Test manufacturer deletion | src/services/manufacturer.rs | 292 |
| `test_empty_manufacturer_name` | Empty manufacturer name error | src/services/manufacturer.rs | 316 |
| `test_add_duplicate_manufacturer` | Duplicate manufacturer name error | src/services/manufacturer.rs | 331 |
| `test_update_to_duplicate_manufacturer_name` | Duplicate name update error | src/services/manufacturer.rs | 355 |
| `test_update_same_manufacturer_name` | Same name update (allowed) | src/services/manufacturer.rs | 389 |

**Total**: 7 tests

### services/product.rs

Product management service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_add_product_without_manufacturer` | Add product without manufacturer | src/services/product.rs | 256 |
| `test_add_product_with_manufacturer` | Add product with manufacturer | src/services/product.rs | 276 |
| `test_update_product` | Test product update | src/services/product.rs | 309 |
| `test_delete_product` | Test product deletion | src/services/product.rs | 342 |
| `test_empty_product_name` | Empty product name error | src/services/product.rs | 367 |
| `test_add_duplicate_product` | Duplicate product name error | src/services/product.rs | 383 |
| `test_manufacturer_deletion_sets_product_manufacturer_to_null` | Manufacturer deletion impact on products (CASCADE NULL) | src/services/product.rs | 409 |

**Total**: 7 tests

### services/shop.rs

Shop management service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_add_shop` | Test shop addition | src/services/shop.rs | 232 |
| `test_update_shop` | Test shop update | src/services/shop.rs | 249 |
| `test_delete_shop` | Test shop deletion | src/services/shop.rs | 278 |
| `test_empty_shop_name` | Empty shop name error | src/services/shop.rs | 301 |
| `test_add_duplicate_shop` | Duplicate shop name error | src/services/shop.rs | 315 |
| `test_update_to_duplicate_shop_name` | Duplicate name update error | src/services/shop.rs | 337 |
| `test_update_same_shop_name` | Same name update (allowed) | src/services/shop.rs | 368 |

**Total**: 7 tests

### services/transaction.rs

Transaction management service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_save_transaction_header_with_tax_excluded` | Save tax-excluded transaction header | src/services/transaction.rs | 1246 |
| `test_save_transaction_header_with_tax_included` | Save tax-included transaction header | src/services/transaction.rs | 1278 |
| `test_update_transaction_header_tax_type` | Update transaction header tax type | src/services/transaction.rs | 1309 |
| `test_default_tax_type_is_excluded` | Verify default tax type is excluded | src/services/transaction.rs | 1351 |
| `test_tax_type_validation_values` | Verify valid tax type values | src/services/transaction.rs | 1375 |

**Total**: 5 tests

### services/aggregation.rs

Aggregation service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_monthly_aggregation_current_month` | Monthly aggregation for current month | src/services/aggregation.rs | 1554 |
| `test_monthly_aggregation_next_month` | Monthly aggregation for next month | src/services/aggregation.rs | 1563 |

**Total**: 2 tests

### services/session.rs

Session management service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_session_state_initialization` | Session state initialization | src/services/session.rs | 92 |
| `test_set_and_get_user` | Set and get user info | src/services/session.rs | 101 |
| `test_clear_user` | Clear user info | src/services/session.rs | 119 |
| `test_set_and_get_source_screen` | Set and get source screen | src/services/session.rs | 136 |
| `test_clear_source_screen` | Clear source screen | src/services/session.rs | 144 |
| `test_set_and_get_category1_code` | Set and get category1 code | src/services/session.rs | 155 |
| `test_clear_category1_code` | Clear category1 code | src/services/session.rs | 163 |
| `test_clear_all` | Clear all session info | src/services/session.rs | 174 |
| `test_multiple_session_operations` | Multiple session operations | src/services/session.rs | 199 |

**Total**: 9 tests

### services/i18n.rs

Internationalization (i18n) service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_get_resource` | Test resource retrieval | src/services/i18n.rs | 220 |
| `test_get_with_params` | Retrieve resource with parameters | src/services/i18n.rs | 232 |
| `test_fallback_to_default` | Fallback to default language | src/services/i18n.rs | 241 |
| `test_get_by_category` | Retrieve resource by category | src/services/i18n.rs | 251 |
| `test_error_messages_exist` | Verify error messages exist | src/services/i18n.rs | 261 |
| `test_language_and_font_error_messages_exist` | Verify language/font error messages exist | src/services/i18n.rs | 285 |
| `test_validation_messages_exist` | Verify validation messages exist | src/services/i18n.rs | 307 |
| `test_all_error_messages_have_both_languages` | Verify all error messages exist in both languages | src/services/i18n.rs | 322 |

**Total**: 8 tests

---

## Test Statistics Summary

| Category | Test Count |
|----------|------------|
| **Common Test Suites** | **23** |
| validation_tests.rs | 10 |
| font_size_tests.rs | 13 |
| **Inline Tests** | **178** |
| validation.rs | 23 |
| security.rs | 13 |
| crypto.rs | 15 |
| db.rs | 2 |
| settings.rs | 9 |
| services/auth.rs | 13 |
| services/user_management.rs | 13 |
| services/encryption.rs | 4 |
| services/category.rs | 13 |
| services/manufacturer.rs | 7 |
| services/product.rs | 7 |
| services/shop.rs | 7 |
| services/transaction.rs | 5 |
| services/aggregation.rs | 2 |
| services/session.rs | 9 |
| services/i18n.rs | 8 |
| **Total** | **201** |

---

## How to Run Tests

### Run all tests

```bash
cargo test
```

### Run specific module

```bash
# Common test suites
cargo test validation_tests::
cargo test font_size_tests::

# Inline tests
cargo test validation::
cargo test security::
cargo test services::auth::
cargo test services::user_management::
```

### Run specific test function

```bash
cargo test test_empty_passwords
cargo test test_register_admin_user
```

### Run with output

```bash
cargo test -- --nocapture
```

### Generate coverage report

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

---

## Related Documents

- [Frontend Test Index](FRONTEND_TEST_INDEX.md) - Complete list of JavaScript tests
- [Test Overview](TEST_OVERVIEW.md) - Test strategy and execution guide
- [Test Design](TEST_DESIGN.md) - Test architecture and design philosophy
- [Test Results](TEST_RESULTS.md) - Latest test execution results
