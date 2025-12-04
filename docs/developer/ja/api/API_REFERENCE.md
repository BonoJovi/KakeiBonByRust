# API リファレンス

**最終更新**: 2025-12-05 00:56 JST

## 概要

本ドキュメントは、KakeiBonの全バックエンドAPIの統合リファレンスです。コード例は最小限に抑え、APIの構造とパラメータに焦点を当てています。

---

## 目次

1. [共通仕様](#共通仕様)
2. [費目管理API](#費目管理api)
3. [入出金管理API](#入出金管理api)
4. [集計API](#集計api)
5. [店舗管理API](#店舗管理api)
6. [メーカー管理API](#メーカー管理api)
7. [商品管理API](#商品管理api)
8. [エラーハンドリング](#エラーハンドリング)

---

## 共通仕様

### 呼び出し方法

すべてのAPIはTauri Commandとして実装されており、フロントエンドから`invoke`で呼び出します。

```javascript
import { invoke } from '@tauri-apps/api/core';

const result = await invoke('command_name', { param1: value1, param2: value2 });
```

### 共通パラメータ

- `user_id` (i64): すべてのAPIで必須。データはユーザーごとに分離されています。

### 共通戻り値

- 成功: `Result<T, String>` の `T` 部分
- エラー: `String` 型のエラーメッセージ

### 論理削除

多くのエンティティで論理削除を採用しています。

- `is_disabled = 0`: 有効
- `is_disabled = 1`: 非表示（論理削除済み）

---

## 費目管理API

### get_category_tree

カテゴリツリー（3階層）を取得します。

**パラメータ:**
- `user_id` (i64): ユーザーID

**戻り値:**
- `Vec<CategoryTree>`: カテゴリツリー配列

---

### get_category_tree_with_lang

多言語名を含むカテゴリツリーを取得します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `lang_code` (Option\<String\>): 言語コード（"ja", "en"など）

**戻り値:**
- `Vec<CategoryTree>`: 多言語名を含むカテゴリツリー

**レスポンス構造:**
- `category1_name_i18n`: I18Nテーブルから取得した翻訳名
- `category2_name_i18n`: 同上
- `category3_name_i18n`: 同上

---

### add_category1 / add_category2 / add_category3

各階層のカテゴリを追加します。

**共通パラメータ:**
- `user_id` (i64)
- `category_name` (String): カテゴリ名
- `display_order` (i64): 表示順
- 上位カテゴリコード（category2/3のみ）

**共通戻り値:**
- `String`: "Category added successfully"

**バリデーション:**
- カテゴリ名は必須
- 同一階層・同一親での重複不可

---

### update_category1 / update_category2 / update_category3

各階層のカテゴリを更新します。

**共通パラメータ:**
- `user_id` (i64)
- カテゴリコード
- `category_name` (String)
- `display_order` (i64)
- `is_disabled` (i64)

**共通戻り値:**
- `String`: "Category updated successfully"

---

### delete_category1 / delete_category2 / delete_category3

カテゴリを論理削除します。

**パラメータ:**
- `user_id` (i64)
- カテゴリコード

**戻り値:**
- `String`: "Category deleted successfully"

**注意:**
- 子カテゴリがある場合は削除不可

---

## 入出金管理API

### get_transaction_header

入出金ヘッダ（1件）を取得します。

**パラメータ:**
- `transaction_id` (i64): 入出金ID

**戻り値:**
- `serde_json::Value`: トランザクションヘッダ情報（メモを含む）

**レスポンスフィールド:**
- `transaction_id`, `user_id`, `transaction_date`
- `category1_code`, `from_account_code`, `to_account_code`
- `total_amount`, `tax_rounding_type`
- `memo_id`, `shop_id`, `is_disabled`
- `entry_dt`, `update_dt`
- `memo`: メモテキスト

---

### get_transaction_headers_by_date_range

日付範囲でヘッダを取得します。

**パラメータ:**
- `user_id` (i64)
- `start_date` (String): "YYYY-MM-DD"
- `end_date` (String): "YYYY-MM-DD"

**戻り値:**
- `Vec<serde_json::Value>`: ヘッダ配列

---

### get_transaction_details

明細を取得します。

**パラメータ:**
- `transaction_id` (i64)

**戻り値:**
- `Vec<TransactionDetail>`: 明細配列

**DetailフィールD:**
- `detail_id`, `transaction_id`, `line_number`
- `category2_code`, `category3_code`
- `item_name`, `quantity`, `unit_price`
- `subtotal`, `tax_rate`, `tax_amount`
- `product_id`, `manufacturer_id`

---

### save_transaction

入出金を新規登録します。

**パラメータ:**
- `user_id` (i64)
- `header` (TransactionHeader): ヘッダ情報
- `details` (Vec\<TransactionDetail\>): 明細配列
- `memo` (Option\<String\>): メモ

**戻り値:**
- `i64`: 作成されたtransaction_id

**トランザクション:**
- ヘッダ・明細・メモの保存をトランザクション内で実行

---

### update_transaction

入出金を更新します。

**パラメータ:**
- `user_id` (i64)
- `transaction_id` (i64)
- `header` (TransactionHeader)
- `details` (Vec\<TransactionDetail\>)
- `memo` (Option\<String\>)

**戻り値:**
- `String`: "Transaction updated successfully"

**動作:**
- 既存の明細を削除し、新しい明細を挿入
- メモも更新

---

### delete_transaction

入出金を論理削除します。

**パラメータ:**
- `user_id` (i64)
- `transaction_id` (i64)

**戻り値:**
- `String`: "Transaction deleted successfully"

**注意:**
- ヘッダのみ論理削除（明細は物理削除しない）

---

## 集計API

### aggregate_by_monthly

月次集計を実行します。

**パラメータ:**
- `user_id` (i64)
- `start_date` (String): "YYYY-MM-DD"
- `end_date` (String): "YYYY-MM-DD"
- `group_by` (GroupBy): グルーピング条件
  - `Category1`, `Category2`, `Category3`
  - `FromAccount`, `ToAccount`, `Shop`
- `category1_code` (Option\<String\>): フィルタ

**戻り値:**
- `Vec<AggregationResult>`: 集計結果

**結果フィールド:**
- `period`: "YYYY-MM"
- `group_value`: グルーピングキー
- `total_amount`: 合計金額
- `transaction_count`: 件数

---

### aggregate_by_daily

日次集計を実行します。

**パラメータ:**
- 月次集計と同じ

**戻り値:**
- `Vec<AggregationResult>`（`period`が"YYYY-MM-DD"形式）

---

### aggregate_by_period

期間集計を実行します（期間全体を1つの集計単位とする）。

**パラメータ:**
- 月次集計と同じ

**戻り値:**
- `Vec<AggregationResult>`（`period`が"YYYY-MM-DD to YYYY-MM-DD"形式）

---

### aggregate_by_weekly

週次集計を実行します。

**パラメータ:**
- 月次集計と同じ

**戻り値:**
- `Vec<AggregationResult>`（`period`が週の開始日"YYYY-MM-DD"形式）

---

### aggregate_by_yearly

年次集計を実行します。

**パラメータ:**
- 月次集計と同じ

**戻り値:**
- `Vec<AggregationResult>`（`period`が"YYYY"形式）

---

### アーキテクチャ

集計APIは3階層設計を採用しています：

1. **コア関数**: 動的SQL生成とクエリ実行
2. **ラッパー関数**: バリデーションとデータベース接続管理
3. **Tauriコマンド**: パラメータ変換とエラーハンドリング

**GroupBy Enum:**
```rust
pub enum GroupBy {
    Category1,
    Category2,
    Category3,
    FromAccount,
    ToAccount,
    Shop,
}
```

---

## 店舗管理API

### get_shops

店舗一覧を取得します。

**パラメータ:**
- `user_id` (i64)

**戻り値:**
- `Vec<Shop>`: 論理削除されていない店舗

**Shopフィールド:**
- `shop_id`, `user_id`, `shop_name`
- `memo`, `display_order`, `is_disabled`
- `entry_dt`, `update_dt`

---

### add_shop

店舗を追加します。

**パラメータ:**
- `user_id` (i64)
- `shop_name` (String): 必須
- `memo` (Option\<String\>)

**戻り値:**
- `String`: "Shop added successfully"

**バリデーション:**
- 店舗名は必須
- 同一ユーザー内で重複不可

**自動設定:**
- `display_order`: 最大値+1
- `is_disabled`: 0

---

### update_shop

店舗を更新します。

**パラメータ:**
- `user_id` (i64)
- `shop_id` (i64)
- `shop_name` (String)
- `memo` (Option\<String\>)
- `display_order` (i64)

**戻り値:**
- `String`: "Shop updated successfully"

---

### delete_shop

店舗を論理削除します。

**パラメータ:**
- `user_id` (i64)
- `shop_id` (i64)

**戻り値:**
- `String`: "Shop deleted successfully"

---

## メーカー管理API

### get_manufacturers

メーカー一覧を取得します。

**パラメータ:**
- `user_id` (i64)
- `include_disabled` (bool): 非表示項目を含むか

**戻り値:**
- `Vec<Manufacturer>`

**Manufacturerフィールド:**
- `manufacturer_id`, `user_id`, `manufacturer_name`
- `memo`, `display_order`, `is_disabled`
- `entry_dt`, `update_dt`

---

### add_manufacturer

メーカーを追加します。

**パラメータ:**
- `user_id` (i64)
- `manufacturer_name` (String)
- `memo` (Option\<String\>)
- `is_disabled` (Option\<i64\>): デフォルト0

**戻り値:**
- `String`: "Manufacturer added successfully"

---

### update_manufacturer

メーカーを更新します。

**パラメータ:**
- `user_id` (i64)
- `manufacturer_id` (i64)
- `manufacturer_name` (String)
- `memo` (Option\<String\>)
- `display_order` (i64)
- `is_disabled` (i64)

**戻り値:**
- `String`: "Manufacturer updated successfully"

---

### delete_manufacturer

メーカーを論理削除します。

**パラメータ:**
- `user_id` (i64)
- `manufacturer_id` (i64)

**戻り値:**
- `String`: "Manufacturer deleted successfully"

**注意:**
- 関連する商品は削除されません
- 商品の`manufacturer_id`は保持されますが、取得時に`manufacturer_name`は`null`になります

---

## 商品管理API

### get_products

商品一覧を取得します（メーカー名を含む）。

**パラメータ:**
- `user_id` (i64)
- `include_disabled` (bool)

**戻り値:**
- `Vec<Product>`

**Productフィールド:**
- `product_id`, `user_id`, `product_name`
- `manufacturer_id`, `manufacturer_name` (LEFT JOINで取得)
- `memo`, `display_order`, `is_disabled`
- `entry_dt`, `update_dt`

**注意:**
- メーカーが非表示の場合、`manufacturer_name`は`null`

---

### add_product

商品を追加します。

**パラメータ:**
- `user_id` (i64)
- `product_name` (String)
- `manufacturer_id` (Option\<i64\>)
- `memo` (Option\<String\>)
- `is_disabled` (Option\<i64\>): デフォルト0

**戻り値:**
- `String`: "Product added successfully"

---

### update_product

商品を更新します。

**パラメータ:**
- `user_id` (i64)
- `product_id` (i64)
- `product_name` (String)
- `manufacturer_id` (Option\<i64\>)
- `memo` (Option\<String\>)
- `display_order` (i64)
- `is_disabled` (i64)

**戻り値:**
- `String`: "Product updated successfully"

---

### delete_product

商品を論理削除します。

**パラメータ:**
- `user_id` (i64)
- `product_id` (i64)

**戻り値:**
- `String`: "Product deleted successfully"

---

## エラーハンドリング

### 一般的なエラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"... cannot be empty"` | 必須項目が空 | 有効な値を入力 |
| `"... already exists"` | 重複データ | 異なる値を使用 |
| `"... not found"` | データが存在しない | 正しいIDを指定 |
| `"Failed to ...: ..."` | データベースエラー | 接続・権限を確認 |

### フロントエンドでのエラーハンドリング

```javascript
try {
  const result = await invoke('command_name', params);
  // 成功処理
} catch (error) {
  console.error('エラー:', error);
  alert(`エラー: ${error}`);
}
```

---

## データベーステーブル一覧

| テーブル名 | 説明 | 主キー |
|----------|------|--------|
| TRANSACTION_HEADERS | 入出金ヘッダ | TRANSACTION_ID |
| TRANSACTION_DETAILS | 入出金明細 | DETAIL_ID |
| TRANSACTION_MEMOS | 入出金メモ | MEMO_ID |
| CATEGORIES1 | 費目大分類 | CATEGORY1_CODE |
| CATEGORIES2 | 費目中分類 | CATEGORY2_CODE |
| CATEGORIES3 | 費目小分類 | CATEGORY3_CODE |
| SHOPS | 店舗 | SHOP_ID |
| MANUFACTURERS | メーカー | MANUFACTURER_ID |
| PRODUCTS | 商品 | PRODUCT_ID |
| ACCOUNTS | 口座 | ACCOUNT_CODE |

---

## テストカバレッジ

**バックエンド（Rust）:**
- 201+ テスト実装済み
- すべての主要APIをカバー

**フロントエンド（JavaScript）:**
- 手動テスト実施済み
- 自動テストは今後実装予定

---

## 関連ドキュメント

### 詳細仕様（旧ドキュメント）

以下の個別APIドキュメントには、より詳細なコード例とSQL文が記載されています：

- [費目管理API詳細](./API_CATEGORY.md)
- [入出金管理API詳細](./API_TRANSACTION.md)
- [集計API詳細](./API_AGGREGATION.md)
- [店舗管理API詳細](./API_SHOP.md)
- [メーカー管理API詳細](./API_MANUFACTURER.md)
- [商品管理API詳細](./API_PRODUCT.md)

### 実装ガイド

- [IS_DISABLED実装ガイド](../guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md)
- [データベース設定ガイド](../guides/DATABASE_CONFIGURATION.md)

---

**変更履歴:**
- 2025-12-05: 初版作成（6つのAPIドキュメントを統合）
