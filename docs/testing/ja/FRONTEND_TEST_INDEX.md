# フロントエンドテストインデックス

このドキュメントは、JavaScriptで実装されたフロントエンドテストの完全なインデックスです。

**最終更新**: 2026-10-07 JST  
**総テスト数**: 933件 (jest suite 86 ファイル、`npm test` 実測)

---

## 目次

- [共通テストスイート](#共通テストスイート)
  - [password-validation-tests.js](#password-validation-testsjs)
  - [username-validation-tests.js](#username-validation-testsjs)
  - [user-edit-validation-tests.js](#user-edit-validation-testsjs)
  - [validation-helpers.js](#validation-helpersjs)
- [画面別テスト](#画面別テスト)
  - [admin-setup.test.js](#admin-setuptestjs)
  - [user-addition.test.js](#user-additiontestjs)
  - [admin-edit.test.js](#admin-edittestjs)
  - [general-user-edit.test.js](#general-user-edittestjs)
  - [login.test.js](#logintestjs)
  - [user-deletion.test.js](#user-deletiontestjs)
- [機能別テスト](#機能別テスト)
  - [transaction-edit.test.js](#transaction-edittestjs)
  - [transaction-detail-management.test.js](#transaction-detail-managementtestjs)
  - [transaction-detail-tax-calculation.test.js](#transaction-detail-tax-calculationtestjs)
  - [toast.test.js](#toasttestjs)
  - [tax-calc.test.js](#tax-calctestjs)
  - [product-autocomplete.test.js](#product-autocompletetestjs)
  - [product-draft.test.js](#product-drafttestjs)
  - [product-master-jump-draft.test.js](#product-master-jump-drafttestjs)
  - [modal-double-submit.test.js](#modal-double-submittestjs)
  - [modal-stale-save-close.test.js](#modal-stale-save-closetestjs)
  - [master-crud.test.js](#master-crudtestjs)
  - [attach-char-counter-ime.test.js](#attach-char-counter-imetestjs)
  - [aggregation-error-translate.test.js](#aggregation-error-translatetestjs)
  - [aggregation-latest-request.test.js](#aggregation-latest-requesttestjs)
  - [parse-amount-strict.test.js](#parse-amount-stricttestjs)
  - [format-local-date.test.js](#format-local-datetestjs)
  - [period-end-date.test.js](#period-end-datetestjs)
  - [period-containing.test.js](#period-containingtestjs)
  - [aggregation-render-unspecified.test.js](#aggregation-render-unspecifiedtestjs)
  - [pages/transaction-detail-page.test.js](#pagestransaction-detail-pagetestjs)
  - [pages/transaction-detail-included-typing.test.js](#pagestransaction-detail-included-typingtestjs)
  - [pages/transaction-detail-unreachable-included-price.test.js](#pagestransaction-detail-unreachable-included-pricetestjs)
  - [pages/transaction-detail-hidden-category.test.js](#pagestransaction-detail-hidden-categorytestjs)
  - [pages/transaction-management-page.test.js](#pagestransaction-management-pagetestjs)
  - [pages/user-management-page.test.js](#pagesuser-management-pagetestjs)
  - [pages/recurring-rule-page.test.js](#pagesrecurring-rule-pagetestjs)
  - [pages/recurring-rule-double-submit.test.js](#pagesrecurring-rule-double-submittestjs)
  - [pages/recurring-rule-period-range.test.js](#pagesrecurring-rule-period-rangetestjs)
  - [pages/recurring-rule-anchor-follows-start.test.js](#pagesrecurring-rule-anchor-follows-starttestjs)
  - [pages/product-management-edit-manufacturer-roundtrip.test.js](#pagesproduct-management-edit-manufacturer-roundtriptestjs)
  - [pages/recurring-rule-derived-total.test.js](#pagesrecurring-rule-derived-totaltestjs)
  - [pages/recurring-rule-cycle-options.test.js](#pagesrecurring-rule-cycle-optionstestjs)
  - [pages/recurring-rule-date-order.test.js](#pagesrecurring-rule-date-ordertestjs)
  - [pages/recurring-rule-reset.test.js](#pagesrecurring-rule-resettestjs)
  - [pages/menu-i18n-seed.test.js](#pagesmenu-i18n-seedtestjs)
  - [pages/i18n-literal-user-text.test.js](#pagesi18n-literal-user-texttestjs)
  - [pages/dashboard-balance-header.test.js](#pagesdashboard-balance-headertestjs)
  - [pages/transaction-list-none-account-label.test.js](#pagestransaction-list-none-account-labeltestjs)
  - [pages/transaction-detail-none-account-label.test.js](#pagestransaction-detail-none-account-labeltestjs)
  - [pages/dashboard-balance-sign.test.js](#pagesdashboard-balance-signtestjs)
  - [pages/dashboard-default-period.test.js](#pagesdashboard-default-periodtestjs)
  - [pages/dashboard-stale-reload.test.js](#pagesdashboard-stale-reloadtestjs)
  - [single-flight.test.js](#single-flighttestjs)
  - [pages/product-management-page.test.js](#pagesproduct-management-pagetestjs)
  - [pages/product-management-link-draft.test.js](#pagesproduct-management-link-drafttestjs)
  - [pages/shop-management-disabled.test.js](#pagesshop-management-disabledtestjs)
  - [pages/transaction-management-disabled-shop.test.js](#pagestransaction-management-disabled-shoptestjs)
  - [pages/account-management-disabled.test.js](#pagesaccount-management-disabledtestjs)
  - [pages/account-management-save-error-keeps-form.test.js](#pagesaccount-management-save-error-keeps-formtestjs)
  - [pages/account-management-validation-i18n.test.js](#pagesaccount-management-validation-i18ntestjs)
  - [pages/transaction-management-disabled-account.test.js](#pagestransaction-management-disabled-accounttestjs)
  - [pages/transaction-management-category1-has-details.test.js](#pagestransaction-management-category1-has-detailstestjs)
  - [modal-open-awaits-onopen.test.js](#modal-open-awaits-onopentestjs)
  - [pages/transaction-management-restore-draft.test.js](#pagestransaction-management-restore-drafttestjs)
  - [pages/transaction-management-shop-roundtrip-draft.test.js](#pagestransaction-management-shop-roundtrip-drafttestjs)
  - [pages/transaction-management-rejected-save-keeps-form.test.js](#pagestransaction-management-rejected-save-keeps-formtestjs)
  - [pages/transaction-management-restore-disabled-shop.test.js](#pagestransaction-management-restore-disabled-shoptestjs)
  - [pages/transaction-management-restore-reopened.test.js](#pagestransaction-management-restore-reopenedtestjs)
  - [pages/aggregation-monthly-page.test.js](#pagesaggregation-monthly-pagetestjs)
  - [pages/aggregation-yearly-total-count.test.js](#pagesaggregation-yearly-total-counttestjs)
  - [pages/aggregation-default-period-monthly.test.js](#pagesaggregation-default-period-monthlytestjs)
  - [pages/aggregation-default-period-yearly.test.js](#pagesaggregation-default-period-yearlytestjs)
  - [pages/aggregation-monthly-stale.test.js](#pagesaggregation-monthly-staletestjs)
  - [pages/aggregation-daily-stale.test.js](#pagesaggregation-daily-staletestjs)
  - [pages/aggregation-weekly-stale.test.js](#pagesaggregation-weekly-staletestjs)
  - [pages/aggregation-period-stale.test.js](#pagesaggregation-period-staletestjs)
  - [pages/aggregation-yearly-stale.test.js](#pagesaggregation-yearly-staletestjs)
  - [pages/dashboard-bar-top10.test.js](#pagesdashboard-bar-top10testjs)
  - [pages/index-setup-page.test.js](#pagesindex-setup-pagetestjs)
  - [pages/category-management-page.test.js](#pagescategory-management-pagetestjs)
  - [pages/category-management-move-buttons.test.js](#pagescategory-management-move-buttonstestjs)
  - [pages/transaction-management-filter-hidden-category.test.js](#pagestransaction-management-filter-hidden-categorytestjs)
  - [pages/user-management-password-page.test.js](#pagesuser-management-password-pagetestjs)
  - [pages/user-management-nonadmin-page.test.js](#pagesuser-management-nonadmin-pagetestjs)
  - [pages/index-setup-password-length.test.js](#pagesindex-setup-password-lengthtestjs)
- [集計機能テスト](#集計機能テスト)
  - [aggregation-daily.test.js](#aggregation-dailytestjs)
  - [aggregation-weekly.test.js](#aggregation-weeklytestjs)
  - [aggregation-monthly.test.js](#aggregation-monthlytestjs)
  - [aggregation-yearly.test.js](#aggregation-yearlytestjs)
  - [aggregation-period.test.js](#aggregation-periodtestjs)
- [ブラウザ / スタンドアロン (jest 総計に含めない)](#ブラウザ--スタンドアロン-jest-総計に含めない)
  - [category-management-ui-tests.js](#category-management-ui-testsjs)
  - [tax-rounding-tests.js](#tax-rounding-testsjs)
  - [backend-validation-standalone.js](#backend-validation-standalonejs)
  - [login-test-standalone.js](#login-test-standalonejs)
  - [aggregation-test-helpers.js](#aggregation-test-helpersjs)

---

## 共通テストスイート

### password-validation-tests.js

パスワードバリデーションの再利用可能なテストスイート。すべての画面で共通利用。

| テストスイート関数 | 説明 | テスト数 | 行 |
|-------------------|------|---------|-----|
| `testEmptyPasswordValidation(validationFn)` | 空パスワードのバリデーション | 6件 | 12 |
| `testPasswordLengthValidation(validationFn)` | パスワード長のバリデーション | 6件 | 55 |
| `testPasswordMatchValidation(validationFn)` | パスワード一致のバリデーション | 6件 | 99 |
| `testValidPasswordScenarios(validationFn)` | 有効なパスワードシナリオ | 8件 | 142 |
| `runAllPasswordTests(validationFn, suiteName)` | すべてのパスワードテストを一括実行 | 26件 | 199 |

#### 詳細テストケース

**testEmptyPasswordValidation (6件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should reject empty string password` | 空文字列パスワードを拒否 | エラー: "Password cannot be empty!" |
| `should reject password with only spaces` | スペースのみのパスワードを拒否 | エラー: "Password cannot be empty!" |
| `should reject password with only tabs` | タブのみのパスワードを拒否 | エラー: "Password cannot be empty!" |
| `should reject password with mixed whitespace` | 混合空白文字のみのパスワードを拒否 | エラー: "Password cannot be empty!" |
| `should reject null password` | nullパスワードを拒否 | エラー: "Password cannot be empty!" |
| `should reject undefined password` | undefinedパスワードを拒否 | エラー: "Password cannot be empty!" |

**testPasswordLengthValidation (6件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should reject password shorter than 16 characters` | 16文字未満のパスワードを拒否 | エラー: "Password must be at least 16 characters long!" |
| `should reject password with exactly 1 character` | ちょうど1文字のパスワードを拒否 | エラー: "Password must be at least 16 characters long!" |
| `should reject password with exactly 15 characters` | ちょうど15文字のパスワードを拒否 | エラー: "Password must be at least 16 characters long!" |
| `should accept password with exactly 16 characters` | ちょうど16文字のパスワードを受け入れ | valid: true |
| `should accept password longer than 16 characters` | 16文字以上のパスワードを受け入れ | valid: true |
| `should accept very long password` | 非常に長いパスワード（1000文字）を受け入れ | valid: true |

**testPasswordMatchValidation (6件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should reject non-matching passwords` | 不一致のパスワードを拒否 | エラー: "Passwords do not match!" |
| `should reject when password is valid but confirmation is empty` | パスワードが有効で確認が空の場合を拒否 | エラー: "Password cannot be empty!" or "Passwords do not match!" |
| `should reject when password is valid but confirmation is null` | パスワードが有効で確認がnullの場合を拒否 | エラー: "Passwords do not match!" |
| `should reject case-sensitive mismatch` | 大文字小文字の不一致を拒否 | エラー: "Passwords do not match!" |
| `should accept matching passwords` | 一致するパスワードを受け入れ | valid: true |
| `should accept matching passwords with special chars` | 特殊文字を含む一致するパスワードを受け入れ | valid: true |

**testValidPasswordScenarios (8件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should accept password with spaces (if matching and >= 16 chars)` | スペースを含む有効なパスワードを受け入れ | valid: true |
| `should accept password with special characters` | 特殊文字を含むパスワードを受け入れ | valid: true |
| `should accept password with leading/trailing spaces (if matching and >= 16 chars)` | 前後にスペースがあるパスワードを受け入れ | valid: true |
| `should accept very long password` | 非常に長いパスワードを受け入れ | valid: true |
| `should accept password with unicode characters` | Unicode文字を含むパスワードを受け入れ | valid: true |
| `should accept password with emoji` | 絵文字を含むパスワードを受け入れ | valid: true |
| `should accept alphanumeric only password` | 英数字のみのパスワードを受け入れ | valid: true |
| `should accept numeric only password (if >= 16 chars)` | 数字のみのパスワード（16文字以上）を受け入れ | valid: true |

**使用箇所**: admin-setup.test.js, user-addition.test.js, admin-edit.test.js, general-user-edit.test.js

---

### username-validation-tests.js

ユーザー名バリデーションの再利用可能なテストスイート。

| テストスイート関数 | 説明 | テスト数 | 行 |
|-------------------|------|---------|-----|
| `testUsernameValidation(validationFn)` | ユーザー名バリデーション | 13件 | 11 |
| `testCombinedValidation(validationFn)` | ユーザー名とパスワードの組み合わせバリデーション | 7件 | 98 |

#### 詳細テストケース

**testUsernameValidation (13件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should reject empty username` | 空ユーザー名を拒否 | エラー: "Username cannot be empty!" |
| `should reject username with only spaces` | スペースのみのユーザー名を拒否 | エラー: "Username cannot be empty!" |
| `should reject username with only tabs` | タブのみのユーザー名を拒否 | エラー: "Username cannot be empty!" |
| `should reject username with mixed whitespace` | 混合空白文字のみのユーザー名を拒否 | エラー: "Username cannot be empty!" |
| `should reject null username` | nullユーザー名を拒否 | エラー: "Username cannot be empty!" |
| `should reject undefined username` | undefinedユーザー名を拒否 | エラー: "Username cannot be empty!" |
| `should accept valid username` | 有効なユーザー名を受け入れ | valid: true |
| `should accept username with numbers` | 数字を含むユーザー名を受け入れ | valid: true |
| `should accept username with underscores` | アンダースコアを含むユーザー名を受け入れ | valid: true |
| `should accept username with special characters` | 特殊文字を含むユーザー名を受け入れ | valid: true |
| `should accept unicode username` | Unicode文字を含むユーザー名を受け入れ | valid: true |
| `should accept single character username` | 1文字のユーザー名を受け入れ | valid: true |
| `should accept very long username` | 非常に長いユーザー名を受け入れ | valid: true |

**testCombinedValidation (7件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should reject when both username and password are empty` | ユーザー名とパスワードが両方空の場合を拒否 | エラー: "Username cannot be empty!" |
| `should prioritize username validation over password` | ユーザー名のバリデーションをパスワードより優先 | エラー: "Username cannot be empty!" |
| `should validate password when username is valid` | ユーザー名が有効な場合はパスワードをバリデーション | エラー: "Password cannot be empty!" |
| `should validate password match when username and password are valid` | ユーザー名とパスワードが有効な場合は一致を確認 | エラー: "Passwords do not match!" |
| `should accept completely valid input` | 完全に有効な入力を受け入れ | valid: true |
| `should accept complex username with valid password` | 複雑なユーザー名と有効なパスワードを受け入れ | valid: true |
| `should accept valid username with complex password` | 有効なユーザー名と複雑なパスワードを受け入れ | valid: true |

**使用箇所**: user-addition.test.js, admin-edit.test.js, general-user-edit.test.js

---

### user-edit-validation-tests.js

ユーザー編集バリデーションの再利用可能なテストスイート。

| テストスイート関数 | 説明 | テスト数 | 行 |
|-------------------|------|---------|-----|
| `testUsernameOnlyEdit(validateFunc)` | ユーザー名のみ編集のテスト | 6件 | 13 |
| `testPasswordOnlyEdit(validateFunc)` | パスワードのみ編集のテスト | 8件 | 56 |
| `testCombinedEdit(validateFunc)` | ユーザー名とパスワード同時編集のテスト | 4件 | 113 |
| `testEditModeVsAddMode(validateFunc)` | 編集モードと追加モードの比較テスト | 5件 | 146 |
| `runAllUserEditTests(validateFunc, contextName)` | すべてのユーザー編集テストを一括実行 | 23件 | 184 |

#### 詳細テストケース

**testUsernameOnlyEdit (6件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should allow username change without password` | パスワードなしでユーザー名変更を許可 | valid: true |
| `should reject empty username even when password is empty` | パスワードが空でも空ユーザー名を拒否 | エラー: "Username cannot be empty!" |
| `should reject whitespace-only username` | 空白のみのユーザー名を拒否 | エラー: "Username cannot be empty!" |
| `should accept valid username with empty password in edit mode` | 編集モードで有効なユーザー名と空パスワードを受け入れ | valid: true |
| `should accept unicode username without password change` | パスワード変更なしでUnicodeユーザー名を受け入れ | valid: true |
| `should accept special chars username without password change` | パスワード変更なしで特殊文字ユーザー名を受け入れ | valid: true |

**testPasswordOnlyEdit (8件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should allow password change without username change` | ユーザー名変更なしでパスワード変更を許可 | valid: true |
| `should reject password change if new password is empty` | 新パスワードが空の場合を拒否 | エラー: "Password cannot be empty!" |
| `should reject password change if new password is too short` | 新パスワードが短すぎる場合を拒否 | エラー: "Password must be at least 16 characters long!" |
| `should reject password change if passwords don't match` | パスワードが一致しない場合を拒否 | エラー: "Passwords do not match!" |
| `should accept valid password change with same username` | 同じユーザー名で有効なパスワード変更を受け入れ | valid: true |
| `should accept password with special characters` | 特殊文字を含むパスワードを受け入れ | valid: true |
| `should accept password with unicode characters` | Unicode文字を含むパスワードを受け入れ | valid: true |
| `should accept very long new password` | 非常に長い新パスワードを受け入れ | valid: true |

**testCombinedEdit (4件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should allow both username and password change` | ユーザー名とパスワード両方の変更を許可 | valid: true |
| `should reject if username valid but password invalid` | ユーザー名が有効でパスワードが無効な場合を拒否 | エラー: "Password must be at least 16 characters long!" |
| `should reject if username empty but password valid` | ユーザー名が空でパスワードが有効な場合を拒否 | エラー: "Username cannot be empty!" |
| `should accept unicode username with new password` | Unicodeユーザー名と新パスワードを受け入れ | valid: true |

**testEditModeVsAddMode (5件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `edit mode should allow empty password (no change)` | 編集モードで空パスワード（変更なし）を許可 | valid: true |
| `add mode should reject empty password` | 追加モードで空パスワードを拒否 | エラー: "Password cannot be empty!" |
| `edit mode should validate password if provided` | 編集モードでパスワード提供時はバリデーション | エラー: "Password must be at least 16 characters long!" |
| `add mode should require password` | 追加モードでパスワードを必須に | エラー: "Password cannot be empty!" |
| `both modes should accept valid complete input` | 両モードで完全に有効な入力を受け入れ | valid: true |

**使用箇所**: admin-edit.test.js, general-user-edit.test.js

---

### validation-helpers.js

共通バリデーション関数（テスト関数なし、ユーティリティのみ）

| 関数名 | 説明 | パラメータ | 戻り値 |
|-------|------|-----------|-------|
| `validatePassword(password, passwordConfirm)` | パスワードとパスワード確認のバリデーション | `password`, `passwordConfirm` | `{valid: boolean, message: string}` |
| `validateUserAddition(username, password, passwordConfirm)` | ユーザー追加時のバリデーション | `username`, `password`, `passwordConfirm` | `{valid: boolean, message: string}` |
| `validateUserEdit(username, password, passwordConfirm, isEditMode)` | ユーザー編集時のバリデーション | `username`, `password`, `passwordConfirm`, `isEditMode` | `{valid: boolean, message: string}` |

---

## 画面別テスト

### admin-setup.test.js

管理者登録画面のテスト。

**テスト数**: 32件（共通26件 + 画面固有6件、jest 実測）

| テストカテゴリ | 説明 | テスト数 | 実装方法 |
|--------------|------|---------|---------|
| パスワードバリデーション | 共通パスワードテストスイート | 26件 | `runAllPasswordTests()` |
| 画面固有エッジケース | 管理者登録画面特有のテスト | 6件 | 個別実装 (`Admin Setup Specific Edge Cases` describe) |

#### 画面固有テスト (6件)

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should handle password with leading/trailing spaces if matching and long enough` | 前後にスペース付き、両者一致・16文字以上 | valid: true |
| `should accept very long password` | 1000 文字パスワード | valid: true |
| `should accept password with emojis` | 絵文字を含むパスワード (2 バイト以上文字含む) | valid: true |
| `should handle password with newlines (not trimmed)` | 改行を含み、確認側で改行が落ちる | valid: false, "Passwords do not match!" |
| `should handle zero-width space` | ゼロ幅スペース 1 文字 (可視 0 だが `.length` は 1) | valid: false, "at least 16 characters" |
| `should reject short numeric password` | 6 桁の数字のみ | valid: false, "at least 16 characters" |

**ファイル**: res/tests/admin-setup.test.js

---

### user-addition.test.js

ユーザー追加画面のテスト。共通スイートのみで構成 (画面固有 describe は無し)。

**テスト数**: 46件 (jest 実測、13 + 26 + 7)

| テストカテゴリ | 説明 | テスト数 | 実装方法 |
|--------------|------|---------|---------|
| ユーザー名バリデーション | 共通ユーザー名テストスイート | 13件 | `testUsernameValidation()` |
| パスワードバリデーション | 共通パスワードテストスイート | 26件 | `runAllPasswordTests()` |
| 組み合わせバリデーション | ユーザー名とパスワードの組み合わせ | 7件 | `testCombinedValidation()` |

**ファイル**: res/tests/user-addition.test.js

---

### admin-edit.test.js

管理者ユーザー編集画面のテスト。

**テスト数**: 63件（パスワード26件 + ユーザー名13件 + ユーザー編集23件 + サマリー1件）

| テストカテゴリ | 説明 | テスト数 | 実装方法 |
|--------------|------|---------|---------|
| パスワードバリデーション | 共通パスワードテストスイート | 26件 | `runAllPasswordTests()` |
| ユーザー名バリデーション | 共通ユーザー名テストスイート | 13件 | `testUsernameValidation()` |
| ユーザー編集バリデーション | 共通ユーザー編集テストスイート | 23件 | `runAllUserEditTests()` |
| テストサマリー | テスト数の確認 | 1件 | 個別実装 |

**ファイル**: res/tests/admin-edit.test.js

---

### general-user-edit.test.js

一般ユーザー編集画面のテスト。

**テスト数**: 63件（パスワード26件 + ユーザー名13件 + ユーザー編集23件 + サマリー1件）

| テストカテゴリ | 説明 | テスト数 | 実装方法 |
|--------------|------|---------|---------|
| パスワードバリデーション | 共通パスワードテストスイート | 26件 | `runAllPasswordTests()` |
| ユーザー名バリデーション | 共通ユーザー名テストスイート | 13件 | `testUsernameValidation()` |
| ユーザー編集バリデーション | 共通ユーザー編集テストスイート | 23件 | `runAllUserEditTests()` |
| テストサマリー | テスト数の確認 | 1件 | 個別実装 |

**ファイル**: res/tests/general-user-edit.test.js

---

### login.test.js

ログイン画面のテスト。

**テスト数**: 58件

| テストカテゴリ | 説明 | テスト数 |
|--------------|------|---------|
| 空フィールドバリデーション | 空のユーザー名・パスワードの検証 | 10件 |
| ユーザー名バリデーション | ユーザー名の有効性検証 | 8件 |
| パスワードバリデーション | パスワードの有効性検証 | 5件 |
| ログイン状態管理 | ログイン成功後の状態管理 | 8件 |
| フォーム表示 | フォームの表示/非表示制御 | 12件 |
| フォームクリア | フォームのクリア処理 | 5件 |
| エラーメッセージ | エラーメッセージのフォーマット | 10件 |

#### 詳細テストケース例

**Empty field validation (10件)**

| テスト名 | 説明 |
|---------|------|
| `should reject when both username and password are empty` | ユーザー名とパスワードが両方空の場合を拒否 |
| `should reject when username is empty` | ユーザー名が空の場合を拒否 |
| `should reject when password is empty` | パスワードが空の場合を拒否 |
| その他... | ... |

**ファイル**: res/tests/login.test.js

---

### user-deletion.test.js

ユーザー削除機能のテスト。

**テスト数**: 46件

| テストカテゴリ | 説明 | テスト数 |
|--------------|------|---------|
| ユーザー名フォーマット | ユーザー名の表示フォーマット | 8件 |
| ユーザーデータバリデーション | ユーザーデータの有効性確認 | 12件 |
| モーダル状態 | 削除確認モーダルの状態管理 | 10件 |
| エッジケース | 特殊ケースのテスト | 4件 |
| 削除順序テスト | 複数ユーザー削除の順序テスト | 12件 |

#### 詳細テストケース例

**Deletion Order Tests (12件)**

| テスト名 | 説明 |
|---------|------|
| `Three users - Delete last user` | 3ユーザー中の最後のユーザーを削除 |
| `Three users - Delete middle user` | 3ユーザー中の中間のユーザーを削除 |
| `Three users - Delete first user` | 3ユーザー中の最初のユーザーを削除 |
| `Multiple deletions` | 複数のユーザーを連続削除 |
| その他... | ... |

**ファイル**: res/tests/user-deletion.test.js

---

## 機能別テスト

### transaction-edit.test.js

取引編集機能のテスト。

**テスト数**: 112件

| テストカテゴリ | 説明 | テスト数 |
|--------------|------|---------|
| モーダル状態管理 | モーダルの開閉・状態制御 | 25件 |
| データロード | 取引データの読み込み | 35件 |
| 日時フォーマット変換 | SQLite ⇔ datetime-local変換 | 18件 |
| カテゴリ変更と口座リセット | カテゴリ変更時の口座リセット処理 | 24件 |
| メモハンドリング | メモの正規化・表示処理 | 10件 |

**ファイル**: res/tests/transaction-edit.test.js

---

### transaction-detail-management.test.js

取引明細管理機能のテスト。

**テスト数**: 51件 (jest 実測)

| テストカテゴリ | 説明 |
|--------------|------|
| カテゴリ選択ロジック | カテゴリ選択時の動作 |
| 金額バリデーション | 金額入力の検証 |
| 税率バリデーション | 税率入力の検証 |
| 金額フォーマット | 金額の表示フォーマット |
| 税種別選択 | 税込/税抜の選択 |
| メモバリデーション | メモ入力の検証 |
| 明細IDバリデーション | 明細IDの検証 |
| 税計算フィールド決定 | 税計算に使用するフィールドの決定 |
| 入力フィールド状態管理 | 入力フィールドの有効/無効制御 |

**ファイル**: res/tests/transaction-detail-management.test.js

---

### transaction-detail-tax-calculation.test.js

取引明細の税計算機能のテスト。

**テスト数**: 30件 (jest 実測)

| テストカテゴリ | 説明 |
|--------------|------|
| 税抜→税込計算 | 税抜金額から税込金額を計算 |
| 税込→税抜計算 | 税込金額から税抜金額を計算 |
| 丸め誤差検出 | 税計算の丸め誤差検出 |
| エッジケース | 0円、負の値などのエッジケース |
| 複数税率 | 異なる税率での計算 |
| 三者自動整合 (Fable-5 #8) | `calculateFromIncluding` の pure helper: `excluded + tax` は常に入力した税込額。`tax = round(excluded * rate)` を満たす分割があればそれを使う |
| 入力不能な税込額 (潜在スキャン scan2-T2) | 税抜の式で表せない税込額 (10 %・切り捨ての 1000 円など) も入力どおり残し、税額は `税込 - 税抜` で切り出す。1〜10,000 円 × 8/10 % × 3 丸めで全額が保たれることを確認 |
| pure helper 経路 | `calculateFromExcluding` / `applyTaxRounding` の直接テスト (floor / half-up / ceil / unknown default) |

**ファイル**: res/tests/transaction-detail-tax-calculation.test.js

---

### toast.test.js

共有 `toast.js` (bottom-right transient notification) の動作テスト。マウント処理・レンダリング・variant 分岐・自動消去タイマー。

**テスト数**: 14件

| テストカテゴリ | 説明 |
|--------------|------|
| マウント | style/container の自己マウント、多重マウント抑止 |
| レンダリング | メッセージ表示、variant クラス、複数トーストの並び |
| クリア | `clearAllToasts()` の挙動、setTimeout との整合 |

**ファイル**: res/tests/toast.test.js

---

### tax-calc.test.js

税計算ユーティリティのテスト (`tax-calc.js`)。税抜⇔税込変換、丸めモード。

**テスト数**: 12件

| テストカテゴリ | 説明 |
|--------------|------|
| 税抜→税込 | 8%/10% での加算計算 |
| 税込→税抜 | 8%/10% での逆算 |
| 丸め処理 | floor / round / ceil の 3 モード |
| エッジケース | 0円、端数 |
| 内税/外税 (潜在監査 H5/L1) | AMOUNT は常に税抜。外税は AMOUNT を税率単位で gross-up (税額 0 円の少額明細も)、内税は AMOUNT_INCLUDING_TAX を合算し欠損行は導出 |

**ファイル**: res/tests/tax-calc.test.js

---

### product-autocomplete.test.js

商品オートコンプリート UI のテスト。マスタからの候補提示・キーボード操作・選択確定。

**テスト数**: 10件

| テストカテゴリ | 説明 |
|--------------|------|
| 候補表示 | 入力キーワードに応じた suggestion 一覧 |
| キーボード操作 | 上下キー・Enter での選択 |
| 選択確定 | フォーム反映と autocomplete 閉じ |
| クリア | 入力クリア時の suggestion 消去 |

**ファイル**: res/tests/product-autocomplete.test.js

---

### product-draft.test.js

商品マスタの下書き (draft) 状態管理テスト。未保存商品の一時保持と復元。

**テスト数**: 11件

| テストカテゴリ | 説明 |
|--------------|------|
| draft 保存 | sessionStorage への draft 書き込み |
| draft 復元 | 遷移復帰時の form 再構築 |
| draft 破棄 | 保存成功後 / 明示キャンセル時の cleanup |

**ファイル**: res/tests/product-draft.test.js

---

### product-master-jump-draft.test.js

入出金画面から商品マスタへの側訪 (jump) と、戻り時の draft 引き継ぎテスト。

**テスト数**: 11件

| テストカテゴリ | 説明 |
|--------------|------|
| jump | 入出金画面から商品マスタへの遷移と source 記録 |
| 新規商品作成 | マスタ側で保存した商品を入出金 draft に紐付け |
| 復帰 | source 画面へ戻った際の draft 復元 |

**ファイル**: res/tests/product-master-jump-draft.test.js

---

### modal-double-submit.test.js

共有 `Modal` クラス (`res/js/modal.js`) の `_handleSave` 再入ガードに対する回帰テスト（Fable-5 レビュー #D2 修正）。Save ボタン連打・Enter 連打で `onSave` が並行発火し、Rust マスタ CRUD 側の SELECT-then-INSERT 重複チェック（TOCTOU）を両方通過して 2 発目が生の `UNIQUE constraint failed` で失敗する経路を防ぐ。

**テスト数**: 5件

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `form-submit rapid-fire invokes onSave only once` | フォーム submit を短時間に3回発火 | onSave 呼び出しは1回のみ |
| `save button is disabled while onSave is pending` | 保存中の Save ボタン状態 | disabled=true → 完了後 false |
| `after a successful save, a second open+submit fires onSave again` | 保存成功後に再オープン | 2回目の submit で onSave が発火 |
| `after a failed save, the guard resets and retry fires onSave again` | 保存失敗後のリトライ | ガードが解除されリトライ成功 |
| `rapid save-button clicks invoke onSave only once` (saveButtonId パス) | Save ボタン ID 経由での連打 | onSave 呼び出しは1回のみ |

**ファイル**: res/tests/modal-double-submit.test.js

---

### modal-stale-save-close.test.js

共有 `Modal` クラス (`res/js/modal.js`) の保存セッションに対する回帰テスト (潜在監査 L22)。保存中にモーダルを閉じて開き直した後、先の保存が完了しても、開き直したモーダルを閉じたり保存ガードを解除したりしない。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[L22] should not close or reset a re-opened modal when an earlier save finishes` | 先の保存が完了しても、開き直したモーダルは開いたまま・ローディング表示も維持 |
| `[L22] should let a re-opened modal save while an earlier save is still pending` | 先の保存が未完了でも、開き直したモーダルから保存でき、完了時に閉じる |

**ファイル**: res/tests/modal-stale-save-close.test.js

---

### master-crud.test.js

共有 `res/js/master-crud.js` の `saveMasterEntry` オーケストレーターと `mapMasterErrorCode` 分類器の単体テスト (Fable-5 レビュー #D3/#D4/#23)。Rust 側 `ApiError { code, message, entity? }` を JS 側で `err.code` ベースに分類し、i18n key へマップする契約を検証。

**テスト数**: 30件

| テストブロック | 説明 | テスト数 |
|--------------|------|---------|
| `mapMasterErrorCode` | 各 code (`duplicate_name` / `duplicate_code` / `not_found` / `manufacturer_not_found` / `admin_protected` / `validation` の 3 サブケース / `database` / legacy string 3 パス) が正しい i18n key と inline/toast ルーティングを返す | 14件 |
| `formatApiError` | `{ code, message, entity }` オブジェクトから message 抽出、Error インスタンス、legacy 文字列、null/undefined 各種入力での `[object Object]` 回避 (Devin #99 レビュー対応) | 5件 |
| `saveMasterEntry — validation before invoke` | client 側 empty / name-too-long / memo-too-long がバックエンド invoke より前に inline エラーで蹴られる | 3件 |
| `saveMasterEntry — edit target vanished` | editingId で cache に見つからない場合の `onNotFoundBeforeInvoke` 呼び出し + デフォルト toast fallback | 2件 |
| `saveMasterEntry — happy path` | 新規追加 / 更新 それぞれで `invokeAdd`/`invokeUpdate` が正しい引数で呼ばれ `onSuccess` が発火 | 2件 |
| `saveMasterEntry — backend error re-throws and classifies` | `duplicate_name` → inline、`manufacturer_not_found` → toast (product スコープ)、いずれも throw で Modal は開いたまま。backend not_found (invoke 後) は cache-miss と同じく `onNotFoundBeforeInvoke` を通して一覧を再読み込み + Modal を閉じる (Devin #97 レビュー対応の 2 件を含む) | 4件 |

**ファイル**: res/tests/master-crud.test.js

---

### attach-char-counter-ime.test.js

共有ヘルパー `attachCharCounter` (`res/js/validation-display.js`) の IME 変換中ガード回帰テスト (Fable-5 レビュー #D1)。maxlength 撤去で露見した「IME 変換中に `input` イベントで `.value` を切り詰めると変換バッファが壊れる」経路を防ぐ。加えて既存の非 IME 動作 (プレーン typing 切り詰め、コードポイント計数、idempotent detach) の基準テストも含む。

**テスト数**: 8件

| テストブロック | 説明 | テスト数 |
|--------------|------|---------|
| 非 IME baseline | 単純タイピング切り詰め / 初期値切り詰め / 表示カウンター描画 | 3件 |
| IME composition ガード | compositionstart 中は `.value` を書き換えない / compositionend で切り詰め / 連続 composition / idempotent (二重 attach でリスナー累積しない) / detach で全リスナー除去 | 5件 |

**ファイル**: res/tests/attach-char-counter-ime.test.js

---

### aggregation-error-translate.test.js

`res/js/aggregation-common.js` の `translateAggregationError` 型防御ピン (Fable-5 レビュー #9)。旧実装は `error.toString()` を直接呼んでおり、バックエンドが `ApiError { code, message }` 形式で例外を返した瞬間に substring マッチが全部外れ、ユーザーには banner に `"[object Object]"` の文字列が出ていた。修正で `formatApiError` 経由に統一され、`Err(String)` / `ApiError` / `Error` のいずれの形状も同じ i18n キーに解決するようになったのを固定する。

**テスト数**: 13件

| テストブロック | 説明 | テスト数 |
|--------------|------|---------|
| Legacy string errors (`Err(String)`) | Invalid year / month / date range / day / date format の 5 経路 + 未マッチ時の raw fallback | 6件 |
| ApiError 形状 (`{ code, message }`) | `.message` を substring マッチに使い、未マッチでも `.message` を返し `"[object Object]"` を出さない | 2件 |
| Error instance | `Error.message` が substring 分岐にちゃんと渡る | 1件 |
| hostile shapes | `.message` の無い object / null / undefined / 空 `.message` を汎用 i18n バナー (`aggregation.error_generic`) にすげ替え、`"[object Object]"` / `"null"` / `"undefined"` / `""` の literal を絶対に見せない | 4件 |

**ファイル**: res/tests/aggregation-error-translate.test.js

---

### aggregation-latest-request.test.js

集計画面の「最後の要求かどうか」を判定する共通部品 `createLatestRequestGuard` (aggregation-common.js) のテスト (潜在スキャン scan2-A4)。画面ごとの動作は `pages/aggregation-*-stale.test.js` で確かめる。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[scan2-A4] only the most recent request is latest` | 最後に始めた要求だけが「最後」と判定され、次の要求が始まると前の要求は「最後」でなくなる |
| `[scan2-A4] each guard counts its own requests` | 部品ごとに独立して数える (別画面の要求に影響されない) |

**ファイル**: res/tests/aggregation-latest-request.test.js

---

### parse-amount-strict.test.js

`res/js/parse-amount-strict.js` の `parseAmountStrict` 金額パーサの accept/reject テーブル (Fable-5 レビュー #10)。旧実装 `parseInt(el.value) || 0` は `"1099.5"` → 1099 (0.5 円損失)、`"1,099"` → 1 (99% ずれ) を無警告で通していた。新パーサは trim 後に `/^\d+$/` を要求し、空文字と null/undefined は既存の `|| 0` 挙動を保つため 0 を返す。呼び出し側は明細フォーム / 入出金フォーム / 繰り返しルールフォームの 3 経路。

**テスト数**: 24件

| テストブロック | 説明 | テスト数 |
|--------------|------|---------|
| accept | 純粋整数 / "0" / 先頭 0 / 前後空白 / 大整数 | 5件 |
| empty inputs default to 0 | 空 / 空白のみ / null / undefined | 4件 |
| reject (Fable-5 #10 pin cases) | 小数 / "0.5" / カンマ区切り / 指数表記 / 末尾ゴミ / 先頭ゴミ / 符号 (`-5` `+5`) / 内部空白 / 全角数字 / 単独ピリオド / 末尾ピリオド / `2^53-1` は受理 / `2^53` は拒否 / `9007199254740993` は precision loss なので拒否 | 15件 |

**ファイル**: res/tests/parse-amount-strict.test.js

---

### format-local-date.test.js

`res/js/format-local-date.js` の `formatLocalDate` タイムゾーン安全な `YYYY-MM-DD` フォーマッタのテスト (Fable-5 レビュー #13)。旧実装は `new Date().toISOString().slice(0, 10)` (UTC 変換) で、JST ユーザーが 09:00 JST 前に繰り返しルールモーダルを開くと start-date / end-date / anchor-date が全て前日になっていた。ローカル getter (getFullYear / getMonth / getDate) で組み立てる新実装は、実行タイムゾーンに依存せず常に「ローカル壁時計の日付」を返すので、テストはローカル `Date` コンストラクタ経由で書いてある。

**テスト数**: 16件

テストファイルの先頭で `process.env.TZ = 'Asia/Tokyo'` を pin — CodeRabbit on #134 指摘、UTC 実行では local getter と `.toISOString()` の結果が一致するため UTC 回帰が検出できない問題を解消。

| テストブロック | 説明 | テスト数 |
|--------------|------|---------|
| normal cases | 通常日付 / 月ゼロ埋め / 日ゼロ埋め / 両方ゼロ埋め / 12月 / 深夜 0 時 / 23:59:59 | 7件 |
| Fable-5 #13 pin (does not drift to UTC) | UTC 21:30 → JST 翌日 06:30 / UTC 15:30 → JST 翌日 00:30 (UTC/local 発散を確実に検出) + ローカル 06:30 / 23:30 の壁時計固定 | 4件 |
| boundary years | 1900 / 2100 / 閏年 2月29日 / 年 1 (4桁ゼロ埋め) / 年 999 (4桁ゼロ埋め) | 5件 |

**ファイル**: res/tests/format-local-date.test.js

---

### period-end-date.test.js

`res/js/period.js` の `fetchMonthlyPeriodEndDate` のテスト (潜在監査 L14)。ダッシュボードの口座残高がカレンダーの月末基準で、起算日のカスタマイズ (例: 25 日始まり) とずれていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[L14] should return the last day of the user's monthly period` | `get_monthly_period_bounds` の期間最終日を返す (起算日・休日シフト適用済み) |
| `[L14] should fall back to the calendar month end when the backend fails` | バックエンドが失敗したらカレンダーの月末 (うるう年対応) を返す |

**ファイル**: res/tests/period-end-date.test.js

---

### period-containing.test.js

`findMonthlyPeriodContaining` / `findYearlyPeriodContaining` (period.js) のテスト (潜在スキャン scan2-A3)。年度も開始日の年で名前が付くので、暦の年の年度が今日を含まないことがある。月次の期間は開始日の月で名前が付くので、暦の月の期間が今日を含まないことがある。ダッシュボードはこの関数で、今日を含む期間を開いたときの対象月にする。

**テスト数**: 16件

| テスト | 説明 |
|--------|------|
| `should keep the calendar month when its period contains the date` | 暦の月の期間が日付を含むなら、その月を返す |
| `should step back when the calendar month's period starts after the date` | 暦の月の期間が日付より後に始まるなら前月を返す (起算日 25 日。1 月 → 前年 12 月の年またぎも) |
| `should step forward when the calendar month's period ended before the date` | 休日シフトで暦の月の期間が日付より前に終わっているなら翌月を返す (12 月 → 翌年 1 月の年またぎも) |
| `should keep stepping when the neighbouring month does not contain the date either` | 隣の月の期間も日付を含まないときは、含む月まで進む (起算日 31 日・翌営業日。2026 年 1/31 と 2/28 が土曜なので「1 月」は 2/2〜3/1 になり、3/1 は 2 か月前の「1 月」) |
| `should give up on the calendar month when no period ever matches` | どの月の期間も日付を含まない答えが続いたら、上限回数で打ち切って暦の月を返す |
| `should fall back to the calendar month when the backend fails` | バックエンドが答えられないときは暦の月を返す |
| `[scan2-A3] start %i/%i, date %p -> %i` (10 ケース) | 年度の開始が 1/1・4/1・12/31・2/31 (月末に寄せる) のそれぞれで、開始日の前後の日付がどの年度に入るか |

**ファイル**: res/tests/period-containing.test.js

---

### aggregation-render-unspecified.test.js

`res/js/aggregation-common.js` の `renderResults` における unspecified グループの i18n スワップテスト (Fable-5 レビュー #22)。バックエンド (`aggregation.rs`) は SHOP_ID / PRODUCT_ID が NULL のケース、および `account_code === 'NONE'` のケースで空文字を返すよう修正済み。renderResults 側で空文字を `i18n.t('common.unspecified')` に置換することで、英語 UI で「指定なし」のハードコード漏れが banner に出るのを防ぐ。

**テスト数**: 5件

| テストブロック | 説明 | テスト数 |
|--------------|------|---------|
| unspecified-group i18n swap (Fable-5 #22) | 空文字 group_name が i18n ラベルにスワップ / 通常の group_name はそのまま / mixed rows / null group_name も同様 | 4件 |
| regression: no-results path | 空 results 時に no_results i18n セルが正しく表示される | 1件 |

**ファイル**: res/tests/aggregation-render-unspecified.test.js

---

### pages/transaction-detail-page.test.js

実際の明細画面モジュールを `transaction-detail-management.html` に対して起動する回帰テスト (潜在監査 H3)。共通ハーネスは `pages/_page-harness.js`。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `[H3] saving a product-linked detail without changes keeps its productId` | 商品に紐付いた明細を開いて無変更で保存しても `update_transaction_detail` に元の `productId` が送られる |
| `[M19] double submit of the add-detail form invokes add_transaction_detail once` | 保存中の二重送信で `add_transaction_detail` が 1 回しか呼ばれない (潜在監査 M19) |
| `[L7] should show the row total instead of ¥0 when a legacy row has amount_including_tax = 0` | 税込額が 0 の古い明細は ¥0 ではなく AMOUNT + TAX_AMOUNT を表示 (潜在監査 L7) |

**ファイル**: res/tests/pages/transaction-detail-page.test.js

### pages/transaction-detail-included-typing.test.js

実際の明細画面で税込額を 1 文字ずつ入力する回帰テスト (潜在スキャン scan2-T1)。以前は入力のたびに税込欄が書き換えられ、途中の「10」が 9 に変わって保存額がずれていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[T1] typing "100" tax-included at 10 % (floor) keeps 100 and saves 100 / 91 / 9` | 10 %・切り捨てで「100」を 1 文字ずつ入力しても 100 のまま残り、`add_transaction_detail` に 100 / 91 / 9 が送られる |

**ファイル**: res/tests/pages/transaction-detail-included-typing.test.js

### pages/transaction-detail-unreachable-included-price.test.js

税抜の式で表せない税込額を実際の明細画面で入力する回帰テスト (潜在スキャン scan2-T2)。以前は 1000 円が 999 円に書き換えられていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[T2] 1000 tax-included at 10 % (floor) is kept, with excluded 909 and tax 91` | 10 %・切り捨てで 1000 を入力すると 1000 のまま残り、`add_transaction_detail` に 1000 / 909 / 91 が送られる |

**ファイル**: res/tests/pages/transaction-detail-unreachable-included-price.test.js

### pages/transaction-detail-hidden-category.test.js

中分類・小分類を非表示にした後で、その分類の明細を編集する回帰テスト (潜在スキャン scan2-T3)。以前は選択肢に非表示の分類がなく、メモだけ直して保存すると分類が黙って消えていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[T3] a hidden category2 (and its category3) survives a memo-only edit` | 非表示の中分類 (と配下の小分類) の明細をメモだけ変えて保存しても、分類がそのまま送られる |
| `[T3] a hidden category3 under an enabled category2 survives a memo-only edit` | 表示中の中分類の下で非表示にした小分類も同様に残る |

**ファイル**: res/tests/pages/transaction-detail-hidden-category.test.js

---

### pages/transaction-management-page.test.js

実際の入出金画面モジュールを `transaction-management.html` に対して起動する回帰テスト (潜在監査 H4)。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `[H4] saving a header without details does not prompt to overwrite the total with ¥0` | 明細なしヘッダーの保存で ¥0 上書き確認が出ず、`update_transaction_header_total` も送られず、保存フローが一覧再読込まで完了する |
| `[L8] should reject the save without calling update_transaction_header when the transaction date is blank` | 日時が空欄なら `validation.required` を表示して送信せず、モーダルを開いたままにする (潜在監査 L8) |
| `[L5] should move back to the last page when its only row is deleted` | 最終ページの唯一の行を削除すると最後に存在するページへ戻る (潜在監査 L5) |
| `[L5] should keep the newer page when an older page response resolves late` | 古いページ要求の応答が遅れて届いても新しいページの表示を上書きしない (潜在監査 L5) |

**ファイル**: res/tests/pages/transaction-management-page.test.js

---

### pages/user-management-page.test.js

実際のユーザー管理画面モジュールを `user-management.html` に対して管理者セッションで起動する回帰テスト (潜在監査 M13)。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[M13] a whitespace-only username is rejected before create_general_user` | 空白のみのユーザー名は `validation.required` をユーザー名欄に表示し、`create_general_user` を送らない |
| `[M13] a normal username still reaches create_general_user` | 通常のユーザー名は `create_general_user` に送られる (比較用) |

**ファイル**: res/tests/pages/user-management-page.test.js

---

### pages/recurring-rule-page.test.js

実際の繰り返しルール画面モジュールを `recurring-rule.html` に対して起動する回帰テスト (潜在監査 M16)。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `[M16] a TRANSFER from an account to itself is rejected before create_recurring_rule` | 出金元と入金先が同じ振替テンプレートは `transaction_mgmt.transfer_same_account` を表示し、`create_recurring_rule` を送らない |
| `[M16] a TRANSFER between two different accounts still reaches create_recurring_rule` | 異なる口座間の振替は `create_recurring_rule` に送られる (比較用) |
| `[M16] a backend transfer_same_account rejection shows the dedicated message` | バックエンドが `transfer_same_account` で拒否した場合も同じ専用メッセージを表示し、汎用の作成失敗メッセージを出さない |
| `a backend recurring_holiday_shift_too_long rejection shows the localized message` | 休日シフトが 14 日を超えるためバックエンドが拒否したとき、`recurring_rule.holiday_shift_too_long` を表示 (#171 の CodeRabbit 指摘) |

**ファイル**: res/tests/pages/recurring-rule-page.test.js

---

### pages/recurring-rule-double-submit.test.js

実際の繰り返しルール画面での二重送信の回帰テスト (潜在監査 M19)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[M19] double submit invokes create_recurring_rule only once` | `create_recurring_rule` の実行中に送信を重ねても 1 回しか呼ばれない |

**ファイル**: res/tests/pages/recurring-rule-double-submit.test.js

---

### pages/recurring-rule-period-range.test.js

繰り返し予定の期間の回帰テスト (潜在監査 M15 / M18)。祝日データのある年の範囲外では休日シフトが効かず、年の打ち間違い (9999 年など) で大量の予定が生成されていた。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `[M15/M18] should bound the date pickers to the seeded holiday years` | 開始日・終了日の入力欄に (今年 − 5) 年 1/1 〜 (今年 + 10) 年 12/31 の min / max を設定 |
| `[M15/M18] should stop an end date past the limit before create_recurring_rule` | 終了日が上限を超えたら送信せず `recurring_rule.period_out_of_range` を表示 |
| `[M15/M18] should stop a start date before the limit before create_recurring_rule` | 開始日が下限より前でも同様 |
| `[M15/M18] should move the date pickers to bounds that changed since the page loaded` | 送信時に取得した範囲が画面表示時と変わっていたら、日付入力欄の上限・下限も更新する |
| `[M15/M18] should show the same message for a backend recurring_period_out_of_range rejection` | バックエンドの `recurring_period_out_of_range` も同じメッセージで表示 |

**ファイル**: res/tests/pages/recurring-rule-period-range.test.js

### pages/recurring-rule-anchor-follows-start.test.js

毎日の予定の起点日の回帰テスト (潜在スキャン scan2-R2)。以前は起点日の初期値が今日で開始日に追従せず、開始日を前に動かすと今日より前の日が黙って抜けていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `starts as the start date and follows it until the anchor is edited` | 起点日の初期値は開始日。起点日を手で変えるまでは開始日に追従する |
| `follows the start date again after Reset` | リセット後は再び開始日に追従する |

**ファイル**: res/tests/pages/recurring-rule-anchor-follows-start.test.js

### pages/product-management-edit-manufacturer-roundtrip.test.js

商品の編集中にメーカーマスタへ移動して戻る往復の回帰テスト (潜在スキャン scan2-M2)。以前は戻ると「追加」画面になり、保存で同じ商品を追加しようとしていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `comes back in edit mode for the same product (or the jump is not offered in edit mode)` | 戻ると同じ商品の「編集」画面で開き、保存で `update_product` を呼ぶ (`add_product` は呼ばない) |

**ファイル**: res/tests/pages/product-management-edit-manufacturer-roundtrip.test.js

---

### pages/recurring-rule-derived-total.test.js

繰り返し予定の合計金額の回帰テスト (潜在監査 M17)。合計は手入力で初期値 0、明細との整合チェックがなく、0 円の予定が生成されていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[M17] should show a read-only total that follows the detail and tax settings` | 合計欄は読み取り専用で、明細とヘッダーの丸め・内税/外税設定から自動計算される |
| `[M17] should not send a typed total to create_recurring_rule` | `create_recurring_rule` に合計を送らない (バックエンドが明細から計算) |

**ファイル**: res/tests/pages/recurring-rule-derived-total.test.js

---

### pages/recurring-rule-cycle-options.test.js

繰り返し予定の周期オプションの回帰テスト (潜在監査 M14 / L13)。29〜31 日指定は該当日のない月を黙って飛ばし、毎日 + 祝日シフトは同日重複や終了日後の予定を生んでいた。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `[M14] should send DAY_OR_END for a day of the month` | 日付指定は `DAY_OR_END` で送る (該当日のない月は月末) |
| `[M14] should offer an end-of-month mode that sends END` | 「月末」モードを選ぶと `END` を送り、日付欄は隠れる |
| `[L13] should reset and disable the holiday shift for a daily rule` | 「毎日」を選ぶと祝日シフトを「なし」に戻して無効化し、他の周期では再び選べる |

**ファイル**: res/tests/pages/recurring-rule-cycle-options.test.js

---

### pages/recurring-rule-date-order.test.js

繰り返し予定の日付チェックの回帰テスト (潜在スキャン scan2-R8)。以前は開始日 > 終了日などを画面でチェックせず、バックエンドの英語メッセージ (`start_date must be on or before end_date` など) がそのまま表示されていた。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `should reject start > end with a localized message, not the backend English text` | 開始日が終了日より後なら i18n メッセージを出し、`create_recurring_rule` を呼ばない |
| `should reject a daily anchor after the end date with a localized message` | 毎日の起点日が終了日より後なら i18n メッセージを出す |
| `should reject an empty end date with a localized message` | 終了日が空なら i18n メッセージを出す |
| `should reject an empty start date with the same message` | 開始日が空でも同じメッセージを出す (期間範囲外のメッセージにしない) |
| `should still create a rule when the dates are in order` | 日付の順序が正しければ従来どおり作成する |

**ファイル**: res/tests/pages/recurring-rule-date-order.test.js

---

### pages/recurring-rule-reset.test.js

繰り返し予定のリセットボタンの回帰テスト (潜在スキャン scan2-R4)。以前は「毎月」を選んでからリセットすると、ラジオボタンは「毎日」に戻るのに毎月用の欄が表示されたまま、起点日の欄は隠れたまま、休日シフトも選べるままで、開始日・終了日・起点日は空になっていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should bring the cycle UI and the default dates back in line after Reset` | リセット後は「毎日」の表示 (起点日あり・毎月用の欄なし・休日シフトは「なし」で無効) に戻り、既定の日付 (今日 / 1 年後 / 起点日 = 開始日) が入り直す |

**ファイル**: res/tests/pages/recurring-rule-reset.test.js

---

### pages/menu-i18n-seed.test.js

メニューバーの翻訳登録の回帰テスト (潜在スキャン scan2-C2)。以前は `menu.back_to_transactions` が旧スクリプト `sql/add_detail_mgmt_i18n.sql` にしかなく、`res/sql/dbaccess.sql` から作った DB では明細画面の「ファイル」メニューにキー文字列がそのまま表示されていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `menu.back_to_transactions is seeded for ja and en` | `dbaccess.sql` に `menu.back_to_transactions` の ja / en 両方の行がある |
| `every data-i18n key in the menu bar is seeded for ja and en` | 明細画面のメニューバーが描画する `data-i18n` キーがすべて ja / en 両方とも `dbaccess.sql` に登録されている |

**ファイル**: res/tests/pages/menu-i18n-seed.test.js

---

### pages/i18n-literal-user-text.test.js

ユーザーが入力した文字が、メッセージにそのまま差し込まれることを確かめる (潜在スキャン scan2-C4)。

**テスト数**: 7件

| テスト | 説明 |
|--------|------|
| `[scan2-C4] i18n.t() keeps the user name %s literally` (4 件) | ユーザー名の `$&` `$'` `` $` `` `$$` が置き換えの記号として解釈されず、そのまま出る |
| `[scan2-C4] a value containing another placeholder is not substituted again` | 差し込んだ値に `{b}` が含まれていても、もう一度置き換えない (1 回でまとめて置き換える) |
| `[scan2-C4] a placeholder with no param is left as it is` | 値を渡していない `{b}` はそのまま残る |
| `[scan2-C4] recurring-rule delete confirmation keeps the rule name literally` | 繰り返しルールの削除確認で、`$'` や `{1}` を含むルール名がそのまま出て、件数も正しい位置に入る |

**ファイル**: res/tests/pages/i18n-literal-user-text.test.js

---

### pages/dashboard-balance-header.test.js

ダッシュボードの口座別残高の列見出しの回帰テスト (潜在スキャン scan2-C3)。以前は `dashboard.balance` が「収支」(グラフの凡例) と「残高」(列見出し) の 2 回登録されていて、後の行が INSERT OR IGNORE で捨てられ、列見出しが「収支」になっていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `the balance column header resolves to 残高 (ja) / Balance (en)` | `dbaccess.sql` を上から適用した結果で、列見出しのキーが ja「残高」/ en「Balance」になる |

**ファイル**: res/tests/pages/dashboard-balance-header.test.js

---

### pages/transaction-list-none-account-label.test.js

入出金一覧の「指定なし」口座の表示の回帰テスト (潜在スキャン scan2-M8)。以前は DB に保存された口座名「指定なし」をそのまま出していたので、英語表示でも「Main Bank → 指定なし」となっていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `renders the NONE side with common.unspecified, not the stored name` | 口座コードが NONE の側は `common.unspecified` で表示し、保存された「指定なし」は出さない |

**ファイル**: res/tests/pages/transaction-list-none-account-label.test.js

---

### pages/transaction-detail-none-account-label.test.js

明細画面の上部 (取引情報) の「指定なし」口座の表示の回帰テスト (潜在スキャン scan2-M8)。入出金一覧と同じく、保存された口座名「指定なし」がそのまま出ていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `renders a NONE account with common.unspecified, not the stored name` | 口座コードが NONE の口座は `common.unspecified` で表示し、保存された「指定なし」は出さない |

**ファイル**: res/tests/pages/transaction-detail-none-account-label.test.js

---

### pages/dashboard-balance-sign.test.js

ダッシュボードの金額表示の回帰テスト (潜在スキャン scan2-A2)。以前は推移グラフの収支の吹き出しが絶対値で、赤字 3 万円が「¥30,000」と黒字に見えていた。縦軸の目盛りはマイナスが「¥-30,000」、プラスが「¥30K」「¥1.5M」の略記、口座別残高は「¥-1,234」だった。集計画面と同じ「-¥30,000」の形にそろえ、略記はやめて金額をそのまま出す (読み上げや、K・M に慣れていない人への配慮。2026-10-07 ボノさん判断)。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `[scan2-A2] a -30,000 deficit is shown with its minus sign` | 収支 -30,000 の吹き出しが「-¥30,000」になる |
| `[scan2-A2] axis ticks show the full signed amount, without K / M` | 推移グラフと棒グラフの目盛りが「-¥30,000」「¥1,500,000」「¥0」のように略さず符号付きで出る |
| `[scan2-A2] account balances put the minus sign before ¥` | 口座別残高が「¥1,500,000」「-¥1,234」になる |

**ファイル**: res/tests/pages/dashboard-balance-sign.test.js

---

### pages/dashboard-default-period.test.js

ダッシュボードを開いたときの対象月の回帰テスト (潜在スキャン scan2-A3)。以前は暦の月で決めていたので、起算日 25 日・今日 9 月 10 日だと、まるごと未来の「9 月」(9/25〜10/24) が開いていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[scan2-A3] start day 25, today 2026-09-10 -> defaults to the August period that contains today` | 今日を含む「8 月」の期間で開き、その月のデータを読み込む |

**ファイル**: res/tests/pages/dashboard-default-period.test.js

---

### pages/dashboard-stale-reload.test.js

ダッシュボードの再読み込みの回帰テスト (潜在スキャン scan2-A4)。以前は 9 月から 3 月へ素早く切り替えると、遅れて届いた 9 月の結果でグラフと見出しが 9 月に戻っていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[scan2-A4] a slower, older September load does not overwrite the newer March charts` | 古い 9 月の結果は捨てられ、グラフと見出しは 3 月のまま |

**ファイル**: res/tests/pages/dashboard-stale-reload.test.js

---

### single-flight.test.js

送信ハンドラの二重実行防止 `singleFlight` (`res/js/single-flight.js`) のテスト (潜在監査 M19)。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| ignores a second call while the first is in flight | 実行中の再送信を無視する |
| calls preventDefault on every submit, including ignored ones | 無視した送信でも `preventDefault` を呼ぶ |
| accepts a new call after the previous one resolved | 完了後は次の送信を受け付ける |
| releases the guard when the handler throws | ハンドラが例外を投げてもガードを解除する |

**ファイル**: res/tests/single-flight.test.js

---

### pages/product-management-page.test.js

実際の商品マスタ画面の回帰テスト (潜在監査 M5)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[M5] editing a product of a disabled manufacturer keeps manufacturer_id on save` | 無効化されたメーカーに紐付く商品を無変更で保存しても `manufacturer_id` が保たれる (無効メーカーを「（非表示）」付きで選択肢に追加) |

**ファイル**: res/tests/pages/product-management-page.test.js

---

### pages/product-management-link-draft.test.js

明細 → 商品マスタへのジャンプ (`?return_to=`) で商品を追加したときの回帰テスト (潜在監査 L17)。明細の下書きには名前が完全一致した商品だけを紐付け、検索の別候補を紐付けない。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[L17] should leave the detail draft alone when no product name matches exactly` | 完全一致が無ければ、下書きの商品紐付けと品名を変えない |
| `[L17] should link the detail draft to the product whose name matches exactly` | 部分一致の候補が先に並んでも、完全一致の商品を紐付ける |

**ファイル**: res/tests/pages/product-management-link-draft.test.js

---

### pages/shop-management-disabled.test.js

店舗マスタ画面の無効化の回帰テスト (潜在監査 M7)。使用中の店舗は削除できず「代わりに無効化してください」と案内していたが、無効化する手段が無かった。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `[M7] should list disabled shops, marked, only while "show disabled" is on` | 「非表示も表示」で無効な店舗を非表示ラベル付きで一覧に出す |
| `[M7] should not let a late "show disabled" response overwrite a newer list` | 切替の連打で遅れて届いた古い応答が新しい一覧を上書きしない |
| `[M7] should send the disabled checkbox when adding a shop` | 追加時に「非表示」チェックを `isDisabled` として送る |
| `[M7] should show and send the disabled state when editing a shop` | 編集時にチェック状態を表示し、変更を送る (再有効化) |

**ファイル**: res/tests/pages/shop-management-disabled.test.js

---

### pages/transaction-management-disabled-shop.test.js

入出金画面で無効な店舗を使った取引の回帰テスト (潜在監査 M7)。選択肢に無効な店舗が無く「未指定」に落ち、保存で店舗が消えていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[M7] should keep a disabled shop selected when editing a transaction that names it` | 編集時は無効な店舗を非表示ラベル付きで選択したまま保存する |
| `[M7] should not offer a disabled shop for a new transaction` | 新規取引では無効な店舗を選択肢に出さない |

**ファイル**: res/tests/pages/transaction-management-disabled-shop.test.js

---

### pages/account-management-disabled.test.js

口座マスタ画面の無効化の回帰テスト (潜在監査 M7)。使用中の口座は削除できず「代わりに無効化してください」と案内していたが、無効化する手段が無かった。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `[M7] should list disabled accounts, marked, only while "show disabled" is on` | 「非表示も表示」で無効な口座を非表示ラベル付きで一覧に出す (NONE は出さない) |
| `[M7] should not let a late "show disabled" response overwrite a newer list` | 切替の連打で遅れて届いた古い応答が新しい一覧を上書きしない |
| `[M7] should send the disabled checkbox when adding an account` | 追加時に「非表示」チェックを `isDisabled` として送る |
| `[M7] should show and send the disabled state when editing an account` | 編集時にチェック状態を表示し、変更を送る (再有効化) |

**ファイル**: res/tests/pages/account-management-disabled.test.js

### pages/account-management-save-error-keeps-form.test.js

口座マスタの保存失敗時の回帰テスト (潜在スキャン scan2-M4)。以前は保存に失敗しても入力画面が閉じ、入力内容が消えていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `keeps the modal open and the typed input when add_account fails with duplicate_code` | 口座コードの重複でバックエンドが拒否しても、画面が開いたまま入力が残る |
| `keeps the modal open when a whitespace-only name is stopped before add_account` | 空白だけの名前を入力チェックで止めたときも、画面が開いたまま残る |

**ファイル**: res/tests/pages/account-management-save-error-keeps-form.test.js

---

### pages/account-management-validation-i18n.test.js

口座マスタの入力チェックと一覧読み込みエラーの i18n 回帰テスト (潜在スキャン scan2-R8 の続き)。以前は `Account name is required` などの英語が直書きで、口座コード・テンプレート・初期残高のメッセージは存在しない要素を指していたため表示もされなかった。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `empty account code` | 口座コードが空なら入力欄の下に `validation.required` を出す |
| `empty account name` | 口座名が空白だけなら `validation.required` を出す |
| `no template selected` | テンプレート未選択なら `validation.required` を出す |
| `empty initial balance` | 初期残高が空なら `common.error_amount_not_integer` を出す |
| `shows only the localized message, not the backend detail` | 一覧の読み込み失敗時は `account_mgmt.failed_to_load` だけを出し、バックエンドの英語の詳細は出さない |

**ファイル**: res/tests/pages/account-management-validation-i18n.test.js

---

### pages/transaction-management-disabled-account.test.js

入出金画面で無効な口座を使った取引の回帰テスト (潜在監査 M7)。選択肢に無効な口座が無く、保存で口座が失われていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[M7] should keep a disabled account selected when editing a transaction that names it` | 編集時は無効な口座を非表示ラベル付きで選択したまま保存する |
| `[M7] should not offer a disabled account for a new transaction` | 新規取引では無効な口座を選択肢に出さない |

**ファイル**: res/tests/pages/transaction-management-disabled-account.test.js

---

### pages/transaction-management-category1-has-details.test.js

明細がある取引の大分類変更の回帰テスト (潜在監査 M2)。ヘッダーの大分類だけが変わり、収入の明細が支出の円グラフに混ざっていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[M2] should explain why the category cannot change and keep the modal open` | バックエンドの `category1_has_details` を受けて `transaction_mgmt.category1_has_details` を表示し、モーダルを開いたままにする |

**ファイル**: res/tests/pages/transaction-management-category1-has-details.test.js

---

### modal-open-awaits-onopen.test.js

共有 `Modal` クラスの `open()` の回帰テスト (潜在監査 L6)。`onOpen` の完了を待たずに戻っていたため、入出金画面の下書き復元が後から走る初期化で上書きされていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[L6] should settle only after an async onOpen has finished` | 非同期の `onOpen` が終わるまで `open()` の Promise は完了しない (モーダルはすぐ表示) |
| `[L6] should settle at once for a synchronous onOpen` | 同期の `onOpen` ならすぐ完了する |

**ファイル**: res/tests/modal-open-awaits-onopen.test.js

---

### pages/transaction-management-restore-draft.test.js

入出金画面の新規取引の下書き復元の回帰テスト (潜在監査 L6)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[L6] should keep the restored draft instead of the modal's late defaults` | 復元した日付・店舗・メモが、モーダル自身の初期化 (フォームのリセット・現在日時) で上書きされない |

**ファイル**: res/tests/pages/transaction-management-restore-draft.test.js

### pages/transaction-management-shop-roundtrip-draft.test.js

入出金の入力中に店舗管理へ移動して戻る往復の回帰テスト (潜在スキャン scan2-T4)。以前は予定フラグが保存されず、編集中の大分類・口座・税設定も DB の値に戻っていた。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `[T4a] a new scheduled transaction is still scheduled after the round trip` | 新規で「予定」にチェックして往復しても、チェックが残り予定として保存される |
| `[T4b] edit-mode edits to rounding, account and memo survive the round trip` | 編集中に変えた丸め・口座と、空にしたメモが往復後も残る |
| `[T4c] a category1 cleared in edit mode stays cleared after the round trip` | 編集中に空にした大分類が、往復後も DB の値に戻らず空のまま残る (#170 の CodeRabbit 指摘) |

**ファイル**: res/tests/pages/transaction-management-shop-roundtrip-draft.test.js

### pages/transaction-management-rejected-save-keeps-form.test.js

入出金の保存が止められた・失敗したときの回帰テスト (潜在スキャン scan2-T5)。以前は入力画面が閉じ、入力内容が消えていた。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `[T5] TRANSFER with both accounts "Unspecified" keeps the form open` | 出金元・入金先とも「指定なし」の振替を止めても、画面が開いたまま入力が残る |
| `[T5] a total rejected by parseAmountStrict ("1e3") keeps the form open` | 金額 `1e3` を入力チェックで止めても、画面が開いたまま残る |
| `[T5] a generic backend error keeps the form open` | バックエンドの一般的なエラーでも、画面が開いたまま残る |

**ファイル**: res/tests/pages/transaction-management-rejected-save-keeps-form.test.js

---

### pages/transaction-management-restore-disabled-shop.test.js

下書き保存後に無効化された店舗の復元の回帰テスト (潜在監査 M7、L6 修正で到達可能になった)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[M7] should not give a restored new transaction a shop disabled since the draft was saved` | 新規取引の復元では無効化された店舗を選ばず「未指定」にする |

**ファイル**: res/tests/pages/transaction-management-restore-disabled-shop.test.js

---

### pages/transaction-management-restore-reopened.test.js

下書き復元の途中でモーダルを閉じて開き直した場合の回帰テスト (潜在監査 L6)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[L6] should not write the draft into a modal reopened while the restore was waiting` | 復元が待っている間に閉じて開き直したモーダルには、古い下書きを書き込まない |

**ファイル**: res/tests/pages/transaction-management-restore-reopened.test.js

---

### pages/aggregation-monthly-page.test.js

実際の月次集計画面の回帰テスト (潜在監査 M11 / M12)。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `[M12] empty group_name renders as common.unspecified` | 空の `group_name` を `common.unspecified` で表示 |
| `[M11] account axis: ...` | 口座軸の合計行は件数・平均を「—」で表示 (振替の二重計上を避ける) |
| `[M11] category2 axis: ...` | 費目2軸の合計行も件数・平均を「—」で表示 |
| `[M11] category1 axis still sums the count into the total row` | 費目1軸では従来どおり件数を合計 (比較用) |
| `[L12] should put the minus sign before the yen symbol when the amount is negative` | 負の金額を「-¥1,234」と表示 (潜在監査 L12) |

**ファイル**: res/tests/pages/aggregation-monthly-page.test.js

---

### pages/aggregation-yearly-total-count.test.js

実際の年次集計画面 (共通レンダラ `renderResults`) の回帰テスト (潜在監査 M11)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[M11] account axis: one transfer is not counted twice in the shared total row` | 口座軸の合計行は件数・平均を「—」で表示 |

**ファイル**: res/tests/pages/aggregation-yearly-total-count.test.js

### pages/aggregation-default-period-monthly.test.js

月次集計を開いたときの対象月の回帰テスト (潜在スキャン scan2-A3)。以前は暦の月で決めていたので、起算日 25 日・今日 9 月 10 日だと、まるごと未来の「9 月」が開いていた (ダッシュボードと同じ問題、#178 で修正済み)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[scan2-A3] start day 25, today 2026-09-10 -> opens on the August period that contains today` | 今日を含む「8 月」の期間で開く |

**ファイル**: res/tests/pages/aggregation-default-period-monthly.test.js

### pages/aggregation-default-period-yearly.test.js

年次集計を開いたときの対象年の回帰テスト (潜在スキャン scan2-A3)。以前は暦の年で決めていたので、年度の開始が 4/1・今日 2026 年 2 月 10 日だと、まるごと未来の「2026 年度」(2026/4/1〜2027/3/31) が開いていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[scan2-A3] year starting 04-01, today 2026-02-10 -> opens on the 2025 period that contains today` | 今日を含む「2025 年度」で開く |

**ファイル**: res/tests/pages/aggregation-default-period-yearly.test.js

### pages/aggregation-monthly-stale.test.js

月次集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `[scan2-A4] a slower, older result arriving later is dropped` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `[scan2-A4] an older request failing later neither shows its error nor clears the table` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `[scan2-A4] the loading state stays until the latest request finishes` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `[scan2-A4] an Execute stopped by the input checks does not strand the running request` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-monthly-stale.test.js

### pages/aggregation-daily-stale.test.js

日次集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `[scan2-A4] a slower, older result arriving later is dropped` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `[scan2-A4] an older request failing later neither shows its error nor clears the table` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `[scan2-A4] the loading state stays until the latest request finishes` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `[scan2-A4] an Execute stopped by the input checks does not strand the running request` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-daily-stale.test.js

### pages/aggregation-weekly-stale.test.js

週次集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `[scan2-A4] a slower, older result arriving later is dropped` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `[scan2-A4] an older request failing later neither shows its error nor clears the table` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `[scan2-A4] the loading state stays until the latest request finishes` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `[scan2-A4] an Execute stopped by the input checks does not strand the running request` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-weekly-stale.test.js

### pages/aggregation-period-stale.test.js

期間指定集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `[scan2-A4] a slower, older result arriving later is dropped` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `[scan2-A4] an older request failing later neither shows its error nor clears the table` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `[scan2-A4] the loading state stays until the latest request finishes` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `[scan2-A4] an Execute stopped by the input checks does not strand the running request` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-period-stale.test.js

### pages/aggregation-yearly-stale.test.js

年次集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `[scan2-A4] a slower, older result arriving later is dropped` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `[scan2-A4] an older request failing later neither shows its error nor clears the table` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `[scan2-A4] the loading state stays until the latest request finishes` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `[scan2-A4] an Execute stopped by the input checks does not strand the running request` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-yearly-stale.test.js

### pages/dashboard-bar-top10.test.js

実際のダッシュボードを起動する回帰テスト (潜在スキャン scan2-A1)。支出の合計は負の値なのに符号付きで降順に並べていたため、棒グラフには小さい支出から並び、上位 10 件から最大の支出 (家賃など) が落ちていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[scan2-A1] largest expense category is drawn first and kept in the top 10` | 支出を金額の大きさ順に並べ、最大の支出が先頭に来て上位 10 件に残る |

**ファイル**: res/tests/pages/dashboard-bar-top10.test.js

---

### pages/index-setup-page.test.js

初回セットアップ画面 (`index.html` 上の menu.js) の回帰テスト (潜在監査 L25)。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `[L25] should reject the admin setup without calling register_admin when the username is blank` | 空白のみのユーザー名は `register_admin` を送らず `error.username_required` を表示 |
| `[L25] should report the username, not the password, when the backend rejects a blank username` | バックエンドの「Username cannot be empty」をパスワードではなくユーザー名のエラーとして表示 |
| `[L25] should show the duplicate-username message when the backend reports duplicate_name` | `duplicate_name` で `error.username_duplicate` を表示 |

**ファイル**: res/tests/pages/index-setup-page.test.js

---

### pages/category-management-page.test.js

実際の費目管理画面の回帰テスト (潜在監査 L19)。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[L19] should show the not-found message and reload the tree when moving a vanished category` | 存在しなくなった費目の移動で `category_mgmt.not_found` を表示しツリーを再読込 |
| `[L19] should show the not-found message and reload the tree when showing a vanished category` | 存在しなくなった費目の再表示でも同様 |

**ファイル**: res/tests/pages/category-management-page.test.js

---

### pages/category-management-move-buttons.test.js

実際の費目管理画面で、↑/↓ ボタンが非表示の費目を数えないことを確かめる (潜在スキャン scan2-M5)。非表示の費目は表示中の費目の後ろに並ぶ。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[scan2-M5] the last visible CATEGORY2 cannot move down past hidden ones` | 表示中の最初の中分類は「↑」、最後の中分類は「↓」が押せない (後ろに非表示があっても)。非表示の行には ↑/↓ が無い |
| `[scan2-M5] the last visible CATEGORY3 cannot move down past hidden ones` | 小分類でも同じ |

**ファイル**: res/tests/pages/category-management-move-buttons.test.js

---

### pages/transaction-management-filter-hidden-category.test.js

実際の入出金一覧で、費目フィルタに非表示の費目も出ることを確かめる (潜在スキャン scan2-M7)。入力用の選択肢からは外したまま。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `[scan2-M7] offers a hidden CATEGORY2 and its CATEGORY3, labelled as hidden` | 非表示の中分類「外食」とその小分類が `common.disabled_label` 付きで選べる。表示中の「食費」はラベル無しのまま |

**ファイル**: res/tests/pages/transaction-management-filter-hidden-category.test.js

---

### pages/user-management-password-page.test.js

ユーザー管理画面 (管理者セッション) のパスワード検証の回帰テスト (潜在監査 L24 / L31)。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[L24] 16-space password is reported on the password, ...` | 空白16文字のパスワードは、未定義キー `user_mgmt.empty_name` をユーザー名欄に出さず、パスワードのエラーとして表示 |
| `[L31] 8 emoji (16 UTF-16 units, 8 chars) is rejected by the frontend length check` | 文字数を UTF-16 単位ではなく文字 (コードポイント) で数え、絵文字8文字を拒否 |

**ファイル**: res/tests/pages/user-management-password-page.test.js

---

### pages/user-management-nonadmin-page.test.js

ユーザー管理画面 (一般ユーザーセッション) の回帰テスト (潜在監査 L30)。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[L30] a non-admin user is not offered the Add User button` | 一般ユーザーには「ユーザー追加」ボタンを出さない |
| `[L30] a non-admin user has no delete button on their own row` | 一般ユーザーの自分の行に削除ボタンを出さない |

**ファイル**: res/tests/pages/user-management-nonadmin-page.test.js

---

### pages/index-setup-password-length.test.js

初回セットアップ画面のパスワード文字数の回帰テスト (潜在監査 L31)。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `[L31] admin setup rejects 8 emoji (8 chars) on the frontend` | 管理者セットアップで絵文字8文字を拒否 |
| `[L31] user setup rejects 8 emoji (8 chars) on the frontend` | 一般ユーザーセットアップでも同様 |

**ファイル**: res/tests/pages/index-setup-password-length.test.js

---

## 集計機能テスト

### aggregation-daily.test.js

日次集計機能のテスト。

**テスト数**: 16件 (jest 実測)

| テストカテゴリ | 説明 |
|--------------|------|
| UI初期化 | 画面の初期表示 |
| 日付入力バリデーション | 日付入力の検証 |
| 集計実行 | 集計処理の実行 |
| Enterキー実行 | Enterキーでの集計実行 |
| グルーピング軸変更 | 集計軸の変更 |
| 口座メモ表示 | 口座メモの表示 |

**ファイル**: res/tests/aggregation-daily.test.js

---

### aggregation-weekly.test.js

週次集計機能のテスト。

**テスト数**: 22件 (jest 実測)

| テストカテゴリ | 説明 |
|--------------|------|
| UI初期化 | 画面の初期表示 |
| 基準日バリデーション | 基準日入力の検証 |
| 週開始曜日選択 | 週の開始曜日選択 |
| 集計実行 | 集計処理の実行 |
| 週範囲計算 | 週の範囲計算 |
| グルーピング軸変更 | 集計軸の変更 |
| 口座メモ表示 | 口座メモの表示 |
| 異なる曜日 | 異なる曜日での動作 |

**ファイル**: res/tests/aggregation-weekly.test.js

---

### aggregation-monthly.test.js

月次集計機能のテスト。

**テスト数**: 33件 (jest 実測)

| テストカテゴリ | 説明 |
|--------------|------|
| UI初期化 | 画面の初期表示 |
| 年入力バリデーション | 年入力の検証 |
| 月選択 | 月の選択 |
| 年スピナーボタン | 年の増減ボタン |
| 集計実行 | 集計処理の実行 |
| 口座メモ表示 | 口座メモの表示 |
| フィルタートグル | フィルターの切り替え |
| グルーピング軸変更 | 集計軸の変更 |
| 未来日付バリデーション | 未来日付の検証 |

**ファイル**: res/tests/aggregation-monthly.test.js

---

### aggregation-yearly.test.js

年次集計機能のテスト。

**テスト数**: 21件 (jest 実測)

| テストカテゴリ | 説明 |
|--------------|------|
| UI初期化 | 画面の初期表示 |
| 年入力バリデーション | 年入力の検証 |
| 年スピナーボタン | 年の増減ボタン |
| 年度開始月選択 | 年度開始月の選択 |
| 集計実行 | 集計処理の実行 |
| 会計年度期間 | 会計年度の期間計算 |
| グルーピング軸変更 | 集計軸の変更 |
| 口座メモ表示 | 口座メモの表示 |

**ファイル**: res/tests/aggregation-yearly.test.js

---

### aggregation-period.test.js

期間集計機能のテスト。

**テスト数**: 23件 (jest 実測)

| テストカテゴリ | 説明 |
|--------------|------|
| UI初期化 | 画面の初期表示 |
| 日付範囲バリデーション | 開始日・終了日の検証 |
| 集計実行 | 集計処理の実行 |
| 一般的なユースケース | よくある使い方のテスト |
| グルーピング軸変更 | 集計軸の変更 |
| 口座メモ表示 | 口座メモの表示 |
| 境界値ケース | 境界値のテスト |

**ファイル**: res/tests/aggregation-period.test.js

---

## ブラウザ / スタンドアロン (jest 総計に含めない)

jest では走らない (`node --experimental-vm-modules ... jest.js` の pick 対象外)。ブラウザ上または `node` 直接実行で走らせる補助テスト・ヘルパー群。

### category-management-ui-tests.js

カテゴリ管理 UI の DOM ベーステスト。`console.log`/`console.error` によるアサーション形式で、ブラウザで対象ページを開いたセッションから直接 `<script>` として読み込んで走らせる想定。

**ファイル**: res/tests/category-management-ui-tests.js

---

### tax-rounding-tests.js

`tax-rounding-tests.html` のコンパニオン。純関数 (`applyTaxRounding` 等) の入出力を HTML ページ経由で網羅的に叩く手動テスト。

**ファイル**: res/tests/tax-rounding-tests.js

---

### backend-validation-standalone.js

Tauri 不要な validation ロジック単体テスト。`node backend-validation-standalone.js` で実行。

**ファイル**: res/tests/backend-validation-standalone.js

---

### login-test-standalone.js

Tauri 不要な login ロジック単体テスト。`node login-test-standalone.js` で実行。

**ファイル**: res/tests/login-test-standalone.js

---

### aggregation-test-helpers.js

集計機能テスト (aggregation-*.test.js) が共通で import する mock / fixture ヘルパー。単体でテストとして走ることはない。

**ファイル**: res/tests/aggregation-test-helpers.js

---

## テスト統計サマリー

| カテゴリ | テスト数 |
|---------|---------|
| **共通テストスイート** (ヘルパー — 画面別テストの中で invoke されるため総計には別計上しない) | 56件 |
| password-validation-tests.js | 26 |
| username-validation-tests.js | 20 (13 + 7) |
| user-edit-validation-tests.js | 23 |
| **画面別テスト** | **308件** |
| admin-setup.test.js | 32 |
| user-addition.test.js | 46 |
| admin-edit.test.js | 63 |
| general-user-edit.test.js | 63 |
| login.test.js | 58 |
| user-deletion.test.js | 46 |
| **機能別テスト** | **510件** |
| transaction-edit.test.js | 112 |
| transaction-detail-management.test.js | 51 |
| transaction-detail-tax-calculation.test.js | 30 |
| toast.test.js | 14 |
| tax-calc.test.js | 12 |
| product-autocomplete.test.js | 10 |
| product-draft.test.js | 11 |
| product-master-jump-draft.test.js | 11 |
| modal-double-submit.test.js | 6 |
| modal-stale-save-close.test.js | 2 |
| master-crud.test.js | 30 |
| attach-char-counter-ime.test.js | 8 |
| aggregation-error-translate.test.js | 13 |
| aggregation-latest-request.test.js | 2 |
| parse-amount-strict.test.js | 24 |
| format-local-date.test.js | 16 |
| period-end-date.test.js | 2 |
| period-containing.test.js | 16 |
| aggregation-render-unspecified.test.js | 5 |
| pages/transaction-detail-page.test.js | 3 |
| pages/transaction-detail-included-typing.test.js | 1 |
| pages/transaction-detail-unreachable-included-price.test.js | 1 |
| pages/transaction-detail-hidden-category.test.js | 2 |
| pages/transaction-management-page.test.js | 4 |
| pages/user-management-page.test.js | 2 |
| pages/recurring-rule-page.test.js | 4 |
| pages/recurring-rule-double-submit.test.js | 1 |
| pages/recurring-rule-period-range.test.js | 5 |
| pages/recurring-rule-anchor-follows-start.test.js | 2 |
| pages/product-management-edit-manufacturer-roundtrip.test.js | 1 |
| pages/recurring-rule-derived-total.test.js | 2 |
| pages/recurring-rule-cycle-options.test.js | 3 |
| pages/recurring-rule-date-order.test.js | 5 |
| pages/recurring-rule-reset.test.js | 1 |
| pages/menu-i18n-seed.test.js | 2 |
| pages/i18n-literal-user-text.test.js | 7 |
| pages/dashboard-balance-header.test.js | 1 |
| pages/transaction-list-none-account-label.test.js | 1 |
| pages/transaction-detail-none-account-label.test.js | 1 |
| pages/dashboard-balance-sign.test.js | 3 |
| pages/dashboard-default-period.test.js | 1 |
| pages/dashboard-stale-reload.test.js | 1 |
| single-flight.test.js | 4 |
| pages/product-management-page.test.js | 1 |
| pages/product-management-link-draft.test.js | 2 |
| pages/shop-management-disabled.test.js | 4 |
| pages/transaction-management-disabled-shop.test.js | 2 |
| pages/account-management-disabled.test.js | 4 |
| pages/account-management-save-error-keeps-form.test.js | 2 |
| pages/account-management-validation-i18n.test.js | 5 |
| pages/transaction-management-disabled-account.test.js | 2 |
| pages/transaction-management-category1-has-details.test.js | 1 |
| modal-open-awaits-onopen.test.js | 2 |
| pages/transaction-management-restore-draft.test.js | 1 |
| pages/transaction-management-shop-roundtrip-draft.test.js | 3 |
| pages/transaction-management-rejected-save-keeps-form.test.js | 3 |
| pages/transaction-management-restore-disabled-shop.test.js | 1 |
| pages/transaction-management-restore-reopened.test.js | 1 |
| pages/aggregation-monthly-page.test.js | 5 |
| pages/aggregation-yearly-total-count.test.js | 1 |
| pages/aggregation-default-period-monthly.test.js | 1 |
| pages/aggregation-default-period-yearly.test.js | 1 |
| pages/aggregation-monthly-stale.test.js | 4 |
| pages/aggregation-daily-stale.test.js | 4 |
| pages/aggregation-weekly-stale.test.js | 4 |
| pages/aggregation-period-stale.test.js | 4 |
| pages/aggregation-yearly-stale.test.js | 4 |
| pages/dashboard-bar-top10.test.js | 1 |
| pages/index-setup-page.test.js | 3 |
| pages/category-management-page.test.js | 2 |
| pages/category-management-move-buttons.test.js | 2 |
| pages/transaction-management-filter-hidden-category.test.js | 1 |
| pages/user-management-password-page.test.js | 2 |
| pages/user-management-nonadmin-page.test.js | 2 |
| pages/index-setup-password-length.test.js | 2 |
| **集計機能テスト** | **115件** |
| aggregation-daily.test.js | 16 |
| aggregation-weekly.test.js | 22 |
| aggregation-monthly.test.js | 33 |
| aggregation-yearly.test.js | 21 |
| aggregation-period.test.js | 23 |
| **総計 (jest)** | **933件** |

総計は 画面別 + 機能別 + 集計機能 の合計。共通テストスイートは画面別テストの内部で `runAll*` 経由で invoke されるヘルパー library であり、そのアサーションは既に画面別テストの数に含まれているため、総計には別途加算しない (double-count 防止)。

---

## テストの実行方法

### すべてのテストを実行

```bash
cd res/tests
npm test
```

### 特定のテストファイルのみ実行

```bash
npm test admin-setup.test.js
npm test login.test.js
npm test user-deletion.test.js
```

### 特定のテストケースのみ実行

```bash
npm test -- --testNamePattern="Empty Password"
npm test -- --testNamePattern="Username Validation"
```

### カバレッジレポート生成

```bash
npm run test:coverage
```

### スタンドアロンテスト（Node.js）

```bash
node login-test-standalone.js
node backend-validation-standalone.js
```

### 権威的なカウントを再取得する

`--json` を付けて jest を走らせ、per-file の実測値を採取。統計サマリー更新時はこれをソースに。`--silent` は付けない (assertion 数が 0 になる)。

```bash
cd res/tests
node --experimental-vm-modules node_modules/jest/bin/jest.js --json > /tmp/jest.json
node -e "const j=JSON.parse(require('fs').readFileSync('/tmp/jest.json','utf8')); \
  j.testResults.map(r=>({f:r.name.replace(/^.*\\//,''),n:r.assertionResults.length})) \
  .sort((a,b)=>a.f.localeCompare(b.f)).forEach(r=>console.log(String(r.n).padStart(4)+'  '+r.f)); \
  console.log('total:',j.numTotalTests);"
```

---

## 関連ドキュメント

- [バックエンドテストインデックス](BACKEND_TEST_INDEX.md) - Rustテストの完全一覧
- [テスト概要](TEST_OVERVIEW.md) - テスト戦略と実行ガイド
- [テスト設計](TEST_DESIGN.md) - テストアーキテクチャと設計思想
- [テスト結果](TEST_RESULTS.md) - 最新のテスト実行結果
