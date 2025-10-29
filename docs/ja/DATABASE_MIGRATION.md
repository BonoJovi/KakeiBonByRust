# データベースマイグレーションガイド

**最終更新**: 2025-10-29 13:30 JST

## 目次
1. [概要](#概要)
2. [データベース構造](#データベース構造)
3. [初期化プロセス](#初期化プロセス)
4. [翻訳リソースの追加](#翻訳リソースの追加)
5. [スキーマ変更手順](#スキーマ変更手順)
6. [バックアップとリストア](#バックアップとリストア)
7. [マイグレーションスクリプトの管理](#マイグレーションスクリプトの管理)
8. [トラブルシューティング](#トラブルシューティング)

---

## 概要

KakeiBonは**SQLite**データベースを使用しており、データベースファイルは以下の場所に保存されます：

```
$HOME/.kakeibon/KakeiBonDB.sqlite3
```

### マイグレーション戦略

KakeiBonは現在、**シンプルな初期化ベースのアプローチ**を採用しています：

- **初回起動時**: `res/sql/dbaccess.sql`からスキーマを初期化
- **既存データベース**: アプリケーションの起動時にチェック（再初期化はしない）
- **マイグレーション**: 必要に応じて手動でSQLスクリプトを実行

> **注意**: 本格的なマイグレーションツール（例: Refinery, sqlxのマイグレーション機能）は将来的に導入を検討していますが、現時点では実装していません。

---

## データベース構造

### 主要テーブル

| テーブル名 | 説明 | 主キー |
|-----------|------|--------|
| `USERS` | ユーザーアカウント | `USER_ID` |
| `I18N_RESOURCES` | システム翻訳リソース | `RESOURCE_ID` |
| `CATEGORY1` | 大分類（支出/収入/振替） | `(USER_ID, CATEGORY1_CODE)` |
| `CATEGORY2` | 中分類 | `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE)` |
| `CATEGORY3` | 小分類 | `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE)` |
| `CATEGORY1_I18N` | 大分類の多言語名 | `(USER_ID, CATEGORY1_CODE, LANG_CODE)` |
| `CATEGORY2_I18N` | 中分類の多言語名 | `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, LANG_CODE)` |
| `CATEGORY3_I18N` | 小分類の多言語名 | `(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE, LANG_CODE)` |
| `ENCRYPTED_FIELDS` | 暗号化フィールド管理 | `FIELD_ID` |

### 外部キー制約

```sql
-- CATEGORY2 → CATEGORY1
FOREIGN KEY(USER_ID, CATEGORY1_CODE) 
    REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE) 
    ON DELETE CASCADE

-- CATEGORY3 → CATEGORY2
FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) 
    REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) 
    ON DELETE CASCADE
```

カスケード削除により、親カテゴリを削除すると子カテゴリも自動的に削除されます。

---

## 初期化プロセス

### アプリケーション起動時の処理フロー

```rust
// src/db.rs
pub async fn new() -> Result<Self, sqlx::Error> {
    let db_path = get_db_path();  // $HOME/.kakeibon/KakeiBonDB.sqlite3
    
    // ディレクトリが存在しなければ作成
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // データベースに接続（存在しなければ作成）
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());
    let pool = SqlitePool::connect(&db_url).await?;
    
    // WALモードを有効化（パフォーマンス向上）
    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await?;
    
    Ok(Database { pool })
}

pub async fn initialize(&self) -> Result<(), sqlx::Error> {
    // res/sql/dbaccess.sql を読み込んで実行
    let sql_path = get_sql_file_path();  // res/sql/dbaccess.sql
    let sql_content = std::fs::read_to_string(&sql_path)?;
    
    // コメント行を削除して実行
    for statement in sql_content.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() && !trimmed.starts_with("--") {
            sqlx::query(trimmed).execute(&self.pool).await?;
        }
    }
    
    Ok(())
}
```

### 初期化SQL

`res/sql/dbaccess.sql`には以下が含まれます：

1. テーブル作成（`CREATE TABLE IF NOT EXISTS`）
2. インデックス作成（`CREATE INDEX IF NOT EXISTS`）
3. システム翻訳リソースの挿入（`INSERT OR IGNORE`）
4. テンプレートユーザー（USER_ID=1）のカテゴリ初期データ

---

## 翻訳リソースの追加

### システム翻訳リソース（I18N_RESOURCES）

新しいUI要素を追加する際は、翻訳リソースを`res/sql/dbaccess.sql`に追加します。

#### 手順

**1. SQLファイルに追加**

`res/sql/dbaccess.sql`の末尾に以下の形式で追加：

```sql
-- 日本語リソース
INSERT OR IGNORE INTO I18N_RESOURCES (
    RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, 
    CATEGORY, DESCRIPTION, ENTRY_DT
) VALUES (
    'new_feature.button_label',  -- キー（ドット記法）
    'ja',                         -- 言語コード
    'ボタンのラベル',              -- 翻訳テキスト
    'new_feature',                -- カテゴリ（機能名）
    'Button label for new feature',  -- 説明
    datetime('now')               -- 登録日時
);

-- 英語リソース
INSERT OR IGNORE INTO I18N_RESOURCES (
    RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, 
    CATEGORY, DESCRIPTION, ENTRY_DT
) VALUES (
    'new_feature.button_label',
    'en',
    'Button Label',
    'new_feature',
    'Button label for new feature',
    datetime('now')
);
```

**2. 命名規則**

- **キー形式**: `{category}.{subcategory}.{element}`
  - 例: `user_mgmt.add_user`, `category_mgmt.edit_category1`
- **カテゴリ**: 機能単位でグループ化
  - 例: `user_mgmt`, `category_mgmt`, `common`, `error_messages`

**3. 既存データベースへの適用**

新しいリソースを既存のデータベースに追加する場合：

```bash
# 翻訳リソースのみのSQLファイルを作成
cat > add_translations.sql << 'EOF'
INSERT OR IGNORE INTO I18N_RESOURCES ...
EOF

# 既存データベースに適用
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < add_translations.sql
```

**4. フロントエンドでの使用**

```javascript
// res/js/i18n.js経由でアクセス
const label = await i18n.t('new_feature.button_label');
```

### カテゴリ名の多言語化（CATEGORY*_I18N）

費目名を追加・変更する場合は、対応するI18Nテーブルにもレコードを追加します。

```sql
-- 中分類の多言語名を追加
INSERT INTO CATEGORY2_I18N (
    USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, 
    LANG_CODE, CATEGORY2_NAME_I18N, ENTRY_DT
) VALUES
    (1, 'EXPENSE', 'C2_E_FOOD', 'ja', '食費', datetime('now')),
    (1, 'EXPENSE', 'C2_E_FOOD', 'en', 'Food', datetime('now'));
```

---

## スキーマ変更手順

### 新しいテーブルの追加

**1. SQLスクリプトの作成**

```sql
-- sql/migrations/001_add_new_table.sql
CREATE TABLE IF NOT EXISTS NEW_TABLE (
    ID INTEGER PRIMARY KEY,
    NAME VARCHAR(128) NOT NULL,
    ENTRY_DT DATETIME NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_new_table_name ON NEW_TABLE(NAME);
```

**2. dbaccess.sqlに追加**

`res/sql/dbaccess.sql`の適切な位置に上記SQLを追加。

**3. 既存データベースへの適用**

```bash
# 開発環境で確認
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < sql/migrations/001_add_new_table.sql

# テーブルが作成されたか確認
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 ".schema NEW_TABLE"
```

**4. Rustコードの更新**

```rust
// src/models.rs または適切なファイル
#[derive(Debug, Serialize, Deserialize)]
pub struct NewTable {
    pub id: i64,
    pub name: String,
    pub entry_dt: String,
}
```

### カラムの追加

**既存テーブルに新しいカラムを追加する場合：**

```sql
-- 既存データベースへのマイグレーション
ALTER TABLE USERS ADD COLUMN EMAIL VARCHAR(256);
ALTER TABLE USERS ADD COLUMN PHONE VARCHAR(20);

-- dbaccess.sqlも更新（新規インストール用）
CREATE TABLE IF NOT EXISTS USERS (
    USER_ID INTEGER NOT NULL,
    NAME VARCHAR(128) NOT NULL UNIQUE,
    PAW VARCHAR(128) NOT NULL,
    ROLE INTEGER NOT NULL,
    EMAIL VARCHAR(256),          -- 追加
    PHONE VARCHAR(20),            -- 追加
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID)
);
```

> **注意**: SQLiteでは`NOT NULL`制約のあるカラムを`ALTER TABLE`で追加できません。その場合は以下のいずれかの方法を使用します：
> - デフォルト値を指定: `ALTER TABLE USERS ADD COLUMN EMAIL VARCHAR(256) NOT NULL DEFAULT '';`
> - テーブルを再作成する（データ移行が必要）

### カラムの削除・変更

SQLiteは`ALTER TABLE DROP COLUMN`や`ALTER TABLE MODIFY COLUMN`をサポートしていません。
カラムを削除または変更するには、以下の手順でテーブルを再作成します：

```sql
-- 手順1: 新しい構造でテーブルを作成
CREATE TABLE USERS_NEW (
    USER_ID INTEGER NOT NULL,
    NAME VARCHAR(128) NOT NULL UNIQUE,
    PAW VARCHAR(128) NOT NULL,
    ROLE INTEGER NOT NULL,
    -- EMAIL カラムを削除
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID)
);

-- 手順2: データを移行
INSERT INTO USERS_NEW 
SELECT USER_ID, NAME, PAW, ROLE, ENTRY_DT, UPDATE_DT 
FROM USERS;

-- 手順3: 古いテーブルを削除
DROP TABLE USERS;

-- 手順4: 新しいテーブルをリネーム
ALTER TABLE USERS_NEW RENAME TO USERS;

-- 手順5: インデックスを再作成
CREATE INDEX IF NOT EXISTS idx_users_name ON USERS(NAME);
```

**開発環境での実行**:

```bash
# マイグレーションスクリプトを作成
cat > sql/migrations/002_remove_email_column.sql << 'EOF'
-- (上記のSQL)
EOF

# バックアップを作成
cp ~/.kakeibon/KakeiBonDB.sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3.backup

# マイグレーションを実行
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < sql/migrations/002_remove_email_column.sql
```

---

## バックアップとリストア

### バックアップ

**方法1: ファイルコピー（推奨）**

```bash
# データベースファイルを直接コピー
cp ~/.kakeibon/KakeiBonDB.sqlite3 ~/backup/KakeiBonDB_$(date +%Y%m%d_%H%M%S).sqlite3
```

**方法2: SQLダンプ**

```bash
# SQLダンプを作成（テキスト形式）
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 .dump > backup_$(date +%Y%m%d).sql
```

**自動バックアップスクリプト（例）**

```bash
#!/bin/bash
# backup-kakeibo.sh

BACKUP_DIR="$HOME/backup/kakeibon"
DB_PATH="$HOME/.kakeibon/KakeiBonDB.sqlite3"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

mkdir -p "$BACKUP_DIR"
cp "$DB_PATH" "$BACKUP_DIR/KakeiBonDB_$TIMESTAMP.sqlite3"

# 7日より古いバックアップを削除
find "$BACKUP_DIR" -name "KakeiBonDB_*.sqlite3" -mtime +7 -delete

echo "Backup created: $BACKUP_DIR/KakeiBonDB_$TIMESTAMP.sqlite3"
```

**cronで自動実行（毎日2:00 AM）**

```bash
crontab -e

# 以下を追加
0 2 * * * /path/to/backup-kakeibo.sh
```

### リストア

**方法1: ファイルから復元**

```bash
# アプリケーションを終了
# バックアップファイルをコピー
cp ~/backup/KakeiBonDB_20250129.sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3

# WALファイルも削除（一貫性のため）
rm -f ~/.kakeibon/KakeiBonDB.sqlite3-wal
rm -f ~/.kakeibon/KakeiBonDB.sqlite3-shm

# アプリケーションを再起動
```

**方法2: SQLダンプから復元**

```bash
# 既存のデータベースを削除
rm ~/.kakeibon/KakeiBonDB.sqlite3

# ダンプから復元
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < backup_20250129.sql
```

### データ整合性チェック

```bash
# データベースの整合性を確認
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA integrity_check;"

# 期待される出力: ok

# 外部キー制約の整合性を確認
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA foreign_key_check;"

# 出力がなければOK
```

---

## マイグレーションスクリプトの管理

### ディレクトリ構造

```
sql/
├── README.md                           # SQLスクリプトの説明
├── phase4/                             # フェーズ別スクリプト
│   ├── phase4-0_init_categories.sql
│   └── phase4-0_update_category_codes.sql
└── migrations/                         # マイグレーションスクリプト（将来）
    ├── 001_add_new_table.sql
    └── 002_remove_email_column.sql
```

### マイグレーションスクリプトの命名規則

```
{番号}_{説明}.sql
```

- **番号**: 3桁のシーケンス番号（001, 002, ...）
- **説明**: スネークケースで簡潔に（例: `add_email_field`, `update_category_codes`）

### スクリプトの実行順序

マイグレーションスクリプトは番号順に実行します：

```bash
# 手動実行例
for script in sql/migrations/*.sql; do
    echo "Applying $script..."
    sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < "$script"
done
```

### フェーズ別スクリプト

`sql/phase4/`のようなフェーズ別ディレクトリは、開発中の特定機能に関連するスクリプトを保存します。

**用途**:
- 開発環境での初期データセットアップ
- テストデータの投入
- 参照用（実際のマイグレーションには使用しない）

**実行例**:

```bash
# Phase 4のカテゴリ初期化
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < sql/phase4/phase4-0_init_categories.sql
```

> **注意**: Phase 4-0以降、データベースは既に正しい状態になっているため、これらのスクリプトは通常不要です。

---

## トラブルシューティング

### データベースがロックされている

**エラー**:
```
Error: database is locked
```

**原因**: 別のプロセスがデータベースにアクセスしています。

**解決方法**:

```bash
# 1. アプリケーションを完全に終了
# 2. WALファイルをチェック
ls -la ~/.kakeibon/KakeiBonDB.sqlite3*

# 3. 必要に応じてWALをチェックポイント
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA wal_checkpoint(TRUNCATE);"

# 4. それでもロックされている場合、プロセスを確認
lsof ~/.kakeibon/KakeiBonDB.sqlite3
```

### 外部キー制約違反

**エラー**:
```
FOREIGN KEY constraint failed
```

**原因**: 親レコードが存在しない子レコードを作成しようとしています。

**デバッグ方法**:

```bash
# 外部キー制約を確認
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA foreign_keys = ON;"
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA foreign_key_check;"

# 出力例:
# CATEGORY2|1|CATEGORY1|0
# → CATEGORY2テーブルの1行目が参照しているCATEGORY1が存在しない
```

**解決方法**:

```bash
# 孤立したレコードを削除
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 << 'EOF'
-- CATEGORY2で親が存在しないレコードを削除
DELETE FROM CATEGORY2
WHERE NOT EXISTS (
    SELECT 1 FROM CATEGORY1 
    WHERE CATEGORY1.USER_ID = CATEGORY2.USER_ID 
    AND CATEGORY1.CATEGORY1_CODE = CATEGORY2.CATEGORY1_CODE
);
EOF
```

### スキーマが古い

**症状**: 新しいカラムやテーブルが存在しない。

**確認方法**:

```bash
# 現在のスキーマを確認
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 ".schema USERS"

# 期待されるスキーマと比較
cat res/sql/dbaccess.sql | grep -A 10 "CREATE TABLE.*USERS"
```

**解決方法**:

1. **バックアップを作成**
   ```bash
   cp ~/.kakeibon/KakeiBonDB.sqlite3 ~/backup/
   ```

2. **マイグレーションスクリプトを実行**
   ```bash
   sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < sql/migrations/XXX_add_missing_columns.sql
   ```

3. **または、データベースを再作成（データ損失に注意！）**
   ```bash
   rm ~/.kakeibon/KakeiBonDB.sqlite3
   # アプリケーションを起動して再初期化
   ```

### I18Nリソースが見つからない

**症状**: UIに翻訳キーがそのまま表示される（例: `user_mgmt.add_user`）

**原因**: `I18N_RESOURCES`テーブルにリソースが登録されていない。

**確認方法**:

```bash
# リソースが存在するか確認
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 << 'EOF'
SELECT RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE 
FROM I18N_RESOURCES 
WHERE RESOURCE_KEY = 'user_mgmt.add_user';
EOF
```

**解決方法**:

```bash
# 不足しているリソースを追加
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 << 'EOF'
INSERT OR IGNORE INTO I18N_RESOURCES (
    RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, 
    CATEGORY, ENTRY_DT
) VALUES 
    ('user_mgmt.add_user', 'ja', 'ユーザーを追加', 'user_mgmt', datetime('now')),
    ('user_mgmt.add_user', 'en', 'Add User', 'user_mgmt', datetime('now'));
EOF
```

---

## 関連ドキュメント

- [データベース設定ガイド](DATABASE_CONFIGURATION.md) - データベースの基本設定
- [開発者ガイド](DEVELOPER_GUIDE.md) - データベース接続パターン
- [トラブルシューティング](TROUBLESHOOTING.md) - 一般的な問題と解決方法

---

## 将来の改善案

### マイグレーションツールの導入

現在は手動でSQLスクリプトを管理していますが、将来的には以下のツールの導入を検討しています：

**オプション1: sqlxのマイグレーション機能**

```bash
# マイグレーションの作成
sqlx migrate add create_users_table

# マイグレーションの実行
sqlx migrate run --database-url sqlite://~/.kakeibon/KakeiBonDB.sqlite3
```

**オプション2: Refinery**

```rust
use refinery::embed_migrations;

embed_migrations!("migrations");

async fn run_migrations(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    migrations::runner().run_async(pool).await?;
    Ok(())
}
```

**メリット**:
- バージョン管理の自動化
- ロールバック機能
- マイグレーション履歴の追跡

---

**最終更新**: 2025-10-29 13:30 JST
