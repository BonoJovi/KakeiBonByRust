# 開発者ガイド (Developer Guide)

## 目次
1. [プロジェクト概要](#プロジェクト概要)
2. [開発環境のセットアップ](#開発環境のセットアップ)
3. [プロジェクト構成](#プロジェクト構成)
4. [アーキテクチャ](#アーキテクチャ)
5. [コーディング規約](#コーディング規約)
6. [定数管理のベストプラクティス](#定数管理のベストプラクティス)
7. [データベース接続パターン](#データベース接続パターン)
8. [テストとプロダクションの分離](#テストとプロダクションの分離)
9. [ビルドとテスト](#ビルドとテスト)
10. [デバッグ方法](#デバッグ方法)
11. [トラブルシューティング](#トラブルシューティング)

---

## プロジェクト概要

**KakeiBon** は、Rust + Tauri v2 + SQLiteで構築された家計簿アプリケーションです。

### 技術スタック
- **フロントエンド**: Vanilla JavaScript (ES6 Modules), HTML5, CSS3
- **バックエンド**: Rust 1.77.2+, Tauri v2.8.5
- **データベース**: SQLite 3
- **主要ライブラリ**:
  - `rusqlite`: SQLiteデータベース操作
  - `serde/serde_json`: JSONシリアライゼーション
  - `argon2`: パスワードハッシュ化
  - `aes-gcm`: データ暗号化
  - `chrono`: 日時処理

### 主要機能
- ユーザー管理（管理者・一般ユーザー）
- 多言語対応（日本語・英語）
- カスタマイズ可能なフォントサイズ
- 階層的な費目管理（大分類・中分類・小分類）
- セキュアなパスワード管理

---

## 開発環境のセットアップ

### 必要なツール
```bash
# Rustのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (Tauriのフロントエンド依存関係用)
# 推奨: v18以上

# Tauri CLIのインストール
cargo install tauri-cli

# SQLite3 (デバッグ用)
sudo apt install sqlite3  # Ubuntu/Debian
brew install sqlite3      # macOS
```

### プロジェクトのクローンとセットアップ
```bash
git clone <repository-url>
cd KakeiBonByRust

# 依存関係のインストール
cargo build

# 開発モードで実行
cargo tauri dev
```

---

## プロジェクト構成

```
KakeiBonByRust/
├── src/                    # フロントエンドソース
│   ├── index.html          # メインHTML
│   ├── js/                 # JavaScriptモジュール
│   │   ├── main.js
│   │   ├── i18n.js        # 多言語対応
│   │   ├── user_mgmt.js   # ユーザー管理
│   │   └── category.js    # 費目管理
│   └── css/               # スタイルシート
│
├── src-tauri/             # バックエンドソース (Rust)
│   ├── src/
│   │   ├── main.rs        # アプリケーションエントリポイント
│   │   ├── lib.rs         # Tauriコマンド登録
│   │   ├── consts.rs      # 定数定義
│   │   ├── commands/      # Tauriコマンド実装
│   │   │   ├── mod.rs
│   │   │   ├── i18n.rs
│   │   │   ├── category.rs
│   │   │   └── settings.rs
│   │   ├── db/            # データベースアクセス層
│   │   │   ├── mod.rs
│   │   │   ├── i18n.rs
│   │   │   └── category.rs
│   │   └── models/        # データモデル
│   │       ├── mod.rs
│   │       └── category.rs
│   ├── Cargo.toml         # Rust依存関係
│   └── tauri.conf.json    # Tauri設定
│
├── res/                   # リソースファイル
│   └── sql/
│       └── dbaccess.sql   # データベース初期化SQL
│
├── docs/                  # ドキュメント
│   ├── ja/               # 日本語ドキュメント
│   └── en/               # 英語ドキュメント
│
└── TODO.md               # タスク管理
```

---

## アーキテクチャ

### レイヤー構成

```
┌─────────────────────────────────────┐
│      Frontend (JavaScript)          │
│  - UI レンダリング                   │
│  - ユーザーインタラクション           │
│  - Tauri API 呼び出し                │
└──────────────┬──────────────────────┘
               │ invoke()
               ▼
┌─────────────────────────────────────┐
│    Tauri Commands (lib.rs)          │
│  - commands::i18n::*                │
│  - commands::category::*            │
│  - commands::settings::*            │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│    Database Access Layer (db/)      │
│  - db::i18n::get_all_translations() │
│  - db::category::*                  │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│       SQLite Database               │
│  $HOME/.kakeibon/KakeiBonDB.sqlite3 │
└─────────────────────────────────────┘
```

### データフロー例：言語変更

```
1. ユーザーが言語メニューをクリック (Frontend)
   ↓
2. invoke('set_language', {language: 'en'}) (Frontend)
   ↓
3. commands::i18n::set_language() (Tauri Command)
   ↓
4. KakeiBon.jsonに設定を保存 (Backend)
   ↓
5. Success メッセージを返す (Backend → Frontend)
   ↓
6. invoke('get_translations', {language: 'en'}) (Frontend)
   ↓
7. db::i18n::get_all_translations() (Database Access)
   ↓
8. HashMap<String, String> を返す (Backend → Frontend)
   ↓
9. UIを更新 (Frontend)
```

---

## コーディング規約

### Rust コーディング規約

#### 命名規則
```rust
// 定数: UPPER_SNAKE_CASE
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";

// 関数: snake_case
pub fn get_translations(language: String) -> Result<HashMap<String, String>, String> {
    // ...
}

// 構造体: PascalCase
pub struct Category1 {
    pub user_id: i64,
    pub category1_code: String,
    // ...
}

// 変数: snake_case
let db_path = get_db_path();
```

#### エラーハンドリング
```rust
// Result型を使用
pub fn get_connection() -> Result<Connection, rusqlite::Error> {
    Connection::open(get_db_path())
}

// map_errでエラーを文字列に変換
#[tauri::command]
pub fn get_translations(language: String) -> Result<HashMap<String, String>, String> {
    get_all_translations(&language)
        .map_err(|e| format!("Failed to get translations: {}", e))
}
```

#### ドキュメントコメント
```rust
/// ユーザーIDに紐づく大分類一覧を取得
///
/// # 引数
/// * `user_id` - ユーザーID
///
/// # 戻り値
/// カテゴリ1のリストまたはエラー
pub fn get_category1_list(user_id: i64) -> Result<Vec<Category1>, rusqlite::Error> {
    // 実装
}
```

### JavaScript コーディング規約

#### モジュール構成
```javascript
// ES6 Modules を使用
import { invoke } from '@tauri-apps/api/core';

// 名前空間でグローバル汚染を防ぐ
const CategoryManager = {
    init() { /* ... */ },
    loadCategories() { /* ... */ }
};

export default CategoryManager;
```

#### 非同期処理
```javascript
// async/await を使用
async function loadTranslations(language) {
    try {
        const translations = await invoke('get_translations', { language });
        return translations;
    } catch (error) {
        console.error('Failed to load translations:', error);
        throw error;
    }
}
```

#### 命名規則
```javascript
// クラス: PascalCase
class I18n { }

// 関数: camelCase
function loadCategories() { }

// 定数: UPPER_SNAKE_CASE
const DEFAULT_LANGUAGE = 'ja';
```

---

## 定数管理のベストプラクティス

### 定数の一元管理 (`src-tauri/src/consts.rs`)

プロジェクト全体で使用する定数は `consts.rs` で一元管理します。

```rust
// src-tauri/src/consts.rs

// ユーザーロール定数
pub const ROLE_ADMIN: i64 = 0;
pub const ROLE_USER: i64 = 1;
pub const ROLE_VISIT: i64 = 999;

// データベース定数
pub const DB_DIR_NAME: &str = ".kakeibon";
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";
pub const SQL_INIT_FILE_PATH: &str = "res/sql/dbaccess.sql";

// 言語定数
pub const LANG_ENGLISH: &str = "en";
pub const LANG_JAPANESE: &str = "ja";
pub const LANG_DEFAULT: &str = LANG_JAPANESE;

// フォントサイズ定数
pub const FONT_SIZE_SMALL: &str = "small";
pub const FONT_SIZE_MEDIUM: &str = "medium";
pub const FONT_SIZE_LARGE: &str = "large";
pub const FONT_SIZE_DEFAULT: &str = FONT_SIZE_MEDIUM;
```

### 定数の使用例

```rust
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME, LANG_DEFAULT};

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}

fn get_default_language() -> &'static str {
    LANG_DEFAULT
}
```

### 定数追加時のチェックリスト

- [ ] `consts.rs` に定数を追加
- [ ] 適切な命名規則を使用（UPPER_SNAKE_CASE）
- [ ] 型を明示（`&str`, `i64`, etc.）
- [ ] 関連するドキュメントを更新
- [ ] 既存のハードコードされた値を置き換え

---

## データベース接続パターン

### 基本パターン

```rust
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME};

/// データベースパスを取得
fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}

/// データベース接続を取得
pub fn get_connection() -> Result<Connection> {
    Connection::open(get_db_path())
}
```

### トランザクション処理

```rust
pub fn add_category_with_children(
    user_id: i64,
    category: Category1,
    children: Vec<Category2>
) -> Result<(), rusqlite::Error> {
    let mut conn = get_connection()?;
    let tx = conn.transaction()?;

    // 親カテゴリを追加
    tx.execute(
        "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, ...) VALUES (?1, ?2, ...)",
        params![user_id, category.code],
    )?;

    // 子カテゴリを追加
    for child in children {
        tx.execute(
            "INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, ...) VALUES (?1, ?2, ?3, ...)",
            params![user_id, category.code, child.code],
        )?;
    }

    tx.commit()?;
    Ok(())
}
```

### プリペアドステートメントの使用

```rust
pub fn get_categories_by_user(user_id: i64) -> Result<Vec<Category1>, rusqlite::Error> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT USER_ID, CATEGORY1_CODE, CATEGORY1_NAME, DISPLAY_ORDER 
         FROM CATEGORY1 
         WHERE USER_ID = ?1 
         ORDER BY DISPLAY_ORDER"
    )?;

    let category_iter = stmt.query_map([user_id], |row| {
        Ok(Category1 {
            user_id: row.get(0)?,
            category1_code: row.get(1)?,
            category1_name: row.get(2)?,
            display_order: row.get(3)?,
        })
    })?;

    let mut categories = Vec::new();
    for category in category_iter {
        categories.push(category?);
    }
    Ok(categories)
}
```

### データベース初期化

```rust
use std::fs;

pub fn initialize_database() -> Result<(), String> {
    let db_path = get_db_path();
    
    // データベースディレクトリを作成
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create db directory: {}", e))?;
    }

    // データベースが存在しない場合のみ初期化
    if !db_path.exists() {
        let conn = Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;

        // SQLファイルを読み込んで実行
        let sql = fs::read_to_string("res/sql/dbaccess.sql")
            .map_err(|e| format!("Failed to read SQL file: {}", e))?;

        conn.execute_batch(&sql)
            .map_err(|e| format!("Failed to execute SQL: {}", e))?;
    }

    Ok(())
}
```

---

## テストとプロダクションの分離

### テスト用データベース接続

```rust
#[cfg(test)]
fn get_test_connection() -> Result<Connection> {
    // インメモリデータベースを使用
    let conn = Connection::open_in_memory()?;
    
    // テスト用テーブルを作成
    conn.execute(
        "CREATE TABLE CATEGORY1 (
            USER_ID INTEGER NOT NULL,
            CATEGORY1_CODE VARCHAR(64) NOT NULL,
            DISPLAY_ORDER INTEGER NOT NULL,
            CATEGORY1_NAME VARCHAR(128) NOT NULL,
            PRIMARY KEY(USER_ID, CATEGORY1_CODE)
        )",
        [],
    )?;
    
    Ok(conn)
}
```

### テストの構造

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_category() {
        // Arrange
        let conn = get_test_connection().unwrap();
        
        // Act
        let result = add_category1(&conn, 1, "EXPENSE", "支出", 1);
        
        // Assert
        assert!(result.is_ok());
        
        // Verify
        let categories = get_category1_list(&conn, 1).unwrap();
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].category1_code, "EXPENSE");
    }

    #[test]
    fn test_transaction_rollback() {
        let conn = get_test_connection().unwrap();
        
        // トランザクションが失敗した場合、ロールバックされることを確認
        let result = add_invalid_category(&conn);
        assert!(result.is_err());
        
        let categories = get_category1_list(&conn, 1).unwrap();
        assert_eq!(categories.len(), 0); // データが追加されていないことを確認
    }
}
```

### 環境変数による分岐

```rust
fn get_db_path() -> PathBuf {
    if cfg!(test) {
        // テスト環境では一時ディレクトリを使用
        PathBuf::from("/tmp/kakeibon_test.db")
    } else {
        // プロダクション環境
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
    }
}
```

---

## ビルドとテスト

### 開発モード実行

```bash
# Tauriアプリを開発モードで起動
cargo tauri dev

# バックエンドのみビルド
cd src-tauri
cargo build
```

### テスト実行

```bash
# 全テストを実行
cargo test

# 特定のモジュールのテストのみ実行
cargo test db::category

# テスト出力を詳細表示
cargo test -- --nocapture

# 並列実行を無効化（デバッグ用）
cargo test -- --test-threads=1
```

### プロダクションビルド

```bash
# リリースビルド
cargo tauri build

# ビルド成果物は以下に生成される
# target/release/bundle/
```

### コードフォーマット

```bash
# コードフォーマット
cargo fmt

# フォーマットチェックのみ（CIで使用）
cargo fmt -- --check
```

### Lintチェック

```bash
# Clippy でコード品質チェック
cargo clippy

# 厳格モード
cargo clippy -- -D warnings
```

---

## デバッグ方法

### ログ出力

```rust
use log::{info, warn, error, debug};

pub fn some_function() {
    debug!("Debug message");
    info!("Info message");
    warn!("Warning message");
    error!("Error message");
}
```

### フロントエンドからのデバッグ

```javascript
// コンソールログ
console.log('Debug info:', data);
console.error('Error:', error);

// Tauri コマンドのエラーをキャッチ
try {
    const result = await invoke('some_command', { param: value });
    console.log('Success:', result);
} catch (error) {
    console.error('Command failed:', error);
}
```

### データベースのデバッグ

```bash
# SQLiteデータベースに直接接続
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3

# テーブル一覧
.tables

# スキーマ確認
.schema CATEGORY1

# データ確認
SELECT * FROM I18N_RESOURCES WHERE LANG_CODE = 'ja' LIMIT 10;

# 終了
.quit
```

### ブレークポイントを使用したデバッグ

```rust
// dbg!マクロを使用
let result = dbg!(some_function());

// panic!でクラッシュさせて調査
panic!("Debug point: value = {:?}", value);
```

---

## トラブルシューティング

### よくある問題と解決策

#### 1. データベース接続エラー

**問題**: `Failed to open database file`

**原因**: データベースファイルまたはディレクトリが存在しない

**解決策**:
```bash
# ディレクトリを作成
mkdir -p ~/.kakeibon

# データベースを初期化
sqlite3 ~/.kakeibon/KakeiBonDB.sqlite3 < res/sql/dbaccess.sql
```

#### 2. ビルドエラー

**問題**: `error: linker 'cc' not found`

**原因**: C コンパイラがインストールされていない

**解決策**:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# macOS
xcode-select --install
```

#### 3. テスト失敗

**問題**: テストが不規則に失敗する

**原因**: 並列実行による競合

**解決策**:
```bash
# シングルスレッドで実行
cargo test -- --test-threads=1
```

#### 4. Tauri コマンドが呼び出せない

**問題**: `Command not found`

**原因**: コマンドが `lib.rs` に登録されていない

**解決策**:
```rust
// src-tauri/src/lib.rs
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::i18n::get_translations,
            commands::category::get_category_tree,
            // ← 新しいコマンドをここに追加
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### デバッグのヒント

1. **エラーメッセージを注意深く読む**: Rustのエラーメッセージは詳細で有用
2. **ログを活用する**: `log` クレートで適切にログを出力
3. **小さく分けてテストする**: 問題を分離して特定
4. **ドキュメントを参照する**: Tauri、rusqlite のドキュメントを確認
5. **コミュニティに質問する**: GitHub Issues、Discord で質問

---

## 参考リンク

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [rusqlite Documentation](https://docs.rs/rusqlite/latest/rusqlite/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)

---

最終更新: 2025-10-28
