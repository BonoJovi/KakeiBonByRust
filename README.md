# 📖 KakeiBon（家計簿）

<div align="center">

> **A Modern Household Budget App with Focus on Readability and Usability**
> **見やすさと使いやすさを追求した、モダンな家計簿アプリケーション**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2.9.3-blue.svg)](https://tauri.app/)
[![Tests](https://img.shields.io/badge/tests-613%20passing-brightgreen.svg)](#test-results--テスト結果)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/BonoJovi/KakeiBonByRust/releases)

[🇯🇵 日本語詳細](README_ja.md) | [🇬🇧 English Details](README_en.md)

</div>

---

## 💌 Message from Developer / 開発者からのメッセージ

<div style="border: 3px solid #4a90e2; padding: 20px; margin: 20px 0; background-color: #f8f9fa; font-size: 1.1em;">

### 愛すべきKakeiBonユーザの皆さんへ

いつもKakeiBonに気を留めていただき、誠にありがとうございます。
プロジェクト発案者のBonoJovi(Yoshihiro NAKAHARA)です。

**Ver.1.0.0を正式リリースいたしました！**

入出金データの入力機能が完成し、基本的な家計簿アプリケーションとしてご利用いただける状態となりました。
このmainブランチは正規リリース版です。安定版をお使いになりたい方は、こちらをご利用ください。

最新の開発版や次期バージョンの機能を試してみたい方は、[devブランチ](https://github.com/BonoJovi/KakeiBonByRust/tree/dev)をご覧ください。

今後も機能拡張を続けていく予定です。GitHubのissueやeメールでのメッセージも受け付けていますので、応援メッセージや将来的に実装してほしい機能など、ちょっとしたことでも良いのでご連絡いただければ幸いです。

それでは、引き続きKakeiBonをご愛顧頂ますよう、お願い申し上げます。

**2025-11-23 (JST) Written by Yoshihiro NAKAHARA**

---

### To All Beloved KakeiBon Users

Thank you for your continued interest in KakeiBon.
I'm BonoJovi (Yoshihiro NAKAHARA), the project initiator.

**We have officially released Ver.1.0.0!**

The transaction data input functionality is now complete, and KakeiBon is ready to be used as a basic household budget application.
This main branch is the stable release version. If you want to use the stable version, please use this branch.

If you want to try the latest development version or next version features, please check the [dev branch](https://github.com/BonoJovi/KakeiBonByRust/tree/dev).

We will continue to expand features in the future. We welcome messages via GitHub issues or email, whether it's words of encouragement or suggestions for features you'd like to see in the future—any feedback is appreciated.

Thank you for your continued support of KakeiBon.

**2025-11-23 (JST) Written by Yoshihiro NAKAHARA**

</div>

---

## 📑 Table of Contents / 目次

- [🎉 What's New in Ver.1.0.0 / Ver.1.0.0の新機能](#-whats-new-in-ver100--ver100の新機能)
- [📊 Repository Statistics / リポジトリ統計](#-repository-statistics--リポジトリ統計)
- [📚 Legacy Version / 前身プロジェクト](#-legacy-version--前身プロジェクト)
- [✨ Key Features / 主な特徴](#-key-features--主な特徴)
- [🚀 Implemented Features / 実装済み機能](#-implemented-features--実装済み機能)
- [💻 Technology Stack / 技術スタック](#-technology-stack--技術スタック)
- [📦 Installation / インストール](#-installation--インストール)
- [🧪 Test Results / テスト結果](#-test-results--テスト結果)
- [📚 Documentation / ドキュメント](#-documentation--ドキュメント)
- [🤝 Contributing / コントリビューション](#-contributing--コントリビューション)
- [📄 License / ライセンス](#-license--ライセンス)

---

## 🎉 What's New in Ver.1.0.0 / Ver.1.0.0の新機能

**Ver.1.0.0 (2025-11-23) - Initial Stable Release**

### Core Features / コア機能
- ✅ **Complete Transaction Management** / **完全な入出金管理**
  - Transaction header and detail CRUD operations
  - Smart tax calculation with automatic rounding detection
  - Flexible filtering and pagination
  - 入出金ヘッダ・明細のCRUD操作
  - スマート税計算と端数処理自動検出
  - 柔軟なフィルタリングとページネーション

- ✅ **Master Data Management** / **マスタデータ管理**
  - Category management (hierarchical: Major/Middle/Minor)
  - Account management
  - Shop management
  - Manufacturer & Product management
  - 費目管理（階層的：大分類/中分類/小分類）
  - 口座管理
  - 店舗管理
  - メーカー・商品管理

- ✅ **User & Security** / **ユーザー・セキュリティ**
  - Multi-user support (Admin/General users)
  - Argon2id password hashing
  - AES-256-GCM data encryption
  - マルチユーザー対応（管理者/一般ユーザー）
  - Argon2idパスワードハッシュ化
  - AES-256-GCMデータ暗号化

- ✅ **Accessibility & i18n** / **アクセシビリティ・国際化**
  - Full multilingual support (Japanese/English)
  - Font size customization (10-30px)
  - Keyboard navigation support
  - 完全多言語対応（日本語/英語）
  - フォントサイズカスタマイズ（10-30px）
  - キーボードナビゲーション対応

### Technical Achievements / 技術的達成
- 📊 **613 tests** passing (100% success rate)
- 📝 **~35,478 lines** of code
- 🌐 **992 i18n resources** (496 unique keys, 2 languages)
- 🤖 **100% AI-assisted development** (GitHub Copilot + Claude)

---

<!-- STATS_START -->
## 📊 Repository Statistics / リポジトリ統計

<div align="center">

### 📈 Daily Traffic / 日次トラフィック

![Daily Traffic Stats](docs/stats_graph_daily.png)

### 📊 Cumulative Traffic / 累積トラフィック

![Cumulative Traffic Stats](docs/stats_graph_cumulative.png)

| Metric | Count |
|--------|-------|
| 👁️ **Total Views** / 総閲覧数 | **583** |
| 📦 **Total Clones** / 総クローン数 | **170** |

*Last Updated / 最終更新: 2025-11-22 00:07 UTC*

</div>
<!-- STATS_END -->

---

## 📚 Legacy Version / 前身プロジェクト

**Looking for the stable Lazarus/Free Pascal version? / 安定版（Lazarus/Free Pascal版）をお探しですか？**

👉 **[KakeiBon (Original)](https://github.com/BonoJovi/KakeiBon)** - すぐに使える完成版！

The original KakeiBon is a **fully functional household budget app** ready to use right now!
元祖KakeiBonは、**今すぐ使える完成版の家計簿アプリ**です！

**Key Differences / 主な違い:**
- ✅ **Stable & Production-Ready** / **安定版・本番利用可能**
- 📦 **Pre-built Binaries Available** / **ビルド済みバイナリあり**（[Releases](https://github.com/BonoJovi/KakeiBon/releases/)）
- 🇯🇵 **Japanese Interface Only** / **日本語インターフェース専用**
- 🖥️ **Linux & Windows Support** / **Linux & Windows 対応**
- 🔤 **Large Fonts & Accessibility** / **大きな文字とアクセシビリティ**

**Why Rust Version? / なぜRust版？**

This Rust rewrite offers:
- ⚡ **Better Performance** / より高速
- 🔒 **Enhanced Security** (Argon2 + AES-256-GCM) / 強化されたセキュリティ
- 🌐 **Full Multilingual Support** / 完全多言語対応
- 🎨 **Modern Architecture** / モダンなアーキテクチャ
- 🔮 **Future Expandability** / 将来の拡張性

💡 **Try both and choose what works best for you!** / 両方試して、お好みの方をお使いください！

---

## ✨ Key Features / 主な特徴

### 🎨 NOT Vibe Coding / NOTバイブコーディング
Built with **proper planning and documentation first**, not vibes
雰囲気ではなく、**きちんとした計画とドキュメント作成**を先に行う開発スタイル

### 👤 Clear User-First Policy / 明確なユーザーファーストポリシーによる設計
Every feature is designed with **explicit user needs and usability** in mind
すべての機能は**明確なユーザーニーズと使いやすさ**を念頭に置いて設計されています

### 🔤 Large, Easy-to-Read Text / 大きな文字で見やすい
Designed with high visibility in mind - comfortable for long-term use
視認性を重視した設計で、長時間の使用でも目が疲れにくい

### 🎯 Intuitive User Interface / 直感的な操作性
Simple and clear UI that anyone can master quickly
誰でもすぐに使いこなせる、シンプルで分かりやすいUI

### ♿ Accessibility Support / アクセシビリティ対応
- **Font Size Adjustment**: Small/Medium/Large/Custom (10-30px)
  **フォントサイズ調整**: 小/中/大/カスタム（10-30px）
- **Keyboard Navigation**: Fully supported
  **キーボードナビゲーション**: 完全対応
- **Focus Indicators**: Clear visual feedback
  **フォーカスインジケーター**: 明確な視覚フィードバック

### 🌐 Multilingual Support / 多言語対応
Switch between Japanese and English seamlessly
日本語・英語の切り替えが可能

### 🔒 Strong Security / 強固なセキュリティ
- Argon2id password hashing / パスワードハッシュ化
- AES-256-GCM data encryption / データ暗号化
- Role-based access control / ロールベースのアクセス制御

---

## 🚀 Implemented Features / 実装済み機能

| Feature / 機能 | Description / 説明 | Status / ステータス |
|----------------|-------------------|---------------------|
| 🔐 **Session Management**<br/>**セッション管理** | In-memory session state management<br/>メモリ内セッション状態管理 | ✅ Complete<br/>完成 |
| 💰 **Category Management**<br/>**費目管理** | Hierarchical category system (Major/Middle/Minor)<br/>大分類・中分類・小分類の階層的管理 | ✅ Complete<br/>完成 |
| 👥 **User Management**<br/>**ユーザー管理** | Multi-user support (Admin/General)<br/>マルチユーザー対応（管理者/一般） | ✅ Complete<br/>完成 |
| 🏦 **Account Management**<br/>**口座管理** | Account master data management<br/>口座マスタ管理 | ✅ Complete<br/>完成 |
| 🏪 **Shop Management**<br/>**店舗管理** | Shop master data management<br/>店舗マスタ管理 | ✅ Complete<br/>完成 |
| 🏭 **Manufacturer Management**<br/>**メーカー管理** | Manufacturer master data with IS_DISABLED feature<br/>IS_DISABLED機能付きメーカーマスタ管理 | ✅ Complete<br/>完成 |
| 📦 **Product Management**<br/>**商品管理** | Product master data with manufacturer linkage<br/>メーカー連携付き商品マスタ管理 | ✅ Complete<br/>完成 |
| 🌍 **Multilingual**<br/>**多言語対応** | Dynamic language switching (JP/EN) - 992 resources<br/>日本語・英語の動的切り替え - 992リソース | ✅ Complete<br/>完成 |
| 🔧 **Customization**<br/>**カスタマイズ** | Font size, language preferences<br/>フォントサイズ、言語設定 | ✅ Complete<br/>完成 |
| 📝 **Transaction Management**<br/>**入出金管理** | Header-level CRUD, filters, pagination<br/>ヘッダレベルCRUD、フィルター、ページネーション | ✅ Complete<br/>完成 |
| 🧾 **Transaction Details**<br/>**入出金明細** | CRUD operations with smart tax calculation, automatic rounding detection<br/>スマート税計算付きCRUD操作、端数処理自動検出 | ✅ Complete<br/>完成 |

---

## 💻 Technology Stack / 技術スタック

| Category / カテゴリ | Technology / 技術 | Details / 詳細 |
|---------------------|-------------------|----------------|
| **Frontend** / **フロントエンド** | Vanilla JavaScript + HTML5 + CSS3 | ES6 Modules |
| **Backend** / **バックエンド** | Rust + Tauri | v2.8.5 |
| **Database** / **データベース** | SQLite | WAL mode |
| **Security** / **セキュリティ** | Argon2id + AES-256-GCM | Password hashing + Data encryption |
| **Testing** / **テスト** | Jest + Cargo Test | 613 tests passing (Rust: 201, JS: 412) |
| **i18n Resources** / **翻訳** | JSON-based | 992 resources (496 unique keys, 2 languages) |
| **Code Lines** / **コード行数** | Total / 合計 | ~35,478 lines (Rust: 13,870, JS: 8,810, HTML: 3,355, CSS: 6,109, SQL: 3,334) |

---

## 📦 Installation / インストール

### Prerequisites / 前提条件
- Rust 1.70+ (Install via [rustup](https://rustup.rs/) / [rustup](https://rustup.rs/)でインストール)
- Node.js 18+ (for Tauri CLI / Tauri CLI用)

### Build & Run / ビルド・実行

```bash
# Clone repository / リポジトリをクローン
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust

# Run in development mode / 開発モードで起動
cargo tauri dev

# Production build / プロダクションビルド
cargo tauri build
```

---

## 🧪 Test Results / テスト結果

```
Backend (Rust) / バックエンド:    201 passing
Frontend (JavaScript) / フロント:  412 passing
Total Tests / 総テスト数:          613 passing ✅
Success Rate / 成功率:            100%
```

See [TEST_SUMMARY.md](docs/ja/TEST_SUMMARY.md) for details / 詳細は [TEST_SUMMARY.md](docs/ja/TEST_SUMMARY.md) を参照

---

## 📚 Documentation / ドキュメント

### For Users / ユーザー向け
- 🔧 **Troubleshooting / トラブルシューティング**
  - [English](docs/en/TROUBLESHOOTING.md) / [日本語](docs/ja/TROUBLESHOOTING.md)

### For Developers / 開発者向け

#### Core Guides / コアガイド
- 🏗️ **Developer Guide / 開発者ガイド**
  - [English](docs/en/DEVELOPER_GUIDE.md) / [日本語](docs/ja/DEVELOPER_GUIDE.md)
- 🧪 **Testing Strategy / テスト戦略**
  - [English](docs/en/TESTING.md) / [日本語](docs/ja/TESTING.md)
- 📊 **Test Summary / テストサマリー**
  - [English](docs/en/TEST_SUMMARY.md) / [日本語](docs/ja/TEST_SUMMARY.md)

#### API Documentation / API ドキュメント
- 📁 **Category Management API / 費目管理 API**
  - [English](docs/en/API_CATEGORY.md) / [日本語](docs/ja/API_CATEGORY.md)
- 🏪 **Shop Management API / 店舗管理 API**
  - [English](docs/en/API_SHOP.md) / [日本語](docs/ja/API_SHOP.md)
- 🏭 **Manufacturer Management API / メーカー管理 API**
  - [English](docs/en/API_MANUFACTURER.md) / [日本語](docs/ja/API_MANUFACTURER.md)
- 📦 **Product Management API / 商品管理 API**
  - [English](docs/en/API_PRODUCT.md) / [日本語](docs/ja/API_PRODUCT.md)
- 💰 **Transaction Management API / 入出金管理 API**
  - [English](docs/en/API_TRANSACTION.md) / [日本語](docs/ja/API_TRANSACTION.md)

#### UI Documentation / UI ドキュメント
- 👥 **User Management UI / ユーザー管理 UI**
  - [English](docs/en/USER_MANAGEMENT_UI.md) / [日本語](docs/ja/USER_MANAGEMENT_UI.md)
- 🏦 **Account Management UI / 口座管理 UI**
  - [English](docs/en/ACCOUNT_MANAGEMENT_UI.md) / [日本語](docs/ja/ACCOUNT_MANAGEMENT_UI.md)
- 📁 **Category Management UI / 費目管理 UI**
  - [English](docs/en/CATEGORY_MANAGEMENT_UI.md) / [日本語](docs/ja/CATEGORY_MANAGEMENT_UI.md)
- 🏭 **Manufacturer & Product Management / メーカー・商品管理**
  - [English](docs/en/MANUFACTURER_PRODUCT_MANAGEMENT.md) / [日本語](docs/ja/MANUFACTURER_PRODUCT_MANAGEMENT.md)
- 💰 **Transaction Management UI / 入出金管理 UI**
  - [English](docs/en/TRANSACTION_MANAGEMENT_UI_V2.md) / [日本語](docs/ja/TRANSACTION_MANAGEMENT_UI_V2.md)

#### Feature Implementation / 機能実装
- 🧮 **Tax Calculation Logic / 税計算ロジック**
  - [Bilingual / 日英併記](docs/tax-calculation-logic.md)
- 🌐 **I18N Implementation / 国際化実装**
  - [English](docs/en/I18N_IMPLEMENTATION.md) / [日本語](docs/ja/I18N_IMPLEMENTATION.md)
- 🌍 **I18N Resources / 国際化リソース**
  - [English](docs/en/I18N_RESOURCES.md) / [日本語](docs/ja/I18N_RESOURCES.md)
- 🌐 **Dynamic Language Menu / 動的言語メニュー**
  - [English](docs/en/DYNAMIC_LANGUAGE_MENU.md) / [日本語](docs/ja/DYNAMIC_LANGUAGE_MENU.md)
- 🔤 **Font Size Implementation / フォントサイズ実装**
  - [English](docs/en/font-size-implementation.md) / [日本語](docs/ja/font-size-implementation.md)
- ♿ **Accessibility Indicators / アクセシビリティインジケーター**
  - [English](docs/en/ACCESSIBILITY_INDICATORS.md) / [日本語](docs/ja/ACCESSIBILITY_INDICATORS.md)
- 🚫 **IS_DISABLED Implementation / IS_DISABLED実装**
  - [English](docs/en/IS_DISABLED_IMPLEMENTATION_GUIDE.md) / [日本語](docs/ja/IS_DISABLED_IMPLEMENTATION_GUIDE.md)

#### Database & Security / データベース・セキュリティ
- 🗄️ **Database Configuration / データベース設定**
  - [English](docs/en/DATABASE_CONFIGURATION.md) / [日本語](docs/ja/DATABASE_CONFIGURATION.md)
- 🔄 **Database Migration / データベースマイグレーション**
  - [English](docs/en/DATABASE_MIGRATION.md) / [日本語](docs/ja/DATABASE_MIGRATION.md)
- 🔐 **Encryption Management / 暗号化管理**
  - [English](docs/en/ENCRYPTION_MANAGEMENT.md) / [日本語](docs/ja/ENCRYPTION_MANAGEMENT.md)
- 👤 **User Management / ユーザー管理**
  - [English](docs/en/USER_MANAGEMENT.md) / [日本語](docs/ja/USER_MANAGEMENT.md)
- ⚙️ **Settings Management / 設定管理**
  - [English](docs/en/SETTINGS_MANAGEMENT.md) / [日本語](docs/ja/SETTINGS_MANAGEMENT.md)

#### Design Documents / 設計ドキュメント
- 💰 **Transaction Design V2 / 入出金設計 V2**
  - [English](docs/en/TRANSACTION_DESIGN_V2.md) / [日本語](docs/ja/TRANSACTION_DESIGN_V2.md)

### Project Information / プロジェクト情報
- 👥 **Project Participants / プロジェクト参加者**
  - [English](docs/en/PROJECT_PARTICIPANTS.md) / [日本語](docs/ja/PROJECT_PARTICIPANTS.md)

---

## 🤝 Contributing / コントリビューション

Contributions are welcome! / プルリクエストを歓迎します！

1. Fork this repository / このリポジトリをフォーク
2. Create a feature branch / フィーチャーブランチを作成
   `git checkout -b feature/AmazingFeature`
3. Commit your changes / 変更をコミット
   `git commit -m 'Add some AmazingFeature'`
4. Push to the branch / ブランチにプッシュ
   `git push origin feature/AmazingFeature`
5. Open a Pull Request / プルリクエストを開く

See [CONTRIBUTING.md](CONTRIBUTING.md) for details / 詳細は [CONTRIBUTING.md](CONTRIBUTING.md) を参照

---

## 📄 License / ライセンス

This project is licensed under the terms in the [LICENSE](LICENSE) file.
このプロジェクトは [LICENSE](LICENSE) の下でライセンスされています。

---

<div align="center">

**Made with ❤️ and Rust**

**Ver.1.0.0 Stable Release**

[Report Bug / バグ報告](https://github.com/BonoJovi/KakeiBonByRust/issues) · [Request Feature / 機能リクエスト](https://github.com/BonoJovi/KakeiBonByRust/issues) · [Development Version / 開発版](https://github.com/BonoJovi/KakeiBonByRust/tree/dev)

</div>
