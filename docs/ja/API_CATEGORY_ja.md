# 費目管理 API ドキュメント

## 概要

本ドキュメントは、KakeiBonの費目管理に関するバックエンドAPIについて説明します。
APIのフロントエンドへの公開は、以下の各項目の説明に準じます。

---

## API一覧

### カテゴリツリー取得

#### `get_category_tree`
ユーザーの完全なカテゴリツリーを取得します。

**パラメータ:**
- `user_id` (i64): ユーザーID

**戻り値:**
- `Vec<CategoryTree>`: カテゴリツリーの配列

**使用例:**
```javascript
const tree = await invoke('get_category_tree', { user_id: 1 });
```

---

#### `get_category_tree_with_lang`
言語別の名前を含む完全なカテゴリツリーを取得します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `lang_code` (Option<String>): 言語コード（例: "ja", "en"）

**戻り値:**
- `Vec<CategoryTree>`: 多言語名を含むカテゴリツリーの配列

**レスポンス構造:**
```javascript
[
  {
    category1: {
      user_id: 1,
      category1_code: "EXPENSE",
      category1_name: "支出",
      category1_name_i18n: "Expense",  // I18Nテーブルから取得
      display_order: 1,
      is_disabled: false,
      entry_dt: "2025-10-28...",
      update_dt: null
    },
    children: [
      {
        category2: {
          user_id: 1,
          category1_code: "EXPENSE",
          category2_code: "C2_E_1",
          category2_name: "食費",
          category2_name_i18n: "Food",
          display_order: 1,
          is_disabled: false,
          entry_dt: "2025-10-28...",
          update_dt: null
        },
        children: [
          {
            user_id: 1,
            category1_code: "EXPENSE",
            category2_code: "C2_E_1",
            category3_code: "C3_1",
            category3_name: "食料品",
            category3_name_i18n: "Groceries",
            display_order: 1,
            is_disabled: false,
            entry_dt: "2025-10-28...",
            update_dt: null
          }
        ]
      }
    ]
  }
]
```

**使用例:**
```javascript
const tree = await invoke('get_category_tree_with_lang', { 
  user_id: 1, 
  lang_code: "en" 
});
```

---

## 大分類（Category1）のAPI

### `add_category1`
新しい大分類を追加します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `code` (String): カテゴリコード（例: "EXPENSE"）
- `name` (String): カテゴリ名（日本語）

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**注記:**
- 表示順は自動的に最大値+1に設定されます
- 現在の設計では、大分類は固定でUIからこのAPIは使用されません

**使用例:**
```javascript
await invoke('add_category1', { 
  user_id: 1, 
  code: "CUSTOM", 
  name: "カスタム" 
});
```

---

### `update_category1`
大分類の名前を更新します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `code` (String): カテゴリコード
- `name` (String): 新しいカテゴリ名

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**使用例:**
```javascript
await invoke('update_category1', { 
  user_id: 1, 
  code: "EXPENSE", 
  name: "支出（更新）" 
});
```

---

### `move_category1_order`
大分類を上下に移動します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `code` (String): カテゴリコード
- `direction` (i32): -1で上へ、1で下へ

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**動作:**
- 隣接するカテゴリとdisplay_orderを入れ替えます
- すでに先頭/末尾の場合は変更されません

**使用例:**
```javascript
// 上へ移動
await invoke('move_category1_order', { 
  user_id: 1, 
  code: "EXPENSE", 
  direction: -1 
});
```

---

### `delete_category1`
大分類とその全ての子要素を削除します（CASCADE）。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `code` (String): カテゴリコード

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**注記:**
- これは内部用APIで、UIには公開されません
- ユーザーアカウント削除時のみ使用されます
- 外部キー制約によりCASCADE削除が保証されます

**使用例:**
```javascript
await invoke('delete_category1', { 
  user_id: 1, 
  code: "EXPENSE" 
});
```

---

## 中分類（Category2）のAPI

### `add_category2`
新しい中分類を追加します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `category1_code` (String): 親カテゴリコード
- `category2_code` (String): カテゴリコード（例: "C2_E_1"）
- `name` (String): カテゴリ名

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**注記:**
- 表示順は親カテゴリ内で自動的に最大値+1に設定されます
- コードは親カテゴリ内で一意である必要があります

**使用例:**
```javascript
await invoke('add_category2', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_" + Date.now(),
  name: "新しい費目" 
});
```

---

### `update_category2`
中分類の名前を更新します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `category1_code` (String): 親カテゴリコード
- `category2_code` (String): カテゴリコード
- `name` (String): 新しいカテゴリ名

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**使用例:**
```javascript
await invoke('update_category2', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  name: "食費（更新）" 
});
```

---

### `move_category2_order`
中分類を親カテゴリ内で上下に移動します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `category1_code` (String): 親カテゴリコード
- `category2_code` (String): カテゴリコード
- `direction` (i32): -1で上へ、1で下へ

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**使用例:**
```javascript
await invoke('move_category2_order', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  direction: -1 
});
```

---

### `delete_category2`
中分類とその全ての子要素を削除します（CASCADE）。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `category1_code` (String): 親カテゴリコード
- `category2_code` (String): カテゴリコード

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**注記:**
- 内部用APIで、UIには公開されません
- ユーザーアカウント削除時のみ使用されます

**使用例:**
```javascript
await invoke('delete_category2', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1"
});
```

---

## 小分類（Category3）のAPI

### `add_category3`
新しい小分類を追加します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `category1_code` (String): 大分類コード
- `category2_code` (String): 親カテゴリコード
- `category3_code` (String): カテゴリコード（例: "C3_1"）
- `name` (String): カテゴリ名

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**使用例:**
```javascript
await invoke('add_category3', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  category3_code: "C3_" + Date.now(),
  name: "新しい小分類" 
});
```

---

### `update_category3`
小分類の名前を更新します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `category1_code` (String): 大分類コード
- `category2_code` (String): 親カテゴリコード
- `category3_code` (String): カテゴリコード
- `name` (String): 新しいカテゴリ名

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**使用例:**
```javascript
await invoke('update_category3', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  category3_code: "C3_1",
  name: "食料品（更新）" 
});
```

---

### `move_category3_order`
小分類を親カテゴリ内で上下に移動します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `category1_code` (String): 大分類コード
- `category2_code` (String): 親カテゴリコード
- `category3_code` (String): カテゴリコード
- `direction` (i32): -1で上へ、1で下へ

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**使用例:**
```javascript
await invoke('move_category3_order', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  category3_code: "C3_1",
  direction: 1 
});
```

---

### `delete_category3`
小分類を削除します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `category1_code` (String): 大分類コード
- `category2_code` (String): 親カテゴリコード
- `category3_code` (String): カテゴリコード

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**注記:**
- 内部用APIで、UIには公開されません
- ユーザーアカウント削除時のみ使用されます

**使用例:**
```javascript
await invoke('delete_category3', { 
  user_id: 1, 
  category1_code: "EXPENSE",
  category2_code: "C2_E_1",
  category3_code: "C3_1"
});
```

---

## ユーティリティAPI

### `initialize_categories_for_new_user`
テンプレートユーザー（USER_ID=1）から新規ユーザーに費目をコピーして初期化します。

**パラメータ:**
- `user_id` (i64): 新規ユーザーID

**戻り値:**
- `Result<(), String>`: 成功またはエラーメッセージ

**注記:**
- テンプレートユーザーの全Category1/2/3をコピーします
- 全I18Nレコードもコピーします
- 新規ユーザー作成時に自動的に実行されます

**使用例:**
```javascript
await invoke('initialize_categories_for_new_user', { 
  user_id: 2 
});
```

---

## エラーハンドリング

全てのAPIは `Result<T, String>` を返します：
- **成功**: `Ok(value)` - 操作が正常に完了
- **エラー**: `Err(message)` - エラー内容を説明するメッセージ

**一般的なエラーケース:**
1. **データベース接続失敗**
   - メッセージ: "Failed to open database"
   
2. **データが見つからない**
   - メッセージ: "Category not found"
   
3. **外部キー制約違反**
   - メッセージ: "Parent category does not exist"
   
4. **SQL実行エラー**
   - メッセージ: 具体的なSQLiteエラーメッセージ

**フロントエンドでのエラーハンドリング例:**
```javascript
try {
  await invoke('add_category2', params);
  alert('カテゴリを追加しました');
} catch (error) {
  console.error('カテゴリの追加に失敗:', error);
  alert('カテゴリの追加に失敗しました: ' + error);
}
```

---

## データベーススキーマ

### CATEGORY1 テーブル
```sql
CREATE TABLE CATEGORY1 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY1_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE)
);
```

### CATEGORY2 テーブル
```sql
CREATE TABLE CATEGORY2 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY2_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE) 
        REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE) 
        ON DELETE CASCADE
);
```

### CATEGORY3 テーブル
```sql
CREATE TABLE CATEGORY3 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY3_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) 
        REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) 
        ON DELETE CASCADE
);
```

### I18N テーブル
`_I18N` 接尾辞を持つ同様の構造で、言語別の名前を保存します。

---

## コード生成

**フロントエンドでのコード生成（推奨）:**
```javascript
function generateCategoryCode(level) {
  const prefix = level === 2 ? 'C2_' : 'C3_';
  const timestamp = Date.now();
  return `${prefix}${timestamp}`;
}
```

**親カテゴリ別のプレフィックス付きCategory2コード生成:**
```javascript
function generateCategory2Code(category1_code) {
  let prefix = 'C2_';
  if (category1_code === 'EXPENSE') prefix = 'C2_E_';
  else if (category1_code === 'INCOME') prefix = 'C2_I_';
  else if (category1_code === 'TRANSFER') prefix = 'C2_T_';
  
  return prefix + Date.now();
}
```

---

## 設計上の注意事項

### 大分類（Category1）
- **固定セット**: EXPENSE（支出）、INCOME（収入）、TRANSFER（振替）
- **ユーザー変更不可**: ユーザーは追加/編集/削除できません
- **表示のみ**: 中分類の親として使用されます

### 中分類・小分類（Category2/3）
- **ユーザー管理**: ユーザーが自由に追加/編集できます
- **UIから削除不可**: ユーザーアカウント削除時のみ削除されます
- **自動並び順**: 新しいカテゴリは末尾に追加されます
- **インライン編集**: モーダルを使わず直接編集します

### 削除操作
- 削除APIは実装済みですが、UIには公開されません
- ユーザーアカウント削除時のみ使用されます
- CASCADE削除により参照整合性が保証されます

---

## バージョン履歴

- **v1.0** (2025-10-28): 初版APIドキュメント
  - Category1/2/3の完全なCRUD操作
  - 多言語対応のツリー取得
  - 並び順変更機能

---

## 関連ドキュメント

- [フロントエンド設計（Phase 4）](../FRONTEND_DESIGN_PHASE4.md)
- [テスト戦略](../TESTING.md)
- [TODO.md](../../TODO.md)
- [English Version](../API_CATEGORY.md)
