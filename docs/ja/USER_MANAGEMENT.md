# ユーザ管理機能実装ドキュメント

## 概要
ユーザ管理画面用のバックエンド機能を実装しました。一般ユーザと管理者ユーザの作成、編集、削除が可能です。

## 実装ファイル

### 1. `src/services/user_management.rs`
ユーザ管理サービスの実装

#### 構造体
- `UserInfo`: ユーザ情報（ID、名前、ロール、登録日時、更新日時）
- `UserManagementError`: エラー型
- `UserManagementService`: ユーザ管理サービス

#### 主要メソッド

##### `list_users()`
- 全ユーザ一覧を取得
- 戻り値: `Vec<UserInfo>`

##### `get_user(user_id: i64)`
- 指定したIDのユーザ情報を取得
- 戻り値: `UserInfo`
- エラー: `UserNotFound`

##### `register_general_user(username: &str, password: &str)`
- 一般ユーザを登録
- パスワードは自動的にArgon2でハッシュ化
- ユーザIDは自動採番
- 重複チェックあり
- 戻り値: 新規作成されたユーザID

##### `update_general_user(user_id: i64, new_username: Option<&str>, new_password: Option<&str>)`
- 一般ユーザ（ROLE_USER）の情報を更新
- ユーザ名とパスワードを個別に更新可能
- パスワード変更時は再暗号化処理を呼び出し（現在はプレースホルダー）
- ロール検証あり（一般ユーザのみ）

##### `update_admin_user(user_id: i64, new_username: Option<&str>, new_password: Option<&str>)`
- 管理者ユーザ（ROLE_ADMIN）の情報を更新
- ユーザ名とパスワードを個別に更新可能
- パスワード変更時は再暗号化処理を呼び出し（現在はプレースホルダー）
- ロール検証あり（管理者のみ）

##### `delete_general_user(user_id: i64)`
- 一般ユーザを削除
- 管理者ユーザは削除不可
- エラー: `AdminUserCannotBeDeleted`, `InvalidRole`

##### `re_encrypt_user_data(user_id: i64, new_password: &str)` (private)
- パスワード変更時の暗号化フィールド再暗号化（TODO）
- 将来的に暗号化フィールドが追加された際に実装
- 手順:
  1. ユーザの全暗号化データを取得
  2. 旧パスワード由来の鍵で復号
  3. 新パスワード由来の鍵で再暗号化
  4. データベースを更新

### 2. `src/lib.rs` の変更

#### AppState構造体
```rust
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub auth: Arc<Mutex<AuthService>>,
    pub user_mgmt: Arc<Mutex<UserManagementService>>,  // 追加
}
```

#### 新規Tauriコマンド

##### `list_users()`
- 全ユーザ一覧をJSON形式で返す
- フロントエンド呼び出し: `invoke('list_users')`

##### `get_user(user_id: i64)`
- 指定したユーザ情報をJSON形式で返す
- フロントエンド呼び出し: `invoke('get_user', { user_id: 1 })`

##### `create_general_user(username: String, password: String)`
- 一般ユーザを作成
- パスワードバリデーション実施
- 戻り値: 新規ユーザID
- フロントエンド呼び出し: `invoke('create_general_user', { username: 'user1', password: 'password123' })`

##### `update_general_user_info(user_id: i64, username: Option<String>, password: Option<String>)`
- 一般ユーザ情報を更新
- ユーザ名とパスワードの両方またはいずれかを更新
- パスワードバリデーション実施（パスワード指定時）
- フロントエンド呼び出し: `invoke('update_general_user_info', { user_id: 2, username: 'newname', password: null })`

##### `update_admin_user_info(user_id: i64, username: Option<String>, password: Option<String>)`
- 管理者ユーザ情報を更新
- ユーザ名とパスワードの両方またはいずれかを更新
- パスワードバリデーション実施（パスワード指定時）
- フロントエンド呼び出し: `invoke('update_admin_user_info', { user_id: 1, username: null, password: 'newpassword' })`

##### `delete_general_user_info(user_id: i64)`
- 一般ユーザを削除
- 管理者ユーザは削除不可
- フロントエンド呼び出し: `invoke('delete_general_user_info', { user_id: 2 })`

## エラーハンドリング

### UserManagementError列挙型
- `DatabaseError`: データベースエラー
- `SecurityError`: セキュリティエラー（パスワードハッシュ失敗等）
- `UserNotFound`: ユーザが見つからない
- `AdminUserCannotBeDeleted`: 管理者ユーザは削除不可
- `InvalidRole`: 不正なロール（一般ユーザ専用操作を管理者に実行等）
- `DuplicateUsername`: ユーザ名重複

## セキュリティ機能

### パスワード管理
- パスワードはArgon2でハッシュ化して保存
- パスワード検証は`validation`モジュールで実施
  - 最小15文字、最大128文字
  - 空白のみ不可

### ロール検証
- 一般ユーザ操作は`ROLE_USER`のみ
- 管理者操作は`ROLE_ADMIN`のみ
- 管理者ユーザの削除は禁止

### 重複チェック
- ユーザ名の重複を登録時・更新時に検証

## テスト

### 実装されたテスト（7件、全てパス）
1. `test_register_general_user`: 一般ユーザ登録
2. `test_update_general_user`: 一般ユーザ更新（ユーザ名とパスワード）
3. `test_update_admin_user`: 管理者ユーザ更新
4. `test_delete_general_user`: 一般ユーザ削除
5. `test_cannot_delete_admin_user`: 管理者削除不可の検証
6. `test_duplicate_username`: ユーザ名重複エラー
7. `test_list_users`: ユーザ一覧取得

### テスト実行方法
```bash
cargo test services::user_management::tests --lib
```

## 今後の実装予定

### 暗号化フィールドの再暗号化
パスワード変更時に、ユーザに紐づく暗号化データを再暗号化する機能を実装予定：
1. 暗号化テーブルのスキーマ追加
2. `derive_encryption_key`を使用して暗号化キー生成
3. `re_encrypt_user_data`メソッドの実装
4. トランザクション処理の追加

### フロントエンド実装
- ユーザ一覧画面
- ユーザ作成フォーム
- ユーザ編集フォーム
- 削除確認ダイアログ

## 使用例

### フロントエンド（JavaScript）
```javascript
// ユーザ一覧取得
const users = await invoke('list_users');
console.log(users); // [{ user_id: 1, name: "admin", role: 0, ... }]

// 一般ユーザ作成
const userId = await invoke('create_general_user', {
  username: 'john',
  password: 'secure_password_123'
});

// ユーザ名のみ更新
await invoke('update_general_user_info', {
  user_id: userId,
  username: 'john_updated',
  password: null
});

// パスワードのみ更新
await invoke('update_general_user_info', {
  user_id: userId,
  username: null,
  password: 'new_secure_password'
});

// 管理者ユーザのパスワード更新
await invoke('update_admin_user_info', {
  user_id: 1,
  username: null,
  password: 'new_admin_password'
});

// ユーザ削除
await invoke('delete_general_user_info', { user_id: userId });
```

## データベーススキーマ（参考）
```sql
CREATE TABLE IF NOT EXISTS USERS (
    USER_ID INTEGER NOT NULL,
    NAME VARCHAR(128) NOT NULL UNIQUE,
    PAW VARCHAR(128) NOT NULL,  -- Argon2ハッシュ
    ROLE INTEGER NOT NULL,       -- 0: ADMIN, 1: USER, 999: VISIT
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID)
);
```

## まとめ
ユーザ管理の4つの主要機能を実装しました：
1. ✅ 一般ユーザ登録
2. ✅ 一般ユーザ編集（ユーザ名・パスワード）
3. ✅ 一般ユーザ削除
4. ✅ 管理者ユーザ編集（ユーザ名・パスワード）

パスワード変更時の暗号化フィールド再暗号化機能は、暗号化フィールドが追加された際に実装する設計となっています。
