# IS_DISABLED機能 実装ガイド（開発者向け）

**Last Updated**: 2025-11-12 01:31 JST

## 概要

IS_DISABLED機能は、データを物理削除せず論理削除（非表示化）する機能です。このドキュメントでは、メーカー管理・商品管理で実装された機能を他の管理画面に適用する手順を説明します。

---

## 実装パターン

IS_DISABLED機能の実装は、以下の7ステップで構成されています：

```
1. SQL: GET_ALL_INCLUDING_DISABLED クエリ追加
2. SQL: INSERT/UPDATE に IS_DISABLED カラム追加
3. Backend: Request構造体に is_disabled フィールド追加
4. Backend: get関数に include_disabled パラメータ追加
5. Tauri: コマンドに is_disabled/include_disabled パラメータ追加
6. HTML: トグルボタン + モーダルにチェックボックス追加
7. JavaScript: showDisabledItems state + スタイリング + バッジ表示
```

---

## ステップ1: SQLクエリの追加

### 1.1 GET_ALL_INCLUDING_DISABLED クエリ

非表示項目を含む全件取得クエリを追加します。

**例（メーカー管理）:**

```rust
// src/sql_queries.rs

pub const MANUFACTURER_GET_ALL_INCLUDING_DISABLED: &str = r#"
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
"#;
```

### 1.2 INSERT/UPDATEクエリの更新

IS_DISABLEDカラムを含めるように既存クエリを更新します。

**例（INSERT）:**

```rust
pub const MANUFACTURER_INSERT: &str = r#"
INSERT INTO MANUFACTURERS (
    USER_ID,
    MANUFACTURER_NAME,
    MEMO,
    DISPLAY_ORDER,
    IS_DISABLED,
    ENTRY_DT
) VALUES (?, ?, ?, ?, ?, datetime('now', 'localtime'))
"#;
```

**例（UPDATE）:**

```rust
pub const MANUFACTURER_UPDATE: &str = r#"
UPDATE MANUFACTURERS SET
    MANUFACTURER_NAME = ?,
    MEMO = ?,
    DISPLAY_ORDER = ?,
    IS_DISABLED = ?,
    UPDATE_DT = datetime('now', 'localtime')
WHERE USER_ID = ? AND MANUFACTURER_ID = ?
"#;
```

---

## ステップ2: Backend - Request構造体の更新

### 2.1 AddRequest構造体

`is_disabled` フィールドを `Option<i64>` として追加します。

```rust
// src/services/manufacturer.rs

#[derive(Debug, Deserialize, Clone)]
pub struct AddManufacturerRequest {
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub is_disabled: Option<i64>,  // 追加
}
```

**ポイント:**
- `Option<i64>` を使用（デフォルト値は関数内で処理）
- 0 = 有効、1 = 非表示

### 2.2 UpdateRequest構造体

`is_disabled` フィールドを `i64` として追加します。

```rust
#[derive(Debug, Deserialize)]
pub struct UpdateManufacturerRequest {
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,  // 追加
}
```

**ポイント:**
- 更新時は必須なので `Option` なし

---

## ステップ3: Backend - get関数の更新

### 3.1 include_disabledパラメータの追加

```rust
pub async fn get_manufacturers(
    pool: &SqlitePool, 
    user_id: i64, 
    include_disabled: bool  // 追加
) -> Result<Vec<Manufacturer>, String> {
    let query = if include_disabled {
        sql_queries::MANUFACTURER_GET_ALL_INCLUDING_DISABLED
    } else {
        sql_queries::MANUFACTURER_GET_ALL
    };

    let manufacturers = sqlx::query_as::<_, Manufacturer>(query)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to get manufacturers: {}", e))?;

    Ok(manufacturers)
}
```

**ポイント:**
- `include_disabled` パラメータで使用するクエリを切り替え
- デフォルトは非表示項目を除外（false）

### 3.2 add/update関数の更新

**add関数:**

```rust
pub async fn add_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    request: AddManufacturerRequest,
) -> Result<String, String> {
    // バリデーション...

    let is_disabled = request.is_disabled.unwrap_or(0);  // デフォルト0

    sqlx::query(sql_queries::MANUFACTURER_INSERT)
        .bind(user_id)
        .bind(&request.manufacturer_name)
        .bind(&request.memo)
        .bind(display_order)
        .bind(is_disabled)  // 追加
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to add manufacturer: {}", e))?;

    Ok("Manufacturer added successfully".to_string())
}
```

**update関数:**

```rust
pub async fn update_manufacturer(
    pool: &SqlitePool,
    user_id: i64,
    manufacturer_id: i64,
    request: UpdateManufacturerRequest,
) -> Result<String, String> {
    // バリデーション...

    sqlx::query(sql_queries::MANUFACTURER_UPDATE)
        .bind(&request.manufacturer_name)
        .bind(&request.memo)
        .bind(request.display_order)
        .bind(request.is_disabled)  // 追加
        .bind(user_id)
        .bind(manufacturer_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update manufacturer: {}", e))?;

    Ok("Manufacturer updated successfully".to_string())
}
```

---

## ステップ4: Tauri Commands

### 4.1 コマンド関数の更新

```rust
// src/lib.rs

#[tauri::command]
async fn get_manufacturers(
    state: tauri::State<'_, AppState>,
    user_id: i64,
    include_disabled: bool,  // 追加
) -> Result<Vec<Manufacturer>, String> {
    let pool = &state.pool;
    manufacturer::get_manufacturers(pool, user_id, include_disabled).await
}

#[tauri::command]
async fn add_manufacturer(
    state: tauri::State<'_, AppState>,
    user_id: i64,
    manufacturer_name: String,
    memo: Option<String>,
    is_disabled: Option<i64>,  // 追加
) -> Result<String, String> {
    let pool = &state.pool;
    let request = manufacturer::AddManufacturerRequest {
        manufacturer_name,
        memo,
        is_disabled,
    };
    manufacturer::add_manufacturer(pool, user_id, request).await
}

#[tauri::command]
async fn update_manufacturer(
    state: tauri::State<'_, AppState>,
    user_id: i64,
    manufacturer_id: i64,
    manufacturer_name: String,
    memo: Option<String>,
    display_order: i64,
    is_disabled: i64,  // 追加
) -> Result<String, String> {
    let pool = &state.pool;
    let request = manufacturer::UpdateManufacturerRequest {
        manufacturer_name,
        memo,
        display_order,
        is_disabled,
    };
    manufacturer::update_manufacturer(pool, user_id, manufacturer_id, request).await
}
```

---

## ステップ5: HTML - UIコンポーネントの追加

### 5.1 トグルボタンの追加

```html
<!-- res/manufacturer-management.html -->

<div class="toolbar">
    <button id="add-manufacturer-btn" class="btn btn-primary" data-i18n="manufacturer_mgmt.add"></button>
    <button id="toggle-disabled-btn" class="btn btn-secondary" data-i18n="common.show_disabled"></button>
</div>
```

### 5.2 モーダルにチェックボックス追加

```html
<div class="form-group">
    <label>
        <input type="checkbox" id="manufacturer-is-disabled">
        <span data-i18n="common.is_disabled"></span>
    </label>
</div>
```

---

## ステップ6: JavaScript - フロントエンド実装

### 6.1 State管理

```javascript
// res/js/manufacturer-management.js

let showDisabledItems = false;
```

### 6.2 イベントリスナー

```javascript
function setupEventListeners() {
    // トグルボタン
    document.getElementById('toggle-disabled-btn').addEventListener('click', () => {
        showDisabledItems = !showDisabledItems;
        updateToggleButton();
        loadManufacturers();
    });
}

function updateToggleButton() {
    const btn = document.getElementById('toggle-disabled-btn');
    if (showDisabledItems) {
        btn.setAttribute('data-i18n', 'common.hide_disabled');
        btn.textContent = i18n.t('common.hide_disabled');
    } else {
        btn.setAttribute('data-i18n', 'common.show_disabled');
        btn.textContent = i18n.t('common.show_disabled');
    }
}
```

### 6.3 データロード

```javascript
async function loadManufacturers() {
    manufacturers = await invoke('get_manufacturers', {
        userId: currentUserId,
        includeDisabled: showDisabledItems  // 追加
    });
    renderManufacturers();
}
```

### 6.4 レンダリング - スタイリング

```javascript
function renderManufacturers() {
    manufacturers.forEach(manufacturer => {
        const row = tbody.insertRow();
        const isDisabled = manufacturer.is_disabled === 1;
        
        // 非表示項目のスタイリング
        if (isDisabled) {
            row.style.backgroundColor = '#6c757d';  // ダークグレー
        }
        
        // 名前セル + バッジ
        const nameCell = row.insertCell();
        if (isDisabled) {
            const badge = `<span style="color: #ffc107; font-weight: bold; margin-left: 8px;">[${i18n.t('common.disabled_label')}]</span>`;
            nameCell.innerHTML = `<span style="color: #ffffff;">${manufacturer.manufacturer_name}</span>${badge}`;
        } else {
            nameCell.textContent = manufacturer.manufacturer_name;
        }
        
        // メモセル
        const memoCell = row.insertCell();
        if (isDisabled) {
            memoCell.style.color = '#ffffff';
        }
        // ...
    });
}
```

### 6.5 モーダル - データの読み書き

```javascript
function initManufacturerModal() {
    manufacturerModal = new Modal('manufacturer-modal', {
        onOpen: (mode, data) => {
            if (mode === 'add') {
                document.getElementById('manufacturer-is-disabled').checked = false;
            } else if (mode === 'edit') {
                document.getElementById('manufacturer-is-disabled').checked = data.is_disabled === 1;
            }
        },
        // ...
    });
}

async function saveManufacturer() {
    const isDisabled = document.getElementById('manufacturer-is-disabled').checked ? 1 : 0;
    
    if (editingManufacturerId) {
        await invoke('update_manufacturer', {
            // ...
            isDisabled: isDisabled
        });
    } else {
        await invoke('add_manufacturer', {
            // ...
            isDisabled: isDisabled === 1 ? isDisabled : null
        });
    }
}
```

---

## ステップ7: 多言語対応

### 7.1 翻訳リソースの追加

```javascript
// res/locales/ja.json
{
  "common": {
    "is_disabled": "非表示",
    "show_disabled": "非表示項目を表示",
    "hide_disabled": "非表示項目を隠す",
    "disabled_label": "非表示"
  }
}

// res/locales/en.json
{
  "common": {
    "is_disabled": "Disabled",
    "show_disabled": "Show Disabled Items",
    "hide_disabled": "Hide Disabled Items",
    "disabled_label": "Disabled"
  }
}
```

---

## テストの更新

### Backend テスト

```rust
#[tokio::test]
async fn test_add_manufacturer_with_is_disabled() {
    let pool = setup_test_db().await;

    let request = AddManufacturerRequest {
        manufacturer_name: "テストメーカー".to_string(),
        memo: None,
        is_disabled: Some(1),  // 非表示で追加
    };

    add_manufacturer(&pool, 2, request).await.unwrap();

    // 通常の取得では表示されない
    let manufacturers = get_manufacturers(&pool, 2, false).await.unwrap();
    assert_eq!(manufacturers.len(), 0);

    // include_disabled=true では取得できる
    let manufacturers_all = get_manufacturers(&pool, 2, true).await.unwrap();
    assert_eq!(manufacturers_all.len(), 1);
    assert_eq!(manufacturers_all[0].is_disabled, 1);
}
```

---

## デザインガイドライン

### カラーパレット

| 要素 | カラーコード | 用途 |
|------|-------------|------|
| 背景色（非表示行） | `#6c757d` | ダークグレー（Medium Gray） |
| テキスト色（非表示行） | `#ffffff` | 白（高コントラスト） |
| バッジ色 | `#ffc107` | 黄色（警告色） |

### アクセシビリティ

- **コントラスト比**: 背景 `#6c757d` + テキスト `#ffffff` = 高コントラスト
- **ボタン**: 非表示行でも操作ボタンは通常表示（opacity適用なし）
- **視覚的フィードバック**: バッジで明確に非表示状態を示す

---

## チェックリスト

実装時に確認すべき項目：

### Backend
- [ ] `GET_ALL_INCLUDING_DISABLED` クエリ追加
- [ ] `INSERT` クエリに `IS_DISABLED` 追加
- [ ] `UPDATE` クエリに `IS_DISABLED` 追加
- [ ] `AddRequest` に `is_disabled: Option<i64>` 追加
- [ ] `UpdateRequest` に `is_disabled: i64` 追加
- [ ] `get` 関数に `include_disabled` パラメータ追加
- [ ] `add` 関数で `is_disabled` のデフォルト値処理
- [ ] Tauri コマンドにパラメータ追加

### Frontend
- [ ] トグルボタン追加（HTML）
- [ ] モーダルにチェックボックス追加（HTML）
- [ ] `showDisabledItems` state 追加（JS）
- [ ] トグルボタンのイベントリスナー（JS）
- [ ] `loadXxx` 関数に `includeDisabled` パラメータ（JS）
- [ ] レンダリング時のスタイリング（JS）
- [ ] バッジ表示（JS）
- [ ] モーダルでの読み書き処理（JS）

### 多言語
- [ ] `common.is_disabled` 追加
- [ ] `common.show_disabled` 追加
- [ ] `common.hide_disabled` 追加
- [ ] `common.disabled_label` 追加

### テスト
- [ ] 非表示項目の追加テスト
- [ ] 非表示項目の更新テスト
- [ ] `include_disabled=false` テスト
- [ ] `include_disabled=true` テスト

---

## 参考実装

完全な実装例は以下を参照してください：

- **Backend**: `src/services/manufacturer.rs`
- **Backend**: `src/services/product.rs`
- **Frontend**: `res/js/manufacturer-management.js`
- **Frontend**: `res/js/product-management.js`
- **HTML**: `res/manufacturer-management.html`
- **HTML**: `res/product-management.html`
- **Commit**: `11269cb` - 全変更内容

---

## トラブルシューティング

### 問題: 非表示項目がクエリに含まれてしまう

**原因**: `include_disabled` パラメータが正しく渡されていない

**解決策**:
1. Tauri コマンドのパラメータ名を確認
2. JavaScript の `invoke` 呼び出しで `includeDisabled` を確認
3. Backend の get 関数でクエリ分岐を確認

### 問題: チェックボックスの状態が保存されない

**原因**: モーダルの保存処理で `is_disabled` が渡されていない

**解決策**:
1. `saveXxx` 関数で `is_disabled` の取得を確認
2. `invoke` の引数に `isDisabled` が含まれているか確認
3. Backend の Request 構造体に `is_disabled` フィールドがあるか確認

### 問題: スタイリングが適用されない

**原因**: `is_disabled` の値チェックが正しくない

**解決策**:
1. `manufacturer.is_disabled === 1` で厳密比較
2. Backend から返される値が `i64` (数値) であることを確認

---

## ベストプラクティス

1. **デフォルト値**: `is_disabled` のデフォルトは 0（有効）
2. **型の一貫性**: Backend は `i64`、Frontend は JavaScript の Number
3. **UI/UX**: 非表示項目は視覚的に明確に区別
4. **アクセシビリティ**: 高コントラスト比を維持
5. **テスト**: 非表示・表示の両方のケースをテスト

---

**変更履歴**
- 2025-11-12: 初版作成
