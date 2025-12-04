# 📖 KakeiBon（家計簿）

<div align="center">

> **見やすさと使いやすさを追求した、モダンな家計簿アプリケーション**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2.9.3-blue.svg)](https://tauri.app/)
[![Tests](https://img.shields.io/badge/tests-527%20passing-brightgreen.svg)](#テスト結果)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[🇬🇧 English Version](README_en.md) | [🌐 Bilingual README](README.md)

</div>

---

## 💌 開発者からのメッセージ

<div style="border: 3px solid #4a90e2; padding: 20px; margin: 20px 0; background-color: #f8f9fa; font-size: 1.1em;">

### 愛すべきKakeiBonユーザの皆さんへ

いつもKakeiBonに気を留めていただき、誠にありがとうございます。
プロジェクト発案者のBonoJovi(Yoshihiro NAKAHARA)です。

**Ver.1.0.1を正式リリースいたしました！**

入出金データの入力機能が完成し、基本的な家計簿アプリケーションとしてご利用いただける状態となりました。
安定版リリースをご利用になりたい方は、[mainブランチ](https://github.com/BonoJovi/KakeiBonByRust/tree/main)をご参照ください。

現在ご覧いただいているdevブランチは開発版となり、次期バージョンの機能を開発中です。
最新の機能をいち早く試してみたい方は、こちらのdevブランチをお使いください。

今後は集計・レポート機能の実装を進めていく予定です。細々した機能も追々実装していきますので、機能拡張にご期待いただければと思います。
GitHubのissueやeメールでのメッセージも受け付けていますので、応援メッセージや将来的に実装してほしい機能など、ちょっとしたことでも良いのでご連絡いただければ幸いです。

それでは、引き続きKakeiBonをご愛顧頂ますよう、お願い申し上げます。

**2025-11-30 (JST) Written by Yoshihiro NAKAHARA**

</div>

---

## 📑 目次

- [🚧 開発状況](#-開発状況)
- [📊 リポジトリ統計](#-リポジトリ統計)
- [📚 前身プロジェクト](#-前身プロジェクト)
- [✨ 主な特徴](#-主な特徴)
- [🚀 実装済み機能](#-実装済み機能)
- [💻 技術スタック](#-技術スタック)
- [📦 インストール](#-インストール)
- [🧪 テスト結果](#-テスト結果)
- [📚 ドキュメント](#-ドキュメント)
- [🤝 コントリビューション](#-コントリビューション)
- [📄 ライセンス](#-ライセンス)
- [🌟 開発ロードマップ](#-開発ロードマップ)

---

## 🚧 開発状況

**🔥 鋭意開発中**

開発は順調に進んでおり、できるだけ日々更新するようにしています！

**プロジェクト開始**: 2025-10-22 (JST)  
**最終更新**: 2025-11-30 (JST)

> **🤖 AI支援開発**  
> 本プロジェクトのソースコードおよびドキュメントは、生成AI（GitHub Copilot、Claude）の支援により**100%生成**され、開発者による監修とレビューを経ています。これは、AI支援開発の可能性を示す事例です。
> 
> 📊 **[AI開発の生産性と品質分析を見る →](docs/etc/AI_DEVELOPMENT_METRICS.md)**

<!-- STATS_START -->
## 📊 リポジトリ統計

<div align="center">

### 📈 日次トラフィック

![Daily Traffic Stats](docs/stats_graph_daily.png)

### 📊 累積トラフィック

![Cumulative Traffic Stats](docs/stats_graph_cumulative.png)

| 指標 | 件数 |
|------|------|
| 👁️ **総閲覧数** | **660** |
| 📦 **総クローン数** | **214** |

*最終更新: 2025-11-30 12:09 UTC*

</div>
<!-- STATS_END -->

---

## 📚 前身プロジェクト

**安定版（Lazarus/Free Pascal版）をお探しですか？**

👉 **[KakeiBon (オリジナル版)](https://github.com/BonoJovi/KakeiBon)** - 今すぐ使える完成版！

元祖KakeiBonは、**今すぐ使える完成版の家計簿アプリ**です！

**主な違い:**
- ✅ **安定版・本番利用可能**
- 📦 **ビルド済みバイナリあり**（[Releases](https://github.com/BonoJovi/KakeiBon/releases/)）
- 🇯🇵 **日本語インターフェース専用**
- 🖥️ **Linux & Windows 対応**
- 🔤 **大きな文字とアクセシビリティ**

**なぜRust版？**

このRust版では以下を実現：
- ⚡ **より高速**
- 🔒 **強化されたセキュリティ** (Argon2 + AES-256-GCM)
- 🌐 **完全多言語対応**
- 🎨 **モダンなアーキテクチャ**
- 🔮 **将来の拡張性**

💡 **両方試して、お好みの方をお使いください！**

---

## ✨ 主な特徴

### 🎨 NOTバイブコーディング
雰囲気ではなく、**きちんとした計画とドキュメント作成**を先に行う開発スタイル

### 👤 明確なユーザーファーストポリシーによる設計
すべての機能は**明確なユーザーニーズと使いやすさ**を念頭に置いて設計されています

### 🔤 大きな文字で見やすい
視認性を重視した設計で、長時間の使用でも目が疲れにくい

### 🏗️ エンタープライズグレードのアーキテクチャ
**セッションベース認証**を全52個のAPI関数で実装

- 🔐 **セキュアなセッション管理**
- 👥 **ユーザーデータの完全分離**
- ✅ **ハードコードされたユーザーID排除**
- 🧪 **527テスト（100%合格）**

### 🎯 直感的な操作性
誰でもすぐに使いこなせる、シンプルで分かりやすいUI

### ♿ アクセシビリティ対応
- **フォントサイズ調整**: 小/中/大/カスタム（10-30px）
- **キーボードナビゲーション**: 完全対応
- **フォーカスインジケーター**: 明確な視覚フィードバック

### 🌐 多言語対応
日本語・英語の切り替えが可能

### 🔒 強固なセキュリティ
- Argon2idパスワードハッシュ化
- AES-256-GCMデータ暗号化
- ロールベースのアクセス制御

---

## 🚀 実装済み機能

| 機能 | 説明 | ステータス |
|------|------|------------|
| 🔐 **セッション管理** | メモリ内セッション状態管理 | ✅ 完成 |
| 💰 **費目管理** | 大分類・中分類・小分類の階層的管理 | ✅ 完成 |
| 👥 **ユーザー管理** | マルチユーザー対応（管理者/一般） | ✅ 完成 |
| 🏦 **口座管理** | 口座マスタ管理 | ✅ 完成 |
| 🏪 **店舗管理** | 店舗マスタ管理 | ✅ 完成 |
| 🏭 **メーカー管理** | IS_DISABLED機能付きメーカーマスタ管理 | ✅ 完成 |
| 📦 **商品管理** | メーカー連携付き商品マスタ管理 | ✅ 完成 |
| 🌍 **多言語対応** | 日本語・英語の動的切り替え - 992リソース | ✅ 完成 |
| 🔧 **カスタマイズ** | フォントサイズ、言語設定 | ✅ 完成 |
| 📝 **入出金管理** | ヘッダレベルCRUD、フィルター、ページネーション | ✅ 完成 |
| 🧾 **入出金明細** | スマート税計算付きCRUD操作、端数処理自動検出 | ✅ 完成 |
| 📊 **集計・レポート** | 月次・年次レポート、グラフ | 🚧 開発中 |

---

## 💻 技術スタック

| カテゴリ | 技術 | 詳細 |
|----------|------|------|
| **フロントエンド** | Vanilla JavaScript + HTML5 + CSS3 | ES6 Modules |
| **バックエンド** | Rust + Tauri | v2.8.5 |
| **データベース** | SQLite | WAL mode |
| **セキュリティ** | Argon2id + AES-256-GCM | Password hashing + Data encryption |
| **テスト** | Jest + Cargo Test | 527 tests passing (Rust: 201, JS: 326) |
| **翻訳** | JSON-based | 992 resources (496 unique keys, 2 languages) |
| **コード行数** | 合計 | ~35,478 lines (Rust: 13,870, JS: 8,810, HTML: 3,355, CSS: 6,109, SQL: 3,334) |

---

## 📦 インストール

### 前提条件
- Rust 1.70+ ([rustup](https://rustup.rs/)でインストール)
- Node.js 18+ (Tauri CLI用)

### ビルド・実行

```bash
# リポジトリをクローン
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust

# 開発モードで起動
cargo tauri dev

# プロダクションビルド
cargo tauri build
```

---

## 🧪 テスト結果

```
バックエンド (Rust):      201 passing ✅
フロントエンド (JavaScript): 326 passing ✅
総テスト数:               527 passing ✅
成功率:                  100%
```

**最近の改善**:
- ✅ **セッション管理統合** (2025-11-30)
  - 全52個のAPI関数がセッションベース認証を使用
  - 適切なユーザー分離による強化されたセキュリティ
  - コードベース全体からハードコードされたユーザーIDを削除

- ✅ **テスト品質向上** (2025-11-30)
  - 委譲されたテストに明示的なアサーションを追加
  - テストの可読性と保守性を向上
  - エンタープライズグレードのテスト構造を実現

**テスト件数計測方法** (2025-11-30更新):
- **以前のカウント (613)**: ネストされた`describe`ブロックとテスト構造を含む
- **現在のカウント (527)**: 実際に実行可能なテストケースのみをカウント
- **変更理由**: 精度の向上と業界標準の方法論
- **注意**: テストは削除されておらず、計測方法の精密化のみです

詳細は [TEST_SUMMARY.md](docs/developer/ja/testing/TEST_SUMMARY.md) を参照

---

## 📚 ドキュメント

### 📖 ドキュメント索引
- 🗂️ **[ドキュメント全体索引](docs/INDEX_ja.md)** - すべてのドキュメントへのクイックアクセス

### 🎯 はじめに

#### インストール・セットアップ
- 📦 **[インストールガイド](docs/user/ja/SETUP_GUIDE.md)** - アプリのインストール方法

#### ユーザーマニュアル
- 📖 **[ユーザーマニュアル](docs/user/ja/USER_MANUAL.md)** - 機能の使い方
- ❓ **[FAQ（よくある質問）](docs/user/ja/FAQ.md)** - よくある質問と回答
- 🔧 **[トラブルシューティング](docs/user/ja/TROUBLESHOOTING.md)** - 問題解決ガイド

---

### 👨‍💻 開発者向け

#### 設計ドキュメント
- 🏗️ **[アーキテクチャ](docs/developer/ja/design/ARCHITECTURE.md)** - システム全体の構造
- 🔒 **[セキュリティ設計](docs/developer/ja/design/SECURITY_DESIGN.md)** - セキュリティの実装
- 🗄️ **[データベース設計](docs/developer/ja/design/DATABASE_DESIGN.md)** - DBスキーマとER図
- 🎨 **[UI設計](docs/developer/ja/design/UI_DESIGN.md)** - ユーザーインターフェース設計

#### 開発ガイド
- 🚀 **[開発環境セットアップ](docs/developer/ja/guides/DEVELOPMENT_SETUP.md)** - 開発環境の構築
- 📝 **[コーディング規約](docs/developer/ja/guides/CODING_STANDARDS.md)** - コードスタイルガイド
- 🧪 **[テストガイド](docs/developer/ja/guides/TESTING_GUIDE.md)** - テスト戦略と実行方法

#### API ドキュメント
- 🔗 **[共通API](docs/developer/ja/api/API_COMMON.md)** - 認証・セッション・i18n
- 👥 **[ユーザー管理API](docs/developer/ja/api/API_USER.md)** - ユーザーCRUD操作
- 📁 **[費目管理API](docs/developer/ja/api/API_CATEGORY.md)** - 費目の階層管理
- 💰 **[入出金管理API](docs/developer/ja/api/API_TRANSACTION.md)** - 取引データ管理
- 🏦 **[口座管理API](docs/developer/ja/api/API_ACCOUNT.md)** - 口座マスタ管理
- 🏢 **[マスタデータAPI](docs/developer/ja/api/API_MASTER_DATA.md)** - 店舗・メーカー・商品
- 📊 **[集計API](docs/developer/ja/api/API_AGGREGATION.md)** - レポートと統計
- ⚙️ **[設定API](docs/developer/ja/api/API_SETTINGS.md)** - ユーザー設定管理

---

### 📋 プロジェクト情報
- 👥 **[プロジェクト参加者](docs/etc/PROJECT_PARTICIPANTS.md)** - コントリビューター一覧
- 📊 **[AI開発メトリクス](docs/etc/AI_DEVELOPMENT_METRICS.md)** - AI支援開発の分析

---

## 🤝 コントリビューション

プルリクエストを歓迎します！

1. このリポジトリをフォーク
2. フィーチャーブランチを作成  
   `git checkout -b feature/AmazingFeature`
3. 変更をコミット  
   `git commit -m 'Add some AmazingFeature'`
4. ブランチにプッシュ  
   `git push origin feature/AmazingFeature`
5. プルリクエストを開く

詳細は [CONTRIBUTING.md](CONTRIBUTING.md) を参照

---

## 📄 ライセンス

このプロジェクトは [LICENSE](LICENSE) の下でライセンスされています。

---

## 🌟 開発ロードマップ

- [x] ユーザー管理機能
- [x] 費目管理機能
- [x] 多言語対応
- [x] アクセシビリティ機能
- [x] 入出金データ管理
- [x] 月次・年次集計
- [ ] データエクスポート（CSV）
- [ ] バックアップ・リストア

---

<div align="center">

**Made with ❤️ and Rust**

[バグ報告](https://github.com/BonoJovi/KakeiBonByRust/issues) · [機能リクエスト](https://github.com/BonoJovi/KakeiBonByRust/issues)

</div>
