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

### Phase 2: データベーススキーマの確認と調整
- [ ] CATEGORY1テーブルの確認
  - [ ] USER_ID（ユーザーごとの費目管理）
  - [ ] CATEGORY1_ID（大分類ID）
  - [ ] CATEGORY1_NAME_JA（日本語名）
  - [ ] CATEGORY1_NAME_EN（英語名）
  - [ ] DISPLAY_ORDER（表示順）
- [ ] CATEGORY2テーブルの確認
  - [ ] USER_ID
  - [ ] CATEGORY2_ID（中分類ID）
  - [ ] CATEGORY1_ID（親ID）
  - [ ] CATEGORY2_NAME_JA
  - [ ] CATEGORY2_NAME_EN
  - [ ] DISPLAY_ORDER
- [ ] CATEGORY3テーブルの確認
  - [ ] USER_ID
  - [ ] CATEGORY3_ID（小分類ID）
  - [ ] CATEGORY2_ID（親ID）
  - [ ] CATEGORY3_NAME_JA
  - [ ] CATEGORY3_NAME_EN
  - [ ] DISPLAY_ORDER

### Phase 3: バックエンドAPI実装（Rust）

#### 3-1. データ構造の定義
- [ ] `Category1`, `Category2`, `Category3` 構造体の定義
- [ ] カテゴリツリー用のレスポンス構造体
- [ ] エラーハンドリング用のEnum

#### 3-2. 大分類（CATEGORY1）のAPI
- [ ] `get_category1_list` - 大分類一覧取得
  - [ ] ユーザーIDでフィルタリング
  - [ ] 表示順でソート
  - [ ] 子要素（中分類）も含めて取得
- [ ] `add_category1` - 大分類追加
  - [ ] バリデーション（名前の重複チェック）
  - [ ] 表示順の自動設定
- [ ] `update_category1` - 大分類更新
  - [ ] 名前の変更
  - [ ] 表示順の変更
- [ ] `move_category1_order` - 大分類の並び順変更
  - [ ] 上へ移動
  - [ ] 下へ移動
  - [ ] display_orderの再計算

#### 3-3. 中分類（CATEGORY2）のAPI
- [ ] `get_category2_list` - 中分類一覧取得（親IDで絞り込み）
- [ ] `add_category2` - 中分類追加
- [ ] `update_category2` - 中分類更新
- [ ] `move_category2_order` - 中分類の並び順変更

#### 3-4. 小分類（CATEGORY3）のAPI
- [ ] `get_category3_list` - 小分類一覧取得（親IDで絞り込み）
- [ ] `add_category3` - 小分類追加
- [ ] `update_category3` - 小分類更新
- [ ] `move_category3_order` - 小分類の並び順変更

#### 3-5. 統合API
- [ ] `get_category_tree` - 全階層のツリー構造を一度に取得
  - [ ] 大→中→小の階層構造でJSON返却
  - [ ] 現在の言語に応じた名前を返す

### Phase 4: フロントエンド実装（JavaScript）

#### 4-1. カテゴリ一覧表示
- [ ] バックエンドからツリーデータ取得
- [ ] モックデータの削除
- [ ] エラーハンドリング

#### 4-2. 大分類の追加・編集
- [ ] `openCategory1Modal()` の実装
- [ ] `handleCategory1Save()` の実装
  - [ ] フォームバリデーション
  - [ ] API呼び出し
  - [ ] 成功時のツリー再読み込み

#### 4-3. 中分類の追加・編集
- [ ] `openAddChildModal()` の実装（level=1の場合）
- [ ] `openEditModal()` の実装（level=2の場合）
- [ ] `handleCategory2Save()` の実装

#### 4-4. 小分類の追加・編集
- [ ] `openAddChildModal()` の実装（level=2の場合）
- [ ] `openEditModal()` の実装（level=3の場合）
- [ ] `handleCategory3Save()` の実装

#### 4-5. 並び順変更
- [ ] `moveCategoryUp()` の実装
- [ ] `moveCategoryDown()` の実装
- [ ] 楽観的UI更新（即座に表示を更新）
- [ ] エラー時のロールバック

### Phase 5: テスト実装

#### 5-1. バックエンドテスト
- [ ] カテゴリ追加のテスト
- [ ] カテゴリ更新のテスト
- [ ] 並び順変更のテスト
- [ ] バリデーションのテスト
- [ ] エラーケースのテスト

#### 5-2. フロントエンドテスト
- [ ] ツリー表示のテスト
- [ ] モーダル操作のテスト
- [ ] 並び順変更のテスト

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
