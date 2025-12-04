# 開発環境セットアップガイド

**Last Updated: 2024-12-05 05:03 JST**

このドキュメントは、KakeiBonの開発に参加するための環境セットアップ手順を説明します。

---

## 目次

1. [前提条件](#前提条件)
2. [必要なツールのインストール](#必要なツールのインストール)
3. [プロジェクトのクローン](#プロジェクトのクローン)
4. [依存関係のインストール](#依存関係のインストール)
5. [開発用データベースのセットアップ](#開発用データベースのセットアップ)
6. [ビルドと実行](#ビルドと実行)
7. [テストの実行](#テストの実行)
8. [トラブルシューティング](#トラブルシューティング)

---

## 前提条件

- **OS**: Linux, macOS, Windows
- **ディスク容量**: 最低2GB（ビルド成果物含む）
- **メモリ**: 最低4GB RAM推奨

---

## 必要なツールのインストール

### 1. Rust

```bash
# Rustupをインストール（公式推奨方法）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# インストール後、シェルを再起動するか以下を実行
source $HOME/.cargo/env

# バージョン確認
rustc --version
cargo --version
```

### 2. Node.js (18以上推奨)

```bash
# nvm経由でのインストール推奨
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# バージョン確認
node --version
npm --version
```

### 3. システム依存ライブラリ

#### Linux (Debian/Ubuntu)

```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.0-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libsqlite3-dev
```

#### Linux (Fedora)

```bash
sudo dnf install -y \
  webkit2gtk4.0-devel \
  openssl-devel \
  gtk3-devel \
  libappindicator-gtk3-devel \
  librsvg2-devel \
  sqlite-devel
```

#### macOS

```bash
# Homebrewが必要
brew install sqlite
```

#### Windows

- Visual Studio 2022のC++ビルドツールをインストール
- WebView2ランタイム（通常はWindows 11に同梱）

---

## プロジェクトのクローン

```bash
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust
```

---

## 依存関係のインストール

### Rust依存関係

```bash
# Cargo.tomlから依存関係をインストール
cargo fetch
```

### フロントエンド依存関係（もしあれば）

```bash
# package.jsonがある場合
npm install
# または
pnpm install
```

---

## 開発用データベースのセットアップ

### 開発用DBの初期化

```bash
# 開発用スクリプトを使用
./dev.sh

# または手動で
sqlite3 dev_kakeibo.db < sql/schema.sql
```

**注意**: 本番用データベース（`./db.sh`）には触れないでください。

### データベーススキーマ確認

```bash
sqlite3 dev_kakeibo.db ".schema"
```

---

## ビルドと実行

### 開発モード（ホットリロード付き）

```bash
cargo tauri dev
```

### リリースビルド

```bash
cargo tauri build
```

ビルド成果物:
- **Linux**: `target/release/bundle/appimage/KakeiBon_*.AppImage`
- **macOS**: `target/release/bundle/macos/KakeiBon.app`
- **Windows**: `target/release/bundle/msi/KakeiBon_*.msi`

---

## テストの実行

### すべてのテストを実行

```bash
cargo test
```

### 特定のテストモジュールを実行

```bash
# 認証関連のテストのみ
cargo test auth

# バリデーション関連のテストのみ
cargo test validation
```

### テストカバレッジ（tarpaulin使用）

```bash
# tarpaulinをインストール（初回のみ）
cargo install cargo-tarpaulin

# カバレッジ実行
cargo tarpaulin --out Html --output-dir .
```

結果は `tarpaulin-report.html` に出力されます。

---

## トラブルシューティング

### 問題: `webkit2gtk`が見つからない

**解決策**: システム依存ライブラリを再インストール

```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.0-dev

# Fedora
sudo dnf install webkit2gtk4.0-devel
```

### 問題: ビルドが遅い

**解決策**: リンカーを`lld`または`mold`に変更

`.cargo/config.toml`を作成:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```

### 問題: `sqlx`のコンパイルエラー

**解決策**: オフラインモードを有効化

```bash
# .sqlxディレクトリが存在することを確認
ls -la .sqlx

# 存在しない場合は準備
cargo sqlx prepare
```

### 問題: テストが失敗する

**原因**: 開発用DBが破損している可能性

**解決策**: DBを再初期化

```bash
rm dev_kakeibo.db
./dev.sh
cargo test
```

---

## 次のステップ

セットアップが完了したら、以下のドキュメントを参照してください:

- [開発者ガイド](../guides/DEVELOPER_GUIDE.md) - コーディング規約とベストプラクティス
- [アーキテクチャ設計](../../design/ja/ARCHITECTURE.md) - システム全体像
- [API仕様書](../api/ja/) - 各種APIの詳細仕様

---

## サポート

質問や問題がある場合:
- [GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues)
- [Discussions](https://github.com/BonoJovi/KakeiBonByRust/discussions)

---

**貢献に感謝します！**
