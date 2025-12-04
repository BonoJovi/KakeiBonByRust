# データベース設計

**最終更新**: 2025-12-05 03:59 JST  
**対象バージョン**: v0.1.0

---

## 概要

KakeiBonは、SQLiteデータベースを使用して家計簿データを管理します。このドキュメントでは、データベーススキーマの設計、テーブル構造、インデックス戦略について説明します。

---

## データベース構成

### 基本情報
- **DBMS**: SQLite 3
- **ファイル配置**: `$HOME/.kakeibo/kakeibo.db`
- **文字コード**: UTF-8
- **外部キー制約**: 有効化必須（`PRAGMA foreign_keys = ON`）

### アクセス方法
```bash
# 本番DBへのアクセス
./db.sh

# スキーマ確認
./db.sh .schema

# テーブル一覧
./db.sh .tables
```

---

## テーブル一覧

### コアテーブル
| テーブル名 | 用途 | 特記事項 |
|-----------|------|---------|
| USERS | ユーザー情報 | パスワードハッシュ、暗号化マスターキー |
| ACCOUNTS | 口座情報 | ユーザーごとの口座管理 |
| TRANSACTIONS_HEADER | 取引ヘッダー | 取引の基本情報 |
| TRANSACTIONS_DETAIL | 取引明細 | 取引の詳細情報（品目単位） |

### マスタデータテーブル
| テーブル名 | 用途 | 階層 |
|-----------|------|-----|
| CATEGORY1 | カテゴリ（大分類） | 第1階層 |
| CATEGORY2 | カテゴリ（中分類） | 第2階層 |
| CATEGORY3 | カテゴリ（小分類） | 第3階層 |
| SHOPS | 店舗マスタ | - |
| MANUFACTURERS | メーカーマスタ | - |
| PRODUCTS | 商品マスタ | メーカーとの関連 |
| ACCOUNT_TEMPLATES | 口座テンプレート | システム共通 |

### 国際化テーブル
| テーブル名 | 用途 |
|-----------|------|
| I18N_RESOURCES | システムリソース翻訳 |
| CATEGORY1_I18N | カテゴリ1翻訳 |
| CATEGORY2_I18N | カテゴリ2翻訳 |
| CATEGORY3_I18N | カテゴリ3翻訳 |

### その他
| テーブル名 | 用途 |
|-----------|------|
| MEMOS | メモデータ（重複排除） |
| DATA_FIELDS | データフィールド定義 |

---

## 主要テーブル設計

### USERS テーブル

**用途**: ユーザー認証・暗号化情報の管理

| カラム名 | 型 | 制約 | 説明 |
|---------|-----|-----|------|
| USER_ID | INTEGER | PK, AUTOINCREMENT | ユーザーID |
| USERNAME | VARCHAR(255) | NOT NULL, UNIQUE | ユーザー名 |
| PASSWORD_HASH | TEXT | NOT NULL | Argon2ハッシュ値 |
| ROLE | INTEGER | NOT NULL | ロール（0:管理者、1:一般） |
| MASTER_KEY_ENCRYPTED | BLOB | NOT NULL | 暗号化マスターキー（AES-256-GCM） |
| MASTER_KEY_SALT | BLOB | NOT NULL | マスターキー用ソルト |
| MASTER_KEY_NONCE | BLOB | NOT NULL | マスターキー用ノンス |
| IS_DISABLED | INTEGER | DEFAULT 0 | 無効フラグ |
| ENTRY_DT | DATETIME | NOT NULL | 登録日時 |
| UPDATE_DT | DATETIME | | 更新日時 |

**インデックス**:
- `idx_users_username`: `USERNAME`（ログイン高速化）

---

### ACCOUNTS テーブル

**用途**: ユーザーごとの口座管理

| カラム名 | 型 | 制約 | 説明 |
|---------|-----|-----|------|
| ACCOUNT_ID | INTEGER | PK, AUTOINCREMENT | 口座ID |
| USER_ID | INTEGER | NOT NULL, FK → USERS | ユーザーID |
| ACCOUNT_CODE | VARCHAR(50) | NOT NULL | 口座コード |
| ACCOUNT_NAME | TEXT | NOT NULL | 口座名 |
| TEMPLATE_CODE | VARCHAR(50) | NOT NULL, FK → ACCOUNT_TEMPLATES | テンプレートコード |
| INITIAL_BALANCE | INTEGER | DEFAULT 0 | 初期残高 |
| DISPLAY_ORDER | INTEGER | | 表示順序 |
| IS_DISABLED | INTEGER | DEFAULT 0 | 無効フラグ |
| ENTRY_DT | DATETIME | NOT NULL | 登録日時 |
| UPDATE_DT | DATETIME | | 更新日時 |

**制約**:
- `UNIQUE(USER_ID, ACCOUNT_CODE)`: ユーザー内でコード重複不可
- `ON DELETE CASCADE`: ユーザー削除時にカスケード削除

**インデックス**:
- `idx_accounts_user`: `(USER_ID, ACCOUNT_CODE)`
- `idx_accounts_user_order`: `(USER_ID, DISPLAY_ORDER)`
- `idx_accounts_template`: `TEMPLATE_CODE`

---

### TRANSACTIONS_HEADER テーブル

**用途**: 取引の基本情報（1取引1レコード）

| カラム名 | 型 | 制約 | 説明 |
|---------|-----|-----|------|
| TRANSACTION_ID | INTEGER | PK, AUTOINCREMENT | 取引ID |
| USER_ID | INTEGER | NOT NULL, FK → USERS | ユーザーID |
| SHOP_ID | INTEGER | FK → SHOPS | 店舗ID |
| CATEGORY1_CODE | VARCHAR(50) | NOT NULL, FK → CATEGORY1 | 大分類コード |
| FROM_ACCOUNT_CODE | VARCHAR(50) | NOT NULL, FK → ACCOUNTS | 支払元口座 |
| TO_ACCOUNT_CODE | VARCHAR(50) | NOT NULL, FK → ACCOUNTS | 支払先口座 |
| TRANSACTION_DATE | DATETIME | NOT NULL | 取引日時 |
| TOTAL_AMOUNT | INTEGER | NOT NULL | 合計金額 |
| TAX_ROUNDING_TYPE | INTEGER | DEFAULT 0 | 税額丸め方式 |
| TAX_INCLUDED_TYPE | INTEGER | DEFAULT 1, NOT NULL | 税込/税抜 |
| MEMO_ID | INTEGER | FK → MEMOS | メモID |
| IS_DISABLED | INTEGER | DEFAULT 0 | 無効フラグ |
| ENTRY_DT | DATETIME | NOT NULL | 登録日時 |
| UPDATE_DT | DATETIME | | 更新日時 |

**インデックス**:
- `idx_transactions_header_user`: `(USER_ID, TRANSACTION_DATE)` – ユーザー別日付検索
- `idx_transactions_header_accounts`: `(FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE)` – 口座間移動検索
- `idx_transactions_header_category`: `CATEGORY1_CODE` – カテゴリ検索
- `idx_transactions_header_date`: `TRANSACTION_DATE` – 日付範囲検索

---

### TRANSACTIONS_DETAIL テーブル

**用途**: 取引明細（品目単位）

| カラム名 | 型 | 制約 | 説明 |
|---------|-----|-----|------|
| DETAIL_ID | INTEGER | PK, AUTOINCREMENT | 明細ID |
| TRANSACTION_ID | INTEGER | NOT NULL, FK → TRANSACTIONS_HEADER | 取引ID |
| USER_ID | INTEGER | NOT NULL | ユーザーID |
| CATEGORY1_CODE | VARCHAR(50) | NOT NULL | 大分類コード |
| CATEGORY2_CODE | VARCHAR(50) | NOT NULL | 中分類コード |
| CATEGORY3_CODE | VARCHAR(50) | NOT NULL | 小分類コード |
| ITEM_NAME | TEXT | NOT NULL, CHECK ≠ '' | 品目名 |
| AMOUNT | INTEGER | NOT NULL | 金額 |
| TAX_AMOUNT | INTEGER | DEFAULT 0 | 税額 |
| TAX_RATE | INTEGER | DEFAULT 8 | 税率（%） |
| MEMO_ID | INTEGER | FK → MEMOS | メモID |
| ENTRY_DT | DATETIME | NOT NULL | 登録日時 |
| UPDATE_DT | DATETIME | | 更新日時 |

**制約**:
- `ON DELETE CASCADE`: ヘッダー削除時にカスケード削除
- 複合外部キー制約でカテゴリ整合性を保証

**インデックス**:
- `idx_transactions_detail_transaction`: `TRANSACTION_ID` – ヘッダーへの結合
- `idx_transactions_detail_categories`: `(CATEGORY2_CODE, CATEGORY3_CODE)` – カテゴリ検索

---

### CATEGORY1/2/3 テーブル

**用途**: 3階層カテゴリマスタ

**共通カラム**:
| カラム名 | 型 | 制約 | 説明 |
|---------|-----|-----|------|
| USER_ID | INTEGER | PK部分, NOT NULL | ユーザーID |
| CATEGORY1_CODE | VARCHAR(64) | PK部分, NOT NULL | 大分類コード |
| DISPLAY_ORDER | INTEGER | NOT NULL | 表示順序 |
| CATEGORY*_NAME | VARCHAR(128) | NOT NULL | カテゴリ名 |
| IS_DISABLED | INTEGER | DEFAULT 0 | 無効フラグ |
| ENTRY_DT | DATETIME | NOT NULL | 登録日時 |
| UPDATE_DT | DATETIME | | 更新日時 |

**階層構造**:
- CATEGORY2: 追加PK `CATEGORY2_CODE`、FK → CATEGORY1
- CATEGORY3: 追加PK `CATEGORY2_CODE, CATEGORY3_CODE`、FK → CATEGORY2

**カスケード削除**:
- CATEGORY1削除 → CATEGORY2削除 → CATEGORY3削除

**インデックス**:
- 各階層で `(USER_ID, DISPLAY_ORDER)` インデックス
- 親カテゴリへの参照インデックス
- 無効フラグ検索インデックス

---

### MEMOS テーブル

**用途**: メモテキストの重複排除ストア

| カラム名 | 型 | 制約 | 説明 |
|---------|-----|-----|------|
| MEMO_ID | INTEGER | PK, AUTOINCREMENT | メモID |
| USER_ID | INTEGER | NOT NULL, FK → USERS | ユーザーID |
| MEMO_TEXT | TEXT | NOT NULL, CHECK ≠ '' | メモ本文 |
| ENTRY_DT | DATETIME | NOT NULL | 登録日時 |
| UPDATE_DT | DATETIME | | 更新日時 |

**設計意図**:
- 同一テキストの重複保存を防ぐ
- 複数の取引/明細から参照可能
- ユーザー単位で管理

**インデックス**:
- `idx_memos_user`: `USER_ID`
- `idx_memos_text`: `(USER_ID, MEMO_TEXT)` – テキスト検索

---

## インデックス戦略

### 基本方針
1. **主キー**: すべてのテーブルで定義済み（自動インデックス）
2. **外部キー**: 参照頻度が高いものにインデックス
3. **複合インデックス**: WHERE句で頻繁に使う組み合わせ
4. **表示順序**: `DISPLAY_ORDER` を含む検索にインデックス

### パフォーマンス最適化ポイント
- **ユーザー×日付**: 取引検索の最頻パターン
- **カテゴリ階層**: 複合外部キーにインデックス
- **テキスト検索**: MEMO_TEXT、SHOP_NAME等にインデックス

---

## 外部キー制約とカスケード

### カスケード削除の設計
| 親テーブル | 子テーブル | 動作 |
|----------|----------|-----|
| USERS | ACCOUNTS, CATEGORY1, MEMOS | ON DELETE CASCADE |
| CATEGORY1 | CATEGORY2 | ON DELETE CASCADE |
| CATEGORY2 | CATEGORY3 | ON DELETE CASCADE |
| TRANSACTIONS_HEADER | TRANSACTIONS_DETAIL | ON DELETE CASCADE |
| MANUFACTURERS | PRODUCTS | ON DELETE SET NULL |

### 参照整合性
- すべての外部キーで `ON DELETE` アクションを明示的に定義
- アプリケーションレベルでの制約チェックは最小限

---

## データ型の選択

### 数値型
- **INTEGER**: ID、金額、税率、フラグ
  - SQLiteでは64bit整数として扱われる
  - 金額は「円」単位で整数保存（小数点なし）

### 文字列型
- **VARCHAR(n)**: コード系、短い固定長文字列
- **TEXT**: 可変長テキスト（メモ、名称）

### 日時型
- **DATETIME**: ISO 8601形式（例: `2024-12-05 03:59:00`）
  - SQLiteでは文字列として保存
  - デフォルト値: `datetime('now')`

### バイナリ型
- **BLOB**: 暗号化データ、ソルト、ノンス

---

## 国際化対応

### 多言語化テーブル
- `I18N_RESOURCES`: システムメッセージ
- `CATEGORY*_I18N`: カテゴリ名の翻訳

### 設計パターン
- メインテーブル: デフォルト言語（日本語）
- I18Nテーブル: 追加言語のみ
- `LANG_CODE`: 'ja', 'en' 等（ISO 639-1）

---

## セキュリティ考慮事項

### 暗号化データ
- `MASTER_KEY_ENCRYPTED`: AES-256-GCM暗号化
- パスワード派生鍵で暗号化（ユーザーログイン時に復号）

### パスワード管理
- `PASSWORD_HASH`: Argon2idハッシュ
- ソルトは自動生成（Argon2内部）

### アクセス制御
- ユーザー単位でのデータ分離（USER_ID外部キー）
- ROLEによる機能制限（アプリケーション層）

---

## 関連ドキュメント

- [データベース設定ガイド](../guides/DATABASE_CONFIGURATION.md)
- [データベースマイグレーション](../guides/DATABASE_MIGRATION.md)
- [セキュリティ設計](SECURITY_DESIGN.md)
- [アーキテクチャ概要](ARCHITECTURE.md)

---

**Last Updated**: 2025-12-05 03:59 JST
