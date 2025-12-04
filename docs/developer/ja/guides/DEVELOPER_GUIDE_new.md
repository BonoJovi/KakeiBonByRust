# 開発者ガイド (Developer Guide)

**最終更新**: 2024-12-05 04:30 JST

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
12. [参考リンク](#参考リンク)

---

## プロジェクト概要

**KakeiBon** は、Rust + Tauri v2 + SQLiteで構築された個人向け家計簿デスクトップアプリケーションです。

### 技術スタック
- **フロントエンド**: Vanilla JavaScript (ES6 Modules), HTML5, CSS3
- **バックエンド**: Rust 1.77.2+, Tauri v2.1.1
- **データベース**: SQLite 3
- **主要ライブラリ**:
  - `rusqlite`: SQLiteデータベース操作
  - `sqlx`: 非同期SQLiteアクセス
  - `serde/serde_json`: JSONシリアライゼーション
  - `argon2`: パスワードハッシュ化
  - `aes-gcm`: データ暗号化
  - `chrono`: 日時処理

### 主要機能
- **ユーザー管理**: 管理者・一般ユーザーの役割ベースアクセス制御
- **多言語対応**: 日本語・英語の動的切り替え
- **カスタマイズ可能なUI**: フォントサイズ調整、テーマ設定
- **階層的費目管理**: 大分類・中分類・小分類の3階層カテゴリ
- **口座管理**: 複数口座の管理とクローズ機能
- **店舗・メーカー・商品管理**: IS_DISABLED機能付きマスターデータ管理
- **入出金管理**: ヘッダ+詳細の二段階入力、スマート税計算
- **集計機能**: 日別・週別・期間別・年別の多様な集計
- **セキュリティ**: Argon2パスワードハッシュ、AES-256-GCM暗号化

---

## 開発環境のセットアップ

### 必要なツール

#### 1. Rust
```bash
# Rustのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# インストール確認
rustc --version  # 1.77.2以上
cargo --version
```

#### 2. Tauri CLI
```bash
# Tauri CLIのインストール
cargo install tauri-cli

# バージョン確認
cargo tauri --version  # 2.1.1以上推奨
```

#### 3. Node.js (オプション)
```bash
# フロントエンド開発ツール用
# 推奨: v18以上
node --version
npm --version
```

#### 4. SQLite3
```bash
# Ubuntu/Debian
sudo apt install sqlite3

# macOS
brew install sqlite3

# バージョン確認
sqlite3 --version
```

#### 5. 開発ツール (推奨)
```bash
# コードフォーマッタ
rustup component add rustfmt

# Linter
rustup component add clippy
```

### プロジェクトのクローンとセットアップ

```bash
# リポジトリのクローン
git clone <repository-url>
cd KakeiBonByRust

# 依存関係のビルド
cargo build

# 開発モードで実行
cargo tauri dev

# または専用スクリプトを使用
./dev.sh
```

### データベースの初期化

```bash
# 開発用データベースのセットアップ
./db.sh

# これにより以下が実行されます:
# - データベースファイル作成 (kakeibo.db)
# - テーブル作成
# - 初期データ投入（任意）
```

---

## プロジェクト構成

### ディレクトリ構造

```
KakeiBonByRust/
├── src/                        # Rustバックエンド
│   ├── main.rs                 # エントリポイント
│   ├── lib.rs                  # Tauriコマンド登録
│   ├── db.rs                   # データベース接続管理
│   ├── validation.rs           # 入力検証
│   ├── security.rs             # パスワードハッシュ
│   ├── crypto.rs               # 暗号化/復号化
│   ├── settings.rs             # アプリケーション設定
│   ├── sql_queries.rs          # SQL定数管理
│   ├── validation_tests.rs     # 検証テスト
│   ├── test_helpers.rs         # テストヘルパー
│   └── services/               # ビジネスロジック
│       ├── auth.rs             # 認証
│       ├── user_management.rs  # ユーザー管理
│       ├── category.rs         # 費目管理
│       ├── account.rs          # 口座管理
│       ├── shop.rs             # 店舗管理
│       ├── manufacturer.rs     # メーカー管理
│       ├── product.rs          # 商品管理
│       ├── transaction.rs      # 入出金管理
│       ├── aggregation.rs      # 集計機能
│       ├── i18n.rs             # 多言語化
│       ├── encryption.rs       # 暗号化サービス
│       └── session.rs          # セッション管理
│
├── res/                        # フロントエンドリソース
│   ├── index.html              # ログイン/管理者セットアップ画面
│   ├── user-management.html    # ユーザー管理画面
│   ├── category-management.html # 費目管理画面
│   ├── account-management.html # 口座管理画面
│   ├── shop-management.html    # 店舗管理画面
│   ├── manufacturer-management.html # メーカー管理画面
│   ├── product-management.html # 商品管理画面
│   ├── transaction-management.html # 入出金管理画面
│   ├── aggregation.html        # 集計画面
│   ├── js/                     # JavaScriptモジュール
│   │   ├── consts.js           # フロントエンド定数
│   │   ├── i18n.js             # 多言語化
│   │   ├── session.js          # セッション管理
│   │   ├── menu.js             # メニューバー
│   │   ├── font-size.js        # フォントサイズ管理
│   │   ├── modal.js            # モーダルダイアログ
│   │   ├── modal-utils.js      # モーダルユーティリティ
│   │   ├── indicators.js       # ローディング表示
│   │   ├── html-files.js       # HTML遷移管理
│   │   ├── category-management.js
│   │   ├── account-management.js
│   │   ├── shop-management.js
│   │   ├── manufacturer-management.js
│   │   ├── product-management.js
│   │   ├── transaction-management.js
│   │   ├── aggregation.js
│   │   ├── aggregation-common.js
│   │   ├── aggregation-daily.js
│   │   ├── aggregation-weekly.js
│   │   ├── aggregation-period.js
│   │   └── aggregation-yearly.js
│   ├── css/                    # スタイルシート
│   │   ├── common.css          # 共通スタイル
│   │   ├── modal.css           # モーダルスタイル
│   │   └── components.css      # コンポーネントスタイル
│   ├── locales/                # 翻訳リソース
│   │   ├── ja.json             # 日本語
│   │   └── en.json             # 英語
│   └── tests/                  # フロントエンドテスト
│       ├── validation-helpers.js
│       └── *-validation-tests.js
│
├── sql/                        # SQLスクリプト
│   ├── schema.sql              # データベーススキーマ
│   └── seed.sql                # 初期データ（任意）
│
├── docs/                       # ドキュメント
│   ├── developer/              # 開発者向けドキュメント
│   │   ├── ja/                 # 日本語
│   │   │   ├── api/            # API仕様書
│   │   │   ├── design/         # 設計ドキュメント
│   │   │   ├── guides/         # 開発ガイド
│   │   │   └── testing/        # テストドキュメント
│   │   └── en/                 # 英語
│   │       ├── api/
│   │       ├── design/
│   │       ├── guides/
│   │       └── testing/
│   └── user/                   # ユーザー向けドキュメント（将来実装）
│
├── .ai-context/                # AI開発支援コンテキスト
│   ├── CONVENTIONS.md
│   ├── DOCUMENTATION_GUIDELINES.md
│   ├── PROJECT_STRUCTURE.md
│   └── projects-guidelines.md
│
├── Cargo.toml                  # Rust依存関係
├── tauri.conf.json             # Tauri設定
├── build.rs                    # ビルドスクリプト
├── TODO.md                     # タスク管理
├── CHANGELOG.md                # 変更履歴
├── README.md                   # プロジェクト概要
└── db.sh                       # データベース管理スクリプト
```

### 主要モジュールの責務

#### バックエンド (Rust)

| モジュール | 責務 |
|-----------|------|
| `main.rs` | アプリケーションエントリポイント、Tauri初期化 |
| `lib.rs` | Tauriコマンドの登録・エクスポート |
| `db.rs` | SQLite接続管理、トランザクション処理 |
| `validation.rs` | 入力検証ロジック |
| `security.rs` | Argon2パスワードハッシュ |
| `crypto.rs` | AES-256-GCM暗号化/復号化 |
| `services/auth.rs` | ログイン、ログアウト、認証状態管理 |
| `services/user_management.rs` | ユーザーCRUD操作 |
| `services/category.rs` | 費目管理ロジック |
| `services/account.rs` | 口座管理ロジック |
| `services/shop.rs` | 店舗管理ロジック |
| `services/manufacturer.rs` | メーカー管理ロジック |
| `services/product.rs` | 商品管理ロジック |
| `services/transaction.rs` | 入出金管理ロジック |
| `services/aggregation.rs` | 集計機能ロジック |
| `services/i18n.rs` | 多言語化サービス |
| `services/encryption.rs` | 暗号化サービス |
| `services/session.rs` | セッション管理 |

#### フロントエンド (JavaScript)

| モジュール | 責務 |
|-----------|------|
| `i18n.js` | 翻訳リソース読み込み、言語切り替え |
| `session.js` | セッション状態管理、認証確認 |
| `menu.js` | メニューバー、言語切り替え、ログアウト |
| `font-size.js` | フォントサイズ管理、永続化 |
| `modal.js` | 汎用モーダルダイアログ |
| `modal-utils.js` | モーダル表示ヘルパー関数 |
| `indicators.js` | ローディングインジケーター |
| `html-files.js` | 画面遷移定数 |
| `*-management.js` | 各画面の画面固有ロジック |

---

## アーキテクチャ

KakeBonは**レイヤードアーキテクチャ**を採用しています。詳細な設計情報は以下のドキュメントを参照してください：

- **[アーキテクチャ設計](../design/ARCHITECTURE.md)**: 全体構成、レイヤー設計、データフロー
- **[セキュリティ設計](../design/SECURITY_DESIGN.md)**: 認証、暗号化、パスワード管理
- **[データベース設計](../design/DATABASE_DESIGN.md)**: スキーマ、リレーション、制約
- **[UI設計](../design/UI_DESIGN.md)**: 画面構成、コンポーネント、UX方針

### 簡易レイヤー構成

```
┌─────────────────────────────────────┐
│         Presentation Layer          │  res/*.html, res/js/*.js
│  (HTML/CSS/JavaScript - Tauri IPC)  │
└─────────────────────────────────────┘
              ↓ Tauri Commands
┌─────────────────────────────────────┐
│         Application Layer           │  src/lib.rs (command registration)
│      (Tauri Command Handlers)       │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│          Business Logic             │  src/services/*.rs
│     (Validation, Encryption)        │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│         Data Access Layer           │  src/db.rs, src/sql_queries.rs
│       (SQLite Operations)           │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│            Database                 │  kakeibo.db (SQLite)
└─────────────────────────────────────┘
```

---

## コーディング規約

### Rust コーディング規約

#### 命名規則

```rust
// モジュール: snake_case
mod user_management;

// 構造体: PascalCase
pub struct UserInfo {
    pub user_id: i32,
    pub user_name: String,
}

// 列挙型: PascalCase、バリアント: PascalCase
pub enum UserRole {
    Admin,
    User,
}

// 関数: snake_case
pub fn verify_login(user_name: &str, password: &str) -> Result<i32, String> {
    // ...
}

// 定数: UPPER_SNAKE_CASE
pub const ROLE_ADMIN: i32 = 0;
pub const ROLE_USER: i32 = 1;
pub const MIN_PASSWORD_LENGTH: usize = 16;
```

#### エラーハンドリング

```rust
// ❌ Bad: unwrap()の使用 (プロダクションコードでは禁止)
let conn = Connection::open("kakeibo.db").unwrap();

// ✅ Good: Result<T, E>による明示的なエラーハンドリング
pub fn get_user(user_id: i32) -> Result<UserInfo, String> {
    let conn = Connection::open("kakeibo.db")
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    // ...
    
    Ok(user_info)
}

// Tauriコマンドでのエラーハンドリング
#[tauri::command]
pub fn fetch_user(user_id: i32) -> Result<UserInfo, String> {
    get_user(user_id)
}
```

#### ドキュメントコメント

```rust
/// ユーザーログイン認証を実行します。
///
/// # Arguments
/// * `user_name` - ログインユーザー名
/// * `password` - プレーンテキストパスワード（ハッシュ化前）
///
/// # Returns
/// * `Ok(user_id)` - 認証成功時、ユーザーID
/// * `Err(message)` - 認証失敗時、エラーメッセージ
///
/// # Examples
/// ```
/// let result = verify_login("admin", "securepassword123");
/// assert!(result.is_ok());
/// ```
pub fn verify_login(user_name: &str, password: &str) -> Result<i32, String> {
    // implementation
}
```

### JavaScript コーディング規約

#### モジュール構成

```javascript
// ES6 Modulesを使用、拡張子を明示
import { loadTranslations, getCurrentLanguage } from './i18n.js';
import { showModal, hideModal } from './modal.js';

// 定数はCONSTANTS形式
const ROLE_ADMIN = 0;
const ROLE_USER = 1;

// 関数はcamelCase
async function fetchUserList() {
    // implementation
}

// エクスポート
export { fetchUserList, ROLE_ADMIN, ROLE_USER };
```

#### 非同期処理

```javascript
// ❌ Bad: エラーハンドリングなし
async function fetchData() {
    const result = await invoke('get_data');
    return result;
}

// ✅ Good: try/catchによる明示的なエラーハンドリング
async function fetchData() {
    try {
        const result = await invoke('get_data');
        return result;
    } catch (error) {
        console.error('Failed to fetch data:', error);
        showErrorModal(error);
        throw error;
    }
}
```

#### 命名規則

```javascript
// 変数・関数: camelCase
const userName = 'John Doe';
function calculateTotal(items) { /* ... */ }

// クラス: PascalCase
class UserManager {
    constructor() { /* ... */ }
}

// 定数: UPPER_SNAKE_CASE
const MAX_RETRY_COUNT = 3;
const API_TIMEOUT_MS = 5000;

// プライベートメソッド/変数: _prefix (慣例)
class DataService {
    _privateMethod() { /* ... */ }
}
```

---

## 定数管理のベストプラクティス

### 定数の一元管理

#### バックエンド (`src/consts.rs`)

```rust
// ユーザーロール
pub const ROLE_ADMIN: i32 = 0;
pub const ROLE_USER: i32 = 1;

// データベースパス
pub const DB_PATH: &str = "kakeibo.db";

// 検証ルール
pub const MIN_PASSWORD_LENGTH: usize = 16;
```

#### フロントエンド (`res/js/consts.js`)

```javascript
// バックエンドと同期させる
export const ROLE_ADMIN = 0;
export const ROLE_USER = 1;

export const MIN_PASSWORD_LENGTH = 16;
```

### 定数の使用例

```rust
// Rustでの使用
use crate::consts::ROLE_ADMIN;

if user.role == ROLE_ADMIN {
    // 管理者専用処理
}
```

```javascript
// JavaScriptでの使用
import { ROLE_ADMIN } from './consts.js';

if (user.role === ROLE_ADMIN) {
    // 管理者専用処理
}
```

### 定数追加時のチェックリスト

- [ ] `src/consts.rs` に定数を追加
- [ ] `res/js/consts.js` にも同じ値で追加（必要な場合）
- [ ] ドキュメントに追加（必要な場合）
- [ ] テストコードで使用している箇所を確認

---

## データベース接続パターン

### 基本パターン

```rust
use rusqlite::{Connection, Result};
use crate::consts::DB_PATH;

pub fn get_connection() -> Result<Connection, String> {
    Connection::open(DB_PATH)
        .map_err(|e| format!("Failed to open database: {}", e))
}

// 使用例
pub fn fetch_user(user_id: i32) -> Result<UserInfo, String> {
    let conn = get_connection()?;
    
    let user = conn.query_row(
        "SELECT user_id, user_name, role FROM USERS WHERE user_id = ?1",
        [user_id],
        |row| Ok(UserInfo {
            user_id: row.get(0)?,
            user_name: row.get(1)?,
            role: row.get(2)?,
        })
    ).map_err(|e| format!("User not found: {}", e))?;
    
    Ok(user)
}
```

### トランザクション処理

```rust
use rusqlite::Transaction;

pub fn create_transaction_with_details(
    header: TransactionHeader,
    details: Vec<TransactionDetail>,
) -> Result<(), String> {
    let mut conn = get_connection()?;
    
    let tx = conn.transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;
    
    // ヘッダ挿入
    tx.execute(
        "INSERT INTO TRANSACTION_HEADERS (...) VALUES (...)",
        rusqlite::params![...],
    ).map_err(|e| format!("Failed to insert header: {}", e))?;
    
    let header_id = tx.last_insert_rowid();
    
    // 詳細挿入
    for detail in details {
        tx.execute(
            "INSERT INTO TRANSACTION_DETAILS (...) VALUES (...)",
            rusqlite::params![header_id, ...],
        ).map_err(|e| format!("Failed to insert detail: {}", e))?;
    }
    
    tx.commit()
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;
    
    Ok(())
}
```

### プリペアドステートメントの使用

```rust
// ❌ Bad: SQL injection vulnerable
let query = format!("SELECT * FROM USERS WHERE user_name = '{}'", user_name);

// ✅ Good: プレースホルダ使用
conn.query_row(
    "SELECT * FROM USERS WHERE user_name = ?1",
    [user_name],
    |row| { /* ... */ }
)?;
```

---

## テストとプロダクションの分離

### テスト用データベース接続

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn get_test_connection() -> Result<Connection, String> {
        let conn = Connection::open_in_memory()
            .map_err(|e| format!("Failed to create test DB: {}", e))?;
        
        // テーブル作成
        conn.execute_batch(include_str!("../sql/schema.sql"))
            .map_err(|e| format!("Failed to create schema: {}", e))?;
        
        Ok(conn)
    }
    
    #[test]
    fn test_create_user() {
        let conn = get_test_connection().unwrap();
        // テストロジック
    }
}
```

### テストの構造

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // AAA (Arrange-Act-Assert) パターン
    #[test]
    fn test_user_creation() {
        // Arrange: テストデータ準備
        let conn = get_test_connection().unwrap();
        let user_name = "testuser";
        let password = "securepassword123456";
        
        // Act: 実行
        let result = create_user(&conn, user_name, password, ROLE_USER);
        
        // Assert: 検証
        assert!(result.is_ok());
        let user_id = result.unwrap();
        assert!(user_id > 0);
    }
}
```

---

## ビルドとテスト

### 開発モード実行

```bash
# Tauriアプリを開発モードで起動
cargo tauri dev

# または専用スクリプト使用
./dev.sh
```

### テスト実行

```bash
# 全テスト実行
cargo test

# 特定モジュールのテスト
cargo test validation

# テスト出力を表示
cargo test -- --nocapture

# カバレッジ測定 (tarpaulin使用)
cargo tarpaulin --out Html --output-dir .
```

### プロダクションビルド

```bash
# リリースビルド
cargo tauri build

# 成果物は以下に生成されます:
# target/release/bundle/
```

### コードフォーマット

```bash
# コードフォーマット
cargo fmt

# フォーマットチェックのみ
cargo fmt -- --check
```

### Lintチェック

```bash
# Clippy実行
cargo clippy

# より厳密なチェック
cargo clippy -- -D warnings
```

---

## デバッグ方法

### ログ出力

#### Rust

```rust
use log::{info, warn, error, debug};

pub fn some_function() {
    info!("Starting operation");
    debug!("Debug info: {:?}", data);
    
    if let Err(e) = operation() {
        error!("Operation failed: {}", e);
    }
}
```

#### JavaScript

```javascript
// 開発環境では詳細ログ、本番では最小限
console.log('Info:', data);
console.warn('Warning:', message);
console.error('Error:', error);
```

### フロントエンドからのデバッグ

```javascript
// Tauri開発者ツールを開く
// 開発モードで起動すると自動的に有効化されます
// ブラウザの開発者ツールと同様に使用可能
```

### データベースのデバッグ

```bash
# SQLiteコマンドラインで直接確認
./db.sh

# データベース内容確認
sqlite3 kakeibo.db
.tables
.schema USERS
SELECT * FROM USERS;
.quit
```

### ブレークポイントを使用したデバッグ

```rust
// デバッグビルドで実行
// VS Codeの場合: launch.jsonを設定
// RustRoverの場合: ブレークポイント設定して実行
```

---

## トラブルシューティング

### よくある問題と解決策

#### 1. データベース接続エラー

**症状**: `Error: Failed to open database`

**原因**:
- データベースファイルが存在しない
- ファイルパーミッションの問題
- 別プロセスがロック中

**解決策**:
```bash
# データベース再作成
rm kakeibo.db
./db.sh

# パーミッション確認
ls -l kakeibo.db
chmod 664 kakeibo.db
```

#### 2. ビルドエラー

**症状**: `error: could not compile`

**原因**:
- 依存関係の不整合
- Rustバージョンが古い

**解決策**:
```bash
# 依存関係のクリーンアップ
cargo clean

# Rustアップデート
rustup update

# 再ビルド
cargo build
```

#### 3. テスト失敗

**症状**: テストが通らない

**原因**:
- テストDBの状態が不正
- 非同期処理のタイミング問題

**解決策**:
```bash
# テストを一つずつ実行
cargo test test_name -- --nocapture

# テストDBを毎回再作成（in-memory使用）
```

#### 4. Tauri コマンドが呼び出せない

**症状**: `Command not found` エラー

**原因**:
- コマンドが`lib.rs`に登録されていない
- 関数シグネチャが間違っている

**解決策**:
```rust
// lib.rsでコマンドを登録
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            your_command_name,  // ← 追加
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### デバッグのヒント

1. **ログを活用**: `info!()`, `debug!()`, `error!()` マクロを適宜使用
2. **小さく分けてテスト**: 大きな機能は小さい単位に分けてテスト
3. **型を活用**: Rustの型システムでコンパイル時にバグを発見
4. **テストDBを使用**: プロダクションDBを汚さないようin-memoryDBを使用
5. **エラーメッセージを詳細に**: `map_err(|e| format!("Context: {}", e))` で情報を追加

---

## 参考リンク

### 公式ドキュメント
- [Tauri Documentation](https://tauri.app/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [rusqlite Documentation](https://docs.rs/rusqlite/)

### 関連ドキュメント
- [API仕様書](../api/)
- [アーキテクチャ設計](../design/ARCHITECTURE.md)
- [セキュリティ設計](../design/SECURITY_DESIGN.md)
- [データベース設計](../design/DATABASE_DESIGN.md)
- [UI設計](../design/UI_DESIGN.md)
- [テストガイド](testing-guide.md)

### ツール
- [Clippy](https://github.com/rust-lang/rust-clippy)
- [Rustfmt](https://github.com/rust-lang/rustfmt)
- [Tarpaulin (コードカバレッジ)](https://github.com/xd009642/tarpaulin)

---

**最終更新**: 2024-12-05 04:30 JST
