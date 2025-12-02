# メーカー管理 API仕様

**Last Updated**: 2025-11-12 01:31 JST

## 概要

メーカー管理機能のTauri Command APIの仕様を定義します。

---

## API一覧

| API名 | 説明 |
|-------|------|
| `get_manufacturers` | メーカー一覧取得 |
| `add_manufacturer` | メーカー追加 |
| `update_manufacturer` | メーカー更新 |
| `delete_manufacturer` | メーカー削除（論理削除） |

---

## get_manufacturers

### 概要

指定されたユーザーのメーカー一覧を取得します。

### パラメータ

| パラメータ名 | 型 | 必須 | 説明 |
|-------------|---|------|------|
| `user_id` | `i64` | ✅ | ユーザーID |
| `include_disabled` | `bool` | ✅ | 非表示項目を含めるか（true: 含める、false: 除外） |

### 戻り値

**成功時**: `Result<Vec<Manufacturer>, String>`

```rust
pub struct Manufacturer {
    pub manufacturer_id: i64,
    pub user_id: i64,
    pub manufacturer_name: String,
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
// 有効なメーカーのみ取得
const manufacturers = await invoke('get_manufacturers', {
    userId: 1,
    includeDisabled: false
});

// 非表示項目を含めて取得
const allManufacturers = await invoke('get_manufacturers', {
    userId: 1,
    includeDisabled: true
});
```

### SQL

**include_disabled = false の場合:**

```sql
SELECT 
    MANUFACTURER_ID,
    USER_ID,
    MANUFACTURER_NAME,
    MEMO,
    DISPLAY_ORDER,
    IS_DISABLED,
    ENTRY_DT,
    UPDATE_DT
FROM MANUFACTURERS
WHERE USER_ID = ? AND IS_DISABLED = 0
ORDER BY DISPLAY_ORDER
```

**include_disabled = true の場合:**

```sql
SELECT 
    MANUFACTURER_ID,
    USER_ID,
    MANUFACTURER_NAME,
    MEMO,
    DISPLAY_ORDER,
    IS_DISABLED,
    ENTRY_DT,
    UPDATE_DT
FROM MANUFACTURERS
WHERE USER_ID = ?
ORDER BY DISPLAY_ORDER
```

---

## add_manufacturer

### 概要

新しいメーカーを追加します。

### パラメータ

| パラメータ名 | 型 | 必須 | 説明 |
|-------------|---|------|------|
| `user_id` | `i64` | ✅ | ユーザーID |
| `manufacturer_name` | `String` | ✅ | メーカー名 |
| `memo` | `Option<String>` | ❌ | メモ |
| `is_disabled` | `Option<i64>` | ❌ | 非表示フラグ（0: 有効, 1: 非表示、デフォルト: 0） |

### 戻り値

**成功時**: `Result<String, String>` - `"Manufacturer added successfully"`

**エラー時**: `String` (エラーメッセージ)

### バリデーション

| 項目 | ルール | エラーメッセージ |
|------|--------|-----------------|
| `manufacturer_name` | 必須、空白不可 | `"Manufacturer name cannot be empty"` |
| `manufacturer_name` | 重複不可 | `"Manufacturer name already exists"` |

### 使用例（JavaScript）

```javascript
// 基本的な追加
await invoke('add_manufacturer', {
    userId: 1,
    manufacturerName: 'ニッスイ',
    memo: '日本水産株式会社',
    isDisabled: null  // デフォルト: 0（有効）
});

// 非表示で追加
await invoke('add_manufacturer', {
    userId: 1,
    manufacturerName: 'テストメーカー',
    memo: null,
    isDisabled: 1  // 非表示
});
```

### SQL

```sql
INSERT INTO MANUFACTURERS (
    USER_ID,
    MANUFACTURER_NAME,
    MEMO,
    DISPLAY_ORDER,
    IS_DISABLED,
    ENTRY_DT
) VALUES (?, ?, ?, ?, ?, datetime('now', 'localtime'))
```

### 注意事項

- `DISPLAY_ORDER` は自動的に設定されます（最大値 + 1）
- `is_disabled` が `null` の場合、デフォルト値 `0`（有効）が設定されます
- メーカー名の重複チェックは、非表示項目を含めて行われます

---

## update_manufacturer

### 概要

既存のメーカー情報を更新します。

### パラメータ

| パラメータ名 | 型 | 必須 | 説明 |
|-------------|---|------|------|
| `user_id` | `i64` | ✅ | ユーザーID |
| `manufacturer_id` | `i64` | ✅ | メーカーID |
| `manufacturer_name` | `String` | ✅ | メーカー名 |
| `memo` | `Option<String>` | ❌ | メモ |
| `display_order` | `i64` | ✅ | 表示順序 |
| `is_disabled` | `i64` | ✅ | 非表示フラグ（0: 有効, 1: 非表示） |

### 戻り値

**成功時**: `Result<String, String>` - `"Manufacturer updated successfully"`

**エラー時**: `String` (エラーメッセージ)

### バリデーション

| 項目 | ルール | エラーメッセージ |
|------|--------|-----------------|
| `manufacturer_id` | 存在確認 | `"Manufacturer not found"` |
| `manufacturer_name` | 必須、空白不可 | `"Manufacturer name cannot be empty"` |
| `manufacturer_name` | 重複不可（自身を除く） | `"Manufacturer name already exists"` |

### 使用例（JavaScript）

```javascript
const manufacturer = manufacturers[0];

await invoke('update_manufacturer', {
    userId: 1,
    manufacturerId: manufacturer.manufacturer_id,
    manufacturerName: '日本水産',
    memo: '更新後のメモ',
    displayOrder: manufacturer.display_order,
    isDisabled: 0  // 有効
});
```

### SQL

```sql
UPDATE MANUFACTURERS SET
    MANUFACTURER_NAME = ?,
    MEMO = ?,
    DISPLAY_ORDER = ?,
    IS_DISABLED = ?,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND MANUFACTURER_ID = ?
```

### 注意事項

- 同じメーカー名での更新は許可されます（自身の名前を変更しない場合）
- 重複チェックは、自身を除外して行われます

---

## delete_manufacturer

### 概要

メーカーを論理削除します（IS_DISABLED を 1 に設定）。

### パラメータ

| パラメータ名 | 型 | 必須 | 説明 |
|-------------|---|------|------|
| `user_id` | `i64` | ✅ | ユーザーID |
| `manufacturer_id` | `i64` | ✅ | メーカーID |

### 戻り値

**成功時**: `Result<String, String>` - `"Manufacturer deleted successfully"`

**エラー時**: `String` (エラーメッセージ)

### バリデーション

| 項目 | ルール | エラーメッセージ |
|------|--------|-----------------|
| `manufacturer_id` | 存在確認 | `"Manufacturer not found"` |

### 使用例（JavaScript）

```javascript
await invoke('delete_manufacturer', {
    userId: 1,
    manufacturerId: 123
});
```

### SQL

```sql
UPDATE MANUFACTURERS SET
    IS_DISABLED = 1,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND MANUFACTURER_ID = ?
```

### 注意事項

- 物理削除ではなく論理削除（IS_DISABLED = 1）を実行します
- 削除されたメーカーは `get_manufacturers` で `include_disabled=false` の場合、取得されません
- 削除されたメーカーに関連する商品は削除されません（商品データは保持されます）

---

## エラーハンドリング

### 一般的なエラー

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"Manufacturer name cannot be empty"` | メーカー名が空白 | 有効なメーカー名を入力 |
| `"Manufacturer name already exists"` | 重複するメーカー名 | 異なるメーカー名を入力 |
| `"Manufacturer not found"` | メーカーが存在しない | 有効なメーカーIDを指定 |
| `"Failed to get manufacturers: ..."` | データベースエラー | データベース接続を確認 |
| `"Failed to add manufacturer: ..."` | データベースエラー | データベース接続を確認 |
| `"Failed to update manufacturer: ..."` | データベースエラー | データベース接続を確認 |
| `"Failed to delete manufacturer: ..."` | データベースエラー | データベース接続を確認 |

---

## データモデル

### MANUFACTURERS テーブル

| カラム名 | 型 | NULL | 説明 |
|---------|---|------|------|
| `MANUFACTURER_ID` | INTEGER | ❌ | 主キー（自動採番） |
| `USER_ID` | INTEGER | ❌ | ユーザーID |
| `MANUFACTURER_NAME` | TEXT | ❌ | メーカー名 |
| `MEMO` | TEXT | ✅ | メモ |
| `DISPLAY_ORDER` | INTEGER | ❌ | 表示順序 |
| `IS_DISABLED` | INTEGER | ❌ | 非表示フラグ（0: 有効, 1: 非表示）デフォルト: 0 |
| `ENTRY_DT` | TEXT | ❌ | 登録日時 |
| `UPDATE_DT` | TEXT | ✅ | 更新日時 |

### インデックス

```sql
CREATE UNIQUE INDEX IDX_MANUFACTURERS_USER_NAME 
ON MANUFACTURERS(USER_ID, MANUFACTURER_NAME);

CREATE INDEX IDX_MANUFACTURERS_USER_ORDER 
ON MANUFACTURERS(USER_ID, DISPLAY_ORDER);
```

---

## 関連ドキュメント

- [メーカー管理・商品管理機能 - ユーザーガイド](./MANUFACTURER_PRODUCT_MANAGEMENT.md)
- [IS_DISABLED実装ガイド（開発者向け）](./IS_DISABLED_IMPLEMENTATION_GUIDE.md)
- [商品管理 API仕様](./API_PRODUCT.md)

---

**変更履歴**
- 2025-11-12: 初版作成（IS_DISABLED機能を含む）
