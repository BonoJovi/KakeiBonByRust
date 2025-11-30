# データベース設定ガイド

## 概要

KakeiBonアプリケーションは、ユーザーデータと設定情報をSQLiteデータベースに保存します。このガイドでは、データベースファイルの配置、パス管理、および設定方法について説明します。

## データベースファイルの場所

### 本番環境

正式なデータベースファイルは以下の場所に配置されます：

```
$HOME/.kakeibon/KakeiBonDB.sqlite3
```

- **パス**: `$HOME/.kakeibon/KakeiBonDB.sqlite3`
- **ディレクトリ**: `$HOME/.kakeibon/`
- **ファイル名**: `KakeiBonDB.sqlite3`
- **サイズ**: 約170KB（初期状態）

### テスト環境

テスト実行時はインメモリデータベースを使用します：

```rust
#[cfg(test)]
fn get_test_connection() -> Result<Connection> {
    Connection::open_in_memory()?
}
```

## データベースパス管理

### 定数の定義

データベース関連の定数は`src-tauri/src/consts.rs`で一元管理します：

```rust
// src-tauri/src/consts.rs
pub const DB_DIR_NAME: &str = ".kakeibon";
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";
```

### パス取得ヘルパー関数

すべてのデータベース接続で使用する共通のヘルパー関数：

```rust
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME};

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}
```

**使用例:**

```rust
// src-tauri/src/db/i18n.rs
pub fn get_all_translations(lang_code: &str) -> Result<HashMap<String, String>> {
    let conn = Connection::open(get_db_path())?;
    // ...
}

// src-tauri/src/db/category.rs
pub fn get_connection() -> Result<Connection> {
    Connection::open(get_db_path())
}
```

## データベース接続のベストプラクティス

### ✅ 推奨される方法

```rust
// 1. consts.rsで定数を定義
pub const DB_DIR_NAME: &str = ".kakeibon";
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";

// 2. ヘルパー関数を使用
fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}

// 3. すべての接続でヘルパー関数を使用
let conn = Connection::open(get_db_path())?;
```

### ❌ 避けるべき方法

```rust
// ハードコードされたパス（本番と開発で異なる可能性）
Connection::open("kakeibo.db")?
Connection::open("src-tauri/kakeibo.db")?

// 相対パス（カレントディレクトリに依存）
Connection::open("./database/kakeibo.db")?
```

## テストとプロダクションの分離

### 条件付きコンパイル

テスト環境と本番環境で異なるデータベースを使用します：

```rust
#[cfg(test)]
pub fn get_connection() -> Result<Connection> {
    // テスト: インメモリDB
    Connection::open_in_memory()
}

#[cfg(not(test))]
pub fn get_connection() -> Result<Connection> {
    // 本番: ファイルベースDB
    Connection::open(get_db_path())
}
```

### テストデータのセットアップ

```rust
#[cfg(test)]
fn setup_test_db(conn: &Connection) -> Result<()> {
    // テーブル作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS I18N_RESOURCES (
            RESOURCE_ID INTEGER PRIMARY KEY,
            RESOURCE_KEY VARCHAR(256) NOT NULL,
            LANG_CODE VARCHAR(10) NOT NULL,
            RESOURCE_VALUE TEXT NOT NULL,
            ENTRY_DT DATETIME NOT NULL
        )",
        [],
    )?;
    
    // テストデータ挿入
    conn.execute(
        "INSERT INTO I18N_RESOURCES VALUES (1, 'test.key', 'ja', 'テスト', datetime('now'))",
        [],
    )?;
    
    Ok(())
}

#[test]
fn test_translation() {
    let conn = get_connection().unwrap();
    setup_test_db(&conn).unwrap();
    // テスト実行...
}
```

## データベース初期化

### アプリケーション起動時

アプリケーション起動時に`.kakeibon`ディレクトリとデータベースファイルを初期化します：

```rust
use std::fs;
use std::path::Path;

pub fn initialize_database() -> Result<()> {
    // ディレクトリが存在しない場合は作成
    let home = std::env::var("HOME")?;
    let db_dir = Path::new(&home).join(DB_DIR_NAME);
    
    if !db_dir.exists() {
        fs::create_dir_all(&db_dir)?;
    }
    
    // データベースファイルが存在しない場合は作成
    let db_path = get_db_path();
    if !db_path.exists() {
        let conn = Connection::open(&db_path)?;
        create_tables(&conn)?;
        insert_initial_data(&conn)?;
    }
    
    Ok(())
}
```

## データベーススキーマ

### 主要テーブル

#### USERS
```sql
CREATE TABLE USERS (
    USER_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USERNAME VARCHAR(64) NOT NULL UNIQUE,
    PASSWORD_HASH TEXT NOT NULL,
    ROLE INTEGER NOT NULL DEFAULT 1,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME
);
```

#### I18N_RESOURCES
```sql
CREATE TABLE I18N_RESOURCES (
    RESOURCE_ID INTEGER PRIMARY KEY,
    RESOURCE_KEY VARCHAR(256) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    RESOURCE_VALUE TEXT NOT NULL,
    CATEGORY VARCHAR(64),
    DESCRIPTION VARCHAR(512),
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    UNIQUE(RESOURCE_KEY, LANG_CODE)
);
```

#### CATEGORY1/2/3
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

## バックアップとリストア

### 手動バックアップ

```bash
# バックアップ作成
cp $HOME/.kakeibon/KakeiBonDB.sqlite3 \
   $HOME/.kakeibon/KakeiBonDB.sqlite3.backup.$(date +%Y%m%d_%H%M%S)

# リストア
cp $HOME/.kakeibon/KakeiBonDB.sqlite3.backup.20251028_120000 \
   $HOME/.kakeibon/KakeiBonDB.sqlite3
```

### SQLダンプによるバックアップ

```bash
# バックアップ（SQLダンプ）
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 .dump > backup.sql

# リストア
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 < backup.sql
```

## トラブルシューティング

### データベースファイルが見つからない

```bash
# データベースファイルの存在確認
ls -lh $HOME/.kakeibon/KakeiBonDB.sqlite3

# ディレクトリの確認
ls -la $HOME/.kakeibon/
```

### 権限エラー

```bash
# 権限確認
ls -l $HOME/.kakeibon/KakeiBonDB.sqlite3

# 権限修正
chmod 644 $HOME/.kakeibon/KakeiBonDB.sqlite3
```

### データベースの破損

```bash
# 整合性チェック
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 "PRAGMA integrity_check;"

# 修復（新しいDBに再構築）
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 ".dump" | \
  sqlite3 $HOME/.kakeibon/KakeiBonDB_new.sqlite3
```

## .gitignoreの設定

開発用データベースファイルをGit管理から除外します：

```gitignore
# Development databases
*.db
*.sqlite
*.sqlite3
!schema.sql

# Test databases
test_*.db

# Backup files
*.backup
*.bak

# 注: 本番データベースは$HOME/.kakeibon/にあるため、
# プロジェクト内には存在しないので除外設定不要
```

## 関連ドキュメント

- [トラブルシューティングガイド](./TROUBLESHOOTING.md)
- [I18N実装ガイド](./I18N_IMPLEMENTATION.md)
- [開発者ガイド](./DEVELOPER_GUIDE.md)

---

最終更新: 2025-10-28
