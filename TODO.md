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
- [x] 費目管理画面の実装（Phase 4-0～4-3 + 4-1完了）
- [x] データベースパス管理の統一（consts.rs）
- [x] 翻訳リソースの追加とデバッグ
- [x] 費目データ自動投入機能（ユーザー作成時）
- [x] 費目管理画面UI/UX改善（アクセシビリティ向上）

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

### 検討中
- [ ] ボタン表示モード切り替え機能
  - ユーザー設定でボタンをテキスト表示/アイコン表示に切り替え可能にする案
  - 全画面で統一的に適用
  - アクセシビリティ向上とUI柔軟性を提供
  - 実装判断: 主要機能完成後に改めて検討

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

## 💰 入出金管理機能 (Transaction Management)

### Phase 1: データベース・バックエンドAPI ✅ (2025-11-08)
- [x] TRANSACTIONSテーブル設計
- [x] インデックス設計
- [x] `get_transactions` API（ページネーション、フィルター、JOIN）
- [x] `delete_transaction` API
- [x] `add_transaction` API
- [x] `update_transaction` API（ヘッダー編集、メモ追加・編集・削除）

### Phase 2: フロントエンド（一覧表示） ✅ (2025-11-05)
- [x] transaction-management.html/css/js
- [x] 管理メニューに追加
- [x] 一覧表示（カテゴリ名表示）
- [x] ページネーション（50件/ページ）
- [x] 削除機能
- [x] フィルター機能 ✅
  - [x] パネル開閉
  - [x] 日付範囲フィルター（27件テスト成功）
  - [x] カテゴリドロップダウン（3階層動的選択）
    - [x] 大分類のみ: 24件
    - [x] 中分類まで: 6件
    - [x] 小分類まで: 3件
  - [x] 金額範囲フィルター（10件テスト成功）
  - [x] キーワード検索（LIKE検索、DESCRIPTION+MEMO対象、3件テスト成功）
  - [x] 複合条件フィルター（20件テスト成功）
- [x] テストデータ31件作成

**完了日**: 2025-11-05 18:12 JST

### Phase 3: 登録・編集機能 ✅ (2025-11-08)
- [x] 新規登録モーダル（取引ヘッダー + 明細一括登録）
- [x] 編集モーダル（ヘッダー編集、メモ管理）
- [x] メモ追加・編集・削除機能（モーダル内）
- [x] テスト実装（60件のJavaScriptテスト）
- [ ] **明細管理ボタン機能**（未実装）
  - 一覧画面の「明細管理」ボタンから別モーダル/画面で複数明細を編集
  - 明細の追加・編集・削除・並び替え
  - 金額自動計算
  - 優先度: 中（将来実装予定）

### Phase 4: UI/UX改善（検討中）
- [ ] MEMO列の一覧表示追加
  - 現状: DESCRIPTIONのみ表示、MEMOは検索対象だが非表示
  - 改善: DESCRIPTION下に表示、長文はツールチップで全文表示
  - 優先度: 中（次回実装予定）
- [ ] ソート機能
- [ ] エクスポート機能

### 低優先度の改善検討
- [ ] 正規表現検索（パワーユーザー向けオプション機能）
  - **実装方針**: アプリケーション層での実装推奨
    - `regex`クレートを使用してRust側でフィルタリング
    - UIに「正規表現モード」チェックボックス追加
    - エラーハンドリング（無効なパターン）実装
    - ヘルプ/使用例の表示
  - **メリット**: 外部依存なし、実装が簡単、デバッグしやすい
  - **デメリット**: 大量データで遅い可能性、DBインデックス使えない
  - **想定データ量**: 年間数千～数万件（アプリ層実装で十分）
  - **優先度**: 低（基本機能完成後に検討）
  - **参考**: SQLiteのREGEXP演算子はデフォルト未実装のため非推奨

### Phase 5: 振替機能の拡張（将来検討）
- [ ] 口座マスタテーブル追加
- [ ] FROM_ACCOUNT_ID, TO_ACCOUNT_ID追加
- [ ] 口座管理画面
- [ ] 口座別集計機能

### UX改善検討項目
- [ ] 日付ピッカーのカスタマイズ（エリア外クリックで閉じる）
  - 現状: HTML標準の`<input type="date">`使用
  - 優先度: 低（リリース時に検討）

---

最終更新: 2025-11-09 20:41 JST
