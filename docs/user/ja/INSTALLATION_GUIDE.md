# インストールガイド

**バージョン**: 1.0.1  
**最終更新**: 2025-12-03 14:40 JST

---

## 目次

- [システム要件](#システム要件)
- [Linux へのインストール](#linux-へのインストール)
- [Windows へのインストール](#windows-へのインストール)
- [macOS へのインストール](#macos-へのインストール)
- [ソースからのビルド](#ソースからのビルド)
- [初回起動](#初回起動)
- [トラブルシューティング](#トラブルシューティング)

---

## システム要件

### 最小要件
- **OS**: Linux (Ubuntu 20.04+, Debian 11+), Windows 10+, macOS 10.15+
- **RAM**: 512MB
- **ディスク容量**: 100MB
- **ディスプレイ**: 1024x768 以上

### 推奨
- **RAM**: 1GB 以上
- **ディスプレイ**: 1920x1080 以上

---

## Linux へのインストール

### 方法1: ビルド済みバイナリを使用（推奨）

**注意**: 現在、ビルド済みバイナリは配布していません。方法2（ソースからビルド）をご利用ください。

### 方法2: ソースからビルド

#### 前提条件

KakeiBon は Tauri v2 を使用しており、以下のシステム依存関係が必要です：

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Fedora/RHEL
sudo dnf install \
  webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel

# Arch Linux
sudo pacman -S \
  webkit2gtk \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  gtk3 \
  libappindicator-gtk3 \
  librsvg \
  libvips
```

**重要な注意事項:**
- パッケージ名はディストリビューションによって異なる場合があります
- ビルド時に依存関係エラーが発生した場合は、[GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues) で報告してください

#### Rust のインストール

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### KakeiBon のビルド

```bash
# リポジトリのクローン
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust

# 安定版をチェックアウト
git checkout v1.0.1

# ビルド
cargo tauri build

# ビルドされたバイナリの場所
# ./target/release/bundle/
```

#### バイナリのインストール

```bash
# ローカル bin にコピー（例）
sudo cp target/release/kakeibon /usr/local/bin/

# または .deb パッケージをインストール（利用可能な場合）
sudo dpkg -i target/release/bundle/deb/kakeibon_1.0.1_amd64.deb
```

---

## Windows へのインストール

### 方法1: インストーラを使用（準備中）

ビルド済み Windows インストーラは今後のリリースで提供予定です。

### 方法2: ソースからビルド

#### 前提条件

1. **Rust のインストール**: [rustup.rs](https://rustup.rs/) からダウンロード
2. **ビルドツールのインストール**: Visual Studio Build Tools または Visual Studio Community
3. **WebView2 のインストール**: Windows 11 では通常プリインストール済み（必要に応じて [Microsoft](https://developer.microsoft.com/microsoft-edge/webview2/) からダウンロード）

#### ビルド

```powershell
# リポジトリのクローン
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust

# 安定版をチェックアウト
git checkout v1.0.1

# ビルド
cargo tauri build

# ビルドされたバイナリの場所
# .\target\release\
```

**注意**: Windows のビルド手順は Tauri ドキュメントに基づいており、実環境での検証は行っていません。問題が発生した場合は [GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues) で報告してください。

---

## macOS へのインストール

### 方法1: DMG を使用（準備中）

ビルド済み macOS DMG は今後のリリースで提供予定です。

### 方法2: ソースからビルド

#### 前提条件

1. **Xcode Command Line Tools のインストール**:
   ```bash
   xcode-select --install
   ```

2. **Rust のインストール**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

#### ビルド

```bash
# リポジトリのクローン
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust

# 安定版をチェックアウト
git checkout v1.0.1

# ビルド
cargo tauri build

# ビルドされたバイナリの場所
# ./target/release/bundle/
```

**注意**: macOS のビルド手順は Tauri ドキュメントに基づいており、実環境での検証は行っていません。問題が発生した場合は [GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues) で報告してください。

---

## ソースからのビルド

詳細なビルド手順については、上記のプラットフォーム別セクションを参照してください。

### 開発ビルド

```bash
cargo tauri dev
```

### 本番ビルド

```bash
cargo tauri build
```

---

## 初回起動

### 1. 初期セットアップ

KakeiBon を初めて起動すると：

1. **管理者アカウント作成画面**が表示されます
2. 管理者ユーザー名を入力（3-20文字）
3. パスワードを入力（最低16文字）
4. 「管理者を作成」をクリック

### 2. データベースの場所

KakeiBon は以下の場所にデータを保存します：
- **Linux**: `$HOME/.kakeibon/KakeiBonDB.sqlite3`
- **Windows**: `%USERPROFILE%\.kakeibon\KakeiBonDB.sqlite3`
- **macOS**: `$HOME/.kakeibon/KakeiBonDB.sqlite3`

### 3. 次のステップ

ログイン後、以下のことができます：
- 一般ユーザーアカウントの作成（管理者のみ）
- カテゴリ階層の設定
- 口座の追加（銀行口座、現金、クレジットカード）
- 取引の記録開始

詳細な使い方については、[クイックスタートガイド](QUICK_START_GUIDE.md)（準備中）を参照してください。

---

## トラブルシューティング

### 依存関係不足でビルドに失敗する

**Linux**: エラーメッセージで不足しているライブラリを確認し、パッケージマネージャーでインストールしてください。

例：
```bash
# "webkit2gtk" に関するエラーの場合
sudo apt install libwebkit2gtk-4.1-dev

# "libssl" に関するエラーの場合
sudo apt install libssl-dev
```

### データベースのアクセス権限エラー

`~/.kakeibon/` ディレクトリの権限を確認してください：

```bash
chmod 700 ~/.kakeibon/
chmod 600 ~/.kakeibon/*.sqlite3
```

### アプリケーションが起動しない

ログを確認してください：
```bash
# Linux
journalctl -xe
tail -f ~/.local/share/kakeibon/logs/*

# ターミナルから実行してコンソール出力を確認
./kakeibon
```

### その他の問題

[トラブルシューティングガイド](TROUBLESHOOTING.md) を参照するか、[GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues) で報告してください。

---

## 貢献

このガイドでカバーされていない問題に遭遇した場合：
1. 詳細を記載して [issue](https://github.com/BonoJovi/KakeiBonByRust/issues) を開いてください
2. OS、バージョン、エラーメッセージを明記してください
3. あなたのフィードバックがこのガイドの改善に役立ちます！

---

**次へ**: [クイックスタートガイド](QUICK_START_GUIDE.md)（準備中）
