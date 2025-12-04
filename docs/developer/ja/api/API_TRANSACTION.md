# 入出金管理API リファレンス

**最終更新**: 2025-12-05 02:00 JST

## 概要

本ドキュメントは、入出金管理画面（transaction-management.html、transaction-detail-management.html）で使用されるAPIの仕様を定義します。

---

## 目次

1. [トランザクション基本操作API](#トランザクション基本操作api)
2. [トランザクションヘッダ管理API](#トランザクションヘッダ管理api)
3. [トランザクション明細管理API](#トランザクション明細管理api)
4. [データ構造](#データ構造)

---

## トランザクション基本操作API

### add_transaction

シンプルな入出金を追加します（簡易版）。

**パラメータ:**
- `transaction_date` (String): 入出金日時（"YYYY-MM-DD HH:MM:SS"）
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード
- `category3_code` (String): 小分類コード
- `amount` (i64): 金額
- `description` (Option<String>): 説明
- `memo` (Option<String>): メモ

**戻り値:**
- `i64`: 作成されたtransaction_id

**使用例:**
```javascript
const transactionId = await invoke('add_transaction', {
    transactionDate: '2025-12-05 10:30:00',
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    amount: 1500,
    description: '食料品',
    memo: 'スーパーで購入'
});
```

**注意:**
- セッションユーザーIDを自動取得
- 簡易入力用（明細なし）

---

### get_transaction

トランザクション情報を取得します。

**パラメータ:**
- `transaction_id` (i64): トランザクションID

**戻り値:**
- `Transaction`: トランザクション情報

**使用例:**
```javascript
const transaction = await invoke('get_transaction', {
    transactionId: 123
});
```

---

### get_transactions

条件指定でトランザクション一覧を取得します（ページネーション対応）。

**パラメータ:**
- `start_date` (Option<String>): 開始日（"YYYY-MM-DD"）
- `end_date` (Option<String>): 終了日（"YYYY-MM-DD"）
- `category1_code` (Option<String>): 大分類フィルタ
- `category2_code` (Option<String>): 中分類フィルタ
- `category3_code` (Option<String>): 小分類フィルタ
- `min_amount` (Option<i64>): 最小金額
- `max_amount` (Option<i64>): 最大金額
- `keyword` (Option<String>): キーワード検索
- `page` (i64): ページ番号（1から開始）
- `per_page` (i64): 1ページあたりの件数

**戻り値:**
- `Vec<Transaction>`: トランザクション配列

**使用例:**
```javascript
const transactions = await invoke('get_transactions', {
    startDate: '2025-12-01',
    endDate: '2025-12-31',
    category1Code: 'EXPENSE',
    category2Code: null,
    category3Code: null,
    minAmount: null,
    maxAmount: null,
    keyword: null,
    page: 1,
    perPage: 50
});
```

**ページネーション:**
- `page`: 1から開始
- `per_page`: 1～100の範囲推奨

---

### update_transaction

トランザクションを更新します。

**パラメータ:**
- `transaction_id` (i64): トランザクションID
- `transaction_date` (String): 入出金日時
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード
- `category3_code` (String): 小分類コード
- `amount` (i64): 金額
- `description` (Option<String>): 説明
- `memo` (Option<String>): メモ

**戻り値:** なし

**使用例:**
```javascript
await invoke('update_transaction', {
    transactionId: 123,
    transactionDate: '2025-12-05 11:00:00',
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    amount: 2000,
    description: '食料品（更新）',
    memo: null
});
```

---

### delete_transaction

トランザクションを削除します。

**パラメータ:**
- `transaction_id` (i64): トランザクションID

**戻り値:** なし

**使用例:**
```javascript
if (confirm('削除してもよろしいですか？')) {
    await invoke('delete_transaction', { transactionId: 123 });
}
```

**注意:**
- 論理削除（is_disabled = 1）
- 関連する明細も削除される

---

## トランザクションヘッダ管理API

### save_transaction_header

新しいトランザクションヘッダを保存します（明細管理用）。

**パラメータ:**
- `shop_id` (Option<i64>): 店舗ID
- `category1_code` (String): 大分類コード
- `from_account_code` (String): 出金元口座コード
- `to_account_code` (String): 入金先口座コード
- `transaction_date` (String): 入出金日時
- `total_amount` (i64): 合計金額
- `tax_rounding_type` (i64): 税額丸め区分（0=切り捨て、1=切り上げ、2=四捨五入）
- `tax_included_type` (i64): 税込区分（0=税抜、1=税込）
- `memo` (Option<String>): メモ

**戻り値:**
- `i64`: 作成されたtransaction_id

**使用例:**
```javascript
const transactionId = await invoke('save_transaction_header', {
    shopId: 1,
    category1Code: 'EXPENSE',
    fromAccountCode: 'CASH',
    toAccountCode: 'NONE',
    transactionDate: '2025-12-05 10:00:00',
    totalAmount: 5000,
    taxRoundingType: 0,
    taxIncludedType: 1,
    memo: 'スーパーで買い物'
});
```

**注意:**
- 明細は別途`add_transaction_detail`で追加
- メモが暗号化される

---

### get_transaction_header

トランザクションヘッダとメモを取得します。

**パラメータ:**
- `transaction_id` (i64): トランザクションID

**戻り値:**
- `JSON`: ヘッダ情報とメモ

**レスポンス構造:**
```javascript
{
    transaction_id: 123,
    user_id: 2,
    shop_id: 1,
    transaction_date: "2025-12-05 10:00:00",
    category1_code: "EXPENSE",
    from_account_code: "CASH",
    to_account_code: "NONE",
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo_id: 5,
    is_disabled: 0,
    entry_dt: "2025-12-05 10:00:00",
    update_dt: null,
    memo: "スーパーで買い物"
}
```

**使用例:**
```javascript
const header = await invoke('get_transaction_header', {
    transactionId: 123
});
console.log(header.memo);
```

---

### get_transaction_header_with_info

トランザクションヘッダに関連情報（店舗名等）を含めて取得します。

**パラメータ:**
- `transaction_id` (i64): トランザクションID

**戻り値:**
- `TransactionHeaderWithInfo`: 拡張ヘッダ情報

**TransactionHeaderWithInfo構造:**
```javascript
{
    transaction_id: 123,
    user_id: 2,
    shop_id: 1,
    shop_name: "イオン新宿店",  // 追加情報
    transaction_date: "2025-12-05 10:00:00",
    category1_code: "EXPENSE",
    from_account_code: "CASH",
    from_account_name: "現金",  // 追加情報
    to_account_code: "NONE",
    to_account_name: "-",  // 追加情報
    total_amount: 5000,
    tax_rounding_type: 0,
    tax_included_type: 1,
    memo: "スーパーで買い物",
    is_disabled: 0
}
```

**使用例:**
```javascript
const header = await invoke('get_transaction_header_with_info', {
    transactionId: 123
});
console.log(`店舗: ${header.shop_name}`);
```

**用途:**
- 画面表示用（店舗名・口座名を含む）

---

### select_transaction_headers

複数のトランザクションヘッダを一括取得します。

**パラメータ:**
- `transaction_ids` (Vec<i64>): トランザクションIDの配列

**戻り値:**
- `Vec<TransactionHeader>`: ヘッダ配列

**使用例:**
```javascript
const headers = await invoke('select_transaction_headers', {
    transactionIds: [1, 2, 3, 5]
});
```

**注意:**
- 存在しないIDはスキップ（エラーにならない）
- 将来の一括操作用

---

### update_transaction_header

トランザクションヘッダを更新します。

**パラメータ:**
- `transaction_id` (i64): トランザクションID
- 他は`save_transaction_header`と同じ

**戻り値:** なし

**使用例:**
```javascript
await invoke('update_transaction_header', {
    transactionId: 123,
    shopId: 2,
    category1Code: 'EXPENSE',
    fromAccountCode: 'CASH',
    toAccountCode: 'NONE',
    transactionDate: '2025-12-05 11:00:00',
    totalAmount: 6000,
    taxRoundingType: 0,
    taxIncludedType: 1,
    memo: '更新後のメモ'
});
```

---

## トランザクション明細管理API

### get_transaction_details

トランザクションの明細一覧を取得します。

**パラメータ:**
- `transaction_id` (i64): トランザクションID

**戻り値:**
- `Vec<TransactionDetailWithInfo>`: 明細配列（カテゴリ名等を含む）

**TransactionDetailWithInfo構造:**
```javascript
{
    detail_id: 1,
    transaction_id: 123,
    line_number: 1,
    category1_code: "EXPENSE",
    category1_name: "支出",
    category2_code: "C2_E_1",
    category2_name: "食費",
    category3_code: "C3_1",
    category3_name: "食料品",
    item_name: "野菜",
    quantity: 1,
    amount: 500,
    tax_rate: 10,
    tax_amount: 50,
    amount_including_tax: 550,
    product_id: null,
    manufacturer_id: null,
    memo: null
}
```

**使用例:**
```javascript
const details = await invoke('get_transaction_details', {
    transactionId: 123
});
details.forEach(detail => {
    console.log(`${detail.item_name}: ${detail.amount_including_tax}円`);
});
```

---

### add_transaction_detail

トランザクション明細を追加します。

**パラメータ:**
- `transaction_id` (i64): トランザクションID
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード
- `category3_code` (String): 小分類コード
- `item_name` (String): 商品名
- `amount` (i64): 金額（税抜）
- `tax_rate` (i32): 税率（%）
- `tax_amount` (i64): 税額
- `amount_including_tax` (Option<i64>): 税込金額
- `memo` (Option<String>): メモ
- `quantity` (Option<i64>): 数量
- `product_id` (Option<i64>): 商品ID
- `manufacturer_id` (Option<i64>): メーカーID

**戻り値:**
- `i64`: 作成されたdetail_id

**使用例:**
```javascript
const detailId = await invoke('add_transaction_detail', {
    transactionId: 123,
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    itemName: '野菜',
    amount: 500,
    taxRate: 10,
    taxAmount: 50,
    amountIncludingTax: 550,
    memo: null,
    quantity: 1,
    productId: null,
    manufacturerId: null
});
```

**自動処理:**
- `line_number`の自動採番

---

### update_transaction_detail

トランザクション明細を更新します。

**パラメータ:**
- `detail_id` (i64): 明細ID
- 他は`add_transaction_detail`と同じ（transaction_idを除く）

**戻り値:** なし

**使用例:**
```javascript
await invoke('update_transaction_detail', {
    detailId: 1,
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    itemName: '野菜（更新）',
    amount: 600,
    taxRate: 10,
    taxAmount: 60,
    amountIncludingTax: 660,
    memo: null,
    quantity: 1,
    productId: null,
    manufacturerId: null
});
```

---

### delete_transaction_detail

トランザクション明細を削除します。

**パラメータ:**
- `detail_id` (i64): 明細ID

**戻り値:** なし

**使用例:**
```javascript
await invoke('delete_transaction_detail', { detailId: 1 });
```

---

## データ構造

### Transaction（簡易版）

```rust
pub struct Transaction {
    pub transaction_id: i64,
    pub user_id: i64,
    pub transaction_date: String,
    pub category1_code: String,
    pub category2_code: String,
    pub category3_code: String,
    pub amount: i64,
    pub description: Option<String>,
    pub memo: Option<String>,
}
```

---

### TransactionHeader

```rust
pub struct TransactionHeader {
    pub transaction_id: i64,
    pub user_id: i64,
    pub shop_id: Option<i64>,
    pub transaction_date: String,
    pub category1_code: String,
    pub from_account_code: String,
    pub to_account_code: String,
    pub total_amount: i64,
    pub tax_rounding_type: i64,  // 0=切捨, 1=切上, 2=四捨五入
    pub tax_included_type: i64,  // 0=税抜, 1=税込
    pub memo_id: Option<i64>,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

---

### TransactionDetail

```rust
pub struct TransactionDetail {
    pub detail_id: i64,
    pub transaction_id: i64,
    pub line_number: i64,
    pub category1_code: String,
    pub category2_code: String,
    pub category3_code: String,
    pub item_name: String,
    pub quantity: i64,
    pub amount: i64,           // 税抜
    pub tax_rate: i32,         // %
    pub tax_amount: i64,
    pub amount_including_tax: Option<i64>,  // 税込
    pub product_id: Option<i64>,
    pub manufacturer_id: Option<i64>,
    pub memo: Option<String>,
}
```

---

## エラーハンドリング

### 共通エラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"User not authenticated"` | セッション未認証 | ログインが必要 |
| `"Failed to get transaction: ..."` | 取得エラー | データベース確認 |
| `"Failed to delete transaction: ..."` | 削除エラー | データベース確認 |

### フロントエンドでのエラーハンドリング例

```javascript
// トランザクション追加
async function addTransaction(data) {
    try {
        const transactionId = await invoke('add_transaction', data);
        alert(`トランザクションを追加しました（ID: ${transactionId}）`);
        return transactionId;
    } catch (error) {
        alert(`エラー: ${error}`);
        return null;
    }
}

// 明細付きトランザクション保存
async function saveTransactionWithDetails(header, details) {
    try {
        // ヘッダ保存
        const transactionId = await invoke('save_transaction_header', header);
        
        // 明細を順次追加
        for (const detail of details) {
            await invoke('add_transaction_detail', {
                transactionId,
                ...detail
            });
        }
        
        alert('保存しました');
        return transactionId;
    } catch (error) {
        alert(`保存エラー: ${error}`);
        return null;
    }
}
```

---

## 使用シナリオ

### シナリオ1: 簡易入力（明細なし）

```javascript
// 食費1,500円を記録
await invoke('add_transaction', {
    transactionDate: '2025-12-05 10:30:00',
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',  // 食費
    category3Code: 'C3_1',    // 食料品
    amount: 1500,
    description: '昼食',
    memo: null
});
```

### シナリオ2: 明細付き入力

```javascript
// 1. ヘッダ作成
const transactionId = await invoke('save_transaction_header', {
    shopId: 1,
    category1Code: 'EXPENSE',
    fromAccountCode: 'CASH',
    toAccountCode: 'NONE',
    transactionDate: '2025-12-05 14:00:00',
    totalAmount: 3250,
    taxRoundingType: 0,
    taxIncludedType: 1,
    memo: 'スーパーで買い物'
});

// 2. 明細追加（野菜）
await invoke('add_transaction_detail', {
    transactionId,
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    itemName: '野菜',
    amount: 1500,
    taxRate: 8,
    taxAmount: 120,
    amountIncludingTax: 1620,
    memo: null,
    quantity: 1,
    productId: null,
    manufacturerId: null
});

// 3. 明細追加（肉）
await invoke('add_transaction_detail', {
    transactionId,
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    itemName: '肉',
    amount: 1500,
    taxRate: 8,
    taxAmount: 120,
    amountIncludingTax: 1620,
    memo: null,
    quantity: 1,
    productId: null,
    manufacturerId: null
});
```

---

## テストカバレッジ

**TransactionService:**
- ✅ トランザクション追加テスト
- ✅ トランザクション取得テスト
- ✅ トランザクション更新テスト
- ✅ トランザクション削除テスト
- ✅ ヘッダ・明細の統合テスト
- ✅ メモの暗号化・復号化テスト
- ✅ ページネーションテスト

---

## 関連ドキュメント

### 実装ファイル

- トランザクションサービス: `src/services/transaction.rs`
- SQL定義: `src/sql_queries.rs`
- Tauri Commands: `src/lib.rs`

### その他のAPIリファレンス

- [共通API](./API_COMMON.md) - セッション管理
- [費目管理API](./API_CATEGORY.md) - カテゴリ情報
- [口座管理API](./API_ACCOUNT.md) - 口座情報
- [店舗管理API](./API_MASTER_DATA.md) - 店舗情報

---

**変更履歴:**
- 2024-11-10: 初版作成
- 2025-12-05: 実装コードに基づいて全面改訂
  - get_transaction_header_with_infoを追加
  - パラメータ名をcamelCaseに統一
  - 新しいテンプレートに統一
  - 使用シナリオを追加
