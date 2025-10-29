# TODO - KakeiBon Development Tasks

## 現在の実装状況

### ✅ 完了済み
- [x] 基本認証機能（管理者・一般ユーザー登録、ログイン）
- [x] ユーザー管理画面（追加・編集・パスワード変更）
- [x] 多言語対応（日本語・英語）
- [x] 言語切り替えメニュー
- [x] フォントサイズ変更機能（small/medium/large + カスタム）
- [x] フォントサイズ機能のモジュール化
- [x] フォントサイズ機能のテストスイート（13テスト）
- [x] 費目管理画面のスケルトン（フロントエンドのみ）
- [x] データベースパス管理の統一（consts.rs）
- [x] 翻訳リソースの追加とデバッグ

---

## 📝 ドキュメント整備（優先）

### 作成済み
- [x] トラブルシューティングガイド（日本語・英語）
  - docs/ja/TROUBLESHOOTING.md
  - docs/en/TROUBLESHOOTING.md
- [x] データベース設定ガイド（日本語・英語）
  - docs/ja/DATABASE_CONFIGURATION.md
  - docs/en/DATABASE_CONFIGURATION.md

### 完了済み
- [x] トラブルシューティングガイド（日本語・英語）
- [x] データベース設定ガイド（日本語・英語）
- [x] I18N API仕様書の更新（日本語・英語）
- [x] 開発者向けガイド（日本語・英語）
- [x] データベースマイグレーションガイド（日本語・英語）

### 未完了
- [ ] Rust APIドキュメント（rustdoc）の整備
  - `cargo doc` でドキュメント生成
  - 公開関数・構造体にドキュメントコメント追加
  - 使用例の追加
  - モジュールレベルのドキュメント

**注記**: 2025-10-29ドキュメント整備完了（rustdocを除く）

---

## 🚧 費目管理機能 (Category Management)

### Phase 1: i18nリソースの追加 ✅
- [x] SQLに費目管理用の翻訳リソースを追加
  - [x] `category_mgmt.title` - 費目管理
  - [x] `category_mgmt.category_tree` - 費目ツリー
  - [x] `category_mgmt.add_category1` - 大分類を追加
  - [x] `category_mgmt.add_category2` - 中分類を追加
  - [x] `category_mgmt.add_category3` - 小分類を追加
  - [x] `category_mgmt.edit_category1` - 大分類を編集
  - [x] `category_mgmt.edit_category2` - 中分類を編集
  - [x] `category_mgmt.edit_category3` - 小分類を編集
  - [x] `category_mgmt.name_ja` - 名前（日本語）
  - [x] `category_mgmt.name_en` - 名前（英語）
  - [x] `category_mgmt.display_order` - 表示順
  - [x] `category_mgmt.parent_category` - 親カテゴリ
  - [x] `category_mgmt.add_sub` - サブカテゴリ追加
  - [x] `category_mgmt.order` - 順序
  - [x] `category_mgmt.no_categories` - カテゴリがありません
  - [x] `common.edit` - 編集
  - [x] `common.save` - 保存
  - [x] `common.cancel` - キャンセル
  - [x] `common.loading` - 読み込み中

### Phase 2: データベーススキーマの確認と調整 ✅
- [x] CATEGORY1テーブルの確認
  - [x] USER_ID（ユーザーごとの費目管理）
  - [x] CATEGORY1_CODE（大分類コード - IDとして機能）
  - [x] CATEGORY1_NAME（デフォルト名）
  - [x] CATEGORY1_I18N（多言語名 - 別テーブルで管理）
  - [x] DISPLAY_ORDER（表示順）
- [x] CATEGORY2テーブルの確認
  - [x] USER_ID
  - [x] CATEGORY2_CODE（中分類コード）
  - [x] CATEGORY1_CODE（親ID）
  - [x] CATEGORY2_NAME（デフォルト名）
  - [x] CATEGORY2_I18N（多言語名）
  - [x] DISPLAY_ORDER
- [x] CATEGORY3テーブルの確認
  - [x] USER_ID
  - [x] CATEGORY3_CODE（小分類コード）
  - [x] CATEGORY2_CODE（親ID）
  - [x] CATEGORY3_NAME（デフォルト名）
  - [x] CATEGORY3_I18N（多言語名）
  - [x] DISPLAY_ORDER

**注記**: 多言語対応は各カラムに直接JA/ENを持つのではなく、I18Nテーブルで管理する設計を採用（よりスケーラブル）

### Phase 3: バックエンドAPI実装（Rust） ✅

#### 3-1. データ構造の定義 ✅
- [x] `Category1`, `Category2`, `Category3` 構造体の定義
- [x] カテゴリツリー用のレスポンス構造体
- [x] エラーハンドリング用のEnum（Result<T, String>を使用）

#### 3-2. 大分類（CATEGORY1）のAPI ✅
- [x] `get_category1_list` - 大分類一覧取得
  - [x] ユーザーIDでフィルタリング
  - [x] 表示順でソート
  - [x] 子要素（中分類）も含めて取得（via get_category_tree）
- [x] `add_category1` - 大分類追加
  - [x] 表示順の自動設定
  - ⚠️ バリデーション（名前の重複チェック）は未実装
- [x] `update_category1` - 大分類更新
  - [x] 名前の変更
- [x] `move_category1_order` - 大分類の並び順変更
  - [x] 上へ移動
  - [x] 下へ移動
  - [x] display_orderの再計算
- [x] `delete_category1` - 大分類削除（CASCADE）

#### 3-3. 中分類（CATEGORY2）のAPI ✅
- [x] `add_category2` - 中分類追加
- [x] `update_category2` - 中分類更新
- [x] `move_category2_order` - 中分類の並び順変更
- [x] `delete_category2` - 中分類削除（CASCADE）

#### 3-4. 小分類（CATEGORY3）のAPI ✅
- [x] `add_category3` - 小分類追加
- [x] `update_category3` - 小分類更新
- [x] `move_category3_order` - 小分類の並び順変更
- [x] `delete_category3` - 小分類削除

#### 3-5. 統合API ✅
- [x] `get_category_tree` - 全階層のツリー構造を一度に取得
  - [x] 大→中→小の階層構造でJSON返却
  - [x] 現在の言語に応じた名前を返す（via get_category_tree_with_lang）

**注記**: 
- 削除APIは実装済みだが、内部的に使用するAPIのため、UIには公開しない（ユーザー削除時のみ使用）
- バリデーション（重複チェック等）は後続フェーズで実装予定

### Phase 3.5: バックエンドテスト ✅
- [x] テストデータベースセットアップ（インメモリSQLite）
- [x] Category1 CRUDテスト
  - [x] 追加テスト
  - [x] 更新テスト
  - [x] 削除テスト（CASCADE検証）
- [x] Category2 CRUDテスト
  - [x] 追加テスト
  - [x] 更新テスト
  - [x] 削除テスト（CASCADE検証）
- [x] Category3 CRUDテスト
  - [x] 追加テスト
  - [x] 更新テスト
  - [x] 削除テスト
- [x] ビジネスロジックテスト
  - [x] 表示順管理テスト
  - [x] ユーザーデータ分離テスト
- [x] テスト戦略ドキュメント作成（docs/TESTING.md）

**テスト結果**: 11 tests passed ✅

### Phase 4: フロントエンド実装（JavaScript）

#### 4-0. 初期データの準備 ✅
- [x] テンプレートユーザー（USER_ID=1）の大分類を更新
  - [x] EXPENSE（支出）、INCOME（収入）、TRANSFER（振替）
  - [x] コードを数値から文字列に変更
- [x] 大分類のI18Nレコード登録（日本語・英語）
- [x] Category2/3のコードも更新（C2_E_1、C3_1 形式）
- [x] 孤立データ（USER_ID=999）の削除

**注記**: 大分類は固定で、ユーザーによる追加・編集・削除は不可

#### 4-1. カテゴリ一覧表示
- [ ] バックエンドからツリーデータ取得
- [ ] モックデータの削除
- [ ] 実データでのレンダリング
- [ ] 大分類は操作ボタンなしで表示（サブカテゴリ追加のみ）
- [ ] エラーハンドリング

#### 4-2. 中分類の追加・編集（モーダル方式） 🚧
- [x] 「サブカテゴリ追加」ボタンでモーダル表示
- [x] 編集ボタンでモーダル表示（既存データ読み込み）
- [x] `add_category2` API呼び出し（フロントエンド実装済み）
- [ ] `update_category2_i18n` API実装（**バックエンド未実装**）
  - **再開ポイント**: src/services/category.rsに`update_category2_i18n`関数を実装
  - パラメータ: user_id, category1_code, category2_code, name_ja, name_en
  - CATEGORY2_I18Nテーブルを更新（UPSERT）

#### 4-3. 小分類の追加・編集（モーダル方式） 🚧
- [x] 中分類と同じモーダル方式
- [x] `add_category3` API呼び出し（フロントエンド実装済み）
- [ ] `update_category3_i18n` API実装（**バックエンド未実装**）
  - src/services/category.rsに`update_category3_i18n`関数を実装
  - パラメータ: user_id, category1_code, category2_code, category3_code, name_ja, name_en
  - CATEGORY3_I18Nテーブルを更新（UPSERT）

**現在の状況 (2025-10-30 00:00 JST)**:
- ✅ フロントエンド（編集モーダル、データ取得、保存処理）実装完了
- ✅ バックエンドAPI（get_category2_for_edit, get_category3_for_edit）実装完了
- ❌ バックエンド更新関数（update_category2_i18n, update_category3_i18n）未実装
- ⚠️ 動作テスト結果: "Command update_category2 not found" エラー発生
- 📝 原因: src-tauri/src/commands/category.rsが呼び出している`category::update_category2_i18n`が存在しない

**次回作業手順**:
1. src/services/category.rsに`update_category2_i18n`と`update_category3_i18n`を実装
2. src/lib.rsで公開関数として追加
3. 動作テスト（中分類・小分類の編集）
4. テストケース作成
5. ドキュメント整備

#### 4-4. 並び順変更
- [ ] `moveCategoryUp()` の実装
- [ ] `moveCategoryDown()` の実装
- [ ] 楽観的UI更新（即座に表示を更新）
- [ ] エラー時のロールバック（再読み込み）

#### 4-5. UI調整とエラーハンドリング
- [ ] ボタンの有効/無効制御
- [ ] エラーメッセージの多言語対応
- [ ] ローディング表示の改善

### Phase 5: テスト実装

#### 5-1. バックエンドテスト
- [ ] カテゴリ追加のテスト
- [ ] カテゴリ更新のテスト
- [ ] 並び順変更のテスト
- [ ] バリデーションのテスト
- [ ] エラーケースのテスト

#### 5-2. フロントエンドテスト
- [ ] ツリー表示のテスト
- [ ] モーダル操作のテスト（共通Modal クラス）
  - [ ] ESCキーでの閉じる動作
  - [ ] フォーカストラップ（TAB/SHIFT+TAB）
  - [ ] 開閉時のaria属性の変更
  - [ ] バックドロップクリックでの閉じる動作
  - [ ] 各画面でのモーダル動作確認
- [ ] 並び順変更のテスト

**注記**: フロントエンドテストはJest等のテストフレームワーク導入後に実装予定（現在は手動テストのみ）

### Phase 6: ドキュメント整備
- [ ] 費目管理機能の実装ドキュメント作成（日本語）
- [ ] 費目管理機能の実装ドキュメント作成（英語）
- [ ] APIリファレンス

---

## 📋 将来の機能（優先度順）

### 高優先度
- [ ] 入出金データ登録画面
- [ ] 入出金データ一覧・編集画面
- [ ] 月次集計・レポート機能

### 中優先度
- [ ] データのエクスポート機能（CSV）
- [ ] データのバックアップ・リストア
- [ ] 予算設定機能

### 低優先度
- [ ] グラフ表示機能
- [ ] 複数通貨対応
- [ ] モバイル対応の最適化

---

## 🐛 既知の問題・改善点

### バグ
（現時点でなし）

### 改善提案
- [ ] パスワード強度チェックの視覚化
- [ ] フォーム入力時のリアルタイムバリデーション
- [ ] アクセシビリティの更なる向上

---

## 🗑️ データ削除機能（後回し）

### ユーザー削除時の関連データ削除
**実装タイミング**: 入出金データ管理機能の実装後

ユーザーアカウントを削除する際に、関連する以下のデータを併せて削除する必要がある：
- [ ] 入出金データの削除
- [ ] 費目データ（CATEGORY1/2/3）の削除
- [ ] その他ユーザーに紐づくデータの削除

**設計方針**:
- ユーザー削除機能のサブセットとして実装
- トランザクション内で全データを削除
- 外部キー制約（ON DELETE CASCADE）を活用
- 削除前の確認ダイアログを表示
- 削除実行後はログイン画面へ遷移

**注意事項**:
- 費目データは通常削除しないが、ユーザー削除時は例外
- ゴミデータを残さないため、完全削除を保証

---

## 📝 メモ

### 設計方針
- **費目削除は実装しない**: 入出金データとの整合性を保つため（ユーザー削除時を除く）
- **ユーザーごとの費目**: 各ユーザーが独自の費目体系を持つ
- **多言語対応**: 費目名は日本語・英語の両方を保存
- **並び順の柔軟性**: ユーザーが自由に並び順を変更可能
- **カスケード削除**: ユーザー削除時は関連する費目・入出金データも削除

### 技術スタック
- **フロントエンド**: Vanilla JS (ES6 Modules), CSS3
- **バックエンド**: Rust, Tauri v2
- **データベース**: SQLite
- **テスト**: Cargo test, JavaScript unit tests

---

最終更新: 2025-10-27
