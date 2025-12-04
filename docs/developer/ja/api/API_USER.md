# ユーザー管理API リファレンス

**最終更新**: 2025-12-05 01:42 JST

## 概要

本ドキュメントは、ユーザー管理画面（user-management.html）で使用されるAPIの仕様を定義します。管理者が一般ユーザーを管理し、ユーザー自身が自分の情報を編集する機能を提供します。

---

## 目次

1. [ユーザー一覧・取得API](#ユーザー一覧取得api)
2. [ユーザー作成API](#ユーザー作成api)
3. [ユーザー更新API](#ユーザー更新api)
4. [ユーザー削除API](#ユーザー削除api)
5. [ユーザー設定API](#ユーザー設定api)
6. [データ構造](#データ構造)

---

## ユーザー一覧・取得API

### list_users

すべてのユーザーの一覧を取得します。

**パラメータ:** なし

**戻り値:**
- `Vec<UserInfo>`: ユーザー情報の配列

**UserInfo構造:**
```javascript
{
    user_id: number,
    name: string,
    role: number,       // 0: Admin, 1: General User
    entry_dt: string,   // "YYYY-MM-DD HH:MM:SS"
    update_dt: string | null
}
```

**使用例:**
```javascript
const users = await invoke('list_users');
users.forEach(user => {
    console.log(`${user.name} (${user.role === 0 ? 'Admin' : 'User'})`);
});
```

**権限:**
- 管理者のみ実行可能（セッションチェック推奨）

---

### get_user

特定のユーザー情報を取得します。

**パラメータ:**
- `user_id` (i64): ユーザーID

**戻り値:**
- `UserInfo`: ユーザー情報

**使用例:**
```javascript
const user = await invoke('get_user', { userId: 2 });
console.log(user.name);
```

**注意:**
- 管理者が他のユーザーを照会する場合に使用
- 自分自身の情報を取得する場合は`get_current_session_user`を推奨

**エラー:**
- `"Failed to get user: User not found"`: ユーザーが存在しない

---

## ユーザー作成API

### create_general_user

新しい一般ユーザーを作成します。

**パラメータ:**
- `username` (String): ユーザー名
- `password` (String): パスワード（最小16文字）

**戻り値:**
- `i64`: 作成されたユーザーID

**使用例:**
```javascript
try {
    const userId = await invoke('create_general_user', {
        username: 'user01',
        password: 'SecurePassword123456'
    });
    console.log(`ユーザーID ${userId} を作成しました`);
    
    // ユーザー一覧を再読み込み
    await loadUsers();
} catch (error) {
    alert(`ユーザー作成失敗: ${error}`);
}
```

**自動処理:**
1. パスワードのArgon2ハッシュ化
2. 暗号化キーの生成
3. デフォルトカテゴリの投入（20中分類、126小分類）
4. ロールは自動的に一般ユーザー（ROLE_USER = 1）

**バリデーション:**
- パスワードが16文字以上
- ユーザー名の重複チェック

**エラー:**
- `"Password must be at least 16 characters long"`: パスワードが短い
- `"Username already exists"`: ユーザー名が重複
- `"Failed to create user: ..."`: データベースエラー

---

## ユーザー更新API

### update_general_user_info

ログイン中の一般ユーザーが自分の情報を更新します（パスワード変更なし）。

**パラメータ:**
- `username` (Option\<String\>): 新しいユーザー名（変更する場合）
- `password` (Option\<String\>): 新しいパスワード（変更する場合）

**戻り値:** なし

**使用例:**
```javascript
// ユーザー名のみ変更
await invoke('update_general_user_info', {
    username: 'new_username',
    password: null
});

// パスワードのみ変更
await invoke('update_general_user_info', {
    username: null,
    password: 'NewPassword123456'
});
```

**注意:**
- セッションユーザーIDを自動取得（`get_session_user_id`）
- パスワード変更時、暗号化データは**再暗号化されません**
- 暗号化データがある場合は`update_general_user_with_reencryption`を使用

**権限:**
- 一般ユーザーが自分自身を更新

---

### update_general_user_with_reencryption

パスワード変更と同時に暗号化データを再暗号化します。

**パラメータ:**
- `old_password` (String): 現在のパスワード（認証用）
- `username` (Option\<String\>): 新しいユーザー名
- `new_password` (Option\<String\>): 新しいパスワード

**戻り値:** なし

**使用例:**
```javascript
try {
    await invoke('update_general_user_with_reencryption', {
        oldPassword: 'CurrentPassword123456',
        username: null,
        newPassword: 'NewPassword123456'
    });
    alert('パスワードを変更しました。暗号化データを再暗号化しました。');
} catch (error) {
    alert(`更新失敗: ${error}`);
}
```

**処理フロー:**
1. 現在のパスワードで認証
2. 暗号化データを復号化（旧パスワード）
3. ユーザー情報を更新
4. 暗号化データを再暗号化（新パスワード）

**エラー:**
- `"Invalid password"`: 現在のパスワードが間違っている
- `"Password must be at least 16 characters long"`: 新パスワードが短い
- `"Failed to update user: ..."`: 更新エラー

**重要:**
- 暗号化データがある場合は必ずこのAPIを使用
- パスワード変更時のみ使用（ユーザー名のみ変更なら`update_general_user_info`）

---

### update_admin_user_info

ログイン中の管理者が自分の情報を更新します（パスワード変更なし）。

**パラメータ:**
- `username` (Option\<String\>): 新しいユーザー名
- `password` (Option\<String\>): 新しいパスワード

**戻り値:** なし

**使用例:**
```javascript
await invoke('update_admin_user_info', {
    username: 'new_admin_name',
    password: null
});
```

**注意:**
- 一般ユーザー向けの`update_general_user_info`と同様の動作
- 管理者専用（内部的にロールチェックあり）

---

### update_admin_user_with_reencryption

管理者のパスワード変更と暗号化データの再暗号化を実行します。

**パラメータ:**
- `old_password` (String): 現在のパスワード
- `username` (Option\<String\>): 新しいユーザー名
- `new_password` (Option\<String\>): 新しいパスワード

**戻り値:** なし

**使用例:**
```javascript
await invoke('update_admin_user_with_reencryption', {
    oldPassword: 'CurrentAdminPassword',
    username: null,
    newPassword: 'NewAdminPassword123'
});
```

**注意:**
- `update_general_user_with_reencryption`と同様の動作
- 管理者専用

---

## ユーザー削除API

### delete_general_user_info

一般ユーザーを削除します。

**パラメータ:**
- `user_id` (i64): 削除するユーザーID

**戻り値:** なし

**使用例:**
```javascript
if (confirm('このユーザーを削除してもよろしいですか？')) {
    try {
        await invoke('delete_general_user_info', { userId: 3 });
        alert('ユーザーを削除しました');
        await loadUsers(); // 一覧を再読み込み
    } catch (error) {
        alert(`削除失敗: ${error}`);
    }
}
```

**制約:**
- 管理者ユーザーは削除不可
- 削除は論理削除ではなく物理削除

**エラー:**
- `"Admin user cannot be deleted"`: 管理者を削除しようとした
- `"User not found"`: ユーザーが存在しない
- `"Failed to delete user: ..."`: 削除エラー

**カスケード削除:**
- ユーザーに関連するデータ（入出金、カテゴリ等）も削除される
- 外部キー制約により自動的に削除

---

## ユーザー設定API

### get_user_settings

現在のユーザー設定を取得します。

**パラメータ:** なし

**戻り値:**
```javascript
{
    language: string,    // "ja" | "en"
    font_size: string    // "small" | "medium" | "large"
}
```

**使用例:**
```javascript
const settings = await invoke('get_user_settings');
console.log(`言語: ${settings.language}, フォントサイズ: ${settings.font_size}`);
```

**デフォルト値:**
- `language`: "ja"（LANG_DEFAULT）
- `font_size`: "medium"（FONT_SIZE_DEFAULT）

---

### update_user_settings

ユーザー設定を更新します。

**パラメータ:**
- `settings` (Object): 設定キーと値のマップ

**使用例:**
```javascript
await invoke('update_user_settings', {
    settings: {
        language: 'en',
        font_size: 'large'
    }
});
```

**注意:**
- 一部の設定のみ更新可能（指定したキーのみ更新）
- 設定はアプリケーション設定ファイルに保存

---

## データ構造

### UserInfo

```rust
#[derive(Debug, Clone)]
pub struct UserInfo {
    pub user_id: i64,
    pub name: String,
    pub role: i64,          // 0: Admin, 1: General User
    pub entry_dt: String,   // "YYYY-MM-DD HH:MM:SS"
    pub update_dt: Option<String>,
}
```

**ロール定数:**
- `ROLE_ADMIN = 0`: 管理者
- `ROLE_USER = 1`: 一般ユーザー

---

## エラーハンドリング

### 共通エラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"User not authenticated"` | セッション未認証 | ログインが必要 |
| `"Password must be at least 16 characters long"` | パスワードが短い | 16文字以上に変更 |
| `"Username already exists"` | ユーザー名重複 | 別の名前を使用 |
| `"User not found"` | ユーザーが存在しない | 正しいIDを指定 |
| `"Admin user cannot be deleted"` | 管理者削除不可 | 一般ユーザーのみ削除可能 |
| `"Invalid password"` | 現在のパスワードが間違い | 正しいパスワードを入力 |
| `"Failed to ... user: ..."` | データベースエラー | データベース確認 |

### フロントエンドでのエラーハンドリング例

```javascript
// ユーザー作成
async function createUser(username, password) {
    try {
        // バリデーション
        await invoke('validate_password_frontend', { password });
        
        // 作成
        const userId = await invoke('create_general_user', { 
            username, 
            password 
        });
        
        alert(`ユーザーID ${userId} を作成しました`);
        return userId;
    } catch (error) {
        if (error.includes('already exists')) {
            alert('このユーザー名は既に使用されています');
        } else if (error.includes('16 characters')) {
            alert('パスワードは16文字以上必要です');
        } else {
            alert(`エラー: ${error}`);
        }
        return null;
    }
}

// パスワード変更（再暗号化付き）
async function changePassword(oldPassword, newPassword) {
    try {
        await invoke('update_general_user_with_reencryption', {
            oldPassword,
            username: null,
            newPassword
        });
        
        alert('パスワードを変更しました');
        return true;
    } catch (error) {
        if (error.includes('Invalid password')) {
            alert('現在のパスワードが間違っています');
        } else {
            alert(`エラー: ${error}`);
        }
        return false;
    }
}
```

---

## セキュリティ考慮事項

### パスワード管理

1. **最小長**: 16文字（MIN_PASSWORD_LENGTH）
2. **ハッシュ化**: Argon2（ソルト自動生成）
3. **保存**: ハッシュ化済みパスワードのみDB保存
4. **再暗号化**: パスワード変更時は必ず再暗号化APIを使用

### 権限管理

1. **管理者**:
   - すべてのユーザーの閲覧・作成・削除が可能
   - 自分自身の情報更新が可能
2. **一般ユーザー**:
   - 自分自身の情報更新のみ可能
   - 他ユーザーの情報は閲覧・変更不可

### 暗号化データ

1. **暗号化キー**: ユーザーパスワードから派生
2. **再暗号化**: パスワード変更時は必須
3. **対象データ**: トランザクションメモなど

**重要:** パスワード変更時に再暗号化を忘れると、暗号化データが復号不可能になります。

---

## 使用例：ユーザー管理画面の実装

### ユーザー一覧表示

```javascript
async function loadUsers() {
    try {
        const users = await invoke('list_users');
        
        const tbody = document.getElementById('user-table-body');
        tbody.innerHTML = '';
        
        users.forEach(user => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td>${user.user_id}</td>
                <td>${user.name}</td>
                <td>${user.role === 0 ? '管理者' : '一般ユーザー'}</td>
                <td>${user.entry_dt}</td>
                <td>
                    ${user.role === 1 ? `
                        <button onclick="deleteUser(${user.user_id})">削除</button>
                    ` : ''}
                </td>
            `;
            tbody.appendChild(row);
        });
    } catch (error) {
        console.error('ユーザー一覧の読み込みエラー:', error);
    }
}
```

### ユーザー作成フォーム

```javascript
async function handleCreateUser(event) {
    event.preventDefault();
    
    const username = document.getElementById('new-username').value;
    const password = document.getElementById('new-password').value;
    const confirmPassword = document.getElementById('confirm-password').value;
    
    // クライアント側バリデーション
    if (password !== confirmPassword) {
        alert('パスワードが一致しません');
        return;
    }
    
    try {
        // サーバー側バリデーション
        await invoke('validate_passwords_frontend', { 
            password, 
            passwordConfirm: confirmPassword 
        });
        
        // ユーザー作成
        const userId = await invoke('create_general_user', { 
            username, 
            password 
        });
        
        alert(`ユーザー「${username}」を作成しました（ID: ${userId}）`);
        
        // フォームリセット
        event.target.reset();
        
        // 一覧再読み込み
        await loadUsers();
    } catch (error) {
        alert(`エラー: ${error}`);
    }
}
```

---

## テストカバレッジ

**UserManagementService:**
- ✅ ユーザー一覧取得テスト
- ✅ ユーザー作成テスト
- ✅ ユーザー更新テスト（ユーザー名・パスワード）
- ✅ ユーザー削除テスト
- ✅ 重複ユーザー名チェック
- ✅ パスワード変更時の再暗号化テスト
- ✅ 管理者削除防止テスト

**バリデーション:**
- ✅ パスワード長チェック
- ✅ パスワード一致チェック

---

## 関連ドキュメント

### 実装ファイル

- ユーザー管理サービス: `src/services/user_management.rs`
- 認証サービス: `src/services/auth.rs`
- 暗号化サービス: `src/services/encryption.rs`
- セキュリティ: `src/security.rs`
- バリデーション: `src/validation.rs`
- Tauri Commands: `src/lib.rs`

### その他のAPIリファレンス

- [共通API](./API_COMMON.md) - セッション管理、バリデーション
- [認証・セットアップAPI](./API_AUTH.md) - ログイン、初回セットアップ
- [設定API](./API_SETTINGS.md) - アプリケーション設定

---

**変更履歴:**
- 2025-12-05: 初版作成（実装コードに基づく）
