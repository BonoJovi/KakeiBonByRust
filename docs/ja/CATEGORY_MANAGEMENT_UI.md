# 費目管理画面 実装ドキュメント

## 概要

本ドキュメントは、KakeiBonの費目管理画面（Phase 4-1～4-3）の実装内容を記録します。

**実装期間**: 2025-10-28 ～ 2025-10-31  
**最終更新**: 2025-11-02 21:41 JST

---

## Phase 4-1: カテゴリ一覧表示 ✅

### 実装内容

#### 基本機能
- **ツリー構造表示**: 大分類→中分類→小分類の階層構造
- **実データ連携**: `get_category_tree_with_lang` APIから取得
- **展開/折りたたみ**: クリック可能な展開アイコン（▶/▼）
- **多言語対応**: 現在の言語に応じた名称表示

#### UI/UXの改善

##### 1. レイアウト改善
- **名称の横方向配置**: カテゴリ名を最大限横方向に表示
- **ボタンの次行配置**: 操作ボタンは名称の次行に右寄せで配置（`flex-basis: 100%`使用）
- **改行制御**: カテゴリ名の不自然な文字単位の改行を防止（`word-break: keep-all`）

##### 2. 展開アイコンの改善
- **大型化**: アイコンコンテナを20px × 20px（`2em × 2em`）に拡大
- **文字サイズ調整**: 
  - `▶` (collapsed): `font-size: 1.25em`
  - `▼` (expanded): `font-size: 1.5em`（Unicode文字の視覚的サイズ差を補正）
- **ダブルクリック展開**: 名称をダブルクリックで展開/折りたたみ
- **視認性向上**: アイコンの色とサイズを最適化

##### 3. アクセシビリティ向上
- **フォーカス管理**: `:focus-visible` による賢いフォーカス表示
  - キーボード操作時のみフォーカス枠を表示
  - マウスクリック時は非表示
- **二重枠表示**: フォーカス/ホバー時の視覚フィードバック
  - 2pxの濃いグレー枠（#444）
  - 同時に1要素のみ表示（他はクリア）
- **ホバー時の動作**: 他の要素のフォーカス枠を一時非表示
  - マウスアウト時に自動復帰

#### 技術的な実装

**ファイル構成**:
- `res/category-management.html`: 画面HTML
- `res/js/category-management.js`: ビジネスロジック（801行）
- `res/css/category-management.css`: スタイル定義（319行）

**主要な関数**:
```javascript
// ツリーデータの取得と表示
async loadCategoryTree()

// 各レベルのレンダリング
renderCategory1(cat1)
renderCategory2(cat2, parent1Code)
renderCategory3(cat3, parent1Code, parent2Code)

// 展開/折りたたみ
toggleCategory(level, code)
```

**CSS主要クラス**:
```css
.category-name          /* 名称の基本スタイル */
.category-name.expandable /* 展開可能な名称 */
.expand-icon            /* 展開アイコン */
.expand-icon.expanded   /* 展開状態（▼） */
.expand-icon.collapsed  /* 折りたたみ状態（▶） */
.category-actions       /* 操作ボタンコンテナ（次行配置）*/
.mouse-active           /* ホバー時のマーカー */
```

#### データ構造

**CategoryTree構造**:
```javascript
{
  category1: {
    user_id: 1,
    category1_code: "EXPENSE",
    category1_name: "支出",
    category1_name_i18n: "Expense",
    display_order: 1,
    is_disabled: false
  },
  category2_list: [
    {
      user_id: 1,
      category2_code: "C2_E_1",
      category1_code: "EXPENSE",
      category2_name: "食費",
      category2_name_i18n: "Food",
      display_order: 1,
      is_disabled: false
    }
  ],
  category3_list: [
    {
      user_id: 1,
      category3_code: "C3_1",
      category2_code: "C2_E_1",
      category3_name: "食料品",
      category3_name_i18n: "Groceries",
      display_order: 1,
      is_disabled: false
    }
  ]
}
```

#### 制約事項

1. **大分類の固定**: EXPENSE（支出）、INCOME（収入）、TRANSFER（振替）
   - ユーザーによる追加・編集・削除は不可
   - サブカテゴリ追加のみ可能

2. **初期データ**: ユーザー作成時に自動投入（`populate_default_categories`）
   - 20種類の中分類（CATEGORY2）
   - 126種類の小分類（CATEGORY3）
   - 日本語名は全て投入済み
   - 英語I18Nデータは一部のみ（中分類20件、小分類10件）

---

## Phase 4-2: 中分類の追加・編集 ✅

### 実装内容

#### モーダルダイアログ方式
- **共通Modalクラス**: `res/js/modal.js`（ES Module）
- **フォーカストラップ**: TAB/SHIFT+TABでモーダル内を循環
- **ESCキー対応**: モーダルを閉じる
- **バックドロップクリック**: モーダル外クリックで閉じる

#### 追加機能
**トリガー**: 大分類の「サブカテゴリ追加」ボタン

**入力項目**:
- 名前（日本語）: 必須、重複チェック
- 名前（英語）: 必須、重複チェック

**処理フロー**:
1. モーダル表示（親カテゴリ情報を表示）
2. 入力検証
3. `add_category2` API呼び出し
4. ツリー再読み込み
5. 成功メッセージ表示

#### 編集機能
**トリガー**: 中分類の「編集」ボタン

**処理フロー**:
1. 既存データを取得してモーダルに表示
2. 入力検証（自身を除外した重複チェック）
3. `update_category2` API呼び出し
4. ツリー再読み込み

#### API連携

**追加API**:
```javascript
await invoke('add_category2', {
  userId: this.userId,
  category1Code: parent1Code,
  category2NameJa: nameJa,
  category2NameEn: nameEn
});
```

**編集API**:
```javascript
await invoke('update_category2', {
  userId: this.userId,
  category2Code: code,
  category2NameJa: nameJa,
  category2NameEn: nameEn
});
```

#### バリデーション

**バックエンド（Rust）**:
```sql
-- 追加時の重複チェック
SELECT COUNT(*) FROM CATEGORY2_I18N 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? 
AND LANG_CODE = ? AND CATEGORY2_NAME_I18N = ?

-- 編集時の重複チェック（自身を除外）
SELECT COUNT(*) FROM CATEGORY2_I18N 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? 
AND CATEGORY2_CODE != ? AND LANG_CODE = ? 
AND CATEGORY2_NAME_I18N = ?
```

---

## Phase 4-3: 小分類の追加・編集 ✅

### 実装内容

#### 基本構造
中分類と同様のモーダル方式を採用。

#### 追加機能
**トリガー**: 中分類の「サブカテゴリ追加」ボタン

**入力項目**:
- 名前（日本語）: 必須、重複チェック
- 名前（英語）: 必須、重複チェック

**処理フロー**:
1. モーダル表示（親カテゴリ情報を表示）
2. 入力検証
3. `add_category3` API呼び出し
4. ツリー再読み込み

#### 編集機能
**トリガー**: 小分類の「編集」ボタン

**処理フロー**:
1. 既存データを取得してモーダルに表示
2. 入力検証（自身を除外した重複チェック）
3. `update_category3` API呼び出し
4. ツリー再読み込み

#### 実装の修正内容

**データ属性の追加**:
```javascript
// 小分類ボタンにdata-category3-code属性を追加
button.setAttribute('data-category3-code', cat3.category3_code);
```

**親カテゴリコードの受け渡し**:
```javascript
// renderCategory2からrenderCategory3への受け渡し
renderCategory2(cat2, parent1Code) {
  // ...
  cat2.category3_list.forEach(cat3 => {
    renderCategory3(cat3, parent1Code, cat2.category2_code);
  });
}
```

**イベントハンドラの修正**:
```javascript
// 適切なカテゴリコードを取得
const category3Code = button.dataset.category3Code;
```

#### API連携

**追加API**:
```javascript
await invoke('add_category3', {
  userId: this.userId,
  category2Code: parent2Code,
  category3NameJa: nameJa,
  category3NameEn: nameEn
});
```

**編集API**:
```javascript
await invoke('update_category3', {
  userId: this.userId,
  category3Code: code,
  category3NameJa: nameJa,
  category3NameEn: nameEn
});
```

---

## 共通実装パターン

### レベル定数化
```javascript
const LEVEL_CATEGORY1 = 1;
const LEVEL_CATEGORY2 = 2;
const LEVEL_CATEGORY3 = 3;
```

即値比較を定数比較に変更してコードの可読性を向上。

### エラーハンドリング
```javascript
try {
  await invoke('add_category2', params);
  await this.loadCategoryTree();
  alert(await this.i18n.t('common.save_success'));
} catch (error) {
  console.error('Error adding category:', error);
  alert(await this.i18n.t('common.error_occurred'));
}
```

### モーダル管理
```javascript
// モーダルオープン
this.modal.open();

// モーダルクローズ
this.modal.close();

// ESCキー対応（自動）
// バックドロップクリック対応（自動）
```

---

## パフォーマンス最適化

### ツリー再描画
- 追加・編集後は全体を再読み込み
- 展開状態は保持しない（シンプルな実装を優先）
- 将来的な改善: 展開状態の保存と復元

### DOM操作
- `innerHTML` を使用してシンプルな実装
- イベントデリゲーションを活用（検討中）

---

## テスト

### 手動テスト（完了）
- ✅ 中分類の追加・編集
- ✅ 小分類の追加・編集
- ✅ 重複チェック（追加時・編集時）
- ✅ 言語切り替え
- ✅ 展開/折りたたみ
- ✅ アクセシビリティ（キーボード操作）

### 自動テスト（未実装）
- フロントエンドテストフレームワーク導入後に実装予定
- 対象: モーダル操作、ツリー表示、API連携

---

## 既知の問題

### 英語I18Nデータ不足
- ✅ 日本語名: 全て投入済み（中分類20件、小分類126件）
- ⚠️ 英語I18N: 一部のみ投入済み（中分類20件、小分類10件）
- **対応**: 残りの英語名は将来的にデータ投入スクリプトで補完予定

### 展開アイコンのフォント差異
- `▶` と `▼` の Unicode文字はフォントによって視覚的サイズが異なる
- **対応**: `font-size` を個別調整（▶: 1.25em、▼: 1.5em）で視認性を確保

---

## 今後の拡張予定

### Phase 4-4: 並び順変更

#### 上下移動機能
- [ ] `moveCategoryUp()` の実装
- [ ] `moveCategoryDown()` の実装
- [ ] 楽観的UI更新
- [ ] エラー時のロールバック

#### 並び順リセット機能
- [ ] **「並び順を初期状態に戻す」ボタンの実装**
  
**設計方針**:
- ENTRY_DT（登録日時）順で DISPLAY_ORDER を振り直す
- 登録日時(登録順)を基準とすることで、費目データを自動投入した順番に戻る

**実装方法**:
```sql
-- CATEGORY2の例
WITH ordered AS (
  SELECT USER_ID, CATEGORY1_CODE, CATEGORY2_CODE,
         ROW_NUMBER() OVER (
           PARTITION BY USER_ID, CATEGORY1_CODE 
           ORDER BY ENTRY_DT
         ) as new_order
  FROM CATEGORY2
  WHERE USER_ID = ?
)
UPDATE CATEGORY2 
SET DISPLAY_ORDER = (
  SELECT new_order FROM ordered 
  WHERE ordered.USER_ID = CATEGORY2.USER_ID 
    AND ordered.CATEGORY1_CODE = CATEGORY2.CATEGORY1_CODE
    AND ordered.CATEGORY2_CODE = CATEGORY2.CATEGORY2_CODE
)
WHERE USER_ID = ?;
```

**メリット**:
- ENTRY_DTはレコード毎に一意
- 登録順序が明確に保持される
- テーブル構造の変更不要

### Phase 4-5: UI調整
- [ ] ボタンの有効/無効制御
- [ ] エラーメッセージの多言語対応改善
- [ ] ローディング表示の改善

---

## 参照ドキュメント

- [費目管理 API ドキュメント](API_CATEGORY_ja.md)
- [アクセシビリティインジケータ](ACCESSIBILITY_INDICATORS.md)
- [テスト戦略](TEST_SUMMARY.md)

---

**作成**: 2025-11-02 21:41 JST
