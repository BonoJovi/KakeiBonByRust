# バックエンドテストインデックス

このドキュメントは、Rustで実装されたバックエンドテストの完全なインデックスです。

**最終更新**: 2026-08-26 JST  
**総テスト数**: 321件 (差分反映後。`cargo test --lib` の権威的総数は 563 で、既存の未反映分は別 PR でバックフィル予定)

---

## 目次

- [共通テストスイート](#共通テストスイート)
  - [validation_tests.rs](#validation_testsrs)
  - [test_helpers.rs](#test_helpersrs)
  - [font_size_tests.rs](#font_size_testsrs)
- [インラインテスト](#インラインテスト)
  - [validation.rs](#validationrs)
  - [security.rs](#securityrs)
  - [crypto.rs](#cryptors)
  - [db.rs](#dbrs)
  - [settings.rs](#settingsrs)
  - [services/auth.rs](#servicesauthrs)
  - [services/user_management.rs](#servicesuser_managementrs)
  - [services/encryption.rs](#servicesencryptionrs)
  - [services/account.rs](#servicesaccountrs)
  - [services/category.rs](#servicescategoryrs)
  - [api_error.rs](#api_errorrs)
  - [services/master_data.rs](#servicesmaster_datars)
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

## 共通テストスイート

### validation_tests.rs

パスワードバリデーションの再利用可能なテストスイート。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_empty_passwords` | 完全に空のパスワードと空白のみのパスワードを拒否 | src/validation_tests.rs | 12 |
| `test_whitespace_only_passwords` | スペース、タブ、改行のみのパスワードを拒否 | src/validation_tests.rs | 22 |
| `test_short_passwords` | 16文字未満のパスワードを拒否 | src/validation_tests.rs | 46 |
| `test_password_length_boundaries` | 15文字（拒否）、16文字（受け入れ）、17文字（受け入れ）の境界値テスト | src/validation_tests.rs | 63 |
| `test_valid_password_variations` | 特殊文字、Unicode、スペースを含む有効なパスワードを受け入れ | src/validation_tests.rs | 85 |
| `test_password_confirmation_logic` | パスワード確認の一致・不一致・大文字小文字区別のテスト | src/validation_tests.rs | 111 |
| `test_full_validation` | パスワードと確認を組み合わせた完全なバリデーション | src/validation_tests.rs | 132 |
| `test_validation_error_priority` | 複数のエラーがある場合の優先順位テスト | src/validation_tests.rs | 160 |
| `test_passwords_with_spaces` | 先頭・末尾・中間にスペースがあるパスワードの処理 | src/validation_tests.rs | 176 |
| `test_boundary_cases` | 非常に長いパスワード、特殊な文字列のテスト | src/validation_tests.rs | 191 |

**合計**: 10件

### test_helpers.rs

テストヘルパー関数（テスト関数なし、ユーティリティのみ）

### font_size_tests.rs

フォントサイズ設定機能のテストスイート。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_font_size_default` | デフォルトフォントサイズの確認 | src/font_size_tests.rs | 39 |
| `test_set_font_size_small` | 小サイズ（Small）設定テスト | src/font_size_tests.rs | 59 |
| `test_set_font_size_medium` | 中サイズ（Medium）設定テスト | src/font_size_tests.rs | 74 |
| `test_set_font_size_large` | 大サイズ（Large）設定テスト | src/font_size_tests.rs | 89 |
| `test_validate_font_size_preset` | プリセットサイズのバリデーション | src/font_size_tests.rs | 104 |
| `test_validate_font_size_custom_percentage` | カスタムパーセンテージのバリデーション | src/font_size_tests.rs | 120 |
| `test_invalid_font_size_custom_percentage` | 無効なカスタムパーセンテージの拒否 | src/font_size_tests.rs | 135 |
| `test_invalid_font_size_string` | 無効な文字列の拒否 | src/font_size_tests.rs | 151 |
| `test_font_size_persistence` | フォントサイズの永続化テスト | src/font_size_tests.rs | 167 |
| `test_font_size_custom_percentage_persistence` | カスタムパーセンテージの永続化 | src/font_size_tests.rs | 185 |
| `test_font_size_boundary_values` | 境界値（50%, 200%）のテスト | src/font_size_tests.rs | 203 |
| `test_font_size_overwrite` | フォントサイズの上書きテスト | src/font_size_tests.rs | 228 |
| `test_font_size_constants` | フォントサイズ定数の確認 | src/font_size_tests.rs | 250 |

**合計**: 13件

---

## インラインテスト

各機能モジュールに実装された`#[cfg(test)]`ブロックのテスト。

### validation.rs

パスワードバリデーションロジックのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_all_password_validations` | すべてのパスワードバリデーションテストを実行 | src/validation.rs | 66 |
| `test_empty_password` | 空パスワードの拒否 | src/validation.rs | 76 |
| `test_whitespace_only_password` | 空白のみのパスワードの拒否 | src/validation.rs | 85 |
| `test_password_too_short` | 短すぎるパスワード（5文字）の拒否 | src/validation.rs | 94 |
| `test_single_character_password` | 1文字パスワードの拒否 | src/validation.rs | 103 |
| `test_password_exactly_15_characters` | ちょうど15文字のパスワードの拒否 | src/validation.rs | 112 |
| `test_password_exactly_16_characters` | ちょうど16文字のパスワードの受け入れ | src/validation.rs | 123 |
| `test_password_more_than_16_characters` | 16文字以上のパスワードの受け入れ | src/validation.rs | 130 |
| `test_password_with_spaces` | スペースを含むパスワードの受け入れ | src/validation.rs | 137 |
| `test_password_with_special_characters` | 特殊文字を含むパスワードの受け入れ | src/validation.rs | 144 |
| `test_password_with_unicode` | Unicode文字を含むパスワードの受け入れ (16 BMP 日本語文字) | src/validation.rs | 236 |
| `test_multibyte_password_below_min_length_rejected` | バイト数16以上でも文字数15の日本語パスワードは拒否 (Fable-5 #9 リグレッション) | src/validation.rs | 249 |
| `test_multibyte_password_at_min_length_accepted` | 16文字日本語パスワードは Unicode scalar 境界で受け入れ | src/validation.rs | 262 |
| `test_very_long_password` | 非常に長いパスワード（128文字）の受け入れ | src/validation.rs | 158 |
| `test_password_confirmation_matching` | パスワード確認の一致テスト | src/validation.rs | 164 |
| `test_password_confirmation_not_matching` | パスワード確認の不一致テスト | src/validation.rs | 173 |
| `test_password_confirmation_case_sensitive` | パスワード確認の大文字小文字区別 | src/validation.rs | 184 |
| `test_full_validation_with_valid_passwords` | 完全バリデーション（有効） | src/validation.rs | 191 |
| `test_full_validation_with_empty_password` | 完全バリデーション（空パスワード） | src/validation.rs | 197 |
| `test_full_validation_with_short_password` | 完全バリデーション（短いパスワード） | src/validation.rs | 204 |
| `test_full_validation_with_non_matching_passwords` | 完全バリデーション（不一致） | src/validation.rs | 215 |
| `test_full_validation_error_priority` | エラー優先順位テスト | src/validation.rs | 224 |
| `test_password_with_leading_trailing_spaces` | 前後にスペースがあるパスワード | src/validation.rs | 233 |
| `test_numeric_password` | 数字のみのパスワード | src/validation.rs | 242 |
| `test_password_boundary_cases` | 境界値ケース（15, 16, 17文字） | src/validation.rs | 248 |

**合計**: 25件

### security.rs

セキュリティ機能（パスワードハッシュ化、暗号化鍵導出）のテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_hash_password` | パスワードのハッシュ化テスト | src/security.rs | 99 |
| `test_verify_password_success` | パスワード検証成功テスト | src/security.rs | 108 |
| `test_verify_password_failure` | パスワード検証失敗テスト | src/security.rs | 117 |
| `test_hash_uniqueness` | ハッシュの一意性テスト（同じパスワードで異なるハッシュ） | src/security.rs | 127 |
| `test_derive_encryption_key` | 暗号化鍵の導出テスト | src/security.rs | 141 |
| `test_derive_encryption_key_deterministic` | 暗号化鍵の決定性テスト（同じ入力で同じ鍵） | src/security.rs | 151 |
| `test_derive_encryption_key_different_passwords` | 異なるパスワードで異なる鍵を生成 | src/security.rs | 163 |
| `test_derive_encryption_key_different_salts` | 異なるsaltで異なる鍵を生成 | src/security.rs | 174 |
| `test_derive_encryption_key_short_salt` | 短いsaltでのエラーハンドリング | src/security.rs | 187 |
| `test_empty_password_hash` | 空パスワードのハッシュ化 | src/security.rs | 197 |
| `test_long_password` | 長いパスワードのハッシュ化 | src/security.rs | 203 |
| `test_unicode_password` | Unicodeパスワードのハッシュ化 | src/security.rs | 211 |
| `test_special_characters_password` | 特殊文字パスワードのハッシュ化 | src/security.rs | 219 |

**合計**: 13件

### crypto.rs

AES-256-GCM暗号化・復号化のテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_encrypt_decrypt_basic` | 基本的な暗号化・復号化 | src/crypto.rs | 120 |
| `test_encrypt_produces_different_outputs` | 同じ平文でも異なる暗号文を生成 | src/crypto.rs | 131 |
| `test_empty_string` | 空文字列の暗号化 | src/crypto.rs | 147 |
| `test_long_string` | 長い文字列の暗号化 | src/crypto.rs | 158 |
| `test_unicode_text` | Unicode文字列の暗号化 | src/crypto.rs | 169 |
| `test_special_characters` | 特殊文字の暗号化 | src/crypto.rs | 180 |
| `test_newlines_and_whitespace` | 改行・空白を含む文字列の暗号化 | src/crypto.rs | 191 |
| `test_different_keys_produce_different_results` | 異なる鍵で異なる暗号文を生成 | src/crypto.rs | 202 |
| `test_wrong_key_fails_decryption` | 間違った鍵での復号化失敗 | src/crypto.rs | 216 |
| `test_corrupted_ciphertext` | 破損した暗号文の復号化失敗 | src/crypto.rs | 230 |
| `test_invalid_base64` | 無効なBase64の復号化失敗 | src/crypto.rs | 246 |
| `test_too_short_ciphertext` | 短すぎる暗号文の復号化失敗 | src/crypto.rs | 255 |
| `test_numeric_strings` | 数値文字列の暗号化 | src/crypto.rs | 265 |
| `test_json_like_string` | JSON形式文字列の暗号化 | src/crypto.rs | 276 |
| `test_sql_like_string` | SQL形式文字列の暗号化 | src/crypto.rs | 287 |

**合計**: 15件

### db.rs

データベース初期化・マイグレーションのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_wal_mode_enabled` | WALモード有効化の確認 | src/db.rs | 187 |
| `test_transactions_detail_migration` | transactions_detailテーブルのマイグレーション | src/db.rs | 217 |
| `test_migrate_survives_orphaned_memo_reference` | 旧DBの孤児MEMO_ID参照でも migration が成功 (Fable-5 #11) | src/db.rs | 1032 |
| `test_migrate_leaves_foreign_keys_on` | migration 後に PRAGMA foreign_keys が ON に復元されている (Fable-5 #11) | src/db.rs | 1137 |
| `migrate_shops_unique_dedupes_and_repoints_references` | SHOPS の重複行を smallest SHOP_ID に集約、TRANSACTIONS_HEADER + RECURRING_RULES の参照も repoint、unique index 作成、追加 INSERT が拒否されることを end-to-end で確認 (PR15, Fable-5 #20) | src/db.rs | 1373 |
| `migrate_shops_unique_is_idempotent` | 一度成功した migration の 2 回目実行が no-op で行数を変えない (PR15, Fable-5 #20) | src/db.rs | 1424 |
| `migrate_shops_unique_scopes_per_user` | user A と user B が同じ SHOP_NAME を持つケースは重複扱いしない (constraint は per-user scope) (PR15, Fable-5 #20) | src/db.rs | 1437 |
| `migrate_shops_unique_keeps_active_row_over_soft_deleted_older_id` | 論理削除された古い店舗 (小さい SHOP_ID) と再作成された有効な同名店舗 (大きい SHOP_ID) が並存するとき、有効な行が survivor に選ばれ、旧 transaction 参照も active row に repoint される (PR15, Devin #118 review) | src/db.rs | 1459 |

**合計**: 8件

### settings.rs

設定管理機能のテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_settings_manager_creation` | SettingsManager作成テスト | src/settings.rs | 188 |
| `test_get_and_set_string` | 文字列の取得・設定 | src/settings.rs | 198 |
| `test_get_and_set_int` | 整数の取得・設定 | src/settings.rs | 214 |
| `test_get_and_set_bool` | 真偽値の取得・設定 | src/settings.rs | 227 |
| `test_save_and_reload` | 設定の保存・再読み込み | src/settings.rs | 240 |
| `test_remove_entry` | エントリの削除 | src/settings.rs | 258 |
| `test_entry_not_found` | 存在しないエントリのエラーハンドリング | src/settings.rs | 273 |
| `test_complex_type` | 複雑な型（JSON）の保存・取得 | src/settings.rs | 289 |
| `test_keys_list` | キー一覧の取得 | src/settings.rs | 315 |
| `test_save_leaves_no_tmp_sibling_and_target_is_parseable` | save 成功後に tmp ファイルが残らず、target は読み込み可能 (Fable-5 #10) | src/settings.rs | 345 |
| `test_repeated_saves_do_not_accumulate_tmp_files` | 繰り返しの save で tmp ファイルが累積しない (Fable-5 #10) | src/settings.rs | 378 |
| `test_stale_tmp_file_is_not_loaded` | クラッシュ由来の tmp が残っていても real target を優先ロード (Fable-5 #10) | src/settings.rs | 404 |

**合計**: 12件

### api_error.rs

`ApiError` — Tauri master-CRUD コマンドラッパーが `{ code, message, entity? }` として JSON シリアライズする構造化エラー型。Fable-5 レビュー #23/#D4 で導入し、フロントエンド分類器 (`res/js/master-crud.js`) が英語 message の substring 一致ではなく `err.code` で分岐できるようにした。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `duplicate_name_carries_lowercased_entity_and_stable_code` | `ApiError::duplicate_name("Shop")` で `code="duplicate_name"`、`entity="shop"` | src/api_error.rs | 128 |
| `not_found_carries_lowercased_entity_and_stable_code` | `ApiError::not_found("Manufacturer")` で `code="not_found"`、`entity="manufacturer"` | src/api_error.rs | 136 |
| `duplicate_code_carries_lowercased_entity_and_distinct_code` | `ApiError::duplicate_code("Account")` で `code="duplicate_code"` (`duplicate_name` とは区別) | src/api_error.rs | 145 |
| `admin_protected_carries_lowercased_entity_and_stable_code` | `ApiError::admin_protected("User")` で `code="admin_protected"` (user-management delete 保護用) | src/api_error.rs | 154 |
| `manufacturer_not_found_has_its_own_code` | `manufacturer_not_found` は汎用 `not_found` とは区別された専用 code | src/api_error.rs | 163 |
| `validation_carries_message_through_and_omits_entity` | `ApiError::validation(msg)` で `code="validation"`、message 貫通、entity=None | src/api_error.rs | 150 |
| `database_from_sqlx_row_not_found` | `sqlx::Error` → `ApiError::database` の `From` 変換 | src/api_error.rs | 158 |
| `serialises_with_snake_case_code_and_optional_entity` | serialize 出力に snake_case `code` と entity フィールドが含まれる | src/api_error.rs | 165 |
| `serialises_without_entity_key_when_none` | entity=None のときは JSON 出力から `entity` キー自体を省略 (`skip_serializing_if`) | src/api_error.rs | 174 |
| `in_use_carries_lowercased_entity_and_stable_code` | `ApiError::in_use("Shop")` → `code="in_use"`, `entity="shop"`（マスタ削除ロックガード） | src/api_error.rs | 230 |

**合計**: 10件

### services/master_data.rs

マスタ CRUD の共通ヘルパー (`MasterCrudSpec` + `ensure_update_affected_one` + `run_delete_expect_one`) のピュア Rust テスト (PR3, Fable-5 #26)。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `ensure_update_affected_one_maps_zero_to_not_found` | `rows_affected == 0` の UPDATE を spec の entity 付き `ApiError::not_found` にマップ | src/services/master_data.rs | 228 |
| `ensure_update_affected_one_passes_positive_count` | 正の rows_affected は Ok を返す (境界: 1 / 42) | src/services/master_data.rs | 235 |
| `reject_if_in_use_maps_positive_flag_to_in_use` | in-use フラグが正なら `ApiError::in_use(entity)` に変換（マスタ削除ロックガード） | src/services/master_data.rs | 256 |
| `reject_if_in_use_passes_when_flag_is_zero` | in-use フラグが 0 なら Ok を返す（マスタ削除ロックガード） | src/services/master_data.rs | 263 |

**合計**: 4件

### services/auth.rs

認証サービス（ユーザー登録・ログイン）のテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_register_admin_user` | 管理者ユーザー登録テスト | src/services/auth.rs | 246 |
| `test_authenticate_user_success` | 認証成功テスト | src/services/auth.rs | 264 |
| `test_authenticate_user_wrong_password` | 間違ったパスワードでの認証失敗 | src/services/auth.rs | 284 |
| `test_authenticate_user_nonexistent` | 存在しないユーザーでの認証失敗 | src/services/auth.rs | 300 |
| `test_has_users_empty` | 空DBでのユーザー存在確認 | src/services/auth.rs | 312 |
| `test_has_users_with_user` | ユーザー存在時の確認 | src/services/auth.rs | 322 |
| `test_password_is_hashed` | パスワードがハッシュ化されていることを確認 | src/services/auth.rs | 334 |
| `test_admin_role_assigned` | 管理者ロールの割り当て確認 | src/services/auth.rs | 355 |
| `test_multiple_authentication_attempts` | 複数回の認証試行 | src/services/auth.rs | 372 |
| `test_special_characters_in_credentials` | 認証情報の特殊文字テスト | src/services/auth.rs | 387 |
| `test_unicode_credentials` | 認証情報のUnicodeテスト | src/services/auth.rs | 402 |
| `test_role_constants_values` | ロール定数の値確認 | src/services/auth.rs | 417 |
| `test_role_constants_uniqueness` | ロール定数の一意性確認 | src/services/auth.rs | 425 |
| `invalid_credentials_maps_to_auth_invalid_credentials_code` | `AuthError::InvalidCredentials` → `ApiError { code: "auth_invalid_credentials" }` (PR14, Fable-5 #21) | src/services/auth.rs | 577 |
| `database_error_maps_to_database_code` | `AuthError::DatabaseError` → `ApiError { code: "database" }` (PR14, Fable-5 #21) | src/services/auth.rs | 585 |
| `security_error_maps_to_validation_code_with_message` | `AuthError::SecurityError` → `ApiError { code: "validation" }` で message 保持 (PR14, Fable-5 #21) | src/services/auth.rs | 593 |

**合計**: 16件

### services/user_management.rs

ユーザー管理サービス（CRUD操作）のテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_register_general_user` | 一般ユーザー登録テスト | src/services/user_management.rs | 502 |
| `test_update_general_user` | 一般ユーザー更新テスト | src/services/user_management.rs | 519 |
| `test_update_general_user_username_only` | ユーザー名のみ更新 | src/services/user_management.rs | 547 |
| `test_update_general_user_password_only` | `_with_password` 経由でパスワードのみ更新 (Fable-5 #1/#5) | src/services/user_management.rs | 568 |
| `test_update_general_user_username_and_password` | `_with_password` 経由でユーザー名とパスワードを 1 tx で更新 (Fable-5 #1/#5) | src/services/user_management.rs | 601 |
| `test_update_admin_user` | 管理者ユーザー更新テスト | src/services/user_management.rs | 634 |
| `test_update_admin_user_username_only` | 管理者のユーザー名のみ更新 | src/services/user_management.rs | 650 |
| `test_update_admin_user_password_only` | `_with_password` 経由で管理者のパスワードのみ更新 (Fable-5 #1/#5) | src/services/user_management.rs | 668 |
| `test_update_admin_user_username_and_password` | 管理者のユーザー名とパスワードを 1 tx で更新 (Fable-5 #1/#5) | src/services/user_management.rs | 698 |
| `test_delete_general_user` | 一般ユーザー削除テスト | src/services/user_management.rs | 728 |
| `test_cannot_delete_admin_user` | 管理者ユーザー削除の防止 | src/services/user_management.rs | 744 |
| `test_duplicate_username` | 重複ユーザー名のエラー | src/services/user_management.rs | 755 |
| `test_list_users` | ユーザー一覧取得テスト | src/services/user_management.rs | 769 |
| `test_register_general_user_accepts_max_chars_of_multibyte_name` | USERS.NAME 長制約は文字数 (byte 数ではない) — MAX_NAME_LEN 分の多バイト文字を受理 (issue #37) | src/services/user_management.rs | 785 |
| `test_register_general_user_rejects_over_max_chars_of_multibyte_name` | 登録時に MAX_NAME_LEN+1 の多バイト文字を拒否 (issue #37) | src/services/user_management.rs | 796 |
| `test_update_general_user_with_password_rejects_wrong_old_password` | 現在パスワード誤り → `OldPasswordIncorrect`。ハッシュ・ユーザー名とも未変更 (Fable-5 #1/#5) | src/services/user_management.rs | 817 |
| `test_update_general_user_with_password_rename_only_rejects_wrong_old_password` | 改名専用分岐でも `OldPasswordIncorrect` に統一 (CodeRabbit on #123) | src/services/user_management.rs | 864 |
| `test_update_admin_user_with_password_rejects_wrong_old_password` | 管理者版: 現在パスワード誤り → `OldPasswordIncorrect`。ハッシュ未変更 (Fable-5 #1/#5) | src/services/user_management.rs | 891 |
| `test_update_general_user_rejects_over_max_chars_of_multibyte_name` | 改名時に MAX_NAME_LEN+1 の多バイト文字を拒否 (issue #37) | src/services/user_management.rs | 920 |

**合計**: 19件

### services/encryption.rs

暗号化サービス（フィールド暗号化・再暗号化）のテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_register_encrypted_field` | 暗号化フィールド登録テスト | src/services/encryption.rs | 285 |
| `test_encrypt_decrypt_field` | フィールドの暗号化・復号化テスト | src/services/encryption.rs | 304 |
| `test_re_encrypt_user_data` | ユーザーデータの再暗号化テスト | src/services/encryption.rs | 326 |
| `test_decrypt_with_wrong_password_fails` | 間違ったパスワードでの復号化失敗 | src/services/encryption.rs | 380 |
| `test_re_encrypt_user_data_preserves_per_row_plaintext` | 複数行の再暗号化で各行の平文が保持される (Fable-5 #14) | src/services/encryption.rs | 473 |
| `test_encrypt_uses_per_user_salt_not_user_id` | 同じ password/plaintext でもユーザーごとに ciphertext が異なる (Fable-5 #15) | src/services/encryption.rs | 657 |
| `test_encrypt_decrypt_salt_survives_service_reconstruction` | salt を DB から再取得するため、新しい service インスタンスで round-trip が成立 (Fable-5 #15) | src/services/encryption.rs | 703 |
| `test_encrypt_errors_when_user_missing` | USERS 行が無い場合は user_id 由来 salt に fallback せずエラー (Fable-5 #15) | src/services/encryption.rs | 722 |

**合計**: 8件

### services/account.rs

口座管理サービスのテスト。empty-name/duplicate-code 系の assertion は `ApiError { code: "validation" | "duplicate_code" }` に移行 (Fable-5 #23); 挙動は不変。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_add_account_rejects_empty_name` | 口座名が空のとき `ApiError { code: "validation" }` (Fable-5 #16, #23) | src/services/account.rs | 597 |
| `test_add_account_rejects_whitespace_only_name` | 空白のみの口座名は `ApiError { code: "validation" }` (Fable-5 #16, #23) | src/services/account.rs | 612 |
| `test_update_account_rejects_empty_name` | 更新で口座名が空のとき `ApiError { code: "validation" }` (Fable-5 #16, #23) | src/services/account.rs | 627 |
| `test_update_account_not_found_has_stable_code_and_entity` | 存在しない口座の更新は `ApiError { code: "not_found", entity: "account" }` (Fable-5 #23) | src/services/account.rs | 815 |
| `test_delete_account_not_found_has_stable_code_and_entity` | 存在しない口座の削除は `ApiError { code: "not_found" }` (Fable-5 #23) | src/services/account.rs | 837 |
| `test_delete_account_rejected_when_referenced_as_from_account` | TRANSACTIONS_HEADER が FROM 側で参照中なら `ApiError { code: "in_use" }` で削除拒否（マスタ削除ロック） | src/services/account.rs | 827 |
| `test_delete_account_rejected_when_referenced_as_to_account` | TRANSACTIONS_HEADER が TO 側で参照中なら `ApiError { code: "in_use" }` で削除拒否（マスタ削除ロック） | src/services/account.rs | 847 |
| `test_delete_account_rejected_when_referenced_by_recurring_rule` | RECURRING_RULES が参照中なら `ApiError { code: "in_use" }` で削除拒否（マスタ削除ロック） | src/services/account.rs | 864 |
| `test_delete_account_ignores_other_users_references` | 他ユーザーの同一 ACCOUNT_CODE 参照は削除をブロックしない（コードはユーザースコープ、マスタ削除ロック） | src/services/account.rs | 881 |
| `test_delete_account_normalizes_input_before_in_use_check` | `"  cash  "` 入力は正規化されてから CHECK_IN_USE に流れ、ガードが発火する（マスタ削除ロック） | src/services/account.rs | 899 |

**合計**: 10件

### services/category.rs

カテゴリ管理サービス（3階層カテゴリのCRUD）のテスト。Tauri wrapper 境界で内部の `CategoryError` を `From<CategoryError>` により `ApiError { code, message, entity? }` へマッピング (Fable-5 #23)。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_populate_default_categories` | デフォルトカテゴリの登録 | src/services/category.rs | 1087 |
| `test_get_category1_list` | 大カテゴリ一覧取得 | src/services/category.rs | 1155 |
| `test_add_category2` | 中カテゴリ追加 | src/services/category.rs | 1198 |
| `test_add_category2_duplicate_name` | 中カテゴリの重複名エラー | src/services/category.rs | 1234 |
| `test_add_category3` | 小カテゴリ追加 | src/services/category.rs | 1267 |
| `test_add_category3_duplicate_name` | 小カテゴリの重複名エラー | src/services/category.rs | 1301 |
| `test_move_category2_order` | 中カテゴリの表示順変更 | src/services/category.rs | 1342 |
| `test_move_category3_order` | 小カテゴリの表示順変更 | src/services/category.rs | 1426 |
| `test_update_category2` | 中カテゴリ更新 | src/services/category.rs | 1503 |
| `test_update_category3` | 小カテゴリ更新 | src/services/category.rs | 1527 |
| `test_update_category2_duplicate_name` | 中カテゴリの重複名更新エラー | src/services/category.rs | 1552 |
| `test_move_category2_boundary` | 中カテゴリの境界値移動テスト | src/services/category.rs | 1571 |
| `test_get_category_for_edit` | 編集用カテゴリ情報取得 | src/services/category.rs | 1623 |
| `test_get_category2_for_edit_returns_not_found_for_missing` | 消失した中カテゴリ編集取得は NotFound を返す (Fable-5 #6) | src/services/category.rs | 1873 |
| `test_get_category3_for_edit_returns_not_found_for_missing` | 消失した小カテゴリ編集取得は NotFound を返す (Fable-5 #6) | src/services/category.rs | 1884 |
| `test_disable_category2_returns_not_found_for_missing` | 消失した中カテゴリ論理削除は NotFound を返す (Fable-5 #7) | src/services/category.rs | 1899 |
| `test_disable_category3_returns_not_found_for_missing` | 消失した小カテゴリ論理削除は NotFound を返す (Fable-5 #7) | src/services/category.rs | 1910 |
| `test_disable_category2_succeeds_with_no_children` | 子カテゴリなしの中カテゴリ論理削除は成功する（子スイープは0件許容） | src/services/category.rs | 1926 |
| `not_found_maps_to_not_found_code_with_category_entity` | `CategoryError::NotFound` → `ApiError { code: "not_found", entity: "category" }` (Fable-5 #23) | src/services/category.rs | 2101 |
| `duplicate_name_maps_to_duplicate_name_code_with_category_entity` | `CategoryError::DuplicateName(_)` → `ApiError { code: "duplicate_name", entity: "category" }` (Fable-5 #23) | src/services/category.rs | 2108 |
| `validation_preserves_message_and_omits_entity` | `CategoryError::Validation(msg)` → `ApiError { code: "validation" }` で message を保持 (Fable-5 #23) | src/services/category.rs | 2115 |
| `database_error_maps_to_database_code` | `CategoryError::DatabaseError(_)` → `ApiError { code: "database" }` (Fable-5 #23) | src/services/category.rs | 2125 |
| `test_get_category_tree_groups_children_under_parent` | 3-flat-queries + HashMap grouping で cat1→cat2→cat3 の親子関係が正しく組み上がる regression pin (PR11, Fable-5 #31) | src/services/category.rs | 2022 |
| `test_get_category_tree_preserves_display_order` | move_category2_up で並び替えた cat2 の DISPLAY_ORDER が flat-query grouping 後も維持されること (PR11, Fable-5 #31) | src/services/category.rs | 2077 |
| `test_get_category_tree_all_includes_disabled_flags` | `get_category_tree_all` は disabled 行を含め `is_disabled` フィールド付きで返す (PR11, Fable-5 #31)。反面 `get_category_tree` は disabled 行を除外する対比も同時にチェック | src/services/category.rs | 2106 |

**合計**: 25件

### services/manufacturer.rs

メーカー管理サービスのテスト。empty/duplicate 系のテスト名は `ApiError` 移行 (Fable-5 #23) に合わせ `_returns_validation_code` / `_returns_duplicate_name_code` にリネーム — 挙動は変わらず assertion の対象のみ変更。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_add_manufacturer` | メーカー追加テスト | src/services/manufacturer.rs | 243 |
| `test_update_manufacturer` | メーカー更新テスト | src/services/manufacturer.rs | 261 |
| `test_delete_manufacturer` | メーカー削除テスト | src/services/manufacturer.rs | 292 |
| `test_empty_manufacturer_name_returns_validation_code` | 空メーカー名は `ApiError { code: "validation" }` (Fable-5 #23) | src/services/manufacturer.rs | 316 |
| `test_add_duplicate_manufacturer_returns_duplicate_name_code` | 重複は `ApiError { code: "duplicate_name", entity: "manufacturer" }` (Fable-5 #23) | src/services/manufacturer.rs | 331 |
| `test_update_to_duplicate_manufacturer_name_returns_duplicate_name_code` | 重複への更新は `ApiError { code: "duplicate_name" }` (Fable-5 #23) | src/services/manufacturer.rs | 355 |
| `test_update_missing_manufacturer_returns_not_found_code` | 存在しないメーカーの更新は `ApiError { code: "not_found", entity: "manufacturer" }` (Fable-5 #23) | src/services/manufacturer.rs | 383 |
| `test_delete_missing_manufacturer_returns_not_found_code` | 存在しないメーカーの削除は `ApiError { code: "not_found" }` (Fable-5 #23) | src/services/manufacturer.rs | 398 |
| `test_update_same_manufacturer_name` | 同じ名前への更新（許可） | src/services/manufacturer.rs | 405 |
| `test_delete_manufacturer_rejected_when_referenced_by_product` | PRODUCTS がメーカーを参照中なら `ApiError { code: "in_use", entity: "manufacturer" }` で削除拒否（マスタ削除ロック） | src/services/manufacturer.rs | 374 |
| `test_delete_manufacturer_rejected_when_only_disabled_products_reference` | IS_DISABLED=1 の商品でも参照とみなす（FK は残り、「無効表示」でも一覧に出るため、マスタ削除ロック） | src/services/manufacturer.rs | 402 |
| `test_delete_manufacturer_ignores_other_users_references` | 他ユーザーの同一 MANUFACTURER_ID 参照は削除をブロックしない（USER_ID スコープ、マスタ削除ロック） | src/services/manufacturer.rs | 429 |

**合計**: 12件

### services/product.rs

商品管理サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_add_product_without_manufacturer` | メーカーなしの商品追加 | src/services/product.rs | 256 |
| `test_add_product_with_manufacturer` | メーカーありの商品追加 | src/services/product.rs | 276 |
| `test_update_product` | 商品更新テスト | src/services/product.rs | 309 |
| `test_delete_product` | 商品削除テスト | src/services/product.rs | 342 |
| `test_empty_product_name` | 空商品名のエラー | src/services/product.rs | 367 |
| `test_add_duplicate_product` | 重複商品名のエラー | src/services/product.rs | 383 |
| `test_manufacturer_deletion_rejected_while_product_references_it` | 商品が参照中はメーカー削除が `ApiError { code: "in_use", entity: "manufacturer" }` で拒否される — マスタ削除ロック導入で `test_manufacturer_deletion_sets_product_manufacturer_to_null` からリネーム（旧: 論理削除→CASCADE NULL の fallback） | src/services/product.rs | 512 |
| `test_add_product_rejects_foreign_manufacturer_id` | 他ユーザーの manufacturer_id で add は "Manufacturer not found" (Fable-5 #13) | src/services/product.rs | 716 |
| `test_add_product_rejects_nonexistent_manufacturer_id` | 存在しない manufacturer_id で add は "Manufacturer not found" (Fable-5 #13) | src/services/product.rs | 767 |
| `test_update_product_rejects_foreign_manufacturer_id` | 他ユーザーの manufacturer_id で update は "Manufacturer not found" (Fable-5 #13) | src/services/product.rs | 792 |
| `test_product_join_scopes_manufacturer_by_user_id` | PRODUCT_GET_* JOIN は他ユーザーの manufacturer 名を漏らさない (Fable-5 #13) | src/services/product.rs | 871 |
| `test_delete_product_rejected_when_referenced_by_transaction_detail` | TRANSACTIONS_DETAIL が商品を参照中なら `ApiError { code: "in_use", entity: "product" }` で削除拒否（TRANSACTIONS_HEADER.USER_ID 経由でスコープ、マスタ削除ロック） | src/services/product.rs | 444 |
| `test_delete_product_ignores_other_users_transaction_details` | 他ユーザーの明細参照は削除をブロックしない（TRANSACTIONS_HEADER.USER_ID でスコープ、マスタ削除ロック） | src/services/product.rs | 481 |

**合計**: 13件

### services/shop.rs

店舗管理サービスのテスト。empty/duplicate 系のテスト名は `ApiError` 移行 (Fable-5 #23) に合わせ `_returns_validation_code` / `_returns_duplicate_name_code` にリネーム — 挙動は変わらず assertion の対象のみ変更。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_add_shop` | 店舗追加テスト | src/services/shop.rs | 232 |
| `test_update_shop` | 店舗更新テスト | src/services/shop.rs | 249 |
| `test_delete_shop` | 店舗削除テスト | src/services/shop.rs | 278 |
| `test_empty_shop_name_returns_validation_code` | 空店舗名は `ApiError { code: "validation" }` (Fable-5 #23) | src/services/shop.rs | 301 |
| `test_add_duplicate_shop_returns_duplicate_name_code` | 重複は `ApiError { code: "duplicate_name", entity: "shop" }` (Fable-5 #23) | src/services/shop.rs | 315 |
| `test_update_to_duplicate_shop_name_returns_duplicate_name_code` | 重複への更新は `ApiError { code: "duplicate_name" }` (Fable-5 #23) | src/services/shop.rs | 337 |
| `test_update_missing_shop_returns_not_found_code` | 存在しない店舗の更新は `ApiError { code: "not_found", entity: "shop" }` (Fable-5 #23) | src/services/shop.rs | 363 |
| `test_delete_missing_shop_returns_not_found_code` | 存在しない店舗の削除は `ApiError { code: "not_found" }` (Fable-5 #23) | src/services/shop.rs | 375 |
| `test_update_same_shop_name` | 同じ名前への更新（許可） | src/services/shop.rs | 382 |
| `test_delete_shop_rejected_when_referenced_by_transaction` | TRANSACTIONS_HEADER が店舗を参照中なら `ApiError { code: "in_use", entity: "shop" }` で削除拒否（マスタ削除ロック） | src/services/shop.rs | 346 |
| `test_delete_shop_rejected_when_referenced_by_recurring_rule` | RECURRING_RULES が店舗を参照中なら `ApiError { code: "in_use" }` で削除拒否（マスタ削除ロック） | src/services/shop.rs | 372 |
| `test_delete_shop_ignores_other_users_references` | 他ユーザーの同一 SHOP_ID 参照は削除をブロックしない（USER_ID スコープ、マスタ削除ロック） | src/services/shop.rs | 394 |

**合計**: 12件

### services/transaction.rs

取引管理サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_save_transaction_header_with_tax_excluded` | 税抜取引ヘッダー保存 | src/services/transaction.rs | 1246 |
| `test_save_transaction_header_with_tax_included` | 税込取引ヘッダー保存 | src/services/transaction.rs | 1278 |
| `test_update_transaction_header_tax_type` | 取引ヘッダーの税種別更新 | src/services/transaction.rs | 1309 |
| `test_default_tax_type_is_excluded` | デフォルト税種別が税抜であることを確認 | src/services/transaction.rs | 1351 |
| `test_tax_type_validation_values` | 税種別の有効値確認 | src/services/transaction.rs | 1375 |
| `test_get_transactions_end_date_includes_boundary_day` | 終了日フィルタが同日タイムスタンプを含むこと (bare 'YYYY-MM-DD' を 23:59:59 に正規化) | src/services/transaction.rs | 3293 |
| `test_get_transactions_keyword_matches_header_and_detail_memo` | キーワードがヘッダー/明細のメモテキストで部分一致すること | src/services/transaction.rs | 3364 |
| `test_update_detail_memo_does_not_corrupt_shared_header_memo` | 明細メモ編集が MEMO_ID を共有するヘッダーメモを破壊しないこと | src/services/transaction.rs | 3584 |
| `test_delete_detail_preserves_memo_still_referenced_by_header` | ヘッダーが参照中の memo は明細削除で残ること | src/services/transaction.rs | 3618 |
| `test_update_detail_memo_updates_in_place_when_not_shared` | 単独参照メモは in-place update のままであること | src/services/transaction.rs | 3644 |
| `test_delete_detail_removes_orphaned_memo` | 単独参照メモは明細削除で MEMOS 行も削除されること | src/services/transaction.rs | 3674 |
| `test_clear_detail_memo_does_not_delete_memo_still_used_by_header` | 共有中の明細メモをクリアしてもヘッダー側の memo 行が残ること | src/services/transaction.rs | 3706 |
| `test_update_detail_memo_does_not_corrupt_recurring_rule_memo` | 明細メモ編集が繰り返しルールと共有するメモを破壊しないこと | src/services/transaction.rs | 3760 |
| `test_delete_detail_preserves_memo_still_referenced_by_recurring_rule` | 繰り返しルールが参照中の memo は明細削除で残ること | src/services/transaction.rs | 3805 |
| `test_clear_detail_memo_succeeds_under_foreign_keys_on` | 明細メモのクリアが MEMOS 外部キーに違反しないこと | src/services/transaction.rs | 3843 |
| `test_add_detail_rejects_foreign_transaction_id` | 他ユーザーの transaction_id で明細追加は NotFound を返す (Fable-5 #12) | src/services/transaction.rs | 4109 |
| `test_add_detail_rejects_nonexistent_transaction_id` | 存在しない transaction_id で明細追加は NotFound を返す (Fable-5 #12) | src/services/transaction.rs | 4142 |
| `not_found_maps_to_not_found_code_with_transaction_entity` | TransactionError::NotFound が ApiError::not_found("transaction") にマッピングされること (PR2b) | src/services/transaction.rs | 4199 |
| `validation_preserves_message_and_omits_entity` | TransactionError::ValidationError が ApiError::CODE_VALIDATION に変換され、メッセージが保持されること (PR2b) | src/services/transaction.rs | 4206 |
| `database_error_maps_to_database_code` | TransactionError::DatabaseError が ApiError::CODE_DATABASE に変換されること (PR2b) | src/services/transaction.rs | 4217 |
| `field_needle_message_survives_conversion_for_frontend_routing` | 2 つのフィールド needle (`"Item name must be"` / `"Memo must be"`) が変換後もそのまま先頭に残り、フロントの `startsWith` ルーティングを維持できること (PR2b) | src/services/transaction.rs | 4224 |
| `test_find_matching_pattern_preserves_user_half_up_when_settings_match` | 端数なしの伝票 (500円 × 10% = 550円) で `HALF_UP + EXCLUDED` を保存している場合、一括再計算で FLOOR に無言で書き換えられないこと (Fable-5 #2) | src/services/transaction.rs | 1802 |
| `test_find_matching_pattern_preserves_user_ceil_when_settings_match` | `UP + EXCLUDED` にも同じ保証 (Fable-5 #2) | src/services/transaction.rs | 1819 |
| `test_find_matching_pattern_falls_back_to_priority_when_preferred_mismatches` | 現在設定で `target_total` を再現できない場合、優先順 PATTERNS 探索へフォールバック (Fable-5 #2) | src/services/transaction.rs | 1836 |
| `test_find_matching_pattern_returns_none_when_no_pattern_fits` | どの組み合わせも `target_total` を再現できない場合は `None`、呼び出し側は設定列でなく TOTAL_AMOUNT を上書き (Fable-5 #2) | src/services/transaction.rs | 1859 |
| `test_save_header_rejects_invalid_tax_included_type` | `save_transaction_header` が `{TAX_INCLUDED, TAX_EXCLUDED}` 以外の `tax_included_type` を拒否し、無効値が `find_matching_pattern` の「優先設定を先に確認する判定」に流れて残らないこと (#125 の CodeRabbit 指摘) | src/services/transaction.rs | 3157 |
| `test_update_header_rejects_invalid_tax_included_type` | 更新入口にも同じガード (#125 の CodeRabbit 指摘) | src/services/transaction.rs | 3184 |

**合計**: 27件

### services/aggregation.rs

集計サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_monthly_aggregation_current_month` | 当月の月次集計 | src/services/aggregation.rs | 1554 |
| `test_monthly_aggregation_next_month` | 翌月の月次集計 | src/services/aggregation.rs | 1563 |
| `test_detail_query_grosses_up_null_tax_included_row` | TAX_RATE>0 で AMOUNT_INCLUDING_TAX が NULL の明細も税抜として割増 (Fable-5 #3) | src/services/aggregation.rs | 2581 |
| `test_detail_query_grosses_up_zero_tax_included_row` | AMOUNT_INCLUDING_TAX=0 (フロント空欄) も税抜扱い (Fable-5 #3) | src/services/aggregation.rs | 2610 |
| `test_detail_query_included_header_legacy_null_row_no_double_taxation` | 税込ヘッダー (`TAX_INCLUDED_TYPE = TAX_INCLUDED (0)`) + レガシー `AMOUNT_INCLUDING_TAX = NULL` 明細を「税込み済み」として扱い、二重課税しない (Fable-5 #3 残) | src/services/aggregation.rs | 2653 |
| `test_detail_query_included_header_zero_col_no_double_taxation` | 同じ #3 残: 税込ヘッダー配下で `AMOUNT_INCLUDING_TAX = 0` (フロント空欄) も「税込み済み」扱い | src/services/aggregation.rs | 2690 |
| `test_detail_query_matches_header_query_for_included_ledger` | 税込ヘッダーの同一伝票でヘッダー集計と明細集計の値が一致する (Fable-5 #4) | src/services/aggregation.rs | 2726 |
| `test_detail_query_avg_matches_total_over_count_with_mixed_rates` | 混在税率取引で avg × count == total を保持 (Fable-5 #4) | src/services/aggregation.rs | 2774 |
| `test_detail_query_avg_multi_transaction_arithmetic` | 2 取引の avg = total / txn_count 検証 (Fable-5 #4) | src/services/aggregation.rs | 2811 |
| `test_detail_query_binds_category_filter_no_injection` | カテゴリフィルタの値が bind されている (SQL 直埋めではない) ことを End-to-End で確認。`EXPENSE' OR '1'='1` payload は 0 rows を返す (PR5, Fable-5 #25) | src/services/aggregation.rs | 2846 |
| `test_category_filter_category2_targets_detail_column` | Category2 フィルタが `td.CATEGORY2_CODE` (detail-scope、実在する列) を参照し、`th.CATEGORY2_CODE` (存在しない列) を参照しないこと (PR6, Fable-5 #17) | src/services/aggregation.rs | 2902 |
| `test_category_filter_category3_targets_detail_column` | Category3 フィルタが `td.CATEGORY2/3_CODE` を参照すること (PR6, Fable-5 #17) | src/services/aggregation.rs | 2918 |
| `test_account_query_applies_category_filter_to_all_union_branches` | 口座別集計の 4-branch UNION ALL 全てにカテゴリフィルタが適用され、bind vec に 4 回登場することを確認 (PR6, Fable-5 #18: silent drop の regression pin) | src/services/aggregation.rs | 2943 |

**合計**: 13件

### services/session.rs

セッション管理サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_session_state_initialization` | セッション状態の初期化 | src/services/session.rs | 92 |
| `test_set_and_get_user` | ユーザー情報の設定・取得 | src/services/session.rs | 101 |
| `test_clear_user` | ユーザー情報のクリア | src/services/session.rs | 119 |
| `test_set_and_get_source_screen` | ソース画面の設定・取得 | src/services/session.rs | 136 |
| `test_clear_source_screen` | ソース画面のクリア | src/services/session.rs | 144 |
| `test_set_and_get_category1_code` | カテゴリ1コードの設定・取得 | src/services/session.rs | 155 |
| `test_clear_category1_code` | カテゴリ1コードのクリア | src/services/session.rs | 163 |
| `test_clear_all` | すべてのセッション情報のクリア | src/services/session.rs | 174 |
| `test_multiple_session_operations` | 複数のセッション操作 | src/services/session.rs | 199 |

**合計**: 9件

### services/i18n.rs

国際化（i18n）サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_get_resource` | リソース取得テスト | src/services/i18n.rs | 220 |
| `test_get_with_params` | パラメータ付きリソース取得 | src/services/i18n.rs | 232 |
| `test_fallback_to_default` | デフォルト言語へのフォールバック | src/services/i18n.rs | 241 |
| `test_get_by_category` | カテゴリ別リソース取得 | src/services/i18n.rs | 251 |
| `test_error_messages_exist` | エラーメッセージの存在確認 | src/services/i18n.rs | 261 |
| `test_language_and_font_error_messages_exist` | 言語・フォント関連エラーメッセージの存在確認 | src/services/i18n.rs | 285 |
| `test_validation_messages_exist` | バリデーションメッセージの存在確認 | src/services/i18n.rs | 307 |
| `test_all_error_messages_have_both_languages` | すべてのエラーメッセージが日英両方存在することを確認 | src/services/i18n.rs | 322 |

**合計**: 8件

### services/recurring.rs

繰り返し予定入出金ルールサービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_delete_rule_returns_not_found_for_missing` | 消失したルールの削除は空コミット偽成功でなく NotFound を返す (Fable-5 #8) | src/services/recurring.rs | 1811 |
| `not_found_maps_to_not_found_code_with_recurring_rule_entity` | RecurringError::NotFound が ApiError::not_found("recurring rule") にマッピングされること (PR2a) | src/services/recurring.rs | 1833 |
| `validation_preserves_message_and_omits_entity` | RecurringError::Validation が ApiError::CODE_VALIDATION に変換され、メッセージが保持されること (PR2a) | src/services/recurring.rs | 1840 |
| `database_error_maps_to_database_code` | RecurringError::Database が ApiError::CODE_DATABASE に変換されること (PR2a) | src/services/recurring.rs | 1851 |
| `field_needle_message_survives_conversion_for_frontend_routing` | 4 つのフィールド needle (`"Rule name must be"` 等) が変換後もそのまま先頭に残り、フロントの `startsWith` ルーティングを維持できること (PR2a) | src/services/recurring.rs | 1858 |

**合計**: 5件

### lib.rs

`set_language` / `set_font_size` / `update_user_settings` コマンドが使う設定値バリデーションのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `normalize_language_accepts_names_and_codes` | 言語名・言語コード（en/English/ja/日本語/Japanese）を受理 | src/lib.rs | 2733 |
| `normalize_language_rejects_unknown_values` | 未知の言語値を拒否 | src/lib.rs | 2742 |
| `normalize_font_size_accepts_keywords_and_percentages` | サイズキーワードと50〜200%の指定を受理 | src/lib.rs | 2748 |
| `normalize_font_size_rejects_out_of_range_and_garbage` | 範囲外の割合と不正文字列を拒否 | src/lib.rs | 2757 |
| `monthly_bounds_with_shift_rejects_out_of_range_month` | month=0/13/100 で `services::period::end_of_month` に到達する前に early-Err を返し、バックエンドスレッド crash を防ぐ (PR6, Fable-5 #22) | src/lib.rs | 2695 |
| `monthly_bounds_with_shift_accepts_boundary_months` | month=1/12 の境界は引き続き受理されることを確認 (PR6, Fable-5 #22) | src/lib.rs | 2722 |

**合計**: 6件

---

## テスト統計サマリー

| カテゴリ | テスト数 |
|---------|---------|
| **共通テストスイート** | **23件** |
| validation_tests.rs | 10 |
| font_size_tests.rs | 13 |
| **インラインテスト** | **298件** |
| validation.rs | 25 |
| security.rs | 13 |
| crypto.rs | 15 |
| db.rs | 8 |
| settings.rs | 12 |
| api_error.rs | 10 |
| services/master_data.rs | 4 |
| services/auth.rs | 16 |
| services/user_management.rs | 19 |
| services/encryption.rs | 8 |
| services/account.rs | 10 |
| services/category.rs | 25 |
| services/manufacturer.rs | 12 |
| services/product.rs | 13 |
| services/shop.rs | 12 |
| services/transaction.rs | 27 |
| services/aggregation.rs | 13 |
| services/session.rs | 9 |
| services/i18n.rs | 8 |
| services/recurring.rs | 5 |
| lib.rs | 6 |
| **総計** | **321件** |

---

## テストの実行方法

### すべてのテストを実行

```bash
cargo test
```

### 特定のモジュールのみ実行

```bash
# 共通テストスイート
cargo test validation_tests::
cargo test font_size_tests::

# インラインテスト
cargo test validation::
cargo test security::
cargo test services::auth::
cargo test services::user_management::
```

### 特定のテスト関数のみ実行

```bash
cargo test test_empty_passwords
cargo test test_register_admin_user
```

### 出力付きで実行

```bash
cargo test -- --nocapture
```

### カバレッジレポート生成

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

---

## 関連ドキュメント

- [フロントエンドテストインデックス](FRONTEND_TEST_INDEX.md) - JavaScriptテストの完全一覧
- [テスト概要](TEST_OVERVIEW.md) - テスト戦略と実行ガイド
- [テスト設計](TEST_DESIGN.md) - テストアーキテクチャと設計思想
- [テスト結果](TEST_RESULTS.md) - 最新のテスト実行結果
