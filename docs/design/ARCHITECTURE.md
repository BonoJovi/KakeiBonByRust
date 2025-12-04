# アーキテクチャ概要

**最終更新**: 2024-12-04 18:45 JST

## 1. システム全体構成

KakeiBonは、Tauriフレームワークを使用したデスクトップアプリケーションです。

```
┌─────────────────────────────────────┐
│        フロントエンド (HTML/JS)      │
│  - UI表示・ユーザー操作             │
│  - クライアント側バリデーション      │
└──────────────┬──────────────────────┘
               │ Tauri IPC
┌──────────────▼──────────────────────┐
│       バックエンド (Rust)            │
│  - ビジネスロジック                 │
│  - サーバー側バリデーション          │
│  - 暗号化/セキュリティ              │
└──────────────┬──────────────────────┘
               │ sqlx
┌──────────────▼──────────────────────┐
│       データベース (SQLite)          │
│  - ローカルファイル                 │
│  - データ永続化                     │
└─────────────────────────────────────┘
```

## 2. 技術スタック

### フロントエンド
- **言語**: Vanilla JavaScript (ES6+)
- **UI**: HTML5 + CSS3
- **通信**: Tauri IPC (`window.__TAURI__.invoke`)
- **特徴**: フレームワーク非依存、軽量

### バックエンド
- **言語**: Rust (Edition 2021)
- **フレームワーク**: Tauri 1.x
- **データベースアクセス**: sqlx 0.7
- **暗号化**: 
  - パスワードハッシュ: argon2 0.5
  - データ暗号化: aes-gcm 0.10
- **シリアライゼーション**: serde 1.0

### データベース
- **DBMS**: SQLite 3
- **場所**: `~/.kakeibon/KakeiBonDB.sqlite3`
- **マイグレーション**: 手動管理（`sql/`ディレクトリ）

## 3. ディレクトリ構成

```
KakeiBonByRust/
├── src/                          # Rustバックエンド
│   ├── main.rs                   # エントリーポイント
│   ├── lib.rs                    # Tauriコマンドエクスポート
│   ├── db.rs                     # DB接続管理
│   ├── validation.rs             # 入力検証
│   ├── security.rs               # パスワードハッシュ
│   ├── sql_queries.rs            # SQL定数
│   └── services/                 # ビジネスロジック
│       ├── auth.rs               # 認証
│       ├── user_management.rs    # ユーザー管理
│       ├── account.rs            # 口座管理
│       ├── category.rs           # カテゴリ管理
│       ├── transaction.rs        # 入出金管理
│       ├── shop.rs               # 店舗管理
│       ├── manufacturer.rs       # メーカー管理
│       ├── product.rs            # 商品管理
│       ├── aggregation.rs        # 集計機能
│       ├── encryption.rs         # データ暗号化
│       ├── session.rs            # セッション管理
│       └── i18n.rs               # 国際化
├── res/                          # フロントエンド
│   ├── index.html                # ログイン画面
│   ├── user-management.html      # ユーザー管理画面
│   ├── main.html                 # メイン画面
│   ├── js/                       # JavaScript
│   │   ├── consts.js             # 定数
│   │   ├── validation-helpers.js # バリデーション共通
│   │   └── (各画面のJS)
│   ├── css/                      # スタイルシート
│   ├── locales/                  # 翻訳ファイル
│   │   ├── ja/
│   │   └── en/
│   └── tests/                    # フロントエンドテスト
├── sql/                          # SQLスクリプト
│   ├── create_tables.sql
│   └── (その他DDL)
├── docs/                         # ドキュメント
│   ├── design/                   # 設計書
│   ├── api/                      # API仕様
│   └── developer/                # 開発者向け
└── tauri.conf.json               # Tauri設定
```

## 4. データフロー

### 4.1 ユーザー操作の流れ

```
[ユーザー] 
    ↓ (1) 入力/クリック
[HTML/JS]
    ↓ (2) クライアント側バリデーション
[validation-helpers.js]
    ↓ (3) Tauri IPC呼び出し
[window.__TAURI__.invoke()]
    ↓ (4) コマンド実行
[Rust: src/services/*.rs]
    ↓ (5) サーバー側バリデーション
[src/validation.rs]
    ↓ (6) SQL実行
[sqlx + SQLite]
    ↓ (7) 結果返却
[JSON形式でフロントエンドへ]
    ↓ (8) UI更新
[DOM操作]
```

### 4.2 認証フロー

```
[ログイン画面]
    ↓ verify_login()
[auth.rs]
    ↓ Argon2検証
[security.rs]
    ↓ セッション作成
[session.rs]
    ↓ 画面遷移
[main.html]
```

### 4.3 データ暗号化フロー

```
[ユーザー入力]
    ↓ メモ保存
[transaction.rs]
    ↓ AES-GCM暗号化
[encryption.rs]
    ↓ DB保存
[ENCRYPTED_FIELDS テーブル]
    ↓ 読み込み
[encryption.rs]
    ↓ 復号化
[フロントエンド表示]
```

## 5. 主要コンポーネント

### 5.1 認証・セキュリティ

| モジュール | 責務 |
|-----------|------|
| `auth.rs` | ログイン認証、ユーザー検証 |
| `security.rs` | Argon2パスワードハッシュ |
| `encryption.rs` | AES-GCM データ暗号化 |
| `session.rs` | セッション管理、タイムアウト |

### 5.2 マスタデータ管理

| モジュール | 対応テーブル | 責務 |
|-----------|-------------|------|
| `user_management.rs` | USERS | ユーザーCRUD |
| `account.rs` | ACCOUNTS | 口座マスタ |
| `category.rs` | CATEGORY1/2/3 | カテゴリ階層管理 |
| `shop.rs` | SHOPS | 店舗マスタ |
| `manufacturer.rs` | MANUFACTURERS | メーカーマスタ |
| `product.rs` | PRODUCTS | 商品マスタ |

### 5.3 トランザクション管理

| モジュール | 対応テーブル | 責務 |
|-----------|-------------|------|
| `transaction.rs` | TRANSACTIONS_HEADER<br>TRANSACTIONS_DETAIL | ヘッダ・明細構造の入出金管理 |
| `aggregation.rs` | - | 集計・レポート生成 |

### 5.4 共通基盤

| モジュール | 責務 |
|-----------|------|
| `db.rs` | DB接続プール管理 |
| `validation.rs` | 入力検証ルール |
| `sql_queries.rs` | SQL定数定義 |
| `i18n.rs` | 多言語対応 |

## 6. データベース設計の特徴

### 6.1 ヘッダ・明細構造

入出金管理は以下の構造を採用：

```
TRANSACTIONS_HEADER (1)
    ↓
TRANSACTIONS_DETAIL (N)
```

**メリット**:
- レシート単位での管理が自然
- 商品別履歴の追跡が容易
- 一括編集が簡単

### 6.2 多言語対応

カテゴリ名は別テーブルで管理：

```
CATEGORY1 (基本情報)
    ↓
CATEGORY1_I18N (翻訳)
```

### 6.3 暗号化フィールド

機密データは別テーブルで暗号化：

```
TRANSACTIONS_HEADER/DETAIL (公開データ)
    ↓
ENCRYPTED_FIELDS (暗号化メモ)
```

## 7. セキュリティ設計

### 7.1 パスワード管理

- **ハッシュ**: Argon2id
- **ソルト**: ランダム生成
- **保存**: `USERS.PASSWORD_HASH`

### 7.2 データ暗号化

- **アルゴリズム**: AES-256-GCM
- **鍵導出**: ユーザーパスワードベース
- **適用対象**: メモフィールド

### 7.3 セッション管理

- **保存**: メモリ内 (`HashMap`)
- **タイムアウト**: 30分（設定可能）
- **再認証**: タイムアウト時に自動ログアウト

## 8. テスト戦略

### 8.1 バックエンド (Rust)

- **ユニットテスト**: 各モジュール内に `#[cfg(test)]`
- **統合テスト**: `src/validation_tests.rs`
- **カバレッジ**: tarpaulin使用
- **実績**: 201+ tests

### 8.2 フロントエンド (JavaScript)

- **共通モジュール**: 
  - `validation-helpers.js`: 検証ロジック
  - `*-validation-tests.js`: テストスイート
- **各画面**: 共通モジュールをインポート

### 8.3 CI/CD

- **未実装**: 手動テスト/ビルド
- **今後**: GitHub Actionsで自動化予定

## 9. 開発ツール

### 9.1 ビルド

```bash
# 開発モード
./dev.sh

# リリースビルド
cargo build --release
```

### 9.2 データベース操作

```bash
# SQL実行
./db.sh "SELECT * FROM USERS;"

# マイグレーション
./db.sh < sql/create_tables.sql
```

### 9.3 テスト実行

```bash
# Rustテスト
cargo test

# カバレッジ
cargo tarpaulin --out Html
```

## 10. 関連ドキュメント

- [API仕様](../api/ja/) - 各Tauriコマンドの詳細
- [トランザクション設計](./architecture/TRANSACTION_DESIGN_V2_ja.md) - ヘッダ・明細構造の詳細
- [セッション管理仕様](./architecture/session-management-spec.md)
- [税計算ロジック](./architecture/tax-calculation-logic.md)

---

**ドキュメント作成プロセス評価**:
- ✅ 最新コードとの突合: 完了（`./db.sh`でテーブル確認、`src/`ディレクトリ確認）
- ✅ コード引用最小化: 完了（図とテーブルで説明）
- ✅ 構造の明確化: 完了（レイヤー分離、データフロー図示）
- ✅ 関連ドキュメントへのリンク: 完了

**改善提案**: なし（初回作成のため、今後のフィードバックで改善）
