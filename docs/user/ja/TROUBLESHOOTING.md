# トラブルシューティングガイド

KakeiBonアプリケーション使用時に発生する可能性のある問題とその解決方法を記載しています。

## 目次

1. [翻訳リソースが表示されない問題](#翻訳リソースが表示されない問題)
2. [データベース関連の問題](#データベース関連の問題)
3. [起動・動作の問題](#起動動作の問題)
4. [ビルド関連の問題](#ビルド関連の問題)

---

## 翻訳リソースが表示されない問題

### 症状
- メニューやUIの翻訳リソースキー（例：`menu.admin`）がそのまま文字列として表示される
- 一部の翻訳は正しく表示されるが、特定のキーだけが翻訳されない

### 原因の特定手順

#### 1. データベースファイルの確認
まず、正しいデータベースファイルが使用されているか確認します。

```bash
# 正式なデータベースファイルの場所
ls -lh $HOME/.kakeibon/KakeiBonDB.sqlite3

# プロジェクト内に開発用DBが残っていないか確認
find . -name "*.db" -o -name "*.sqlite*" 2>/dev/null | grep -v target
```

#### 2. データベース内の翻訳リソース確認
SQLiteで直接データベースを確認します。

```bash
# 特定のキーが存在するか確認
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 \
  "SELECT RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE 
   FROM I18N_RESOURCES 
   WHERE RESOURCE_KEY = 'menu.admin';"

# menu.で始まるすべてのキーを確認
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 \
  "SELECT RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE 
   FROM I18N_RESOURCES 
   WHERE RESOURCE_KEY LIKE 'menu.%' 
   ORDER BY RESOURCE_KEY, LANG_CODE;"
```

#### 3. コード内のデータベースパス確認
コードが正しいデータベースファイルを参照しているか確認します。

```bash
# データベース接続コードを検索
grep -r "Connection::open" src/db.rs

# データベースパスの定義を確認
grep -r "DB_FILE_NAME\|DB_DIR_NAME" src/consts.rs
```

### よくある原因と解決方法

#### 原因1: 翻訳リソースがデータベースに登録されていない

**確認方法:**
```bash
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 \
  "SELECT COUNT(*) FROM I18N_RESOURCES WHERE RESOURCE_KEY = 'menu.admin';"
```

**解決方法:**
翻訳リソースをデータベースに追加します。

```bash
sqlite3 $HOME/.kakeibon/KakeiBonDB.sqlite3 << 'EOF'
INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, ENTRY_DT) 
VALUES 
  ((SELECT COALESCE(MAX(RESOURCE_ID), 0) + 1 FROM I18N_RESOURCES), 
   'menu.admin', 'ja', '管理', datetime('now')),
  ((SELECT COALESCE(MAX(RESOURCE_ID), 0) + 2 FROM I18N_RESOURCES), 
   'menu.admin', 'en', 'Admin', datetime('now'));
EOF
```

#### 原因2: データベースファイルパスの不整合

**症状:**
- 開発用データベース（`src-tauri/kakeibo.db`等）と本番用データベース（`$HOME/.kakeibon/KakeiBonDB.sqlite3`）が混在
- コードがハードコードされたパスを使用している

**確認方法:**
```rust
// 悪い例（ハードコード）
Connection::open("kakeibo.db")?
Connection::open("src-tauri/kakeibo.db")?

// 良い例（定数使用）
Connection::open(get_db_path())?
```

**解決方法:**
1. `src/consts.rs`で定数を定義
```rust
pub const DB_DIR_NAME: &str = ".kakeibon";
pub const DB_FILE_NAME: &str = "KakeiBonDB.sqlite3";
```

2. データベースパス取得関数を実装（`src/db.rs`）
```rust
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME};

fn get_db_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(DB_DIR_NAME)
        .join(DB_FILE_NAME)
}
```

3. 不要な開発用データベースを削除
```bash
rm -f kakeibo.db src-tauri/kakeibo.db KakeiBonDB.sqlite3
```

#### 原因3: カレントディレクトリの問題

**症状:**
- 相対パスでデータベースを開いているため、実行時のカレントディレクトリによって異なるファイルにアクセスしている

**確認方法:**
```bash
# 実行時のカレントディレクトリを確認
pwd

# データベースファイルが複数存在しないか確認
find . -name "kakeibo.db" -o -name "KakeiBonDB.sqlite3"
```

**解決方法:**
絶対パスまたはHOME基準のパスを使用します（上記の`get_db_path()`関数を参照）。

### デバッグ手法

#### 1. フロントエンドでのデバッグログ追加

```javascript
// res/js/i18n.js
async loadTranslations() {
    console.log('[DEBUG] Loading translations for:', this.currentLanguage);
    try {
        const translations = await invoke('get_translations', { 
            language: this.currentLanguage 
        });
        console.log('[DEBUG] Received translations:', Object.keys(translations).length, 'keys');
        console.log('[DEBUG] Sample keys:', Object.keys(translations).slice(0, 10));
        
        // 特定のキーをチェック
        console.log('[DEBUG] menu.admin:', translations['menu.admin']);
        
        this.translations = translations;
    } catch (error) {
        console.error('[DEBUG] Error loading translations:', error);
    }
}
```

#### 2. バックエンドでのデバッグログ追加

```rust
// src-tauri/src/db/i18n.rs
pub fn get_all_translations(lang_code: &str) -> Result<HashMap<String, String>> {
    let db_path = get_db_path();
    eprintln!("DEBUG: Opening database at: {:?}", db_path);
    
    let conn = Connection::open(&db_path)?;
    
    // レコード数を確認
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM I18N_RESOURCES WHERE LANG_CODE = ?1",
        [lang_code],
        |row| row.get(0)
    )?;
    eprintln!("DEBUG: Found {} translation records for {}", count, lang_code);
    
    let mut stmt = conn.prepare(
        "SELECT RESOURCE_KEY, RESOURCE_VALUE FROM I18N_RESOURCES WHERE LANG_CODE = ?1"
    )?;
    
    let rows = stmt.query_map([lang_code], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    
    let mut translations = HashMap::new();
    for row_result in rows {
        let (key, value) = row_result?;
        if key == "menu.admin" {
            eprintln!("DEBUG: Found menu.admin = {}", value);
        }
        translations.insert(key, value);
    }
    
    eprintln!("DEBUG: Loaded {} translations", translations.len());
    Ok(translations)
}
```

#### 3. ファイルログ出力（一時的なデバッグ用）

```rust
use std::fs::OpenOptions;
use std::io::Write;

fn log_to_file(message: &str) {
    let log_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("work")
        .join("debug.log");
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let _ = writeln!(file, "[{}] {}", 
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), 
            message);
    }
}
```

**注意:** デバッグログは問題解決後に必ず削除してください。

### チェックリスト

問題が発生した場合、以下の順序で確認します：

- [ ] 正式なデータベースファイルが存在するか（`$HOME/.kakeibon/KakeiBonDB.sqlite3`）
- [ ] データベースファイルが空でないか（`ls -lh`でサイズ確認）
- [ ] 翻訳リソースがデータベースに登録されているか（SQL確認）
- [ ] コードが正しいデータベースパスを使用しているか
- [ ] 不要な開発用データベースファイルが残っていないか
- [ ] テスト用のインメモリDBが実ファイルとして作成されていないか
- [ ] ブラウザキャッシュがクリアされているか（Ctrl+Shift+R）

### 予防策

#### 1. データベースパスの統一
すべてのデータベース接続で`get_db_path()`関数を使用します。

#### 2. 定数の一元管理
`src-tauri/src/consts.rs`でデータベース関連の定数を管理します。

#### 3. テストとプロダクションの分離
```rust
#[cfg(test)]
fn get_test_connection() -> Result<Connection> {
    // インメモリDBを使用
    Connection::open_in_memory()
}

#[cfg(not(test))]
pub fn get_connection() -> Result<Connection> {
    // 本番DBを使用
    Connection::open(get_db_path())
}
```

#### 4. データベースファイルの除外
`.gitignore`に追加して、開発用DBをコミットしないようにします。

```gitignore
# 開発用データベース
*.db
*.sqlite
*.sqlite3
!schema.sql

# 本番データベースは$HOME/.kakeibon/にあるため除外不要
```

## 関連ドキュメント

- [ユーザーマニュアル](USER_MANUAL.md)
- [インストールガイド](SETUP_GUIDE.md)
- [FAQ](FAQ.md)
- [開発者向けI18N実装ガイド](../../developer/ja/guides/I18N_IMPLEMENTATION.md)
- [開発者向けデータベース設定ガイド](../../developer/ja/guides/DATABASE_CONFIGURATION.md)

---

## データベース関連の問題

### データベースファイルが見つからない

**症状:**
- アプリ起動時にエラーが発生
- 「データベースに接続できません」というメッセージ

**原因:**
データベースファイルが初期化されていない、または誤った場所を参照している

**解決方法:**

1. データベースディレクトリの確認
```bash
ls -la $HOME/.kakeibon/
```

2. データベースが存在しない場合、アプリを起動すると自動的に初期化されます

3. 権限の確認
```bash
chmod 700 $HOME/.kakeibon
chmod 600 $HOME/.kakeibon/KakeiBonDB.sqlite3
```

### データベースロックエラー

**症状:**
- 「database is locked」エラー
- 操作が完了しない

**原因:**
- 複数のアプリインスタンスが同時にアクセスしている
- 以前のプロセスが正常終了していない

**解決方法:**

1. 実行中のプロセスを確認
```bash
ps aux | grep kakeibon
```

2. 不要なプロセスを終了
```bash
killall kakeibon
```

3. ロックファイルの削除（最終手段）
```bash
rm -f $HOME/.kakeibon/KakeiBonDB.sqlite3-shm
rm -f $HOME/.kakeibon/KakeiBonDB.sqlite3-wal
```

---

## 起動・動作の問題

### アプリが起動しない

**症状:**
- アプリケーションウィンドウが表示されない
- エラーメッセージなしで終了する

**確認項目:**

1. システム要件の確認
```bash
# Rust環境
rustc --version

# Node.js環境（開発時のみ）
node --version
```

2. 依存ライブラリの確認（Linux）
```bash
# 必要なライブラリがインストールされているか
ldd target/release/kakeibon
```

**解決方法:**

- システム要件を満たしていない場合：必要なソフトウェアをインストール
- ライブラリ不足の場合：不足しているライブラリをインストール

### 画面が真っ白になる

**症状:**
- アプリは起動するが画面に何も表示されない
- または一部のUIコンポーネントが表示されない

**原因:**
- フロントエンドのJavaScriptエラー
- リソースファイルの読み込み失敗

**解決方法:**

1. 開発者ツールを開く（開発モード時）
```
Ctrl+Shift+I (Windows/Linux)
Cmd+Option+I (Mac)
```

2. コンソールエラーを確認

3. キャッシュのクリア
```
Ctrl+Shift+R (Windows/Linux)
Cmd+Shift+R (Mac)
```

---

## ビルド関連の問題

### ビルドが失敗する

**症状:**
- `cargo build`が失敗
- 依存関係のエラー

**解決方法:**

1. 依存関係の更新
```bash
cargo clean
cargo update
cargo build --release
```

2. Rustツールチェーンの更新
```bash
rustup update
```

3. キャッシュのクリア
```bash
rm -rf target/
cargo build --release
```

---

## 一般的なチェックリスト

問題が発生した場合、以下を順に確認してください：

### 基本確認
- [ ] 最新版を使用しているか
- [ ] システム要件を満たしているか
- [ ] データベースファイルが正常に存在するか（`$HOME/.kakeibon/KakeiBonDB.sqlite3`）
- [ ] ディスク容量は十分か

### アプリケーション確認
- [ ] 他のインスタンスが起動していないか
- [ ] ブラウザキャッシュをクリアしたか（開発モード時）
- [ ] 翻訳リソースがデータベースに登録されているか

### 開発環境確認（開発者向け）
- [ ] Rust/Cargoが正しくインストールされているか
- [ ] 依存ライブラリがすべてインストールされているか
- [ ] ビルドが正常に完了しているか

---

最終更新: 2024-12-05 05:49 JST
