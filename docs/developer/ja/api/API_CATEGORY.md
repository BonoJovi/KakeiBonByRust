# 費目管理API リファレンス

**最終更新**: 2025-12-05 01:51 JST

## 概要

本ドキュメントは、費目管理画面（category-management.html）で使用されるAPIの仕様を定義します。3階層のカテゴリ構造（大分類・中分類・小分類）を管理します。

---

## 目次

1. [カテゴリツリー取得API](#カテゴリツリー取得api)
2. [中分類管理API](#中分類管理api)
3. [小分類管理API](#小分類管理api)
4. [データ構造](#データ構造)

---

## カテゴリツリー取得API

### get_category_tree_with_lang

多言語名を含む完全なカテゴリツリー（3階層）を取得します。

**パラメータ:**
- `user_id` (i64): ユーザーID
- `lang_code` (Option<String>): 言語コード（"ja", "en"など）

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
    userId: 1, 
    langCode: 'ja' 
});
```

**注意:**
- 大分類（Category1）は固定で3つ：EXPENSE（支出）、INCOME（収入）、TRANSFER（振替）
- ユーザー作成時に自動投入される
- 大分類は追加・編集・削除不可

---

## 中分類管理API

### add_category2

新しい中分類を追加します。

**パラメータ:**
- `category1_code` (String): 親となる大分類コード（"EXPENSE", "INCOME", "TRANSFER"）
- `name_ja` (String): 日本語名
- `name_en` (String): 英語名

**戻り値:**
- `String`: 生成されたカテゴリコード（例: "C2_E_21"）

**使用例:**
```javascript
try {
    const code = await invoke('add_category2', {
        category1Code: 'EXPENSE',
        nameJa: '日用品',
        nameEn: 'Daily Necessities'
    });
    console.log(`中分類を追加しました: ${code}`);
} catch (error) {
    alert(`追加失敗: ${error}`);
}
```

**自動処理:**
1. カテゴリコードの自動生成（C2_E_1, C2_E_2...）
2. 表示順の自動設定（最大値+1）
3. I18Nテーブルへの登録（日本語・英語）
4. is_disabled = 0（有効）

**バリデーション:**
- 同一親での名前重複チェック

**エラー:**
- `"Category name '...' already exists"`: 名前が重複

---

### get_category2_for_edit

編集用に中分類の詳細情報を取得します。

**パラメータ:**
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード

**戻り値:**
- `CategoryForEdit`: 編集用カテゴリ情報

**CategoryForEdit構造:**
```javascript
{
    code: string,
    name_ja: string,
    name_en: string,
    display_order: number
}
```

**使用例:**
```javascript
const category = await invoke('get_category2_for_edit', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1'
});

// フォームに値を設定
document.getElementById('name-ja').value = category.name_ja;
document.getElementById('name-en').value = category.name_en;
```

**注意:**
- セッションユーザーIDを自動取得
- 編集モーダル表示時に使用

---

### update_category2

中分類を更新します。

**パラメータ:**
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード
- `name_ja` (String): 新しい日本語名
- `name_en` (String): 新しい英語名

**戻り値:** なし

**使用例:**
```javascript
await invoke('update_category2', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    nameJa: '食費（更新）',
    nameEn: 'Food (Updated)'
});
```

**動作:**
- CATEGORYテーブルのname更新
- I18Nテーブルの両言語を更新

**バリデーション:**
- 自分自身を除いて名前重複チェック

---

### move_category2_up

中分類を1つ上に移動します。

**パラメータ:**
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード

**戻り値:** なし

**使用例:**
```javascript
await invoke('move_category2_up', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_2'
});
```

**動作:**
- 上の兄弟カテゴリとdisplay_orderを交換
- 既に先頭の場合は何もしない

---

### move_category2_down

中分類を1つ下に移動します。

**パラメータ:**
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード

**戻り値:** なし

**使用例:**
```javascript
await invoke('move_category2_down', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1'
});
```

**動作:**
- 下の兄弟カテゴリとdisplay_orderを交換
- 既に末尾の場合は何もしない

---

## 小分類管理API

### add_category3

新しい小分類を追加します。

**パラメータ:**
- `category1_code` (String): 大分類コード
- `category2_code` (String): 親となる中分類コード
- `name_ja` (String): 日本語名
- `name_en` (String): 英語名

**戻り値:**
- `String`: 生成されたカテゴリコード（例: "C3_127"）

**使用例:**
```javascript
const code = await invoke('add_category3', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    nameJa: '外食',
    nameEn: 'Dining Out'
});
```

**自動処理:**
- add_category2と同様

---

### get_category3_for_edit

編集用に小分類の詳細情報を取得します。

**パラメータ:**
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード
- `category3_code` (String): 小分類コード

**戻り値:**
- `CategoryForEdit`: 編集用カテゴリ情報

**使用例:**
```javascript
const category = await invoke('get_category3_for_edit', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1'
});
```

---

### update_category3

小分類を更新します。

**パラメータ:**
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード
- `category3_code` (String): 小分類コード
- `name_ja` (String): 新しい日本語名
- `name_en` (String): 新しい英語名

**戻り値:** なし

**使用例:**
```javascript
await invoke('update_category3', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1',
    nameJa: '食料品（更新）',
    nameEn: 'Groceries (Updated)'
});
```

---

### move_category3_up

小分類を1つ上に移動します。

**パラメータ:**
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード
- `category3_code` (String): 小分類コード

**戻り値:** なし

**使用例:**
```javascript
await invoke('move_category3_up', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_2'
});
```

---

### move_category3_down

小分類を1つ下に移動します。

**パラメータ:**
- `category1_code` (String): 大分類コード
- `category2_code` (String): 中分類コード
- `category3_code` (String): 小分類コード

**戻り値:** なし

**使用例:**
```javascript
await invoke('move_category3_down', {
    category1Code: 'EXPENSE',
    category2Code: 'C2_E_1',
    category3Code: 'C3_1'
});
```

---

## データ構造

### CategoryTree

```rust
// トップレベル構造（大分類）
pub struct CategoryTree {
    pub category1: Category1,
    pub children: Vec<Category2Tree>,  // 中分類の配列
}

// 中分類レベル
pub struct Category2Tree {
    pub category2: Category2,
    pub children: Vec<Category3>,  // 小分類の配列
}
```

### Category1（大分類）

```rust
pub struct Category1 {
    pub user_id: i64,
    pub category1_code: String,     // "EXPENSE", "INCOME", "TRANSFER"
    pub category1_name: String,     // "支出", "収入", "振替"
    pub category1_name_i18n: String, // 翻訳名
    pub display_order: i64,
    pub is_disabled: bool,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

**固定値:**
- EXPENSE（支出）
- INCOME（収入）
- TRANSFER（振替）

---

### Category2（中分類）

```rust
pub struct Category2 {
    pub user_id: i64,
    pub category1_code: String,
    pub category2_code: String,     // "C2_E_1", "C2_E_2"...
    pub category2_name: String,
    pub category2_name_i18n: String,
    pub display_order: i64,
    pub is_disabled: bool,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

---

### Category3（小分類）

```rust
pub struct Category3 {
    pub user_id: i64,
    pub category1_code: String,
    pub category2_code: String,
    pub category3_code: String,     // "C3_1", "C3_2"...
    pub category3_name: String,
    pub category3_name_i18n: String,
    pub display_order: i64,
    pub is_disabled: bool,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

---

### CategoryForEdit（編集用）

```rust
pub struct CategoryForEdit {
    pub code: String,
    pub name_ja: String,
    pub name_en: String,
    pub display_order: i64,
}
```

**用途:**
- 編集モーダルにデータを表示
- 日本語・英語の両方を取得

---

## エラーハンドリング

### 共通エラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"User not authenticated"` | セッション未認証 | ログインが必要 |
| `"Category name '...' already exists"` | 名前が重複 | 別の名前を使用 |
| `"Failed to add category2: ..."` | 追加エラー | データベース確認 |
| `"Failed to update category2: ..."` | 更新エラー | データベース確認 |
| `"Failed to move category2 up: ..."` | 移動エラー | データベース確認 |

### フロントエンドでのエラーハンドリング例

```javascript
// 中分類追加
async function addCategory2(category1Code, nameJa, nameEn) {
    try {
        const code = await invoke('add_category2', {
            category1Code,
            nameJa,
            nameEn
        });
        
        alert(`中分類を追加しました: ${code}`);
        await reloadCategoryTree();
        return code;
    } catch (error) {
        if (error.includes('already exists')) {
            alert('この名前は既に使用されています');
        } else {
            alert(`エラー: ${error}`);
        }
        return null;
    }
}

// カテゴリ移動
async function moveUp(category1Code, category2Code) {
    try {
        await invoke('move_category2_up', {
            category1Code,
            category2Code
        });
        
        // 楽観的UI更新
        await reloadCategoryTree();
    } catch (error) {
        console.error('移動エラー:', error);
        alert(`移動に失敗しました: ${error}`);
    }
}
```

---

## カテゴリ構造の設計

### 階層構造

```
大分類（Category1）← 固定（3つ）
  ├─ 中分類（Category2）← ユーザー追加可能
  │   ├─ 小分類（Category3）← ユーザー追加可能
  │   └─ 小分類（Category3）
  └─ 中分類（Category2）
      └─ 小分類（Category3）
```

### カテゴリコード体系

**大分類:**
- EXPENSE（支出）
- INCOME（収入）
- TRANSFER（振替）

**中分類:**
- C2_E_1, C2_E_2... （支出の中分類）
- C2_I_1, C2_I_2... （収入の中分類）
- C2_T_1, C2_T_2... （振替の中分類）

**小分類:**
- C3_1, C3_2... （通番のみ）

### デフォルトカテゴリ

ユーザー作成時に以下が自動投入されます：

- **中分類**: 20カテゴリ
- **小分類**: 126カテゴリ
- **I18N**: 日本語・英語の翻訳

**投入処理:**
- `create_general_user`実行時に自動呼び出し
- `populate_default_categories`関数
- `res/sql/default_categories_seed.sql`から読み込み

---

## セキュリティ考慮事項

### ユーザー分離

1. **セッションユーザーID**: 各APIで自動取得（`get_session_user_id`）
2. **データ分離**: 各ユーザーは自分のカテゴリのみアクセス可能
3. **大分類の固定**: 悪意ある操作を防止

### 名前の一意性

1. **同一階層・同一親での重複不可**
2. **編集時は自分自身を除外してチェック**
3. **I18Nテーブルも同時に管理**

### カスケード削除

1. **中分類削除**: 子の小分類も削除（未実装）
2. **ユーザー削除**: 全カテゴリが削除
3. **外部キー制約**: データ整合性を保証

---

## 使用例：カテゴリ管理画面の実装

### カテゴリツリー表示

```javascript
async function loadCategoryTree() {
    try {
        const user = await invoke('get_current_session_user');
        const lang = localStorage.getItem('language') || 'ja';
        
        const tree = await invoke('get_category_tree_with_lang', {
            userId: user.user_id,
            langCode: lang
        });
        
        renderCategoryTree(tree);
    } catch (error) {
        console.error('ツリー読み込みエラー:', error);
    }
}

function renderCategoryTree(tree) {
    const container = document.getElementById('category-tree');
    container.innerHTML = '';
    
    tree.forEach(cat1 => {
        const cat1Div = createCategory1Element(cat1);
        container.appendChild(cat1Div);
    });
}
```

### 中分類追加モーダル

```javascript
async function handleAddCategory2(event) {
    event.preventDefault();
    
    const category1Code = document.getElementById('category1-code').value;
    const nameJa = document.getElementById('name-ja').value;
    const nameEn = document.getElementById('name-en').value;
    
    try {
        const code = await invoke('add_category2', {
            category1Code,
            nameJa,
            nameEn
        });
        
        alert(`中分類を追加しました: ${code}`);
        closeModal();
        await loadCategoryTree();
    } catch (error) {
        alert(`エラー: ${error}`);
    }
}
```

---

## テストカバレッジ

**CategoryService:**
- ✅ カテゴリツリー取得テスト
- ✅ 中分類追加テスト
- ✅ 小分類追加テスト
- ✅ カテゴリ更新テスト（日英）
- ✅ カテゴリ移動テスト（上下）
- ✅ 名前重複チェック
- ✅ デフォルトカテゴリ投入テスト（20中分類、126小分類）

---

## 関連ドキュメント

### 実装ファイル

- カテゴリサービス: `src/services/category.rs`
- I18Nサービス: `src/services/i18n.rs`
- SQL定義: `src/sql_queries.rs`
- デフォルトデータ: `res/sql/default_categories_seed.sql`
- Tauri Commands: `src/lib.rs`

### その他のAPIリファレンス

- [共通API](./API_COMMON.md) - セッション管理、I18n
- [入出金管理API](./API_TRANSACTION.md) - カテゴリの利用

---

**変更履歴:**
- 2025-10-28: 初版作成
- 2025-12-05: 実装コードに基づいて全面改訂
  - 未実装のCategory1 APIを削除
  - get_category2_for_edit、get_category3_for_editを追加
  - 新しいテンプレートに統一
  - パラメータ名をcamelCaseに修正
