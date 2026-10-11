# KakeiBon Tests

このディレクトリには、KakeiBonアプリケーションのフロントエンドテストが含まれています。

## [Books] ドキュメント

テストの詳細なドキュメントは以下を参照してください：

- **[テスト概要](../../docs/testing/ja/TEST_OVERVIEW.md)** - テスト戦略・実行方法
- **[バックエンドテストインデックス](../../docs/testing/ja/BACKEND_TEST_INDEX.md)** - Rustテスト完全一覧
- **[フロントエンドテストインデックス](../../docs/testing/ja/FRONTEND_TEST_INDEX.md)** - JavaScriptテスト完全一覧

## クイックスタート

### すべてのテストを実行

```bash
npm test
```

### 特定のテストファイルのみ実行

```bash
npm test pages/index-login-page.test.js
npm test pages/user-management-delete-page.test.js
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

### 画面別テスト
- `pages/index-setup-validation.test.js` - 管理者登録・ユーザー登録フォームの入力チェックのテスト
- `pages/user-management-admin-validation.test.js` - ユーザー管理（管理者）の追加・編集の入力チェックのテスト
- `pages/user-management-self-edit-validation.test.js` - ユーザー管理（一般ユーザー）の自分の編集の入力チェックのテスト
- `pages/index-login-page.test.js` - ログイン・ログアウトのテスト
- `pages/user-management-delete-page.test.js` - ユーザー削除のテスト

### 機能別テスト
- `pages/transaction-management-edit-roundtrip.test.js` - 取引ヘッダーの読み込みと保存で送る値のテスト
- `pages/transaction-management-edit-modal-errors.test.js` - 編集ウィンドウが扱うヘッダーと、読み込み・保存失敗時の表示のテスト
- `pages/transaction-management-amounts.test.js` - 合計金額の表示と読み取りのテスト
- `pages/transaction-detail-form-page.test.js` - 取引明細の入力フォームとヘッダー合計のテスト
- `transaction-detail-tax-calculation.test.js` - 税計算テスト

### 集計機能テスト
- `aggregation-daily.test.js` - 日次集計テスト
- `aggregation-weekly.test.js` - 週次集計テスト
- `aggregation-monthly.test.js` - 月次集計テスト
- `aggregation-yearly.test.js` - 年次集計テスト
- `aggregation-period.test.js` - 期間集計テスト

## 詳細情報

すべてのテストケースの詳細（テスト名、説明、ファイル、行番号）については、[フロントエンドテストインデックス](../../docs/testing/ja/FRONTEND_TEST_INDEX.md)を参照してください。
