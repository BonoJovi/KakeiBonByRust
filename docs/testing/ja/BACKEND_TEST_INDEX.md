# バックエンドテストインデックス

このドキュメントは、Rustで実装されたバックエンドテストの完全なインデックスです。

**最終更新**: 2025-12-06 06:24 JST  
**総テスト数**: 201件

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
  - [services/category.rs](#servicescategoryrs)
  - [services/manufacturer.rs](#servicesmanufacturerrs)
  - [services/product.rs](#servicesproductrs)
  - [services/shop.rs](#servicesshoprs)
  - [services/transaction.rs](#servicestransactionrs)
  - [services/aggregation.rs](#servicesaggregationrs)
  - [services/session.rs](#servicessessionrs)
  - [services/i18n.rs](#servicesi18nrs)

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
| `test_password_with_unicode` | Unicode文字を含むパスワードの受け入れ | src/validation.rs | 151 |
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

**合計**: 23件

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

**合計**: 2件

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

**合計**: 9件

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

**合計**: 13件

### services/user_management.rs

ユーザー管理サービス（CRUD操作）のテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_register_general_user` | 一般ユーザー登録テスト | src/services/user_management.rs | 354 |
| `test_update_general_user` | 一般ユーザー更新テスト | src/services/user_management.rs | 371 |
| `test_update_general_user_username_only` | ユーザー名のみ更新 | src/services/user_management.rs | 396 |
| `test_update_general_user_password_only` | パスワードのみ更新 | src/services/user_management.rs | 417 |
| `test_update_general_user_username_and_password` | ユーザー名とパスワード両方更新 | src/services/user_management.rs | 447 |
| `test_update_admin_user` | 管理者ユーザー更新テスト | src/services/user_management.rs | 477 |
| `test_update_admin_user_username_only` | 管理者のユーザー名のみ更新 | src/services/user_management.rs | 493 |
| `test_update_admin_user_password_only` | 管理者のパスワードのみ更新 | src/services/user_management.rs | 511 |
| `test_update_admin_user_username_and_password` | 管理者のユーザー名とパスワード両方更新 | src/services/user_management.rs | 538 |
| `test_delete_general_user` | 一般ユーザー削除テスト | src/services/user_management.rs | 565 |
| `test_cannot_delete_admin_user` | 管理者ユーザー削除の防止 | src/services/user_management.rs | 581 |
| `test_duplicate_username` | 重複ユーザー名のエラー | src/services/user_management.rs | 592 |
| `test_list_users` | ユーザー一覧取得テスト | src/services/user_management.rs | 606 |

**合計**: 13件

### services/encryption.rs

暗号化サービス（フィールド暗号化・再暗号化）のテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_register_encrypted_field` | 暗号化フィールド登録テスト | src/services/encryption.rs | 285 |
| `test_encrypt_decrypt_field` | フィールドの暗号化・復号化テスト | src/services/encryption.rs | 304 |
| `test_re_encrypt_user_data` | ユーザーデータの再暗号化テスト | src/services/encryption.rs | 326 |
| `test_decrypt_with_wrong_password_fails` | 間違ったパスワードでの復号化失敗 | src/services/encryption.rs | 380 |

**合計**: 4件

### services/category.rs

カテゴリ管理サービス（3階層カテゴリのCRUD）のテスト。

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

**合計**: 13件

### services/manufacturer.rs

メーカー管理サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_add_manufacturer` | メーカー追加テスト | src/services/manufacturer.rs | 243 |
| `test_update_manufacturer` | メーカー更新テスト | src/services/manufacturer.rs | 261 |
| `test_delete_manufacturer` | メーカー削除テスト | src/services/manufacturer.rs | 292 |
| `test_empty_manufacturer_name` | 空メーカー名のエラー | src/services/manufacturer.rs | 316 |
| `test_add_duplicate_manufacturer` | 重複メーカー名のエラー | src/services/manufacturer.rs | 331 |
| `test_update_to_duplicate_manufacturer_name` | 重複名への更新エラー | src/services/manufacturer.rs | 355 |
| `test_update_same_manufacturer_name` | 同じ名前への更新（許可） | src/services/manufacturer.rs | 389 |

**合計**: 7件

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
| `test_manufacturer_deletion_sets_product_manufacturer_to_null` | メーカー削除時の商品への影響（CASCADE NULL） | src/services/product.rs | 409 |

**合計**: 7件

### services/shop.rs

店舗管理サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_add_shop` | 店舗追加テスト | src/services/shop.rs | 232 |
| `test_update_shop` | 店舗更新テスト | src/services/shop.rs | 249 |
| `test_delete_shop` | 店舗削除テスト | src/services/shop.rs | 278 |
| `test_empty_shop_name` | 空店舗名のエラー | src/services/shop.rs | 301 |
| `test_add_duplicate_shop` | 重複店舗名のエラー | src/services/shop.rs | 315 |
| `test_update_to_duplicate_shop_name` | 重複名への更新エラー | src/services/shop.rs | 337 |
| `test_update_same_shop_name` | 同じ名前への更新（許可） | src/services/shop.rs | 368 |

**合計**: 7件

### services/transaction.rs

取引管理サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_save_transaction_header_with_tax_excluded` | 税抜取引ヘッダー保存 | src/services/transaction.rs | 1246 |
| `test_save_transaction_header_with_tax_included` | 税込取引ヘッダー保存 | src/services/transaction.rs | 1278 |
| `test_update_transaction_header_tax_type` | 取引ヘッダーの税種別更新 | src/services/transaction.rs | 1309 |
| `test_default_tax_type_is_excluded` | デフォルト税種別が税抜であることを確認 | src/services/transaction.rs | 1351 |
| `test_tax_type_validation_values` | 税種別の有効値確認 | src/services/transaction.rs | 1375 |

**合計**: 5件

### services/aggregation.rs

集計サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_monthly_aggregation_current_month` | 当月の月次集計 | src/services/aggregation.rs | 1554 |
| `test_monthly_aggregation_next_month` | 翌月の月次集計 | src/services/aggregation.rs | 1563 |

**合計**: 2件

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

---

## テスト統計サマリー

| カテゴリ | テスト数 |
|---------|---------|
| **共通テストスイート** | **23件** |
| validation_tests.rs | 10 |
| font_size_tests.rs | 13 |
| **インラインテスト** | **178件** |
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
| **総計** | **201件** |

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
