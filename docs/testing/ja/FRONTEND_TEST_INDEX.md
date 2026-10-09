# フロントエンドテストインデックス

このドキュメントは、JavaScriptで実装されたフロントエンドテストの完全なインデックスです。

**最終更新**: 2026-10-09 JST  
**総テスト数**: 876件 (jest suite 93 ファイル、`npm test` 実測)

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
- [機能別テスト](#機能別テスト)
  - [transaction-edit.test.js](#transaction-edittestjs)
  - [transaction-detail-management.test.js](#transaction-detail-managementtestjs)
  - [transaction-detail-tax-calculation.test.js](#transaction-detail-tax-calculationtestjs)
  - [toast.test.js](#toasttestjs)
  - [tax-calc.test.js](#tax-calctestjs)
  - [pages/transaction-detail-draft-storage.test.js](#pagestransaction-detail-draft-storagetestjs)
  - [pages/transaction-detail-product-link.test.js](#pagestransaction-detail-product-linktestjs)
  - [pages/product-management-product-draft.test.js](#pagesproduct-management-product-drafttestjs)
  - [pages/manufacturer-management-product-draft.test.js](#pagesmanufacturer-management-product-drafttestjs)
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
  - [pages/transaction-detail-product-suggest.test.js](#pagestransaction-detail-product-suggesttestjs)
  - [pages/transaction-management-page.test.js](#pagestransaction-management-pagetestjs)
  - [pages/user-management-page.test.js](#pagesuser-management-pagetestjs)
  - [pages/user-management-delete-last-user.test.js](#pagesuser-management-delete-last-usertestjs)
  - [pages/user-management-delete-page.test.js](#pagesuser-management-delete-pagetestjs)
  - [pages/index-logout-hides-user-setup.test.js](#pagesindex-logout-hides-user-setuptestjs)
  - [pages/index-login-page.test.js](#pagesindex-login-pagetestjs)
  - [pages/recurring-rule-page.test.js](#pagesrecurring-rule-pagetestjs)
  - [pages/recurring-rule-double-submit.test.js](#pagesrecurring-rule-double-submittestjs)
  - [pages/recurring-rule-period-range.test.js](#pagesrecurring-rule-period-rangetestjs)
  - [pages/recurring-rule-anchor-follows-start.test.js](#pagesrecurring-rule-anchor-follows-starttestjs)
  - [pages/product-management-edit-manufacturer-roundtrip.test.js](#pagesproduct-management-edit-manufacturer-roundtriptestjs)
  - [pages/recurring-rule-derived-total.test.js](#pagesrecurring-rule-derived-totaltestjs)
  - [pages/recurring-rule-cycle-options.test.js](#pagesrecurring-rule-cycle-optionstestjs)
  - [pages/recurring-rule-date-order.test.js](#pagesrecurring-rule-date-ordertestjs)
  - [pages/recurring-rule-reset.test.js](#pagesrecurring-rule-resettestjs)
  - [pages/recurring-rule-rounding-recalc.test.js](#pagesrecurring-rule-rounding-recalctestjs)
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
  - [pages/account-management-code-max-length.test.js](#pagesaccount-management-code-max-lengthtestjs)
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
  - [pages/transaction-management-save-before-details.test.js](#pagestransaction-management-save-before-detailstestjs)
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
| `should reject the password when it is an empty string` | 空文字列のパスワード | エラー: "Password cannot be empty!" |
| `should reject the password when it has only spaces` | スペースのみのパスワード | エラー: "Password cannot be empty!" |
| `should reject the password when it has only tabs` | タブのみのパスワード | エラー: "Password cannot be empty!" |
| `should reject the password when it has only mixed whitespace` | スペース・タブ・改行が混ざった空白のみのパスワード | エラー: "Password cannot be empty!" |
| `should reject the password when it is null` | null のパスワード | エラー: "Password cannot be empty!" |
| `should reject the password when it is undefined` | undefined のパスワード | エラー: "Password cannot be empty!" |

**testPasswordLengthValidation (6件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should reject the password when it is shorter than 16 characters` | 16 文字未満 (5 文字) のパスワード | エラー: "Password must be at least 16 characters long!" |
| `should reject the password when it has a single character` | 1 文字のパスワード | エラー: "Password must be at least 16 characters long!" |
| `should reject the password when it has exactly 15 characters` | ちょうど 15 文字のパスワード | エラー: "Password must be at least 16 characters long!" |
| `should accept the password when it has exactly 16 characters` | ちょうど 16 文字のパスワード | valid: true |
| `should accept the password when it has more than 16 characters` | 16 文字を超えるパスワード | valid: true |
| `should accept the password when it is very long (100 characters)` | 100 文字のパスワード | valid: true |

**testPasswordMatchValidation (6件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should reject the passwords when they do not match` | パスワードと確認が一致しない | エラー: "Passwords do not match!" |
| `should reject when password is correct but confirmation is empty` | パスワードは正しいが確認が空 | エラー: "Passwords do not match!" |
| `should reject when password is correct but confirmation is null` | パスワードは正しいが確認が null | エラー: "Passwords do not match!" |
| `should reject the passwords when they differ only in letter case` | 大文字・小文字だけが違う | エラー: "Passwords do not match!" |
| `should reject when passwords differ by one character` | 1 文字だけ違う | エラー: "Passwords do not match!" |
| `should accept when both passwords match exactly` | 完全に一致する | valid: true |

**testValidPasswordScenarios (8件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should accept the passwords when they match and have 16 characters` | 一致する 16 文字のパスワード | valid: true |
| `should accept password with spaces when matching and long enough` | スペースを含み、一致し 16 文字以上 | valid: true |
| `should accept the password when it has special characters` | 記号を含むパスワード | valid: true |
| `should accept the password when it has unicode characters` | Unicode 文字を含むパスワード | valid: true |
| `should accept the password when it mixes letters, digits and symbols` | 英字・数字・記号が混ざったパスワード | valid: true |
| `should accept the password when it has only digits` | 数字のみ (16 文字) | valid: true |
| `should accept the password when it has only letters` | 英字のみ (16 文字) | valid: true |
| `should accept the password when it has only special characters` | 記号のみ (16 文字) | valid: true |

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
| `should reject the username when it is empty` | 空のユーザー名 | エラー: "Username cannot be empty!" |
| `should reject the username when it has only spaces` | スペースのみのユーザー名 | エラー: "Username cannot be empty!" |
| `should reject the username when it has only tabs` | タブのみのユーザー名 | エラー: "Username cannot be empty!" |
| `should reject the username when it has only mixed whitespace` | スペース・タブ・改行が混ざった空白のみのユーザー名 | エラー: "Username cannot be empty!" |
| `should reject the username when it is null` | null のユーザー名 | エラー: "Username cannot be empty!" |
| `should reject the username when it is undefined` | undefined のユーザー名 | エラー: "Username cannot be empty!" |
| `should accept the username when it has a single character` | 1 文字のユーザー名 | valid: true |
| `should accept the username when it has several characters` | 複数文字のユーザー名 | valid: true |
| `should accept the username when it has digits` | 数字を含むユーザー名 | valid: true |
| `should accept the username when it has underscores and hyphens` | アンダースコアとハイフンを含むユーザー名 | valid: true |
| `should accept the username when it is in email format` | メールアドレス形式のユーザー名 | valid: true |
| `should accept the username when it has leading spaces (trimmed)` | 先頭にスペースがあるユーザー名 (trim される) | valid: true |
| `should accept the username when it has trailing spaces (trimmed)` | 末尾にスペースがあるユーザー名 (trim される) | valid: true |

**testCombinedValidation (7件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should reject when both username and password are empty` | ユーザー名・パスワードとも空 | エラー: "Username cannot be empty!" |
| `should report the username error first when both the username and the password are invalid` | ユーザー名が空でパスワードも短い (ユーザー名のエラーが先) | エラー: "Username cannot be empty!" |
| `should report the empty-password error when the password is empty` | ユーザー名は有効でパスワードが空 (長さより空のエラーが先) | エラー: "Password cannot be empty!" |
| `should report the length error first when the password is short and does not match` | パスワードが短く確認とも一致しない (一致より長さのエラーが先) | エラー: "Password must be at least 16 characters long!" |
| `should accept the user addition when every field is valid` | すべての項目が有効 | valid: true |
| `should accept the user addition when the username is in email format` | メールアドレス形式のユーザー名 | valid: true |
| `should accept the user addition when the password mixes letters, digits and symbols` | 英字・数字・記号が混ざったパスワード | valid: true |

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
| `should allow a username change when no password is entered` | パスワード未入力でユーザー名だけ変更 | valid: true |
| `should reject the username when it is empty and no password is entered` | パスワード未入力で、ユーザー名が空 | エラー: "Username cannot be empty!" |
| `should reject the username when it has only whitespace` | 空白のみのユーザー名 | エラー: "Username cannot be empty!" |
| `should allow the username when it has special characters` | 記号を含むユーザー名 | valid: true |
| `should allow the username when it has unicode characters` | Unicode 文字のユーザー名 | valid: true |
| `should allow the username when it is very long (128 characters)` | 128 文字のユーザー名 | valid: true |

**testPasswordOnlyEdit (8件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should allow a password change when the username is unchanged` | ユーザー名は変えずにパスワードを変更 | valid: true |
| `should reject the password when it is shorter than 16 characters` | 16 文字未満のパスワード | エラー: "Password must be at least 16 characters long!" |
| `should reject the password when the confirmation does not match` | 確認が一致しない | エラー: "Passwords do not match!" |
| `should reject the password when the confirmation is empty` | 確認が空 | エラー: "Passwords do not match!" |
| `should reject the password when it is empty but the confirmation is filled` | パスワードが空で確認だけ入力 | エラー: "Password cannot be empty!" |
| `should allow the password when it has spaces` | スペースを含むパスワード | valid: true |
| `should allow the password when it has special characters` | 記号を含むパスワード | valid: true |
| `should allow the password when it has unicode characters` | Unicode 文字を含むパスワード | valid: true |

**testCombinedEdit (4件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should allow the change when both the username and the password change` | ユーザー名とパスワードを両方変更 | valid: true |
| `should reject the username when it is empty even though the password is valid` | パスワードは有効だがユーザー名が空 | エラー: "Username cannot be empty!" |
| `should reject the change when the username is valid but the password is short` | ユーザー名は有効だがパスワードが短い | エラー: "Password must be at least 16 characters long!" |
| `should reject the change when the username is valid but the passwords do not match` | ユーザー名は有効だがパスワードが一致しない | エラー: "Passwords do not match!" |

**testEditModeVsAddMode (5件)**

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should allow an empty password when in edit mode` | 編集モードで空パスワード (変更なし) | valid: true |
| `should reject an empty password when in add mode` | 追加モードで空パスワード | エラー: "Password cannot be empty!" |
| `should reject a short password when one is entered in edit mode` | 編集モードで短いパスワードを入力 | エラー: "Password must be at least 16 characters long!" |
| `should accept a valid password when in add mode` | 追加モードで有効なパスワード | valid: true |
| `should require the confirmation when a password is entered in edit mode` | 編集モードでパスワードを入力し確認が空 | エラー: "Passwords do not match!" |

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
| `should accept the password when it has leading and trailing spaces and both entries match` | 前後にスペース付き、両者一致・16文字以上 | valid: true |
| `should accept the password when it is very long (1000 characters)` | 1000 文字パスワード | valid: true |
| `should accept the password when it has emojis` | 絵文字を含むパスワード (2 バイト以上文字含む) | valid: true |
| `should reject the password when only one entry has newlines (not trimmed)` | 改行を含み、確認側で改行が落ちる | valid: false, "Passwords do not match!" |
| `should reject the password when it is a single zero-width space` | ゼロ幅スペース 1 文字 (可視 0 だが `.length` は 1) | valid: false, "at least 16 characters" |
| `should reject the password when it has only 6 digits` | 6 桁の数字のみ | valid: false, "at least 16 characters" |

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

**テスト数**: 62件（パスワード26件 + ユーザー名13件 + ユーザー編集23件）

| テストカテゴリ | 説明 | テスト数 | 実装方法 |
|--------------|------|---------|---------|
| パスワードバリデーション | 共通パスワードテストスイート | 26件 | `runAllPasswordTests()` |
| ユーザー名バリデーション | 共通ユーザー名テストスイート | 13件 | `testUsernameValidation()` |
| ユーザー編集バリデーション | 共通ユーザー編集テストスイート | 23件 | `runAllUserEditTests()` |

**ファイル**: res/tests/admin-edit.test.js

---

### general-user-edit.test.js

一般ユーザー編集画面のテスト。

**テスト数**: 62件（パスワード26件 + ユーザー名13件 + ユーザー編集23件）

| テストカテゴリ | 説明 | テスト数 | 実装方法 |
|--------------|------|---------|---------|
| パスワードバリデーション | 共通パスワードテストスイート | 26件 | `runAllPasswordTests()` |
| ユーザー名バリデーション | 共通ユーザー名テストスイート | 13件 | `testUsernameValidation()` |
| ユーザー編集バリデーション | 共通ユーザー編集テストスイート | 23件 | `runAllUserEditTests()` |

**ファイル**: res/tests/general-user-edit.test.js

---

## 機能別テスト

### transaction-edit.test.js

取引編集機能のテスト。

**テスト数**: 111件

| テストカテゴリ (describe) | 説明 | テスト数 |
|--------------|------|---------|
| Modal State Management | モーダルの開閉・状態制御 | 5件 |
| Data Loading | 取引データの読み込み | 13件 |
| Date/Time Format Conversion | SQLite ⇔ datetime-local変換 | 15件 |
| Category Change and Account Reset | カテゴリ変更時の口座リセット処理 | 8件 |
| Memo Handling | メモの正規化・表示処理 | 14件 |
| Form Validation | 入力値の確認 | 10件 |
| Amount Formatting | 金額の表示形式と読み取り | 17件 |
| Error Handling | エラー時の処理 | 10件 |
| Integration Scenarios | 一連の操作の組み合わせ | 3件 |
| Shop Selection | 店舗の選択 | 11件 |
| Shop Selection Integration | 店舗選択の組み合わせ | 5件 |

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

### pages/transaction-detail-draft-storage.test.js

明細画面の下書き (明細 → 商品マスタ往復の間 sessionStorage に置く入力内容) のテスト。本物の `transaction-detail-management.js` を読み込み、公開されている `persistDraft` / `consumeDraft` / `clearDraft` を呼ぶ (画面の起動はしない)。

**テスト数**: 6件

| テスト | 説明 |
|--------|------|
| `should return the same payload when a draft is persisted and then consumed` | 保存した下書きがそのまま読み出せる |
| `should return null from consume when nothing is stored` | 何も無ければ null |
| `should return null and clear storage when the stored draft is malformed JSON` | 壊れた JSON は null を返し、保存内容を消す |
| `should remove the persisted entry when clearDraft is called` | `clearDraft` で保存内容が消える |
| `should overwrite the earlier draft when persist is called again` | 2 回目の保存が前の下書きを上書きする |
| `should keep detail_id and selected_product_id when an edit-mode draft makes a round trip` | 編集中の下書きは `detail_id` と `selected_product_id` を保つ |

**ファイル**: res/tests/pages/transaction-detail-draft-storage.test.js

---

### pages/transaction-detail-product-link.test.js

本物の明細画面を起動し、品名の商品候補と商品の紐付け (隠し欄 `#product-id`) を試すテスト。

**テスト数**: 9件

| テスト | 説明 |
|--------|------|
| `should set the product id when a suggestion is picked` | 候補を選ぶと品名と商品 ID が入る |
| `should replace the product id when a different suggestion is picked` | 別の候補を選び直すと商品 ID が置き換わる |
| `should save without a product id when the user types after picking a suggestion` | 候補を選んだ後に文字を打つと紐付けが外れ、`productId` null で保存される |
| `should restore the product id when a product-linked detail is opened for editing` | 商品に紐付いた明細を編集で開くと商品 ID が戻る |
| `should have no product id when a free-text detail is opened for editing` | 自由入力の明細 (product_id null) を開くと商品 ID は空 |
| `should save a null product id when a detail without a product_id field is opened and saved` | product_id 項目の無い明細を開いて保存すると `productId` は null |
| `should clear the product id and the list when the detail window is opened again` | ウィンドウを開き直すと商品 ID と候補リストが消える |
| `should show the newest answer with no item active when the search answers` | 候補リストは最新の答えを表示し、選択中の行は無い (Enter で何も選ばれない) |
| `should not show a pending answer when the detail window is opened again before it arrives` | 検索中にウィンドウを開き直すと、後から届いた答えは表示しない |

**ファイル**: res/tests/pages/transaction-detail-product-link.test.js

---

### pages/product-management-product-draft.test.js

本物の商品マスタ画面を起動し、商品 → メーカーマスタの寄り道で使う商品の下書きを試すテスト。テストごとに URL を変えて画面を起動し直す。

**テスト数**: 7件

| テスト | 説明 |
|--------|------|
| `should save the window inputs and the transaction to return to when the user came from a detail` | 「メーカーマスタを開く」でウィンドウの入力と戻り先の入出金 (`?return_to=`) を下書きに保存する |
| `should save a null transaction to return to when the user came from the menu` | メニューから来た場合、戻り先は null |
| `should overwrite the earlier draft when the user leaves for the manufacturer master again` | もう一度寄り道すると下書きを上書きする |
| `should restore the window inputs and remove the draft when a draft is stored` | `?restore_product=1` で戻ると入力を復元して下書きを消し、「明細入力に戻る」を表示する |
| `should not offer "Back to detail entry" when the restored draft has no transaction to return to` | 戻り先の無い下書きでは「明細入力に戻る」を出さない |
| `should discard the draft and open no window when the stored draft is malformed JSON` | 壊れた下書きは捨て、ウィンドウを開かない |
| `should open no window when no draft is stored` | 下書きが無ければウィンドウを開かない |

**ファイル**: res/tests/pages/product-management-product-draft.test.js

---

### pages/manufacturer-management-product-draft.test.js

本物のメーカーマスタ画面を `?return_to_product=1` で起動し、追加したメーカーを商品の下書きに紐付ける処理を試すテスト。

**テスト数**: 6件

| テスト | 説明 |
|--------|------|
| `should not create a product draft when no draft is stored` | 下書きが無ければ作らない |
| `should leave the draft alone when the new manufacturer is not in the reloaded list` | 再読み込みした一覧に名前が無ければ下書きを変えない |
| `should write the new manufacturer id as a string when the manufacturer is in the reloaded list` | 一覧にあればメーカー ID を文字列 (`<select>` の値) で書き込む |
| `should keep all non-manufacturer fields when the manufacturer is linked` | メーカー以外の項目はそのまま |
| `should replace the manufacturer id when one was already selected` | 選択済みのメーカー ID を新しいメーカーで置き換える |
| `should leave the draft alone when the manufacturer master was opened from the menu` | メニューから開いた場合は下書きに触れない |

**ファイル**: res/tests/pages/manufacturer-management-product-draft.test.js

---

### modal-double-submit.test.js

共有 `Modal` クラス (`res/js/modal.js`) の `_handleSave` 再入ガードに対する回帰テスト（Fable-5 レビュー #D2 修正）。Save ボタン連打・Enter 連打で `onSave` が並行発火し、Rust マスタ CRUD 側の SELECT-then-INSERT 重複チェック（TOCTOU）を両方通過して 2 発目が生の `UNIQUE constraint failed` で失敗する経路を防ぐ。

**テスト数**: 6件

| テスト名 | 説明 | 期待結果 |
|---------|------|---------|
| `should call onSave only once when the form is submitted rapidly` | フォーム submit を短時間に3回発火 | onSave 呼び出しは1回のみ |
| `should disable the save button when onSave is still pending` | 保存中の Save ボタン状態 | disabled=true → 完了後 false |
| `should call onSave again when the modal is opened and submitted after a successful save` | 保存成功後に再オープン | 2回目の submit で onSave が発火 |
| `should reset the guard and call onSave again when a retry follows a failed save` | 保存失敗後のリトライ | ガードが解除されリトライ成功 |
| `should catch the onSave rejection at the listener when the form is submitted` | onSave が reject する状態でフォーム submit | rejection はリスナー内で捕捉され、未処理の rejection にならない。モーダルは開いたまま |
| `should call onSave only once when the save button is clicked rapidly` (saveButtonId パス) | Save ボタン ID 経由での連打 | onSave 呼び出しは1回のみ |

**ファイル**: res/tests/modal-double-submit.test.js

---

### modal-stale-save-close.test.js

共有 `Modal` クラス (`res/js/modal.js`) の保存セッションに対する回帰テスト (潜在監査 L22)。保存中にモーダルを閉じて開き直した後、先の保存が完了しても、開き直したモーダルを閉じたり保存ガードを解除したりしない。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should not close or reset a re-opened modal when an earlier save finishes (L22)` | 先の保存が完了しても、開き直したモーダルは開いたまま・ローディング表示も維持 |
| `should let a re-opened modal save when an earlier save is still pending (L22)` | 先の保存が未完了でも、開き直したモーダルから保存でき、完了時に閉じる |

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
| `should treat only the most recent request as latest when several were started (scan2-A4)` | 最後に始めた要求だけが「最後」と判定され、次の要求が始まると前の要求は「最後」でなくなる |
| `should count only its own requests when there are several guards (scan2-A4)` | 部品ごとに独立して数える (別画面の要求に影響されない) |

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

`res/js/format-local-date.js` の `formatLocalDate` タイムゾーン安全な `YYYY-MM-DD` フォーマッタのテスト (Fable-5 レビュー #13)。旧実装は `new Date().toISOString().slice(0, 10)` (UTC 変換) で、JST ユーザーが 09:00 JST 前に繰り返しルールモーダルを開くと start-date / end-date / anchor-date が全て前日になっていた。ローカル getter (getFullYear / getMonth / getDate) で組み立てる新実装は、実行タイムゾーンに依存せず常に「ローカル壁時計の日付」を返すので、テストはローカル `Date` コンストラクタ経由で書いてある。あわせて、UTC で保存された日時を表示用にローカル時刻へ直す `formatUtcAsLocalDateTime` もテストする。

**テスト数**: 22件

テストファイルの先頭で `process.env.TZ = 'Asia/Tokyo'` を pin — CodeRabbit on #134 指摘、UTC 実行では local getter と `.toISOString()` の結果が一致するため UTC 回帰が検出できない問題を解消。

| テストブロック | 説明 | テスト数 |
|--------------|------|---------|
| normal cases | 通常日付 / 月ゼロ埋め / 日ゼロ埋め / 両方ゼロ埋め / 12月 / 深夜 0 時 / 23:59:59 | 7件 |
| Fable-5 #13 pin (does not drift to UTC) | UTC 21:30 → JST 翌日 06:30 / UTC 15:30 → JST 翌日 00:30 (UTC/local 発散を確実に検出) + ローカル 06:30 / 23:30 の壁時計固定 | 4件 |
| boundary years | 1900 / 2100 / 閏年 2月29日 / 年 1 (4桁ゼロ埋め) / 年 999 (4桁ゼロ埋め) | 5件 |
| formatUtcAsLocalDateTime — stored UTC shown in local time | UTC で保存された `YYYY-MM-DD HH:MM:SS` を JST で表示 (00:00 → 09:00、日付をまたぐ場合、`T` 区切り、日時の後ろに文字が続く値と日時でない値はそのまま、空は空文字) | 6件 |

**ファイル**: res/tests/format-local-date.test.js

---

### period-end-date.test.js

`res/js/period.js` の `fetchMonthlyPeriodEndDate` のテスト (潜在監査 L14)。ダッシュボードの口座残高がカレンダーの月末基準で、起算日のカスタマイズ (例: 25 日始まり) とずれていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should return the last day of the user's monthly period when the period is requested (L14)` | `get_monthly_period_bounds` の期間最終日を返す (起算日・休日シフト適用済み) |
| `should fall back to the calendar month end when the backend fails (L14)` | バックエンドが失敗したらカレンダーの月末 (うるう年対応) を返す |

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
| `should place the date in the right yearly period when the year starts on %i/%i (date %p -> year %i) (scan2-A3)` (10 ケース) | 年度の開始が 1/1・4/1・12/31・2/31 (月末に寄せる) のそれぞれで、開始日の前後の日付がどの年度に入るか |

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
| `should keep the productId when a product-linked detail is saved without changes (H3)` | 商品に紐付いた明細を開いて無変更で保存しても `update_transaction_detail` に元の `productId` が送られる |
| `should call add_transaction_detail once when the add-detail form is submitted twice (M19)` | 保存中の二重送信で `add_transaction_detail` が 1 回しか呼ばれない (潜在監査 M19) |
| `should show the row total instead of ¥0 when a legacy row has amount_including_tax = 0 (L7)` | 税込額が 0 の古い明細は ¥0 ではなく AMOUNT + TAX_AMOUNT を表示 (潜在監査 L7) |

**ファイル**: res/tests/pages/transaction-detail-page.test.js

### pages/transaction-detail-included-typing.test.js

実際の明細画面で税込額を 1 文字ずつ入力する回帰テスト (潜在スキャン scan2-T1)。以前は入力のたびに税込欄が書き換えられ、途中の「10」が 9 に変わって保存額がずれていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should keep 100 and save 100 / 91 / 9 when "100" is typed tax-included at 10 % (floor) (T1)` | 10 %・切り捨てで「100」を 1 文字ずつ入力しても 100 のまま残り、`add_transaction_detail` に 100 / 91 / 9 が送られる |

**ファイル**: res/tests/pages/transaction-detail-included-typing.test.js

### pages/transaction-detail-unreachable-included-price.test.js

税抜の式で表せない税込額を実際の明細画面で入力する回帰テスト (潜在スキャン scan2-T2)。以前は 1000 円が 999 円に書き換えられていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should keep 1000 with excluded 909 and tax 91 when 1000 is typed tax-included at 10 % (floor) (T2)` | 10 %・切り捨てで 1000 を入力すると 1000 のまま残り、`add_transaction_detail` に 1000 / 909 / 91 が送られる |

**ファイル**: res/tests/pages/transaction-detail-unreachable-included-price.test.js

### pages/transaction-detail-hidden-category.test.js

中分類・小分類を非表示にした後で、その分類の明細を編集する回帰テスト (潜在スキャン scan2-T3)。以前は選択肢に非表示の分類がなく、メモだけ直して保存すると分類が黙って消えていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should keep a hidden category2 (and its category3) when only the memo is edited (T3)` | 非表示の中分類 (と配下の小分類) の明細をメモだけ変えて保存しても、分類がそのまま送られる |
| `should keep a hidden category3 under an enabled category2 when only the memo is edited (T3)` | 表示中の中分類の下で非表示にした小分類も同様に残る |

**ファイル**: res/tests/pages/transaction-detail-hidden-category.test.js

---

### pages/transaction-detail-product-suggest.test.js

明細の品名欄の商品候補のテスト。品名欄には商品の補完機能があるが、文字を打つまで候補が出ないため、登録済みの商品を選べることに気づけなかった。

**テスト数**: 6件

| テスト | 説明 |
|--------|------|
| `should put the medium and minor categories before the item name when the detail window is shown` | 明細のウィンドウで中分類・小分類が品名より前にある (分類を先に選ぶ) |
| `should have a defined placeholder and tooltip hint when the item name field is shown` | 品名欄に短いプレースホルダーと、全文のツールチップ (バルーンヘルプ) があり、どちらも i18n に定義されている |
| `should list suggestions with the chosen categories when the empty item name field gets focus` | 空の品名欄にカーソルが入った時点で候補を出し、検索に明細の分類 (`category1Code` / `category2Code` / `category3Code`) を渡す |
| `should not show the list when focus has left the field before the answer` | 候補の応答が届く前に欄を離れていたら、一覧を出さない |
| `should not show the stale answer of the focus search when the user types while it is pending` | カーソルが入ったときの検索の応答が届く前に文字を打ったら、その古い応答 (空欄の候補) は出さない |
| `should close the list and keep it closed when a suggestion is picked` | 候補を選ぶと一覧が閉じ、そのあと勝手に開き直さない |

**ファイル**: res/tests/pages/transaction-detail-product-suggest.test.js

---

### pages/transaction-management-page.test.js

実際の入出金画面モジュールを `transaction-management.html` に対して起動する回帰テスト (潜在監査 H4)。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should not prompt to overwrite the total with ¥0 when a header without details is saved (H4)` | 明細なしヘッダーの保存で ¥0 上書き確認が出ず、`update_transaction_header_total` も送られず、保存フローが一覧再読込まで完了する |
| `should reject the save without calling update_transaction_header when the transaction date is blank (L8)` | 日時が空欄なら `validation.required` を表示して送信せず、モーダルを開いたままにする (潜在監査 L8) |
| `should move back to the last page when its only row is deleted (L5)` | 最終ページの唯一の行を削除すると最後に存在するページへ戻る (潜在監査 L5) |
| `should keep the newer page when an older page response resolves late (L5)` | 古いページ要求の応答が遅れて届いても新しいページの表示を上書きしない (潜在監査 L5) |

**ファイル**: res/tests/pages/transaction-management-page.test.js

---

### pages/user-management-page.test.js

実際のユーザー管理画面モジュールを `user-management.html` に対して管理者セッションで起動する回帰テスト (潜在監査 M13、L30、作成日時・更新日時のローカル時刻表示)。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should reject the username before create_general_user when it is only whitespace (M13)` | 空白のみのユーザー名は `validation.required` をユーザー名欄に表示し、`create_general_user` を送らない |
| `should call create_general_user when the username is normal (M13)` | 通常のユーザー名は `create_general_user` に送られる (比較用) |
| `should show the Add User button and its footer when the user is an admin (L30)` | 管理者には「ユーザー追加」ボタンとその下部の区切り線 (フッター) が表示される (比較用) |
| `should show the created and updated times in local time when they are stored in UTC` | 作成日時・更新日時を、保存された UTC のままではなくローカル時刻 (JST) で表示する。更新日時が空なら「-」 |

**ファイル**: res/tests/pages/user-management-page.test.js

---

### pages/user-management-delete-last-user.test.js

実際のユーザー管理画面 (管理者セッション) で、最後の一般ユーザーの削除を断られたときの表示を確かめる (潜在スキャン scan2-C5)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should show the dedicated message when the backend refuses to delete the last general user (scan2-C5)` | バックエンドが `last_general_user` で削除を断ると、英語のエラー文付きの汎用メッセージではなく `user_mgmt.last_general_user` を表示 |

**ファイル**: res/tests/pages/user-management-delete-last-user.test.js

---

### pages/user-management-delete-page.test.js

実際のユーザー管理画面 (管理者セッション) で、ユーザーの削除の流れを確かめる。旧 `user-deletion.test.js` (テスト内に写したロジックを試していた 45 件) の置き換え。

**テスト数**: 9件

| テスト | 説明 |
|--------|------|
| `should show a delete button only on general user rows when the admin views the list` | 削除ボタンは一般ユーザーの行だけにあり、管理者の行にはない |
| `should open the delete window with the user name as it is when a delete button is clicked` | 削除ボタンで削除ウィンドウが開き、ユーザー名がそのまま (引用符などを付けずに) 表示される。この時点では削除しない |
| `should close the delete window without deleting when #cancel-delete is clicked` | キャンセルでウィンドウが閉じ、削除しない |
| `should close the delete window without deleting when #close-delete-modal is clicked` | × でウィンドウが閉じ、削除しない |
| `should delete the user picked last when the window was cancelled and opened for another user` | 一度キャンセルして別のユーザーで開き直すと、あとで選んだユーザーが削除される |
| `should show the success message, reload the list and close the window when the delete succeeds` | 削除に成功すると `user_mgmt.user_deleted` を表示し、一覧を読み直し (削除したユーザーが消える)、ウィンドウを閉じる |
| `should show the generic failure with the backend message and keep the window open when the user is not found` | `not_found` では `error.delete_user_failed: <バックエンドの文>` を表示し、ウィンドウは開いたまま |
| `should show the admin-protected message when the backend refuses to delete an admin` | `admin_protected` では `user_mgmt.admin_protected` を表示 |
| `should show a name with HTML markup as plain text when the name contains markup` | HTML を含む名前は、一覧でも削除ウィンドウでも文字のまま表示される |

**ファイル**: res/tests/pages/user-management-delete-page.test.js

---

### pages/index-logout-hides-user-setup.test.js

実際のログイン画面 (index.html + menu.js) で、ログアウト後にログインフォームだけが残ることを確かめる (潜在スキャン scan2-C6)。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `should hide the user-setup form and show only the login form when the user logs out (scan2-C6)` | ユーザー登録フォームが出ている状態でログアウトすると、登録フォーム (`#user-setup`、`#admin-setup`) とメイン画面が隠れ、ログインフォームだけになる |
| `should keep only the login form when the user logs out before the login timer runs (scan2-C6)` | ログイン後 1 秒の画面切り替えより前にログアウトしても、切り替えは取り消され、ログインフォームだけが残る |
| `should not switch screens when a setup check answers after the logout (scan2-C6)` | ログイン後の「ユーザー登録が必要か」の確認がログアウト後に返ってきても、画面を切り替えない |

**ファイル**: res/tests/pages/index-logout-hides-user-setup.test.js

---

### pages/index-login-page.test.js

実際のログイン画面 (index.html + menu.js) で、ログインとログアウトの流れを確かめる。旧 `login.test.js` (テスト内で書いた値を確かめていた 58 件) の置き換え。

**テスト数**: 12件

| テスト | 説明 |
|--------|------|
| `should show the login form and focus the user name when no session is active at start` | セッションがない状態で開くと、ログインフォームを表示してユーザー名欄にフォーカスする (メイン画面と管理者登録は隠れる) |
| `should mask the password when the login form is shown` | パスワード欄は `type="password"` で伏せ字になる |
| `should cancel the browser form submission when the login form is submitted` | 送信時にブラウザ標準のフォーム送信を止める (`preventDefault`) |
| `should send the user name and password as typed when they have surrounding spaces` | 前後に空白があっても、ユーザー名とパスワードを入力どおり (trim せず) `login_user` に送る |
| `should show the success and welcome messages when the login succeeds` | ログインに成功すると `login.success` と `login.welcome` (名前入り) を成功の形で表示 |
| `should show the user name as text when the name in the welcome message contains HTML` | 歓迎メッセージの名前に HTML が含まれていても、文字のまま表示する |
| `should show the invalid-credentials message when the user name or password is wrong` | `auth_invalid_credentials` では `error.invalid_credentials` を表示し、ログインフォームのまま |
| `should show the login failure with the backend message when the error is an ApiError from the database` | データベースのエラーでは `error.login_failed: <バックエンドの文>` を表示 |
| `should show the login failure with the backend message when the error is a plain string` | 文字列のエラーでも `error.login_failed: <その文字列>` を表示 |
| `should show the app and hide the login form one second after login when no user setup is needed` | ユーザー登録が不要なら、ログインの 1 秒後にログインフォームを隠してメイン画面を表示する |
| `should clear the session, the user name, the password and the message when the user logs out` | ログアウトでセッションを消し、ユーザー名・パスワード・メッセージを空にしてログインフォームを表示する |
| `should keep the app on screen and show the failure when the session cannot be cleared at logout` | セッションを消せなかったときは、メイン画面のまま `error.logout_failed` を表示する |

**ファイル**: res/tests/pages/index-login-page.test.js

---

### pages/recurring-rule-page.test.js

実際の繰り返しルール画面モジュールを `recurring-rule.html` に対して起動する回帰テスト (潜在監査 M16)。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should reject the rule before create_recurring_rule when a TRANSFER goes from an account to itself (M16)` | 出金元と入金先が同じ振替テンプレートは `transaction_mgmt.transfer_same_account` を表示し、`create_recurring_rule` を送らない |
| `should call create_recurring_rule when a TRANSFER is between two different accounts (M16)` | 異なる口座間の振替は `create_recurring_rule` に送られる (比較用) |
| `should show the dedicated message when the backend rejects with transfer_same_account (M16)` | バックエンドが `transfer_same_account` で拒否した場合も同じ専用メッセージを表示し、汎用の作成失敗メッセージを出さない |
| `should show the localized message when the backend rejects with recurring_holiday_shift_too_long` | 休日シフトが 14 日を超えるためバックエンドが拒否したとき、`recurring_rule.holiday_shift_too_long` を表示 (#171 の CodeRabbit 指摘) |

**ファイル**: res/tests/pages/recurring-rule-page.test.js

---

### pages/recurring-rule-double-submit.test.js

実際の繰り返しルール画面での二重送信の回帰テスト (潜在監査 M19)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should call create_recurring_rule only once when the form is submitted twice (M19)` | `create_recurring_rule` の実行中に送信を重ねても 1 回しか呼ばれない |

**ファイル**: res/tests/pages/recurring-rule-double-submit.test.js

---

### pages/recurring-rule-period-range.test.js

繰り返し予定の期間の回帰テスト (潜在監査 M15 / M18)。祝日データのある年の範囲外では休日シフトが効かず、年の打ち間違い (9999 年など) で大量の予定が生成されていた。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `should bound the date pickers to the seeded holiday years when the page loads (M15/M18)` | 開始日・終了日の入力欄に (今年 − 5) 年 1/1 〜 (今年 + 10) 年 12/31 の min / max を設定 |
| `should stop the rule before create_recurring_rule when the end date is past the limit (M15/M18)` | 終了日が上限を超えたら送信せず `recurring_rule.period_out_of_range` を表示 |
| `should stop the rule before create_recurring_rule when the start date is before the limit (M15/M18)` | 開始日が下限より前でも同様 |
| `should move the date pickers to the new bounds when the bounds changed since the page loaded (M15/M18)` | 送信時に取得した範囲が画面表示時と変わっていたら、日付入力欄の上限・下限も更新する |
| `should show the same message when the backend rejects with recurring_period_out_of_range (M15/M18)` | バックエンドの `recurring_period_out_of_range` も同じメッセージで表示 |

**ファイル**: res/tests/pages/recurring-rule-period-range.test.js

### pages/recurring-rule-anchor-follows-start.test.js

毎日の予定の起点日の回帰テスト (潜在スキャン scan2-R2)。以前は起点日の初期値が今日で開始日に追従せず、開始日を前に動かすと今日より前の日が黙って抜けていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should start as the start date and follow it when the anchor has not been edited` | 起点日の初期値は開始日。起点日を手で変えるまでは開始日に追従する |
| `should follow the start date again when the form has been reset` | リセット後は再び開始日に追従する |

**ファイル**: res/tests/pages/recurring-rule-anchor-follows-start.test.js

### pages/product-management-edit-manufacturer-roundtrip.test.js

商品の編集中にメーカーマスタへ移動して戻る往復の回帰テスト (潜在スキャン scan2-M2)。以前は戻ると「追加」画面になり、保存で同じ商品を追加しようとしていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should come back in edit mode for the same product when returning from the manufacturer master (or not offer the jump in edit mode) (scan2-M2)` | 戻ると同じ商品の「編集」画面で開き、保存で `update_product` を呼ぶ (`add_product` は呼ばない) |

**ファイル**: res/tests/pages/product-management-edit-manufacturer-roundtrip.test.js

---

### pages/recurring-rule-derived-total.test.js

繰り返し予定の合計金額の回帰テスト (潜在監査 M17)。合計は手入力で初期値 0、明細との整合チェックがなく、0 円の予定が生成されていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should show a read-only total that follows the detail and tax settings when they change (M17)` | 合計欄は読み取り専用で、明細とヘッダーの丸め・内税/外税設定から自動計算される |
| `should not send a typed total to create_recurring_rule when the rule is saved (M17)` | `create_recurring_rule` に合計を送らない (バックエンドが明細から計算) |

**ファイル**: res/tests/pages/recurring-rule-derived-total.test.js

---

### pages/recurring-rule-cycle-options.test.js

繰り返し予定の周期オプションの回帰テスト (潜在監査 M14 / L13)。29〜31 日指定は該当日のない月を黙って飛ばし、毎日 + 祝日シフトは同日重複や終了日後の予定を生んでいた。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `should send DAY_OR_END when a day of the month is chosen (M14)` | 日付指定は `DAY_OR_END` で送る (該当日のない月は月末) |
| `should send END when the end-of-month mode is chosen (M14)` | 「月末」モードを選ぶと `END` を送り、日付欄は隠れる |
| `should reset and disable the holiday shift when the rule is daily (L13)` | 「毎日」を選ぶと祝日シフトを「なし」に戻して無効化し、他の周期では再び選べる |

**ファイル**: res/tests/pages/recurring-rule-cycle-options.test.js

---

### pages/recurring-rule-date-order.test.js

繰り返し予定の日付チェックの回帰テスト (潜在スキャン scan2-R8)。以前は開始日 > 終了日などを画面でチェックせず、バックエンドの英語メッセージ (`start_date must be on or before end_date` など) がそのまま表示されていた。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `should show a localized message, not the backend English text, when the start date is after the end date` | 開始日が終了日より後なら i18n メッセージを出し、`create_recurring_rule` を呼ばない |
| `should show a localized message when a daily anchor is after the end date` | 毎日の起点日が終了日より後なら i18n メッセージを出す |
| `should show a localized message when the end date is empty` | 終了日が空なら i18n メッセージを出す |
| `should show the same message when the start date is empty` | 開始日が空でも同じメッセージを出す (期間範囲外のメッセージにしない) |
| `should still create a rule when the dates are in order` | 日付の順序が正しければ従来どおり作成する |

**ファイル**: res/tests/pages/recurring-rule-date-order.test.js

---

### pages/recurring-rule-reset.test.js

繰り返し予定のリセットボタンの回帰テスト (潜在スキャン scan2-R4)。以前は「毎月」を選んでからリセットすると、ラジオボタンは「毎日」に戻るのに毎月用の欄が表示されたまま、起点日の欄は隠れたまま、休日シフトも選べるままで、開始日・終了日・起点日は空になっていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should bring the cycle UI and the default dates back in line when Reset is pressed` | リセット後は「毎日」の表示 (起点日あり・毎月用の欄なし・休日シフトは「なし」で無効) に戻り、既定の日付 (今日 / 1 年後 / 起点日 = 開始日) が入り直す |

**ファイル**: res/tests/pages/recurring-rule-reset.test.js

---

### pages/recurring-rule-rounding-recalc.test.js

実際の繰り返しルール画面で、金額を入れた後に端数処理を変えると明細の税額が計算し直されることを確かめる (潜在スキャン scan2-R5)。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should recompute the detail tax fields when the rounding changes (tax excluded) (scan2-R5)` | 外税・税率 10%・105 円で、切り捨てから切り上げに変えると税額・税込額が 11 / 116 になり、登録の要求にもその値が載る |
| `should keep the typed tax-included price and recompute the rest when the rounding changes (tax included) (scan2-R5)` | 内税で税込 116 円と入力した後に切り上げへ変えると、税込額はそのままで税抜額・税額が 105 / 11 になり、登録の要求にもその値が載る |

**ファイル**: res/tests/pages/recurring-rule-rounding-recalc.test.js

---

### pages/menu-i18n-seed.test.js

メニューバーの翻訳登録の回帰テスト (潜在スキャン scan2-C2)。以前は `menu.back_to_transactions` が旧スクリプト `sql/add_detail_mgmt_i18n.sql` にしかなく、`res/sql/dbaccess.sql` から作った DB では明細画面の「ファイル」メニューにキー文字列がそのまま表示されていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should seed menu.back_to_transactions for ja and en when the i18n SQL is read` | `dbaccess.sql` に `menu.back_to_transactions` の ja / en 両方の行がある |
| `should seed every data-i18n key in the menu bar for ja and en when the i18n SQL is read` | 明細画面のメニューバーが描画する `data-i18n` キーがすべて ja / en 両方とも `dbaccess.sql` に登録されている |

**ファイル**: res/tests/pages/menu-i18n-seed.test.js

---

### pages/i18n-literal-user-text.test.js

ユーザーが入力した文字が、メッセージにそのまま差し込まれることを確かめる (潜在スキャン scan2-C4)。

**テスト数**: 7件

| テスト | 説明 |
|--------|------|
| `should keep the user name %s literally when i18n.t() fills it in (scan2-C4)` (4 件) | ユーザー名の `$&` `$'` `` $` `` `$$` が置き換えの記号として解釈されず、そのまま出る |
| `should not substitute again when a value contains another placeholder (scan2-C4)` | 差し込んだ値に `{b}` が含まれていても、もう一度置き換えない (1 回でまとめて置き換える) |
| `should leave the placeholder as it is when it has no param (scan2-C4)` | 値を渡していない `{b}` はそのまま残る |
| `should keep the rule name literally when the recurring-rule delete confirmation is shown (scan2-C4)` | 繰り返しルールの削除確認で、`$'` や `{1}` を含むルール名がそのまま出て、件数も正しい位置に入る |

**ファイル**: res/tests/pages/i18n-literal-user-text.test.js

---

### pages/dashboard-balance-header.test.js

ダッシュボードの口座別残高の列見出しの回帰テスト (潜在スキャン scan2-C3)。以前は `dashboard.balance` が「収支」(グラフの凡例) と「残高」(列見出し) の 2 回登録されていて、後の行が INSERT OR IGNORE で捨てられ、列見出しが「収支」になっていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should show 残高 (ja) / Balance (en) when the balance column header is rendered` | `dbaccess.sql` を上から適用した結果で、列見出しのキーが ja「残高」/ en「Balance」になる |

**ファイル**: res/tests/pages/dashboard-balance-header.test.js

---

### pages/transaction-list-none-account-label.test.js

入出金一覧の「指定なし」口座の表示の回帰テスト (潜在スキャン scan2-M8)。以前は DB に保存された口座名「指定なし」をそのまま出していたので、英語表示でも「Main Bank → 指定なし」となっていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should render common.unspecified, not the stored name, when one side is NONE` | 口座コードが NONE の側は `common.unspecified` で表示し、保存された「指定なし」は出さない |

**ファイル**: res/tests/pages/transaction-list-none-account-label.test.js

---

### pages/transaction-detail-none-account-label.test.js

明細画面の上部 (取引情報) の「指定なし」口座の表示の回帰テスト (潜在スキャン scan2-M8)。入出金一覧と同じく、保存された口座名「指定なし」がそのまま出ていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should render common.unspecified, not the stored name, when the account is NONE` | 口座コードが NONE の口座は `common.unspecified` で表示し、保存された「指定なし」は出さない |

**ファイル**: res/tests/pages/transaction-detail-none-account-label.test.js

---

### pages/dashboard-balance-sign.test.js

ダッシュボードの金額表示の回帰テスト (潜在スキャン scan2-A2)。以前は推移グラフの収支の吹き出しが絶対値で、赤字 3 万円が「¥30,000」と黒字に見えていた。縦軸の目盛りはマイナスが「¥-30,000」、プラスが「¥30K」「¥1.5M」の略記、口座別残高は「¥-1,234」だった。集計画面と同じ「-¥30,000」の形にそろえ、略記はやめて金額をそのまま出す (読み上げや、K・M に慣れていない人への配慮。2026-10-07 ボノさん判断)。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `should show the minus sign when the balance is a -30,000 deficit (scan2-A2)` | 収支 -30,000 の吹き出しが「-¥30,000」になる |
| `should show the full signed amount, without K / M, when the axis ticks are drawn (scan2-A2)` | 推移グラフと棒グラフの目盛りが「-¥30,000」「¥1,500,000」「¥0」のように略さず符号付きで出る |
| `should put the minus sign before ¥ when an account balance is negative (scan2-A2)` | 口座別残高が「¥1,500,000」「-¥1,234」になる |

**ファイル**: res/tests/pages/dashboard-balance-sign.test.js

---

### pages/dashboard-default-period.test.js

ダッシュボードを開いたときの対象月の回帰テスト (潜在スキャン scan2-A3)。以前は暦の月で決めていたので、起算日 25 日・今日 9 月 10 日だと、まるごと未来の「9 月」(9/25〜10/24) が開いていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should default to the August period that contains today when the start day is 25 and today is 2026-09-10 (scan2-A3)` | 今日を含む「8 月」の期間で開き、その月のデータを読み込む |

**ファイル**: res/tests/pages/dashboard-default-period.test.js

---

### pages/dashboard-stale-reload.test.js

ダッシュボードの再読み込みの回帰テスト (潜在スキャン scan2-A4)。以前は 9 月から 3 月へ素早く切り替えると、遅れて届いた 9 月の結果でグラフと見出しが 9 月に戻っていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should not overwrite the newer March charts when a slower, older September load finishes later (scan2-A4)` | 古い 9 月の結果は捨てられ、グラフと見出しは 3 月のまま |

**ファイル**: res/tests/pages/dashboard-stale-reload.test.js

---

### single-flight.test.js

送信ハンドラの二重実行防止 `singleFlight` (`res/js/single-flight.js`) のテスト (潜在監査 M19)。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| should ignore a second call when the first is still in flight | 実行中の再送信を無視する |
| should call preventDefault on every submit when some submits are ignored | 無視した送信でも `preventDefault` を呼ぶ |
| should accept a new call when the previous one has resolved | 完了後は次の送信を受け付ける |
| should release the guard when the handler throws | ハンドラが例外を投げてもガードを解除する |

**ファイル**: res/tests/single-flight.test.js

---

### pages/product-management-page.test.js

実際の商品マスタ画面の回帰テスト (潜在監査 M5)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should keep manufacturer_id on save when the product's manufacturer is disabled (M5)` | 無効化されたメーカーに紐付く商品を無変更で保存しても `manufacturer_id` が保たれる (無効メーカーを「（非表示）」付きで選択肢に追加) |

**ファイル**: res/tests/pages/product-management-page.test.js

---

### pages/product-management-link-draft.test.js

明細 → 商品マスタへのジャンプ (`?return_to=`) で商品を追加したときの回帰テスト (潜在監査 L17)。明細の下書きには名前が完全一致した商品だけを紐付け、検索の別候補を紐付けない。あわせて、下書きが無いとき・検索結果が 0 件のとき・紐付けで他の項目が保たれることも試す。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `should leave the detail draft alone when no product name matches exactly (L17)` | 完全一致が無ければ、下書きの商品紐付けと品名を変えない |
| `should link the detail draft to the product when its name matches exactly (L17)` | 部分一致の候補が先に並んでも、完全一致の商品を紐付ける |
| `should not create a detail draft when the product is added without one` | 下書きが無ければ作らない |
| `should leave the detail draft alone when the search returns no candidates` | 検索結果が 0 件なら下書きを変えない |
| `should keep all non-product fields when the detail draft is linked to the product` | 紐付けても商品以外の項目はそのまま |

**ファイル**: res/tests/pages/product-management-link-draft.test.js

---

### pages/shop-management-disabled.test.js

店舗マスタ画面の無効化の回帰テスト (潜在監査 M7)。使用中の店舗は削除できず「代わりに無効化してください」と案内していたが、無効化する手段が無かった。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should list disabled shops, marked, when "show disabled" is on (M7)` | 「非表示も表示」で無効な店舗を非表示ラベル付きで一覧に出す |
| `should not let a late "show disabled" response overwrite a newer list when the toggle changes again (M7)` | 切替の連打で遅れて届いた古い応答が新しい一覧を上書きしない |
| `should send the disabled checkbox when adding a shop (M7)` | 追加時に「非表示」チェックを `isDisabled` として送る |
| `should show and send the disabled state when editing a shop (M7)` | 編集時にチェック状態を表示し、変更を送る (再有効化) |

**ファイル**: res/tests/pages/shop-management-disabled.test.js

---

### pages/transaction-management-disabled-shop.test.js

入出金画面で無効な店舗を使った取引の回帰テスト (潜在監査 M7)。選択肢に無効な店舗が無く「未指定」に落ち、保存で店舗が消えていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should keep a disabled shop selected when editing a transaction that names it (M7)` | 編集時は無効な店舗を非表示ラベル付きで選択したまま保存する |
| `should not offer a disabled shop when the transaction is new (M7)` | 新規取引では無効な店舗を選択肢に出さない |

**ファイル**: res/tests/pages/transaction-management-disabled-shop.test.js

---

### pages/account-management-disabled.test.js

口座マスタ画面の無効化の回帰テスト (潜在監査 M7)。使用中の口座は削除できず「代わりに無効化してください」と案内していたが、無効化する手段が無かった。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should list disabled accounts, marked, when "show disabled" is on (M7)` | 「非表示も表示」で無効な口座を非表示ラベル付きで一覧に出す (NONE は出さない) |
| `should not let a late "show disabled" response overwrite a newer list when the toggle changes again (M7)` | 切替の連打で遅れて届いた古い応答が新しい一覧を上書きしない |
| `should send the disabled checkbox when adding an account (M7)` | 追加時に「非表示」チェックを `isDisabled` として送る |
| `should show and send the disabled state when editing an account (M7)` | 編集時にチェック状態を表示し、変更を送る (再有効化) |

**ファイル**: res/tests/pages/account-management-disabled.test.js

### pages/account-management-save-error-keeps-form.test.js

口座マスタの保存失敗時の回帰テスト (潜在スキャン scan2-M4)。以前は保存に失敗しても入力画面が閉じ、入力内容が消えていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should keep the window open and the typed input when add_account fails with duplicate_code` | 口座コードの重複でバックエンドが拒否しても、画面が開いたまま入力が残る |
| `should keep the window open when a whitespace-only name is stopped before add_account` | 空白だけの名前を入力チェックで止めたときも、画面が開いたまま残る |

**ファイル**: res/tests/pages/account-management-save-error-keeps-form.test.js

---

### pages/account-management-validation-i18n.test.js

口座マスタの入力チェックと一覧読み込みエラーの i18n 回帰テスト (潜在スキャン scan2-R8 の続き)。以前は `Account name is required` などの英語が直書きで、口座コード・テンプレート・初期残高のメッセージは存在しない要素を指していたため表示もされなかった。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `should show the i18n required message when the account code is empty` | 口座コードが空なら入力欄の下に `validation.required` を出す |
| `should show the i18n required message when the account name is empty` | 口座名が空白だけなら `validation.required` を出す |
| `should show the i18n required message when no template is selected` | テンプレート未選択なら `validation.required` を出す |
| `should show the i18n amount message when the initial balance is empty` | 初期残高が空なら `common.error_amount_not_integer` を出す |
| `should show only the localized message, not the backend detail, when the list fails to load` | 一覧の読み込み失敗時は `account_mgmt.failed_to_load` だけを出し、バックエンドの英語の詳細は出さない |

**ファイル**: res/tests/pages/account-management-validation-i18n.test.js

---

### pages/account-management-code-max-length.test.js

口座コードの文字数上限の回帰テスト。以前は口座コードに上限がなく、256 文字を超えるコードも保存できていた (DB の `VARCHAR(50)` は SQLite では強制されない)。上限は新規登録時だけで、登録後はコードを変更できないため、既存の長いコードはそのまま編集できる。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should show a 50-character counter and cut typed input at 50 when an account is added` | 新規登録では口座コード欄に「n / 50」の文字数カウンタが出て、50 文字を超えた入力は切り詰められる |
| `should stop a 51-character code before add_account with the max-length message when an account is added` | 51 文字のコードは `add_account` を呼ぶ前に止まり、`validation.max_length` のメッセージが出る |
| `should send a 50-character code to add_account when an account is added` | 50 文字のコードはそのまま `add_account` に送られる |
| `should keep an existing code longer than 50 characters and send it to update_account when an account is edited` | 編集では口座コード欄が読み取り専用でカウンタもなく、50 文字を超える既存のコードも切り詰めずに `update_account` に送られる |

**ファイル**: res/tests/pages/account-management-code-max-length.test.js

---

### pages/transaction-management-disabled-account.test.js

入出金画面で無効な口座を使った取引の回帰テスト (潜在監査 M7)。選択肢に無効な口座が無く、保存で口座が失われていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should keep a disabled account selected when editing a transaction that names it (M7)` | 編集時は無効な口座を非表示ラベル付きで選択したまま保存する |
| `should not offer a disabled account when the transaction is new (M7)` | 新規取引では無効な口座を選択肢に出さない |

**ファイル**: res/tests/pages/transaction-management-disabled-account.test.js

---

### pages/transaction-management-category1-has-details.test.js

明細がある取引の大分類変更の回帰テスト (潜在監査 M2)。ヘッダーの大分類だけが変わり、収入の明細が支出の円グラフに混ざっていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should explain why the category cannot change and keep the window open when the transaction has details (M2)` | バックエンドの `category1_has_details` を受けて `transaction_mgmt.category1_has_details` を表示し、モーダルを開いたままにする |

**ファイル**: res/tests/pages/transaction-management-category1-has-details.test.js

---

### modal-open-awaits-onopen.test.js

共有 `Modal` クラスの `open()` の回帰テスト (潜在監査 L6)。`onOpen` の完了を待たずに戻っていたため、入出金画面の下書き復元が後から走る初期化で上書きされていた。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should settle only after onOpen has finished when onOpen is async (L6)` | 非同期の `onOpen` が終わるまで `open()` の Promise は完了しない (モーダルはすぐ表示) |
| `should settle at once when onOpen is synchronous (L6)` | 同期の `onOpen` ならすぐ完了する |

**ファイル**: res/tests/modal-open-awaits-onopen.test.js

---

### pages/transaction-management-restore-draft.test.js

入出金画面の新規取引の下書き復元の回帰テスト (潜在監査 L6)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should keep the restored draft instead of the window's late defaults when the defaults arrive after the restore (L6)` | 復元した日付・店舗・メモが、モーダル自身の初期化 (フォームのリセット・現在日時) で上書きされない |

**ファイル**: res/tests/pages/transaction-management-restore-draft.test.js

### pages/transaction-management-shop-roundtrip-draft.test.js

入出金の入力中に店舗管理へ移動して戻る往復の回帰テスト (潜在スキャン scan2-T4)。以前は予定フラグが保存されず、編集中の大分類・口座・税設定も DB の値に戻っていた。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `should keep a new transaction scheduled when it makes the shop round trip (T4a)` | 新規で「予定」にチェックして往復しても、チェックが残り予定として保存される |
| `should keep edit-mode changes to rounding, account and memo when the edit makes the shop round trip (T4b)` | 編集中に変えた丸め・口座と、空にしたメモが往復後も残る |
| `should keep category1 cleared when it was cleared in edit mode before the shop round trip (T4c)` | 編集中に空にした大分類が、往復後も DB の値に戻らず空のまま残る (#170 の CodeRabbit 指摘) |

**ファイル**: res/tests/pages/transaction-management-shop-roundtrip-draft.test.js

### pages/transaction-management-rejected-save-keeps-form.test.js

入出金の保存が止められた・失敗したときの回帰テスト (潜在スキャン scan2-T5)。以前は入力画面が閉じ、入力内容が消えていた。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `should keep the form open when a TRANSFER has both accounts "Unspecified" (T5)` | 出金元・入金先とも「指定なし」の振替を止めても、画面が開いたまま入力が残る |
| `should keep the form open when parseAmountStrict rejects the total ("1e3") (T5)` | 金額 `1e3` を入力チェックで止めても、画面が開いたまま残る |
| `should keep the form open when the backend returns a generic error (T5)` | バックエンドの一般的なエラーでも、画面が開いたまま残る |

**ファイル**: res/tests/pages/transaction-management-rejected-save-keeps-form.test.js

---

### pages/transaction-management-restore-disabled-shop.test.js

下書き保存後に無効化された店舗の復元の回帰テスト (潜在監査 M7、L6 修正で到達可能になった)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should not give a restored new transaction a shop when the shop was disabled after the draft was saved (M7)` | 新規取引の復元では無効化された店舗を選ばず「未指定」にする |

**ファイル**: res/tests/pages/transaction-management-restore-disabled-shop.test.js

---

### pages/transaction-management-restore-reopened.test.js

下書き復元の途中でモーダルを閉じて開き直した場合の回帰テスト (潜在監査 L6)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should not write the draft into a window when it was reopened while the restore was waiting (L6)` | 復元が待っている間に閉じて開き直したモーダルには、古い下書きを書き込まない |

**ファイル**: res/tests/pages/transaction-management-restore-reopened.test.js

---

### pages/aggregation-monthly-page.test.js

実際の月次集計画面の回帰テスト (潜在監査 M11 / M12)。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `should render common.unspecified when group_name is empty (M12)` | 空の `group_name` を `common.unspecified` で表示 |
| `should not count one transfer (FROM row + TO row) twice in the total row when the axis is account (M11)` | 口座軸の合計行は件数・平均を「—」で表示 (振替の二重計上を避ける) |
| `should not count one transaction spanning two groups twice in the total row when the axis is category2 (M11)` | 費目2軸の合計行も件数・平均を「—」で表示 |
| `should still sum the count into the total row when the axis is category1 (M11)` | 費目1軸では従来どおり件数を合計 (比較用) |
| `should put the minus sign before the yen symbol when the amount is negative (L12)` | 負の金額を「-¥1,234」と表示 (潜在監査 L12) |

**ファイル**: res/tests/pages/aggregation-monthly-page.test.js

---

### pages/aggregation-yearly-total-count.test.js

実際の年次集計画面 (共通レンダラ `renderResults`) の回帰テスト (潜在監査 M11)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should not count one transfer twice in the shared total row when the axis is account (M11)` | 口座軸の合計行は件数・平均を「—」で表示 |

**ファイル**: res/tests/pages/aggregation-yearly-total-count.test.js

### pages/aggregation-default-period-monthly.test.js

月次集計を開いたときの対象月の回帰テスト (潜在スキャン scan2-A3)。以前は暦の月で決めていたので、起算日 25 日・今日 9 月 10 日だと、まるごと未来の「9 月」が開いていた (ダッシュボードと同じ問題、#178 で修正済み)。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should open on the August period that contains today when the start day is 25 and today is 2026-09-10 (scan2-A3)` | 今日を含む「8 月」の期間で開く |

**ファイル**: res/tests/pages/aggregation-default-period-monthly.test.js

### pages/aggregation-default-period-yearly.test.js

年次集計を開いたときの対象年の回帰テスト (潜在スキャン scan2-A3)。以前は暦の年で決めていたので、年度の開始が 4/1・今日 2026 年 2 月 10 日だと、まるごと未来の「2026 年度」(2026/4/1〜2027/3/31) が開いていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should open on the 2025 period that contains today when the year starts on 04-01 and today is 2026-02-10 (scan2-A3)` | 今日を含む「2025 年度」で開く |

**ファイル**: res/tests/pages/aggregation-default-period-yearly.test.js

### pages/aggregation-monthly-stale.test.js

月次集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should drop the older result when a slower, older request finishes later (scan2-A4)` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `should neither show the error nor clear the table when an older request fails later (scan2-A4)` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `should keep the loading state until the latest request finishes when an older request finishes first (scan2-A4)` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `should not strand the running request when an Execute is stopped by the input checks (scan2-A4)` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-monthly-stale.test.js

### pages/aggregation-daily-stale.test.js

日次集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should drop the older result when a slower, older request finishes later (scan2-A4)` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `should neither show the error nor clear the table when an older request fails later (scan2-A4)` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `should keep the loading state until the latest request finishes when an older request finishes first (scan2-A4)` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `should not strand the running request when an Execute is stopped by the input checks (scan2-A4)` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-daily-stale.test.js

### pages/aggregation-weekly-stale.test.js

週次集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should drop the older result when a slower, older request finishes later (scan2-A4)` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `should neither show the error nor clear the table when an older request fails later (scan2-A4)` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `should keep the loading state until the latest request finishes when an older request finishes first (scan2-A4)` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `should not strand the running request when an Execute is stopped by the input checks (scan2-A4)` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-weekly-stale.test.js

### pages/aggregation-period-stale.test.js

期間指定集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should drop the older result when a slower, older request finishes later (scan2-A4)` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `should neither show the error nor clear the table when an older request fails later (scan2-A4)` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `should keep the loading state until the latest request finishes when an older request finishes first (scan2-A4)` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `should not strand the running request when an Execute is stopped by the input checks (scan2-A4)` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-period-stale.test.js

### pages/aggregation-yearly-stale.test.js

年次集計の再実行の回帰テスト (潜在スキャン scan2-A4)。以前は古い要求の結果を捨てる仕組みが無く、遅れて届いた古い結果が新しい表を上書きしたり、古い要求のエラーが新しい表を消したりしていた。シナリオは 5 画面共通で `pages/_aggregation-stale.js` にある。

**テスト数**: 4件

| テスト | 説明 |
|--------|------|
| `should drop the older result when a slower, older request finishes later (scan2-A4)` | 遅れて届いた古い結果は捨てられ、新しい表のまま |
| `should neither show the error nor clear the table when an older request fails later (scan2-A4)` | 遅れて失敗した古い要求のエラーは出ず、新しい表も消えない |
| `should keep the loading state until the latest request finishes when an older request finishes first (scan2-A4)` | 古い要求が先に終わっても読み込み中の表示は消えず、最後の要求が終わったときに消える |
| `should not strand the running request when an Execute is stopped by the input checks (scan2-A4)` | 入力チェックで止まった実行は要求を始めないので、実行中の要求は「最後」のままで、終わると読み込み中の表示が消える (CodeRabbit on #179) |

**ファイル**: res/tests/pages/aggregation-yearly-stale.test.js

### pages/dashboard-bar-top10.test.js

実際のダッシュボードを起動する回帰テスト (潜在スキャン scan2-A1)。支出の合計は負の値なのに符号付きで降順に並べていたため、棒グラフには小さい支出から並び、上位 10 件から最大の支出 (家賃など) が落ちていた。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should draw the largest expense category first and keep it in the top 10 when there are more than 10 categories (scan2-A1)` | 支出を金額の大きさ順に並べ、最大の支出が先頭に来て上位 10 件に残る |

**ファイル**: res/tests/pages/dashboard-bar-top10.test.js

---

### pages/index-setup-page.test.js

初回セットアップ画面 (`index.html` 上の menu.js) の回帰テスト (潜在監査 L25)。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `should reject the admin setup without calling register_admin when the username is blank (L25)` | 空白のみのユーザー名は `register_admin` を送らず `error.username_required` を表示 |
| `should report the username, not the password, when the backend rejects a blank username (L25)` | バックエンドの「Username cannot be empty」をパスワードではなくユーザー名のエラーとして表示 |
| `should show the duplicate-username message when the backend reports duplicate_name (L25)` | `duplicate_name` で `error.username_duplicate` を表示 |

**ファイル**: res/tests/pages/index-setup-page.test.js

---

### pages/category-management-page.test.js

実際の費目管理画面の回帰テスト (潜在監査 L19)。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should show the not-found message and reload the tree when moving a vanished category (L19)` | 存在しなくなった費目の移動で `category_mgmt.not_found` を表示しツリーを再読込 |
| `should show the not-found message and reload the tree when showing a vanished category (L19)` | 存在しなくなった費目の再表示でも同様 |

**ファイル**: res/tests/pages/category-management-page.test.js

---

### pages/category-management-move-buttons.test.js

実際の費目管理画面で、↑/↓ ボタンが非表示の費目を数えないことを確かめる (潜在スキャン scan2-M5)。非表示の費目は表示中の費目の後ろに並ぶ。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should not let the last visible CATEGORY2 move down when only hidden ones follow (scan2-M5)` | 表示中の最初の中分類は「↑」、最後の中分類は「↓」が押せない (後ろに非表示があっても)。非表示の行には ↑/↓ が無い |
| `should not let the last visible CATEGORY3 move down when only hidden ones follow (scan2-M5)` | 小分類でも同じ |

**ファイル**: res/tests/pages/category-management-move-buttons.test.js

---

### pages/transaction-management-filter-hidden-category.test.js

実際の入出金一覧で、費目フィルタに非表示の費目も出ることを確かめる (潜在スキャン scan2-M7)。入力用の選択肢からは外したまま。

**テスト数**: 1件

| テスト | 説明 |
|--------|------|
| `should offer a hidden CATEGORY2 and its CATEGORY3, labelled as hidden, when the list filter is shown (scan2-M7)` | 非表示の中分類「外食」とその小分類が `common.disabled_label` 付きで選べる。表示中の「食費」はラベル無しのまま |

**ファイル**: res/tests/pages/transaction-management-filter-hidden-category.test.js

---

### pages/transaction-management-save-before-details.test.js

実際の入出金一覧の編集ウィンドウで、「明細管理」を押したときに保存していないヘッダーの変更が消えないことを確かめる (潜在スキャン scan2-T6)。確認はアプリ内のダイアログ (`#save-before-details-modal`) で出し、ブラウザ標準の `confirm()` は使わない (全テストで呼ばれないことを確認)。

**テスト数**: 5件

| テスト | 説明 |
|--------|------|
| `should save the edited header values when leaving for the details (T6)` | 日付・合計・メモ・予定フラグを変えて「明細管理」を押すと確認ダイアログ (`transaction_mgmt.save_before_details_confirm`) が出て、「保存」で通常の保存によりヘッダーを保存してから移動する |
| `should not save twice when the button is clicked again while saving (T6)` | 保存中にダイアログの「保存」をもう一度押しても、保存は 1 回だけで、保存が終わるまで移動しない |
| `should stay in the edit window without saving when the dialog is cancelled (T6)` | ダイアログで「キャンセル」を押すと、保存も移動もせず、編集ウィンドウが入力した値のまま残る |
| `should close only the dialog, not the edit window behind it, when Esc is pressed (T6)` | Esc キーはダイアログだけを閉じ、後ろの編集ウィンドウは入力した値のまま残る |
| `should move on at once, without asking or saving, when nothing was changed (T6)` | 変更がなければ確認も保存もせずにすぐ移動する |

**ファイル**: res/tests/pages/transaction-management-save-before-details.test.js

---

### pages/user-management-password-page.test.js

ユーザー管理画面 (管理者セッション) のパスワード検証の回帰テスト (潜在監査 L24 / L31)。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should report the error on the password field when the password is 16 spaces (L24)` | 空白16文字のパスワードは、未定義キー `user_mgmt.empty_name` をユーザー名欄に出さず、パスワードのエラーとして表示 |
| `should reject the password in the frontend when it is 8 emoji (16 UTF-16 units) (L31)` | 文字数を UTF-16 単位ではなく文字 (コードポイント) で数え、絵文字8文字を拒否 |

**ファイル**: res/tests/pages/user-management-password-page.test.js

---

### pages/user-management-nonadmin-page.test.js

ユーザー管理画面 (一般ユーザーセッション) の回帰テスト (潜在監査 L30)。

**テスト数**: 3件

| テスト | 説明 |
|--------|------|
| `should not offer the Add User button when the user is not an admin (L30)` | 一般ユーザーには「ユーザー追加」ボタンを出さない |
| `should hide the empty footer line under the list when the user is not an admin (L30)` | ボタンを隠したあとに区切り線 (フッター) だけが残らないよう、フッターごと隠す |
| `should show no delete button on the own row when the user is not an admin (L30)` | 一般ユーザーの自分の行に削除ボタンを出さない |

**ファイル**: res/tests/pages/user-management-nonadmin-page.test.js

---

### pages/index-setup-password-length.test.js

初回セットアップ画面のパスワード文字数の回帰テスト (潜在監査 L31)。

**テスト数**: 2件

| テスト | 説明 |
|--------|------|
| `should reject the password on the frontend when admin setup gets 8 emoji (8 chars) (L31)` | 管理者セットアップで絵文字8文字を拒否 |
| `should reject the password on the frontend when user setup gets 8 emoji (8 chars) (L31)` | 一般ユーザーセットアップでも同様 |

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
| **画面別テスト** | **202件** |
| admin-setup.test.js | 32 |
| user-addition.test.js | 46 |
| admin-edit.test.js | 62 |
| general-user-edit.test.js | 62 |
| **機能別テスト** | **559件** |
| transaction-edit.test.js | 111 |
| transaction-detail-management.test.js | 51 |
| transaction-detail-tax-calculation.test.js | 30 |
| toast.test.js | 14 |
| tax-calc.test.js | 12 |
| pages/transaction-detail-draft-storage.test.js | 6 |
| pages/transaction-detail-product-link.test.js | 9 |
| pages/product-management-product-draft.test.js | 7 |
| pages/manufacturer-management-product-draft.test.js | 6 |
| modal-double-submit.test.js | 6 |
| modal-stale-save-close.test.js | 2 |
| master-crud.test.js | 30 |
| attach-char-counter-ime.test.js | 8 |
| aggregation-error-translate.test.js | 13 |
| aggregation-latest-request.test.js | 2 |
| parse-amount-strict.test.js | 24 |
| format-local-date.test.js | 22 |
| period-end-date.test.js | 2 |
| period-containing.test.js | 16 |
| aggregation-render-unspecified.test.js | 5 |
| pages/transaction-detail-page.test.js | 3 |
| pages/transaction-detail-included-typing.test.js | 1 |
| pages/transaction-detail-unreachable-included-price.test.js | 1 |
| pages/transaction-detail-hidden-category.test.js | 2 |
| pages/transaction-detail-product-suggest.test.js | 6 |
| pages/transaction-management-page.test.js | 4 |
| pages/user-management-page.test.js | 4 |
| pages/user-management-delete-last-user.test.js | 1 |
| pages/user-management-delete-page.test.js | 9 |
| pages/index-logout-hides-user-setup.test.js | 3 |
| pages/index-login-page.test.js | 12 |
| pages/recurring-rule-page.test.js | 4 |
| pages/recurring-rule-double-submit.test.js | 1 |
| pages/recurring-rule-period-range.test.js | 5 |
| pages/recurring-rule-anchor-follows-start.test.js | 2 |
| pages/product-management-edit-manufacturer-roundtrip.test.js | 1 |
| pages/recurring-rule-derived-total.test.js | 2 |
| pages/recurring-rule-cycle-options.test.js | 3 |
| pages/recurring-rule-date-order.test.js | 5 |
| pages/recurring-rule-reset.test.js | 1 |
| pages/recurring-rule-rounding-recalc.test.js | 2 |
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
| pages/product-management-link-draft.test.js | 5 |
| pages/shop-management-disabled.test.js | 4 |
| pages/transaction-management-disabled-shop.test.js | 2 |
| pages/account-management-disabled.test.js | 4 |
| pages/account-management-save-error-keeps-form.test.js | 2 |
| pages/account-management-validation-i18n.test.js | 5 |
| pages/account-management-code-max-length.test.js | 4 |
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
| pages/transaction-management-save-before-details.test.js | 5 |
| pages/user-management-password-page.test.js | 2 |
| pages/user-management-nonadmin-page.test.js | 3 |
| pages/index-setup-password-length.test.js | 2 |
| **集計機能テスト** | **115件** |
| aggregation-daily.test.js | 16 |
| aggregation-weekly.test.js | 22 |
| aggregation-monthly.test.js | 33 |
| aggregation-yearly.test.js | 21 |
| aggregation-period.test.js | 23 |
| **総計 (jest)** | **876件** |

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
cd res/tests
npm test admin-setup.test.js
npm test pages/index-login-page.test.js
npm test pages/user-management-delete-page.test.js
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
