# KakeiBon Tests

このディレクトリには、KakeiBonアプリケーションのテストコードが含まれています。

## テストの種類

### 1. Rustユニットテスト
- **場所**: `src/validation.rs`, `src/db.rs`
- **実行方法**: 
  ```bash
  cargo test
  # または特定のモジュールのみ
  cargo test validation
  cargo test db::tests
  ```
- **説明**: Rustのバックエンドバリデーションロジックと認証ロジックをテスト

### 2. ブラウザ統合テスト
- **ファイル**: 
  - `admin-setup.test.html` - 管理者登録の統合テスト (12件)
  - `login-integration.test.html` - ログイン機能の統合テスト (15件)
- **実行方法**: 
  1. アプリケーションを起動: `cargo tauri dev`
  2. ブラウザで各HTMLファイルを開く
  3. 「Run All Tests」ボタンをクリック
- **説明**: JavaScriptでのバリデーションとUI動作をブラウザ環境でテスト

### 3. Node.jsスタンドアロンテスト（推奨）
- **ファイル**: 
  - `login-test-standalone.js` - ログイン機能のテスト (37件)
  - `backend-validation-standalone.js` - バックエンドバリデーションテスト (22件)
- **実行方法**: 
  ```bash
  cd res/tests
  node login-test-standalone.js
  node backend-validation-standalone.js
  ```
- **説明**: アプリ起動不要、依存関係なしで実行可能

### 4. Node.jsユニットテスト（Jest - オプション）
- **ファイル**: 
  - `admin-setup.test.js` - 管理者登録のバリデーションテスト
  - `login.test.js` - ログイン機能のユニットテスト
- **フレームワーク**: Jest
- **実行方法**: 
  ```bash
  cd res/tests
  npm install
  npm test
  ```

## テスト内容

### パスワードバリデーションテスト

#### 空のパスワードのバリデーション
- ✓ 空文字列のパスワードを拒否
- ✓ スペースのみのパスワードを拒否
- ✓ タブのみのパスワードを拒否
- ✓ 混合空白文字のみのパスワードを拒否
- ✓ nullパスワードを拒否（JavaScriptのみ）
- ✓ undefinedパスワードを拒否（JavaScriptのみ）

#### パスワード長のバリデーション
- ✓ 16文字未満のパスワードを拒否
- ✓ 1文字のパスワードを拒否
- ✓ 15文字のパスワードを拒否
- ✓ 16文字ちょうどのパスワードを受け入れ
- ✓ 16文字以上のパスワードを受け入れ

#### パスワード一致のバリデーション
- ✓ 一致しないパスワードを拒否
- ✓ パスワードが正しく確認が空の場合を拒否
- ✓ パスワードが正しく確認がnullの場合を拒否
- ✓ 大文字小文字の不一致を拒否

#### 有効なパスワードのバリデーション
- ✓ 有効な一致するパスワードを受け入れ（16文字以上）
- ✓ スペースを含むパスワード（一致時、16文字以上）を受け入れ
- ✓ 特殊文字を含むパスワードを受け入れ
- ✓ 前後にスペースがあるパスワード（一致時、16文字以上）を受け入れ
- ✓ 非常に長いパスワードを受け入れ
- ✓ Unicode文字を含むパスワードを受け入れ
- ✓ 絵文字を含むパスワードを受け入れ

#### エッジケース
- ✓ 改行を含むパスワードの処理
- ✓ ゼロ幅スペースの処理
- ✓ 数字のみのパスワードの処理（16文字以上）
- ✓ 境界値のテスト（15文字、16文字、17文字）

## テストの実行

### すべてのテストを一括実行（推奨）

```bash
# プロジェクトルートから
./res/tests/run-all-tests.sh
```

このスクリプトは以下を自動実行します：
- Rustユニットテスト: 47件
- JavaScriptスタンドアロンテスト: 59件
- **合計**: 106件

### 個別にテストを実行

```bash
# Rustユニットテスト
cargo test --lib

# フロントエンドテスト（Jestが必要）
cd res/tests
npm install
npm test

# 統合テスト（手動）
# 1. アプリを起動
cargo tauri dev

# 2. ブラウザで以下のファイルを開く
# - res/tests/admin-setup.test.html (12件)
# - res/tests/login-integration.test.html (15件)
```

### 特定のテストのみ実行

```bash
# Rustの特定のテストのみ
cargo test test_password_too_short
cargo test validation::tests

# Jestの特定のテストのみ
cd res/tests
npm test -- --testNamePattern="Empty Password"
```

### カバレッジレポート

```bash
# Rust（cargo-tarpaulinが必要）
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Jest
cd res/tests
npm run test:coverage
```

## テストの追加

新しいバリデーションルールを追加した場合：

1. **Rustユニットテスト**を`src/validation.rs`の`tests`モジュールに追加
2. **JavaScriptスタンドアロンテスト**を対応するファイルに追加
   - ログイン関連: `login-test-standalone.js`
   - バリデーション関連: `backend-validation-standalone.js`
3. **Jestテスト**を対応するファイルに追加 (オプション)
   - ログイン: `login.test.js`
   - 管理者登録: `admin-setup.test.js`
4. すべてのテストを実行して確認: `./res/tests/run-all-tests.sh`

## 期待される出力

### Rustユニットテスト
```
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured
  - validation.rs: 22 tests
  - db.rs: 27 tests
```

### Jestテスト
```
Tests: 110+ | Passed: 110+ | Failed: 0
  - admin-setup.test.js: 30+ tests
  - login.test.js: 80+ tests
```

### ブラウザテスト
```
Admin Setup Tests: 12 | Passed: 12 | Failed: 0
Login Integration Tests: 15 | Passed: 15 | Failed: 0
```

### 統合テスト
```
Backend Validation Tests: 11 | Passed: 11 | Failed: 0
```

## テスト統計

### 合計テスト数
- **Rustユニットテスト**: 49件
  - パスワードバリデーション: 22件
  - 認証・ログイン: 27件
- **JavaScriptユニットテスト**: 110+件
  - 管理者登録: 30+件
  - ログイン機能: 80+件
- **ブラウザ統合テスト**: 38件
  - 管理者登録UI: 12件
  - バックエンド統合: 11件
  - ログイン統合: 15件

**総計**: 197+件のテスト

## トラブルシューティング

### Rustテストが失敗する場合
- `cargo clean`を実行してから再ビルド
- `Cargo.toml`の依存関係を確認

### Jestテストが失敗する場合
```bash
cd res/tests
rm -rf node_modules package-lock.json
npm install
npm test
```

### 統合テストが失敗する場合
- アプリが正しく起動しているか確認
- ブラウザのコンソールでエラーを確認
- Tauriコマンドが正しく登録されているか確認（`src/lib.rs`）

### フロントエンドとバックエンドの不整合
統合テストが失敗し、JavaScriptテストとRustテストの両方が成功している場合：

1. エラーメッセージの文字列が完全に一致しているか確認
2. Tauriコマンドの引数名が一致しているか確認
3. バックエンドのバリデーションロジックが更新されているか確認

## テスト戦略

### レイヤー別テスト

1. **ユニットテスト（最優先・推奨）**
   - Rust: `src/validation.rs`, `src/db.rs`
   - JavaScript スタンドアロン: `login-test-standalone.js`, `backend-validation-standalone.js`
   - 高速で信頼性が高い、依存関係なし

2. **Jestテスト（オプション）**
   - `admin-setup.test.js`, `login.test.js`
   - npm環境が必要

3. **ブラウザ統合テスト（手動）**
   - `admin-setup.test.html`, `login-integration.test.html`
   - アプリ起動が必要
   - ユーザー体験に近い環境でテスト

### テストの優先順位

1. 空のパスワードチェック
2. 長さチェック（16文字）
3. パスワード一致チェック
4. エッジケース
5. パフォーマンステスト（非常に長いパスワード）

## CI/CD統合

```yaml
# .github/workflows/test.yml の例
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
      - name: Run Rust tests
        run: cargo test
      - name: Install Node.js
        uses: actions/setup-node@v2
      - name: Run Jest tests
        run: |
          cd res/tests
          npm install
          npm test
```
