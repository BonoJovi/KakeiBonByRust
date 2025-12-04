# 暗号化フィールド管理システム

## 概要
メタデータ駆動の暗号化管理システムを実装しました。暗号化管理テーブル（ENCRYPTED_FIELDS）に登録された情報を元に、パスワード変更時の自動再暗号化を実現します。

## アーキテクチャ

### メタデータ駆動設計
暗号化が必要なフィールドを`ENCRYPTED_FIELDS`テーブルで管理することで、以下のメリットがあります：

1. **柔軟性**: 新しい暗号化フィールドをコード変更なしで追加可能
2. **保守性**: 暗号化対象フィールドが一元管理され、把握しやすい
3. **拡張性**: フィールドのアクティブ/非アクティブ切り替えが容易

### データフロー
```
パスワード変更要求
    ↓
旧パスワードで認証
    ↓
ENCRYPTED_FIELDSテーブルから暗号化対象を取得
    ↓
各テーブル・フィールドを順次処理:
  - 旧パスワードで復号
  - 新パスワードで再暗号化
  - データベース更新（トランザクション）
    ↓
パスワードハッシュ更新
```

## データベーススキーマ

### ENCRYPTED_FIELDS テーブル
```sql
CREATE TABLE IF NOT EXISTS ENCRYPTED_FIELDS (
    FIELD_ID INTEGER NOT NULL,
    TABLE_NAME VARCHAR(128) NOT NULL,      -- 暗号化フィールドを持つテーブル名
    COLUMN_NAME VARCHAR(128) NOT NULL,     -- 暗号化フィールドのカラム名
    DESCRIPTION VARCHAR(256),              -- フィールドの説明
    IS_ACTIVE INTEGER NOT NULL DEFAULT 1,  -- アクティブフラグ (1=有効, 0=無効)
    ENTRY_DT DATETIME NOT NULL,
    PRIMARY KEY(FIELD_ID),
    UNIQUE(TABLE_NAME, COLUMN_NAME)
);
```

### 使用例
```sql
-- 例: メモテーブルの暗号化フィールド登録
INSERT INTO ENCRYPTED_FIELDS (FIELD_ID, TABLE_NAME, COLUMN_NAME, DESCRIPTION, IS_ACTIVE, ENTRY_DT)
VALUES (1, 'MEMOS', 'CONTENT', 'メモ本文（暗号化）', 1, datetime('now'));

INSERT INTO ENCRYPTED_FIELDS (FIELD_ID, TABLE_NAME, COLUMN_NAME, DESCRIPTION, IS_ACTIVE, ENTRY_DT)
VALUES (2, 'NOTES', 'SECRET_TEXT', '秘密メモ（暗号化）', 1, datetime('now'));
```

## 実装ファイル

### 1. `src/services/encryption.rs`
暗号化管理サービスの実装

#### 主要構造体
- `EncryptedField`: 暗号化フィールド情報
- `EncryptionError`: エラー型
- `EncryptionService`: 暗号化管理サービス

#### 主要メソッド

##### `get_encrypted_fields()`
- アクティブな暗号化フィールド一覧を取得
- 戻り値: `Vec<EncryptedField>`

##### `register_encrypted_field(table_name: &str, column_name: &str, description: Option<&str>)`
- 新しい暗号化フィールドを登録
- 戻り値: フィールドID

##### `re_encrypt_user_data(user_id: i64, old_password: &str, new_password: &str)`
- ユーザの全暗号化データを再暗号化
- トランザクション処理で一括更新
- 手順:
  1. ENCRYPTED_FIELDSから対象フィールド取得
  2. テーブルごとにグループ化
  3. 各レコードを旧パスワードで復号
  4. 新パスワードで再暗号化
  5. データベース更新（コミット）

##### `encrypt_field(user_id: i64, password: &str, plaintext: &str)`
- データを暗号化
- ユーザIDをソルトとして使用
- AES-256-GCMで暗号化

##### `decrypt_field(user_id: i64, password: &str, ciphertext: &str)`
- データを復号
- ユーザIDをソルトとして使用

### 2. `src/services/user_management.rs` の更新

#### 新規メソッド

##### `update_general_user_with_password(user_id, old_password, new_username, new_password)`
- 旧パスワード検証付きの一般ユーザ更新
- パスワード変更時に自動的に再暗号化実行
- 旧パスワードが間違っている場合はエラー

##### `update_admin_user_with_password(user_id, old_password, new_username, new_password)`
- 旧パスワード検証付きの管理者ユーザ更新
- パスワード変更時に自動的に再暗号化実行
- 旧パスワードが間違っている場合はエラー

#### 内部メソッド
- `update_user_internal()`: 再暗号化を含む内部更新処理
- `re_encrypt_user_data()`: EncryptionServiceへの委譲

### 3. `src/lib.rs` の変更

#### 新規Tauriコマンド

##### `update_general_user_with_reencryption(user_id, old_password, username, new_password)`
- 再暗号化付きの一般ユーザ更新
- フロントエンド呼び出し例:
```javascript
await invoke('update_general_user_with_reencryption', {
  user_id: 2,
  old_password: 'current_password',
  username: null,
  new_password: 'new_secure_password'
});
```

##### `update_admin_user_with_reencryption(user_id, old_password, username, new_password)`
- 再暗号化付きの管理者ユーザ更新
- フロントエンド呼び出し例:
```javascript
await invoke('update_admin_user_with_reencryption', {
  user_id: 1,
  old_password: 'admin_current_password',
  username: 'superadmin',
  new_password: 'new_admin_password'
});
```

##### `list_encrypted_fields()`
- 登録済み暗号化フィールド一覧を取得
- フロントエンド呼び出し: `invoke('list_encrypted_fields')`

##### `register_encrypted_field(table_name, column_name, description)`
- 新しい暗号化フィールドを登録
- フロントエンド呼び出し例:
```javascript
await invoke('register_encrypted_field', {
  table_name: 'MEMOS',
  column_name: 'CONTENT',
  description: 'メモ本文（暗号化）'
});
```

## セキュリティ

### 暗号化方式
- **アルゴリズム**: AES-256-GCM
- **鍵導出**: Argon2id (パスワード + ユーザID)
- **ソルト**: ユーザIDをLE bytes形式で使用

### パスワード検証
- 再暗号化前に旧パスワードを必ず検証
- 検証失敗時は処理を中断

### トランザクション管理
- 再暗号化処理は全てトランザクション内で実行
- 一つでも失敗した場合はロールバック
- データ整合性を保証

## テスト

### 実装されたテスト（4件、全てパス）
1. `test_register_encrypted_field`: 暗号化フィールド登録
2. `test_encrypt_decrypt_field`: 暗号化・復号化
3. `test_re_encrypt_user_data`: 再暗号化処理
4. `test_decrypt_with_wrong_password_fails`: 誤パスワードでの復号失敗

### テスト実行方法
```bash
# 暗号化サービスのテスト
cargo test services::encryption::tests --lib

# 全テスト
cargo test --lib
```

## 使用フロー

### 1. 暗号化フィールドの登録
```rust
// バックエンド（またはマイグレーション）
let encryption_service = EncryptionService::new(pool);
encryption_service.register_encrypted_field(
    "MEMOS",
    "CONTENT",
    Some("メモ本文")
).await?;
```

### 2. データの暗号化
```rust
let user_id = 1;
let password = "user_password";
let plaintext = "機密情報";

let encrypted = encryption_service.encrypt_field(
    user_id,
    password,
    plaintext
).await?;

// データベースに保存
sqlx::query("INSERT INTO MEMOS (USER_ID, CONTENT) VALUES (?, ?)")
    .bind(user_id)
    .bind(encrypted)
    .execute(&pool)
    .await?;
```

### 3. パスワード変更（自動再暗号化）
```rust
// ユーザがパスワードを変更
user_mgmt_service.update_general_user_with_password(
    user_id,
    "old_password",      // 旧パスワード
    None,                // ユーザ名変更なし
    Some("new_password") // 新パスワード
).await?;

// 内部で自動的に:
// 1. 旧パスワード検証
// 2. ENCRYPTED_FIELDSから対象フィールド取得
// 3. 全暗号化データを再暗号化
// 4. パスワードハッシュ更新
```

### 4. データの復号
```rust
let encrypted = "..."; // DBから取得
let decrypted = encryption_service.decrypt_field(
    user_id,
    password,
    encrypted
).await?;
```

## エラーハンドリング

### EncryptionError
- `DatabaseError`: データベースエラー
- `SecurityError`: セキュリティエラー
- `DecryptionFailed`: 復号失敗（パスワード間違いなど）
- `EncryptionFailed`: 暗号化失敗
- `NoEncryptedFields`: 暗号化フィールド未登録

## パフォーマンス考慮事項

### トランザクション
- 再暗号化は単一トランザクションで実行
- 大量データの場合はバッチ処理を検討

### テーブルグループ化
- 同じテーブルのフィールドをまとめて処理
- 不要なSELECT/UPDATEを削減

### インデックス推奨
```sql
CREATE INDEX idx_encrypted_fields_active 
ON ENCRYPTED_FIELDS(IS_ACTIVE) 
WHERE IS_ACTIVE = 1;
```

## 今後の拡張案

### 1. バッチ再暗号化
大量データの場合、バッチ処理で段階的に再暗号化

### 2. 暗号化バージョン管理
暗号化アルゴリズムのバージョン管理とマイグレーション

### 3. 監査ログ
再暗号化処理の実行履歴を記録

### 4. 非同期処理
再暗号化をバックグラウンドジョブとして実行

## まとめ

✅ 実装完了機能:
- 暗号化管理テーブル（ENCRYPTED_FIELDS）
- メタデータ駆動の暗号化フィールド管理
- パスワード変更時の自動再暗号化
- トランザクション処理による整合性保証
- 旧パスワード検証機能
- AES-256-GCM暗号化
- Argon2id鍵導出

この設計により、将来的に暗号化フィールドを追加する際も、データベースに登録するだけでシステムが自動的に再暗号化処理に組み込まれます。
