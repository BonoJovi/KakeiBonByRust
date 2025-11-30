# ユーザ管理画面UI実装ドキュメント

## 概要
ユーザ管理画面のフロントエンド実装を行いました。管理者が一般ユーザの追加、編集、削除を行うためのWebインターフェースです。

## 実装日
2025-10-25

## 実装ファイル

### HTML
- `res/user-management.html` - ユーザ管理画面のメインHTML

### CSS
- `res/css/user-management.css` - ユーザ管理画面専用スタイル
- `res/css/indicators.css` - アクセシビリティインジケータ共通スタイル（新規作成）

### JavaScript
- `res/js/user-management.js` - ユーザ管理画面のロジック
- `res/js/indicators.js` - インジケータ共通モジュール（新規作成）
- `res/js/consts.js` - 定数定義モジュール

## 画面構成

### メニューバー
- **Fileメニュー**
  - Back to Main: メイン画面に戻る
  - Logout: ログアウト
  - Quit: アプリケーション終了
  
- **Languageメニュー**
  - 動的に生成される言語選択メニュー
  - 現在選択中の言語にはインジケータ表示

### ユーザ一覧セクション
ユーザ情報をテーブル形式で表示：
- User ID（ユーザID）
- Username（ユーザ名）
- Role（役割）- Admin/User
- Created At（作成日時）
- Updated At（更新日時）
- Actions（操作）- 編集/削除ボタン

**特徴**
- 管理者ユーザは削除ボタンが表示されない（削除不可）
- ロールによって異なる色のバッジ表示
- ユーザがいない場合は「No users found」メッセージ表示

### ユーザ追加/編集モーダル
モーダルダイアログでユーザ情報の入力：

**追加モード**
- Username（ユーザ名）- 必須
- Password（パスワード）- 必須、最低16文字
- Password (Confirm)（パスワード確認）- 必須

**編集モード**
- Username（ユーザ名）- 必須、変更可能
- Password（パスワード）- 任意、変更する場合のみ入力
- Password (Confirm)（パスワード確認）- パスワード変更時のみ必須

**バリデーション**
- パスワードは16文字以上
- パスワードと確認用パスワードの一致チェック
- 入力フィールドにフォーカスインジケータ表示

### ユーザ削除確認モーダル
削除実行前に確認ダイアログを表示：
- 削除対象のユーザ名を大きく強調表示（1.5倍、ダブルクォートで囲む）
- 削除/キャンセルボタン
- 削除結果のメッセージ表示

## アクセシビリティ機能

### フォーカストラップ（2025-10-26実装）

モーダルダイアログ内でキーボードフォーカスを適切に制御：

**機能**
- TABキーで前方にフォーカス移動、SHIFT+TABで後方に移動
- モーダルの最初の要素と最後の要素でループ
- モーダル外にフォーカスが逃げることを防止

**実装の詳細**
- TauriアプリケーションでSHIFT+TABが`"Unidentified"`として報告される問題に対応
- キャプチャフェーズでイベントをインターセプト
- `res/js/modal-utils.js`の`setupFocusTrap()`関数で実装

### フォーカスインジケータ

フォーカスされている要素を視覚的に明示：

**入力フィールド**
- フォーカス時に緑色の大きな●マーク（左側）
- 黒い縁取りで色覚異常のユーザにも配慮
- `.form-group.active`クラスで制御

**ボタン（2025-10-26統一）**
- 非アクティブ時：2pxの黒枠
- フォーカス時：白2px + 黒4pxの二重ボックスシャドウ
- 全ボタンで統一されたデザイン
- 高いコントラスト比でアクセシビリティを確保

**ドロップダウン項目**
- アクティブな項目に○マーク（塗りつぶし）
- 黒い縁取りで視認性向上

### キーボード操作対応
- Tabキーでフォーカス移動
- Enterキーでフォーム送信
- Escキーでモーダルを閉じる
- モーダル内でのフォーカストラップ（実装済み）

## 主要機能

### ユーザ一覧表示 (`loadUsers()`)
```javascript
// バックエンドからユーザ一覧を取得して表示
const users = await invoke('list_users');
```
- i18n対応のメッセージ表示
- ロール別バッジの色分け
- 管理者の削除ボタン非表示処理

### ユーザ追加 (`createUser()`)
```javascript
const userId = await invoke('create_general_user', {
    username: username,
    password: password
});
```
- パスワードバリデーション
- 成功/失敗メッセージ表示
- 完了後に一覧を再読み込み

### ユーザ編集 (`updateUser()`)
```javascript
// 一般ユーザ更新
await invoke('update_general_user_info', updateParams);

// 管理者更新
await invoke('update_admin_user_info', updateParams);
```
- ロールに応じた更新関数の使い分け
- 変更がない項目はnullを渡す
- パスワードは任意（変更時のみ）

### ユーザ削除 (`handleDeleteConfirm()`)
```javascript
await invoke('delete_general_user_info', { userId: userId });
```
- 確認ダイアログ表示
- 削除結果のフィードバック
- 1.5秒後に自動でモーダルを閉じて一覧更新

### 言語切り替え (`handleLanguageChange()`)
```javascript
await i18n.setLanguage(langCode);
await setupLanguageMenu();
await loadUsers();
```
- 言語変更後にメニューと一覧を再構築
- 現在の言語にインジケータ表示

## 多言語対応 (i18n)

### 対応キー
```javascript
// メニュー
menu.file, menu.back_to_main, menu.logout, menu.quit
menu.language

// ユーザ管理
user_mgmt.title, user_mgmt.user_list
user_mgmt.user_id, user_mgmt.username, user_mgmt.role
user_mgmt.created_at, user_mgmt.updated_at, user_mgmt.actions
user_mgmt.add_user, user_mgmt.edit_user, user_mgmt.edit, user_mgmt.delete
user_mgmt.password, user_mgmt.password_confirm
user_mgmt.save, user_mgmt.cancel
user_mgmt.role_admin, user_mgmt.role_user
user_mgmt.no_users
user_mgmt.loading, user_mgmt.creating, user_mgmt.updating, user_mgmt.deleting
user_mgmt.user_created, user_mgmt.user_updated, user_mgmt.user_deleted
user_mgmt.delete_user, user_mgmt.delete_confirmation

// エラー
error.password_mismatch, error.password_too_short
error.load_users_failed, error.save_user_failed, error.delete_user_failed
```

## コードリファクタリング

### indicators.cssの分離
**目的**: メンテナンス性の向上

**変更内容**:
- `menu.css`からインジケータ関連のスタイルを分離
- `res/css/indicators.css`として独立したファイルに
- 各画面で`<link rel="stylesheet" href="css/indicators.css" />`でインクルード

**メリット**:
1. 変更の局所化 - インジケータ修正時は1ファイルのみ
2. 可読性向上 - ファイル名から機能が明確
3. 再利用性 - 新しい画面でも簡単にインクルード可能

### indicators.jsモジュール化
**目的**: コードの重複排除とメンテナンス性向上

**エクスポート関数**:

#### `wrapInputFields()`
```javascript
// .form-group内のinputを.input-wrapperでラップ
// インジケータ表示用のレイアウト構築
```

#### `setupInputIndicators()`
```javascript
// input, textarea, selectにfocus/blurイベントハンドラを設定
// フォーカス時に.form-groupに.activeクラスを追加
```

#### `setupButtonIndicators()`
```javascript
// すべてのボタンに.focus-indicatorクラスを追加
// フォーカス時の下線表示を有効化
```

#### `setupIndicators()`
```javascript
// 上記3つの関数をまとめて実行する便利関数
// 各画面のDOMContentLoadedで呼び出す
```

**使用方法**:
```javascript
import { setupIndicators } from './indicators.js';

document.addEventListener('DOMContentLoaded', async function() {
    // ... 他の初期化処理
    setupIndicators();  // インジケータセットアップ
    // ...
});
```

**適用画面**:
- `res/index.html` (ログイン/セットアップ画面) - `menu.js`で使用
- `res/user-management.html` (ユーザ管理画面) - `user-management.js`で使用

**メリット**:
1. DRY原則の遵守 - 重複コード削除
2. バグ修正の効率化 - 1箇所の修正で全画面に反映
3. 統一されたUX - 全画面で同じ動作
4. テスト容易性 - モジュール単位でテスト可能

## データフロー

### ユーザ追加フロー
1. 「Add User」ボタンクリック → `openAddUserModal()`
2. フォーム入力 → フォーカスインジケータ表示
3. 「Save」ボタンクリック → `handleUserFormSubmit()`
4. バリデーション → パスワード長、一致チェック
5. バックエンド呼び出し → `create_general_user`
6. 成功メッセージ表示 → モーダルを閉じる
7. ユーザ一覧を再読み込み → `loadUsers()`

### ユーザ編集フロー
1. 「Edit」ボタンクリック → `openEditUserModal(user)`
2. 既存データをフォームに設定
3. フォーム編集 → パスワードは任意
4. 「Save」ボタンクリック → `handleUserFormSubmit()`
5. ロール判定 → `update_general_user_info` or `update_admin_user_info`
6. 変更項目のみをパラメータに含める
7. 成功メッセージ表示 → モーダルを閉じる
8. ユーザ一覧を再読み込み

### ユーザ削除フロー
1. 「Delete」ボタンクリック → `openDeleteModal(user)`
2. 確認ダイアログ表示 → ユーザ名を明示
3. 「Delete」ボタンクリック → `handleDeleteConfirm()`
4. バックエンド呼び出し → `delete_general_user_info`
5. 成功メッセージ表示 → 1.5秒待機
6. モーダルを閉じる → ユーザ一覧を再読み込み

## セキュリティ考慮事項

### パスワード処理
- フロントエンドではプレーンテキストで送信
- バックエンドでArgon2ハッシュ化
- 最低16文字の長さを要求

### XSS対策
```javascript
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
```
- ユーザ名表示時にHTMLエスケープ
- テーブル描画時に`escapeHtml()`を使用

### CSRF対策
- Tauriの安全なIPC通信を使用
- Webベースの通常のHTTPリクエストは使用しない

## 今後の改善点

### 実装済み機能（2025-10-26更新）
1. ✅ ユーザ追加機能
2. ✅ ユーザ編集機能
3. ✅ ユーザ削除機能
4. ✅ 管理者編集機能
5. ✅ フォーカストラップ（モーダル内）
6. ✅ 統一されたボタンフォーカススタイル
7. ✅ 削除確認モーダルの改善

### UI/UX改善
- ページネーション（ユーザ数が多い場合）
- ソート機能（列ヘッダクリックでソート）
- 検索/フィルタ機能
- 一括操作機能

### アクセシビリティ
- ARIAラベルの追加
- スクリーンリーダー対応のメッセージ
- キーボードショートカットの追加

### エラーハンドリング
- より詳細なエラーメッセージ
- ネットワークエラー時の再試行
- オフライン時の動作

## テスト状況

### 実施済みテスト
- ✅ ユーザ一覧表示
- ✅ 言語切り替え
- ✅ ユーザ追加モーダル表示
- ✅ フォーカスインジケータ表示（input）
- ✅ フォーカスインジケータ表示（button: primary）
- ✅ フォーカスインジケータ表示（button: secondary）
- ✅ ドロップダウンメニューのインジケータ

### 未実施テスト
- ⚠ ユーザ追加の実行
- ⚠ ユーザ編集の実行
- ⚠ ユーザ削除の実行
- ⚠ 管理者編集の実行
- ⚠ バリデーションエラーのハンドリング
- ⚠ エラーケースの動作確認

## 関連ドキュメント
- [ユーザ管理機能実装ドキュメント](./USER_MANAGEMENT.md) - バックエンド実装
- [アクセシビリティインジケータ](./ACCESSIBILITY_INDICATORS.md) - インジケータ仕様
- [動的言語メニュー](./DYNAMIC_LANGUAGE_MENU.md) - 言語切り替え機能
- [多言語化実装](./I18N_IMPLEMENTATION.md) - i18n基盤

## 参考情報

### 使用技術
- HTML5
- CSS3 (Flexbox)
- JavaScript (ES6 Modules)
- Tauri IPC
- i18next (多言語化)

### コーディング規約
- セミコロン必須
- async/awaitでの非同期処理
- エラーハンドリング必須
- 関数名はキャメルケース
- コメントは日本語/英語併記

### ファイル構成原則
- CSS: 機能ごとにファイル分割
- JS: モジュール化して再利用性向上
- HTML: セマンティックなマークアップ
