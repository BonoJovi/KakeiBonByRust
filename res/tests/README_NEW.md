# KakeiBon テストドキュメント

このディレクトリには、KakeiBonアプリケーションのテストコードとドキュメントが含まれています。

## ドキュメント構成

- **[TEST_DESIGN.md](TEST_DESIGN.md)** - テストモジュールの設計仕様と設計ポリシー
- **[TEST_CASES.md](TEST_CASES.md)** - 全テストケースの一覧と説明
- **[QUICK_START.md](QUICK_START.md)** - テストの実行方法クイックガイド
- このファイル（README.md）- 概要とインデックス

## テストの種類

### 1. Rustユニットテスト
バックエンドのバリデーションロジックと認証ロジックをテスト
```bash
cargo test
```

### 2. JavaScriptユニットテスト（Jest）
フロントエンドのバリデーションロジックをテスト
```bash
cd res/tests
npm install
npm test
```

### 3. ブラウザ統合テスト
実際のブラウザ環境でUIとバリデーションをテスト
```bash
cargo tauri dev
# ブラウザで各 .test.html ファイルを開く
```

## クイックスタート

### すべてのテストを実行
```bash
./res/tests/run-all-tests.sh
```

### 特定のテストのみ実行
```bash
# Jestテスト（推奨）
cd res/tests
npm test

# 特定のテストファイル
npm test admin-setup.test.js
npm test user-addition.test.js
```

## テスト統計（2024-10-26更新）

- **Rustユニットテスト**: 49件
- **JavaScriptユニットテスト**: 136件
  - 管理者登録: 29件
  - ログイン機能: 58件
  - ユーザ追加: 49件
- **合計**: 185件

## 主要な改善点（2024-10-26）

### テストコードの一元管理
共通モジュール構造を導入し、テストコードの再利用性と保守性を大幅に向上：

- `validation-helpers.js` - 共通バリデーション関数
- `password-validation-tests.js` - パスワードテストスイート
- `username-validation-tests.js` - ユーザ名テストスイート

詳細は [TEST_DESIGN.md](TEST_DESIGN.md) を参照してください。

## 次のステップ

1. **テストを実行したい** → [QUICK_START.md](QUICK_START.md)
2. **テストケースを確認したい** → [TEST_CASES.md](TEST_CASES.md)
3. **テストを追加/修正したい** → [TEST_DESIGN.md](TEST_DESIGN.md)
4. **トラブルシューティング** → [QUICK_START.md#トラブルシューティング](QUICK_START.md)

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

## 貢献

テストの追加や改善を行う際は、[TEST_DESIGN.md](TEST_DESIGN.md) の設計ポリシーに従ってください。
