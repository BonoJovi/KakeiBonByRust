# 認証・セットアップAPI リファレンス

**最終更新**: 2025-12-05 01:31 JST

## 概要

本ドキュメントは、ログイン画面（index.html）で使用される認証・セットアップAPIの仕様を定義します。

---

## 目次

1. [セットアップ確認API](#セットアップ確認api)
2. [ユーザー登録API](#ユーザー登録api)
3. [ログインAPI](#ログインapi)
4. [データ構造](#データ構造)

---

## セットアップ確認API

### check_needs_setup

管理者ユーザーが存在するかチェックします（初回セットアップ判定）。

**パラメータ:** なし

**戻り値:**
- `bool`: `true` - 管理者が未登録（初回セットアップが必要）
- `bool`: `false` - 管理者が登録済み

**使用例:**
```javascript
const needsSetup = await invoke('check_needs_setup');
if (needsSetup) {
    // 管理者登録フォームを表示
    showAdminRegistrationForm();
} else {
    // ログインフォームを表示
    showLoginForm();
}
```

**実装詳細:**
- `has_users()` の結果を反転して返却
- データベースにユーザーが1人もいない場合に`true`

---

### check_needs_user_setup

一般ユーザーが存在するかチェックします。

**パラメータ:** なし

**戻り値:**
- `bool`: `true` - 一般ユーザーが未登録
- `bool`: `false` - 一般ユーザーが登録済み

**使用例:**
```javascript
const needsUserSetup = await invoke('check_needs_user_setup');
if (needsUserSetup) {
    alert('一般ユーザーを作成してください');
    // ユーザー管理画面へ遷移
}
```

**実装詳細:**
- `has_general_users()` の結果を反転して返却
- 管理者のみ存在し、一般ユーザーがいない場合に`true`

**用途:**
- 管理者ログイン後に一般ユーザー作成を促す

---

## ユーザー登録API

### register_admin

管理者ユーザーを登録します（初回セットアップ時のみ）。

**パラメータ:**
- `username` (String): ユーザー名
- `password` (String): パスワード（最小16文字）

**戻り値:**
- `String`: "Admin user registered successfully"

**バリデーション:**
1. パスワードが16文字以上
2. データベースに管理者が存在しない

**使用例:**
```javascript
try {
    await invoke('register_admin', {
        username: 'admin',
        password: 'SecurePassword123456'
    });
    alert('管理者登録完了');
    location.reload(); // ログイン画面へ
} catch (error) {
    alert(`登録失敗: ${error}`);
}
```

**エラーパターン:**
- `"Password must be at least 16 characters long"`: パスワードが短い
- `"Admin user already exists"`: 既に管理者が登録済み
- `"Registration failed: ..."`: データベースエラー

**セキュリティ:**
- パスワードはArgon2でハッシュ化
- ソルトは自動生成
- ハッシュ化されたパスワードのみDB保存

---

### register_user

一般ユーザーを登録します。

**パラメータ:**
- `username` (String): ユーザー名
- `password` (String): パスワード（最小16文字）

**戻り値:**
- `String`: "User registered successfully"

**バリデーション:**
1. パスワードが16文字以上
2. ユーザー名の重複チェック

**使用例:**
```javascript
try {
    await invoke('register_user', {
        username: 'user01',
        password: 'UserPassword123456'
    });
    alert('ユーザー登録完了');
} catch (error) {
    alert(`登録失敗: ${error}`);
}
```

**注意:**
- このコマンドは認証なしで呼び出し可能（初回セットアップ用）
- 本番運用ではユーザー管理画面（`create_general_user`）を使用

---

## ログインAPI

### login_user

ユーザー名とパスワードで認証し、セッションを開始します。

**パラメータ:**
- `username` (String): ユーザー名
- `password` (String): パスワード

**戻り値:**
- `User`: ログイン成功時のユーザー情報

**User構造:**
```rust
pub struct User {
    pub user_id: i64,
    pub name: String,
    pub role: i64,  // 0: Admin, 1: General User
}
```

**使用例:**
```javascript
try {
    const user = await invoke('login_user', {
        username: 'admin',
        password: 'MyPassword123456'
    });
    
    console.log(`ログイン成功: ${user.name}`);
    
    // ロールに応じて遷移先を変更
    if (user.role === 0) {
        // 管理者
        window.location.href = 'user-management.html';
    } else {
        // 一般ユーザー
        window.location.href = 'transaction-management.html';
    }
} catch (error) {
    alert(`ログイン失敗: ${error}`);
}
```

**エラーパターン:**
- `"Invalid username or password"`: 認証失敗
- `"Authentication error: ..."`: データベースエラー

**セッション:**
- ログイン成功時に`SessionState`にユーザー情報を保存
- `get_current_session_user`で取得可能
- `clear_session`でログアウト

---

## データ構造

### User

```rust
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub user_id: i64,      // ユーザーID
    pub name: String,      // ユーザー名
    pub role: i64,         // 0: Admin, 1: General User
}
```

**ロール定数:**
- `ROLE_ADMIN = 0`: 管理者（全機能アクセス可能）
- `ROLE_USER = 1`: 一般ユーザー（自分のデータのみアクセス可能）

---

## 認証フロー

### 初回セットアップフロー

```
1. アプリ起動
   ↓
2. check_needs_setup() → true
   ↓
3. 管理者登録フォーム表示
   ↓
4. register_admin(username, password)
   ↓
5. 登録成功 → ログイン画面へリロード
```

### 通常ログインフロー

```
1. ログイン画面表示
   ↓
2. check_needs_setup() → false
   ↓
3. ユーザー名・パスワード入力
   ↓
4. login_user(username, password)
   ↓
5. 認証成功 → セッション開始
   ↓
6. ロールに応じた画面へ遷移
```

### 一般ユーザー作成促進フロー

```
1. 管理者ログイン成功
   ↓
2. check_needs_user_setup() → true
   ↓
3. 「一般ユーザーを作成してください」メッセージ表示
   ↓
4. ユーザー管理画面へ遷移
```

---

## エラーハンドリング

### 共通エラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"Password must be at least 16 characters long"` | パスワードが短い | 16文字以上に変更 |
| `"Invalid username or password"` | 認証失敗 | 正しい認証情報を入力 |
| `"Admin user already exists"` | 管理者重複 | ログイン画面を表示 |
| `"Registration failed: ..."` | DB登録エラー | データベース確認 |
| `"Authentication error: ..."` | DB接続エラー | データベース確認 |
| `"Database error: ..."` | DB接続エラー | データベース確認 |

### フロントエンドでのエラーハンドリング例

```javascript
// 管理者登録
async function setupAdmin(username, password) {
    try {
        // クライアント側バリデーション
        if (password.length < 16) {
            throw new Error('パスワードは16文字以上必要です');
        }
        
        // サーバー側バリデーション
        await invoke('validate_password_frontend', { password });
        
        // 登録実行
        await invoke('register_admin', { username, password });
        
        alert('管理者登録が完了しました');
        location.reload();
    } catch (error) {
        alert(`エラー: ${error}`);
        console.error('Admin setup error:', error);
    }
}

// ログイン
async function login(username, password) {
    try {
        const user = await invoke('login_user', { username, password });
        
        // セッション確認
        const isAuth = await invoke('is_session_authenticated');
        console.log('Session authenticated:', isAuth);
        
        // 遷移先決定
        const redirectUrl = user.role === 0 
            ? 'user-management.html' 
            : 'transaction-management.html';
        
        window.location.href = redirectUrl;
    } catch (error) {
        alert(`ログイン失敗: ${error}`);
        console.error('Login error:', error);
    }
}
```

---

## セキュリティ考慮事項

### パスワード管理

1. **最小長**: 16文字（`MIN_PASSWORD_LENGTH`定数）
2. **ハッシュ化**: Argon2（推奨設定）
3. **ソルト**: ユーザーごとに自動生成
4. **保存**: ハッシュ化済みパスワードのみDB保存

### 認証

1. **セッション管理**: インメモリ（`SessionState`）
2. **タイムアウト**: なし（アプリケーション単位で実装が必要）
3. **多重ログイン**: 同一ユーザーの多重ログインは許可

### 初回セットアップ

1. **管理者登録**: `check_needs_setup`で既存ユーザーを確認
2. **上書き防止**: 既に管理者がいる場合はエラー
3. **ロールの固定**: 初回セットアップでは必ず管理者（ROLE_ADMIN）

---

## テストカバレッジ

**認証サービス（AuthService）:**
- ✅ ユーザー登録テスト
- ✅ 認証テスト（成功・失敗）
- ✅ ユーザー存在確認テスト
- ✅ パスワードハッシュ検証テスト

**バリデーション:**
- ✅ パスワード長チェック
- ✅ Unicode文字対応

**セッション:**
- ✅ ログイン後のセッション確認
- ✅ ログアウト後のセッションクリア

---

## 画面実装例（index.html）

### 基本HTML構造

```html
<!-- 初回セットアップフォーム -->
<div id="setup-form" style="display: none;">
    <h2>管理者登録</h2>
    <input type="text" id="admin-username" placeholder="ユーザー名">
    <input type="password" id="admin-password" placeholder="パスワード（16文字以上）">
    <button onclick="setupAdmin()">登録</button>
</div>

<!-- ログインフォーム -->
<div id="login-form" style="display: none;">
    <h2>ログイン</h2>
    <input type="text" id="login-username" placeholder="ユーザー名">
    <input type="password" id="login-password" placeholder="パスワード">
    <button onclick="login()">ログイン</button>
</div>
```

### 初期化スクリプト

```javascript
async function initializeLoginPage() {
    try {
        const needsSetup = await invoke('check_needs_setup');
        
        if (needsSetup) {
            document.getElementById('setup-form').style.display = 'block';
            document.getElementById('login-form').style.display = 'none';
        } else {
            document.getElementById('setup-form').style.display = 'none';
            document.getElementById('login-form').style.display = 'block';
        }
    } catch (error) {
        console.error('Initialization error:', error);
        alert('初期化エラーが発生しました');
    }
}

document.addEventListener('DOMContentLoaded', initializeLoginPage);
```

---

## 関連ドキュメント

### 実装ファイル

- 認証サービス: `src/services/auth.rs`
- セッション管理: `src/services/session.rs`
- バリデーション: `src/validation.rs`
- セキュリティ: `src/security.rs`（Argon2ハッシュ化）
- Tauri Commands: `src/lib.rs`

### その他のAPIリファレンス

- [共通API](./API_COMMON.md) - セッション管理、バリデーション
- [ユーザー管理API](./API_USER.md) - 管理者による一般ユーザー管理

---

**変更履歴:**
- 2025-12-05: 初版作成（実装コードに基づく）
