# 入出金管理機能 要件定義書

## 概要 / Overview

**入出金データの記録・編集・削除・検索機能を実装する**  
Implement functionality to record, edit, delete, and search transaction data

**最終更新 / Last Updated**: 2025-11-05 22:44 JST

**実装状況 / Implementation Status**:
- ✅ Phase 1: データベース・バックエンドAPI実装完了
- ✅ Phase 2: フロントエンド（一覧表示）実装完了
- ✅ フィルター機能実装完了（日付範囲、カテゴリ、金額範囲、キーワード）
- ✅ ページネーション実装完了（50件/ページ）
- ✅ 削除機能実装完了
- ⏳ Phase 3: 登録・編集機能は未実装

---

## 1. 基本機能 / Basic Features

### 1.1 入出金データ登録 / Transaction Registration

#### 入力項目 / Input Fields

| 項目 / Field | 必須 / Required | 型 / Type | 説明 / Description |
|-------------|----------------|-----------|-------------------|
| **取引日** / Transaction Date | ✅ Yes | DATE | 入出金が発生した日付 / Date of transaction |
| **大分類** / Major Category | ✅ Yes | SELECT | EXPENSE/INCOME/TRANSFER |
| **中分類** / Middle Category | ✅ Yes | SELECT | 大分類に紐づく中分類 / Middle category linked to major |
| **小分類** / Minor Category | ✅ Yes | SELECT | 中分類に紐づく小分類 / Minor category linked to middle |
| **金額** / Amount | ✅ Yes | INTEGER | 正の整数（1円単位） / Positive integer (1 yen unit) |
| **摘要** / Description | ❌ No | TEXT | 取引の説明（最大500文字） / Transaction description (max 500 chars) |
| **メモ** / Memo | ❌ No | TEXT | 自由記述（最大1000文字） / Free text (max 1000 chars) |

#### バリデーションルール / Validation Rules

1. **取引日**:
   - 過去の日付OK / Past dates allowed
   - 未来の日付NG / Future dates not allowed
   - 範囲: 1900-01-01 ～ 本日 / Range: 1900-01-01 to today

2. **金額**:
   - 最小値: 1円 / Minimum: 1 yen
   - 最大値: 999,999,999円（9億円未満） / Maximum: 999,999,999 yen
   - 整数のみ / Integer only
   - カンマ区切り表示対応 / Display with comma separators

3. **摘要・メモ**:
   - 任意入力 / Optional
   - 文字数制限あり / Character limit enforced
   - 空白のみは不可 / Whitespace-only not allowed

#### デフォルト値 / Default Values

- 取引日: 本日 / Transaction date: Today
- 大分類: EXPENSE（支出） / Major category: EXPENSE
- 中分類: 最初の項目 / Middle category: First item
- 小分類: 最初の項目 / Minor category: First item
- 金額: 空欄 / Amount: Empty

---

### 1.2 入出金データ一覧表示 / Transaction List Display

#### 表示項目 / Display Columns

| 列 / Column | 説明 / Description |
|------------|-------------------|
| 取引日 / Date | YYYY-MM-DD形式 |
| 大分類 / Major | EXPENSE/INCOME/TRANSFER |
| 中分類 / Middle | 中分類名（多言語対応） |
| 小分類 / Minor | 小分類名（多言語対応） |
| 金額 / Amount | カンマ区切り表示 |
| 摘要 / Description | 省略表示（最大30文字） |
| 操作 / Actions | 編集・削除ボタン |

#### ソート機能 / Sort Features

- **デフォルト**: 取引日降順（新しい順） / Default: Date descending (newest first)
- **選択可能**: 取引日昇順/降順、金額昇順/降順 / Selectable: Date/amount asc/desc

#### ページネーション / Pagination

- 1ページあたり: 50件 / Per page: 50 items
- ページャー表示 / Pager display
- 総件数表示 / Total count display

#### フィルター機能 / Filter Features

1. **期間指定** / Date Range:
   - 開始日・終了日 / Start date - End date
   - プリセット: 今月/先月/今年/昨年 / Presets: This month/Last month/This year/Last year

2. **カテゴリ指定** / Category Filter:
   - 大分類で絞り込み / Filter by major category
   - 中分類で絞り込み / Filter by middle category
   - 小分類で絞り込み / Filter by minor category

3. **金額範囲** / Amount Range:
   - 最小金額・最大金額 / Min amount - Max amount

4. **キーワード検索** / Keyword Search:
   - 摘要・メモを対象 / Search in description and memo

---

### 1.3 入出金データ編集 / Transaction Edit

#### 編集方法 / Edit Method

- 一覧画面の「編集」ボタンから / From "Edit" button in list
- モーダルダイアログで編集 / Edit in modal dialog
- 全項目編集可能 / All fields editable

#### 制約 / Constraints

- 登録者以外は編集不可（将来的に権限管理を追加） / Only creator can edit (permission management in future)
- 編集時にUPDATE_DTを更新 / Update UPDATE_DT on edit

---

### 1.4 入出金データ削除 / Transaction Delete

#### 削除方法 / Delete Method

- 一覧画面の「削除」ボタンから / From "Delete" button in list
- 確認ダイアログを表示 / Show confirmation dialog
- 論理削除ではなく物理削除 / Physical delete (not soft delete)

#### 確認ダイアログ / Confirmation Dialog

```
本当に削除しますか？この操作は取り消せません。
Are you sure you want to delete? This action cannot be undone.

取引日: 2025-11-05
金額: ¥1,000
摘要: コンビニ

[キャンセル / Cancel] [削除 / Delete]
```

---

## 2. データベース設計 / Database Design

### 2.1 TRANSACTIONSテーブル

```sql
CREATE TABLE IF NOT EXISTS TRANSACTIONS (
    TRANSACTION_ID INTEGER NOT NULL,
    USER_ID INTEGER NOT NULL,
    TRANSACTION_DATE DATE NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    AMOUNT INTEGER NOT NULL,
    DESCRIPTION VARCHAR(500),
    MEMO TEXT,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(TRANSACTION_ID),
    FOREIGN KEY(USER_ID) REFERENCES USERS(USER_ID) ON DELETE CASCADE,
    FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE) 
        REFERENCES CATEGORY3(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE)
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_transactions_user ON TRANSACTIONS(USER_ID);
CREATE INDEX IF NOT EXISTS idx_transactions_date ON TRANSACTIONS(USER_ID, TRANSACTION_DATE DESC);
CREATE INDEX IF NOT EXISTS idx_transactions_category1 ON TRANSACTIONS(USER_ID, CATEGORY1_CODE);
CREATE INDEX IF NOT EXISTS idx_transactions_category2 ON TRANSACTIONS(USER_ID, CATEGORY2_CODE);
CREATE INDEX IF NOT EXISTS idx_transactions_amount ON TRANSACTIONS(USER_ID, AMOUNT);
```

### 2.2 フィールド説明 / Field Descriptions

| フィールド | 型 | NULL | 説明 |
|-----------|---|------|------|
| TRANSACTION_ID | INTEGER | NOT NULL | 取引ID（自動採番） |
| USER_ID | INTEGER | NOT NULL | ユーザーID |
| TRANSACTION_DATE | DATE | NOT NULL | 取引日 |
| CATEGORY1_CODE | VARCHAR(64) | NOT NULL | 大分類コード |
| CATEGORY2_CODE | VARCHAR(64) | NOT NULL | 中分類コード |
| CATEGORY3_CODE | VARCHAR(64) | NOT NULL | 小分類コード |
| AMOUNT | INTEGER | NOT NULL | 金額（1円単位） |
| DESCRIPTION | VARCHAR(500) | NULL | 摘要 |
| MEMO | TEXT | NULL | メモ |
| ENTRY_DT | DATETIME | NOT NULL | 登録日時 |
| UPDATE_DT | DATETIME | NULL | 更新日時 |

---

## 3. API設計 / API Design

### 3.1 バックエンドAPI（Rust）

#### 入出金データ登録
```rust
#[tauri::command]
async fn add_transaction(
    user_id: i64,
    transaction_date: String,  // YYYY-MM-DD
    category1_code: String,
    category2_code: String,
    category3_code: String,
    amount: i64,
    description: Option<String>,
    memo: Option<String>
) -> Result<i64, String>
```

#### 入出金データ一覧取得
```rust
#[tauri::command]
async fn get_transactions(
    user_id: i64,
    start_date: Option<String>,
    end_date: Option<String>,
    category1_code: Option<String>,
    category2_code: Option<String>,
    category3_code: Option<String>,
    min_amount: Option<i64>,
    max_amount: Option<i64>,
    keyword: Option<String>,
    page: i64,
    per_page: i64
) -> Result<TransactionListResponse, String>
```

#### 入出金データ取得（単一）
```rust
#[tauri::command]
async fn get_transaction(
    user_id: i64,
    transaction_id: i64
) -> Result<Transaction, String>
```

#### 入出金データ更新
```rust
#[tauri::command]
async fn update_transaction(
    user_id: i64,
    transaction_id: i64,
    transaction_date: String,
    category1_code: String,
    category2_code: String,
    category3_code: String,
    amount: i64,
    description: Option<String>,
    memo: Option<String>
) -> Result<(), String>
```

#### 入出金データ削除
```rust
#[tauri::command]
async fn delete_transaction(
    user_id: i64,
    transaction_id: i64
) -> Result<(), String>
```

---

## 4. UI/UX設計 / UI/UX Design

### 4.1 画面構成 / Screen Layout

#### トップメニューから遷移
```
[ユーザー管理] [費目管理] [入出金管理] ← 新規追加
```

#### 入出金管理画面
```
+--------------------------------------------------+
| 入出金管理 / Transaction Management              |
+--------------------------------------------------+
| [+ 新規登録] [フィルター]                        |
|                                                  |
| 期間: [2025-11-01] ～ [2025-11-30]              |
| カテゴリ: [すべて ▼]                             |
|                                                  |
| 取引日    | 分類 | 金額      | 摘要     | 操作  |
|-----------|------|-----------|----------|-------|
| 2025-11-05| 食費 | ¥1,500   | スーパー | [編集][削除] |
| 2025-11-04| 交通 | ¥500     | バス代   | [編集][削除] |
| ...                                              |
|                                                  |
| ページ: [<] 1 / 10 [>]   合計: 500件            |
+--------------------------------------------------+
```

### 4.2 アクセシビリティ / Accessibility

- フォントサイズ調整対応 / Font size adjustment support
- キーボードナビゲーション / Keyboard navigation
- 日付入力: カレンダーピッカー / Date input: Calendar picker
- 金額入力: カンマ自動挿入 / Amount input: Auto comma insertion

---

## 5. 実装フェーズ / Implementation Phases

### Phase 1: データベース・バックエンドAPI ✅ 完了
- ✅ TRANSACTIONSテーブル作成
- ✅ インデックス作成（USER_ID, TRANSACTION_DATE, CATEGORY1/2, AMOUNT）
- ✅ バックエンドAPI実装（全5メソッド）
  - ✅ `add_transaction` - 新規登録
  - ✅ `get_transaction` - 単一取得
  - ✅ `get_transactions` - 一覧取得（フィルター、ページネーション対応）
  - ✅ `update_transaction` - 更新
  - ✅ `delete_transaction` - 削除
- ✅ バリデーション実装（金額範囲、日付形式、文字数制限）
- ✅ ユニットテスト作成（Rust側）

**完了日**: 2025-11-05

### Phase 2: フロントエンド（一覧表示） ✅ 完了
- ✅ transaction-management.html/css/js 作成
- ✅ 管理メニューに追加
- ✅ 一覧画面実装（テーブル表示、カテゴリ名表示）
- ✅ フィルター機能実装
  - ✅ 日付範囲フィルター（開始日・終了日）
  - ✅ カテゴリフィルター（3階層動的選択）
  - ✅ 金額範囲フィルター（最小・最大）
  - ✅ キーワード検索（DESCRIPTION + MEMO対象）
  - ✅ 複合条件対応
- ✅ ページネーション実装（50件/ページ）（テスト未実施）
- ✅ 削除機能実装（確認ダイアログ付き）
- ✅ テストデータ31件作成（sql/test_data_transactions.sql）

**完了日**: 2025-11-05 18:12 JST

**動作確認結果**（手動テスト）:
- 日付範囲フィルター: 27件表示成功
- カテゴリフィルター（大分類のみ）: 24件表示成功
- カテゴリフィルター（中分類まで）: 6件表示成功
- カテゴリフィルター（小分類まで）: 3件表示成功
- 金額範囲フィルター: 10件表示成功
- キーワード検索: 3件表示成功
- 複合条件フィルター: 20件表示成功

**注記**: フロントエンド自動テストは未実施（手動動作確認のみ）

### Phase 3: フロントエンド（登録・編集） ⏳ 未実装
- [ ] 新規登録モーダル実装
  - [ ] 日付入力（デフォルト: 今日）
  - [ ] カテゴリ選択（3階層連動）
  - [ ] 金額入力（カンマ区切り表示）
  - [ ] 摘要・メモ入力
- [ ] 編集モーダル実装
  - [ ] 既存データ読み込み
  - [ ] 更新処理
- [ ] バリデーションエラー表示

**優先度**: 高（次回実装予定）

### Phase 4: テスト・ドキュメント ⏳ 部分実装
- ✅ Rust バックエンドテスト（Cargo test）
- [ ] JavaScript フロントエンド自動テスト（未実装）
- [ ] エンドツーエンドテスト（未実装）
- [ ] ユーザーガイド作成（未実装）
- [ ] 実装ドキュメント作成（日英、次回作成予定）
- [ ] API ドキュメント作成（Rustdoc、後回し）

**注記**: フロントエンド機能は手動テストのみ実施済み。自動テストは今後の課題。

---

## 6. UI/UX改善検討項目 / UI/UX Improvement Considerations

### 実装検討中 / Under Consideration

1. **MEMO列の一覧表示追加**
   - 現状: DESCRIPTIONのみ表示、MEMOは検索対象だが非表示
   - 改善案: DESCRIPTION下に表示、長文はツールチップで全文表示
   - 優先度: 中（次回実装予定）

2. **ソート機能**
   - 取引日昇順/降順
   - 金額昇順/降順
   - 優先度: 中

3. **正規表現検索（パワーユーザー向け）**
   - 実装方針: `regex`クレート使用、アプリケーション層で実装
   - UIに「正規表現モード」チェックボックス追加
   - 優先度: 低（基本機能完成後に検討）

4. **日付ピッカーのカスタマイズ**
   - 現状: HTML標準の`<input type="date">`使用
   - 改善案: エリア外クリックで閉じる動作
   - 優先度: 低（リリース時に検討）

---

## 7. 将来の機能拡張 / Future Enhancements

### 基本機能の拡張
- [ ] 定期的な取引の登録（月次固定費など）
- [ ] 取引の複製機能
- [ ] 一括インポート（CSV）
- [ ] エクスポート機能（CSV, Excel）

### 高度な機能
- [ ] 添付ファイル機能（レシート画像など）
- [ ] タグ機能
- [ ] 予算との比較表示
- [ ] 月次集計・レポート機能
- [ ] グラフ表示機能

### 振替機能の拡張（将来検討）
- [ ] 口座マスタテーブル追加
- [ ] FROM_ACCOUNT_ID, TO_ACCOUNT_ID カラム追加
- [ ] 口座管理画面
- [ ] 口座別集計機能

---

## 8. 参考 / References

### ドキュメント
- [費目管理API（日本語）](./ja/API_CATEGORY_ja.md)
- [費目管理API（英語）](./en/API_CATEGORY.md)
- [ユーザー管理（日本語）](./ja/USER_MANAGEMENT.md)
- [ユーザー管理（英語）](./en/USER_MANAGEMENT.md)

### ソースコード
- データベーススキーマ: `res/sql/dbaccess.sql`
- トランザクションサービス: `src/services/transaction.rs`
- トランザクション画面: `res/transaction-management.html/css/js`
- テストデータ: `sql/test_data_transactions.sql`

### TODO
- [開発タスク管理](../TODO.md)

---

**作成日 / Created**: 2025-11-05  
**最終更新 / Last Updated**: 2025-11-05 22:44 JST  
**作成者 / Author**: Development Team
