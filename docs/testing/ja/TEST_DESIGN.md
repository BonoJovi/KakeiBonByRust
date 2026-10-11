# テスト設計

KakeiBon のテストの作り方と、その理由。

## 方針

### 1. 写しではなく本物のコードを試す

テストはアプリ自身のコードを動かす。ルールをテストファイルに写して写しを試すテストは、アプリが変わっても通り続け、写しがアプリから気付かれないままずれていく。KakeiBon には 2026-10 までこの種のテストがあり、アプリに無い動きを書いた写しもあった：

- パスワードの写しは UTF-16 の単位で数えていたため、`password🔐🔐🔐🔐123` を受け付けていた。アプリはバックエンドと同じく文字 (コードポイント) で数えるので、このパスワードは 15 文字で断られる。
- 金額の写しは `1,000` を 1000 と読んでいた。アプリはこの入力を断る。

### 2. 利用者に見えるものと送られるものを確かめる

フロントエンドのテストは、本物のフォームに入力して送信し、次を確かめる：

- 表示されたメッセージ (テスト用の i18n はキーをそのまま返すので、i18n キーで比べる)
- `invoke` で送られた Tauri コマンドとその引数

### 3. 最後の砦はバックエンド

パスワード 16 文字以上のようなルールは、フロントエンド (すぐに知らせるため) とバックエンド (`src/validation.rs`、安全のため) の両方で確かめる。それぞれにテストがある。

## フロントエンド (JavaScript、Jest + jsdom)

### page テスト

ほとんどの画面は page スクリプトで、`res/js/<画面>.js` が `DOMContentLoaded` のリスナーを登録し、`res/<画面>.html` の DOM を操作する。`tests/frontend/pages/_page-harness.js` がこの画面を jsdom で起動する：

1. ページのモジュールを import する**前に** `mockPageModules(jest, { invoke, user })` を呼び、Tauri の `invoke` と、ウィンドウの枠だけを描くモジュール (メニューバー・文字サイズ・言語メニュー・インジケーター・ウィンドウ調整)、およびセッション・トースト・i18n を `jest.unstable_mockModule` で差し替える。
2. `loadPageBody('<画面>.html')` で本物の画面の HTML を jsdom に入れる。
3. 本物のページのモジュールを import して `bootPage()` を呼ぶ。`DOMContentLoaded` を発火させ、画面の読み込みが終わるまで待つ。

```javascript
// tests/frontend/pages/index-setup-validation.test.js (抜粋)
import { jest } from '@jest/globals';
import { mockPageModules, loadPageBody, bootPage, flush, callsOf } from './_page-harness.js';

const { invoke } = mockPageModules(jest, {
    keepMenu: true,
    invoke: (cmd) => (cmd === 'check_needs_setup' ? true : 'ok'),
});

loadPageBody('index.html');
await import('../../../res/js/menu.js');
await bootPage();

test('should refuse with the right message and not register when the password has 15 characters', async () => {
    document.getElementById('admin-username').value = 'alice';
    document.getElementById('admin-password').value = '123456789012345';
    document.getElementById('admin-password-confirm').value = '123456789012345';
    document.getElementById('admin-setup-form').dispatchEvent(
        new Event('submit', { cancelable: true, bubbles: true })
    );
    await flush(10);

    expect(document.getElementById('setup-message').textContent).toBe('error.password_too_short');
    expect(callsOf(invoke, 'register_admin')).toHaveLength(0);
});
```

### フォームの送信のしかた

- `form.dispatchEvent(new Event('submit', ...))` は画面自身のチェックを動かす。アプリのコードを試すときに使う。
- 送信ボタンをクリックすると、ブラウザと同じく入力欄のチェック (`required`・`min`・`step`・`type="number"`) が先に動く。アプリのコードより前にこれらの属性が止める入力を試すときに使う。

### 複数の画面で共通の確認

複数の画面が同じ確認を通る必要があるときは、ケースを使うテストの隣の補助ファイルにまとめ、各テストファイルには違う部分 (呼ばれるはずのコマンドなど) だけを渡す。例: `tests/frontend/pages/_user-self-edit-checks.js` を管理者用と一般ユーザー用のユーザー管理テストが使う。補助ファイルは `.test.js` で終わらないので、Jest はテストとして集めない。

### 値を計算するだけのモジュール

値を計算するだけのモジュール (`res/js/parse-amount-strict.js`、`res/js/tax-calc.js` など) は、画面を起動せずに直接 import して試す。

### テストが落ちうることを確かめる

テストを書いたら、試している対象のアプリのコードをわざと壊し (文字ではなく UTF-16 の単位で数える、など)、テストが落ちることを確かめる。その後 `git checkout` でコードを戻す。

### Jest の設定

`tests/frontend/package.json`：

- ES Modules のために `"type": "module"` と `node --experimental-vm-modules`
- `"testEnvironment": "jsdom"`、`"testMatch": ["**/*.test.js"]`
- `latent-audit/` は通常は除外 (`npm run test:latent` で実行)

import には `.js` 拡張子を付ける。

## バックエンド (Rust)

Rust のテストはコードの隣 (`#[cfg(test)]` モジュール) にあり、次を共有する：

- `src/test_helpers.rs`: データベースの準備 (`setup_test_db`、`create_test_admin` など)
- `src/validation_tests.rs`: 複数のモジュールで使うパスワードのテストスイート

## テスト名

新しいテストは `should <期待する結果> when <条件>` の形にする。`docs/developer/ja/guides/CODING_STANDARDS.md` の「テスト規約」を参照。

## トラブルシューティング

| エラー | 対処 |
|--------|------|
| `SyntaxError: Cannot use import statement outside a module` | `npx jest` ではなく `npm test` で実行する (`--experimental-vm-modules` が付く) |
| モックが効かない | ページのモジュールを import する前に `mockPageModules` を呼ぶ |
| 起動後に画面に何も出ない | `await bootPage()` し、`invoke` を呼ぶ操作の後は `await flush()` する |

## 関連ドキュメント

- [テスト概要](TEST_OVERVIEW.md)
- [フロントエンドテストインデックス](FRONTEND_TEST_INDEX.md)
- [バックエンドテストインデックス](BACKEND_TEST_INDEX.md)
- [Jest: ECMAScript Modules](https://jestjs.io/docs/ecmascript-modules)
