# フロントエンドテストインデックス

このドキュメントは、JavaScriptで実装されたフロントエンドテストの完全なインデックスです。

**最終更新**: 2026-08-21 JST  
**総テスト数**: 685件 (jest suite 21 ファイル、`npm test` 実測)

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
  - [master-crud.test.js](#master-crudtestjs)
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

**テスト数**: 17件 (jest 実測)

| テストカテゴリ | 説明 |
|--------------|------|
| 税抜→税込計算 | 税抜金額から税込金額を計算 |
| 税込→税抜計算 | 税込金額から税抜金額を計算 |
| 丸め誤差検出 | 税計算の丸め誤差検出 |
| エッジケース | 0円、負の値などのエッジケース |
| 複数税率 | 異なる税率での計算 |

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

**テスト数**: 10件

| テストカテゴリ | 説明 |
|--------------|------|
| 税抜→税込 | 8%/10% での加算計算 |
| 税込→税抜 | 8%/10% での逆算 |
| 丸め処理 | floor / round / ceil の 3 モード |
| エッジケース | 0円、端数 |

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

### master-crud.test.js

共有 `res/js/master-crud.js` の `saveMasterEntry` オーケストレーターと `mapMasterErrorCode` 分類器の単体テスト (Fable-5 レビュー #D3/#D4/#23)。Rust 側 `ApiError { code, message, entity? }` を JS 側で `err.code` ベースに分類し、i18n key へマップする契約を検証。

**テスト数**: 21件

| テストブロック | 説明 | テスト数 |
|--------------|------|---------|
| `mapMasterErrorCode` | 各 code (`duplicate_name` / `not_found` / `manufacturer_not_found` / `validation` の 3 サブケース / `database` / legacy string 3 パス) が正しい i18n key と inline/toast ルーティングを返す | 12件 |
| `saveMasterEntry — validation before invoke` | client 側 empty / name-too-long / memo-too-long がバックエンド invoke より前に inline エラーで蹴られる | 3件 |
| `saveMasterEntry — edit target vanished` | editingId で cache に見つからない場合の `onNotFoundBeforeInvoke` 呼び出し + デフォルト toast fallback | 2件 |
| `saveMasterEntry — happy path` | 新規追加 / 更新 それぞれで `invokeAdd`/`invokeUpdate` が正しい引数で呼ばれ `onSuccess` が発火 | 2件 |
| `saveMasterEntry — backend error re-throws and classifies` | `duplicate_name` → inline、`manufacturer_not_found` → toast (product スコープ)、いずれも throw で Modal は開いたまま | 2件 |

**ファイル**: res/tests/master-crud.test.js

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
| **機能別テスト** | **262件** |
| transaction-edit.test.js | 112 |
| transaction-detail-management.test.js | 51 |
| transaction-detail-tax-calculation.test.js | 17 |
| toast.test.js | 14 |
| tax-calc.test.js | 10 |
| product-autocomplete.test.js | 10 |
| product-draft.test.js | 11 |
| product-master-jump-draft.test.js | 11 |
| modal-double-submit.test.js | 5 |
| master-crud.test.js | 21 |
| **集計機能テスト** | **115件** |
| aggregation-daily.test.js | 16 |
| aggregation-weekly.test.js | 22 |
| aggregation-monthly.test.js | 33 |
| aggregation-yearly.test.js | 21 |
| aggregation-period.test.js | 23 |
| **総計 (jest)** | **685件** |

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
