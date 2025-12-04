# セットアップガイド

**最終更新日**: 2024-12-05 04:46 JST

---

## 概要

このガイドでは、KakeiBonを初めて使用する一般ユーザー向けに、ソースコードからのビルド手順と初回起動までの手順を説明します。

**注意**: 現在KakeiBonは開発段階にあり、ビルド済みバイナリの配布は行っておりません。本ガイドではソースコードからビルドする方法を説明します。

---

## 目次

1. [システム要件](#システム要件)
2. [ビルド環境のセットアップ](#ビルド環境のセットアップ)
3. [ソースコードの取得とビルド](#ソースコードの取得とビルド)
4. [初回起動](#初回起動)
5. [トラブルシューティング](#トラブルシューティング)

---

## システム要件

### 対応OS

- **Windows**: Windows 10 以降 (64-bit)
- **macOS**: macOS 10.15 (Catalina) 以降
- **Linux**: 主要ディストリビューション（Ubuntu 20.04+、Fedora、Debian等）

### ハードウェア要件

- **CPU**: 1GHz以上のプロセッサ
- **メモリ**: 最低 2GB RAM（推奨 4GB以上）
- **ストレージ**: 最低 500MB の空き容量（ビルド環境含む）
- **ディスプレイ**: 1024x768 以上の解像度

---

## ビルド環境のセットアップ

### 共通要件

以下のツールをインストールする必要があります：

1. **Rust**（1.70以降推奨）
2. **Node.js**（16.x以降）
3. **npm**（8.x以降）

### OS別の追加要件

#### Windows

- **Visual Studio Build Tools** または **Visual Studio Community**
  - "C++によるデスクトップ開発" ワークロードをインストール
  - [ダウンロード](https://visualstudio.microsoft.com/downloads/)

#### macOS

- **Xcode Command Line Tools**
  ```bash
  xcode-select --install
  ```

#### Linux (Ubuntu/Debian系)

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf
```

#### Linux (Fedora/RHEL系)

```bash
sudo dnf install -y \
    gcc \
    openssl-devel \
    gtk3-devel \
    webkit2gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel
```

---

## ソースコードの取得とビルド

### 1. ソースコードの取得

```bash
# リポジトリのクローン
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust
```

### 2. 依存関係のインストール

```bash
# Node.js依存関係のインストール
npm install

# Rust依存関係は自動的に解決されます
```

### 3. ビルド

```bash
# 開発ビルド（デバッグ版）
npm run tauri dev

# リリースビルド（本番用）
npm run tauri build
```

リリースビルドが完了すると、以下の場所に実行ファイルが生成されます：

- **Windows**: `src-tauri/target/release/KakeiBon.exe`
- **macOS**: `src-tauri/target/release/bundle/macos/KakeiBon.app`
- **Linux**: `src-tauri/target/release/kakeibo-n`

---

## 初回起動

### 1. 管理者アカウントの作成

アプリケーションを初めて起動すると、管理者セットアップ画面が表示されます。

1. **ユーザー名の入力**
   - 半角英数字、ひらがな、カタカナ、漢字が使用可能
   - 最大30文字

2. **パスワードの設定**
   - 最低16文字必要
   - 英大文字、小文字、数字、記号を組み合わせることを推奨

3. **「登録」ボタンをクリック**

### 2. ログイン

1. 作成したユーザー名とパスワードを入力
2. 「ログイン」ボタンをクリック

### 3. メイン画面の表示

ログインに成功すると、KakeiBonのメイン画面が表示され、家計簿の入力を開始できます。

---

## トラブルシューティング

### アプリケーションが起動しない

**Windows:**
- Windows Defenderやウイルス対策ソフトがブロックしている可能性があります
- セキュリティソフトの設定でKakeiBonを許可リストに追加してください

**macOS:**
- Gatekeeperによってブロックされている可能性があります
- システム環境設定 → セキュリティとプライバシー → 「このまま開く」を選択

**Linux:**
- AppImageの実行権限が付与されているか確認してください
- 依存ライブラリが不足している可能性があります（下記参照）

### Linux: 依存ライブラリエラー

AppImageを使用している場合、以下のライブラリが必要です：

**Ubuntu/Debian:**
```bash
sudo apt install libwebkit2gtk-4.1-0 libgtk-3-0
```

**Fedora:**
```bash
sudo dnf install webkit2gtk4.1 gtk3
```

### データベースエラー

- アプリケーションのデータディレクトリに書き込み権限があるか確認してください
- データベースファイル（`kakeibo.db`）が破損している場合は、バックアップから復元するか、新規作成してください

### パスワードを忘れた場合

現在のバージョンでは、パスワードリセット機能は実装されていません。データベースファイルを削除して、新しい管理者アカウントを作成する必要があります。

**警告**: データベースを削除すると、すべてのデータが失われます。定期的なバックアップを推奨します。

---

## サポート

問題が解決しない場合は、以下の方法でサポートを受けられます：

- **GitHub Issues**: [問題を報告](https://github.com/BonoJovi/KakeiBonByRust/issues)
- **Email**: プロジェクトの連絡先メールアドレスにお問い合わせください

---

## 次のステップ

- [ユーザーマニュアル](USER_MANUAL.md) - 基本的な使い方を学ぶ
- [FAQ](FAQ.md) - よくある質問と回答

---

**関連ドキュメント**:
- [開発環境セットアップガイド](../../developer/ja/guides/SETUP_GUIDE.md)
