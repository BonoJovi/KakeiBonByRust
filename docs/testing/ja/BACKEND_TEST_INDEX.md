# バックエンドテストインデックス

このドキュメントは、Rustで実装されたバックエンドテストの完全なインデックスです。

**最終更新**: 2026-09-06 JST  
**総テスト数**: 466件 (差分反映後。`cargo test --lib` の権威的総数は 704 で、既存の未反映分は別 PR でバックフィル予定)

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
  - [services/like_escape.rs](#serviceslike_escapers)
  - [services/manufacturer.rs](#servicesmanufacturerrs)
  - [services/product.rs](#servicesproductrs)
  - [services/shop.rs](#servicesshoprs)
  - [services/transaction.rs](#servicestransactionrs)
  - [services/aggregation.rs](#servicesaggregationrs)
  - [services/period.rs](#servicesperiodrs)
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
| `pool_connections_all_enforce_foreign_keys` | プールが返すすべての接続で `PRAGMA foreign_keys = ON` が有効。修正前は起動時に 1 回だけ実行された接続のみ FK が有効で、それ以外の借り手が取った接続では SHOPS user-cascade マイグレーションが無効化されていた (#128 CodeRabbit 外側指摘) | src/db.rs | 1630 |
| `migrate_shops_user_id_cascade_adds_cascade_fk_and_preserves_rows` | テーブルを再作成して SHOPS.USER_ID FK に `ON DELETE CASCADE` を追加、SHOP_ID と各列の値はそのまま保持されること (Fable-5 #11) | src/db.rs | 1711 |
| `migrate_shops_user_id_cascade_is_idempotent` | SHOPS CASCADE マイグレーションの 2 回目は既に CASCADE FK があるため早期に戻る。マイグレーション済み DB では DROP/RENAME は走らない (Fable-5 #11) | src/db.rs | 1799 |
| `user_delete_cascades_to_shops_after_migration` | CASCADE マイグレーション後、SHOPS 行を持つユーザーの削除が成功し、SHOPS 行も同時に削除される。修正前は `FOREIGN KEY constraint failed` でロールバックしていた (Fable-5 #11) | src/db.rs | 1823 |
| `latent_h5_migration_backfills_null_amount_including_tax` | 起動時マイグレーションが NULL の AMOUNT_INCLUDING_TAX を AMOUNT + TAX_AMOUNT で補完 (潜在監査 H5) | src/latent_audit/db.rs | 33 |
| `latent_m3_startup_removes_orphan_user_categories` | 起動時の掃除で削除済みユーザーの費目を消し、存在するユーザーの費目は残す (潜在監査 M3) | src/latent_audit/db.rs | 106 |
| `latent_m2_startup_repairs_header_category1_mismatch` | 明細と大分類が食い違うヘッダーを起動時に明細側の大分類へ戻し、口座も対応する側へ移す。明細が混在するヘッダーと整合しているヘッダーは変えない (潜在監査 M2) | src/latent_audit/db.rs | 142 |
| `latent_m2_accounts_follow_category1_side` | 口座は新しい大分類が使う側へ移る (支出: FROM、収入: TO) (潜在監査 M2) | src/latent_audit/db.rs | 226 |

**合計**: 16件

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
| `latent_l28_null_settings_file_falls_back_to_defaults` | `null` の設定ファイルは起動失敗ではなく既定値にフォールバック (潜在監査 L28) | src/latent_audit/settings.rs | 45 |
| `latent_l28_array_settings_file_falls_back_to_defaults` | `[]` の設定ファイルは既定値にフォールバック (潜在監査 L28) | src/latent_audit/settings.rs | 53 |
| `latent_l28_truncated_settings_file_falls_back_to_defaults` | 途中で切れた JSON の設定ファイルは既定値にフォールバック (潜在監査 L28) | src/latent_audit/settings.rs | 61 |
| `latent_l28_corrupt_settings_file_is_backed_up_and_replaced_on_save` | 読めないファイルは `<name>.corrupt` として残し、次の保存で正しいファイルを書く (潜在監査 L28) | src/latent_audit/settings.rs | 68 |
| `latent_l28_existing_backup_is_kept_and_unbackupable_file_is_not_replaced` | 既存の `.corrupt` は上書きせず空いている名前に控える。控えを作れなければ読み込みを失敗させ元ファイルを守る (潜在監査 L28) | src/latent_audit/settings.rs | 95 |
| `latent_l28_backup_holds_the_content_that_failed_to_parse` | `.corrupt` の控えは、読み込み後にファイルが変わっても解析に失敗した中身そのものを保持する (潜在監査 L28) | src/latent_audit/settings.rs | 128 |

**合計**: 18件

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

### services/like_escape.rs

共有 `escape_like_pattern` — `LIKE ? ESCAPE '\'` を使う全ての検索経路が利用する LIKE メタ文字エスケーパの pure-function テスト (Fable-5 レビュー #23 で `transaction.rs` の private helper から抽出、`product::search_products_by_name` と契約を共有)。

| Test Function | Description | File | Line |
|---------------|-------------|------|------|
| `plain_text_passes_through_unchanged` | 通常 ASCII テキストはそのまま返す | src/services/like_escape.rs | 34 |
| `percent_is_escaped` | `%` → `\%` | src/services/like_escape.rs | 39 |
| `underscore_is_escaped` | `_` → `\_` | src/services/like_escape.rs | 44 |
| `backslash_is_escaped_first_then_metacharacters` | `\` を先にエスケープしてから `%` / `_` を処理（自己エスケープ回避） | src/services/like_escape.rs | 49 |
| `multiple_metacharacters_all_escaped` | `50%_off` → `50\%\_off` | src/services/like_escape.rs | 57 |
| `empty_input_yields_empty_output` | 空入力は空出力 | src/services/like_escape.rs | 62 |
| `japanese_text_with_percent_escapes_only_the_metacharacter` | 果汁100%ジュース → 果汁100\%ジュース (Fable-5 #23 の pin シナリオ) | src/services/like_escape.rs | 67 |

**合計**: 7件

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
| `latent_l25_register_rejects_blank_username` | 初回登録で空・空白のみのユーザー名を拒否 (潜在監査 L25) | src/services/latent_audit/auth.rs | 41 |
| `latent_l25_register_rejects_overlong_username` | 初回登録で長すぎるユーザー名を拒否 (潜在監査 L25) | src/services/latent_audit/auth.rs | 62 |
| `latent_l25_register_duplicate_name_maps_to_duplicate_code` | 初回登録のユーザー名重複は生の UNIQUE エラーではなく `duplicate_name` (潜在監査 L25) | src/services/latent_audit/auth.rs | 82 |
| `latent_l26_admin_category_seed_failure_rolls_back_user` | 管理者登録で費目投入に失敗したら管理者を削除し、再セットアップ可能にする (潜在監査 L26) | src/services/latent_audit/auth.rs | 120 |
| `latent_l26_admin_none_account_failure_rolls_back_user_and_categories` | 指定なし口座の作成失敗で管理者と作成済みの費目を削除 (潜在監査 L26) | src/services/latent_audit/auth.rs | 148 |
| `latent_l26_register_user_seed_failure_rolls_back_user` | 一般ユーザー登録で投入に失敗したらユーザーを削除 (潜在監査 L26) | src/services/latent_audit/auth.rs | 171 |

**合計**: 22件

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
| `latent_m3_delete_user_removes_categories` | 一般ユーザー削除で CATEGORY1/2/3 と *_I18N も削除される (潜在監査 M3) | src/services/latent_audit/user_management.rs | 64 |
| `latent_m3_reused_user_id_gets_default_categories` | 削除済み USER_ID を再利用した新ユーザーは既定の費目を持ち、旧ユーザーの費目を引き継がない (潜在監査 M3) | src/services/latent_audit/user_management.rs | 92 |
| `latent_m13_create_rejects_blank_username` | 空 / 空白のみの名前でのユーザー作成を拒否 (潜在監査 M13) | src/services/latent_audit/user_management.rs | 135 |
| `latent_m13_update_rejects_blank_username` | 一般 / 管理者ユーザーの空白名への変更を拒否 (潜在監査 M13) | src/services/latent_audit/user_management.rs | 158 |

**合計**: 23件

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
| `test_register_encrypted_field_rejects_ineligible_fields` | USERS・ユーザー別でないテーブル・TEXT 以外/存在しないカラム・ビュー・WITHOUT ROWID テーブル・ROWID 列を宣言したテーブル・平文が既に入っているカラムの登録を拒否。カラム名は大文字小文字を区別しない (潜在監査 L27) | src/services/encryption.rs | 589 |
| `latent_l27_password_change_survives_plaintext_column_registration` | USERS.NAME の登録を試みた後もパスワード変更が成功する (潜在監査 L27) | src/services/latent_audit/encryption.rs | 47 |
| `latent_l27_password_change_survives_non_text_column_registration` | INTEGER カラムの登録を試みた後もパスワード変更が成功する (潜在監査 L27) | src/services/latent_audit/encryption.rs | 68 |

**合計**: 11件

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
| `test_get_account_balances_as_of_self_transfer_nets_to_zero` | FROM == TO の残存 TRANSFER 行はダッシュボード残高で相殺され、残高が水増しされないこと (Fable-5 #20) | src/services/account.rs | 991 |
| `test_get_accounts_lists_only_own_accounts` | 管理者を含め、各ユーザーは自分の口座だけを一覧する (潜在監査 M4) | src/services/account.rs | 860 |
| `test_get_accounts_include_disabled` | 無効な口座は `include_disabled` 指定時だけ一覧に出る (潜在監査 M7) | src/services/account.rs | 887 |
| `test_delete_disabled_account_removes_row` | 未使用の無効口座も削除でき、行が消える (潜在監査 M7) | src/services/account.rs | 519 |
| `test_disable_account_allowed_while_referenced` | 取引が使用中の口座は削除できないが無効化・再有効化はできる (潜在監査 M7) | src/services/account.rs | 532 |
| `test_account_is_disabled_must_be_zero_or_one` | 無効フラグは追加・更新とも 0 / 1 のみ受け付ける (潜在監査 M7) | src/services/account.rs | 553 |
| `test_none_account_cannot_be_changed` | NONE (未指定) 口座は追加・編集・無効化・削除できない (潜在監査 M7) | src/services/account.rs | 586 |
| `test_get_account_balances_as_of_keeps_disabled_accounts_with_balance` | 残高が残っている無効口座はダッシュボードに `is_disabled` 付きで残る (潜在監査 M7) | src/services/account.rs | 1235 |
| `latent_m4_admin_account_list_excludes_other_users_and_deleted` | 管理者に返す口座一覧は管理者自身の有効な口座だけ (潜在監査 M4) | src/services/latent_audit/account.rs | 40 |

**合計**: 19件

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
| `latent_m8_enable_category2_restores_cascaded_category3` | 非表示にした中分類を表示に戻すと、一緒に非表示になった小分類も戻る (潜在監査 M8) | src/services/latent_audit/category.rs | 83 |
| `latent_l19_enable_missing_category2_returns_not_found` | 存在しない中分類の表示復帰は not_found (潜在監査 L19、中分類の表示復帰のみ) | src/services/latent_audit/category.rs | 223 |
| `latent_m8_enable_already_enabled_category2_keeps_hidden_children` | 表示中の中分類への表示復帰は何もせず、個別に非表示にした小分類は非表示のまま (潜在監査 M8) | src/services/latent_audit/category.rs | 110 |
| `latent_l18_update_missing_category2_returns_not_found` | 存在しない中分類の名前変更は成功扱いにせず not_found (潜在監査 L18) | src/services/latent_audit/category.rs | 140 |
| `latent_l18_update_missing_category3_returns_not_found` | 存在しない小分類の名前変更は not_found (潜在監査 L18) | src/services/latent_audit/category.rs | 153 |
| `latent_l18_add_category2_is_atomic_on_i18n_failure` | i18n の追加失敗で中途半端な中分類が残らない (追加を単一トランザクションで実行、潜在監査 L18) | src/services/latent_audit/category.rs | 172 |
| `latent_l19_move_missing_category2_returns_not_found` | 存在しない中分類の移動は生の RowNotFound ではなく not_found (潜在監査 L19) | src/services/latent_audit/category.rs | 217 |
| `latent_l19_move_missing_category3_returns_not_found` | 存在しない小分類の移動は not_found (潜在監査 L19) | src/services/latent_audit/category.rs | 228 |
| `latent_l19_enable_missing_category3_returns_not_found` | 存在しない小分類の再表示は not_found (潜在監査 L19) | src/services/latent_audit/category.rs | 254 |
| `latent_l20_category3_code_unique_across_category2_parents` | C2_E_1 と C2_E_11 の配下で小分類コードが衝突しない (親コード全体から生成しユーザー内で一意、潜在監査 L20) | src/services/latent_audit/category.rs | 276 |
| `latent_l20_add_category2_empty_category1_code_does_not_panic` | 空の費目1コードはパニックせず拒否 (潜在監査 L20) | src/services/latent_audit/category.rs | 312 |
| `latent_l20_add_category2_multibyte_category1_code_does_not_panic` | 存在しないマルチバイトの費目1コードはパニックせず拒否 (潜在監査 L20) | src/services/latent_audit/category.rs | 323 |
| `latent_l20_add_category3_multibyte_category1_code_does_not_panic` | 存在しないマルチバイトの費目1配下への小分類追加はパニックせず拒否 (潜在監査 L20) | src/services/latent_audit/category.rs | 333 |

**合計**: 38件

### services/manufacturer.rs

メーカー管理サービスのテスト。empty/duplicate 系のテスト名は `ApiError` 移行 (Fable-5 #23) に合わせ `_returns_validation_code` / `_returns_duplicate_name_code` にリネーム — 挙動は変わらず assertion の対象のみ変更。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_add_manufacturer` | メーカー追加テスト | src/services/manufacturer.rs | 243 |
| `test_update_manufacturer` | メーカー更新テスト | src/services/manufacturer.rs | 261 |
| `test_delete_manufacturer` | 未使用のメーカーは非表示ではなく行ごと削除される (潜在監査 M7) | src/services/manufacturer.rs | 283 |
| `test_delete_disabled_manufacturer_removes_row` | 未使用の無効メーカーも削除でき、行が消える (潜在監査 M7) | src/services/manufacturer.rs | 309 |
| `test_disable_manufacturer_allowed_while_referenced` | 商品が使用中のメーカーは削除できないが無効化はでき、商品側にメーカー名は残る (潜在監査 M7) | src/services/manufacturer.rs | 329 |
| `test_empty_manufacturer_name_returns_validation_code` | 空メーカー名は `ApiError { code: "validation" }` (Fable-5 #23) | src/services/manufacturer.rs | 316 |
| `test_add_duplicate_manufacturer_returns_duplicate_name_code` | 重複は `ApiError { code: "duplicate_name", entity: "manufacturer" }` (Fable-5 #23) | src/services/manufacturer.rs | 331 |
| `test_update_to_duplicate_manufacturer_name_returns_duplicate_name_code` | 重複への更新は `ApiError { code: "duplicate_name" }` (Fable-5 #23) | src/services/manufacturer.rs | 355 |
| `test_update_missing_manufacturer_returns_not_found_code` | 存在しないメーカーの更新は `ApiError { code: "not_found", entity: "manufacturer" }` (Fable-5 #23) | src/services/manufacturer.rs | 383 |
| `test_delete_missing_manufacturer_returns_not_found_code` | 存在しないメーカーの削除は `ApiError { code: "not_found" }` (Fable-5 #23) | src/services/manufacturer.rs | 398 |
| `test_update_same_manufacturer_name` | 同じ名前への更新（許可） | src/services/manufacturer.rs | 405 |
| `test_delete_manufacturer_rejected_when_referenced_by_product` | PRODUCTS がメーカーを参照中なら `ApiError { code: "in_use", entity: "manufacturer" }` で削除拒否（マスタ削除ロック） | src/services/manufacturer.rs | 374 |
| `test_delete_manufacturer_rejected_when_only_disabled_products_reference` | IS_DISABLED=1 の商品でも参照とみなす（FK は残り、「無効表示」でも一覧に出るため、マスタ削除ロック） | src/services/manufacturer.rs | 402 |
| `test_delete_manufacturer_ignores_other_users_references` | 他ユーザーの同一 MANUFACTURER_ID 参照は削除をブロックしない（USER_ID スコープ、マスタ削除ロック） | src/services/manufacturer.rs | 429 |
| `latent_m6_readd_deleted_manufacturer_name_is_not_database_error` | 削除済みメーカーと同名の再登録で汎用 database エラーにならない (潜在監査 M6) | src/services/latent_audit/manufacturer.rs | 26 |
| `latent_m6_readd_disabled_manufacturer_name_revives_original_row` | 無効メーカーと同名の追加は元の行を有効化して再利用 (潜在監査 M6) | src/services/latent_audit/manufacturer.rs | 108 |
| `latent_m6_rename_onto_disabled_manufacturer_name_is_duplicate_name` | 無効メーカーの名前への変更は duplicate_name で拒否 (潜在監査 M6) | src/services/latent_audit/manufacturer.rs | 131 |

**合計**: 17件

### services/product.rs

商品管理サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_add_product_without_manufacturer` | メーカーなしの商品追加 | src/services/product.rs | 256 |
| `test_add_product_with_manufacturer` | メーカーありの商品追加 | src/services/product.rs | 276 |
| `test_update_product` | 商品更新テスト | src/services/product.rs | 309 |
| `test_delete_product` | 未使用の商品は非表示ではなく行ごと削除される (潜在監査 M7) | src/services/product.rs | 410 |
| `test_delete_disabled_product_removes_row` | 未使用の無効商品も削除でき、行が消える (潜在監査 M7) | src/services/product.rs | 437 |
| `test_disable_product_allowed_while_referenced` | 明細が使用中の商品は削除できないが無効化はできる (潜在監査 M7) | src/services/product.rs | 458 |
| `test_empty_product_name` | 空商品名のエラー | src/services/product.rs | 367 |
| `test_add_duplicate_product` | 重複商品名のエラー | src/services/product.rs | 383 |
| `test_manufacturer_deletion_rejected_while_product_references_it` | 商品が参照中はメーカー削除が `ApiError { code: "in_use", entity: "manufacturer" }` で拒否される — マスタ削除ロック導入で `test_manufacturer_deletion_sets_product_manufacturer_to_null` からリネーム（旧: 論理削除→CASCADE NULL の fallback） | src/services/product.rs | 512 |
| `test_add_product_rejects_foreign_manufacturer_id` | 他ユーザーの manufacturer_id で add は "Manufacturer not found" (Fable-5 #13) | src/services/product.rs | 716 |
| `test_add_product_rejects_nonexistent_manufacturer_id` | 存在しない manufacturer_id で add は "Manufacturer not found" (Fable-5 #13) | src/services/product.rs | 767 |
| `test_update_product_rejects_foreign_manufacturer_id` | 他ユーザーの manufacturer_id で update は "Manufacturer not found" (Fable-5 #13) | src/services/product.rs | 792 |
| `test_product_join_scopes_manufacturer_by_user_id` | PRODUCT_GET_* JOIN は他ユーザーの manufacturer 名を漏らさない (Fable-5 #13) | src/services/product.rs | 871 |
| `test_delete_product_rejected_when_referenced_by_transaction_detail` | TRANSACTIONS_DETAIL が商品を参照中なら `ApiError { code: "in_use", entity: "product" }` で削除拒否（TRANSACTIONS_HEADER.USER_ID 経由でスコープ、マスタ削除ロック） | src/services/product.rs | 444 |
| `test_delete_product_ignores_other_users_transaction_details` | 他ユーザーの明細参照は削除をブロックしない（TRANSACTIONS_HEADER.USER_ID でスコープ、マスタ削除ロック） | src/services/product.rs | 481 |
| `test_search_products_escapes_percent_metacharacter` | オートコンプリート検索で `"100%ジ"` が「果汁100%ジュース」だけにマッチし「果汁100リンゴジュース」にマッチしないこと — `%` をエスケープし `LIKE ? ESCAPE '\'` を併用 (Fable-5 #23) | src/services/product.rs | 785 |
| `test_search_products_escapes_underscore_metacharacter` | オートコンプリート検索で `"A_1"` が literal "A_1" だけにマッチし "AB1" にマッチしないこと — `_` をエスケープ (Fable-5 #23) | src/services/product.rs | 812 |
| `latent_m6_readd_deleted_product_name_is_not_database_error` | 削除済み商品と同名の再登録で汎用 database エラーにならない (潜在監査 M6) | src/services/latent_audit/product.rs | 27 |
| `latent_m6_readd_disabled_product_name_revives_original_row` | 無効商品と同名の追加は元の行 (同じ PRODUCT_ID) を有効化して再利用 (潜在監査 M6) | src/services/latent_audit/product.rs | 113 |
| `latent_m6_rename_onto_disabled_product_name_is_duplicate_name` | 無効商品の名前への変更は duplicate_name で拒否 (潜在監査 M6) | src/services/latent_audit/product.rs | 136 |

**合計**: 20件

### services/shop.rs

店舗管理サービスのテスト。empty/duplicate 系のテスト名は `ApiError` 移行 (Fable-5 #23) に合わせ `_returns_validation_code` / `_returns_duplicate_name_code` にリネーム — 挙動は変わらず assertion の対象のみ変更。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_add_shop` | 店舗追加テスト | src/services/shop.rs | 232 |
| `test_update_shop` | 店舗更新テスト | src/services/shop.rs | 249 |
| `test_delete_shop` | 未使用の店舗は非表示ではなく行ごと削除される (潜在監査 M7) | src/services/shop.rs | 294 |
| `test_delete_disabled_shop_removes_row` | 未使用の無効店舗も削除でき、行が消える (潜在監査 M7) | src/services/shop.rs | 320 |
| `test_shop_is_disabled_must_be_zero_or_one` | 無効フラグは追加・更新とも 0 / 1 のみ受け付ける (それ以外は validation エラー) (潜在監査 M7) | src/services/shop.rs | 340 |
| `test_disable_shop_allowed_while_referenced` | 取引が使用中の店舗は削除できないが無効化・再有効化はできる (潜在監査 M7) | src/services/shop.rs | 376 |
| `test_empty_shop_name_returns_validation_code` | 空店舗名は `ApiError { code: "validation" }` (Fable-5 #23) | src/services/shop.rs | 301 |
| `test_add_duplicate_shop_returns_duplicate_name_code` | 重複は `ApiError { code: "duplicate_name", entity: "shop" }` (Fable-5 #23) | src/services/shop.rs | 315 |
| `test_update_to_duplicate_shop_name_returns_duplicate_name_code` | 重複への更新は `ApiError { code: "duplicate_name" }` (Fable-5 #23) | src/services/shop.rs | 337 |
| `test_update_missing_shop_returns_not_found_code` | 存在しない店舗の更新は `ApiError { code: "not_found", entity: "shop" }` (Fable-5 #23) | src/services/shop.rs | 363 |
| `test_delete_missing_shop_returns_not_found_code` | 存在しない店舗の削除は `ApiError { code: "not_found" }` (Fable-5 #23) | src/services/shop.rs | 375 |
| `test_update_same_shop_name` | 同じ名前への更新（許可） | src/services/shop.rs | 382 |
| `test_delete_shop_rejected_when_referenced_by_transaction` | TRANSACTIONS_HEADER が店舗を参照中なら `ApiError { code: "in_use", entity: "shop" }` で削除拒否（マスタ削除ロック） | src/services/shop.rs | 346 |
| `test_delete_shop_rejected_when_referenced_by_recurring_rule` | RECURRING_RULES が店舗を参照中なら `ApiError { code: "in_use" }` で削除拒否（マスタ削除ロック） | src/services/shop.rs | 372 |
| `test_delete_shop_ignores_other_users_references` | 他ユーザーの同一 SHOP_ID 参照は削除をブロックしない（USER_ID スコープ、マスタ削除ロック） | src/services/shop.rs | 394 |
| `latent_h6_readd_deleted_shop_name_is_not_database_error` | 削除済み店舗と同名の再登録で汎用 database エラーにならない (潜在監査 H6) | src/services/latent_audit/shop.rs | 24 |
| `latent_h6_readd_disabled_shop_name_revives_original_row` | 無効店舗と同名の再登録は元の行を復活させる (同じ SHOP_ID、メモは新しい値) (潜在監査 H6) | src/services/latent_audit/shop.rs | 82 |
| `latent_h6_rename_onto_disabled_shop_name_is_duplicate_name` | 無効店舗の名前への変更は duplicate_name で拒否 (潜在監査 H6) | src/services/latent_audit/shop.rs | 113 |
| `latent_h6_insert_unique_violation_maps_to_duplicate_name` | 重複チェックをすり抜けた add_shop の INSERT が UNIQUE 違反になった場合 duplicate_name を返す (潜在監査 H6) | src/services/latent_audit/shop.rs | 124 |

**合計**: 19件

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
| `test_find_matching_pattern_falls_back_to_priority_when_preferred_mismatches` | 現在設定で `target_total` を再現できない場合、優先順 PATTERNS 探索へフォールバック (Fable-5 #2) | src/services/transaction.rs | 1968 |
| `test_find_matching_pattern_returns_none_when_no_pattern_fits` | どの組み合わせも `target_total` を再現できない場合は `None`、呼び出し側は設定列でなく TOTAL_AMOUNT を上書き (Fable-5 #2) | src/services/transaction.rs | 1859 |
| `test_save_header_rejects_invalid_tax_included_type` | `save_transaction_header` が `{TAX_INCLUDED, TAX_EXCLUDED}` 以外の `tax_included_type` を拒否し、無効値が `find_matching_pattern` の「優先設定を先に確認する判定」に流れて残らないこと (#125 の CodeRabbit 指摘) | src/services/transaction.rs | 3230 |
| `test_update_header_rejects_invalid_tax_included_type` | 更新入口にも同じガード (#125 の CodeRabbit 指摘) | src/services/transaction.rs | 3257 |
| `test_save_header_rejects_transfer_from_equals_to` | `save_transaction_header` が FROM == TO の TRANSFER を拒否し、ダッシュボード残高の水増しを防ぐ (Fable-5 #20) | src/services/transaction.rs | 3288 |
| `test_update_header_rejects_transfer_from_equals_to` | 更新入口にも同じガード (Fable-5 #20) | src/services/transaction.rs | 3316 |
| `test_save_header_failure_rolls_back_memo_insert_in_same_tx` | tx 内 HEADER insert 失敗 (ローカル `RAISE(FAIL)` トリガー) で MEMO insert も同 tx でロールバック、MEMOS 空を確認 (Fable-5 #6) | src/services/transaction.rs | 3357 |
| `test_save_header_dedupes_memo_text_across_multiple_saves` | 同じ memo 本文で 2 回 save → MEMOS 1 行のみ、MEMO_ID 共有 (tx-based helper 再利用の dedup 副次効果、Fable-5 #6) | src/services/transaction.rs | 3427 |
| `test_add_detail_dedupes_memo_text_across_multiple_adds` | `add_transaction_detail` が同一ユーザーの同一メモ本文で既存 MEMOS 行を再利用。重複行なし、両明細で MEMO_ID 共有 (Fable-5 #7) | src/services/transaction.rs | 4475 |
| `test_add_detail_reuses_memo_shared_with_header` | 親ヘッダーの MEMO_ID と同じ本文で detail 追加すると同じ MEMO_ID を再利用。update の「共有メモ」経路が add 側からも到達可能に (Fable-5 #7) | src/services/transaction.rs | 4519 |
| `test_add_detail_failure_rolls_back_memo_insert_in_same_tx` | DETAIL_INSERT 内の FK 失敗 (`(USER_ID, CATEGORY1_CODE) → CATEGORY1` が未 seed) で MEMO insert も同 tx でロールバック、MEMOS 空を確認 (Fable-5 #7) | src/services/transaction.rs | 4585 |
| `transfer_same_account_maps_to_stable_wire_code_and_omits_entity` | `TransactionError::TransferSameAccount` が `ApiError { code: "transfer_same_account", entity: None }` に変換される wire contract を固定。将来のリファクタで generic な `validation` フォールバックへ無言で退化させないための pin (#127 の CodeRabbit 指摘) | src/services/transaction.rs | 4664 |
| `latent_h5_included_header_total_sums_amount_including_tax` | 税込ヘッダーの合計 = SUM(AMOUNT_INCLUDING_TAX) (潜在監査 H5) | src/services/latent_audit/transaction.rs | 199 |
| `latent_h5_compute_recommended_total_honours_included_header` | `compute_recommended_total` がヘッダーの TAX_INCLUDED_TYPE を考慮する (潜在監査 H5) | src/services/latent_audit/transaction.rs | 211 |
| `latent_h5_bulk_recalc_keeps_consistent_included_header` | 整合した税込ヘッダーを一括再計算が書き換えない (潜在監査 H5) | src/services/latent_audit/transaction.rs | 232 |
| `latent_l1_small_detail_with_zero_tax_is_still_grossed_up` | 税額が丸めで 0 円になる少額明細も税率単位で gross-up される (潜在監査 L1) | src/services/latent_audit/transaction.rs | 259 |
| `test_calculate_recommended_total_uses_amount_not_amount_including_tax` | 外税の合計は AMOUNT_INCLUDING_TAX に関係なく AMOUNT を gross-up (AMOUNT は常に税抜) | src/services/transaction.rs | 1837 |
| `test_calculate_recommended_total_with_settings_included_derives_missing_rows` | 税込の合計で NULL / 0 の行を AMOUNT + TAX_RATE から導出 | src/services/transaction.rs | 1890 |
| `latent_h4_bulk_recalc_keeps_total_without_details` | 明細なしヘッダーは一括再計算で TOTAL_AMOUNT を変更しない (潜在監査 H4) | src/services/latent_audit/transaction.rs | 177 |
| `latent_h4_compute_recommended_total_is_none_without_details` | 明細なしヘッダーの `compute_recommended_total` は None を返す (潜在監査 H4) | src/services/latent_audit/transaction.rs | 666 |
| `latent_m1_update_header_persists_is_scheduled` | ヘッダー更新で予定チェック (IS_SCHEDULED) が保存される (潜在監査 M1) | src/services/latent_audit/transaction.rs | 273 |
| `latent_m1_update_header_without_flag_keeps_is_scheduled` | `is_scheduled: None` のヘッダー更新は既存の値を保つ (潜在監査 M1) | src/services/latent_audit/transaction.rs | 679 |
| `latent_m1_invalid_is_scheduled_is_rejected` | 保存・更新で 0/1 以外の IS_SCHEDULED を拒否する (潜在監査 M1) | src/services/latent_audit/transaction.rs | 698 |
| `latent_m9_restore_reverts_tax_settings_changed_by_recalc` | ロールバックで再計算が変えた税設定も元に戻る (潜在監査 M9) | src/services/latent_audit/transaction.rs | 341 |
| `latent_m9_restore_keeps_edits_made_after_recalc` | 再計算が変えなかったヘッダーへの再計算後の編集はロールバックで上書きされない (潜在監査 M9) | src/services/latent_audit/transaction.rs | 380 |
| `latent_m9_restore_keeps_edit_on_a_header_the_recalc_changed` | 再計算が変えた後にユーザーが編集したヘッダーは編集が残る (潜在監査 M9) | src/services/latent_audit/transaction.rs | 414 |
| `latent_m2_header_category1_change_keeps_details_consistent` | 明細があるヘッダーの大分類変更は拒否され (`Category1HasDetails`)、ヘッダーと明細は一致したまま (潜在監査 M2) | src/services/latent_audit/transaction.rs | 297 |
| `latent_m2_header_without_details_can_change_category1` | 明細のないヘッダーは大分類を変更できる (潜在監査 M2) | src/services/latent_audit/transaction.rs | 339 |
| `latent_m2_detail_category1_must_match_header` | ヘッダーと異なる大分類の明細は追加・編集できない (潜在監査 M2) | src/services/latent_audit/transaction.rs | 363 |
| `latent_m9_restore_without_journal_is_rejected` | バックアップ横の変更記録が無いロールバックは拒否され何も変更しない (潜在監査 M9) | src/services/latent_audit/transaction.rs | 451 |
| `latent_l4_restore_detaches_backup_when_update_fails` | 失敗したロールバック後に `recalc_backup` が ATTACH されたまま残らない (M9 でロールバックが ATTACH しなくなり解消、潜在監査 L4) | src/services/latent_audit/transaction.rs | 657 |
| `latent_m9_back_to_back_recalcs_keep_separate_journals` | 連続した一括再計算は別々のバックアップ・変更記録を使い、1 回目も取り消せる (潜在監査 M9) | src/services/latent_audit/transaction.rs | 452 |
| `latent_l2_save_header_rejects_foreign_shop_id` | 他ユーザーの SHOP_ID でのヘッダー保存を拒否 (潜在監査 L2) | src/services/latent_audit/transaction.rs | 512 |
| `latent_l2_update_header_rejects_foreign_shop_id` | 他ユーザーの SHOP_ID へのヘッダー更新を拒否 (潜在監査 L2) | src/services/latent_audit/transaction.rs | 531 |
| `latent_l2_add_detail_rejects_foreign_product_id` | 他ユーザーの PRODUCT_ID での明細追加を拒否 (潜在監査 L2) | src/services/latent_audit/transaction.rs | 554 |
| `latent_l2_update_detail_rejects_foreign_product_id` | 他ユーザーの PRODUCT_ID への明細更新を拒否 (潜在監査 L2) | src/services/latent_audit/transaction.rs | 583 |
| `latent_l2_header_with_info_does_not_leak_foreign_shop_name` | ヘッダー詳細取得で他ユーザーの店舗名が出ない (SHOPS 結合を USER_ID で限定、潜在監査 L2) | src/services/latent_audit/transaction.rs | 616 |
| `latent_l8_save_header_rejects_malformed_datetime` | 不正・実在しない日時でのヘッダー保存を拒否 (潜在監査 L8) | src/services/latent_audit/transaction.rs | 752 |
| `latent_l8_update_header_rejects_malformed_datetime` | 不正・実在しない日時でのヘッダー更新を拒否 (潜在監査 L8) | src/services/latent_audit/transaction.rs | 766 |
| `latent_l3_failed_detail_update_rolls_back_memo_change` | 明細更新が失敗したらメモの変更も戻る (メモ処理を同じトランザクションで実行、潜在監査 L3) | src/services/latent_audit/transaction.rs | 674 |
| `latent_l3_in_place_memo_update_is_trimmed` | メモの上書き更新は前後の空白を除いた内容で保存し、重複排除と一致させる (潜在監査 L3) | src/services/latent_audit/transaction.rs | 702 |

**合計**: 64件

### services/aggregation.rs

集計サービスのテスト。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `test_monthly_aggregation_current_month` | 当月の月次集計 | src/services/aggregation.rs | 1554 |
| `test_monthly_aggregation_next_month` | 翌月の月次集計 | src/services/aggregation.rs | 1563 |
| `test_detail_query_grosses_up_null_tax_included_row` | TAX_RATE>0 で AMOUNT_INCLUDING_TAX が NULL の明細も税抜として割増 (Fable-5 #3) | src/services/aggregation.rs | 2581 |
| `test_detail_query_grosses_up_zero_tax_included_row` | AMOUNT_INCLUDING_TAX=0 (フロント空欄) も税抜扱い (Fable-5 #3) | src/services/aggregation.rs | 2610 |
| `test_detail_query_included_header_derives_null_tax_included_row` | 税込ヘッダー + レガシー `AMOUNT_INCLUDING_TAX = NULL` 明細は税抜 AMOUNT から税込額を導出する (Fable-5 #3 の解釈を置換、潜在監査 H5) | src/services/aggregation.rs | 2741 |
| `test_detail_query_included_header_derives_zero_tax_included_row` | 税込ヘッダー配下の `AMOUNT_INCLUDING_TAX = 0` (フロント空欄) も同じく税抜 AMOUNT から導出 (潜在監査 H5) | src/services/aggregation.rs | 2770 |
| `test_detail_query_matches_header_query_for_included_ledger` | 税込ヘッダーの同一伝票でヘッダー集計と明細集計の値が一致する (Fable-5 #4) | src/services/aggregation.rs | 2805 |
| `test_detail_query_avg_matches_total_over_count_with_mixed_rates` | 混在税率取引で avg × count == total を保持 (Fable-5 #4) | src/services/aggregation.rs | 2774 |
| `test_detail_query_avg_multi_transaction_arithmetic` | 2 取引の avg = total / txn_count 検証 (Fable-5 #4) | src/services/aggregation.rs | 2811 |
| `test_detail_query_binds_category_filter_no_injection` | カテゴリフィルタの値が bind されている (SQL 直埋めではない) ことを End-to-End で確認。`EXPENSE' OR '1'='1` payload は 0 rows を返す (PR5, Fable-5 #25) | src/services/aggregation.rs | 2846 |
| `test_category_filter_category2_targets_detail_column` | Category2 フィルタが `td.CATEGORY2_CODE` (detail-scope、実在する列) を参照し、`th.CATEGORY2_CODE` (存在しない列) を参照しないこと (PR6, Fable-5 #17) | src/services/aggregation.rs | 2902 |
| `test_category_filter_category3_targets_detail_column` | Category3 フィルタが `td.CATEGORY2/3_CODE` を参照すること (PR6, Fable-5 #17) | src/services/aggregation.rs | 2918 |
| `test_account_query_applies_category_filter_to_all_union_branches` | 口座別集計の 4-branch UNION ALL 全てにカテゴリフィルタが適用され、bind vec に 4 回登場することを確認 (PR6, Fable-5 #18: silent drop の regression pin) | src/services/aggregation.rs | 2943 |
| `test_build_query_shop_uses_empty_string_fallback_no_hardcoded_ja` | Shop 集計が `COALESCE(s.SHOP_NAME, '')` 空文字 sentinel を返し、日本語ハードコード `'指定なし'` を含まないこと (Fable-5 #22) | src/services/aggregation.rs | 2024 |
| `test_build_query_product_uses_empty_string_fallback_no_hardcoded_ja` | Product 集計が `COALESCE(p.PRODUCT_NAME, '')` 空文字 sentinel を返し、日本語ハードコード `'指定なし'` を含まないこと (Fable-5 #22) | src/services/aggregation.rs | 2044 |
| `test_build_query_account_uses_empty_string_for_none_no_hardcoded_ja` | Account 集計が `account_code = 'NONE'` を空文字にマップし、欠損行では `COALESCE(a.ACCOUNT_NAME, '')` を返し、日本語ハードコード `'指定なし'` を含まないこと (Fable-5 #22) | src/services/aggregation.rs | 2064 |
| `latent_h1_category2_null_code_goes_to_unspecified_group` | CATEGORY2_CODE が NULL の明細があっても費目2集計が成功し「指定なし」('') グループに入る（潜在監査 H1 の回帰防止） | src/services/latent_audit/aggregation.rs | 240 |
| `latent_h1_category3_null_code_goes_to_unspecified_group` | CATEGORY2/3 が NULL でも費目3集計が成功し「指定なし」グループに入る（潜在監査 H1 の回帰防止） | src/services/latent_audit/aggregation.rs | 256 |
| `latent_h1_category3_only_code3_null_does_not_fail` | CATEGORY3_CODE のみ NULL でも費目3集計が失敗しない（潜在監査 H1 の回帰防止） | src/services/latent_audit/aggregation.rs | 271 |
| `latent_h5_included_header_category2_uses_amount_including_tax` | 税込ヘッダーの費目2集計が AMOUNT_INCLUDING_TAX を使う (潜在監査 H5) | src/services/latent_audit/aggregation.rs | 438 |
| `latent_l11_weekly_week1_covers_jan1_and_matches_iso` | 1週目が1月1日を含み、週番号が ISO 8601 (フロントの getWeekNumber) と一致 (潜在監査 L11) | src/services/latent_audit/aggregation.rs | 369 |
| `latent_l11_weekly_every_day_of_year_is_covered` | 月曜・日曜始まりとも、年内の全日がいずれかの週 (1..=53) に入る (潜在監査 L11) | src/services/latent_audit/aggregation.rs | 404 |
| `latent_l11_week_53_and_sunday_start_follow_iso_weeks` | 52 週の年の 53 週目は拒否、日曜始まりの週は ISO 週の前日から始まる (潜在監査 L11) | src/services/latent_audit/aggregation.rs | 425 |
| `latent_m10_detailless_header_counted_in_detail_groupings` | 明細のないヘッダーも中分類・小分類・商品別に集計され、それぞれ大分類の合計と一致する (潜在監査 M10) | src/services/latent_audit/aggregation.rs | 300 |
| `latent_m10_detailless_group_key_and_name` | 明細なしグループのキーは `<大分類>/__NO_DETAILS__` (商品別は `__NO_DETAILS__`)、名前は `aggregation.no_details` (潜在監査 M10) | src/services/latent_audit/aggregation.rs | 330 |
| `latent_l9_category2_rounding_drift_is_bounded` | 仕様として受容: 1 取引を複数の中分類に分けると、大分類との差は税率ごとに (グループ数 − 1) 円以内 (潜在監査 L9) | src/services/latent_audit/aggregation.rs | 374 |

**合計**: 26件

### services/period.rs

月次・年次の期間境界の計算。

| テスト関数 | 説明 | ファイル | 行 |
|-----------|------|---------|-----|
| `latent_l10_monthly_bounds_rejects_out_of_range_year_without_shift` | 休日シフトなしの月次境界は範囲外の年でパニックせず Err (潜在監査 L10) | src/services/latent_audit/period.rs | 39 |
| `latent_l10_monthly_bounds_rejects_out_of_range_year_with_shift` | 休日シフトありの月次境界も範囲外の年でパニックせず Err (潜在監査 L10) | src/services/latent_audit/period.rs | 55 |
| `latent_l10_period_helpers_return_none_on_out_of_range_year` | 期間計算の関数は表現できない日付でパニックせず None を返す (潜在監査 L10) | src/services/latent_audit/period.rs | 70 |

**合計**: 3件

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
| `latent_h2_cascade_delete_keeps_confirmed_headers` | ルールのカスケード削除は未確定の予定取引だけを消し、確定済み (IS_SCHEDULED = 0) は残して紐付けを外す (潜在監査 H2) | src/services/latent_audit/recurring.rs | 195 |
| `latent_m16_transfer_same_account_rejected` | 出金元と入金先が同じ振替ルールの作成を拒否 (潜在監査 M16) | src/services/latent_audit/recurring.rs | 334 |
| `latent_m16_tax_rounding_type_out_of_range_rejected` | 範囲外の端数処理種別でのルール作成を拒否 (潜在監査 M16) | src/services/latent_audit/recurring.rs | 356 |
| `latent_m16_tax_included_type_out_of_range_rejected` | 範囲外の内税/外税種別でのルール作成を拒否 (潜在監査 M16) | src/services/latent_audit/recurring.rs | 377 |
| `latent_m15_holiday_shift_applies_beyond_seeded_range` | 祝日データの範囲外の年にかかるルールは、休日シフトを黙って飛ばさずに拒否される (`PeriodOutOfRange`) (潜在監査 M15) | src/services/latent_audit/recurring.rs | 251 |
| `latent_m18_huge_generation_rejected` | 9999-12-31 までの毎日ルールは数百万件を生成せずに拒否される (`PeriodOutOfRange`) (潜在監査 M18) | src/services/latent_audit/recurring.rs | 448 |
| `latent_m15_m18_period_limits_are_inclusive` | 許可範囲の初日・最終日は受け付け、1 日外れると拒否 (潜在監査 M15 / M18) | src/services/latent_audit/recurring.rs | 478 |
| `latent_m15_period_limit_follows_seeded_holidays` | 祝日が日付基準の上限の前年までしか入っていない場合 (起動したまま年をまたいだ場合)、最後の 1 年は拒否される (潜在監査 M15) | src/services/latent_audit/recurring.rs | 513 |
| `latent_m15_m18_period_limits_service_clamps_to_seeded_years` | `RecurringService::period_limits` (`get_recurring_period_limits` で画面に渡し、作成時にも適用) は日付基準の範囲を祝日シード済みの年で絞ったもの (潜在監査 M15 / M18) | src/services/latent_audit/recurring.rs | 548 |
| `latent_m15_m18_period_limits_follow_seeded_years` | 上下限は (今年 − 5) 年 1/1 〜 (今年 + 10) 年 12/31 で、祝日シードの範囲と一致 (潜在監査 M15 / M18) | src/services/latent_audit/recurring.rs | 575 |
| `latent_m17_total_is_derived_from_the_detail` | ルールと各予定の合計は、1 件の明細からヘッダーの丸め・内税/外税設定で計算される (潜在監査 M17) | src/services/latent_audit/recurring.rs | 591 |
| `latent_l13_daily_rule_rejects_holiday_shift` | 毎日のルールに祝日シフトは指定できない。シフトなしの毎日・シフトありの毎月は受け付ける (潜在監査 L13) | src/services/latent_audit/recurring.rs | 647 |
| `latent_m16_detail_amount_out_of_range_rejected` | 範囲外の明細金額でのルール作成を拒否 (潜在監査 M16) | src/services/latent_audit/recurring.rs | 398 |
| `latent_m16_detail_tax_rate_out_of_range_rejected` | 範囲外の税率でのルール作成を拒否 (潜在監査 M16) | src/services/latent_audit/recurring.rs | 419 |
| `transfer_same_account_maps_to_transfer_same_account_code` | `RecurringError::TransferSameAccount` が `transfer_same_account` コードに変換される (潜在監査 M16) | src/services/recurring.rs | 1881 |
| `latent_l2_recurring_rejects_foreign_shop_and_product` | 繰り返しルール作成で自分の店舗・商品は受理し、他ユーザーのものは拒否 (潜在監査 L2) | src/services/latent_audit/recurring.rs | 688 |
| `latent_l10_generation_terminates_at_the_end_of_the_date_range` | chrono の表現範囲の終わり付近でも月次・年次の日付生成が無限ループせず終了する (潜在監査 L10) | src/services/latent_audit/recurring.rs | 747 |

**合計**: 22件

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
| **インラインテスト** | **440件** |
| validation.rs | 25 |
| security.rs | 13 |
| crypto.rs | 15 |
| db.rs | 16 |
| settings.rs | 18 |
| api_error.rs | 10 |
| services/master_data.rs | 4 |
| services/like_escape.rs | 7 |
| services/auth.rs | 22 |
| services/user_management.rs | 23 |
| services/encryption.rs | 11 |
| services/account.rs | 19 |
| services/category.rs | 38 |
| services/manufacturer.rs | 17 |
| services/product.rs | 20 |
| services/shop.rs | 19 |
| services/transaction.rs | 64 |
| services/aggregation.rs | 26 |
| services/period.rs | 3 |
| services/session.rs | 9 |
| services/i18n.rs | 8 |
| services/recurring.rs | 22 |
| lib.rs | 6 |
| **総計** | **466件** |

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
