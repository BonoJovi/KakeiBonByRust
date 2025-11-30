# KakeiBonByRust
Rust言語で構築された家計簿アプリケーション「KakeiBon」

## 📚 前身プロジェクト

**すぐに使える完成版をお探しですか？**

👉 **[KakeiBon (オリジナル版)](https://github.com/BonoJovi/KakeiBon)** - 今すぐ使える完成版！

元祖KakeiBonは、**Lazarus/Free Pascalで作られた完成版の家計簿アプリ**です。

**主な違い:**
- ✅ **安定版・本番利用可能** - すぐに使えます
- 📦 **ビルド済みバイナリあり** - [Releases](https://github.com/BonoJovi/KakeiBon/releases/)からダウンロード
- 🇯🇵 **日本語専用インターフェース**
- 🖥️ **Linux & Windows 対応**
- 🔤 **大きな文字とアクセシビリティ対応**

**なぜRust版を開発？**

このRust版では以下を実現：
- ⚡ より高速な動作
- 🔒 強化されたセキュリティ（Argon2 + AES-256-GCM）
- 🌐 完全多言語対応（日本語・英語）
- 🎨 モダンなアーキテクチャ
- 🔮 将来の拡張性

💡 **両方試して、お好みの方をお使いください！**

---

## 概要
RustとTauriフレームワークで構築された、モダンな家計簿管理アプリケーションです。

**プロジェクト開始**: 2025-10-22 (JST)

## 機能
- 💰 支出・収入の記録
- 👥 ロールベースのアクセス制御によるマルチユーザサポート
- 🔐 安全なパスワード管理（Argon2id）
- 🔒 データ暗号化（AES-256-GCM）
- 🌐 多言語対応（英語、日本語）
- 📊 階層的な費目管理
- ⚙️ ユーザ設定管理

## 技術スタック
- **フロントエンド**: HTML, CSS, JavaScript
- **バックエンド**: Rust
- **フレームワーク**: Tauri v2.8.5
- **データベース**: SQLite (WALモード)
- **セキュリティ**: Argon2id（パスワードハッシュ化）、AES-256-GCM（データ暗号化）

## ドキュメント

📚 **[English Documentation](./README_en.md)** is also available.

詳細なドキュメントは [docs/ja](./docs/ja) ディレクトリにあります：

- [ユーザ管理](./docs/ja/USER_MANAGEMENT.md) - ユーザ登録、認証、管理
- [暗号化管理](./docs/ja/ENCRYPTION_MANAGEMENT.md) - データ暗号化と再暗号化システム
- [設定管理](./docs/ja/SETTINGS_MANAGEMENT.md) - ユーザ設定と環境設定
- [多言語対応実装](./docs/ja/I18N_IMPLEMENTATION.md) - 多言語対応システム
- [テストサマリー](./docs/ja/TEST_SUMMARY.md) - テスト結果とカバレッジ

## セットアップ

### 必要な環境
- Rust 1.70+
- Node.js（Tauri開発用）

### ビルド
```bash
cargo build
```

### テスト実行
```bash
cargo test --lib
```

### アプリケーション実行
```bash
cargo tauri dev
```

## プロジェクト構造
```
KakeiBonByRust/
├── src/               # Rustソースコード
│   ├── services/      # ビジネスロジックサービス
│   ├── db.rs          # データベース管理
│   ├── crypto.rs      # 暗号化ユーティリティ
│   ├── consts.rs      # アプリケーション定数
│   └── ...
├── res/               # リソース
│   └── sql/           # SQLスキーマファイル
├── docs/              # ドキュメント
│   ├── en/            # 英語ドキュメント
│   └── ja/            # 日本語ドキュメント
└── $HOME/.kakeibon/   # ユーザデータディレクトリ
    ├── KakeiBonDB.sqlite3
    └── KakeiBon.json
```

## テスト結果
```
総テスト数: 90
成功: 90
失敗: 0
成功率: 100%
```

## セキュリティ機能
- Argon2idによるパスワードハッシュ化
- AES-256-GCMによるデータ暗号化
- パスワード長: 16-128文字
- パスワード複雑性要件の強制
- パスワード変更時の再暗号化
- ロールベースのアクセス制御

## ライセンス
詳細は[LICENSE](./LICENSE)ファイルをご確認ください。

## コントリビューション
プルリクエストを歓迎します！お気軽にご提出ください。

## お問い合わせ
質問やフィードバックは、GitHubのIssueでお願いします。
