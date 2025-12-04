# コーディング規約

**対象者**: 開発者・コントリビューター  
**最終更新**: 2024-12-05 05:09 JST

---

## 目次
1. [基本原則](#基本原則)
2. [ファイル修正ガイドライン](#ファイル修正ガイドライン)
3. [Rust規約](#rust規約)
4. [JavaScript規約](#javascript規約)
5. [バリデーションルール](#バリデーションルール)
6. [SQL管理](#sql管理)
7. [データベース命名規則](#データベース命名規則)
8. [テスト規約](#テスト規約)
9. [ドキュメント規約](#ドキュメント規約)

---

## 基本原則

### 1. DRY (Don't Repeat Yourself)
共通ロジックは共通モジュールで管理します。

```rust
// ✓ Good - 共通モジュールを使用
use crate::validation::validate_password;

// ✗ Bad - ロジックの重複
fn check_password(pwd: &str) -> bool {
    pwd.len() >= 16  // 各所で同じチェックを実装
}
```

### 2. 一貫性
バックエンドとフロントエンドで同じバリデーションルールを使用します。

### 3. 型安全性
- Rustの型システムを活用
- JavaScriptでは`any`を避ける（TypeScript導入時）

### 4. 明示的エラーハンドリング
すべてのエラーを明示的に処理します。

---

## ファイル修正ガイドライン

### ❌ 禁止: ファイル全体の書き直し
- ファイル全体を書き直さない
- **例外**: 変更の性質上必要な場合のみ
- **理由**: 意図しない変更やマージコンフリクトのリスク最小化

### ✅ 推奨: 外科的編集
- **必要な部分のみを修正**
- 特定の行やブロックのみを変更
- 周囲のコードとコンテキストを保持
- **メリット**:
  - バージョン管理の変更履歴が明確
  - バグ混入リスクの低減
  - コードレビューが容易
  - マージコンフリクトの処理が簡単

---

## Rust規約

### 命名規則
- **関数**: `snake_case` (例: `verify_login`)
- **構造体**: `PascalCase` (例: `UserInfo`)
- **定数**: `UPPER_SNAKE_CASE` (例: `ROLE_ADMIN`)
- **モジュール**: `snake_case` (例: `user_management`)

### エラーハンドリング

```rust
// ✓ Good - 明示的なResult型
pub async fn get_user(id: i64) -> Result<UserInfo, UserManagementError> {
    // ...
}

// ✗ Bad - 本番コードでunwrap
pub async fn get_user(id: i64) -> UserInfo {
    db.query().await.unwrap()  // これはしない
}
```

### Tauriコマンド

```rust
// パターン: #[tauri::command] + pub async fn + Result
#[tauri::command]
pub async fn create_user(
    user_data: UserData,
) -> Result<UserInfo, String> {
    // 実装
}
```

### モジュール構成
```
src/
├── main.rs              # エントリーポイント
├── lib.rs               # Tauriコマンドエクスポート
├── consts.rs            # 定数定義
├── validation.rs        # 入力バリデーション
├── security.rs          # パスワードハッシング
├── crypto.rs            # 暗号化/復号化
├── sql_queries.rs       # SQL定義
└── services/            # ビジネスロジック
    ├── auth.rs
    ├── user_management.rs
    └── ...
```

---

## JavaScript規約

### ES Modules
インポート時は常に`.js`拡張子を含めます。

```javascript
// ✓ Good
import { validatePassword } from './validation-helpers.js';

// ✗ Bad
import { validatePassword } from './validation-helpers';
```

### Async/Await
非同期関数では常にtry/catchを使用します。

```javascript
// ✓ Good
async function loadData() {
    try {
        const data = await fetchData();
        return data;
    } catch (error) {
        console.error('Error:', error);
        throw error;
    }
}

// ✗ Bad
async function loadData() {
    const data = await fetchData();  // エラーハンドリングなし
    return data;
}
```

### 命名規則
- **関数**: `camelCase` (例: `validatePassword`)
- **クラス**: `PascalCase` (例: `UserManager`)
- **定数**: `UPPER_SNAKE_CASE` (例: `ROLE_ADMIN`)

---

## バリデーションルール

### パスワード
- **最小長**: 16文字
- バックエンド（`src/validation.rs`）とフロントエンド両方で強制

### Unicode処理
- **JavaScript**: `.length`（UTF-16コードユニット）
- **Rust**: `.len()`（UTF-8バイト）、文字数は`.chars().count()`
- **テスト**: 境界値テストはASCII、文字サポートテストはUnicode

### 共通バリデーション
```rust
// src/validation.rs
pub const MIN_PASSWORD_LENGTH: usize = 16;

pub fn validate_password(password: &str) -> ValidationResult {
    if password.chars().count() < MIN_PASSWORD_LENGTH {
        return ValidationResult::Invalid("Password too short".to_string());
    }
    ValidationResult::Valid
}
```

```javascript
// res/js/validation-helpers.js
export const MIN_PASSWORD_LENGTH = 16;

export function validatePassword(password) {
    if (password.length < MIN_PASSWORD_LENGTH) {
        return { valid: false, error: 'Password too short' };
    }
    return { valid: true };
}
```

---

## SQL管理

### SQL集約化規則

#### ❌ 禁止
- SQL文字列をサービス/コマンドコードに直接埋め込む

#### ✅ 必須
- **すべてのSQLクエリ**: `src/sql_queries.rs`で定数として定義
- **命名規則**:
  - 本番クエリ: `{SCOPE}_{ACTION}` (例: `CATEGORY2_UPDATE`, `USER_INSERT`)
  - テストクエリ: `TEST_`プレフィックス (例: `TEST_CATEGORY_GET_CATEGORY2_NAME`)

#### メリット
- SQL の単一情報源
- メンテナンスとテストの容易化
- 一貫したパラメータ化によるSQLインジェクション防止
- SQLレビューと最適化の改善

#### 例

```rust
// src/sql_queries.rs
pub const CATEGORY2_UPDATE: &str = r#"
UPDATE CATEGORY2 
SET CATEGORY2_NAME = ?, UPDATE_DT = datetime('now') 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ?
"#;

// サービスコード内
use crate::sql_queries::CATEGORY2_UPDATE;
conn.execute(CATEGORY2_UPDATE, params![name, user_id, cat1, cat2])?;
```

---

## データベース命名規則

### ❌ 禁止: "kakeibo.db"の使用
- `kakeibo.db`をデータベースファイル名として**使用しない**
- **正しいファイル名**: `KakeiBonDB.sqlite3`（`src/consts.rs`で定義）
- **保存場所**: `~/.kakeibon/KakeiBonDB.sqlite3`
- **アクセス**: `./db.sh`スクリプトを使用
- **理由**: 不正確なデータベース名は混乱とデータ不整合を引き起こす

### データベースアクセス

```bash
# ✓ 正解 - db.shスクリプトを使用
./db.sh "SELECT * FROM USERS;"

# ✗ 間違い - ファイル名を手動指定しない
sqlite3 work/kakeibo.db "SELECT * FROM USERS;"
sqlite3 ~/.local/share/kakeibo/kakeibo.db "SELECT * FROM USERS;"
```

---

## テスト規約

### テストパターン
```
"should [expected behavior] when [condition]"
```

### 構造（AAA）
1. **Arrange**: テストデータ準備
2. **Act**: テスト対象実行
3. **Assert**: 結果検証

### 共通モジュールの再利用
- フロントエンド: `*-validation-tests.js`からテストスイートをインポート
- バックエンド: `src/test_helpers.rs`と`src/validation_tests.rs`を使用

### テストカウント
- Rustバックエンド: 201テスト
- JavaScriptフロントエンド: 手動テスト（ランナー未設定）
- **合計**: 201+テスト

---

## ドキュメント規約

### タイムスタンプ
- **ユーザー向けドキュメント**: 日本標準時（JST, UTC+9）を常に使用
- **AIコンテキストドキュメント**: UTCまたはJST可
- **例**: `最終更新: 2024-10-26 13:21 JST`

### 言語
- **ユーザー向けドキュメント**: 日本語と英語の両方必須
- **AI/LLM向けドキュメント**: 英語を推奨

### 構造
```
# タイトル

**対象者**: [対象読者]
**最終更新**: [日時 JST]

## 目次
[...]

## セクション
[...]
```

---

## 重要な定数

| 定数 | 値 | 場所 |
|------|-----|------|
| ROLE_ADMIN | 0 | `src/consts.rs`, `res/js/consts.js` |
| ROLE_USER | 1 | `src/consts.rs`, `res/js/consts.js` |
| MIN_PASSWORD_LENGTH | 16 | `src/validation.rs` |
| DATABASE_FILENAME | KakeiBonDB.sqlite3 | `src/consts.rs` |

---

## アンチパターン

### ❌ してはいけないこと
- バリデーションロジックをファイル間で重複させる
- 本番Rustコードで`unwrap()`を使用
- エラーハンドリングを無視
- 関心事を混在させる（例: バリデーション関数内にUIロジック）
- 文字列のハードコード（i18nを使用）
- パスワードやシークレットをコミット

### ✅ すべきこと
- 共有ロジックに共通モジュールを使用
- すべてのエラーを明示的に処理
- 関心事を分離（UI、ロジック、データ）
- すべてのユーザー向けテキストにi18nを使用
- 外科的編集を行う（ファイル全体を書き直さない）
- SQLクエリを`sql_queries.rs`に集約

---

**関連ドキュメント**:
- [開発環境セットアップ](./DEVELOPMENT_SETUP.md)
- [テストガイド](./testing-guide.md)
- [APIリファレンス](../../reference/ja/api/)
