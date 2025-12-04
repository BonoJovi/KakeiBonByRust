# マスタデータ管理API リファレンス

**最終更新**: 2025-12-05 02:10 JST

## 概要

本ドキュメントは、マスタデータ管理画面（shop-management.html、manufacturer-management.html、product-management.html）で使用されるAPIの仕様を定義します。店舗、メーカー、商品の3つのマスタデータを管理します。

---

## 目次

1. [店舗管理API](#店舗管理api)
2. [メーカー管理API](#メーカー管理api)
3. [商品管理API](#商品管理api)
4. [データ構造](#データ構造)

---

## 店舗管理API

### get_shops

店舗の一覧を取得します。

**パラメータ:** なし

**戻り値:**
- `Vec<Shop>`: 店舗の配列（非表示以外）

**Shop構造:**
```javascript
{
    shop_id: number,
    user_id: number,
    shop_name: string,
    memo: string | null,
    display_order: number,
    is_disabled: number,    // 0=有効, 1=無効
    entry_dt: string,
    update_dt: string | null
}
```

**使用例:**
```javascript
const shops = await invoke('get_shops');
shops.forEach(shop => {
    console.log(`${shop.shop_name}`);
});
```

**注意:**
- セッションユーザーIDを自動取得
- `IS_DISABLED = 0`のみ取得（非表示除外）
- `DISPLAY_ORDER`でソート

---

### add_shop

新しい店舗を追加します。

**パラメータ:**
- `shop_name` (String): 店舗名（必須）
- `memo` (Option<String>): メモ

**戻り値:**
- `String`: "Shop added successfully"

**使用例:**
```javascript
try {
    await invoke('add_shop', {
        shopName: 'イオン新宿店',
        memo: '毎週火曜日にポイント2倍'
    });
    
    alert('店舗を追加しました');
    await loadShops();
} catch (error) {
    alert(`追加失敗: ${error}`);
}
```

**自動処理:**
1. **表示順の自動設定**: 最大値+1
2. **is_disabled**: 0（有効）に設定

**バリデーション:**
- 店舗名は必須
- 同一ユーザー内での名前重複チェック

**エラー:**
- `"Shop name cannot be empty"`: 店舗名が空
- `"Shop name 'XXX' already exists"`: 名前が重複
- `"Failed to add shop: ..."`: データベースエラー

---

### update_shop

店舗情報を更新します。

**パラメータ:**
- `shop_id` (i64): 店舗ID
- `shop_name` (String): 新しい店舗名
- `memo` (Option<String>): 新しいメモ
- `display_order` (i64): 新しい表示順

**戻り値:**
- `String`: "Shop updated successfully"

**使用例:**
```javascript
await invoke('update_shop', {
    shopId: 1,
    shopName: 'イオン新宿店（更新）',
    memo: null,
    displayOrder: 2
});
```

**バリデーション:**
- 自分自身を除いて名前重複チェック

---

### delete_shop

店舗を論理削除します。

**パラメータ:**
- `shop_id` (i64): 店舗ID

**戻り値:**
- `String`: "Shop deleted successfully"

**使用例:**
```javascript
if (confirm('この店舗を削除してもよろしいですか？')) {
    await invoke('delete_shop', { shopId: 1 });
    alert('店舗を削除しました');
    await loadShops();
}
```

**動作:**
- 論理削除（`IS_DISABLED = 1`）
- トランザクションで使用中でも削除可能
- 過去のトランザクション履歴は保持

---

## メーカー管理API

### get_manufacturers

メーカーの一覧を取得します。

**パラメータ:**
- `include_disabled` (bool): 非表示を含むかどうか

**戻り値:**
- `Vec<Manufacturer>`: メーカーの配列

**Manufacturer構造:**
```javascript
{
    manufacturer_id: number,
    user_id: number,
    manufacturer_name: string,
    memo: string | null,
    display_order: number,
    is_disabled: number,
    entry_dt: string,
    update_dt: string | null
}
```

**使用例:**
```javascript
// 有効なメーカーのみ取得
const manufacturers = await invoke('get_manufacturers', { 
    includeDisabled: false 
});

// すべて取得（非表示含む）
const allManufacturers = await invoke('get_manufacturers', { 
    includeDisabled: true 
});
```

**用途:**
- `includeDisabled = false`: 商品登録時の選択肢
- `includeDisabled = true`: メーカー管理画面での一覧表示

---

### add_manufacturer

新しいメーカーを追加します。

**パラメータ:**
- `manufacturer_name` (String): メーカー名（必須）
- `memo` (Option<String>): メモ
- `is_disabled` (Option<i64>): 非表示フラグ（デフォルト: 0）

**戻り値:**
- `String`: "Manufacturer added successfully"

**使用例:**
```javascript
await invoke('add_manufacturer', {
    manufacturerName: 'キリン',
    memo: null,
    isDisabled: 0
});
```

**自動処理:**
1. **表示順の自動設定**: 最大値+1
2. **is_disabled**: デフォルト0（省略時）

**バリデーション:**
- メーカー名は必須
- 同一ユーザー内での名前重複チェック

---

### update_manufacturer

メーカー情報を更新します。

**パラメータ:**
- `manufacturer_id` (i64): メーカーID
- `manufacturer_name` (String): 新しいメーカー名
- `memo` (Option<String>): 新しいメモ
- `display_order` (i64): 新しい表示順
- `is_disabled` (i64): 非表示フラグ

**戻り値:**
- `String`: "Manufacturer updated successfully"

**使用例:**
```javascript
await invoke('update_manufacturer', {
    manufacturerId: 1,
    manufacturerName: 'キリンビバレッジ',
    memo: 'メモ更新',
    displayOrder: 1,
    isDisabled: 0
});
```

---

### delete_manufacturer

メーカーを論理削除します。

**パラメータ:**
- `manufacturer_id` (i64): メーカーID

**戻り値:**
- `String`: "Manufacturer deleted successfully"

**使用例:**
```javascript
await invoke('delete_manufacturer', { manufacturerId: 1 });
```

**動作:**
- 論理削除（`IS_DISABLED = 1`）
- 関連する商品は削除されません
- 商品の`manufacturer_id`は保持されるが、取得時に名前は`null`

**注意:**
- メーカーを削除しても、そのメーカーを参照する商品は削除されない
- 商品一覧でメーカー名が表示されなくなる（LEFT JOINでnull）

---

## 商品管理API

### get_products

商品の一覧を取得します（メーカー名付き）。

**パラメータ:**
- `include_disabled` (bool): 非表示を含むかどうか

**戻り値:**
- `Vec<Product>`: 商品の配列

**Product構造:**
```javascript
{
    product_id: number,
    user_id: number,
    product_name: string,
    manufacturer_id: number | null,
    manufacturer_name: string | null,  // LEFT JOINで取得
    memo: string | null,
    display_order: number,
    is_disabled: number,
    entry_dt: string,
    update_dt: string | null
}
```

**使用例:**
```javascript
const products = await invoke('get_products', { 
    includeDisabled: false 
});

products.forEach(product => {
    const maker = product.manufacturer_name || '(メーカー不明)';
    console.log(`${product.product_name} - ${maker}`);
});
```

**メーカー名の取得:**
- LEFT JOINでMANUFACTURERSテーブルと結合
- メーカーが未設定 → `manufacturer_name = null`
- メーカーが削除済み → `manufacturer_name = null`

---

### add_product

新しい商品を追加します。

**パラメータ:**
- `product_name` (String): 商品名（必須）
- `manufacturer_id` (Option<i64>): メーカーID
- `memo` (Option<String>): メモ
- `is_disabled` (Option<i64>): 非表示フラグ（デフォルト: 0）

**戻り値:**
- `String`: "Product added successfully"

**使用例:**
```javascript
await invoke('add_product', {
    productName: 'キリン一番搾り',
    manufacturerId: 1,
    memo: '350ml缶',
    isDisabled: 0
});

// メーカー未設定でも可能
await invoke('add_product', {
    productName: 'ノーブランド商品',
    manufacturerId: null,
    memo: null,
    isDisabled: 0
});
```

**自動処理:**
- 表示順の自動設定
- is_disabledのデフォルト値設定

**バリデーション:**
- 商品名は必須
- 同一ユーザー内での名前重複チェック

---

### update_product

商品情報を更新します。

**パラメータ:**
- `product_id` (i64): 商品ID
- `product_name` (String): 新しい商品名
- `manufacturer_id` (Option<i64>): 新しいメーカーID
- `memo` (Option<String>): 新しいメモ
- `display_order` (i64): 新しい表示順
- `is_disabled` (i64): 非表示フラグ

**戻り値:**
- `String`: "Product updated successfully"

**使用例:**
```javascript
await invoke('update_product', {
    productId: 1,
    productName: 'キリン一番搾り（更新）',
    manufacturerId: 2,  // メーカー変更
    memo: '500ml缶',
    displayOrder: 1,
    isDisabled: 0
});
```

---

### delete_product

商品を論理削除します。

**パラメータ:**
- `product_id` (i64): 商品ID

**戻り値:**
- `String`: "Product deleted successfully"

**使用例:**
```javascript
await invoke('delete_product', { productId: 1 });
```

**動作:**
- 論理削除（`IS_DISABLED = 1`）
- トランザクション明細で使用中でも削除可能

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

---

### Manufacturer

```rust
pub struct Manufacturer {
    pub manufacturer_id: i64,
    pub user_id: i64,
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

---

### Product

```rust
pub struct Product {
    pub product_id: i64,
    pub user_id: i64,
    pub product_name: String,
    pub manufacturer_id: Option<i64>,
    pub manufacturer_name: Option<String>,  // LEFT JOINで取得
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

**manufacturer_nameがnullになる場合:**
1. `manufacturer_id`が`null`（メーカー未設定）
2. メーカーが論理削除済み（`IS_DISABLED = 1`）

---

## エラーハンドリング

### 共通エラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"User not authenticated"` | セッション未認証 | ログインが必要 |
| `"Shop name cannot be empty"` | 店舗名が空 | 店舗名を入力 |
| `"Shop name 'XXX' already exists"` | 名前が重複 | 別の名前を使用 |
| `"Manufacturer name 'XXX' already exists"` | 名前が重複 | 別の名前を使用 |
| `"Product name 'XXX' already exists"` | 名前が重複 | 別の名前を使用 |
| `"Failed to add ...: ..."` | データベースエラー | データベース確認 |

### フロントエンドでのエラーハンドリング例

```javascript
// 店舗追加
async function addShop(name, memo) {
    try {
        await invoke('add_shop', {
            shopName: name,
            memo
        });
        
        alert('店舗を追加しました');
        return true;
    } catch (error) {
        if (error.includes('already exists')) {
            alert('この店舗名は既に使用されています');
        } else if (error.includes('cannot be empty')) {
            alert('店舗名を入力してください');
        } else {
            alert(`エラー: ${error}`);
        }
        return false;
    }
}

// 商品追加（メーカー連携）
async function addProductWithManufacturer(productName, manufacturerId, memo) {
    try {
        await invoke('add_product', {
            productName,
            manufacturerId,
            memo,
            isDisabled: 0
        });
        
        alert('商品を追加しました');
        return true;
    } catch (error) {
        alert(`エラー: ${error}`);
        return false;
    }
}
```

---

## IS_DISABLED機能の活用

### 論理削除のメリット

1. **データ保全**: 過去のトランザクション履歴を保護
2. **再有効化**: 誤って削除した場合も復活可能
3. **監査証跡**: 削除履歴を保持

### 表示/非表示の切り替え

```javascript
// 管理画面でトグルボタンを実装
async function toggleShowDisabled() {
    const showDisabled = document.getElementById('show-disabled-toggle').checked;
    
    const manufacturers = await invoke('get_manufacturers', {
        includeDisabled: showDisabled
    });
    
    renderManufacturerList(manufacturers);
}
```

### 再有効化の実装例

```javascript
// 論理削除されたメーカーを再有効化
async function reactivateManufacturer(manufacturerId) {
    try {
        // 現在の情報を取得
        const manufacturers = await invoke('get_manufacturers', { 
            includeDisabled: true 
        });
        const manufacturer = manufacturers.find(m => m.manufacturer_id === manufacturerId);
        
        if (!manufacturer) {
            alert('メーカーが見つかりません');
            return;
        }
        
        // is_disabled = 0 に更新
        await invoke('update_manufacturer', {
            manufacturerId,
            manufacturerName: manufacturer.manufacturer_name,
            memo: manufacturer.memo,
            displayOrder: manufacturer.display_order,
            isDisabled: 0  // 再有効化
        });
        
        alert('メーカーを再有効化しました');
        await loadManufacturers();
    } catch (error) {
        alert(`エラー: ${error}`);
    }
}
```

---

## 使用例：マスタデータ管理画面の実装

### 店舗一覧表示

```javascript
async function loadShops() {
    try {
        const shops = await invoke('get_shops');
        
        const tbody = document.getElementById('shop-table-body');
        tbody.innerHTML = '';
        
        shops.forEach(shop => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td>${shop.shop_id}</td>
                <td>${shop.shop_name}</td>
                <td>${shop.memo || '-'}</td>
                <td>
                    <button onclick="editShop(${shop.shop_id})">編集</button>
                    <button onclick="deleteShop(${shop.shop_id})">削除</button>
                </td>
            `;
            tbody.appendChild(row);
        });
    } catch (error) {
        console.error('店舗一覧の読み込みエラー:', error);
    }
}
```

### 商品追加フォーム（メーカー選択付き）

```javascript
async function initializeProductForm() {
    // メーカー選択肢を読み込み
    const manufacturers = await invoke('get_manufacturers', { 
        includeDisabled: false 
    });
    
    const select = document.getElementById('manufacturer-select');
    select.innerHTML = '<option value="">（メーカーなし）</option>';
    
    manufacturers.forEach(m => {
        const option = document.createElement('option');
        option.value = m.manufacturer_id;
        option.textContent = m.manufacturer_name;
        select.appendChild(option);
    });
}

async function handleAddProduct(event) {
    event.preventDefault();
    
    const productName = document.getElementById('product-name').value;
    const manufacturerId = document.getElementById('manufacturer-select').value || null;
    const memo = document.getElementById('memo').value || null;
    
    try {
        await invoke('add_product', {
            productName,
            manufacturerId: manufacturerId ? parseInt(manufacturerId) : null,
            memo,
            isDisabled: 0
        });
        
        alert('商品を追加しました');
        event.target.reset();
        await loadProducts();
    } catch (error) {
        alert(`エラー: ${error}`);
    }
}
```

---

## データ連携

### メーカー・商品の連携

```
MANUFACTURERS (メーカー)
    ↓ 1対多
PRODUCTS (商品)
    ↓ 参照
TRANSACTION_DETAILS (明細)
```

**削除時の挙動:**
1. **メーカー削除**: 商品は削除されない（manufacturer_idは保持）
2. **商品削除**: 明細は削除されない（product_idは保持）

### 店舗・トランザクションの連携

```
SHOPS (店舗)
    ↓ 参照
TRANSACTION_HEADERS (ヘッダ)
```

**削除時の挙動:**
- 店舗削除後もトランザクションは保持
- 店舗名は過去の記録として残る

---

## テストカバレッジ

**ShopService:**
- ✅ 店舗一覧取得テスト
- ✅ 店舗追加テスト
- ✅ 店舗更新テスト
- ✅ 店舗削除テスト
- ✅ 名前重複チェック

**ManufacturerService:**
- ✅ メーカー一覧取得テスト（include_disabled）
- ✅ メーカー追加テスト
- ✅ メーカー更新テスト
- ✅ メーカー削除テスト
- ✅ 名前重複チェック

**ProductService:**
- ✅ 商品一覧取得テスト（メーカー名付き）
- ✅ 商品追加テスト
- ✅ 商品更新テスト
- ✅ 商品削除テスト
- ✅ メーカー連携テスト
- ✅ 名前重複チェック

---

## 関連ドキュメント

### 実装ファイル

- 店舗サービス: `src/services/shop.rs`
- メーカーサービス: `src/services/manufacturer.rs`
- 商品サービス: `src/services/product.rs`
- SQL定義: `src/sql_queries.rs`
- Tauri Commands: `src/lib.rs`

### その他のAPIリファレンス

- [共通API](./API_COMMON.md) - セッション管理
- [入出金管理API](./API_TRANSACTION.md) - 店舗・商品の利用

### ガイドドキュメント

- [IS_DISABLED実装ガイド](../guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md) - 論理削除の詳細

---

**変更履歴:**
- 2025-12-05: 初版作成（実装コードに基づく、Shop/Manufacturer/Product統合）
