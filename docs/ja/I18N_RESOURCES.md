# 翻訳リソース統計

## 概要

本ドキュメントは、KakeiBonアプリケーションで使用されている国際化(i18n)翻訳リソースの統計情報を提供します。

## 翻訳リソース総数

**合計: 298リソース**

- ユニークなキー数: 約149キー
- サポート言語: 2言語（日本語、英語）

## カテゴリ別内訳

| カテゴリ | リソース数 | SQLファイル | 説明 |
|---------|-----------|-------------|------|
| 基本リソース | 126 | `insert_translation_resources.sql` | ユーザー管理、設定、カテゴリ管理など |
| 口座・入出金 | 76 | `insert_account_transaction_i18n.sql` | 口座管理と入出金関連 |
| 入出金管理 | 30 | `add_transaction_mgmt_i18n.sql` | 入出金管理画面 |
| 店舗管理 | 30 | `insert_shop_i18n.sql` | 店舗管理画面 |
| 入出金モーダル | 36 | `insert_transaction_modal_i18n.sql` | 入出金編集モーダル |

## 主要カテゴリ

翻訳リソースは以下のカテゴリに分類されています：

### 1. ユーザー管理 (user_mgmt)
- ユーザー追加、編集、削除
- パスワード変更
- ロール管理

### 2. カテゴリ管理 (category_mgmt)
- 大分類・中分類の管理
- カテゴリの追加、編集、削除

### 3. 口座管理 (account_mgmt)
- 口座の追加、編集、削除
- 残高管理
- 口座テンプレート

### 4. 入出金管理 (transaction_mgmt)
- 入出金一覧
- 入出金の追加、編集、削除
- フィルター機能

### 5. 店舗管理 (shop_mgmt)
- 店舗の追加、編集、削除
- 店舗一覧
- 表示順管理

### 6. 設定 (settings)
- 言語設定
- フォントサイズ設定
- その他の設定

### 7. 共通 (common)
- ボタンラベル（保存、キャンセル、削除など）
- エラーメッセージ
- 確認ダイアログ

## データベース構造

### I18N_RESOURCES テーブル

```sql
CREATE TABLE I18N_RESOURCES (
    RESOURCE_ID INTEGER PRIMARY KEY,
    RESOURCE_KEY TEXT NOT NULL,
    LANG_CODE TEXT NOT NULL,
    RESOURCE_VALUE TEXT NOT NULL,
    CATEGORY TEXT,
    DESCRIPTION TEXT,
    ENTRY_DT TEXT NOT NULL,
    UPDATE_DT TEXT,
    UNIQUE(RESOURCE_KEY, LANG_CODE)
);
```

### リソースキーの命名規則

リソースキーは以下の形式で命名されています：

```
{category}.{subcategory}.{element}
```

**例:**
- `user_mgmt.title` - ユーザー管理画面のタイトル
- `transaction_mgmt.add_new` - 入出金追加ボタン
- `common.btn.save` - 保存ボタン

## サポート言語

### 現在サポート中
- **日本語** (`ja`) - プライマリ言語
- **英語** (`en`) - セカンダリ言語

### 言語コード
- `LANG_DEFAULT = "ja"` (デフォルト言語)

## 翻訳リソースの使用方法

### フロントエンド (JavaScript)

```javascript
// i18nインスタンスの初期化
await i18n.init();

// 翻訳の取得
const title = i18n.t('user_mgmt.title');

// パラメータ付き翻訳
const message = i18n.t('msg.user_added', { username: 'John' });

// HTML要素への自動適用
<button data-i18n="common.btn.save">保存</button>
```

### バックエンド (Rust)

```rust
// i18nサービスの使用
let i18n = I18nService::new(pool);
let value = i18n.get("user_mgmt.title", "ja").await?;

// パラメータ付き
let message = i18n.get_with_params(
    "msg.user_added",
    "ja",
    &["John"]
).await?;

// 全リソース取得
let translations = i18n.get_all("ja").await?;
```

## リソース追加手順

新しい画面や機能を追加する際の翻訳リソース追加手順：

### 1. SQLファイルの作成

```sql
-- insert_{feature}_i18n.sql
INSERT INTO I18N_RESOURCES (RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT)
VALUES 
('feature.title', 'ja', '機能タイトル', 'feature', datetime('now')),
('feature.title', 'en', 'Feature Title', 'feature', datetime('now'));
```

### 2. データベースへの適用

```bash
sqlite3 ~/.local/share/kakeibo/kakeibo.db < sql/insert_{feature}_i18n.sql
```

### 3. フロントエンドでの使用

```html
<h1 data-i18n="feature.title">機能タイトル</h1>
```

## テストカバレッジ

翻訳システムのテストは以下を含みます：

- リソースキーの存在確認
- フォールバック動作（デフォルト言語への切り替え）
- パラメータ置換
- 言語切り替え機能

## 関連ドキュメント

- [I18N実装詳細](./I18N_IMPLEMENTATION.md)
- [動的言語メニュー](./DYNAMIC_LANGUAGE_MENU.md)
- [ユーザー管理](./USER_MANAGEMENT.md)

## 統計情報の更新

このドキュメントの統計は以下のコマンドで更新できます：

```bash
# 各SQLファイルのリソース数を確認
for file in sql/insert_*.sql sql/add_*_i18n.sql; do
  echo "$file: $(grep -c 'RESOURCE_KEY\|VALUES' $file)"
done
```

---

**最終更新日**: 2025-11-10 JST  
**総リソース数**: 298  
**サポート言語**: 日本語、英語
