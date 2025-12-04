# アーキテクチャ設計

**最終更新**: 2024-12-05 03:41 JST

## 概要

KakeiBonは、Tauri v2フレームワークを使用したデスクトップアプリケーションです。バックエンドはRust、フロントエンドはVanilla JavaScript（フレームワーク不使用）、データベースにSQLiteを採用しています。

## アーキテクチャ図

```
┌─────────────────────────────────────────────────────┐
│                   Frontend (HTML/CSS/JS)            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐│
│  │ index.html  │  │  user-mgmt  │  │   main.html ││
│  │  (Login)    │  │   (Admin)   │  │  (Finance)  ││
│  └─────────────┘  └─────────────┘  └─────────────┘│
└───────────────────────┬─────────────────────────────┘
                        │ Tauri IPC (invoke)
┌───────────────────────▼─────────────────────────────┐
│              Backend (Rust - Tauri Commands)        │
│  ┌──────────────────────────────────────────────┐  │
│  │          lib.rs (Command Registration)       │  │
│  └──────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────┐  │
│  │              Services Layer                   │  │
│  │  ┌──────────┐ ┌──────────┐ ┌─────────────┐  │  │
│  │  │   Auth   │ │User Mgmt │ │Transaction  │  │  │
│  │  └──────────┘ └──────────┘ └─────────────┘  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌─────────────┐  │  │
│  │  │ Category │ │ Account  │ │Aggregation  │  │  │
│  │  └──────────┘ └──────────┘ └─────────────┘  │  │
│  │  ┌──────────┐ ┌──────────┐ ┌─────────────┐  │  │
│  │  │   I18n   │ │Encryption│ │   Session   │  │  │
│  │  └──────────┘ └──────────┘ └─────────────┘  │  │
│  └──────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────┐  │
│  │           Core Modules                        │  │
│  │  ┌──────────┐ ┌──────────┐ ┌─────────────┐  │  │
│  │  │    db    │ │validation│ │  security   │  │  │
│  │  └──────────┘ └──────────┘ └─────────────┘  │  │
│  │  ┌──────────┐ ┌──────────┐                  │  │
│  │  │  crypto  │ │ settings │                  │  │
│  │  └──────────┘ └──────────┘                  │  │
│  └──────────────────────────────────────────────┘  │
└───────────────────────┬─────────────────────────────┘
                        │
                        ▼
               ┌─────────────────┐
               │  SQLite Database │
               │   (kakeibo.db)   │
               └─────────────────┘
```

## レイヤー構成

### 1. フロントエンド層

**技術**: Vanilla JavaScript (ES6 Modules)、HTML5、CSS3

**主要画面**:
- `index.html` - ログイン画面・初期管理者セットアップ
- `user-management.html` - ユーザー管理画面（管理者専用）
- `main.html` - メイン家計簿画面（取引管理、カテゴリ管理、集計等）

**特徴**:
- フレームワーク不使用（軽量、依存関係なし）
- ES6 Modulesによるモジュール分割
- Tauri IPCを通じたバックエンド通信

### 2. バックエンド層

#### 2.1 Tauri Commands層

**ファイル**: `src/lib.rs`

**役割**: フロントエンドからの呼び出しを受け付け、各サービスへルーティング

**アプリケーション状態**:
```rust
pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub auth: Arc<Mutex<AuthService>>,
    pub user_mgmt: Arc<Mutex<UserManagementService>>,
    pub encryption: Arc<Mutex<EncryptionService>>,
    pub i18n: Arc<Mutex<I18nService>>,
    pub category: Arc<Mutex<CategoryService>>,
    pub transaction: Arc<Mutex<TransactionService>>,
    pub session: Arc<Mutex<SessionState>>,
    pub settings: Arc<Mutex<SettingsManager>>,
}
```

#### 2.2 Services層

各ドメインのビジネスロジックを担当:

| サービス | 役割 |
|---------|------|
| `auth` | 認証・ログイン・ログアウト |
| `user_management` | ユーザーCRUD操作 |
| `encryption` | データ暗号化・復号化 |
| `i18n` | 多言語対応 |
| `category` | カテゴリ管理 |
| `transaction` | 取引管理 |
| `account` | 口座管理 |
| `shop` | 店舗管理 |
| `manufacturer` | メーカー管理 |
| `product` | 商品管理 |
| `session` | セッション管理 |
| `aggregation` | 集計処理 |

#### 2.3 Core Modules層

共通機能を提供:

| モジュール | 役割 |
|-----------|------|
| `db` | データベース接続・トランザクション管理 |
| `validation` | 入力値検証 |
| `security` | パスワードハッシュ化（Argon2） |
| `crypto` | AES-256-GCM暗号化 |
| `settings` | アプリケーション設定管理 |
| `sql_queries` | SQL文字列の定数管理 |
| `consts` | 定数定義 |

### 3. データベース層

**技術**: SQLite 3

**アクセス方法**:
```bash
./db.sh
```

**接続方法（コード内）**:
```rust
use sqlx::sqlite::SqlitePool;

let pool = SqlitePool::connect("sqlite:kakeibo.db").await?;
```

**主要テーブル**:
- `users` - ユーザー情報
- `categories` - カテゴリマスタ
- `transactions` - 取引記録
- `accounts` - 口座情報
- `shops` - 店舗マスタ
- `manufacturers` - メーカーマスタ
- `products` - 商品マスタ

## データフロー

### 典型的なリクエストフロー

```
1. Frontend: ボタンクリック
   ↓
2. JavaScript: invoke('command_name', { params })
   ↓
3. Tauri IPC: フロントエンド→バックエンド通信
   ↓
4. Backend Command: AppStateから必要なサービスを取得
   ↓
5. Service Layer: ビジネスロジック実行
   ↓
6. Core Module: 検証・暗号化・DB操作
   ↓
7. Database: SQL実行
   ↓
8. Backend: Result返却（成功時データ、失敗時エラー）
   ↓
9. Tauri IPC: バックエンド→フロントエンド通信
   ↓
10. Frontend: 結果表示
```

### 具体例: ログイン処理

```
1. index.html: ログインボタンクリック
   ↓
2. login.js: invoke('verify_login', { username, password })
   ↓
3. lib.rs: verify_login() 関数
   ↓
4. auth.rs: AuthService::verify_login()
   ↓
5. db.rs: Database::get_user_by_username()
   ↓
6. security.rs: verify_password() (Argon2検証)
   ↓
7. session.rs: SessionState::set_logged_in_user()
   ↓
8. Result: Ok(UserInfo) または Err(String)
   ↓
9. フロントエンド: 成功時画面遷移、失敗時エラー表示
```

## セキュリティアーキテクチャ

### パスワード保護

- **ハッシュ化**: Argon2id（メモリハード関数）
- **ソルト**: 自動生成（ユーザーごと）
- **最小長**: 16文字

### データ暗号化

- **アルゴリズム**: AES-256-GCM
- **鍵導出**: ユーザーパスワードベース
- **暗号化対象**: 取引データ、口座情報等

### セッション管理

- **ストレージ**: メモリ内（AppState）
- **有効期限**: アプリケーション再起動まで
- **ログアウト**: セッション即時クリア

## 状態管理

### バックエンド状態

`AppState`構造体にすべての状態を集約:
- Arc<Mutex<T>>パターンで並行アクセス制御
- Tauriのmanaged stateとして管理

### フロントエンド状態

- ローカル変数とDOM操作
- セッションストレージは**使用しない**（セキュリティリスク）
- 必要な情報は都度バックエンドから取得

## ビルド構成

### 開発ビルド

```bash
cargo tauri dev
```

### リリースビルド

```bash
cargo tauri build
```

### テスト実行

```bash
cargo test
```

## 依存関係

### Rustクレート

主要な依存関係:
- `tauri` (v2.x) - デスクトップアプリフレームワーク
- `sqlx` - SQLデータベース操作
- `argon2` - パスワードハッシュ化
- `aes-gcm` - AES暗号化
- `serde` / `serde_json` - JSON シリアライズ
- `tokio` - 非同期ランタイム

### フロントエンド

- 外部依存なし（Vanilla JS）

## パフォーマンス考慮事項

1. **非同期処理**: すべてのDB操作は非同期（tokio）
2. **接続プール**: SQLiteコネクションプール使用
3. **メモリ効率**: Arc/Mutexによる参照カウント
4. **暗号化オーバーヘッド**: 必要な箇所のみ暗号化

## 制約事項

1. **シングルユーザー**: 同時に1ユーザーのみログイン可能
2. **ローカル実行**: ネットワーク通信なし
3. **データベース**: SQLiteのみサポート
4. **プラットフォーム**: Windows、macOS、Linux

## 関連ドキュメント

- [データベース設計](./DATABASE_DESIGN.md)
- [セキュリティ設計](./SECURITY_DESIGN.md)
- [API仕様](../api/)
- [コーディング規約](../.ai-context/CONVENTIONS.md)

---

**ガイドライン遵守**: このドキュメントはドキュメント作成ガイドライン（`.ai-context/DOCUMENTATION_CREATION_GUIDELINES.md`）に従って作成されました。
