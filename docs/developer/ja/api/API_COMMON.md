# 共通API リファレンス

**最終更新**: 2025-12-05 01:11 JST

## 概要

本ドキュメントは、複数の画面で共通して使用されるAPIの仕様を定義します。

---

## 目次

1. [セッション管理API](#セッション管理api)
2. [国際化 (I18n) API](#国際化-i18n-api)
3. [暗号化管理API](#暗号化管理api)
4. [システムAPI](#システムapi)
5. [バリデーションAPI](#バリデーションapi)

---

## セッション管理API

### データ構造

#### User

```rust
pub struct User {
    pub user_id: i64,
    pub name: String,
    pub role: i64,  // 0: Admin, 1: General User
}
```

#### SessionInfo

```rust
pub struct SessionInfo {
    pub source_screen: Option<String>,    // 遷移元画面名
    pub category1_code: Option<String>,   // 選択中の大分類コード
    pub modal_state: Option<String>,      // モーダルの状態
}
```

---

### get_current_session_user

現在ログイン中のユーザー情報を取得します。

**パラメータ:** なし

**戻り値:**
- `Option<User>`: ログイン中の場合はユーザー情報、未ログインの場合は`null`

**使用例:**
```javascript
const user = await invoke('get_current_session_user');
if (user) {
    console.log(`User: ${user.name}, Role: ${user.role}`);
}
```

---

### is_session_authenticated

セッションが認証済みかどうかを確認します。

**パラメータ:** なし

**戻り値:**
- `bool`: 認証済みの場合`true`、未認証の場合`false`

**使用例:**
```javascript
const isAuth = await invoke('is_session_authenticated');
if (!isAuth) {
    window.location.href = 'index.html';
}
```

---

### clear_session

セッション情報をすべてクリアします（ログアウト処理）。

**パラメータ:** なし

**戻り値:** なし

**使用例:**
```javascript
await invoke('clear_session');
window.location.href = 'index.html';
```

**注意:**
- ユーザー情報と`SessionInfo`のすべてのフィールドがクリアされます

---

### set_session_source_screen

遷移元画面名をセッションに保存します。

**パラメータ:**
- `source_screen` (String): 画面名（例: "transaction-management"）

**戻り値:** なし

**使用例:**
```javascript
await invoke('set_session_source_screen', { 
    sourceScreen: 'transaction-management' 
});
```

---

### get_session_source_screen

セッションに保存された遷移元画面名を取得します。

**パラメータ:** なし

**戻り値:**
- `Option<String>`: 画面名、未設定の場合は`null`

**使用例:**
```javascript
const sourceScreen = await invoke('get_session_source_screen');
if (sourceScreen) {
    window.location.href = `${sourceScreen}.html`;
}
```

---

### clear_session_source_screen

セッションから遷移元画面名をクリアします。

**パラメータ:** なし

**戻り値:** なし

---

### set_session_category1_code

選択中の大分類コードをセッションに保存します。

**パラメータ:**
- `category1_code` (String): 大分類コード（例: "EXPENSE"）

**戻り値:** なし

**使用例:**
```javascript
await invoke('set_session_category1_code', { 
    category1Code: 'EXPENSE' 
});
```

---

### get_session_category1_code

セッションに保存された大分類コードを取得します。

**パラメータ:** なし

**戻り値:**
- `Option<String>`: 大分類コード、未設定の場合は`null`

---

### clear_session_category1_code

セッションから大分類コードをクリアします。

**パラメータ:** なし

**戻り値:** なし

---

### set_session_modal_state

モーダルの状態をセッションに保存します。

**パラメータ:**
- `modal_state` (String): モーダル状態（例: "add", "edit"）

**戻り値:** なし

---

### get_session_modal_state

セッションに保存されたモーダルの状態を取得します。

**パラメータ:** なし

**戻り値:**
- `Option<String>`: モーダル状態、未設定の場合は`null`

---

### clear_session_modal_state

セッションからモーダルの状態をクリアします。

**パラメータ:** なし

**戻り値:** なし

---

## 国際化 (I18n) API

### get_translations

指定された言語のすべての翻訳リソースを取得します。

**パラメータ:**
- `language` (String): 言語コード（"ja", "en"など）

**戻り値:**
- `HashMap<String, String>`: キーと翻訳のマップ

**使用例:**
```javascript
const translations = await invoke('get_translations', { language: 'ja' });
console.log(translations['common.save']); // "保存"
```

---

### get_i18n_resource

指定されたキーの翻訳リソースを取得します。

**パラメータ:**
- `key` (String): リソースキー（例: "common.save"）
- `lang_code` (String): 言語コード

**戻り値:**
- `String`: 翻訳文字列

**フォールバック:**
- 指定言語に翻訳がない場合、デフォルト言語（日本語）にフォールバック
- デフォルト言語にもない場合はエラー

**使用例:**
```javascript
const label = await invoke('get_i18n_resource', { 
    key: 'common.save', 
    langCode: 'en' 
});
```

---

### get_i18n_resources_by_category

カテゴリ別に翻訳リソースを取得します。

**パラメータ:**
- `lang_code` (String): 言語コード
- `category` (String): カテゴリ名（例: "common", "user"）

**戻り値:**
- `HashMap<String, String>`: カテゴリに属するリソースのマップ

**使用例:**
```javascript
const commonResources = await invoke('get_i18n_resources_by_category', { 
    langCode: 'ja', 
    category: 'common' 
});
```

---

### get_available_languages

利用可能な言語コードの一覧を取得します。

**パラメータ:** なし

**戻り値:**
- `Vec<String>`: 言語コードの配列（例: `["ja", "en"]`）

**使用例:**
```javascript
const languages = await invoke('get_available_languages');
```

---

### get_language_names

言語コードと表示名のペアを取得します。

**パラメータ:** なし

**戻り値:**
- `Vec<(String, String)>`: `(言語コード, 表示名)`の配列

**表示名の言語:**
- 現在の設定言語に基づいて表示名を取得

**使用例:**
```javascript
const langNames = await invoke('get_language_names');
// [["ja", "日本語"], ["en", "English"]]

langNames.forEach(([code, name]) => {
    const option = document.createElement('option');
    option.value = code;
    option.textContent = name;
    selectElement.appendChild(option);
});
```

---

## 暗号化管理API

### list_encrypted_fields

データベース内の暗号化対象フィールドの一覧を取得します。

**パラメータ:**
- `user_id` (i64): ユーザーID

**戻り値:**
- `Vec<(String, String)>`: `(テーブル名, カラム名)`の配列

**使用例:**
```javascript
const fields = await invoke('list_encrypted_fields', { userId: 1 });
// [["TRANSACTION_MEMOS", "MEMO_TEXT"], ...]
```

---

### register_encrypted_field

新しい暗号化対象フィールドを登録します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `table_name` (String): テーブル名
- `column_name` (String): カラム名

**戻り値:**
- `String`: 成功メッセージ

**使用例:**
```javascript
await invoke('register_encrypted_field', {
    userId: 1,
    tableName: 'CUSTOM_TABLE',
    columnName: 'SECRET_DATA'
});
```

**注意:**
- 既に登録済みの場合はエラー

---

## システムAPI

### test_db_connection

データベース接続をテストします。

**パラメータ:** なし

**戻り値:**
- `String`: "OK" （接続成功時）

**使用例:**
```javascript
try {
    const result = await invoke('test_db_connection');
    console.log('DB接続: ', result);
} catch (error) {
    console.error('DB接続エラー:', error);
}
```

---

### adjust_window_size

ウィンドウサイズを調整します。

**パラメータ:**
- `width` (f64): 幅（ピクセル）
- `height` (f64): 高さ（ピクセル）

**戻り値:** なし

**使用例:**
```javascript
await invoke('adjust_window_size', { 
    width: 1200, 
    height: 800 
});
```

**注意:**
- デスクトップアプリケーションでのみ有効

---

### handle_quit

アプリケーションを終了します。

**パラメータ:** なし

**戻り値:** なし

**使用例:**
```javascript
await invoke('handle_quit');
```

---

## バリデーションAPI

### validate_password_frontend

パスワードの妥当性を検証します。

**パラメータ:**
- `password` (String): 検証するパスワード

**戻り値:**
- 成功時: `null`
- エラー時: エラーメッセージ

**バリデーションルール:**
- 最小16文字

**使用例:**
```javascript
try {
    await invoke('validate_password_frontend', { 
        password: 'MyPassword123456' 
    });
    console.log('パスワードは有効です');
} catch (error) {
    alert(error); // "Password must be at least 16 characters long"
}
```

---

### validate_passwords_frontend

パスワードと確認用パスワードの妥当性を検証します。

**パラメータ:**
- `password` (String): パスワード
- `password_confirm` (String): 確認用パスワード

**戻り値:**
- 成功時: `null`
- エラー時: エラーメッセージ

**バリデーションルール:**
1. パスワードが最小16文字
2. パスワードと確認用パスワードが一致

**使用例:**
```javascript
try {
    await invoke('validate_passwords_frontend', { 
        password: 'MyPassword123456',
        passwordConfirm: 'MyPassword123456'
    });
    console.log('パスワードは有効です');
} catch (error) {
    alert(error);
}
```

---

## エラーハンドリング

### 共通エラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"User not authenticated"` | セッション未認証 | ログイン画面にリダイレクト |
| `"Resource not found: {key}"` | 翻訳リソースが存在しない | リソースキーを確認 |
| `"Failed to get translations: ..."` | DB接続エラー | データベース確認 |
| `"Password must be at least 16 characters long"` | パスワードが短い | 16文字以上に変更 |
| `"Passwords do not match"` | パスワード不一致 | 入力を確認 |

### フロントエンドでのエラーハンドリング例

```javascript
// セッションチェック
async function checkSession() {
    try {
        const isAuth = await invoke('is_session_authenticated');
        if (!isAuth) {
            window.location.href = 'index.html';
            return false;
        }
        return true;
    } catch (error) {
        console.error('Session check failed:', error);
        return false;
    }
}

// 翻訳取得（フォールバック付き）
async function translate(key, fallback = key) {
    try {
        const lang = localStorage.getItem('language') || 'ja';
        return await invoke('get_i18n_resource', { 
            key, 
            langCode: lang 
        });
    } catch (error) {
        console.warn(`Translation not found for key: ${key}`);
        return fallback;
    }
}
```

---

## セキュリティ考慮事項

### セッション管理

1. **タイムアウト**: セッションに有効期限はありません（アプリケーションレベルで実装が必要）
2. **クリア**: ログアウト時は必ず`clear_session`を呼び出す
3. **認証チェック**: 各画面で`is_session_authenticated`を必ず確認

### パスワードバリデーション

1. **最小長**: 16文字（`MIN_PASSWORD_LENGTH`定数）
2. **文字種制限**: なし（Unicode対応）
3. **クライアント・サーバー両側でバリデーション実施**

### 暗号化

1. **AES-256-GCM**: データ暗号化に使用
2. **ユーザー単位**: 各ユーザーが独自の暗号化キーを保持
3. **パスワード変更時**: 暗号化データの再暗号化が必要

---

## テストカバレッジ

**セッション管理:**
- ✅ 初期化テスト
- ✅ ユーザー設定・取得テスト
- ✅ クリア操作テスト
- ✅ 複数操作の連続実行テスト

**I18n:**
- ✅ リソース取得テスト
- ✅ フォールバックテスト
- ✅ パラメータ置換テスト

**バリデーション:**
- ✅ パスワード長チェック
- ✅ パスワード一致チェック
- ✅ Unicode文字対応テスト

---

## 使用例：画面初期化の共通パターン

```javascript
// すべての画面で実行する初期化処理
async function initializePage() {
    // 1. セッションチェック
    const isAuth = await invoke('is_session_authenticated');
    if (!isAuth) {
        window.location.href = 'index.html';
        return;
    }

    // 2. ユーザー情報取得
    const user = await invoke('get_current_session_user');
    console.log(`Logged in as: ${user.name}`);

    // 3. 翻訳リソース読み込み
    const lang = localStorage.getItem('language') || 'ja';
    const translations = await invoke('get_translations', { language: lang });
    
    // 4. UI要素に翻訳を適用
    applyTranslations(translations);
    
    // 5. セッション情報を画面に反映
    const category1Code = await invoke('get_session_category1_code');
    if (category1Code) {
        // 前回選択していたカテゴリを復元
        restoreCategorySelection(category1Code);
    }
}

// ページロード時に実行
document.addEventListener('DOMContentLoaded', initializePage);
```

---

## 関連ドキュメント

### 実装ファイル

- セッション管理: `src/services/session.rs`
- 国際化: `src/services/i18n.rs`
- 暗号化: `src/services/encryption.rs`
- バリデーション: `src/validation.rs`
- Tauri Commands: `src/lib.rs`

### 画面別APIリファレンス

- [認証・セットアップAPI](./API_AUTH.md)
- [ユーザー管理API](./API_USER.md)
- [費目管理API](./API_CATEGORY.md)
- [入出金管理API](./API_TRANSACTION.md)
- [口座管理API](./API_ACCOUNT.md)
- [マスタデータ管理API](./API_MASTER_DATA.md)
- [集計API](./API_AGGREGATION.md)
- [設定API](./API_SETTINGS.md)

---

**変更履歴:**
- 2025-12-05: 初版作成（実装コードに基づく）
