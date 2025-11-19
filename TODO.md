# TODO - KakeiBon Development Tasks

## 現在の実装状況

### 🎉 入力系画面実装完了 (2025-11-19)
**すべての入力系画面の基本実装が完了しました！**

完了した機能:
- ✅ ユーザー管理（Admin/一般ユーザー）
- ✅ 費目管理（3階層カテゴリ）
- ✅ 口座管理（複数口座サポート）
- ✅ 店舗管理（IS_DISABLED機能付き）
- ✅ メーカー管理（IS_DISABLED機能付き）
- ✅ 商品管理（メーカー連携、IS_DISABLED機能付き）
- ✅ 入出金管理（ヘッダ + 詳細フィルター）
- ✅ 入出金明細管理（スマート税計算、端数処理自動検出）
- ✅ UI/UX統一（フォントモーダル、メニューバーモジュール化）

次のステップ:
- 📊 集計・レポート機能の実装
- 📝 APIドキュメントの整備
- 🧪 テストカバレッジの拡充

### ✅ 完了済み
- [x] 基本認証機能（管理者・一般ユーザー登録、ログイン）
- [x] ユーザー管理画面（追加・編集・パスワード変更）
- [x] 多言語対応（日本語・英語）
- [x] 言語切り替えメニュー
- [x] フォントサイズ変更機能（small/medium/large + カスタム）
- [x] フォントサイズ機能のモジュール化
- [x] フォントサイズ機能のテストスイート（13テスト）
- [x] 費目管理機能（CATEGORY1/2/3完全実装）
- [x] 口座管理機能（ACCOUNTS完全実装）
- [x] 店舗管理機能（SHOPS完全実装）
- [x] 入出金管理機能（TRANSACTION_HEADERS完全実装）
- [x] データベースパス管理の統一（consts.rs）
- [x] 翻訳リソースの追加とデバッグ
- [x] 費目データ自動投入機能（ユーザー作成時）
- [x] 費目管理画面UI/UX改善（アクセシビリティ向上）

### ✅ 完了済み（最近追加）
- [x] メーカー管理機能（MANUFACTURERS）
  - [x] バックエンド実装完了（CRUD操作、名前重複チェック）
  - [x] フロントエンド実装完了（Modal統合、翻訳リソース）
  - [x] IS_DISABLED機能完全実装（2025-11-12完了）
    - トグルボタンで非表示項目の表示/非表示切り替え
    - モーダルにチェックボックス追加
    - アクセシビリティ改善（高コントラスト、バッジ表示）
- [x] 商品管理機能（PRODUCTS）
  - [x] バックエンド実装完了（CRUD操作、名前重複チェック、メーカーとの関連）
  - [x] フロントエンド実装完了（Modal統合、翻訳リソース）
  - [x] IS_DISABLED機能完全実装（2025-11-12完了）
    - メーカー管理と同様の実装

**注記**:
- 2025-11-12: メーカー・商品管理のIS_DISABLED機能完全実装完了
  - トグルボタン: 「非表示項目を表示/隠す」
  - モーダル: 「非表示」チェックボックス
  - 視覚表示: グレー背景(#6c757d) + 白文字(#ffffff) + 黄色バッジ
  - アクセシビリティ: 高コントラスト、ボタン鮮明、視覚障害者対応
  - 全150テスト成功

### 🔜 今後の実装予定
- [ ] **IS_DISABLED機能を他の管理画面に適用**（優先度: 中）
  - [ ] 費目管理（CATEGORY1/2/3）
  - [ ] 店舗管理（SHOPS）
  - [ ] 口座管理（ACCOUNTS）
  - **実装パターン**: メーカー・商品管理と同じパターンを適用
  - **実装内容**:
    1. SQL: GET_ALL_INCLUDING_DISABLED クエリ追加
    2. SQL: INSERT/UPDATE に IS_DISABLED カラム追加
    3. Backend: Request構造体に is_disabled フィールド追加
    4. Backend: get関数に include_disabled パラメータ追加
    5. Tauri: コマンドに is_disabled/include_disabled パラメータ追加
    6. HTML: トグルボタン + モーダルにチェックボックス追加
    7. JavaScript: showDisabledItems state + スタイリング + バッジ表示

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
- [x] 翻訳リソース統計ドキュメント（日本語・英語）
  - docs/ja/I18N_RESOURCES.md
  - docs/en/I18N_RESOURCES.md
- [x] 店舗管理APIドキュメント（日本語・英語）
  - docs/ja/API_SHOP.md
  - docs/en/API_SHOP.md
- [x] 入出金管理APIドキュメント更新（SHOP_ID追加、外部キー制約修正）
  - docs/ja/API_TRANSACTION.md
  - docs/en/API_TRANSACTION.md

### 未完了
- [ ] Rust APIドキュメント（rustdoc）の整備
  - `cargo doc` でドキュメント生成
  - 公開関数・構造体にドキュメントコメント追加
  - 使用例の追加
  - モジュールレベルのドキュメント

**注記**: 
- 2025-10-29: 基本ドキュメント整備完了（rustdocを除く）
- 2025-11-10: 翻訳リソース統計、店舗管理API、入出金管理API更新完了

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

#### 4-1. カテゴリ一覧表示 ✅
- [x] バックエンドからツリーデータ取得
- [x] モックデータの削除
- [x] 実データでのレンダリング
- [x] 大分類は操作ボタンなしで表示（サブカテゴリ追加のみ）
- [x] エラーハンドリング
- [x] CSS改善（横方向表示、ボタン配置統一）
- [x] 展開アイコンの大型化とユーザビリティ向上
- [x] 名称ダブルクリックで展開機能
- [x] スマートなフォーカス/ホバー管理（同時に1要素のみ二重線表示）
- [x] アクセシビリティ向上（2pxボーダー、:focus-visible対応）

**完了日**: 2025-10-31

**残課題**:
- ⚠️ I18Nデータ不足（日本語CATEGORY2/3_I18N: 必要、英語CATEGORY3_I18N: 116件不足）

#### 4-2. 中分類の追加・編集（モーダル方式） ✅
- [x] 「サブカテゴリ追加」ボタンでモーダル表示
- [x] 編集ボタンでモーダル表示（既存データ読み込み）
- [x] `add_category2` API呼び出し（フロントエンド実装済み）
- [x] `update_category2_i18n` API実装（バックエンド実装完了）
- [x] 動作テスト完了

#### 4-3. 小分類の追加・編集（モーダル方式） ✅
- [x] 中分類と同じモーダル方式
- [x] `add_category3` API呼び出し（フロントエンド実装済み）
- [x] `update_category3_i18n` API実装（バックエンド実装完了）
- [x] データ属性の修正（data-category3-code追加）
- [x] イベントハンドラの修正（適切なカテゴリコード取得）
- [x] 親カテゴリコードの受け渡し修正（parent1Code, parent2Code使用）
- [x] 動作テスト完了

**完了内容 (2025-10-30 08:11 JST)**:
- ✅ SQL定数追加: `CATEGORY2_CHECK_DUPLICATE_NAME_EXCLUDING`, `CATEGORY3_CHECK_DUPLICATE_NAME_EXCLUDING`
- ✅ バックエンド関数実装: `update_category2_i18n`, `update_category3_i18n`
- ✅ Tauriコマンド追加: `update_category2`, `update_category3`
- ✅ フロントエンド修正: レベル定数化（LEVEL_CATEGORY1/2/3）、即値比較を定数比較に変更
- ✅ 小分類ボタンのdata属性修正: `data-category3-code`追加
- ✅ 親カテゴリコードの受け渡し修正: `renderCategory2`→`renderCategory3`で`parent1Code`を使用
- ✅ 重複チェック機能: 編集対象を除外した重複チェック実装
- ✅ ドキュメント更新: `.ai-context/CONVENTIONS.md`にデータベース名禁止事項を追加
- ✅ 中分類・小分類の編集・保存機能の動作確認完了
- ✅ 実データ投入SQL生成: Python スクリプトでmigrate_categories.sqlを新コード体系に変換
- ✅ `initialize_user_categories`関数実装: SQLファイルから読み込んで実行
- ✅ ユーザー作成時の自動投入: `create_general_user`コマンドで自動呼び出し
- ✅ テストケース作成: `test_initialize_user_categories`で20中分類、126小分類、I18Nデータを検証
- ✅ 全テスト成功（6/6テスト成功）

**次回作業**:
1. ~~編集機能のドキュメント整備~~ ✅ 完了
2. ~~並び順変更機能の実装（Phase 4-4）~~ ✅ 完了

#### 4-4. 並び順変更 ✅
- [x] `moveCategoryUp()` の実装
- [x] `moveCategoryDown()` の実装
- [x] 楽観的UI更新（即座に表示を更新）
- [x] エラー時のロールバック（再読み込み）

**完了日**: 2025-11-04  
**実装内容**:
- バックエンドAPI: `move_category2_up/down`, `move_category3_up/down`
- フロントエンド: ボタンクリックで即座に移動処理実行
- 手動動作確認完了

#### 4-5. UI調整とエラーハンドリング ✅
- [x] ボタンの有効/無効制御
- [x] エラーメッセージの多言語対応
- [x] ローディング表示の改善

**完了日**: 2025-11-04  
**実装内容**:
- 保存ボタン: 処理中に無効化、「保存中...」表示
- 移動ボタン: ダブルクリック防止のため処理中に無効化
- エラー時のボタン状態復元
- I18Nキー追加: `common.saving`, `error.category_duplicate_name`, `category_mgmt.error_load_category`

### Phase 5: テスト実装

#### 5-1. バックエンドテスト ✅
- [x] カテゴリ追加のテスト
- [x] カテゴリ更新のテスト
- [x] 並び順変更のテスト
- [x] バリデーションのテスト
- [x] エラーケースのテスト

**完了日**: 2025-11-04  
**テスト数**: 125件（全て成功）  
**追加テスト**:
- `test_update_category2` - 中分類更新テスト
- `test_update_category3` - 小分類更新テスト
- `test_update_category2_duplicate_name` - 重複バリデーションテスト
- `test_move_category2_boundary` - 境界条件テスト
- `test_get_category_for_edit` - 編集データ取得テスト

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

### Phase 6: ドキュメント整備 ✅
- [x] 費目管理機能の実装ドキュメント作成（日本語）
- [x] 費目管理機能の実装ドキュメント作成（英語）
- [x] APIリファレンス

**完了日**: 2025-11-04  
**ドキュメント**:
- `docs/ja/CATEGORY_MANAGEMENT_UI.md` - 費目管理画面実装ドキュメント（日本語）
- `docs/en/CATEGORY_MANAGEMENT_UI.md` - Category Management Screen Implementation（英語）
- `docs/ja/API_CATEGORY_ja.md` - 費目管理APIドキュメント（日本語）
- `docs/en/API_CATEGORY.md` - Category Management API Documentation（英語）

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

最終更新: 2025-11-04

---

---

## 🎨 UI/UX統一プロジェクト ✅ (2025-11-18)

### フォントサイズモーダルの統一 ✅
**完了日**: 2025-11-18

#### 実装内容
- [x] 全8管理画面のフォントサイズモーダルを統一
  - 対象: account, category, manufacturer, product, shop, transaction, transaction-detail, user
- [x] モーダル構造の統一（プリセット + パーセンテージ形式）
- [x] CSSの相対単位化（px → em）
- [x] モーダル幅の統一（500px → 31em）
- [x] フォントサイズ追従の保証

#### 技術詳細
- HTMLテンプレート: index.htmlと同じ構造に統一
- JavaScript: font-size.jsモジュールを使用
- CSS: 固定フォントサイズ（14px）を継承（inherit）に変更
- モーダル幅: max-width: 31em（フォントサイズに追従）

#### 効果
- 保守性向上: モーダル定義が一元化
- 一貫性向上: 全画面で同じUI/UX
- アクセシビリティ向上: フォントサイズ変更に完全対応

---

### メニューバーのモジュール化 ✅
**完了日**: 2025-11-18

#### 実装内容
- [x] createMenuBar(pageType)関数の追加（menu.js）
- [x] 全8管理画面でメニューバーを動的生成
- [x] HTMLコードの大幅削減（約176行削減）
- [x] 管理メニューの全画面追加
- [x] メニュー排他制御の実装

#### 技術詳細
- メニュー定義: 一元管理（menu.js）
- 画面タイプ対応: index, management, transaction-detail
- HTML簡略化: `<div id="menu-bar"></div>`のみ
- 動的生成タイミング: DOMContentLoaded時

#### 効果
- 保守性向上: メニュー変更が1箇所で完結
- コード削減: 各ファイル約22行削減（8ファイル×22行=176行）
- アクセシビリティ向上: 全画面で管理メニューが利用可能
- 一貫性向上: 全メニューで排他制御が動作

#### モジュール化の恩恵を実感
管理メニューの排他制御修正を1箇所（menu.js）で実施するだけで、全8画面に即座に反映。
これがモジュール化の真価。

---

## 💰 入出金管理機能 (Transaction Management) - TRANSACTION_HEADERS

### Phase 1: データベース・バックエンドAPI ✅ (2025-11-08)
- [x] TRANSACTION_HEADERSテーブル設計
- [x] インデックス設計
- [x] `get_transaction_header` API
- [x] `select_transaction_headers` API  
- [x] `save_transaction_header` API
- [x] `update_transaction_header` API
- [x] `delete_transaction_header` API
- [x] メモ管理機能（自動再利用・新規作成）
- [x] FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE対応
- [x] SHOP_ID対応

**完了日**: 2025-11-08

### Phase 2: フロントエンド（一覧表示） ✅ (2025-11-09)
- [x] transaction-management.html/css/js
- [x] 管理メニューに追加
- [x] 一覧表示（カテゴリ名表示）
- [x] ページネーション（50件/ページ）
- [x] 削除機能
- [x] フィルター機能
  - [x] パネル開閉
  - [x] 日付範囲フィルター
  - [x] カテゴリドロップダウン（3階層動的選択）
  - [x] 金額範囲フィルター
  - [x] キーワード検索（メモ対象）
  - [x] 複合条件フィルター

**完了日**: 2025-11-09

### Phase 3: 登録・編集機能 ✅ (2025-11-09)
- [x] 新規登録モーダル（ヘッダー登録）
- [x] 編集モーダル（ヘッダー編集）
- [x] メモ管理機能
- [x] FROM口座・TO口座選択機能
- [x] 店舗選択機能
- [x] テスト実装（98件のJavaScriptテスト）
- [x] APIドキュメント作成（日英）

**完了日**: 2025-11-09

### Phase 4: UI/UX改善 ✅ (2025-11-16)
- [x] **ウィンドウ最小サイズ調整**
  - 問題: コンテンツがボタンと重なる表示問題
  - 対策: 最小ウィンドウサイズを800px→1100pxに変更
  - 効果: 金額（10億円まで）とボタンの重なり解消、全画面での視認性向上
  - 完了日: 2025-11-16

**完了日**: 2025-11-16

### Phase 5: 明細件数表示 ✅ (2025-11-18)
- [x] **明細件数表示機能の実装**
  - 編集時: データベースから明細を取得して件数を表示
  - 新規作成時: 件数を0にリセット
  - UIフィードバック: 明細管理ボタンの右側に件数表示
  - 完了日: 2025-11-18

**完了日**: 2025-11-18

### Phase 6: 未実装機能
- [ ] **店舗名表示**（一覧画面）
  - 現状: SHOP_IDは保存されているが、一覧に店舗名が表示されていない
  - 優先度: 中
- [ ] **入出金明細管理**（TRANSACTION_DETAILS）
  - 明細の追加・編集・削除・並び替え
  - 金額自動計算・税額計算
  - 明細管理モーダル/画面
  - 優先度: 高

### Phase 6: UI/UX改善（検討中）
- [ ] ソート機能
- [ ] CSVエクスポート機能

---

## 🏪 店舗管理機能 (Shop Management) ✅ 完了

### Phase 1: データベース・バックエンドAPI ✅ (2025-11-10)
- [x] SHOPSテーブル設計
- [x] `get_shops` API
- [x] `add_shop` API
- [x] `update_shop` API
- [x] `delete_shop` API（論理削除）
- [x] 重複チェック機能
- [x] 表示順管理

**完了日**: 2025-11-10

### Phase 2: フロントエンド ✅ (2025-11-10)
- [x] shop-management.html/css/js
- [x] 管理メニューに追加
- [x] 一覧表示
- [x] 追加・編集・削除機能
- [x] 表示順変更機能
- [x] バリデーション
- [x] テスト実装
- [x] APIドキュメント作成（日英）

**完了日**: 2025-11-10

### Phase 3: 入出金管理との連携 ✅ (2025-11-10)
- [x] TRANSACTION_HEADERSにSHOP_ID追加
- [x] 入出金登録・編集時の店舗選択機能
- [x] 外部キー制約設定
- [ ] 入出金一覧での店舗名表示（未実装）

**完了日**: 2025-11-10（店舗名表示を除く）

---

## 🏦 口座管理機能 (Account Management) ✅ 完了

### Phase 1: データベース・バックエンドAPI ✅
- [x] ACCOUNTSテーブル設計
- [x] `get_accounts` API
- [x] `add_account` API
- [x] `update_account` API
- [x] `delete_account` API
- [x] 口座コード管理（USER_ID, ACCOUNT_CODE複合キー）

### Phase 2: フロントエンド ✅
- [x] account-management.html/css/js
- [x] 管理メニューに追加
- [x] 一覧表示
- [x] 追加・編集・削除機能
- [x] テンプレート選択機能
- [x] 初期残高設定
- [x] 表示順管理

### Phase 3: 入出金管理との連携 ✅
- [x] FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE対応
- [x] 入出金登録・編集時の口座選択機能
- [x] 外部キー制約設定（複合キー）

**完了日**: 2025-11-10

---

## 🔧 次のタスク（優先度：高）

### IS_DISABLED機能の実装（全管理画面共通）
**優先度**: 高（メーカー・商品管理の完成に必要）

#### 背景
- 手動テスト中に、IS_DISABLED=1のデータが一覧に表示されない問題を発見
- 現在の実装では、削除時に論理削除（IS_DISABLED=1）するが、表示/非表示を切り替える機能がない
- メーカー、商品、店舗、口座、費目などの管理画面で共通して必要な機能

#### 実装内容
- [ ] **モーダルへのチェックボックス追加**
  - 追加/編集モーダルに「非表示」チェックボックスを追加
  - チェックON: IS_DISABLED = 1（一覧に表示されない）
  - チェックOFF: IS_DISABLED = 0（一覧に表示される）

- [ ] **一覧表示の改善**
  - 非表示項目を半透明/グレーアウトで表示するオプション
  - 「非表示項目を表示」トグルボタンの追加

- [ ] **対象画面**
  - メーカー管理（MANUFACTURERS）
  - 商品管理（PRODUCTS）
  - 店舗管理（SHOPS）
  - 口座管理（ACCOUNTS）
  - 費目管理（CATEGORY1/2/3）

#### 翻訳リソース
- [ ] 共通翻訳リソースの追加
  - `common.is_disabled` - 非表示
  - `common.show_disabled` - 非表示項目を表示
  - `common.hide_disabled` - 非表示項目を隠す

**注記**: この機能は論理削除を採用している全ての管理画面で有用

---

## 📊 未実装機能（優先度順）

### 高優先度
- [ ] **入出金明細管理機能**（TRANSACTION_DETAILS）
  - **Phase 1: データベース設計** ✅ (2025-11-17)
    - [x] TRANSACTIONS_DETAILテーブル設計
    - [x] USER_ID, TRANSACTION_ID, DETAIL_ID主キー
    - [x] 外部キー制約（TRANSACTION_HEADERS, CATEGORY2, CATEGORY3）
    - [x] インデックス設計
    - [x] AMOUNT_INCLUDING_TAX カラム追加（税込金額保存用）
    - [x] メモテーブル（MEMOS）の外部キー制約修正（明細親、メモ子の関係に変更）
  - **Phase 2: バックエンドAPI** ✅ (2025-11-18)
    - [x] `get_transaction_details` - 明細一覧取得
    - [x] `add_transaction_detail` - 明細追加
    - [x] `update_transaction_detail` - 明細更新
    - [x] `delete_transaction_detail` - 明細削除
    - [x] メモ管理（明細単位）
    - [x] バリデーション（金額・税額の範囲チェック）
  - **Phase 3: フロントエンド（入力フォーム）** ✅ (2025-11-18)
    - [x] transaction-detail-management.html/css/js
    - [x] CATEGORY1_CODEをセッション管理に変更
    - [x] 税抜金額・税込金額の両方入力可能なUI
    - [x] 相互自動計算ロジック実装（丸め誤差検出含む）
    - [x] カテゴリドロップダウンの動的更新実装
    - [x] メモ機能の実装
    - [x] バリデーション実装
    - [x] 翻訳リソース追加
  - **Phase 4: フロントエンド（一覧表示・CRUD）** ✅ (2025-11-18)
    - [x] 明細一覧表示（テーブル形式）
    - [x] 明細の追加・編集・削除UI完全統合
    - [x] ローカル日時での保存（ENTRY_DT/UPDATE_DT）
  - **Phase 5: UI/UX統一** ✅ (2025-11-18)
    - [x] フォントサイズモーダルの統一（全8管理画面）
    - [x] メニューバーのモジュール化（全8管理画面）
    - [x] ボタンスタイルの統一
    - [x] モーダル幅の改善（固定px→相対em）
  - **Phase 6: 入出金管理との連携** ✅ (2025-11-18)
    - [x] 明細件数表示の実装
    - [x] 明細管理ボタンからの遷移
  - **Phase 7: 金額計算機能** ✅ (2025-11-18)
    - [x] 税抜⇔税込の相互計算（基本実装）
    - [x] 丸め誤差の検出と警告表示
    - [x] フィールド変更時の警告クリア
    - [x] 端数処理の自動検出（TAX_ROUNDING_TYPE）
      - [x] 3種類の端数処理での税抜金額計算（切り捨て/切り上げ/四捨五入）
      - [x] 再計算による入力値との照合
      - [x] 優先順位による自動選択（切り捨て > 四捨五入 > 切り上げ）
      - [x] テストケース9パターン実装（全て成功）
    - [x] 明細合計の自動計算（将来の拡張として保留）
    - [x] ヘッダー金額との整合性チェック（将来の拡張として保留）
  - **Phase 8: テスト実装** ✅ (2025-11-18)
    - [x] バックエンドテスト（CRUD、計算ロジック）
    - [x] フロントエンドテスト（UI操作、バリデーション）
    - [x] 端数処理自動検出テスト（9パターン）
  - **Phase 9: ドキュメント**
    - [ ] API仕様書（日英）
    - [ ] 明細管理機能仕様書（日英）
- [ ] **集計・レポート機能**
  
  #### 時間軸別集計
  - [ ] 日次集計（Daily Summary）
  - [ ] 週次集計（Weekly Summary）
  - [ ] 月次集計（Monthly Summary）
  - [ ] 年次集計（Annual Summary）
  - [ ] カスタム期間集計（Custom Period）
  
  #### カテゴリ別集計
  - [ ] 大分類別集計（CATEGORY1）
  - [ ] 中分類別集計（CATEGORY2）
  - [ ] 小分類別集計（CATEGORY3）
  - [ ] 階層ドリルダウン機能（大→中→小）
  
  #### 口座・店舗別集計
  - [ ] 口座別集計（Account Summary）
  - [ ] 店舗別集計（Shop Summary）
  - [ ] 口座間移動集計（Transfer Summary）
  
  #### 商品・メーカー別集計
  - [ ] メーカー別集計（Manufacturer Summary）
  - [ ] 商品別集計（Product Summary）
  - [ ] 商品カテゴリクロス集計
  
  #### メモ・キーワード集計
  - [ ] メモキーワード検索集計
  - [ ] タグ別集計（将来拡張）
  
  #### 金額条件抽出（家計改善機能）
  - [ ] 一定額以上の取引抽出（高額出費の洗い出し）
  - [ ] 一定額以下の取引抽出（小額浪費の可視化）
  - [ ] 金額範囲指定抽出（例: 1000円～5000円の範囲）
  - [ ] カテゴリ別金額ランキング（支出TOP10等）
  - [ ] 店舗別累計金額ランキング（よく使う店舗の特定）
  - [ ] 商品別累計購入額（何に一番お金を使っているか）
  
  #### 財務レポート（海外向け）
  - [ ] バランスシート（Balance Sheet / 貸借対照表）
    - 資産（Assets）: 各口座残高
    - 負債（Liabilities）: 未払金など（将来拡張）
    - 純資産（Net Worth）: 資産 - 負債
  - [ ] 損益計算書（Income Statement / 損益計算書）
    - 収入（Income）: CATEGORY1='INCOME'
    - 支出（Expenses）: CATEGORY1='EXPENSE'
    - 収支差額（Net Income）: 収入 - 支出
  - [ ] キャッシュフロー計算書（Cash Flow Statement）
    - 営業CF（Operating）: 通常の収支
    - 投資CF（Investing）: 投資関連（将来拡張）
    - 財務CF（Financing）: 借入・返済（将来拡張）
  
  #### グラフ表示
  - [ ] 棒グラフ（Bar Chart）: 時系列推移、カテゴリ比較
  - [ ] 円グラフ（Pie Chart）: カテゴリ構成比
  - [ ] 折れ線グラフ（Line Chart）: 残高推移、トレンド
  - [ ] 積み上げグラフ（Stacked Chart）: 複数カテゴリの推移
  
  #### 実装優先順位
  **Phase 1**: 基本集計API（高優先度）
  
  #### Step 0: 基礎構造の実装（足場固め）- 型安全設計
  
  **設計思想**: Enumで条件を規定し、不正なSQL生成をコンパイル時に防ぐ
  
  - [ ] **フィルタEnum定義** (`src/services/aggregation.rs`)
    - [ ] `DateFilter` - 日付フィルタ
      - `From(NaiveDate)` - 指定日以降
      - `To(NaiveDate)` - 指定日以前
      - `Between(NaiveDate, NaiveDate)` - 期間範囲
      - `Exact(NaiveDate)` - 特定日
    - [ ] `AmountFilter` - 金額フィルタ
      - `GreaterThan(i64)` - 指定額以上
      - `LessThan(i64)` - 指定額以下
      - `Between(i64, i64)` - 金額範囲
      - `Exact(i64)` - 特定金額
      - `None` - フィルタなし
    - [ ] `CategoryFilter` - カテゴリフィルタ
      - `Category1(String)` - 大分類のみ
      - `Category2(String, String)` - 大分類+中分類
      - `Category3(String, String, String)` - 大分類+中分類+小分類
      - `None` - フィルタなし
    - [ ] 各Enumに`to_sql(&self) -> String`実装（SQL句生成）
  
  - [ ] **集計軸Enum定義**
    - [ ] `GroupBy` - 集計軸
      - `Category1` - 大分類別
      - `Category2` - 中分類別
      - `Category3` - 小分類別
      - `Account` - 口座別
      - `Shop` - 店舗別
      - `Product` - 商品別
      - `Date` - 日付別
    - [ ] 各バリアントに`to_select_clause()`, `to_group_by_clause()`実装
  
  - [ ] **ソートEnum定義**
    - [ ] `OrderField` - ソート対象
      - `TransactionDate` - 取引日
      - `Amount` - 金額
      - `CategoryName` - カテゴリ名
      - `ShopName` - 店舗名
      - `Count` - 件数
    - [ ] `SortOrder` - ソート順
      - `Asc` - 昇順
      - `Desc` - 降順
    - [ ] `to_order_by_clause()`実装
  
  - [ ] **複合構造体定義**
    - [ ] `AggregationFilter` - フィルタをまとめる構造体
      ```rust
      struct AggregationFilter {
          date: DateFilter,                    // 必須
          amount: Option<AmountFilter>,        // オプション
          category: Option<CategoryFilter>,    // オプション
          shop_id: Option<i64>,                // オプション
      }
      ```
    - [ ] `AggregationRequest` - 集計リクエスト全体
      ```rust
      struct AggregationRequest {
          user_id: i64,
          filter: AggregationFilter,
          group_by: GroupBy,
          order_by: OrderField,
          sort_order: SortOrder,
          limit: Option<usize>,
      }
      ```
    - [ ] `AggregationResult` - 集計結果
      ```rust
      struct AggregationResult {
          group_key: String,      // カテゴリ名、店舗名等
          total_amount: i64,      // 合計金額
          count: i64,             // 件数
          avg_amount: i64,        // 平均金額
      }
      ```
  
  - [ ] **SQL生成関数**
    - [ ] `build_where_clause(filter: &AggregationFilter) -> String`
    - [ ] `build_query(request: &AggregationRequest) -> String`
  
  - [ ] **テスト**: 各Enum/構造体のto_sql()動作確認
  
  **メリット**:
  - 型安全: 不正な組み合わせをコンパイル時検出
  - 保守性: SQL生成ロジックが各Enumに集約
  - テスト容易性: Enumごとに独立してテスト
  - バグ削減: match式の網羅性チェック
  
  **設計方針**:
  - **2階層アーキテクチャ**: コア関数（1つ）+ ラッパー関数（複数）
  - **責務分離（SRP）**:
    - **コア関数（SQLジェネレータ）**: パラメータから動的にSQLを生成・実行
      - 日付バリデーション済みを前提
      - パラメータに応じたSELECT/GROUP BY/WHERE/ORDER BY句の動的生成
      - GROUP BY軸（カテゴリ/口座/店舗/商品）の切り替え
      - 動的なORDER BY/ソート順対応
      - 金額フィルタリング（WHERE句への追加）
      - 純粋なSQL生成・実行ロジックのみ
    - **ラッパー関数（ビジネスロジック層）**: コア関数に渡す期間の妥当性検証に専念
      - 日付の論理チェック（未来日付、範囲チェック等）
      - 期間タイプ固有のビジネスロジック（週の始まり、月末日等）
      - コア関数（SQLジェネレータ）への適切なパラメータ変換
      - デフォルトのソート順設定（日次→日付昇順、金額順→降順等）
  - **保守性**: SQL生成ロジック変更時はコア関数のみ修正（ラッパーは影響なし）
  - **拡張性**: 新しい期間タイプ追加が容易（ラッパー追加のみ）
  - **テスト効率**: コア関数（SQL生成）を重点的にテスト、ラッパーは軽量テスト
  
  **Phase 2**: 集計画面UI（高優先度）
  - [ ] 集計結果テーブル表示
  - [ ] 期間選択UI
  - [ ] カテゴリフィルター
  - [ ] CSVエクスポート
  
  **Phase 3**: 財務レポート（中優先度）
  - [ ] バランスシート画面
  - [ ] 損益計算書画面
  - [ ] 多言語対応（日英）
  
  **Phase 4**: グラフ表示（中優先度）
  - [ ] Chart.js統合
  - [ ] 基本グラフ実装
  - [ ] インタラクティブ機能
  
  **Phase 5**: 高度な集計（低優先度）
  - [ ] キャッシュフロー計算書
  - [ ] 予算vs実績比較
  - [ ] トレンド分析

### 中優先度
- [ ] **入出金一覧の店舗名表示**
  - SHOP_IDから店舗名をJOINして表示
- [ ] **データのエクスポート機能（CSV）**
  - 入出金データエクスポート
  - 集計結果エクスポート
- [ ] **データベースバックアップ・リストア機能**
  - SQLiteデータベースのバックアップ
  - リストア機能
  - 自動バックアップ設定
- [ ] **予算設定機能**
  - カテゴリ別予算設定
  - 予算vs実績表示

### 低優先度
- [ ] グラフ表示機能（高度な可視化）
- [ ] 複数通貨対応
- [ ] 日付ピッカーのカスタマイズ

---

---

## 🎯 端数処理の自動検出機能の実装詳細 (2025-11-18)

### 実装内容
- **目的**: 税込金額と税率から税抜金額を逆算する際、店舗が採用している端数処理方法（切り捨て/切り上げ/四捨五入）を自動で検出
- **アルゴリズム**:
  1. 税込金額から3種類の端数処理で税抜金額を計算
  2. それぞれを元に税込金額を再計算
  3. ユーザー入力値と一致するものを採用
  4. 複数一致の場合は優先順位（切り捨て > 四捨五入 > 切り上げ）で選択
- **実装ファイル**: `res/js/transaction-detail-validation.js`
- **テスト**: 9パターンのテストケース実装（全て成功）

### 効果
- ユーザーが端数処理を手動で選択する必要がなくなる
- レシート入力時の利便性向上
- 税額計算の精度向上

---

最終更新: 2025-11-19 08:27 JST
