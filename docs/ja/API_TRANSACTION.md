# 入出金管理 API ドキュメント

## 概要

本ドキュメントは、KakeiBonの入出金管理に関するバックエンドAPIについて説明します。
APIのフロントエンドへの公開は、以下の各項目の説明に準じます。

---

## API一覧

### 入出金ヘッダ取得

#### `get_transaction_header`
入出金ヘッダを1件取得します。メモテキストも含めて返却します。

**パラメータ:**
- `transaction_id` (i64): 入出金ID

**戻り値:**
- `serde_json::Value`: トランザクションヘッダ情報（JSON）

**レスポンス構造:**
```javascript
{
  transaction_id: 123,
  user_id: 2,
  transaction_date: "2024-01-15 10:00:00",
  category1_code: "EXPENSE",
  from_account_code: "CASH",
  to_account_code: "NONE",
  total_amount: 1000,
  tax_rounding_type: 0,
  memo_id: 5,
  is_disabled: 0,
  entry_dt: "2024-01-15 10:00:00",
  update_dt: null,
  memo: "スーパーで食材購入"
}
```

**使用例:**
```javascript
const transaction = await invoke('get_transaction_header', {
  transactionId: 123
});
```

**注記:**
- user_idは現在セッション管理未実装のため、固定で2を使用
- memo_id = NULL の場合、MEMOSテーブルに対応するメモは存在しません
- フロントエンドの入出金一覧では、メモなしの場合は '-'（ハイフン）が表示されます
- transaction_dateはSQLiteのDATETIME形式（YYYY-MM-DD HH:MM:SS）

---

#### `select_transaction_headers`
複数の入出金ヘッダを取得します（将来の一括操作用）。

**パラメータ:**
- `transaction_ids` (Vec<i64>): 入出金IDの配列

**戻り値:**
- `Vec<TransactionHeader>`: トランザクションヘッダの配列

**使用例:**
```javascript
const transactions = await invoke('select_transaction_headers', {
  transactionIds: [1, 2, 3]
});
```

**注記:**
- 存在しないIDは無視されます（エラーになりません）
- メモテキストは含まれません（memo_idのみ）
- 将来の一括編集機能実装時に使用予定

---

### 入出金ヘッダ登録

#### `save_transaction_header`
新しい入出金ヘッダを登録します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `category1_code` (String): 大分類コード（例: "EXPENSE", "INCOME", "TRANSFER"）
- `from_account_code` (String): 出金元口座コード（使用しない場合は "NONE"）
- `to_account_code` (String): 入金先口座コード（使用しない場合は "NONE"）
- `transaction_date` (String): 入出金日時（YYYY-MM-DD HH:MM:SS形式）
- `total_amount` (i64): 合計金額
- `tax_rounding_type` (i64): 税額丸め区分（0=切り捨て、1=切り上げ、2=四捨五入）
- `memo` (Option<String>): メモ（nullまたは空文字列の場合、memo_idはNULL）

**戻り値:**
- `Result<i64, String>`: 成功時は新しいtransaction_id、失敗時はエラーメッセージ

**使用例:**
```javascript
const transactionId = await invoke('save_transaction_header', {
  userId: 2,
  category1Code: 'EXPENSE',
  fromAccountCode: 'CASH',
  toAccountCode: 'NONE',
  transactionDate: '2024-01-15 10:00:00',
  totalAmount: 1000,
  taxRoundingType: 0,
  memo: 'スーパーで食材購入'
});
console.log('Created transaction ID:', transactionId);
```

**バリデーション:**
- 日時形式: YYYY-MM-DD HH:MM:SS（厳密チェック）
- 金額範囲: 0 ≤ total_amount ≤ 999,999,999
- 税丸め区分: 0, 1, 2 のいずれか
- account_code: "NONE" は「指定なし」を意味する特別な口座コード（文字列として保存）

**注記:**
- メモが既に存在する場合は再利用（同じmemo_idを使用）
- メモが新規の場合は自動的にMEMOSテーブルに追加
- transaction_idは自動採番（AUTOINCREMENT）

---

### 入出金ヘッダ更新

#### `update_transaction_header`
既存の入出金ヘッダを更新します。

**パラメータ:**
- `transaction_id` (i64): 更新対象の入出金ID
- `category1_code` (String): 大分類コード
- `from_account_code` (String): 出金元口座コード
- `to_account_code` (String): 入金先口座コード
- `transaction_date` (String): 入出金日時（YYYY-MM-DD HH:MM:SS形式）
- `total_amount` (i64): 合計金額
- `tax_rounding_type` (i64): 税額丸め区分
- `memo` (Option<String>): メモ

**戻り値:**
- `Result<(), String>`: 成功時は空、失敗時はエラーメッセージ

**使用例:**
```javascript
await invoke('update_transaction_header', {
  transactionId: 123,
  category1Code: 'EXPENSE',
  fromAccountCode: 'BANK',
  toAccountCode: 'NONE',
  transactionDate: '2024-01-15 11:00:00',
  totalAmount: 2000,
  taxRoundingType: 0,
  memo: 'スーパーで食材購入（金額訂正）'
});
```

**バリデーション:**
- `save_transaction_header` と同じバリデーションルール適用
- transaction_idが存在しない場合はエラー

**注記:**
- user_idは現在セッション管理未実装のため、固定で2を使用
- UPDATE_DTフィールドが自動更新されます

---

## メモ管理ロジック

### 概要
入出金のメモは共有可能な設計になっており、同じ内容のメモは再利用されます。

### メモの状態

#### 1. 空メモ（メモなし）
```javascript
memo: null  // または空文字列 ""
```
- **動作**: memo_id = NULL
- **MEMOSテーブル**: レコード作成なし
- **フロントエンド表示**: '-'（ハイフン）が表示される（「メモなし」を意味）
- **用途**: メモが不要な入出金

#### 2. 既存メモの再利用
```javascript
memo: "スーパーで買い物"  // 既にMEMOSテーブルに存在
```
- **動作**: 既存のmemo_idを検索して参照
- **MEMOSテーブル**: 変更なし
- **フロントエンド表示**: メモテキストがそのまま表示される
- **用途**: 同じメモを複数の入出金で共有

#### 3. 新規メモ
```javascript
memo: "新しいメモ内容"  // MEMOSテーブルに未存在
```
- **動作**: 新しいmemo_idを自動採番
- **MEMOSテーブル**: 新規レコード作成
- **フロントエンド表示**: 新規メモテキストが表示される
- **用途**: 初めて使用するメモ

### フロントエンドでの表示

**入出金一覧画面でのメモ表示:**
```javascript
// res/js/transaction-management.js:323
if (transaction.memo) {
    memoDiv.textContent = transaction.memo;
} else {
    memoDiv.textContent = '-';  // メモなしの場合はハイフン表示
}
```

### メモ更新時の挙動

#### ケース1: 共有メモの変更
```
元のメモ: "買い物" (memo_id=10)
使用中: Transaction A, Transaction B → memo_id=10

Transaction Aを編集してメモ変更: "食材購入"
結果:
  - 新しいmemo_id=20 が作成される
  - Transaction A → memo_id=20
  - Transaction B → memo_id=10 (変更なし)
```
**理由**: 他のトランザクションに影響を与えないため

#### ケース2: メモの削除
```
元のメモ: "買い物" (memo_id=10)

メモを空に変更: memo = null
結果:
  - memo_id=NULL
  - MEMOSテーブル: memo_id=10 は残る（孤立）
  - フロントエンド表示: '-'（ハイフン）
```
**注記**: 孤立したメモの削除は将来の機能として検討中

#### ケース3: 同じメモ内容に変更
```
元のメモ: "買い物" (memo_id=10)
変更後メモ: "食材購入" (memo_id=15が既存)

結果:
  - 新規作成せず、既存のmemo_id=15を使用
```

### データベース構造

**MEMOSテーブル:**
```sql
CREATE TABLE MEMOS (
    MEMO_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    MEMO_TEXT TEXT NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME
);
```

**TRANSACTION_HEADERSテーブル（メモ部分）:**
```sql
CREATE TABLE TRANSACTION_HEADERS (
    ...
    MEMO_ID INTEGER,  -- MEMOSテーブルへの参照（NULL可）
    ...
    FOREIGN KEY(MEMO_ID) REFERENCES MEMOS(MEMO_ID)
);
```

---

## バリデーション詳細

### 日時形式
**形式**: `YYYY-MM-DD HH:MM:SS`

**有効な例:**
```
2024-01-15 10:00:00  ✅
2024-12-31 23:59:59  ✅
```

**無効な例:**
```
2024-1-15 10:00:00   ❌ (月が1桁)
2024-01-15 10:00     ❌ (秒がない)
2024/01/15 10:00:00  ❌ (区切り文字が違う)
```

**実装:**
```rust
// 正規表現チェック（src/services/transaction.rs:713-717）
if !RE_DATETIME.is_match(&request.transaction_date) {
    return Err(TransactionError::ValidationError(
        "Invalid date format. Use YYYY-MM-DD HH:MM:SS".to_string(),
    ));
}
```

### 金額範囲
**範囲**: 0 ≤ total_amount ≤ 999,999,999

**有効な例:**
```
0           ✅ (ゼロは許可)
1           ✅
1000000     ✅
999999999   ✅ (最大値)
```

**無効な例:**
```
-1          ❌ (マイナスは不可)
1000000000  ❌ (10億以上は不可)
```

**実装:**
```rust
// 金額チェック（src/services/transaction.rs:719-723）
if request.total_amount < 0 || request.total_amount > 999_999_999 {
    return Err(TransactionError::ValidationError(
        "Amount must be between 0 and 999,999,999".to_string(),
    ));
}
```

### 税丸め区分
**有効値**: 0, 1, 2

| 値 | 説明 | 定数名 |
|----|------|--------|
| 0  | 切り捨て | TAX_ROUND_DOWN |
| 1  | 切り上げ | TAX_ROUND_UP |
| 2  | 四捨五入 | TAX_ROUND_HALF_UP |

**実装:**
```rust
// 税丸め区分チェック（src/services/transaction.rs:725-731）
if request.tax_rounding_type != consts::TAX_ROUND_DOWN
    && request.tax_rounding_type != consts::TAX_ROUND_UP
    && request.tax_rounding_type != consts::TAX_ROUND_HALF_UP
{
    return Err(TransactionError::ValidationError(
        "Invalid tax rounding type".to_string(),
    ));
}
```

---

## エラーハンドリング

### エラー型

```rust
pub enum TransactionError {
    DatabaseError(String),     // データベースエラー
    ValidationError(String),   // バリデーションエラー
    NotFound,                  // トランザクション未発見
}
```

### 一般的なエラーケース

#### 1. バリデーションエラー
**原因:**
- 日時形式が不正
- 金額範囲外
- 税丸め区分が不正

**メッセージ例:**
```
"Validation error: Invalid date format. Use YYYY-MM-DD HH:MM:SS"
"Validation error: Amount must be between 0 and 999,999,999"
"Validation error: Invalid tax rounding type"
```

#### 2. トランザクション未発見
**原因:**
- 存在しないtransaction_idを指定

**メッセージ:**
```
"Transaction not found"
```

#### 3. データベースエラー
**原因:**
- 外部キー制約違反
- SQL実行エラー
- 接続エラー

**メッセージ例:**
```
"Database error: FOREIGN KEY constraint failed"
"Database error: no such table: TRANSACTION_HEADERS"
```

### フロントエンドでのエラーハンドリング例

```javascript
try {
  await invoke('update_transaction_header', {
    transactionId: 123,
    // ... parameters
  });
  alert('入出金を更新しました');
  await loadTransactions(); // 一覧再読み込み
} catch (error) {
  console.error('更新失敗:', error);

  if (error.includes('not found')) {
    alert('入出金が見つかりません。削除された可能性があります。');
  } else if (error.includes('Validation error')) {
    alert('入力内容に誤りがあります: ' + error);
  } else {
    alert('入出金の更新に失敗しました: ' + error);
  }
}
```

---

## データベーススキーマ

### TRANSACTION_HEADERS テーブル
```sql
CREATE TABLE TRANSACTION_HEADERS (
    TRANSACTION_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    TRANSACTION_DATE DATETIME NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    FROM_ACCOUNT_CODE VARCHAR(64),
    TO_ACCOUNT_CODE VARCHAR(64),
    TOTAL_AMOUNT INTEGER NOT NULL,
    TAX_ROUNDING_TYPE INTEGER NOT NULL,
    MEMO_ID INTEGER,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY(USER_ID) REFERENCES USERS(USER_ID),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE)
        REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE),
    FOREIGN KEY(MEMO_ID) REFERENCES MEMOS(MEMO_ID)
);
```

### MEMOS テーブル
```sql
CREATE TABLE MEMOS (
    MEMO_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    MEMO_TEXT TEXT NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME
);
```

### TRANSACTION_DETAILS テーブル
```sql
CREATE TABLE TRANSACTION_DETAILS (
    DETAIL_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    TRANSACTION_ID INTEGER NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    ITEM_NAME VARCHAR(256) NOT NULL,
    AMOUNT INTEGER NOT NULL,
    TAX_AMOUNT INTEGER NOT NULL,
    TAX_RATE INTEGER NOT NULL,
    MEMO_ID INTEGER,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    FOREIGN KEY(TRANSACTION_ID)
        REFERENCES TRANSACTION_HEADERS(TRANSACTION_ID)
        ON DELETE CASCADE,
    FOREIGN KEY(MEMO_ID) REFERENCES MEMOS(MEMO_ID)
);
```

**注記:**
- TRANSACTION_DETAILSは将来実装予定（明細機能）
- 現在はヘッダのみの入出金管理

---

## データ構造

### TransactionHeader
```rust
pub struct TransactionHeader {
    pub transaction_id: i64,
    pub user_id: i64,
    pub transaction_date: String,      // YYYY-MM-DD HH:MM:SS
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub total_amount: i64,
    pub tax_rounding_type: i64,
    pub memo_id: Option<i64>,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

### SaveTransactionRequest
```rust
pub struct SaveTransactionRequest {
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub transaction_date: String,
    pub total_amount: i64,
    pub tax_rounding_type: i64,
    pub memo: Option<String>,
}
```

---

## 設計上の注意事項

### 口座コード
- **NONE**: 口座を使用しない場合に指定する特別な口座コード
- **データベース保存**: "NONE" は文字列としてそのまま保存されます（NULLに変換されません）
- **ACCOUNTSテーブル**: ACCOUNT_CODE='NONE', ACCOUNT_NAME='指定なし' のレコードが存在
- **データ型**: from_account_code, to_account_code は String 型
- **カテゴリ別の使用**:
  - EXPENSE（支出）: FROM_ACCOUNT_CODE のみ使用
  - INCOME（収入）: TO_ACCOUNT_CODE のみ使用
  - TRANSFER（振替）: 両方使用

### 税丸め区分
- **現状**: ヘッダレベルで保持
- **将来**: 明細単位での税額計算に使用予定
- **デフォルト値**: 0（切り捨て）

### セッション管理
- **現状**: user_id は固定（2）
- **将来**: セッション/認証実装後に動的取得予定
- **TODO**: コード内に `// TODO: Get user_id from session/auth` コメントあり

### メモの孤立レコード
- **現状**: 削除機能未実装
- **将来**: 定期的なクリーンアップ処理を検討中
- **理由**: メモ共有機能を維持するため、慎重な削除が必要

---

## 実装済み機能

- ✅ 入出金ヘッダの新規登録
- ✅ 入出金ヘッダの取得（1件）
- ✅ 入出金ヘッダの取得（複数件）
- ✅ 入出金ヘッダの更新
- ✅ 入出金ヘッダの削除
- ✅ メモの自動管理（新規/再利用/NULL）
- ✅ バリデーション（日時・金額・税丸め区分）
- ✅ フィルタリング機能（一覧画面）
- ✅ ページング機能（一覧画面）

---

## 今後の実装予定

- [ ] 入出金明細（TRANSACTION_DETAILS）管理
- [ ] セッション管理によるuser_id動的取得
- [ ] メモの孤立レコードクリーンアップ
- [ ] 一括編集機能
- [ ] 入出金の論理削除（IS_DISABLED使用）
- [ ] 税額自動計算（明細単位）
- [ ] 口座残高管理との連携

---

## テスト

### バックエンドテスト (Rust)
```bash
# トランザクションサービステスト
cargo test services::transaction::tests --lib
```

### フロントエンドテスト (JavaScript)
```bash
cd res/tests

# 入出金編集機能テスト
npm test -- transaction-edit.test.js
```

**テスト結果:**
- バックエンド: 121テスト成功
- フロントエンド: 404テスト成功（うち入出金編集: 98テスト）
- 総テスト数: 525テスト
- 成功率: 100%

---

## バージョン履歴

- **v0.2** (2025-11-09): 編集機能の追加
  - update_transaction_header API実装
  - 編集時のメモ管理ロジック
  - テストケース追加（98テスト）

- **v0.1** (2025-11-08): 初版APIドキュメント
  - save_transaction_header API
  - get_transaction_header API
  - select_transaction_headers API
  - メモ自動管理機能

---

## 関連ドキュメント

- [テストサマリー](TEST_SUMMARY.md)
- [費目管理API](API_CATEGORY_ja.md)
- [English Version](../en/API_TRANSACTION.md)

---

**最終更新**: 2025-11-09 15:55 JST
**バージョン**: v0.2 (編集機能完了)
