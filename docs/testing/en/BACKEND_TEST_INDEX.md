# Backend Test Index

This document provides a complete index of all backend tests implemented in Rust.

**Last Updated**: 2026-08-26 JST  
**Total Tests**: 329 (delta-tracked; the full authoritative count from `cargo test --lib` is 571, and a follow-up pass will backfill the remaining pre-existing gap)

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
  - [api_error.rs](#api_errorrs)
  - [services/master_data.rs](#servicesmaster_datars)
  - [services/auth.rs](#servicesauthrs)
  - [services/user_management.rs](#servicesuser_managementrs)
  - [services/encryption.rs](#servicesencryptionrs)
  - [services/account.rs](#servicesaccountrs)
  - [services/category.rs](#servicescategoryrs)
  - [services/manufacturer.rs](#servicesmanufacturerrs)
  - [services/product.rs](#servicesproductrs)
  - [services/shop.rs](#servicesshoprs)
  - [services/transaction.rs](#servicestransactionrs)
  - [services/aggregation.rs](#servicesaggregationrs)
  - [services/session.rs](#servicessessionrs)
  - [services/i18n.rs](#servicesi18nrs)
  - [services/recurring.rs](#servicesrecurringrs)
  - [lib.rs](#librs)

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
| `test_password_with_unicode` | Accept password with Unicode characters (16 BMP JA chars) | src/validation.rs | 236 |
| `test_multibyte_password_below_min_length_rejected` | Reject 15-char JA password despite byte count >= 16 (Fable-5 #9 regression guard) | src/validation.rs | 249 |
| `test_multibyte_password_at_min_length_accepted` | Accept 16-char JA password at Unicode-scalar boundary | src/validation.rs | 262 |
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

**Total**: 25 tests

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
| `test_migrate_survives_orphaned_memo_reference` | Migration cope with FK-orphaned MEMO_ID from legacy DBs (Fable-5 #11) | src/db.rs | 1032 |
| `test_migrate_leaves_foreign_keys_on` | PRAGMA foreign_keys is restored to ON on the acquired connection after migration (Fable-5 #11) | src/db.rs | 1137 |
| `migrate_shops_unique_dedupes_and_repoints_references` | End-to-end proof that duplicate SHOPS rows collapse onto the smallest SHOP_ID, TRANSACTIONS_HEADER + RECURRING_RULES references are repointed, the unique index is created, and further duplicate inserts are rejected (PR15, Fable-5 #20) | src/db.rs | 1373 |
| `migrate_shops_unique_is_idempotent` | A second run of a successful migration is a no-op and leaves the data untouched (PR15, Fable-5 #20) | src/db.rs | 1424 |
| `migrate_shops_unique_scopes_per_user` | User A and User B may each own a shop with the same SHOP_NAME — the uniqueness scope is per-user (PR15, Fable-5 #20) | src/db.rs | 1437 |
| `migrate_shops_unique_keeps_active_row_over_soft_deleted_older_id` | When a soft-deleted old shop (smaller SHOP_ID, `IS_DISABLED=1`) coexists with a re-created active shop of the same name (larger SHOP_ID, `IS_DISABLED=0`), the migration keeps the active row as survivor and repoints legacy transaction references onto it (PR15, Devin #118 review) | src/db.rs | 1459 |
| `pool_connections_all_enforce_foreign_keys` | Every connection the pool hands out enforces `PRAGMA foreign_keys = ON`, not just the first one. Pre-fix, only the connection that ran the one-shot `execute()` at startup had FKs enabled; the SHOPS user-cascade migration was toothless on any borrower that got a later connection (CodeRabbit outside-diff on #128) | src/db.rs | 1630 |
| `migrate_shops_user_id_cascade_adds_cascade_fk_and_preserves_rows` | Table recreate swaps the SHOPS.USER_ID FK to `ON DELETE CASCADE` while keeping every SHOP_ID and column value verbatim (Fable-5 #11) | src/db.rs | 1711 |
| `migrate_shops_user_id_cascade_is_idempotent` | Second run of the SHOPS cascade migration finds the CASCADE FK already present and returns early — no DROP/RENAME on already-migrated DBs (Fable-5 #11) | src/db.rs | 1799 |
| `user_delete_cascades_to_shops_after_migration` | End-to-end guarantee: after the cascade migration, deleting a user with SHOPS rows succeeds and takes those rows with it — the pre-fix DELETE aborted with `FOREIGN KEY constraint failed` (Fable-5 #11) | src/db.rs | 1823 |

**Total**: 11 tests

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
| `test_save_leaves_no_tmp_sibling_and_target_is_parseable` | Successful save renames tmp away and leaves target valid (Fable-5 #10) | src/settings.rs | 345 |
| `test_repeated_saves_do_not_accumulate_tmp_files` | Repeated saves keep the filesystem clean (Fable-5 #10) | src/settings.rs | 378 |
| `test_stale_tmp_file_is_not_loaded` | A leftover `.tmp` from a crashed save is inert; real target still loads (Fable-5 #10) | src/settings.rs | 404 |

**Total**: 12 tests

### api_error.rs

`ApiError` — structured error type serialised into `{ code, message, entity? }` for the Tauri master-CRUD command wrappers. Introduced by Fable-5 review #23/#D4 so the frontend classifier can key off `err.code` instead of substring-matching English messages.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `duplicate_name_carries_lowercased_entity_and_stable_code` | `ApiError::duplicate_name("Shop")` → `code="duplicate_name"`, `entity="shop"` | src/api_error.rs | 128 |
| `not_found_carries_lowercased_entity_and_stable_code` | `ApiError::not_found("Manufacturer")` → `code="not_found"`, `entity="manufacturer"` | src/api_error.rs | 136 |
| `duplicate_code_carries_lowercased_entity_and_distinct_code` | `ApiError::duplicate_code("Account")` → `code="duplicate_code"` (distinct from `duplicate_name`) | src/api_error.rs | 145 |
| `admin_protected_carries_lowercased_entity_and_stable_code` | `ApiError::admin_protected("User")` → `code="admin_protected"` (for user-management delete guard) | src/api_error.rs | 154 |
| `manufacturer_not_found_has_its_own_code` | `ApiError::manufacturer_not_found()` → `code="manufacturer_not_found"` (distinct from generic `not_found`) | src/api_error.rs | 163 |
| `validation_carries_message_through_and_omits_entity` | `ApiError::validation(msg)` → `code="validation"`, message passed through, no entity | src/api_error.rs | 150 |
| `database_from_sqlx_row_not_found` | `sqlx::Error → ApiError::database` via `From<sqlx::Error>` | src/api_error.rs | 158 |
| `serialises_with_snake_case_code_and_optional_entity` | Serialised JSON has snake_case `code` and populated `entity` field | src/api_error.rs | 165 |
| `serialises_without_entity_key_when_none` | Serialised JSON omits `entity` when None (via `skip_serializing_if`) | src/api_error.rs | 174 |
| `in_use_carries_lowercased_entity_and_stable_code` | `ApiError::in_use("Shop")` → `code="in_use"`, `entity="shop"` (master delete-lock guard) | src/api_error.rs | 230 |

**Total**: 10 tests

### services/master_data.rs

Pure-Rust tests for the shared master-CRUD helpers (`MasterCrudSpec` + `ensure_update_affected_one` + `run_delete_expect_one`) introduced in PR3 (Fable-5 #26).

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `ensure_update_affected_one_maps_zero_to_not_found` | `rows_affected == 0` on an UPDATE maps to `ApiError::not_found(entity)` from the spec | src/services/master_data.rs | 228 |
| `ensure_update_affected_one_passes_positive_count` | Positive rows_affected returns Ok (boundary: 1, 42) | src/services/master_data.rs | 235 |
| `reject_if_in_use_maps_positive_flag_to_in_use` | Positive in-use flag maps to `ApiError::in_use(entity)` (master delete-lock guard) | src/services/master_data.rs | 256 |
| `reject_if_in_use_passes_when_flag_is_zero` | Zero in-use flag returns Ok (master delete-lock guard) | src/services/master_data.rs | 263 |

**Total**: 4 tests

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
| `invalid_credentials_maps_to_auth_invalid_credentials_code` | `AuthError::InvalidCredentials` → `ApiError { code: "auth_invalid_credentials" }` (PR14, Fable-5 #21) | src/services/auth.rs | 577 |
| `database_error_maps_to_database_code` | `AuthError::DatabaseError` → `ApiError { code: "database" }` (PR14, Fable-5 #21) | src/services/auth.rs | 585 |
| `security_error_maps_to_validation_code_with_message` | `AuthError::SecurityError` → `ApiError { code: "validation" }` with message preserved (PR14, Fable-5 #21) | src/services/auth.rs | 593 |

**Total**: 16 tests

### services/user_management.rs

User management service tests (CRUD operations).

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_register_general_user` | Test general user registration | src/services/user_management.rs | 502 |
| `test_update_general_user` | Test general user update | src/services/user_management.rs | 519 |
| `test_update_general_user_username_only` | Update username only | src/services/user_management.rs | 547 |
| `test_update_general_user_password_only` | Update password only via `_with_password` (Fable-5 #1/#5) | src/services/user_management.rs | 568 |
| `test_update_general_user_username_and_password` | Update username + password atomically via `_with_password` (Fable-5 #1/#5) | src/services/user_management.rs | 601 |
| `test_update_admin_user` | Test admin user update | src/services/user_management.rs | 634 |
| `test_update_admin_user_username_only` | Update admin username only | src/services/user_management.rs | 650 |
| `test_update_admin_user_password_only` | Update admin password only via `_with_password` (Fable-5 #1/#5) | src/services/user_management.rs | 668 |
| `test_update_admin_user_username_and_password` | Update admin username + password atomically (Fable-5 #1/#5) | src/services/user_management.rs | 698 |
| `test_delete_general_user` | Test general user deletion | src/services/user_management.rs | 728 |
| `test_cannot_delete_admin_user` | Prevent admin user deletion | src/services/user_management.rs | 744 |
| `test_duplicate_username` | Test duplicate username error | src/services/user_management.rs | 755 |
| `test_list_users` | Test user list retrieval | src/services/user_management.rs | 769 |
| `test_register_general_user_accepts_max_chars_of_multibyte_name` | USERS.NAME length guard counts characters, not bytes: accept MAX_NAME_LEN multibyte (issue #37) | src/services/user_management.rs | 785 |
| `test_register_general_user_rejects_over_max_chars_of_multibyte_name` | Reject MAX_NAME_LEN+1 multibyte on registration (issue #37) | src/services/user_management.rs | 796 |
| `test_update_general_user_with_password_rejects_wrong_old_password` | Wrong current password → `OldPasswordIncorrect`; hash + username unchanged (Fable-5 #1/#5) | src/services/user_management.rs | 817 |
| `test_update_general_user_with_password_rename_only_rejects_wrong_old_password` | Rename-only branch also classifies as `OldPasswordIncorrect` (CodeRabbit on #123) | src/services/user_management.rs | 864 |
| `test_update_admin_user_with_password_rejects_wrong_old_password` | Admin-side counterpart: wrong current password → `OldPasswordIncorrect`; hash unchanged (Fable-5 #1/#5) | src/services/user_management.rs | 891 |
| `test_update_general_user_rejects_over_max_chars_of_multibyte_name` | Reject MAX_NAME_LEN+1 multibyte on rename (issue #37) | src/services/user_management.rs | 920 |

**Total**: 19 tests

### services/encryption.rs

Encryption service tests (field encryption, re-encryption).

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_register_encrypted_field` | Test encrypted field registration | src/services/encryption.rs | 285 |
| `test_encrypt_decrypt_field` | Test field encryption/decryption | src/services/encryption.rs | 304 |
| `test_re_encrypt_user_data` | Test user data re-encryption | src/services/encryption.rs | 326 |
| `test_decrypt_with_wrong_password_fails` | Decryption fails with wrong password | src/services/encryption.rs | 380 |
| `test_re_encrypt_user_data_preserves_per_row_plaintext` | Multi-row re-encryption keeps each row's own plaintext (Fable-5 #14) | src/services/encryption.rs | 473 |
| `test_encrypt_uses_per_user_salt_not_user_id` | Same password/plaintext produces distinct ciphertext across users (Fable-5 #15) | src/services/encryption.rs | 657 |
| `test_encrypt_decrypt_salt_survives_service_reconstruction` | Salt is refetched from DB so a new service instance round-trips ciphertext (Fable-5 #15) | src/services/encryption.rs | 703 |
| `test_encrypt_errors_when_user_missing` | Missing USERS row errors loudly instead of falling back to user_id salt (Fable-5 #15) | src/services/encryption.rs | 722 |

**Total**: 8 tests

### services/account.rs

Account management service tests. Assertions on empty-name and duplicate-code paths migrated to `ApiError { code: "validation" | "duplicate_code" }` when moved off `Result<_, String>` (Fable-5 #23); behaviour is unchanged.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_add_account_rejects_empty_name` | Empty account name returns `ApiError { code: "validation" }` (Fable-5 #16, #23) | src/services/account.rs | 597 |
| `test_add_account_rejects_whitespace_only_name` | Whitespace-only account name returns `ApiError { code: "validation" }` (Fable-5 #16, #23) | src/services/account.rs | 612 |
| `test_update_account_rejects_empty_name` | Empty account name via update returns `ApiError { code: "validation" }` (Fable-5 #16, #23) | src/services/account.rs | 627 |
| `test_update_account_not_found_has_stable_code_and_entity` | Updating a missing account returns `ApiError { code: "not_found", entity: "account" }` (Fable-5 #23) | src/services/account.rs | 815 |
| `test_delete_account_not_found_has_stable_code_and_entity` | Deleting a missing account returns `ApiError { code: "not_found" }` (Fable-5 #23) | src/services/account.rs | 837 |
| `test_delete_account_rejected_when_referenced_as_from_account` | Delete rejected with `ApiError { code: "in_use" }` when a TRANSACTIONS_HEADER row names the account as FROM (master delete-lock) | src/services/account.rs | 827 |
| `test_delete_account_rejected_when_referenced_as_to_account` | Delete rejected with `ApiError { code: "in_use" }` when a TRANSACTIONS_HEADER row names the account as TO (master delete-lock) | src/services/account.rs | 847 |
| `test_delete_account_rejected_when_referenced_by_recurring_rule` | Delete rejected with `ApiError { code: "in_use" }` when any RECURRING_RULES row names the account (master delete-lock) | src/services/account.rs | 864 |
| `test_delete_account_ignores_other_users_references` | Cross-user references to the same ACCOUNT_CODE do NOT block delete — codes are user-scoped (master delete-lock) | src/services/account.rs | 881 |
| `test_delete_account_normalizes_input_before_in_use_check` | Delete input (`"  cash  "`) is uppercased/trimmed before the CHECK_IN_USE query so the guard fires (master delete-lock) | src/services/account.rs | 899 |
| `test_get_account_balances_as_of_self_transfer_nets_to_zero` | Stale TRANSFER row with FROM == TO nets to zero on the dashboard instead of inflating the balance (Fable-5 #20) | src/services/account.rs | 991 |

**Total**: 11 tests

### services/category.rs

Category management service tests (3-tier category CRUD). Internal `CategoryError` variants map to `ApiError { code, message, entity? }` at the Tauri wrapper boundary via `From<CategoryError>` (Fable-5 #23).

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
| `test_get_category2_for_edit_returns_not_found_for_missing` | Missing CATEGORY2 edit fetch returns NotFound (Fable-5 #6) | src/services/category.rs | 1873 |
| `test_get_category3_for_edit_returns_not_found_for_missing` | Missing CATEGORY3 edit fetch returns NotFound (Fable-5 #6) | src/services/category.rs | 1884 |
| `test_disable_category2_returns_not_found_for_missing` | Missing CATEGORY2 disable returns NotFound (Fable-5 #7) | src/services/category.rs | 1899 |
| `test_disable_category3_returns_not_found_for_missing` | Missing CATEGORY3 disable returns NotFound (Fable-5 #7) | src/services/category.rs | 1910 |
| `test_disable_category2_succeeds_with_no_children` | Leaf CATEGORY2 disable succeeds (child sweep tolerates 0 rows) | src/services/category.rs | 1926 |
| `not_found_maps_to_not_found_code_with_category_entity` | `CategoryError::NotFound` → `ApiError { code: "not_found", entity: "category" }` (Fable-5 #23) | src/services/category.rs | 2101 |
| `duplicate_name_maps_to_duplicate_name_code_with_category_entity` | `CategoryError::DuplicateName(_)` → `ApiError { code: "duplicate_name", entity: "category" }` (Fable-5 #23) | src/services/category.rs | 2108 |
| `validation_preserves_message_and_omits_entity` | `CategoryError::Validation(msg)` → `ApiError { code: "validation" }` with message preserved (Fable-5 #23) | src/services/category.rs | 2115 |
| `database_error_maps_to_database_code` | `CategoryError::DatabaseError(_)` → `ApiError { code: "database" }` (Fable-5 #23) | src/services/category.rs | 2125 |
| `test_get_category_tree_groups_children_under_parent` | Regression pin for the 3-flat-queries + HashMap grouping shape: cat1 → cat2 → cat3 parent/child pairing is preserved (PR11, Fable-5 #31) | src/services/category.rs | 2022 |
| `test_get_category_tree_preserves_display_order` | Confirms that a `move_category2_up` reorder survives the flat-query regrouping (PR11, Fable-5 #31) | src/services/category.rs | 2077 |
| `test_get_category_tree_all_includes_disabled_flags` | `get_category_tree_all` still includes disabled rows and their `is_disabled` fields; the visible-only `get_category_tree` filters them out (PR11, Fable-5 #31) | src/services/category.rs | 2106 |

**Total**: 25 tests

### services/manufacturer.rs

Manufacturer management service tests. Empty/duplicate assertion tests renamed to `_returns_validation_code` / `_returns_duplicate_name_code` when migrated to `ApiError` (Fable-5 #23) — behaviour unchanged, only assertion targets.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_add_manufacturer` | Test manufacturer addition | src/services/manufacturer.rs | 243 |
| `test_update_manufacturer` | Test manufacturer update | src/services/manufacturer.rs | 261 |
| `test_delete_manufacturer` | Test manufacturer deletion | src/services/manufacturer.rs | 292 |
| `test_empty_manufacturer_name_returns_validation_code` | Empty manufacturer name returns `ApiError { code: "validation" }` (Fable-5 #23) | src/services/manufacturer.rs | 316 |
| `test_add_duplicate_manufacturer_returns_duplicate_name_code` | Duplicate returns `ApiError { code: "duplicate_name", entity: "manufacturer" }` (Fable-5 #23) | src/services/manufacturer.rs | 331 |
| `test_update_to_duplicate_manufacturer_name_returns_duplicate_name_code` | Update to duplicate returns `ApiError { code: "duplicate_name" }` (Fable-5 #23) | src/services/manufacturer.rs | 355 |
| `test_update_missing_manufacturer_returns_not_found_code` | Updating a missing manufacturer returns `ApiError { code: "not_found", entity: "manufacturer" }` (Fable-5 #23) | src/services/manufacturer.rs | 383 |
| `test_delete_missing_manufacturer_returns_not_found_code` | Deleting a missing manufacturer returns `ApiError { code: "not_found" }` (Fable-5 #23) | src/services/manufacturer.rs | 398 |
| `test_update_same_manufacturer_name` | Same name update (allowed) | src/services/manufacturer.rs | 405 |
| `test_delete_manufacturer_rejected_when_referenced_by_product` | Delete rejected with `ApiError { code: "in_use", entity: "manufacturer" }` when any PRODUCTS row names the manufacturer (master delete-lock) | src/services/manufacturer.rs | 374 |
| `test_delete_manufacturer_rejected_when_only_disabled_products_reference` | Even IS_DISABLED products count as a reference — the FK link exists and the products screen still surfaces them (master delete-lock) | src/services/manufacturer.rs | 402 |
| `test_delete_manufacturer_ignores_other_users_references` | Cross-user products with the same MANUFACTURER_ID do NOT block delete — scoping is by USER_ID (master delete-lock) | src/services/manufacturer.rs | 429 |

**Total**: 12 tests

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
| `test_manufacturer_deletion_rejected_while_product_references_it` | Manufacturer delete rejected with `ApiError { code: "in_use", entity: "manufacturer" }` while any product still references it — renamed from `test_manufacturer_deletion_sets_product_manufacturer_to_null` when the master delete-lock landed (was: fallback ON DELETE SET NULL) | src/services/product.rs | 512 |
| `test_add_product_rejects_foreign_manufacturer_id` | Cross-owner manufacturer_id on add returns "Manufacturer not found" (Fable-5 #13) | src/services/product.rs | 716 |
| `test_add_product_rejects_nonexistent_manufacturer_id` | Nonexistent manufacturer_id on add returns "Manufacturer not found" (Fable-5 #13) | src/services/product.rs | 767 |
| `test_update_product_rejects_foreign_manufacturer_id` | Cross-owner manufacturer_id on update returns "Manufacturer not found" (Fable-5 #13) | src/services/product.rs | 792 |
| `test_product_join_scopes_manufacturer_by_user_id` | PRODUCT_GET_* JOIN must not leak another user's manufacturer name (Fable-5 #13) | src/services/product.rs | 871 |
| `test_delete_product_rejected_when_referenced_by_transaction_detail` | Delete rejected with `ApiError { code: "in_use", entity: "product" }` when any TRANSACTIONS_DETAIL row (scoped via TRANSACTIONS_HEADER.USER_ID) names the product (master delete-lock) | src/services/product.rs | 444 |
| `test_delete_product_ignores_other_users_transaction_details` | Cross-user detail rows do NOT block delete — scoping runs through TRANSACTIONS_HEADER.USER_ID (master delete-lock) | src/services/product.rs | 481 |

**Total**: 13 tests

### services/shop.rs

Shop management service tests. Empty/duplicate assertion tests renamed to `_returns_validation_code` / `_returns_duplicate_name_code` when migrated to `ApiError` (Fable-5 #23) — behaviour unchanged, only assertion targets.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_add_shop` | Test shop addition | src/services/shop.rs | 232 |
| `test_update_shop` | Test shop update | src/services/shop.rs | 249 |
| `test_delete_shop` | Test shop deletion | src/services/shop.rs | 278 |
| `test_empty_shop_name_returns_validation_code` | Empty shop name returns `ApiError { code: "validation" }` (Fable-5 #23) | src/services/shop.rs | 301 |
| `test_add_duplicate_shop_returns_duplicate_name_code` | Duplicate returns `ApiError { code: "duplicate_name", entity: "shop" }` (Fable-5 #23) | src/services/shop.rs | 315 |
| `test_update_to_duplicate_shop_name_returns_duplicate_name_code` | Update to duplicate returns `ApiError { code: "duplicate_name" }` (Fable-5 #23) | src/services/shop.rs | 337 |
| `test_update_missing_shop_returns_not_found_code` | Updating a missing shop returns `ApiError { code: "not_found", entity: "shop" }` (Fable-5 #23) | src/services/shop.rs | 363 |
| `test_delete_missing_shop_returns_not_found_code` | Deleting a missing shop returns `ApiError { code: "not_found" }` (Fable-5 #23) | src/services/shop.rs | 375 |
| `test_update_same_shop_name` | Same name update (allowed) | src/services/shop.rs | 382 |
| `test_delete_shop_rejected_when_referenced_by_transaction` | Delete rejected with `ApiError { code: "in_use", entity: "shop" }` when any TRANSACTIONS_HEADER row names the shop (master delete-lock) | src/services/shop.rs | 346 |
| `test_delete_shop_rejected_when_referenced_by_recurring_rule` | Delete rejected with `ApiError { code: "in_use" }` when any RECURRING_RULES row names the shop (master delete-lock) | src/services/shop.rs | 372 |
| `test_delete_shop_ignores_other_users_references` | Cross-user references to the same SHOP_ID do NOT block delete — scoping is by USER_ID (master delete-lock) | src/services/shop.rs | 394 |

**Total**: 12 tests

### services/transaction.rs

Transaction management service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_save_transaction_header_with_tax_excluded` | Save tax-excluded transaction header | src/services/transaction.rs | 1246 |
| `test_save_transaction_header_with_tax_included` | Save tax-included transaction header | src/services/transaction.rs | 1278 |
| `test_update_transaction_header_tax_type` | Update transaction header tax type | src/services/transaction.rs | 1309 |
| `test_default_tax_type_is_excluded` | Verify default tax type is excluded | src/services/transaction.rs | 1351 |
| `test_tax_type_validation_values` | Verify valid tax type values | src/services/transaction.rs | 1375 |
| `test_get_transactions_end_date_includes_boundary_day` | End-date filter must include same-day timestamps (bare 'YYYY-MM-DD' anchored to 23:59:59) | src/services/transaction.rs | 3293 |
| `test_get_transactions_keyword_matches_header_and_detail_memo` | Keyword must substring-match memo text on both header and detail rows | src/services/transaction.rs | 3364 |
| `test_update_detail_memo_does_not_corrupt_shared_header_memo` | Detail memo edit must not clobber header memo sharing MEMO_ID | src/services/transaction.rs | 3584 |
| `test_delete_detail_preserves_memo_still_referenced_by_header` | Detail delete must keep memo row when header still references it | src/services/transaction.rs | 3618 |
| `test_update_detail_memo_updates_in_place_when_not_shared` | Solo-referenced memo still updates in place | src/services/transaction.rs | 3644 |
| `test_delete_detail_removes_orphaned_memo` | Solo-referenced memo is deleted when detail removed | src/services/transaction.rs | 3674 |
| `test_clear_detail_memo_does_not_delete_memo_still_used_by_header` | Clearing shared detail memo must not delete memo row used by header | src/services/transaction.rs | 3706 |
| `test_update_detail_memo_does_not_corrupt_recurring_rule_memo` | Detail memo edit must not overwrite memo shared with a recurring rule | src/services/transaction.rs | 3760 |
| `test_delete_detail_preserves_memo_still_referenced_by_recurring_rule` | Detail delete must keep memo row still referenced by a recurring rule | src/services/transaction.rs | 3805 |
| `test_clear_detail_memo_succeeds_under_foreign_keys_on` | Clearing a detail memo must not violate the MEMOS foreign key | src/services/transaction.rs | 3843 |
| `test_add_detail_rejects_foreign_transaction_id` | Adding a detail against another user's transaction_id must return NotFound (Fable-5 #12) | src/services/transaction.rs | 4109 |
| `test_add_detail_rejects_nonexistent_transaction_id` | Adding a detail against a missing transaction_id must return NotFound (Fable-5 #12) | src/services/transaction.rs | 4142 |
| `not_found_maps_to_not_found_code_with_transaction_entity` | TransactionError::NotFound maps to ApiError::not_found("transaction") (PR2b) | src/services/transaction.rs | 4199 |
| `validation_preserves_message_and_omits_entity` | TransactionError::ValidationError maps to ApiError::CODE_VALIDATION with the message preserved (PR2b) | src/services/transaction.rs | 4206 |
| `database_error_maps_to_database_code` | TransactionError::DatabaseError maps to ApiError::CODE_DATABASE (PR2b) | src/services/transaction.rs | 4217 |
| `field_needle_message_survives_conversion_for_frontend_routing` | Two field needles (`"Item name must be"` / `"Memo must be"`) survive at the head of the wire message so the frontend `startsWith` routing keeps working (PR2b) | src/services/transaction.rs | 4224 |
| `test_find_matching_pattern_preserves_user_half_up_when_settings_match` | `HALF_UP + EXCLUDED` stored on a round-cent receipt (500円 × 10% = 550円) survives bulk recalc instead of being silently downgraded to FLOOR (Fable-5 #2) | src/services/transaction.rs | 1802 |
| `test_find_matching_pattern_preserves_user_ceil_when_settings_match` | Same guarantee for `UP + EXCLUDED` (Fable-5 #2) | src/services/transaction.rs | 1819 |
| `test_find_matching_pattern_falls_back_to_priority_when_preferred_mismatches` | When the stored settings do not reproduce the total, fall back to the priority-ordered PATTERNS scan (Fable-5 #2) | src/services/transaction.rs | 1836 |
| `test_find_matching_pattern_returns_none_when_no_pattern_fits` | No combination reproduces the target → `None`, caller overwrites TOTAL_AMOUNT instead of the setting columns (Fable-5 #2) | src/services/transaction.rs | 1859 |
| `test_save_header_rejects_invalid_tax_included_type` | `save_transaction_header` rejects `tax_included_type` outside `{TAX_INCLUDED, TAX_EXCLUDED}` so a bogus value cannot survive `find_matching_pattern`'s preferred-first check (CodeRabbit on #125) | src/services/transaction.rs | 3191 |
| `test_update_header_rejects_invalid_tax_included_type` | Same guard on the update entry point (CodeRabbit on #125) | src/services/transaction.rs | 3218 |
| `test_save_header_rejects_transfer_from_equals_to` | `save_transaction_header` rejects TRANSFER with FROM == TO so a self-transfer cannot inflate the dashboard balance (Fable-5 #20) | src/services/transaction.rs | 3249 |
| `test_update_header_rejects_transfer_from_equals_to` | Same guard on the update entry point (Fable-5 #20) | src/services/transaction.rs | 3277 |
| `transfer_same_account_maps_to_stable_wire_code_and_omits_entity` | `TransactionError::TransferSameAccount` maps to `ApiError { code: "transfer_same_account", entity: None }` — pins the wire contract so a future refactor cannot silently downgrade to the generic `validation` fallback (CodeRabbit on #127) | src/services/transaction.rs | 4343 |

**Total**: 30 tests

### services/aggregation.rs

Aggregation service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_monthly_aggregation_current_month` | Monthly aggregation for current month | src/services/aggregation.rs | 1554 |
| `test_monthly_aggregation_next_month` | Monthly aggregation for next month | src/services/aggregation.rs | 1563 |
| `test_detail_query_grosses_up_null_tax_included_row` | NULL AMOUNT_INCLUDING_TAX at TAX_RATE>0 is grossed up, not dropped (Fable-5 #3) | src/services/aggregation.rs | 2581 |
| `test_detail_query_grosses_up_zero_tax_included_row` | AMOUNT_INCLUDING_TAX=0 (frontend empty-input sentinel) is treated as pre-tax (Fable-5 #3) | src/services/aggregation.rs | 2610 |
| `test_detail_query_included_header_legacy_null_row_no_double_taxation` | Header `TAX_INCLUDED_TYPE = TAX_INCLUDED (0)` + legacy `AMOUNT_INCLUDING_TAX = NULL` row is treated as already-included, not grossed up a second time (Fable-5 #3 residual) | src/services/aggregation.rs | 2653 |
| `test_detail_query_included_header_zero_col_no_double_taxation` | Same #3 residual with `AMOUNT_INCLUDING_TAX = 0` (frontend empty-input sentinel) under a tax-included header | src/services/aggregation.rs | 2690 |
| `test_detail_query_matches_header_query_for_included_ledger` | Header-dim vs detail-dim aggregation agree on the same tax-included transaction (Fable-5 #4) | src/services/aggregation.rs | 2726 |
| `test_detail_query_avg_matches_total_over_count_with_mixed_rates` | avg × count == total holds for a mixed-rate transaction (Fable-5 #4) | src/services/aggregation.rs | 2774 |
| `test_detail_query_avg_multi_transaction_arithmetic` | avg = total / txn_count over 2 transactions (Fable-5 #4) | src/services/aggregation.rs | 2811 |
| `test_detail_query_binds_category_filter_no_injection` | End-to-end proof that a category filter's value is bound, not inlined: an `EXPENSE' OR '1'='1` payload returns 0 rows (PR5, Fable-5 #25) | src/services/aggregation.rs | 2846 |
| `test_category_filter_category2_targets_detail_column` | Category2 filter now targets the existent `td.CATEGORY2_CODE` (detail scope) instead of the non-existent `th.CATEGORY2_CODE` (PR6, Fable-5 #17) | src/services/aggregation.rs | 2902 |
| `test_category_filter_category3_targets_detail_column` | Category3 filter targets `td.CATEGORY2/3_CODE` (PR6, Fable-5 #17) | src/services/aggregation.rs | 2918 |
| `test_account_query_applies_category_filter_to_all_union_branches` | Account UNION ALL query now applies the category filter to all 4 branches and binds the value 4x — regression pin for the silent drop (PR6, Fable-5 #18) | src/services/aggregation.rs | 2943 |

**Total**: 13 tests

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

### services/recurring.rs

Recurring transaction rule service tests.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `test_delete_rule_returns_not_found_for_missing` | Delete of a missing rule returns NotFound instead of empty-commit fake success (Fable-5 #8) | src/services/recurring.rs | 1811 |
| `not_found_maps_to_not_found_code_with_recurring_rule_entity` | RecurringError::NotFound maps to ApiError::not_found("recurring rule") (PR2a) | src/services/recurring.rs | 1833 |
| `validation_preserves_message_and_omits_entity` | RecurringError::Validation maps to ApiError::CODE_VALIDATION with the message preserved (PR2a) | src/services/recurring.rs | 1840 |
| `database_error_maps_to_database_code` | RecurringError::Database maps to ApiError::CODE_DATABASE (PR2a) | src/services/recurring.rs | 1851 |
| `field_needle_message_survives_conversion_for_frontend_routing` | Four field needles (`"Rule name must be"` etc.) survive at the head of the wire message so the frontend `startsWith` routing keeps working (PR2a) | src/services/recurring.rs | 1858 |

**Total**: 5 tests

### lib.rs

Settings value validation used by the `set_language` / `set_font_size` / `update_user_settings` commands.

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `normalize_language_accepts_names_and_codes` | Accept language names and codes (en/English/ja/日本語/Japanese) | src/lib.rs | 2733 |
| `normalize_language_rejects_unknown_values` | Reject unknown language values | src/lib.rs | 2742 |
| `normalize_font_size_accepts_keywords_and_percentages` | Accept size keywords and percentages in 50-200 | src/lib.rs | 2748 |
| `normalize_font_size_rejects_out_of_range_and_garbage` | Reject out-of-range percentages and invalid strings | src/lib.rs | 2757 |
| `monthly_bounds_with_shift_rejects_out_of_range_month` | month=0/13/100 short-circuits to Err before reaching `services::period::end_of_month` — prevents the backend thread crash (PR6, Fable-5 #22) | src/lib.rs | 2695 |
| `monthly_bounds_with_shift_accepts_boundary_months` | month=1/12 boundaries still accepted (PR6, Fable-5 #22) | src/lib.rs | 2722 |

**Total**: 6 tests

---

## Test Statistics Summary

| Category | Test Count |
|----------|------------|
| **Common Test Suites** | **23** |
| validation_tests.rs | 10 |
| font_size_tests.rs | 13 |
| **Inline Tests** | **306** |
| validation.rs | 25 |
| security.rs | 13 |
| crypto.rs | 15 |
| db.rs | 12 |
| settings.rs | 12 |
| api_error.rs | 10 |
| services/master_data.rs | 4 |
| services/auth.rs | 16 |
| services/user_management.rs | 19 |
| services/encryption.rs | 8 |
| services/account.rs | 11 |
| services/category.rs | 25 |
| services/manufacturer.rs | 12 |
| services/product.rs | 13 |
| services/shop.rs | 12 |
| services/transaction.rs | 30 |
| services/aggregation.rs | 13 |
| services/session.rs | 9 |
| services/i18n.rs | 8 |
| services/recurring.rs | 5 |
| lib.rs | 6 |
| **Total** | **329** |

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
