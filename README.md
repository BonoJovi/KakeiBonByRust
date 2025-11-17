# 📖 KakeiBon（家計簿）

<div align="center">

> **A Modern Household Budget App with Focus on Readability and Usability**  
> **見やすさと使いやすさを追求した、モダンな家計簿アプリケーション**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2.8.5-blue.svg)](https://tauri.app/)
[![Tests](https://img.shields.io/badge/tests-569%20passing-brightgreen.svg)](#test-results--テスト結果)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[🇯🇵 日本語詳細](README_ja.md) | [🇬🇧 English Details](README_en.md)

</div>

---

## 📑 Table of Contents / 目次

- [🚧 Development Status / 開発状況](#-development-status--開発状況)
- [📊 Repository Statistics / リポジトリ統計](#-repository-statistics--リポジトリ統計)
- [📚 Legacy Version / 前身プロジェクト](#-legacy-version--前身プロジェクト)
- [✨ Key Features / 主な特徴](#-key-features--主な特徴)
- [🚀 Current Features / 実装済み機能](#-current-features--実装済み機能)
- [💻 Technology Stack / 技術スタック](#-technology-stack--技術スタック)
- [📦 Installation / インストール](#-installation--インストール)
- [🧪 Test Results / テスト結果](#-test-results--テスト結果)
- [📚 Documentation / ドキュメント](#-documentation--ドキュメント)
- [🤝 Contributing / コントリビューション](#-contributing--コントリビューション)
- [📄 License / ライセンス](#-license--ライセンス)
- [🌟 Development Roadmap / 開発ロードマップ](#-development-roadmap--開発ロードマップ)

---

## 🚧 Development Status / 開発状況

**🔥 Actively Under Development / 鋭意開発中**

Development is progressing smoothly, and we strive to update daily!  
開発は順調に進んでおり、できるだけ日々更新するようにしています！

**Project Started / プロジェクト開始**: 2025-10-22 (JST)  
**Last Updated / 最終更新**: 2025-11-17 (JST)

> **🤖 AI-Assisted Development / AI支援開発**  
> This project's source code and documentation are **100% generated** with the assistance of generative AI (GitHub Copilot, Claude), supervised and reviewed by the developer. This demonstrates the potential of AI-assisted development.  
> 本プロジェクトのソースコードおよびドキュメントは、生成AI（GitHub Copilot、Claude）の支援により**100%生成**され、開発者による監修とレビューを経ています。これは、AI支援開発の可能性を示す事例です。

<!-- STATS_START -->
## 📊 Repository Statistics / リポジトリ統計

<div align="center">

### 📈 Daily Traffic / 日次トラフィック

![Daily Traffic Stats](docs/stats_graph_daily.png)

### 📊 Cumulative Traffic / 累積トラフィック

![Cumulative Traffic Stats](docs/stats_graph_cumulative.png)

| Metric | Count |
|--------|-------|
| 👁️ **Total Views** / 総閲覧数 | **533** |
| 📦 **Total Clones** / 総クローン数 | **151** |

*Last Updated / 最終更新: 2025-11-17 12:04 UTC*

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

## 🚀 Current Features / 実装済み機能

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
| 🧾 **Transaction Details**<br/>**入出金明細** | Detail-level input with smart tax calculation<br/>スマート税計算付き明細入力 | 🚧 In Progress<br/>開発中 |
| 📊 **Reports**<br/>**集計・レポート** | Monthly/annual summaries, graphs<br/>月次・年次レポート、グラフ | 📅 Planned<br/>予定 |

---

## 💻 Technology Stack / 技術スタック

```
Frontend / フロントエンド:  Vanilla JavaScript (ES6 Modules) + HTML5 + CSS3
Backend / バックエンド:     Rust + Tauri v2.8.5
Database / データベース:    SQLite (WAL mode)
Security / セキュリティ:   Argon2id + AES-256-GCM
Testing / テスト:          569 tests passing (Rust: 165, JS: 404)
i18n Resources / 翻訳:     992 resources (496 unique keys, 2 languages)
Code Lines / コード行数:    ~26,844 lines (Rust: 11,879, JS: 6,830, HTML: 2,758, CSS: 3,371, SQL: 2,006)
```

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
Backend (Rust) / バックエンド:    160 passing
Frontend (JavaScript) / フロント:  404 passing
Total Tests / 総テスト数:          564 passing ✅
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

## 🌟 Development Roadmap / 開発ロードマップ

- [x] User management / ユーザー管理機能
- [x] Category management / 費目管理機能
- [x] Multilingual support / 多言語対応
- [x] Accessibility features / アクセシビリティ機能
- [ ] Transaction management / 入出金データ管理 (**In Progress / 開発中**)
- [ ] Monthly/annual reports / 月次・年次集計
- [ ] Data export (CSV) / データエクスポート（CSV）
- [ ] Backup & restore / バックアップ・リストア

---

<div align="center">

**Made with ❤️ and Rust**

[Report Bug / バグ報告](https://github.com/BonoJovi/KakeiBonByRust/issues) · [Request Feature / 機能リクエスト](https://github.com/BonoJovi/KakeiBonByRust/issues)

</div>
