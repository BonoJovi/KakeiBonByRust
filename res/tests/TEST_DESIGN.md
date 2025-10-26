# テストモジュール設計仕様

## 設計ポリシー

### 1. DRY原則（Don't Repeat Yourself）
- バリデーションロジックは共通モジュールで一元管理
- テストケースも共通テストスイートとして再利用
- 画面ごとのテストは共通モジュールをインポートするだけ

### 2. 保守性の向上
- バリデーションルール変更時の修正箇所を最小限に
- テストケース追加時の作業を最小限に
- コードの可読性を重視

### 3. 一貫性の確保
- すべての画面で同じバリデーションルールを適用
- すべての画面で同じテストケースを実行
- エラーメッセージの統一

## アーキテクチャ

### モジュール構成

```
res/tests/
├── validation-helpers.js             # 共通バリデーション関数
├── password-validation-tests.js      # パスワードテストスイート（26件）
├── username-validation-tests.js      # ユーザ名テストスイート（13件）
├── user-edit-validation-tests.js     # ユーザ編集テストスイート（23件）
├── admin-setup.test.js               # 管理者登録テスト（29件）
├── user-addition.test.js             # ユーザ追加テスト（49件）
├── admin-edit.test.js                # 管理者ユーザ編集テスト（63件）
├── general-user-edit.test.js         # 一般ユーザ編集テスト（63件）
└── login.test.js                     # ログインテスト（58件）
```

**JavaScript総テスト数**: 262件

### レイヤー構造

```
┌─────────────────────────────────────┐
│   画面固有テスト (*.test.js)        │
│  - admin-setup.test.js (29)         │
│  - user-addition.test.js (49)       │
│  - admin-edit.test.js (63)          │
│  - general-user-edit.test.js (63)   │
│  - login.test.js (58)               │
└────────────┬────────────────────────┘
             │ インポート
┌────────────▼────────────────────────┐
│   共通テストスイート                 │
│  - password-validation-tests.js (26)│
│  - username-validation-tests.js (13)│
│  - user-edit-validation-tests.js(23)│
└────────────┬────────────────────────┘
             │ インポート
┌────────────▼────────────────────────┐
│   共通バリデーション関数             │
│  - validation-helpers.js            │
│    * validatePassword()             │
│    * validateUserAddition()         │
│    * validateUserEdit()             │
└─────────────────────────────────────┘
```
│  - validation-helpers.js            │
│  - validatePassword()               │
│  - validateUserAddition()           │
└─────────────────────────────────────┘
```

## 共通モジュール

### validation-helpers.js

バリデーション関数を定義する共通モジュール。

#### validatePassword(password, passwordConfirm)
パスワードとパスワード確認のバリデーション。

**パラメータ:**
- `password`: パスワード文字列
- `passwordConfirm`: パスワード確認文字列

**戻り値:**
```javascript
{
    valid: boolean,      // バリデーション結果
    message: string      // エラーメッセージ（成功時は空文字）
}
```

**バリデーションルール:**
1. パスワードが空でないこと（trim後）
2. パスワードが16文字以上であること
3. パスワードと確認が一致すること

#### validateUserAddition(username, password, passwordConfirm)
ユーザ名とパスワードのバリデーション。

**パラメータ:**
- `username`: ユーザ名文字列
- `password`: パスワード文字列
- `passwordConfirm`: パスワード確認文字列

**戻り値:**
```javascript
{
    valid: boolean,
    message: string
}
```

**バリデーションルール:**
1. ユーザ名が空でないこと（trim後）
2. `validatePassword()`のルールをすべて満たすこと

### password-validation-tests.js

パスワードバリデーションの共通テストスイート。

#### runAllPasswordTests(validationFn, suiteName)
すべてのパスワードテストを実行。

**パラメータ:**
- `validationFn`: テスト対象のバリデーション関数（`(password, passwordConfirm) => result`）
- `suiteName`: テストスイート名（デフォルト: "Password Validation"）

**テストカテゴリ:**
1. 空パスワードのバリデーション（6件）
2. パスワード長のバリデーション（6件）
3. パスワード一致のバリデーション（6件）
4. 有効なパスワードシナリオ（8件）

#### 個別テストスイート関数
- `testEmptyPasswordValidation(validationFn)` - 空パスワードテスト
- `testPasswordLengthValidation(validationFn)` - パスワード長テスト
- `testPasswordMatchValidation(validationFn)` - パスワード一致テスト
- `testValidPasswordScenarios(validationFn)` - 有効パスワードテスト

### username-validation-tests.js

ユーザ名バリデーションの共通テストスイート。

#### testUsernameValidation(validationFn)
ユーザ名バリデーションテストを実行。

**パラメータ:**
- `validationFn`: テスト対象のバリデーション関数（`(username, password, passwordConfirm) => result`）

**テストケース:**
- 空ユーザ名の拒否（6件）
- 有効なユーザ名の受け入れ（5件）
- エッジケース（2件）

#### testCombinedValidation(validationFn)
ユーザ名とパスワードの組み合わせテストを実行。

**テストケース:**
- バリデーション優先順位（4件）
- 有効な組み合わせ（3件）

### user-edit-validation-tests.js

ユーザ編集バリデーションの共通テストスイート。

#### runAllUserEditTests(validationFn, contextName)
すべてのユーザ編集テストを実行。

**パラメータ:**
- `validationFn`: テスト対象のバリデーション関数（`(username, password, passwordConfirm, isEditMode) => result`）
- `contextName`: テストコンテキスト名（デフォルト: "User Edit"）

**テストカテゴリ:**
1. ユーザ名のみ編集（6件）
2. パスワードのみ編集（8件）
3. ユーザ名＋パスワード編集（4件）
4. 編集モード vs 追加モード（5件）

#### 個別テストスイート関数
- `testUsernameOnlyEdit(validationFn)` - ユーザ名のみ編集テスト
- `testPasswordOnlyEdit(validationFn)` - パスワードのみ編集テスト
- `testCombinedEdit(validationFn)` - ユーザ名＋パスワード編集テスト
- `testEditModeVsAddMode(validationFn)` - 編集/追加モード比較テスト

**編集モードの特徴:**
- `isEditMode = true`: パスワード空白時は「変更なし」として扱う
- `isEditMode = false`: パスワード必須（追加モード）
- パスワードを提供する場合は、編集モードでも検証が必要

## 使用方法

### 新しい画面のテストを追加

#### ステップ1: テストファイルを作成

```javascript
// new-screen.test.js
import { validatePassword } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';

// 共通パスワードテストを実行
runAllPasswordTests(validatePassword, 'New Screen Password Validation');
```

#### ステップ2: 画面固有のテストを追加

```javascript
// 画面固有のエッジケースがあれば追加
describe('New Screen Specific Tests', () => {
    test('specific edge case', () => {
        const result = validatePassword('特定のケース', '特定のケース');
        expect(result.valid).toBe(true);
    });
});
```

### ユーザ名とパスワードの両方をテスト

```javascript
// user-form.test.js
import { validateUserAddition } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';
import { testUsernameValidation, testCombinedValidation } from './username-validation-tests.js';

// パスワードテストのラッパー関数
const passwordValidation = (password, passwordConfirm) => {
    return validateUserAddition('validuser', password, passwordConfirm);
};

// すべてのテストを実行
runAllPasswordTests(passwordValidation, 'User Form Password');
testUsernameValidation(validateUserAddition);
testCombinedValidation(validateUserAddition);
```

### ユーザ編集画面のテスト

```javascript
// user-edit.test.js
import { validateUserEdit } from './validation-helpers.js';
import { runAllPasswordTests } from './password-validation-tests.js';
import { testUsernameValidation } from './username-validation-tests.js';
import { runAllUserEditTests } from './user-edit-validation-tests.js';

// パスワードテストのラッパー（編集モード）
const passwordValidation = (password, passwordConfirm) => {
    return validateUserEdit('existinguser', password, passwordConfirm, true);
};

// ユーザ名テストのラッパー（編集モード）
const usernameValidation = (username) => {
    return validateUserEdit(username, '', '', true);
};

// すべてのテストを実行
runAllPasswordTests(passwordValidation, 'User Edit Password');
testUsernameValidation(usernameValidation);
runAllUserEditTests(validateUserEdit, 'User Edit');
```

describe('User Form Validation', () => {
    // ユーザ名テスト
    testUsernameValidation(validateUserAddition);
    
    // パスワードテスト
    runAllPasswordTests(passwordValidation, 'Password Validation');
    
    // 組み合わせテスト
    testCombinedValidation(validateUserAddition);
});
```

## 既存テストの修正

### バリデーションルールの変更

#### ステップ1: 共通バリデーション関数を修正

```javascript
// validation-helpers.js
export function validatePassword(password, passwordConfirm) {
    // 新しいルールを追加
    if (password.length < 20) {  // 16文字 → 20文字に変更
        return {
            valid: false,
            message: 'Password must be at least 20 characters long!'
        };
    }
    // ...
}
```

#### ステップ2: 共通テストスイートを修正

```javascript
// password-validation-tests.js
export function testPasswordLengthValidation(validationFn) {
    describe('Password Length Validation', () => {
        test('should reject password with exactly 19 characters', () => {
            const result = validationFn('1234567890123456789', '1234567890123456789');
            expect(result.valid).toBe(false);
            expect(result.message).toBe('Password must be at least 20 characters long!');
        });
        
        test('should accept password with exactly 20 characters', () => {
            const result = validationFn('12345678901234567890', '12345678901234567890');
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
        // ...
    });
}
```

#### ステップ3: すべてのテストを実行して確認

```bash
npm test
```

すべての画面のテストが自動的に更新されたルールでテストされます。

### 新しいテストケースの追加

共通テストスイートに追加すると、すべての画面で自動的にテストされます。

```javascript
// password-validation-tests.js
export function testValidPasswordScenarios(validationFn) {
    describe('Valid Password Scenarios', () => {
        // 既存のテスト...
        
        // 新しいテストケースを追加
        test('should accept password with mixed case', () => {
            const password = 'MixedCase1234567';
            const result = validationFn(password, password);
            expect(result.valid).toBe(true);
            expect(result.message).toBe('');
        });
    });
}
```

## テスト実行環境

### Jest設定（ES Modules対応）

```json
// package.json
{
  "type": "module",
  "scripts": {
    "test": "node --experimental-vm-modules node_modules/jest/bin/jest.js"
  },
  "jest": {
    "testEnvironment": "jsdom",
    "transform": {}
  }
}
```

### 重要な注意点

1. **import文には拡張子が必要**
   ```javascript
   import { validatePassword } from './validation-helpers.js';  // ✓ 正しい
   import { validatePassword } from './validation-helpers';     // ✗ エラー
   ```

2. **export文はnamed exportを使用**
   ```javascript
   export function validatePassword() { }  // ✓ 推奨
   export default function() { }           // △ 可能だが非推奨
   ```

3. **テスト関数は明示的に実行**
   ```javascript
   // 自動実行される
   runAllPasswordTests(validatePassword, 'Test Suite Name');
   
   // describe内で呼び出す
   describe('My Tests', () => {
       testUsernameValidation(validateUserAddition);
   });
   ```

## ベストプラクティス

### 1. 共通化可能なものは共通化する
- 同じバリデーションルールは共通モジュールに
- 同じテストケースは共通テストスイートに
- 画面固有のロジックのみ画面別ファイルに

### 2. テスト名は明確に
```javascript
// ✓ 良い例
test('should reject password shorter than 16 characters', () => { });

// ✗ 悪い例
test('password test', () => { });
```

### 3. エラーメッセージは統一する
```javascript
// ✓ 良い例 - 共通モジュールで定義
const ERRORS = {
    PASSWORD_TOO_SHORT: 'Password must be at least 16 characters long!',
    PASSWORD_EMPTY: 'Password cannot be empty!'
};

// ✗ 悪い例 - 画面ごとに異なるメッセージ
```

### 4. テストは独立して実行可能に
```javascript
// ✓ 良い例 - テスト間で状態を共有しない
test('test 1', () => {
    const result = validate('input1', 'input1');
    expect(result.valid).toBe(true);
});

test('test 2', () => {
    const result = validate('input2', 'input2');
    expect(result.valid).toBe(true);
});

// ✗ 悪い例 - グローバル変数を使用
let globalResult;
test('test 1', () => {
    globalResult = validate('input1', 'input1');
});
test('test 2', () => {
    expect(globalResult.valid).toBe(true);  // test 1に依存
});
```

## トラブルシューティング

### ES Modulesエラー
```
SyntaxError: Cannot use import statement outside a module
```

**解決方法:**
1. `package.json`に`"type": "module"`を追加
2. import文に`.js`拡張子を含める
3. Jest実行コマンドに`--experimental-vm-modules`を追加

### テストが見つからないエラー
```
describe is not defined
```

**解決方法:**
Jest設定で`testEnvironment: "jsdom"`を指定

### 共通モジュールが読み込めない
```
Cannot find module './validation-helpers.js'
```

**解決方法:**
1. ファイルパスが正しいか確認
2. 拡張子`.js`が含まれているか確認
3. ファイルが存在するか確認

## 今後の拡張

### 新しいバリデーション種類の追加

1. 新しいヘルパー関数を追加
   ```javascript
   // validation-helpers.js
   export function validateEmail(email) { }
   ```

2. 新しいテストスイートを作成
   ```javascript
   // email-validation-tests.js
   export function testEmailValidation(validationFn) { }
   ```

3. 各画面テストでインポート
   ```javascript
   import { testEmailValidation } from './email-validation-tests.js';
   ```

### テストカバレッジの向上

```bash
npm run test:coverage
```

カバレッジレポートを確認し、未テストの部分を追加していきます。

## 参考資料

- [Jest公式ドキュメント](https://jestjs.io/)
- [ES Modules in Jest](https://jestjs.io/docs/ecmascript-modules)
- [TEST_CASES.md](TEST_CASES.md) - 全テストケース一覧
