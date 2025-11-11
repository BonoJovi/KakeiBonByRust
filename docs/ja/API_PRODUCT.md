# 商品管理 API仕様

**Last Updated**: 2025-11-12 01:31 JST

## 概要

商品管理機能のTauri Command APIの仕様を定義します。

---

## API一覧

| API名 | 説明 |
|-------|------|
| `get_products` | 商品一覧取得 |
| `add_product` | 商品追加 |
| `update_product` | 商品更新 |
| `delete_product` | 商品削除（論理削除） |

---

## get_products

### 概要

指定されたユーザーの商品一覧を取得します。メーカー名も結合して返します。

### パラメータ

| パラメータ名 | 型 | 必須 | 説明 |
|-------------|---|------|------|
| `user_id` | `i64` | ✅ | ユーザーID |
| `include_disabled` | `bool` | ✅ | 非表示項目を含めるか（true: 含める、false: 除外） |

### 戻り値

**成功時**: `Result<Vec<Product>, String>`

```rust
pub struct Product {
    pub product_id: i64,
    pub user_id: i64,
    pub product_name: String,
    pub manufacturer_id: Option<i64>,
    pub manufacturer_name: Option<String>,  // LEFT JOINで取得
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,  // 0: 有効, 1: 非表示
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

**エラー時**: `String` (エラーメッセージ)

### 使用例（JavaScript）

```javascript
// 有効な商品のみ取得
const products = await invoke('get_products', {
    userId: 1,
    includeDisabled: false
});

// 非表示項目を含めて取得
const allProducts = await invoke('get_products', {
    userId: 1,
    includeDisabled: true
});
```

### SQL

**include_disabled = false の場合:**

```sql
SELECT 
    P.PRODUCT_ID,
    P.USER_ID,
    P.PRODUCT_NAME,
    P.MANUFACTURER_ID,
    M.MANUFACTURER_NAME,
    P.MEMO,
    P.DISPLAY_ORDER,
    P.IS_DISABLED,
    P.ENTRY_DT,
    P.UPDATE_DT
FROM PRODUCTS P
LEFT JOIN MANUFACTURERS M 
    ON P.MANUFACTURER_ID = M.MANUFACTURER_ID 
    AND M.IS_DISABLED = 0
WHERE P.USER_ID = ? AND P.IS_DISABLED = 0
ORDER BY P.DISPLAY_ORDER
```

**include_disabled = true の場合:**

```sql
SELECT 
    P.PRODUCT_ID,
    P.USER_ID,
    P.PRODUCT_NAME,
    P.MANUFACTURER_ID,
    M.MANUFACTURER_NAME,
    P.MEMO,
    P.DISPLAY_ORDER,
    P.IS_DISABLED,
    P.ENTRY_DT,
    P.UPDATE_DT
FROM PRODUCTS P
LEFT JOIN MANUFACTURERS M 
    ON P.MANUFACTURER_ID = M.MANUFACTURER_ID
WHERE P.USER_ID = ?
ORDER BY P.DISPLAY_ORDER
```

### 注意事項

- メーカーが非表示の場合、`manufacturer_name` は `None` になります（LEFT JOINの条件により）
- メーカーが設定されていない商品は、`manufacturer_id` と `manufacturer_name` が `None` です

---

## add_product

### 概要

新しい商品を追加します。

### パラメータ

| パラメータ名 | 型 | 必須 | 説明 |
|-------------|---|------|------|
| `user_id` | `i64` | ✅ | ユーザーID |
| `product_name` | `String` | ✅ | 商品名 |
| `manufacturer_id` | `Option<i64>` | ❌ | メーカーID |
| `memo` | `Option<String>` | ❌ | メモ |
| `is_disabled` | `Option<i64>` | ❌ | 非表示フラグ（0: 有効, 1: 非表示、デフォルト: 0） |

### 戻り値

**成功時**: `Result<String, String>` - `"Product added successfully"`

**エラー時**: `String` (エラーメッセージ)

### バリデーション

| 項目 | ルール | エラーメッセージ |
|------|--------|-----------------|
| `product_name` | 必須、空白不可 | `"Product name cannot be empty"` |
| `product_name` | 重複不可 | `"Product name already exists"` |

### 使用例（JavaScript）

```javascript
// メーカー指定なし
await invoke('add_product', {
    userId: 1,
    productName: 'サバ缶',
    manufacturerId: null,
    memo: '水煮',
    isDisabled: null  // デフォルト: 0（有効）
});

// メーカー指定あり
await invoke('add_product', {
    userId: 1,
    productName: 'サバ缶',
    manufacturerId: 5,
    memo: '水煮',
    isDisabled: null
});

// 非表示で追加
await invoke('add_product', {
    userId: 1,
    productName: 'テスト商品',
    manufacturerId: null,
    memo: null,
    isDisabled: 1  // 非表示
});
```

### SQL

```sql
INSERT INTO PRODUCTS (
    USER_ID,
    PRODUCT_NAME,
    MANUFACTURER_ID,
    MEMO,
    DISPLAY_ORDER,
    IS_DISABLED,
    ENTRY_DT
) VALUES (?, ?, ?, ?, ?, ?, datetime('now', 'localtime'))
```

### 注意事項

- `DISPLAY_ORDER` は自動的に設定されます（最大値 + 1）
- `is_disabled` が `null` の場合、デフォルト値 `0`（有効）が設定されます
- 商品名の重複チェックは、非表示項目を含めて行われます
- `manufacturer_id` は存在チェックされません（NULL許可）

---

## update_product

### 概要

既存の商品情報を更新します。

### パラメータ

| パラメータ名 | 型 | 必須 | 説明 |
|-------------|---|------|------|
| `user_id` | `i64` | ✅ | ユーザーID |
| `product_id` | `i64` | ✅ | 商品ID |
| `product_name` | `String` | ✅ | 商品名 |
| `manufacturer_id` | `Option<i64>` | ❌ | メーカーID |
| `memo` | `Option<String>` | ❌ | メモ |
| `display_order` | `i64` | ✅ | 表示順序 |
| `is_disabled` | `i64` | ✅ | 非表示フラグ（0: 有効, 1: 非表示） |

### 戻り値

**成功時**: `Result<String, String>` - `"Product updated successfully"`

**エラー時**: `String` (エラーメッセージ)

### バリデーション

| 項目 | ルール | エラーメッセージ |
|------|--------|-----------------|
| `product_id` | 存在確認 | `"Product not found"` |
| `product_name` | 必須、空白不可 | `"Product name cannot be empty"` |
| `product_name` | 重複不可（自身を除く） | `"Product name already exists"` |

### 使用例（JavaScript）

```javascript
const product = products[0];

await invoke('update_product', {
    userId: 1,
    productId: product.product_id,
    productName: 'サバの水煮缶',
    manufacturerId: 5,
    memo: '更新後のメモ',
    displayOrder: product.display_order,
    isDisabled: 0  // 有効
});
```

### SQL

```sql
UPDATE PRODUCTS SET
    PRODUCT_NAME = ?,
    MANUFACTURER_ID = ?,
    MEMO = ?,
    DISPLAY_ORDER = ?,
    IS_DISABLED = ?,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND PRODUCT_ID = ?
```

### 注意事項

- 同じ商品名での更新は許可されます（自身の名前を変更しない場合）
- 重複チェックは、自身を除外して行われます
- `manufacturer_id` を `null` にすることでメーカーとの関連を解除できます

---

## delete_product

### 概要

商品を論理削除します（IS_DISABLED を 1 に設定）。

### パラメータ

| パラメータ名 | 型 | 必須 | 説明 |
|-------------|---|------|------|
| `user_id` | `i64` | ✅ | ユーザーID |
| `product_id` | `i64` | ✅ | 商品ID |

### 戻り値

**成功時**: `Result<String, String>` - `"Product deleted successfully"`

**エラー時**: `String` (エラーメッセージ)

### バリデーション

| 項目 | ルール | エラーメッセージ |
|------|--------|-----------------|
| `product_id` | 存在確認 | `"Product not found"` |

### 使用例（JavaScript）

```javascript
await invoke('delete_product', {
    userId: 1,
    productId: 123
});
```

### SQL

```sql
UPDATE PRODUCTS SET
    IS_DISABLED = 1,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND PRODUCT_ID = ?
```

### 注意事項

- 物理削除ではなく論理削除（IS_DISABLED = 1）を実行します
- 削除された商品は `get_products` で `include_disabled=false` の場合、取得されません

---

## エラーハンドリング

### 一般的なエラー

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"Product name cannot be empty"` | 商品名が空白 | 有効な商品名を入力 |
| `"Product name already exists"` | 重複する商品名 | 異なる商品名を入力 |
| `"Product not found"` | 商品が存在しない | 有効な商品IDを指定 |
| `"Failed to get products: ..."` | データベースエラー | データベース接続を確認 |
| `"Failed to add product: ..."` | データベースエラー | データベース接続を確認 |
| `"Failed to update product: ..."` | データベースエラー | データベース接続を確認 |
| `"Failed to delete product: ..."` | データベースエラー | データベース接続を確認 |

---

## データモデル

### PRODUCTS テーブル

| カラム名 | 型 | NULL | 説明 |
|---------|---|------|------|
| `PRODUCT_ID` | INTEGER | ❌ | 主キー（自動採番） |
| `USER_ID` | INTEGER | ❌ | ユーザーID |
| `PRODUCT_NAME` | TEXT | ❌ | 商品名 |
| `MANUFACTURER_ID` | INTEGER | ✅ | メーカーID（外部キー） |
| `MEMO` | TEXT | ✅ | メモ |
| `DISPLAY_ORDER` | INTEGER | ❌ | 表示順序 |
| `IS_DISABLED` | INTEGER | ❌ | 非表示フラグ（0: 有効, 1: 非表示）デフォルト: 0 |
| `ENTRY_DT` | TEXT | ❌ | 登録日時 |
| `UPDATE_DT` | TEXT | ✅ | 更新日時 |

### インデックス

```sql
CREATE UNIQUE INDEX IDX_PRODUCTS_USER_NAME 
ON PRODUCTS(USER_ID, PRODUCT_NAME);

CREATE INDEX IDX_PRODUCTS_USER_ORDER 
ON PRODUCTS(USER_ID, DISPLAY_ORDER);

CREATE INDEX IDX_PRODUCTS_MANUFACTURER 
ON PRODUCTS(MANUFACTURER_ID);
```

### 外部キー制約

```sql
FOREIGN KEY (MANUFACTURER_ID) 
REFERENCES MANUFACTURERS(MANUFACTURER_ID) 
ON DELETE SET NULL
```

**動作:**
- メーカーが削除されても、商品は削除されません
- `MANUFACTURER_ID` が `NULL` に設定されます（ON DELETE SET NULL）

---

## メーカーとの関連

### メーカー削除時の動作

メーカーが論理削除（IS_DISABLED = 1）された場合：

1. **商品データ**: 削除されません（`PRODUCT.MANUFACTURER_ID` は保持）
2. **リスト表示**: `get_products` で `manufacturer_name` が `None` になります
3. **理由**: LEFT JOIN の条件 `M.IS_DISABLED = 0` により、非表示メーカーは結合されません

### 使用例

```javascript
// メーカーを論理削除
await invoke('delete_manufacturer', { userId: 1, manufacturerId: 5 });

// 商品を取得（メーカーIDは残っているが、nameはNone）
const products = await invoke('get_products', { userId: 1, includeDisabled: false });
// products[0].manufacturer_id = 5
// products[0].manufacturer_name = null  ← メーカーが非表示のため
```

---

## 関連ドキュメント

- [メーカー管理・商品管理機能 - ユーザーガイド](./MANUFACTURER_PRODUCT_MANAGEMENT.md)
- [IS_DISABLED実装ガイド（開発者向け）](./IS_DISABLED_IMPLEMENTATION_GUIDE.md)
- [メーカー管理 API仕様](./API_MANUFACTURER.md)

---

**変更履歴**
- 2025-11-12: 初版作成（IS_DISABLED機能を含む）
