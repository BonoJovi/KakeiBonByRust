# テストの実行方法

## スタンドアロンテスト（推奨・簡単）

依存関係なしで実行できます：

```bash
cd res/tests
node login-test-standalone.js
node backend-validation-standalone.js
```

**出力例:**
```
🔐 Running Login Tests (Standalone)...
📊 Test Summary: 37 tests | Passed: 37 ✓ | Failed: 0 ✗

🔧 Running Backend Validation Tests (Standalone)...
📊 Test Summary: 22 tests | Passed: 22 ✓ | Failed: 0 ✗

🎉 All tests passed!
```

## Jestテスト（完全版）

Jest環境が必要です：

```bash
cd res/tests
npm install
npm test login.test.js
```

## ブラウザ統合テスト

アプリを起動してから実行：

```bash
# ターミナル1
cargo tauri dev

# ターミナル2 または ブラウザで直接開く
open res/tests/login-integration.test.html
```

## すべてのテストを一括実行（最も簡単）

```bash
# プロジェクトルートから実行
./res/tests/run-all-tests.sh
```

このスクリプトは以下を自動実行します：
- Rustユニットテスト (47件)
- JavaScriptスタンドアロンテスト (59件)
- テスト結果サマリー

**出力例**:
```
🦀 Running Rust Unit Tests...
✓ Rust tests passed: 47

📝 Running JavaScript Standalone Tests...
✓ Login tests passed: 37
✓ Backend validation tests passed: 22

📊 Test Summary
Total Passed: 106 ✓
Total Failed: 0 ✗
🎉 All tests passed!
```

## 個別にテストを実行

```bash
# Rustテスト (47件)
cargo test --lib

# スタンドアロンJavaScriptテスト (59件)
cd res/tests
node login-test-standalone.js
node backend-validation-standalone.js

# ブラウザテスト（手動）
# 1. cargo tauri dev でアプリ起動
# 2. ブラウザで res/tests/*.test.html を開く
```

**テスト統計**:
- Rust: 47件 (validation: 22件 + db: 25件)
- JavaScript (スタンドアロン): 59件 (login: 37件 + backend: 22件)
- ブラウザ統合: 27件 (login: 15件 + admin: 12件) (要アプリ起動)

## トラブルシューティング

### npm installが動作しない場合
→ スタンドアロンテストを使用してください（`node login-test-standalone.js`）

### Jestでエラーが発生する場合
```bash
cd res/tests
rm -rf node_modules package-lock.json
npm install
```

それでも動作しない場合は、スタンドアロンテストで十分です。
