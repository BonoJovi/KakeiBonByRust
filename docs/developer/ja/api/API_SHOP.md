# 店舗管理 API ドキュメント

## 概要

本ドキュメントは、KakeiBonの店舗管理に関するバックエンドAPIについて説明します。店舗情報の取得、追加、更新、削除機能を提供します。

---

## データ構造

### Shop

```rust
pub struct Shop {
    pub shop_id: i64,
    pub user_id: i64,
    pub shop_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

### AddShopRequest

```rust
pub struct AddShopRequest {
    pub shop_name: String,
    pub memo: Option<String>,
}
```

### UpdateShopRequest

```rust
pub struct UpdateShopRequest {
    pub shop_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
}
```

---

## API一覧

### 1. 店舗一覧取得

#### `get_shops`

指定されたユーザーの店舗一覧を取得します。論理削除されていない店舗のみが返却されます。

**パラメータ:**
- `user_id` (i64): ユーザーID

**戻り値:**
- `Vec<Shop>`: 店舗情報の配列

**レスポンス例:**
```javascript
[
  {
    shop_id: 1,
    user_id: 2,
    shop_name: "イオン新宿店",
    memo: "よく利用するスーパー",
    display_order: 1,
    is_disabled: 0,
    entry_dt: "2024-11-10 12:00:00",
    update_dt: null
  },
  {
    shop_id: 2,
    user_id: 2,
    shop_name: "セブンイレブン駅前店",
    memo: null,
    display_order: 2,
    is_disabled: 0,
    entry_dt: "2024-11-10 13:00:00",
    update_dt: null
  }
]
```

**使用例:**
```javascript
const shops = await invoke('get_shops', { userId: 2 });
console.log('店舗数:', shops.length);
```

**エラー:**
- データベースエラーが発生した場合、エラーメッセージを返します

---

### 2. 店舗追加

#### `add_shop`

新しい店舗を追加します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `shop_name` (String): 店舗名（必須）
- `memo` (Option<String>): メモ（任意）

**戻り値:**
- `String`: 成功メッセージ "Shop added successfully"

**使用例:**
```javascript
try {
  const result = await invoke('add_shop', {
    userId: 2,
    shopName: "イオン新宿店",
    memo: "よく利用するスーパー"
  });
  console.log(result); // "Shop added successfully"
} catch (error) {
  console.error('店舗追加エラー:', error);
}
```

**バリデーション:**
- 店舗名が空文字の場合、エラーを返します
- 同じユーザーで同じ店舗名がすでに存在する場合、エラーを返します

**エラーメッセージ:**
- `"Shop name cannot be empty"`: 店舗名が空
- `"Shop name already exists"`: 店舗名が重複
- `"Failed to add shop: {詳細}"`: データベースエラー

**備考:**
- `display_order`は自動的に設定されます（既存の最大値+1）
- `is_disabled`は0（有効）で作成されます

---

### 3. 店舗更新

#### `update_shop`

既存の店舗情報を更新します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `shop_id` (i64): 店舗ID
- `shop_name` (String): 店舗名（必須）
- `memo` (Option<String>): メモ（任意）
- `display_order` (i64): 表示順

**戻り値:**
- `String`: 成功メッセージ "Shop updated successfully"

**使用例:**
```javascript
try {
  const result = await invoke('update_shop', {
    userId: 2,
    shopId: 1,
    shopName: "イオン渋谷店",
    memo: "リニューアルオープン後の店舗",
    displayOrder: 1
  });
  console.log(result); // "Shop updated successfully"
} catch (error) {
  console.error('店舗更新エラー:', error);
}
```

**バリデーション:**
- 店舗名が空文字の場合、エラーを返します
- 指定された店舗が存在しない場合、エラーを返します
- 同じユーザーで同じ店舗名が他の店舗に存在する場合、エラーを返します

**エラーメッセージ:**
- `"Shop name cannot be empty"`: 店舗名が空
- `"Shop not found"`: 店舗が見つからない
- `"Shop name already exists"`: 店舗名が重複
- `"Failed to update shop: {詳細}"`: データベースエラー

**備考:**
- `update_dt`は自動的に更新されます

---

### 4. 店舗削除

#### `delete_shop`

店舗を論理削除します（物理削除ではありません）。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `shop_id` (i64): 店舗ID

**戻り値:**
- `String`: 成功メッセージ "Shop deleted successfully"

**使用例:**
```javascript
try {
  const result = await invoke('delete_shop', {
    userId: 2,
    shopId: 1
  });
  console.log(result); // "Shop deleted successfully"
} catch (error) {
  console.error('店舗削除エラー:', error);
}
```

**バリデーション:**
- 指定された店舗が存在しない場合、エラーを返します

**エラーメッセージ:**
- `"Shop not found"`: 店舗が見つからない
- `"Failed to delete shop: {詳細}"`: データベースエラー

**備考:**
- 論理削除のため、`is_disabled`フラグが1に設定されます
- 物理的にデータベースから削除されることはありません
- 削除された店舗は`get_shops`では取得されません

---

## データベーステーブル

### SHOPS テーブル

```sql
CREATE TABLE SHOPS (
    SHOP_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    SHOP_NAME TEXT NOT NULL,
    MEMO TEXT,
    DISPLAY_ORDER INTEGER NOT NULL DEFAULT 0,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT TEXT NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT TEXT,
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID)
);
```

### インデックス

- `USER_ID`に対するインデックス（高速検索のため）
- `SHOP_NAME`と`USER_ID`の組み合わせにユニーク制約（論理削除されていない場合）

---

## 使用例：完全なCRUD操作

```javascript
import { invoke } from '@tauri-apps/api/core';

// 1. 店舗一覧取得
async function loadShops(userId) {
  try {
    const shops = await invoke('get_shops', { userId });
    return shops;
  } catch (error) {
    console.error('店舗読み込みエラー:', error);
    return [];
  }
}

// 2. 店舗追加
async function addNewShop(userId, shopName, memo) {
  try {
    await invoke('add_shop', {
      userId,
      shopName,
      memo
    });
    alert('店舗を追加しました');
    return true;
  } catch (error) {
    alert(`エラー: ${error}`);
    return false;
  }
}

// 3. 店舗更新
async function updateExistingShop(userId, shopId, shopName, memo, displayOrder) {
  try {
    await invoke('update_shop', {
      userId,
      shopId,
      shopName,
      memo,
      displayOrder
    });
    alert('店舗を更新しました');
    return true;
  } catch (error) {
    alert(`エラー: ${error}`);
    return false;
  }
}

// 4. 店舗削除
async function deleteExistingShop(userId, shopId) {
  if (!confirm('この店舗を削除してもよろしいですか？')) {
    return false;
  }
  
  try {
    await invoke('delete_shop', { userId, shopId });
    alert('店舗を削除しました');
    return true;
  } catch (error) {
    alert(`エラー: ${error}`);
    return false;
  }
}
```

---

## セキュリティ考慮事項

1. **ユーザー分離**: 各ユーザーは自分の店舗のみアクセス可能
2. **重複チェック**: 同一ユーザー内での店舗名重複を防止
3. **論理削除**: データ整合性を保つため物理削除は行わない
4. **バリデーション**: 空文字や不正なデータを事前にチェック

---

## 関連ドキュメント

- [入出金管理API](./API_TRANSACTION.md)
- [口座管理API](./API_ACCOUNT.md) (今後作成予定)
- [店舗管理UI](./SHOP_MANAGEMENT_UI.md) (今後作成予定)

---

## テスト

店舗管理機能には以下のテストが含まれています：

- `test_add_shop`: 店舗追加機能
- `test_update_shop`: 店舗更新機能
- `test_delete_shop`: 店舗削除機能
- `test_get_shops`: 店舗一覧取得機能
- `test_duplicate_shop_name`: 重複チェック機能

テスト実行方法：
```bash
cargo test --lib services::shop::tests
```

---

**最終更新日**: 2025-11-10 JST  
**バージョン**: 1.0.0
