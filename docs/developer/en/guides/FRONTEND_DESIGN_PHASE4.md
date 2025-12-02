# Phase 4: フロントエンド実装設計書

## 概要

費目管理画面のフロントエンド実装における、UIの動作仕様とバックエンド連携方法を定義する。

---

## 1. 大分類（Category1）の扱い

### 1.1 固定データ
大分類は以下の3つに固定され、ユーザーによる追加・編集・削除は行わない：

1. **支出 (Expense)** - コード: "EXPENSE"
2. **収入 (Income)** - コード: "INCOME"
3. **振替 (Transfer)** - コード: "TRANSFER"

### 1.2 実装方針
- **表示のみ**: ツリーの最上位として表示
- **選択可能**: 中分類追加時の親として選択
- **編集不可**: 追加・編集・削除ボタンは表示しない
- **並び順固定**: 上記の順序で固定

### 1.3 初期データの準備

**データベースへの初期データ登録:**
1. テンプレートユーザー（USER_ID=1）に大分類を登録
2. 各大分類に対応するI18Nレコードも登録
3. `initialize_categories_for_new_user()` で新規ユーザーにコピー

**実装タイミング:**
- Phase 4-0で実施
- データベースに直接SQLでINSERT、または
- 初期化スクリプトを作成

---

## 2. データフロー

### 2.1 初期ロード
```
画面表示
  ↓
loadCategories()
  ↓
invoke('get_category_tree_with_lang', { user_id, lang_code })
  ↓
バックエンドAPI呼び出し
  ↓
CategoryTreeレスポンス受信
  ↓
renderCategoryTree()
  ↓
ツリー表示
  - 大分類: 表示のみ（サブカテゴリ追加ボタンのみ）
  - 中分類: 追加・編集・並び替えボタンあり
  - 小分類: 追加・編集・並び替えボタンあり
```

### 2.2 データ構造

**バックエンドレスポンス (CategoryTree):**
```javascript
[
  {
    category1: {
      user_id: 1,
      category1_code: "EXPENSE",  // 支出
      category1_name: "支出",
      category1_name_i18n: "Expense",
      display_order: 1,
      ...
    },
    children: [  // 中分類のリスト
      {
        category2: {
          category2_code: "C2_01",
          category2_name: "食費",
          category2_name_i18n: "Food",
          ...
        },
        children: [  // 小分類のリスト
          {
            category3_code: "C3_01",
            category3_name: "食料品",
            category3_name_i18n: "Groceries",
            ...
          }
        ]
      }
    ]
  }
]
```

---

## 3. UI操作と動作仕様

### 3.1 カテゴリ一覧表示

**実装関数:** `loadCategories()`, `renderCategoryTree()`

**動作:**
1. ローディング表示
2. バックエンドから現在のユーザーと言語でツリー取得
3. 取得成功 → ツリーレンダリング
4. 取得失敗 → エラーメッセージ表示

**表示ルール:**
- **大分類**: 
  - 操作ボタンなし
  - 「サブカテゴリ追加」ボタンのみ表示（中分類を追加）
- **中分類・小分類**:
  - 「編集」ボタン
  - 「サブカテゴリ追加」ボタン（小分類の場合は中分類のみ）
  - 「↑」「↓」ボタン（並び替え）

**表示名の決定ルール:**
- `category_name_i18n`が存在 → それを表示
- `category_name_i18n`がnull → `category_name`を表示

---

### 3.2 中分類の追加（インライン編集方式）

**トリガー:** 大分類の「サブカテゴリ追加」ボタンクリック

**動作フロー（インライン編集）:**
```
「サブカテゴリ追加」ボタンクリック
  ↓
リストの末尾に新規行を追加
  - デフォルトラベル: "新規費目"
  - ラベル文字列を選択状態にする
  - フォーカスを設定
  ↓
ユーザーが費目名を入力
  ↓
Enterキー押下 または フォーカスアウト
  ↓
handleCategory2Save()
  ↓
バリデーション
  - 日本語名必須チェック
  - 空白のみの場合は追加をキャンセル
  ↓
invoke('add_category2', { 
  user_id,
  category1_code: parent.category1_code,  // "EXPENSE" など
  category2_code: generateCode(),  // "C2_YYYYMMDDHHMMSS"
  name: name_ja  // ユーザーが入力した名前
})
  ↓
成功:
  - 仮の行を実データに置き換え
  - または loadCategories() でツリー再読み込み
失敗:
  - エラーメッセージ表示
  - 仮の行を削除
  ↓
次の操作待ち
```

**UX設計のポイント:**
1. **即座に編集可能**: モーダルを開く手間がない
2. **視覚的フィードバック**: 追加された行が見える
3. **キャンセル可能**: Escキーで追加をキャンセル
4. **末尾に追加**: リストの最後に追加されるので見つけやすい

**実装の詳細:**
```javascript
function addCategoryInline(parentCategory, level) {
  // 1. 親カテゴリのリストを取得
  const parentElement = document.querySelector(`[data-category-code="${parentCategory.code}"]`);
  const childrenContainer = parentElement.querySelector('.children');
  
  // 2. 新規行を作成
  const newRow = document.createElement('div');
  newRow.className = 'category-item editing';
  newRow.innerHTML = `
    <input type="text" 
           class="inline-edit-input" 
           value="新規費目" 
           data-level="${level}"
           data-parent="${parentCategory.code}">
  `;
  
  // 3. リストの末尾に追加
  childrenContainer.appendChild(newRow);
  
  // 4. input要素にフォーカスし、文字列を全選択
  const input = newRow.querySelector('.inline-edit-input');
  input.focus();
  input.select();
  
  // 5. イベントハンドラを設定
  input.addEventListener('keydown', async (e) => {
    if (e.key === 'Enter') {
      await saveInlineCategory(input, parentCategory, level);
    } else if (e.key === 'Escape') {
      cancelInlineEdit(newRow);
    }
  });
  
  input.addEventListener('blur', async () => {
    await saveInlineCategory(input, parentCategory, level);
  });
}

async function saveInlineCategory(input, parentCategory, level) {
  const name = input.value.trim();
  
  // 空白のみの場合はキャンセル
  if (!name || name === '新規費目') {
    cancelInlineEdit(input.closest('.category-item'));
    return;
  }
  
  try {
    const code = generateCategoryCode(level);
    
    if (level === 2) {
      await invoke('add_category2', {
        user_id: currentUserId,
        category1_code: parentCategory.code,
        category2_code: code,
        name: name
      });
    } else if (level === 3) {
      await invoke('add_category3', {
        user_id: currentUserId,
        category1_code: parentCategory.category1_code,
        category2_code: parentCategory.category2_code,
        category3_code: code,
        name: name
      });
    }
    
    // 成功: ツリーを再読み込み
    await loadCategories();
    
  } catch (error) {
    console.error('Failed to add category:', error);
    alert(i18n.t('error.save_failed') + ': ' + error);
    // 仮の行を削除
    cancelInlineEdit(input.closest('.category-item'));
  }
}

function cancelInlineEdit(categoryItem) {
  categoryItem.remove();
}
```

---

### 3.3 中分類の編集

**トリガー:** 中分類の「編集」ボタンクリック

**動作フロー（インライン編集）:**
```
編集ボタンクリック
  ↓
その行を編集モードに切り替え
  - ラベルをinput要素に置き換え
  - 現在の名前を初期値にセット
  - 文字列を全選択
  - フォーカスを設定
  ↓
ユーザーが費目名を編集
  ↓
Enterキー押下 または フォーカスアウト
  ↓
handleCategory2Update()
  ↓
バリデーション
  - 変更がない場合は何もしない
  - 空白のみの場合は元に戻す
  ↓
invoke('update_category2', { 
  user_id, 
  category1_code,
  category2_code,
  name: new_name_ja 
})
  ↓
成功:
  - input要素をラベルに戻す
  - 新しい名前を表示
失敗:
  - エラーメッセージ表示
  - 元の名前に戻す
  ↓
編集モード終了
```

**実装の詳細:**
```javascript
function enableInlineEdit(categoryElement, currentName) {
  const label = categoryElement.querySelector('.category-name');
  
  // ラベルをinputに置き換え
  const input = document.createElement('input');
  input.type = 'text';
  input.className = 'inline-edit-input';
  input.value = currentName;
  
  label.replaceWith(input);
  input.focus();
  input.select();
  
  // イベントハンドラ
  input.addEventListener('keydown', async (e) => {
    if (e.key === 'Enter') {
      await saveInlineEdit(categoryElement, input, currentName);
    } else if (e.key === 'Escape') {
      cancelInlineEdit(categoryElement, input, currentName);
    }
  });
  
  input.addEventListener('blur', async () => {
    await saveInlineEdit(categoryElement, input, currentName);
  });
}

async function saveInlineEdit(categoryElement, input, oldName) {
  const newName = input.value.trim();
  
  // 変更がない、または空白のみ
  if (!newName || newName === oldName) {
    revertToLabel(categoryElement, input, oldName);
    return;
  }
  
  try {
    const categoryData = getCategoryDataFromElement(categoryElement);
    
    if (categoryData.level === 2) {
      await invoke('update_category2', {
        user_id: currentUserId,
        category1_code: categoryData.category1_code,
        category2_code: categoryData.category2_code,
        name: newName
      });
    } else if (categoryData.level === 3) {
      await invoke('update_category3', {
        user_id: currentUserId,
        category1_code: categoryData.category1_code,
        category2_code: categoryData.category2_code,
        category3_code: categoryData.category3_code,
        name: newName
      });
    }
    
    // 成功: ラベルに戻す
    revertToLabel(categoryElement, input, newName);
    
  } catch (error) {
    console.error('Failed to update category:', error);
    alert(i18n.t('error.save_failed') + ': ' + error);
    // 元の名前に戻す
    revertToLabel(categoryElement, input, oldName);
  }
}

function revertToLabel(categoryElement, input, name) {
  const label = document.createElement('span');
  label.className = 'category-name';
  label.textContent = name;
  input.replaceWith(label);
}

function cancelInlineEdit(categoryElement, input, oldName) {
  revertToLabel(categoryElement, input, oldName);
}
```

---

### 3.4 小分類の追加・編集

**小分類の追加:**
- 中分類の「サブカテゴリ追加」ボタンから
- 親として中分類を表示
- `add_category3` API を使用
- その他の動作は中分類と同様

**小分類の編集:**
- 小分類の「編集」ボタンから
- `update_category3` API を使用
- その他の動作は中分類と同様

---

### 3.5 並び順変更

**トリガー:** 各カテゴリ（中分類・小分類）の「↑」「↓」ボタンクリック

**動作フロー:**
```
↑ボタンクリック
  ↓
moveCategoryUp(category, level)
  ↓
楽観的UI更新
  - DOM上で即座に順序を入れ替え
  - ボタンの有効/無効を更新
  ↓
適切なAPIを呼び出し:
  - level=2: invoke('move_category2_order', {..., direction: -1})
  - level=3: invoke('move_category3_order', {..., direction: -1})
  ↓
成功 → そのまま
失敗 → loadCategories() で再読み込み（ロールバック）
```

**楽観的UI更新のメリット:**
- レスポンスが速く感じる
- ユーザー体験向上

**注意点:**
- 大分類は並び替え不可
- 中分類は同じ大分類内でのみ並び替え
- 小分類は同じ中分類内でのみ並び替え

---

## 4. API呼び出し仕様

### 4.1 使用するTauri Commands

| 操作 | コマンド | パラメータ |
|-----|---------|----------|
| ツリー取得 | `get_category_tree_with_lang` | `user_id`, `lang_code` |
| 中分類追加 | `add_category2` | `user_id`, `category1_code`, `category2_code`, `name` |
| 中分類更新 | `update_category2` | `user_id`, `category1_code`, `category2_code`, `name` |
| 中分類並び替え | `move_category2_order` | `user_id`, `category1_code`, `category2_code`, `direction` |
| 小分類追加 | `add_category3` | `user_id`, `category1_code`, `category2_code`, `category3_code`, `name` |
| 小分類更新 | `update_category3` | `user_id`, `category1_code`, `category2_code`, `category3_code`, `name` |
| 小分類並び替え | `move_category3_order` | `user_id`, `category1_code`, `category2_code`, `category3_code`, `direction` |

**注:** 大分類用のAPIは使用しない

### 4.2 多言語名の登録

**当面の対応（Phase 4）:**
- 日本語名のみ管理
- 英語名は自動で日本語名と同じ値をデフォルト設定
- 多言語対応の完全実装は後のフェーズで

---

## 5. エラーハンドリング

### 5.1 ネットワークエラー

```javascript
try {
  await invoke('add_category2', params);
  await loadCategories();
} catch (error) {
  console.error('Failed to add category:', error);
  alert(i18n.t('error.save_failed') + ': ' + error);
  // 仮の行を削除
  cancelInlineEdit(input.closest('.category-item'));
}
```

### 5.2 バリデーションエラー

```javascript
function validateCategoryName(name) {
  if (!name || !name.trim()) {
    return false;
  }
  if (name.length > 128) {
    alert(i18n.t('error.name_too_long'));
    return false;
  }
  return true;
}
```

### 5.3 並び順変更の失敗

```javascript
async function moveCategoryUp(category, level) {
  // 楽観的更新
  applyOptimisticUpdate(category, -1);
  
  try {
    if (level === 2) {
      await invoke('move_category2_order', {
        user_id: currentUserId,
        category1_code: category.category1_code,
        category2_code: category.category2_code,
        direction: -1
      });
    } else if (level === 3) {
      await invoke('move_category3_order', {
        user_id: currentUserId,
        category1_code: category.category1_code,
        category2_code: category.category2_code,
        category3_code: category.category3_code,
        direction: -1
      });
    }
  } catch (error) {
    console.error('Failed to move category:', error);
    alert(i18n.t('error.move_failed'));
    // ロールバック: ツリーを再読み込み
    await loadCategories();
  }
}
```

---

## 6. 実装優先順位

### Phase 4-0: 初期データの準備
1. テンプレートユーザー（USER_ID=1）に大分類を登録
   - EXPENSE（支出）
   - INCOME（収入）
   - TRANSFER（振替）
2. 各大分類のI18Nレコードも登録（日本語・英語）
3. 初期化スクリプトまたはSQLを作成

### Phase 4-1: データ取得と表示 
1. バックエンドAPIとの接続
2. モックデータの削除
3. 実データでのレンダリング
4. 大分類は操作ボタンなしで表示（サブカテゴリ追加ボタンのみ）

### Phase 4-2: 中分類の追加・編集（インライン方式）
1. 「サブカテゴリ追加」ボタンで末尾に新規行追加
2. デフォルト名"新規費目"を全選択状態で表示
3. Enter/Blur時に保存、Escでキャンセル
4. 編集ボタンでインライン編集モードに切り替え

### Phase 4-3: 小分類の追加・編集
1. 中分類と同じインライン編集方式
2. レベルに応じたAPI切り替え

### Phase 4-4: 並び順変更
1. 楽観的UI更新の実装
2. エラー時のロールバック

### Phase 4-5: UI調整とエラーハンドリング
1. ボタンの有効/無効制御
2. エラーメッセージの多言語対応
3. ローディング表示の改善

---

## 7. コード生成ロジック

**フロントエンドで生成（簡易版）:**
```javascript
function generateCategoryCode(level) {
  const prefix = level === 2 ? 'C2_' : 'C3_';
  const timestamp = Date.now();
  return `${prefix}${timestamp}`;
}
```

---

## 8. 未解決事項・要検討事項

### 8.1 多言語名の完全対応
- **現状:** Phase 4では日本語名のみ実装
- **将来:** 英語名入力欄を追加し、I18N登録APIを実装

### 8.2 削除機能
- **現状:** UIに削除ボタンはない
- **理由:** 設計方針により、通常は削除しない
- **確認済み:** 削除UIは実装しない

### 8.3 大分類の初期データ
- **Phase 4-0で実装:** テンプレートユーザーに3つの大分類を登録

---

## 9. 次のアクション

1. **Phase 4-0の実装** - 大分類の初期データ登録
2. **Phase 4-1の実装** - バックエンドAPIとの接続、実データ表示
3. **Phase 4-2の実装** - 中分類のインライン追加・編集

---

作成日: 2025-10-28
更新日: 2025-10-28（インライン編集方式に変更、初期データ準備を明文化）
