# 📖 KakeiBon（家計簿）

<div align="center">

> **A Modern Household Budget App with Focus on Readability and Usability**  
> **見やすさと使いやすさを追求した、モダンな家計簿アプリケーション**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2.9.3-blue.svg)](https://tauri.app/)
[![Tests](https://img.shields.io/badge/tests-527%20passing-brightgreen.svg)](#test-results--テスト結果)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[🇯🇵 日本語詳細](README_ja.md) | [🇬🇧 English Details](README_en.md)

</div>

---

## 💌 Message from Developer / 開発者からのメッセージ

<div style="border: 3px solid #4a90e2; padding: 20px; margin: 20px 0; background-color: #f8f9fa; font-size: 1.1em;">

### 愛すべきKakeiBonユーザの皆さんへ

いつもKakeiBonに気を留めていただき、誠にありがとうございます。
プロジェクト発案者のBonoJovi(Yoshihiro NAKAHARA)です。

**Ver.1.0.9を正式リリースいたしました！**

Ver.1.0.1からVer.1.0.9への継続的なバージョンアップでは、CI/CDパイプラインの導入により、Windows/macOS/Linux向けのマルチプラットフォーム対応とリリース自動化を実現しました。v1.0.8ではテストドキュメントの大幅な整理を行い、v1.0.9では集計機能のテスト安定性を向上させました。この大規模な開発基盤の整備により、プロジェクトの開発効率が劇的に向上し、今後の継続的な機能改善とリリースがスムーズに行えるようになりました。

入出金データの入力機能が完成し、基本的な家計簿アプリケーションとしてご利用いただける状態となりました。
安定版リリースをご利用になりたい方は、[mainブランチ](https://github.com/BonoJovi/KakeiBonByRust/tree/main)をご参照ください。

現在ご覧いただいているdevブランチは開発版となり、次期バージョンの機能を開発中です。
最新の機能をいち早く試してみたい方は、こちらのdevブランチをお使いください。

今後は集計・レポート機能の実装を進めていく予定です。細々した機能も追々実装していきますので、機能拡張にご期待いただければと思います。
GitHubのissueやeメールでのメッセージも受け付けていますので、応援メッセージや将来的に実装してほしい機能など、ちょっとしたことでも良いのでご連絡いただければ幸いです。

それでは、引き続きKakeiBonをご愛顧頂ますよう、お願い申し上げます。

**2025-11-30 (JST) Written by Yoshihiro NAKAHARA**

---

### To All Beloved KakeiBon Users

Thank you for your continued interest in KakeiBon.
I'm BonoJovi (Yoshihiro NAKAHARA), the project initiator.

**We have officially released Ver.1.0.9!**

The continuous version updates from Ver.1.0.1 to Ver.1.0.9 reflect the significant impact of introducing a CI/CD pipeline, enabling multi-platform support (Windows/macOS/Linux) and automated releases. In v1.0.8, we performed major test documentation refactoring, and in v1.0.9, we improved aggregation feature test stability. This large-scale infrastructure improvement has dramatically enhanced development efficiency and enables smooth continuous feature improvements and releases going forward.

The transaction data input functionality is now complete, and KakeiBon is ready to be used as a basic household budget application.
If you would like to use the stable release version, please refer to the [main branch](https://github.com/BonoJovi/KakeiBonByRust/tree/main).

The dev branch you are currently viewing is the development version, where we are working on features for the next release.
If you want to try the latest features early, please use this dev branch.

We plan to proceed with implementing aggregation and reporting features next. We will continue to add various features incrementally, so please look forward to continuous enhancements.
We welcome messages via GitHub issues or email, whether it's words of encouragement or suggestions for features you'd like to see in the future—any feedback is appreciated.

Thank you for your continued support of KakeiBon.

**2025-11-30 (JST) Written by Yoshihiro NAKAHARA**

</div>

---

## 🤝 Join Our Community / コミュニティに参加

**Help make KakeiBon accessible to users worldwide!**
**KakeiBonを世界中のユーザーに届けるお手伝いをしてください！**

We welcome **all types of contributions** - not just code! Whether you're a developer, translator, or user, there's a way for you to contribute.
**あらゆる形の貢献**を歓迎します—コードだけではありません！開発者、翻訳者、ユーザーのいずれであっても、貢献する方法があります。

---

### 🌍 Translators Wanted! / 翻訳者募集！
**No programming experience needed! / プログラミング経験不要！**

Help make KakeiBon available in your language:
あなたの言語でKakeiBonを利用可能にするお手伝いをしてください：

- ✅ **Currently Supported / 現在サポート**: Japanese (ja), English (en)
- 🌐 **Seeking / 募集中**: Chinese (zh), Korean (ko), French (fr), German (de), Spanish (es), and more!

**How to contribute:**
- Add support for a new language / 新しい言語のサポートを追加
- Improve existing translations / 既存の翻訳を改善
- Review translation accuracy / 翻訳の正確性をレビュー

📖 **[Translation Guide](docs/developer/en/guides/translation-guide.md)** | **[翻訳ガイド](docs/developer/ja/guides/translation-guide.md)**
🆕 **[Submit Translation Request](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=translation.yml)**

---

### 🧪 Testers Wanted! / テスター募集！
**No programming experience needed! / プログラミング経験不要！**

**🎉 NEW: v1.0.7 Multi-Platform Binaries Now Available!**
**🎉 新着: v1.0.7でマルチプラットフォームバイナリが利用可能に！**

We've just released Windows and macOS binaries through our new CI/CD pipeline, but we **urgently need real hardware testing** as the developer doesn't have access to Windows/Mac environments!

CI/CDパイプライン導入によりWindows/macOSバイナリのリリースを開始しましたが、開発者がWindows/Mac環境を持っていないため、**実機での動作確認が緊急に必要です**！

**Platform Status:**
- ✅ **Linux**: Verified and tested by developer / 開発者により検証済み・テスト済み
- ⚠️ **Windows**: **Binary available but untested on real hardware!** / **バイナリは利用可能だが実機未テスト！**
- ⚠️ **macOS (Intel & Apple Silicon)**: **Binary available but untested on real hardware!** / **バイナリは利用可能だが実機未テスト！**

**What we need from you:**
**お願いしたいこと：**
- 🔍 Download and test the latest release on your Windows/Mac / Windows/Macで最新リリースをダウンロード＆テスト
- 🐛 Report any bugs or issues you encounter / 遭遇したバグや問題を報告
- ✅ Confirm if basic features work correctly / 基本機能が正常に動作するか確認
- 💬 Share your experience (UI/UX feedback welcome!) / 使用感を共有（UI/UXフィードバック歓迎！）
- 🆕 **Review test case validity** - Check if our 463+ tests make sense! / **テストケースの妥当性をレビュー** - 463件以上のテストが妥当かチェック！

**Download:** [Latest Release](https://github.com/BonoJovi/KakeiBonByRust/releases/latest)

📖 **[Test Overview](docs/testing/en/TEST_OVERVIEW.md)** | **[テスト概要](docs/testing/ja/TEST_OVERVIEW.md)**
📘 **[Backend Test Index](docs/testing/en/BACKEND_TEST_INDEX.md)** (201 tests) | **[バックエンドテストインデックス](docs/testing/ja/BACKEND_TEST_INDEX.md)** (201件)
📗 **[Frontend Test Index](docs/testing/en/FRONTEND_TEST_INDEX.md)** (262+ tests) | **[フロントエンドテストインデックス](docs/testing/ja/FRONTEND_TEST_INDEX.md)** (262件以上)
🆕 **[Submit Testing Feedback](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=testing-feedback.yml)**

---

### 💡 Feature Requests & Feedback / 機能リクエスト & フィードバック

Have ideas to make KakeiBon better?
KakeiBonをより良くするアイデアはありますか？

- 🆕 **[Submit Feature Request](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=feature_request.md)**
- 🐛 **[Report a Bug](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=bug_report.md)**
- 💬 **[Join Discussions](https://github.com/BonoJovi/KakeiBonByRust/discussions)**

---

### 💻 Developers / 開発者

For code contributions:
コード貢献について：

- 📋 **[Contributing Guide](CONTRIBUTING.md)**
- 🔧 **[Development Documentation](docs/developer/en/)**

---

**Every contribution, no matter how small, makes KakeiBon better for everyone.**
**どんなに小さな貢献でも、KakeiBonをみんなにとってより良いものにします。**

**Thank you for your support! / ご支援ありがとうございます！**

---

## 📝 Technical Articles / 技術記事

**Read more about AI-assisted development and other technical insights on Qiita!**  
**AI支援開発やその他の技術的知見についてQiitaで詳しく紹介しています！**

We share articles not only about KakeiBon development but also about AI collaboration techniques, design philosophy, and various technical topics.  
KakeiBon開発だけでなく、AI協働手法、設計思想、その他さまざまな技術トピックについて記事を公開しています。

👉 **[Visit Qiita Profile / Qiitaプロフィールを見る](https://qiita.com/BonoJovi/)**

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
**Last Updated / 最終更新**: 2025-11-30 (JST)

> **🤖 AI-Assisted Development / AI支援開発**  
> This project's source code and documentation are **100% generated** with the assistance of generative AI (GitHub Copilot, Claude), supervised and reviewed by the developer. This demonstrates the potential of AI-assisted development.  
> 本プロジェクトのソースコードおよびドキュメントは、生成AI（GitHub Copilot、Claude）の支援により**100%生成**され、開発者による監修とレビューを経ています。これは、AI支援開発の可能性を示す事例です。
> 
> 📊 **[See AI Development Metrics & Quality Analysis →](docs/etc/AI_DEVELOPMENT_METRICS.md)**  
> **[AI開発の生産性と品質分析を見る →](docs/etc/AI_DEVELOPMENT_METRICS.md)**

<!-- STATS_START -->
## 📊 Repository Statistics / リポジトリ統計

<div align="center">

### 📈 Daily Traffic / 日次トラフィック

![Daily Traffic Stats](docs/stats_graph_daily.png)

### 📊 Cumulative Traffic / 累積トラフィック

![Cumulative Traffic Stats](docs/stats_graph_cumulative.png)

| Metric | Count |
|--------|-------|
| 👁️ **Total Views** / 総閲覧数 | **1,156** |
| 📦 **Total Clones** / 総クローン数 | **909** |

*Last Updated / 最終更新: 2025-12-07 01:37 UTC*

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

### 🏗️ Enterprise-Grade Architecture / エンタープライズグレードのアーキテクチャ
**Session-Based Authentication** throughout all 52 API functions  
**セッションベース認証**を全52個のAPI関数で実装

- 🔐 **Secure Session Management** / セキュアなセッション管理
- 👥 **User Isolation** / ユーザーデータの完全分離
- ✅ **Zero Hardcoded User IDs** / ハードコードされたユーザーID排除
- 🧪 **527 Tests (100% Pass)** / 527テスト（100%合格）

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
| 🧾 **Transaction Details**<br/>**入出金明細** | CRUD operations with smart tax calculation, automatic rounding detection<br/>スマート税計算付きCRUD操作、端数処理自動検出 | ✅ Complete<br/>完成 |
| 📊 **Reports**<br/>**集計・レポート** | Monthly/annual summaries, graphs<br/>月次・年次レポート、グラフ | 🚧 In Progress<br/>開発中 |

---

## 💻 Technology Stack / 技術スタック

| Category / カテゴリ | Technology / 技術 | Details / 詳細 |
|---------------------|-------------------|----------------|
| **Frontend** / **フロントエンド** | Vanilla JavaScript + HTML5 + CSS3 | ES6 Modules |
| **Backend** / **バックエンド** | Rust + Tauri | v2.8.5 |
| **Database** / **データベース** | SQLite | WAL mode |
| **Security** / **セキュリティ** | Argon2id + AES-256-GCM | Password hashing + Data encryption |
| **Testing** / **テスト** | Jest + Cargo Test | 527 tests passing (Rust: 201, JS: 326) |
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
Backend (Rust) / バックエンド:    201 passing ✅
Frontend (JavaScript) / フロント:  326 passing ✅
Total Tests / 総テスト数:          527 passing ✅
Success Rate / 成功率:            100%
```

**Recent Improvements / 最近の改善**:
- ✅ **Session Management Integration** / **セッション管理統合** (2025-11-30)
  - All 52 API functions now use session-based authentication
  - Enhanced security with proper user isolation
  - Removed hardcoded user IDs throughout the codebase

- ✅ **Test Quality Enhancement** / **テスト品質向上** (2025-11-30)
  - Added explicit assertions to delegated tests
  - Improved test readability and maintainability
  - Enterprise-grade test structure achieved

**Test Count Methodology / テスト件数計測方法** (Updated 2025-11-30):
- **Previous count (613)**: Included nested `describe` blocks and test structure
- **Current count (527)**: Counts only actual executable test cases
- **Reason for change / 変更理由**: Improved accuracy and industry-standard methodology
- **Note / 注意**: No tests were removed; this is purely a measurement refinement
  テストは削除されておらず、計測方法の精密化のみです

See [Test Overview](docs/testing/en/TEST_OVERVIEW.md) for details / 詳細は [テスト概要](docs/testing/ja/TEST_OVERVIEW.md) を参照

---

## 📚 Documentation / ドキュメント

### For Users / ユーザー向け
- 🔧 **Troubleshooting / トラブルシューティング**
  - [English](docs/user/en/TROUBLESHOOTING.md) / [日本語](docs/user/ja/TROUBLESHOOTING.md)

### For Developers / 開発者向け

#### Core Guides / コアガイド
- 🏗️ **Developer Guide / 開発者ガイド**
  - [English](docs/developer/en/guides/DEVELOPER_GUIDE.md) / [日本語](docs/developer/ja/guides/DEVELOPER_GUIDE.md)
- 🧪 **Testing Documentation / テストドキュメント**
  - 📖 **[Test Overview](docs/testing/en/TEST_OVERVIEW.md)** / **[テスト概要](docs/testing/ja/TEST_OVERVIEW.md)** - Test strategy and execution guide
  - 📘 **[Backend Test Index](docs/testing/en/BACKEND_TEST_INDEX.md)** / **[バックエンドテストインデックス](docs/testing/ja/BACKEND_TEST_INDEX.md)** - Complete Rust test list (201 tests)
  - 📗 **[Frontend Test Index](docs/testing/en/FRONTEND_TEST_INDEX.md)** / **[フロントエンドテストインデックス](docs/testing/ja/FRONTEND_TEST_INDEX.md)** - Complete JavaScript test list (262+ tests)

#### API Documentation / API ドキュメント
- 📁 **Category Management API / 費目管理 API**
  - [English](docs/developer/en/api/API_CATEGORY.md) / [日本語](docs/developer/ja/api/API_CATEGORY.md)
- 🏪 **Shop Management API / 店舗管理 API**
  - [English](docs/developer/en/api/API_SHOP.md) / [日本語](docs/developer/ja/api/API_SHOP.md)
- 🏭 **Manufacturer Management API / メーカー管理 API**
  - [English](docs/developer/en/api/API_MANUFACTURER.md) / [日本語](docs/developer/ja/api/API_MANUFACTURER.md)
- 📦 **Product Management API / 商品管理 API**
  - [English](docs/developer/en/api/API_PRODUCT.md) / [日本語](docs/developer/ja/api/API_PRODUCT.md)
- 💰 **Transaction Management API / 入出金管理 API**
  - [English](docs/developer/en/api/API_TRANSACTION.md) / [日本語](docs/developer/ja/api/API_TRANSACTION.md)

#### UI Documentation / UI ドキュメント
- 👥 **User Management UI / ユーザー管理 UI**
  - [English](docs/developer/en/guides/USER_MANAGEMENT_UI.md) / [日本語](docs/developer/ja/guides/USER_MANAGEMENT_UI.md)
- 🏦 **Account Management UI / 口座管理 UI**
  - [English](docs/developer/en/guides/ACCOUNT_MANAGEMENT_UI.md) / [日本語](docs/developer/ja/guides/ACCOUNT_MANAGEMENT_UI.md)
- 📁 **Category Management UI / 費目管理 UI**
  - [English](docs/developer/en/guides/CATEGORY_MANAGEMENT_UI.md) / [日本語](docs/developer/ja/guides/CATEGORY_MANAGEMENT_UI.md)
- 🏭 **Manufacturer & Product Management / メーカー・商品管理**
  - [English](docs/etc/MANUFACTURER_PRODUCT_MANAGEMENT.md) / [日本語](docs/etc/MANUFACTURER_PRODUCT_MANAGEMENT.md)
- 💰 **Transaction Management UI / 入出金管理 UI**
  - [English](docs/developer/en/guides/TRANSACTION_MANAGEMENT_UI_V2.md) / [日本語](docs/developer/ja/guides/TRANSACTION_MANAGEMENT_UI_V2.md)

#### Feature Implementation / 機能実装
- 🧮 **Tax Calculation Logic / 税計算ロジック**
  - [Bilingual / 日英併記](docs/design/architecture/tax-calculation-logic.md)
- 🌐 **I18N Implementation / 国際化実装**
  - [English](docs/developer/en/guides/I18N_IMPLEMENTATION.md) / [日本語](docs/developer/ja/guides/I18N_IMPLEMENTATION.md)
- 🌍 **I18N Resources / 国際化リソース**
  - [English](docs/developer/en/guides/I18N_RESOURCES.md) / [日本語](docs/developer/ja/guides/I18N_RESOURCES.md)
- 🌐 **Dynamic Language Menu / 動的言語メニュー**
  - [English](docs/developer/en/guides/DYNAMIC_LANGUAGE_MENU.md) / [日本語](docs/developer/ja/guides/DYNAMIC_LANGUAGE_MENU.md)
- 🔤 **Font Size Implementation / フォントサイズ実装**
  - [English](docs/developer/en/guides/font-size-implementation.md) / [日本語](docs/developer/ja/guides/font-size-implementation.md)
- ♿ **Accessibility Indicators / アクセシビリティインジケーター**
  - [English](docs/etc/ACCESSIBILITY_INDICATORS.md) / [日本語](docs/etc/ACCESSIBILITY_INDICATORS.md)
- 🚫 **IS_DISABLED Implementation / IS_DISABLED実装**
  - [English](docs/developer/en/guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md) / [日本語](docs/developer/ja/guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md)

#### Database & Security / データベース・セキュリティ
- 🗄️ **Database Configuration / データベース設定**
  - [English](docs/developer/en/guides/DATABASE_CONFIGURATION.md) / [日本語](docs/developer/ja/guides/DATABASE_CONFIGURATION.md)
- 🔄 **Database Migration / データベースマイグレーション**
  - [English](docs/developer/en/guides/DATABASE_MIGRATION.md) / [日本語](docs/developer/ja/guides/DATABASE_MIGRATION.md)
- 🔐 **Encryption Management / 暗号化管理**
  - [English](docs/developer/en/guides/ENCRYPTION_MANAGEMENT.md) / [日本語](docs/developer/ja/guides/ENCRYPTION_MANAGEMENT.md)
- 👤 **User Management / ユーザー管理**
  - [English](docs/developer/en/guides/USER_MANAGEMENT.md) / [日本語](docs/developer/ja/guides/USER_MANAGEMENT.md)
- ⚙️ **Settings Management / 設定管理**
  - [English](docs/developer/en/guides/SETTINGS_MANAGEMENT.md) / [日本語](docs/developer/ja/guides/SETTINGS_MANAGEMENT.md)

#### Design Documents / 設計ドキュメント
- 💰 **Transaction Design V2 / 入出金設計 V2**
  - [English](docs/design/architecture/TRANSACTION_DESIGN_V2.md) / [日本語](docs/design/architecture/TRANSACTION_DESIGN_V2_ja.md)

### Project Information / プロジェクト情報
- 👥 **Project Participants / プロジェクト参加者**
  - [English](docs/etc/PROJECT_PARTICIPANTS.md) / [日本語](docs/etc/PROJECT_PARTICIPANTS.md)

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
- [x] Transaction management / 入出金データ管理
- [x] Monthly/annual reports / 月次・年次集計
- [ ] Data export (CSV) / データエクスポート（CSV）
- [ ] Backup & restore / バックアップ・リストア

---

<div align="center">

**Made with ❤️ and Rust**

[Report Bug / バグ報告](https://github.com/BonoJovi/KakeiBonByRust/issues) · [Request Feature / 機能リクエスト](https://github.com/BonoJovi/KakeiBonByRust/issues)

</div>
