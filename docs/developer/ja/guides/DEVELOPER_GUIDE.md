# 開発者ガイド

KakeiBonByRust の開発に参加する開発者向けの概観・横串解説ドキュメントです。
個別領域の詳細は専門ガイドへ誘導しています。

**対象**: バックエンド (Rust) / フロントエンド (Vanilla JS) / DB (SQLite) いずれの貢献者
**最終更新**: 2026-05-30 (v2.5.0 時点)

---

## 1. プロジェクト概要

KakeiBonByRust は、Tauri v2 で構築された家計簿（household budget）デスクトップアプリです。

- **アーキテクチャ**: Tauri v2（Rust バックエンド + Vanilla JS フロントエンド + SQLite）
- **対応言語**: 日本語 / 英語（i18n）
- **対応プラットフォーム**: Linux（検証済み）/ Windows（v2.4.1 以降配布対応）/ macOS（未検証）
- **ライセンス**: ISC（package.json）
- **対象用途**: 家計用途専用（会計処理は対象外）

---

## 2. 技術スタック

実態は `Cargo.toml` / `package.json` を正としますが、v2.5.0 時点の主要要素は以下:

### 2.1 Rust 依存（`Cargo.toml`）

| カテゴリ | クレート | バージョン |
|---|---|---|
| フレームワーク | `tauri` | 2.11.1 |
| ビルド | `tauri-build` | 2.5.2 |
| ロギング | `tauri-plugin-log` | 2.7.1 |
| ランタイム | `tokio` | 1.x (full features) |
| シリアライズ | `serde`, `serde_json` | 1.x |
| DB（テスト用） | `sqlx` | 0.8.6 (`runtime-tokio`, `sqlite`) |
| 日時 | `chrono` | 0.4 (serde) |
| 暗号 | `argon2`, `aes-gcm`, `base64`, `rand` | — |
| ロケール補助 | `glib` | 0.20 |
| 祝日 | `jpholiday` | 0.1.4 |

> **注意**: 本番 DB アクセスは `rusqlite` ベースの自作レイヤ（`src/db.rs` + `src/sql_queries.rs`）を使用。`sqlx` は `src/test_helpers.rs` のテストインフラ専用です。

- Rust edition: 2021
- 最低 Rust バージョン: 1.77.2

### 2.2 フロントエンド

- **形態**: Vanilla JS + HTML + CSS（フレームワークなし）
- **モジュール形式**: ES Modules（**`.js` 拡張子をインポート時に明記**）
- **テスト**: Jest（`res/tests/` 以下）
- **node/npm 管理**: nvm（非対話 zsh では PATH に npm が無いことに注意）

### 2.3 データベース

- **エンジン**: SQLite 3
- **ファイル**: `~/.kakeibon/KakeiBonDB.sqlite3`
- **スキーマ管理**: `res/sql/` 配下の SQL ファイル
- **アクセス規約**: すべてのクエリを `src/sql_queries.rs` に集約、prepared statements 必須

---

## 3. プロジェクト構造

```
KakeiBonByRust/
├── src/                          # Rust バックエンド
│   ├── main.rs / lib.rs          # エントリポイント
│   ├── db.rs                     # DB 接続レイヤ
│   ├── sql_queries.rs            # SQL クエリ集約
│   ├── consts.rs                 # ROLE_ADMIN/USER, MIN_PASSWORD_LENGTH など
│   ├── crypto.rs                 # AES-256-GCM 暗号化
│   ├── security.rs               # argon2 ハッシュ、認証
│   ├── settings.rs               # ユーザー設定 (起算日, HolidayShift など)
│   ├── validation.rs             # 統一バリデーション
│   ├── test_helpers.rs           # テスト用 sqlx ヘルパ
│   └── services/                 # ドメインロジック (15 module)
│       ├── account.rs            # 口座管理
│       ├── aggregation.rs        # 集計
│       ├── auth.rs               # 認証
│       ├── category.rs           # 費目管理
│       ├── encryption.rs         # 暗号化サービス
│       ├── holiday.rs            # 祝日 (jpholiday)
│       ├── i18n.rs               # 多言語リソース
│       ├── manufacturer.rs       # メーカー
│       ├── period.rs             # 期間計算 (起算日含む)
│       ├── product.rs            # 商品
│       ├── recurring.rs          # 繰り返し予定入出金 (RULE_ID 中心設計)
│       ├── session.rs            # セッション
│       ├── shop.rs               # 店舗
│       ├── transaction.rs        # 入出金
│       └── user_management.rs    # ユーザー管理
│
├── res/                          # フロントエンドリソース
│   ├── js/                       # ES Modules (~34 ファイル)
│   ├── css/                      # スタイルシート
│   ├── sql/                      # 初期 SQL / シード
│   └── tests/                    # Jest テストスイート
│
├── docs/                         # ドキュメント (詳細は INDEX_ja.md)
├── scripts/                      # リリース・統計・i18n チェックスクリプト
├── src-tauri ファイル群           # tauri.conf.json など
├── CHANGELOG_ja.md / CHANGELOG_en.md  # リリースノート
└── Cargo.toml / package.json     # 依存定義
```

---

## 4. 開発環境セットアップ

詳細手順は専門ドキュメントへ:
- [開発環境セットアップ](../setup/DEVELOPMENT_SETUP.md)

要点だけ:
- Linux 上では `apt`/`pacman` 等で WebKitGTK と関連パッケージが必要
- node は **nvm 管理**を推奨（非対話シェル PATH に注意）
- Rust は `rustup` で edition 2021 + 1.77.2 以上

---

## 5. よく使うコマンド

```bash
# 開発実行（ホットリロード付き）
cargo tauri dev

# バックエンドテスト
cargo test

# フロントエンドテスト (Jest)
cd res/tests && npm test

# 全テスト一括実行（バックエンド + フロントエンド）
./res/tests/run-all-tests.sh

# リリース前チェック (3 version files 整合, etc.)
./scripts/check-release.sh

# i18n リソース整合チェック
./scripts/check_i18n_resources.sh

# リリースビルド
cargo tauri build
```

> **再起動の用語**: Tauri アプリのため、フロント変更の反映は「ブラウザリロード」ではなく「**アプリ再起動**」になります。

---

## 6. コーディング規約

詳細は [コーディング規約](CODING_STANDARDS.md) を参照。

要点（必ず守る）:

### Rust
- 本番コードで `unwrap()` 禁止 → `Result<T, E>` で返す
- すべての SQL は `src/sql_queries.rs` に集約、prepared statements
- コミット前に `cargo fmt`、`cargo clippy` の警告ゼロ

### JavaScript
- ES Modules（インポートに `.js` 拡張子）
- バリデーションは `validation.rs`（バックエンド）と対応する JS 側で**二段階**実施

### コミットメッセージ
- 英語、Conventional Commits 形式: `type(scope): description`
  - 例: `fix(window): fit + center every screen on load`

### ユーザー向け文字列
- ハードコード禁止、すべて i18n リソース経由
- 新規 RESOURCE_ID 追加時は **MAX ID を確認**してから（`INSERT OR IGNORE` は重複時サイレントスキップする）。`/i18n-add` スキルが推奨手順

---

## 7. i18n ワークフロー

詳細は [I18N 実装ガイド](I18N_IMPLEMENTATION.md) / [翻訳ガイド](translation-guide.md) を参照。

ポイント:
- バックエンド: `src/services/i18n.rs` がリソースを解決
- フロントエンド: `res/js/i18n.js` が言語切替を担当
- リソースは SQLite テーブル (`*_I18N`) に格納、初回起動でシード
- 対応言語: 日本語 (ja) / 英語 (en)、追加歓迎

---

## 8. リリース手順

詳細は CHANGELOG_ja.md と `scripts/check-release.sh` を参照。`/release` スキルが推奨フローです。

3 つのバージョンファイルを必ず**同期**:
1. `Cargo.toml` の `version`
2. `src-tauri/tauri.conf.json` の `version`
3. `package.json` の `version`

その後:
1. `CHANGELOG_ja.md` / `CHANGELOG_en.md` にエントリ追加
2. `./scripts/check-release.sh` で整合チェック
3. `dev` → `main` マージで release workflow が自動起動（draft → publish）
4. `gh release create` を手で打たない（workflow と 422 衝突する）

---

## 9. ブランチ運用

- **`dev`**: 全ての開発作業はこちら
- **`main`**: リリースタグ用、`dev` からのマージで更新
- 大規模リファクタは `dev-vN` のような並行ブランチでマージコンフリクトを回避

---

## 10. テスト戦略

詳細は [テスト概要](../../../testing/ja/TEST_OVERVIEW.md) を参照。

v2.5.0 時点の規模:
- Rust: 390 件
- JavaScript (Jest): 623 件

> テスト数は CHANGELOG_ja.md の各リリースエントリが最新値です。

---

## 11. 関連ドキュメント

- [ドキュメント索引 (INDEX_ja)](../../../INDEX_ja.md) — すべてのドキュメントの入口
- [コーディング規約](CODING_STANDARDS.md)
- [I18N 実装ガイド](I18N_IMPLEMENTATION.md)
- [開発環境セットアップ](../setup/DEVELOPMENT_SETUP.md)
- [テスト概要](../../../testing/ja/TEST_OVERVIEW.md)
- [CHANGELOG_ja](../../../../CHANGELOG_ja.md)
- [貢献ガイド](../../../../CONTRIBUTING.md)
- [行動規範](../../../../CODE_OF_CONDUCT.md)

---

## 12. 質問・サポート

- GitHub Issues — バグ・機能リクエスト
- GitHub Discussions — 質問・議論
- メール: [bonojovi2741@gmail.com](mailto:bonojovi2741@gmail.com)
