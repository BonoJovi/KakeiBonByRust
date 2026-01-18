# KakeiBon Tests

このディレクトリには、KakeiBonアプリケーションのフロントエンドテストが含まれています。

## [Books] ドキュメント

テストの詳細なドキュメントは以下を参照してください：

- **[テスト概要](../../docs/testing/ja/TEST_OVERVIEW.md)** - テスト戦略・実行方法
- **[バックエンドテストインデックス](../../docs/testing/ja/BACKEND_TEST_INDEX.md)** - Rustテスト完全一覧（201件）
- **[フロントエンドテストインデックス](../../docs/testing/ja/FRONTEND_TEST_INDEX.md)** - JavaScriptテスト完全一覧（262件以上）

## クイックスタート

### すべてのテストを実行

```bash
npm test
```

### 特定のテストファイルのみ実行

```bash
npm test login.test.js
npm test user-deletion.test.js
```

### 特定のテストケースのみ実行

```bash
npm test -- --testNamePattern="Empty Password"
```

### カバレッジレポート生成

```bash
npm run test:coverage
```

## テスト構成

### 共通テストスイート
- `password-validation-tests.js` - パスワードバリデーションテスト（26件）
- `username-validation-tests.js` - ユーザー名バリデーションテスト（20件）
- `user-edit-validation-tests.js` - ユーザー編集バリデーションテスト（23件）
- `validation-helpers.js` - 共通バリデーション関数

### 画面別テスト
- `admin-setup.test.js` - 管理者登録テスト（29件）
- `user-addition.test.js` - ユーザー追加テスト（49件）
- `admin-edit.test.js` - 管理者編集テスト（63件）
- `general-user-edit.test.js` - 一般ユーザー編集テスト（63件）
- `login.test.js` - ログインテスト（58件）
- `user-deletion.test.js` - ユーザー削除テスト（46件）

### 機能別テスト
- `transaction-edit.test.js` - 取引編集テスト（112件）
- `transaction-detail-management.test.js` - 取引明細管理テスト
- `transaction-detail-tax-calculation.test.js` - 税計算テスト
- `category-management-ui-tests.js` - カテゴリ管理UIテスト

### 集計機能テスト
- `aggregation-daily.test.js` - 日次集計テスト
- `aggregation-weekly.test.js` - 週次集計テスト
- `aggregation-monthly.test.js` - 月次集計テスト
- `aggregation-yearly.test.js` - 年次集計テスト
- `aggregation-period.test.js` - 期間集計テスト

## 詳細情報

すべてのテストケースの詳細（テスト名、説明、ファイル、行番号）については、[フロントエンドテストインデックス](../../docs/testing/ja/FRONTEND_TEST_INDEX.md)を参照してください。
