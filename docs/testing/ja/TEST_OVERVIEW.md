# テスト概要

このドキュメントは、KakeiBonプロジェクトのテスト戦略、実行方法、テストケースの全体像を説明します。

**最終更新**: 2025-12-06 06:24 JST  
**総テスト数**: 463件以上（Rust 201件 + JavaScript 262件以上）

---

## 目次

- [テスト哲学](#テスト哲学)
- [テスト構成](#テスト構成)
- [クイックスタート](#クイックスタート)
- [テストインデックス](#テストインデックス)
- [テストの実行方法](#テストの実行方法)
- [新規テストの追加](#新規テストの追加)
- [テスト設計原則](#テスト設計原則)
- [CI/CD統合](#cicd統合)
- [トラブルシューティング](#トラブルシューティング)

---

## テスト哲学

KakeiBonプロジェクトは、品質保証と開発効率のバランスを重視したテスト戦略を採用しています。

### 基本原則

1. **DRY（Don't Repeat Yourself）**
   - テストケースを共通モジュール化し、複数の画面・機能で再利用
   - バリデーションロジックも共通化

2. **保守性の重視**
   - テストコードもプロダクションコードと同様に保守性を重視
   - バリデーションルール変更時の修正箇所を最小限に

3. **一貫性の確保**
   - フロントエンドとバックエンドで同じバリデーションルールを適用
   - すべての画面で統一されたエラーメッセージ

4. **インデックス化**
   - すべてのテストケースを表形式でインデックス化
   - テストケースの状況把握と実装漏れの防止

---

## テスト構成

### 3層テストアーキテクチャ

```
┌─────────────────────────────────────────────┐
│  フロントエンド（JavaScript）              │
│  - 画面別テスト: 206件                     │
│  - 共通テストスイート: 56件                │
│  - 機能別テスト: 118件+                    │
│  - 集計機能テスト: 多数                    │
└──────────────┬──────────────────────────────┘
               │
               │ Tauri IPC
               │
┌──────────────▼──────────────────────────────┐
│  バックエンド（Rust）                      │
│  - 共通テストスイート: 23件                │
│  - インラインテスト: 178件                 │
│  - データベース・セキュリティ・暗号化      │
│  - サービス層（認証、ユーザー管理など）    │
└─────────────────────────────────────────────┘
```

### ディレクトリ構造

```
KakeiBonByRust/
├── src/                          # Rustバックエンド
│   ├── validation_tests.rs       # 共通バリデーションテストスイート
│   ├── test_helpers.rs           # テストヘルパー関数
│   ├── font_size_tests.rs        # フォントサイズテスト
│   ├── validation.rs             # インラインテスト
│   ├── security.rs               # インラインテスト
│   ├── crypto.rs                 # インラインテスト
│   └── services/                 # 各サービスのインラインテスト
│       ├── auth.rs
│       ├── user_management.rs
│       ├── encryption.rs
│       └── ...
│
├── res/tests/                    # JavaScriptフロントエンド
│   ├── validation-helpers.js     # 共通バリデーション関数
│   ├── password-validation-tests.js    # パスワードテストスイート
│   ├── username-validation-tests.js    # ユーザー名テストスイート
│   ├── user-edit-validation-tests.js   # ユーザー編集テストスイート
│   ├── admin-setup.test.js       # 管理者登録テスト
│   ├── user-addition.test.js     # ユーザー追加テスト
│   ├── login.test.js             # ログインテスト
│   ├── user-deletion.test.js     # ユーザー削除テスト
│   ├── transaction-*.test.js     # 取引関連テスト
│   └── aggregation-*.test.js     # 集計関連テスト
│
└── docs/testing/                 # テストドキュメント（このディレクトリ）
    ├── ja/
    │   ├── TEST_OVERVIEW.md      # この文書
    │   ├── BACKEND_TEST_INDEX.md # Rustテスト完全インデックス
    │   ├── FRONTEND_TEST_INDEX.md # JavaScriptテスト完全インデックス
    │   ├── TEST_DESIGN.md        # テスト設計思想
    │   └── TEST_RESULTS.md       # 最新テスト結果
    └── en/
        └── (英語版ドキュメント)
```

---

## クイックスタート

### すべてのテストを一括実行

プロジェクトルートから：

```bash
# Rustテストを実行
cargo test

# JavaScriptテストを実行
cd res/tests
npm install  # 初回のみ
npm test
```

### 特定のテストのみ実行

```bash
# Rust: バリデーションテストのみ
cargo test validation::

# JavaScript: ログインテストのみ
cd res/tests
npm test login.test.js
```

---

## テストインデックス

テストケースの完全な一覧は、以下のインデックスドキュメントを参照してください：

### 📘 [バックエンドテストインデックス](BACKEND_TEST_INDEX.md)
- **総テスト数**: 201件
- Rustで実装されたすべてのテストケースを表形式で網羅
- テスト関数名、説明、ファイル名、行番号を含む

### 📗 [フロントエンドテストインデックス](FRONTEND_TEST_INDEX.md)
- **総テスト数**: 262件以上
- JavaScriptで実装されたすべてのテストケースを表形式で網羅
- テスト名、説明、使用箇所を含む

---

## テストの実行方法

### Rustテスト

#### すべてのテストを実行

```bash
cargo test
```

出力例：
```
running 201 tests
test validation::tests::test_empty_password ... ok
test security::tests::test_hash_password ... ok
test services::auth::tests::test_register_admin_user ... ok
...
test result: ok. 201 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

#### 特定のモジュールのみ実行

```bash
# バリデーション関連のみ
cargo test validation::

# 認証サービスのみ
cargo test services::auth::

# ユーザー管理サービスのみ
cargo test services::user_management::
```

#### 特定のテスト関数のみ実行

```bash
cargo test test_empty_password
cargo test test_register_admin_user
```

#### 出力を表示して実行

```bash
cargo test -- --nocapture
```

#### カバレッジレポート生成

```bash
# cargo-tarpaulinのインストール（初回のみ）
cargo install cargo-tarpaulin

# HTMLレポート生成
cargo tarpaulin --out Html

# ブラウザで tarpaulin-report.html を開く
```

### JavaScriptテスト

#### すべてのテストを実行

```bash
cd res/tests
npm test
```

出力例：
```
PASS  ./admin-setup.test.js
PASS  ./login.test.js
PASS  ./user-deletion.test.js
...
Tests: 262 passed, 262 total
```

#### 特定のテストファイルのみ実行

```bash
npm test admin-setup.test.js
npm test login.test.js
npm test user-deletion.test.js
```

#### 特定のテストケースのみ実行

```bash
# テスト名で絞り込み
npm test -- --testNamePattern="Empty Password"
npm test -- --testNamePattern="Username Validation"
```

#### ウォッチモード（ファイル変更を監視）

```bash
npm test -- --watch
```

#### カバレッジレポート生成

```bash
npm run test:coverage
```

#### スタンドアロンテスト（Node.js、依存関係なし）

```bash
node login-test-standalone.js
node backend-validation-standalone.js
```

---

## 新規テストの追加

### Rustテストの追加

#### パターン1: 共通テストスイートに追加（推奨）

既存の機能に新しいテストケースを追加する場合：

```rust
// src/validation_tests.rs
pub fn test_new_password_rule() {
    let result = validate_password("新しいルールのテスト", "新しいルールのテスト");
    assert!(result.is_ok());
}
```

#### パターン2: 新しいサービスにインラインテスト追加

新しいサービスモジュールを作成する場合：

```rust
// src/services/new_service.rs

// サービス実装
pub async fn new_function() -> Result<(), String> {
    // ...
}

// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_new_function() {
        let result = new_function().await;
        assert!(result.is_ok());
    }
}
```

### JavaScriptテストの追加

#### パターン1: 共通テストスイートに追加（推奨）

既存のバリデーションに新しいテストケースを追加する場合：

```javascript
// res/tests/password-validation-tests.js

export function testNewPasswordRule(validationFn) {
    describe('New Password Rule', () => {
        test('should enforce new rule', () => {
            const result = validationFn('test', 'test');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('New rule error!');
        });
    });
}
```

#### パターン2: 新しい画面のテスト追加

新しい画面を追加した場合：

```javascript
// res/tests/new-screen.test.js

import { validatePassword } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';

// 共通パスワードテストを実行
runAllPasswordTests(validatePassword, 'New Screen Password Validation');

// 画面固有のテストを追加
describe('New Screen Specific Tests', () => {
    test('specific edge case', () => {
        const result = validatePassword('特定のケース', '特定のケース');
        expect(result.valid).toBe(true);
    });
});
```

### テストインデックスの更新

新しいテストを追加したら、必ずインデックスドキュメントも更新してください：

1. [BACKEND_TEST_INDEX.md](BACKEND_TEST_INDEX.md) - Rustテスト追加時
2. [FRONTEND_TEST_INDEX.md](FRONTEND_TEST_INDEX.md) - JavaScriptテスト追加時

表に以下の情報を追加：
- テスト関数名
- 説明（簡潔に）
- ファイル名
- 行番号（任意）

---

## テスト設計原則

詳細なテスト設計思想については、[TEST_DESIGN.md](TEST_DESIGN.md)を参照してください。

### 主要な設計原則

1. **共通化とDRY原則**
   - 同じバリデーションロジックは共通モジュールに
   - 同じテストケースは共通テストスイートに
   - 画面固有のロジックのみ個別ファイルに

2. **テスト名の明確化**
   ```javascript
   // ✓ 良い例
   test('should reject password shorter than 16 characters', () => { });
   
   // ✗ 悪い例
   test('password test', () => { });
   ```

3. **エラーメッセージの統一**
   ```rust
   // ✓ 良い例 - 定数で定義
   const ERROR_PASSWORD_TOO_SHORT: &str = "Password must be at least 16 characters long!";
   
   // ✗ 悪い例 - 画面ごとに異なるメッセージ
   ```

4. **テストの独立性**
   - テスト間で状態を共有しない
   - 各テストは単独で実行可能
   - グローバル変数の使用を避ける

---

## CI/CD統合

### GitHub Actionsの例

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  rust-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run Rust tests
        run: cargo test --verbose

  javascript-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Node.js
        uses: actions/setup-node@v2
        with:
          node-version: '18'
      - name: Install dependencies
        run: |
          cd res/tests
          npm install
      - name: Run JavaScript tests
        run: |
          cd res/tests
          npm test
```

---

## トラブルシューティング

### Rustテストが失敗する場合

#### ビルドエラー

```bash
# クリーンビルド
cargo clean
cargo build
cargo test
```

#### 特定のテストのみ失敗

```bash
# 該当テストのみ実行して詳細を確認
cargo test test_name -- --nocapture
```

### JavaScriptテストが失敗する場合

#### ES Modulesエラー

```
SyntaxError: Cannot use import statement outside a module
```

**解決方法:**
1. `package.json`に`"type": "module"`があることを確認
2. import文に`.js`拡張子を含めているか確認
3. Jest実行コマンドに`--experimental-vm-modules`を追加

```bash
node --experimental-vm-modules node_modules/jest/bin/jest.js
```

#### 依存関係エラー

```bash
cd res/tests
rm -rf node_modules package-lock.json
npm install
npm test
```

#### テストが見つからない

```
describe is not defined
```

**解決方法:**
Jest設定で`testEnvironment: "jsdom"`を指定

```json
// package.json
{
  "jest": {
    "testEnvironment": "jsdom",
    "transform": {}
  }
}
```

### フロントエンドとバックエンドの不整合

統合テストが失敗し、JavaScriptテストとRustテストの両方が成功している場合：

1. エラーメッセージの文字列が完全に一致しているか確認
2. Tauriコマンドの引数名が一致しているか確認
3. バックエンドのバリデーションロジックが更新されているか確認

---

## テスト統計

| カテゴリ | テスト数 | 状態 |
|---------|---------|------|
| **Rustバックエンド** | **201件** | ✅ |
| 共通テストスイート | 23 | ✅ |
| インラインテスト | 178 | ✅ |
| **JavaScriptフロントエンド** | **262件以上** | ✅ |
| 共通テストスイート | 56 | ✅ |
| 画面別テスト | 206 | ✅ |
| 機能別テスト | 118+ | ✅ |
| 集計機能テスト | 多数 | ✅ |
| **総計** | **463件以上** | ✅ |

---

## コントリビューター向けガイドライン

### テストケースのレビュー

1. **インデックスで全体を把握**
   - [BACKEND_TEST_INDEX.md](BACKEND_TEST_INDEX.md)
   - [FRONTEND_TEST_INDEX.md](FRONTEND_TEST_INDEX.md)

2. **テストカバレッジの確認**
   ```bash
   # Rust
   cargo tarpaulin --out Html
   
   # JavaScript
   cd res/tests
   npm run test:coverage
   ```

3. **実装漏れの確認**
   - インデックスを見て、各機能に対応するテストがあるか確認
   - 新機能追加時は必ずテストも追加

4. **テストの妥当性チェック**
   - テスト名が内容を正確に表しているか
   - エッジケースがカバーされているか
   - エラーメッセージが統一されているか

### プルリクエスト時のチェックリスト

- [ ] すべてのテストが成功している（Rust + JavaScript）
- [ ] 新規機能に対応するテストを追加した
- [ ] テストインデックスを更新した
- [ ] カバレッジが低下していない
- [ ] テスト名が明確で一貫性がある

---

## 関連ドキュメント

- 📘 [バックエンドテストインデックス](BACKEND_TEST_INDEX.md) - Rustテスト完全一覧
- 📗 [フロントエンドテストインデックス](FRONTEND_TEST_INDEX.md) - JavaScriptテスト完全一覧
- 📙 [テスト設計](TEST_DESIGN.md) - テストアーキテクチャと設計思想
- 📕 [テスト結果](TEST_RESULTS.md) - 最新のテスト実行結果とカバレッジ

---

**質問や提案があれば、Issueを作成してください。**
