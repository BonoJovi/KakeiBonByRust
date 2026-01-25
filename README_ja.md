# [Book] KakeiBon（家計簿）

<div align="center">

> **見やすさと使いやすさを追求した、モダンな家計簿アプリケーション**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2.9.3-blue.svg)](https://tauri.app/)
[![Tests](https://img.shields.io/badge/tests-800%20passing-brightgreen.svg)](#テスト結果)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[[G][B] English Version](README_en.md) | [[Globe] Bilingual README](README.md)

</div>

---

## [Love] 開発者からのメッセージ

<div style="border: 3px solid #4a90e2; padding: 20px; margin: 20px 0; background-color: #f8f9fa; font-size: 1.1em;">

### 愛すべきKakeiBonユーザの皆さんへ

いつもKakeiBonに気を留めていただき、誠にありがとうございます。
プロジェクト発案者のBonoJovi(Yoshihiro NAKAHARA)です。

**Ver.1.1.1を正式リリースいたしました！**

Ver.1.1.0ではダッシュボード機能（Chart.jsによるグラフ表示）を実装し、Ver.1.1.1ではセキュリティ・安定性の向上を行いました。依存関係の更新、未使用コードの整理、CI/CDパイプラインの改善により、より堅牢なリリースプロセスを実現しています。

入出金データの入力・集計機能が完成し、実用的な家計簿アプリケーションとしてご利用いただける状態となりました。
安定版リリースをご利用になりたい方は、[mainブランチ](https://github.com/BonoJovi/KakeiBonByRust/tree/main)をご参照ください。

現在ご覧いただいているdevブランチは開発版となり、次期バージョンの機能を開発中です。
最新の機能をいち早く試してみたい方は、こちらのdevブランチをお使いください。

今後はCSV入出力機能、財務諸表機能、アクセシビリティ対応（ARIA）などの実装を進めていく予定です。
GitHubのissueやeメールでのメッセージも受け付けていますので、応援メッセージや将来的に実装してほしい機能など、ちょっとしたことでも良いのでご連絡いただければ幸いです。

それでは、引き続きKakeiBonをご愛顧頂ますよう、お願い申し上げます。

**2026-01-26 (JST) Written by Yoshihiro NAKAHARA**

</div>

---

## [Bookmark] 目次

- [[WIP] 開発状況](#-開発状況)
- [[Chart] リポジトリ統計](#-リポジトリ統計)
- [[Books] 前身プロジェクト](#-前身プロジェクト)
- [✨ 主な特徴](#-主な特徴)
- [[Rocket] 実装済み機能](#-実装済み機能)
- [[PC] 技術スタック](#-技術スタック)
- [[Package] インストール](#-インストール)
- [[Test] テスト結果](#-テスト結果)
- [[Books] ドキュメント](#-ドキュメント)
- [[Handshake] コントリビューション](#-コントリビューション)
- [[Doc] ライセンス](#-ライセンス)
- [[Star] 開発ロードマップ](#-開発ロードマップ)

---

## [WIP] 開発状況

**[Fire] 鋭意開発中**

開発は順調に進んでおり、できるだけ日々更新するようにしています！

**プロジェクト開始**: 2025-10-22 (JST)  
**最終更新**: 2026-01-26 (JST)

> **[Robot] AI支援開発**  
> 本プロジェクトのソースコードおよびドキュメントは、生成AI（GitHub Copilot、Claude）の支援により**100%生成**され、開発者による監修とレビューを経ています。これは、AI支援開発の可能性を示す事例です。
> 
> [Chart] **[AI開発の生産性と品質分析を見る →](docs/etc/AI_DEVELOPMENT_METRICS.md)**

<!-- STATS_START -->
## [Chart] リポジトリ統計

<div align="center">

### [TrendUp] 日次トラフィック

![Daily Traffic Stats](docs/stats_graph_daily.png)

### [Chart] 累積トラフィック

![Cumulative Traffic Stats](docs/stats_graph_cumulative.png)

| 指標 | 件数 |
|------|------|
| [Eye]️ **総閲覧数** | **660** |
| [Package] **総クローン数** | **214** |

*最終更新: 2025-11-30 12:09 UTC*

</div>
<!-- STATS_END -->

---

## [Books] 前身プロジェクト

**安定版（Lazarus/Free Pascal版）をお探しですか？**

[Point] **[KakeiBon (オリジナル版)](https://github.com/BonoJovi/KakeiBon)** - 今すぐ使える完成版！

元祖KakeiBonは、**今すぐ使える完成版の家計簿アプリ**です！

**主な違い:**
- ✅ **安定版・本番利用可能**
- [Package] **ビルド済みバイナリあり**（[Releases](https://github.com/BonoJovi/KakeiBon/releases/)）
- [J][P] **日本語インターフェース専用**
- [Desktop]️ **Linux & Windows 対応**
- [Text] **大きな文字とアクセシビリティ**

**なぜRust版？**

このRust版では以下を実現：
- ⚡ **より高速**
- [Lock] **強化されたセキュリティ** (Argon2 + AES-256-GCM)
- [Globe] **完全多言語対応**
- [Art] **モダンなアーキテクチャ**
- [Crystal] **将来の拡張性**

[Idea] **両方試して、お好みの方をお使いください！**

---

## ✨ 主な特徴

### [Art] NOTバイブコーディング
雰囲気ではなく、**きちんとした計画とドキュメント作成**を先に行う開発スタイル

### [User] 明確なユーザーファーストポリシーによる設計
すべての機能は**明確なユーザーニーズと使いやすさ**を念頭に置いて設計されています

### [Text] 大きな文字で見やすい
視認性を重視した設計で、長時間の使用でも目が疲れにくい

### [Build]️ エンタープライズグレードのアーキテクチャ
**セッションベース認証**を全52個のAPI関数で実装

- [Key] **セキュアなセッション管理**
- [Users] **ユーザーデータの完全分離**
- ✅ **ハードコードされたユーザーID排除**
- [Test] **527テスト（100%合格）**

### [Target] 直感的な操作性
誰でもすぐに使いこなせる、シンプルで分かりやすいUI

### ♿ アクセシビリティ対応
- **フォントサイズ調整**: 小/中/大/カスタム（10-30px）
- **キーボードナビゲーション**: 完全対応
- **フォーカスインジケーター**: 明確な視覚フィードバック

### [Globe] 多言語対応
日本語・英語の切り替えが可能

### [Lock] 強固なセキュリティ
- Argon2idパスワードハッシュ化
- AES-256-GCMデータ暗号化
- ロールベースのアクセス制御

---

## [Rocket] 実装済み機能

| 機能 | 説明 | ステータス |
|------|------|------------|
| [Key] **セッション管理** | メモリ内セッション状態管理 | ✅ 完成 |
| [Money] **費目管理** | 大分類・中分類・小分類の階層的管理 | ✅ 完成 |
| [Users] **ユーザー管理** | マルチユーザー対応（管理者/一般） | ✅ 完成 |
| [Bank] **口座管理** | 口座マスタ管理 | ✅ 完成 |
| [Shop] **店舗管理** | 店舗マスタ管理 | ✅ 完成 |
| [Factory] **メーカー管理** | IS_DISABLED機能付きメーカーマスタ管理 | ✅ 完成 |
| [Package] **商品管理** | メーカー連携付き商品マスタ管理 | ✅ 完成 |
| [World] **多言語対応** | 日本語・英語の動的切り替え - 992リソース | ✅ 完成 |
| [Fix] **カスタマイズ** | フォントサイズ、言語設定 | ✅ 完成 |
| [Note] **入出金管理** | ヘッダレベルCRUD、フィルター、ページネーション | ✅ 完成 |
| [Receipt] **入出金明細** | スマート税計算付きCRUD操作、端数処理自動検出 | ✅ 完成 |
| [Chart] **集計・レポート** | 月次・年次レポート、グラフ | [WIP] 開発中 |

---

## [PC] 技術スタック

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

## [Package] インストール

### 前提条件
- Rust 1.70+ ([rustup](https://rustup.rs/)でインストール)
- Node.js 18+ (Tauri CLI用)
- SQLite3ネイティブライブラリ
  - **Windows**: [sqlite.org](https://www.sqlite.org/download.html)からダウンロード・インストール
    - `sqlite-dll-win-x64-*.zip` (64ビットDLL) をダウンロード
    - `sqlite3.dll` を `C:\Windows\System32\` に配置（またはPATHに追加）
  - **macOS**: プリインストール済み（またはHomebrewでインストール: `brew install sqlite3`）
  - **Linux**: パッケージマネージャーでインストール
    - Ubuntu/Debian: `sudo apt-get install libsqlite3-dev`
    - Fedora/RHEL: `sudo dnf install sqlite-devel`
    - Arch: `sudo pacman -S sqlite`

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

## [Test] テスト結果

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

詳細は [テスト概要](docs/testing/ja/TEST_OVERVIEW.md) を参照

---

## [Books] ドキュメント

### [Book] ドキュメント索引
- [Files]️ **[ドキュメント全体索引](docs/INDEX_ja.md)** - すべてのドキュメントへのクイックアクセス

### [Target] はじめに

#### インストール・セットアップ
- [Package] **[インストールガイド](docs/user/ja/SETUP_GUIDE.md)** - アプリのインストール方法

#### ユーザーマニュアル
- [Book] **[ユーザーマニュアル](docs/user/ja/USER_MANUAL.md)** - 機能の使い方
- ❓ **[FAQ（よくある質問）](docs/user/ja/FAQ.md)** - よくある質問と回答
- [Fix] **[トラブルシューティング](docs/user/ja/TROUBLESHOOTING.md)** - 問題解決ガイド

---

### [Man]‍[PC] 開発者向け

#### 設計ドキュメント
- [Build]️ **[アーキテクチャ](docs/developer/ja/design/ARCHITECTURE.md)** - システム全体の構造
- [Lock] **[セキュリティ設計](docs/developer/ja/design/SECURITY_DESIGN.md)** - セキュリティの実装
- [Cabinet]️ **[データベース設計](docs/developer/ja/design/DATABASE_DESIGN.md)** - DBスキーマとER図
- [Art] **[UI設計](docs/developer/ja/design/UI_DESIGN.md)** - ユーザーインターフェース設計

#### 開発ガイド
- [Rocket] **[開発環境セットアップ](docs/developer/ja/guides/DEVELOPMENT_SETUP.md)** - 開発環境の構築
- [Note] **[コーディング規約](docs/developer/ja/guides/CODING_STANDARDS.md)** - コードスタイルガイド
- [Test] **テストドキュメント**
  - [Book] **[テスト概要](docs/testing/ja/TEST_OVERVIEW.md)** - テスト戦略と実行方法
  - [BlueBook] **[バックエンドテストインデックス](docs/testing/ja/BACKEND_TEST_INDEX.md)** - Rustテスト完全一覧（201件）
  - [GreenBook] **[フロントエンドテストインデックス](docs/testing/ja/FRONTEND_TEST_INDEX.md)** - JavaScriptテスト完全一覧（262件以上）

#### API ドキュメント
- [Link] **[共通API](docs/developer/ja/api/API_COMMON.md)** - 認証・セッション・i18n
- [Users] **[ユーザー管理API](docs/developer/ja/api/API_USER.md)** - ユーザーCRUD操作
- [Folder] **[費目管理API](docs/developer/ja/api/API_CATEGORY.md)** - 費目の階層管理
- [Money] **[入出金管理API](docs/developer/ja/api/API_TRANSACTION.md)** - 取引データ管理
- [Bank] **[口座管理API](docs/developer/ja/api/API_ACCOUNT.md)** - 口座マスタ管理
- [Office] **[マスタデータAPI](docs/developer/ja/api/API_MASTER_DATA.md)** - 店舗・メーカー・商品
- [Chart] **[集計API](docs/developer/ja/api/API_AGGREGATION.md)** - レポートと統計
- ⚙️ **[設定API](docs/developer/ja/api/API_SETTINGS.md)** - ユーザー設定管理

---

### [List] プロジェクト情報
- [Users] **[プロジェクト参加者](docs/etc/PROJECT_PARTICIPANTS.md)** - コントリビューター一覧
- [Chart] **[AI開発メトリクス](docs/etc/AI_DEVELOPMENT_METRICS.md)** - AI支援開発の分析

---

## [Handshake] コントリビューション

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

## [Doc] ライセンス

このプロジェクトは [LICENSE](LICENSE) の下でライセンスされています。

---

## [Star] 開発ロードマップ

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
