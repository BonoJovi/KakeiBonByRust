# KakeiBon ドキュメント索引

**最終更新**: 2025-12-05 06:35 JST

このドキュメントは、KakeiBonプロジェクトのすべてのドキュメントへのクイックアクセスを提供します。

---

## [List] 目次

- [クイックスタート](#クイックスタート)
- [ユーザー向けドキュメント](#ユーザー向けドキュメント)
- [開発者向けドキュメント](#開発者向けドキュメント)
- [APIリファレンス](#apiリファレンス)
- [設計ドキュメント](#設計ドキュメント)
- [画面別クイックリファレンス](#画面別クイックリファレンス)
- [キーワード索引](#キーワード索引)

---

## [Rocket] クイックスタート

### ユーザー向け
- [インストールガイド](user/ja/INSTALLATION_GUIDE.md) - アプリケーションのインストール方法
- [クイックスタートガイド](user/ja/QUICK_START_GUIDE.md) - 5分で始める
- [セットアップガイド](user/ja/SETUP_GUIDE.md) - 初期設定の手順

### 開発者向け
- [開発環境セットアップ](developer/ja/setup/DEVELOPMENT_SETUP.md) - 開発環境の構築
- [開発者ガイド](developer/ja/guides/DEVELOPER_GUIDE.md) - 開発の始め方

---

## [Users] ユーザー向けドキュメント

### 基本ガイド
| ドキュメント | 説明 |
|------------|------|
| [ユーザーマニュアル](user/ja/USER_MANUAL.md) | 完全な機能説明・操作方法 |
| [セットアップガイド](user/ja/SETUP_GUIDE.md) | 初期設定の詳細手順 |
| [インストールガイド](user/ja/INSTALLATION_GUIDE.md) | インストール方法 |
| [クイックスタートガイド](user/ja/QUICK_START_GUIDE.md) | 5分で始める簡易ガイド |

### 機能別ガイド
| ドキュメント | 説明 |
|------------|------|
| [集計機能ユーザーガイド](user/ja/AGGREGATION_USER_GUIDE.md) | 月次・日次・週次・年次・期間別集計の使い方 |

### トラブルシューティング
| ドキュメント | 説明 |
|------------|------|
| [トラブルシューティング](user/ja/TROUBLESHOOTING.md) | 問題解決ガイド |
| [FAQ](user/ja/FAQ.md) | よくある質問と回答 |

---

## [PC] 開発者向けドキュメント

### セットアップ・環境構築
| ドキュメント | 説明 |
|------------|------|
| [開発環境セットアップ](developer/ja/setup/DEVELOPMENT_SETUP.md) | Rust、Node.js、Tauriの環境構築 |
| [データベース設定](developer/ja/guides/DATABASE_CONFIGURATION.md) | SQLiteデータベースの設定 |
| [データベースマイグレーション](developer/ja/guides/DATABASE_MIGRATION.md) | スキーマ変更・マイグレーション手順 |

### 開発ガイド
| ドキュメント | 説明 |
|------------|------|
| [開発者ガイド](developer/ja/guides/DEVELOPER_GUIDE.md) | 開発の進め方・ワークフロー |
| [コーディング規約](developer/ja/guides/CODING_STANDARDS.md) | Rust/JavaScript/CSS規約 |
| [テストガイド](developer/ja/guides/testing-guide.md) | テスト戦略・実施方法 |
| [ドキュメントポリシー](developer/en/guides/DOCUMENTATION_POLICY.md) | ドキュメント作成ルール（英語版のみ） |

### 機能実装ガイド
| ドキュメント | 説明 |
|------------|------|
| [ユーザー管理UI](developer/ja/guides/USER_MANAGEMENT_UI.md) | ユーザー管理画面の実装 |
| [費目管理UI](developer/ja/guides/CATEGORY_MANAGEMENT_UI.md) | 費目管理画面の実装 |
| [口座管理UI](developer/ja/guides/ACCOUNT_MANAGEMENT_UI.md) | 口座管理画面の実装 |
| [入出金管理UI](developer/ja/guides/TRANSACTION_MANAGEMENT_UI.md) | 入出金管理画面の実装 |
| [入出金管理UI V2](developer/ja/guides/TRANSACTION_MANAGEMENT_UI_V2.md) | 入出金管理画面の改訂版 |
| [IS_DISABLED実装ガイド](developer/ja/guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md) | 論理削除機能の実装パターン |

### 国際化・UI
| ドキュメント | 説明 |
|------------|------|
| [I18N実装ガイド](developer/ja/guides/I18N_IMPLEMENTATION.md) | 多言語対応の実装方法 |
| [I18Nリソース統計](developer/ja/guides/I18N_RESOURCES.md) | 翻訳リソースの統計情報 |
| [翻訳ガイド](developer/ja/guides/translation-guide.md) | 翻訳追加・更新の手順 |
| [動的言語メニュー](developer/ja/guides/DYNAMIC_LANGUAGE_MENU.md) | 言語切り替えメニューの実装 |
| [フォントサイズ機能](developer/ja/guides/font-size-implementation.md) | フォントサイズ変更機能の実装 |

### セキュリティ・暗号化
| ドキュメント | 説明 |
|------------|------|
| [暗号化管理](developer/ja/guides/ENCRYPTION_MANAGEMENT.md) | AES-256-GCM暗号化の実装 |
| [設定管理](developer/ja/guides/SETTINGS_MANAGEMENT.md) | ユーザー設定の管理 |

### テスト
| ドキュメント | 説明 |
|------------|------|
| [テストガイド](developer/ja/guides/testing-guide.md) | テスト戦略・自動テスト |
| [テストサマリー](developer/ja/testing/TEST_SUMMARY.md) | テスト結果サマリー |

---

## [Books] APIリファレンス

### 画面別APIドキュメント
| ドキュメント | 説明 | 画面 |
|------------|------|------|
| [共通API](developer/ja/api/API_COMMON.md) | セッション、I18N、暗号化、システム、バリデーション | 全画面共通 |
| [認証・セットアップAPI](developer/ja/api/API_AUTH.md) | 管理者セットアップ、ログイン | index.html |
| [ユーザー管理API](developer/ja/api/API_USER.md) | ユーザーCRUD、パスワード変更 | user-management.html |
| [費目管理API](developer/ja/api/API_CATEGORY.md) | 費目3階層のCRUD、並び順変更 | category-management.html |
| [口座管理API](developer/ja/api/API_ACCOUNT.md) | 口座CRUD、テンプレート管理 | account-management.html |
| [入出金管理API](developer/ja/api/API_TRANSACTION.md) | ヘッダーCRUD、フィルター、メモ管理 | transaction-management.html |
| [マスタデータAPI](developer/ja/api/API_MASTER_DATA.md) | 店舗、メーカー、商品のCRUD | shop/manufacturer/product-management.html |
| [集計API](developer/ja/api/API_AGGREGATION.md) | 月次、日次、週次、年次、期間別集計 | aggregation-*.html |
| [設定API](developer/ja/api/API_SETTINGS.md) | フォントサイズ、言語、暗号化設定 | settings.html |

### APIコマンド一覧
**全100コマンド**（API分布）:
- 共通API: 24コマンド
- 費目管理: 16コマンド
- 入出金管理: 14コマンド
- ユーザー管理: 13コマンド
- 設定: 10コマンド
- セッション: 9コマンド
- その他: 14コマンド

---

## [Build]️ 設計ドキュメント

### アーキテクチャ設計
| ドキュメント | 説明 |
|------------|------|
| [アーキテクチャ設計](developer/ja/design/ARCHITECTURE.md) | システム全体のアーキテクチャ |
| [データベース設計](developer/ja/design/DATABASE_DESIGN.md) | ERD、テーブル定義、インデックス |
| [セキュリティ設計](design/SECURITY_DESIGN.md) | 認証、暗号化、パスワードハッシュ |
| [UI設計](design/UI_DESIGN.md) | UI/UX設計方針 |

### 詳細設計
| ドキュメント | 説明 |
|------------|------|
| [入出金設計 V2](design/architecture/TRANSACTION_DESIGN_V2.md) | 入出金機能の詳細設計 |
| [入出金設計 V2（日本語）](design/architecture/TRANSACTION_DESIGN_V2_ja.md) | 入出金機能の詳細設計（日本語版） |
| [セッション管理仕様](design/architecture/session-management-spec.md) | セッション管理の仕様 |
| [税額計算ロジック](design/architecture/tax-calculation-logic.md) | 税込/税抜計算、端数処理 |

### 要件定義・問題解決
| ドキュメント | 説明 |
|------------|------|
| [入出金要件定義](design/requirements/TRANSACTION_REQUIREMENTS.md) | 入出金機能の要件 |
| [設計問題と修正](design/decisions/DESIGN_ISSUES_AND_FIXES.md) | 設計上の問題と解決策 |

---

## [Desktop]️ 画面別クイックリファレンス

### 認証・セットアップ（index.html）
- **API**: [認証・セットアップAPI](developer/ja/api/API_AUTH.md)
- **機能**: 管理者セットアップ、ログイン、セッション管理

### ユーザー管理（user-management.html）
- **API**: [ユーザー管理API](developer/ja/api/API_USER.md)
- **実装ガイド**: [ユーザー管理UI](developer/ja/guides/USER_MANAGEMENT_UI.md)
- **機能**: ユーザー追加・編集・削除、パスワード変更

### 費目管理（category-management.html）
- **API**: [費目管理API](developer/ja/api/API_CATEGORY.md)
- **実装ガイド**: [費目管理UI](developer/ja/guides/CATEGORY_MANAGEMENT_UI.md)
- **機能**: 3階層費目（大/中/小分類）のCRUD、並び順変更

### 口座管理（account-management.html）
- **API**: [口座管理API](developer/ja/api/API_ACCOUNT.md)
- **実装ガイド**: [口座管理UI](developer/ja/guides/ACCOUNT_MANAGEMENT_UI.md)
- **機能**: 口座CRUD、テンプレート選択、初期残高設定

### 入出金管理（transaction-management.html）
- **API**: [入出金管理API](developer/ja/api/API_TRANSACTION.md)
- **実装ガイド**: [入出金管理UI](developer/ja/guides/TRANSACTION_MANAGEMENT_UI.md)
- **機能**: ヘッダーCRUD、フィルター、ページネーション、メモ管理

### 店舗管理（shop-management.html）
- **API**: [マスタデータAPI](developer/ja/api/API_MASTER_DATA.md)（店舗セクション）
- **機能**: 店舗CRUD、IS_DISABLED機能

### メーカー管理（manufacturer-management.html）
- **API**: [マスタデータAPI](developer/ja/api/API_MASTER_DATA.md)（メーカーセクション）
- **機能**: メーカーCRUD、IS_DISABLED機能

### 商品管理（product-management.html）
- **API**: [マスタデータAPI](developer/ja/api/API_MASTER_DATA.md)（商品セクション）
- **機能**: 商品CRUD、メーカー連携、IS_DISABLED機能

### 集計画面（aggregation-*.html）
- **API**: [集計API](developer/ja/api/API_AGGREGATION.md)
- **ユーザーガイド**: [集計機能ユーザーガイド](user/ja/AGGREGATION_USER_GUIDE.md)
- **機能**: 月次、日次、週次、年次、期間別集計

### 設定画面（settings.html）
- **API**: [設定API](developer/ja/api/API_SETTINGS.md)
- **実装ガイド**: [設定管理](developer/ja/guides/SETTINGS_MANAGEMENT.md)
- **機能**: フォントサイズ、言語、暗号化鍵管理

---

## [Search] キーワード索引

### あ行
- **アーキテクチャ**: [アーキテクチャ設計](developer/ja/design/ARCHITECTURE.md)
- **暗号化**: [暗号化管理](developer/ja/guides/ENCRYPTION_MANAGEMENT.md), [セキュリティ設計](design/SECURITY_DESIGN.md)
- **入出金**: [入出金管理API](developer/ja/api/API_TRANSACTION.md), [入出金管理UI](developer/ja/guides/TRANSACTION_MANAGEMENT_UI.md)

### か行
- **開発環境**: [開発環境セットアップ](developer/ja/setup/DEVELOPMENT_SETUP.md)
- **カテゴリ**: [費目管理API](developer/ja/api/API_CATEGORY.md), [費目管理UI](developer/ja/guides/CATEGORY_MANAGEMENT_UI.md)
- **集計**: [集計API](developer/ja/api/API_AGGREGATION.md), [集計機能ユーザーガイド](user/ja/AGGREGATION_USER_GUIDE.md)
- **口座**: [口座管理API](developer/ja/api/API_ACCOUNT.md), [口座管理UI](developer/ja/guides/ACCOUNT_MANAGEMENT_UI.md)

### さ行
- **セキュリティ**: [セキュリティ設計](design/SECURITY_DESIGN.md), [暗号化管理](developer/ja/guides/ENCRYPTION_MANAGEMENT.md)
- **セッション**: [セッション管理仕様](design/architecture/session-management-spec.md), [共通API](developer/ja/api/API_COMMON.md)
- **セットアップ**: [セットアップガイド](user/ja/SETUP_GUIDE.md), [開発環境セットアップ](developer/ja/setup/DEVELOPMENT_SETUP.md)

### た行
- **多言語**: [I18N実装ガイド](developer/ja/guides/I18N_IMPLEMENTATION.md), [翻訳ガイド](developer/ja/guides/translation-guide.md)
- **データベース**: [データベース設計](developer/ja/design/DATABASE_DESIGN.md), [データベース設定](developer/ja/guides/DATABASE_CONFIGURATION.md)
- **店舗**: [マスタデータAPI](developer/ja/api/API_MASTER_DATA.md)（店舗セクション）
- **テスト**: [テストガイド](developer/ja/guides/testing-guide.md), [テストサマリー](developer/ja/testing/TEST_SUMMARY.md)

### は行
- **バリデーション**: [共通API](developer/ja/api/API_COMMON.md)（バリデーションセクション）
- **費目**: [費目管理API](developer/ja/api/API_CATEGORY.md), [費目管理UI](developer/ja/guides/CATEGORY_MANAGEMENT_UI.md)
- **フォントサイズ**: [フォントサイズ機能](developer/ja/guides/font-size-implementation.md), [設定API](developer/ja/api/API_SETTINGS.md)

### ま行
- **マイグレーション**: [データベースマイグレーション](developer/ja/guides/DATABASE_MIGRATION.md)
- **マスタデータ**: [マスタデータAPI](developer/ja/api/API_MASTER_DATA.md)
- **メーカー**: [マスタデータAPI](developer/ja/api/API_MASTER_DATA.md)（メーカーセクション）
- **メモ**: [入出金管理API](developer/ja/api/API_TRANSACTION.md)（メモ管理セクション）

### や行
- **ユーザー管理**: [ユーザー管理API](developer/ja/api/API_USER.md), [ユーザー管理UI](developer/ja/guides/USER_MANAGEMENT_UI.md)

### ら行
- **ログイン**: [認証・セットアップAPI](developer/ja/api/API_AUTH.md)
- **論理削除**: [IS_DISABLED実装ガイド](developer/ja/guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md)

### A-Z
- **API**: [APIリファレンス](#apiリファレンス)セクション参照
- **CRUD**: 各管理画面のAPIリファレンス参照
- **FAQ**: [FAQ](user/ja/FAQ.md)
- **I18N**: [I18N実装ガイド](developer/ja/guides/I18N_IMPLEMENTATION.md)
- **IS_DISABLED**: [IS_DISABLED実装ガイド](developer/ja/guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md)
- **Rust**: [コーディング規約](developer/ja/guides/CODING_STANDARDS.md)
- **SQLite**: [データベース設定](developer/ja/guides/DATABASE_CONFIGURATION.md)
- **Tauri**: [開発環境セットアップ](developer/ja/setup/DEVELOPMENT_SETUP.md)
- **UI**: [UI設計](design/UI_DESIGN.md)

---

## [Book] その他のドキュメント

### プロジェクト情報
| ドキュメント | 説明 |
|------------|------|
| [プロジェクト参加者](etc/PROJECT_PARTICIPANTS.md) | 開発者・貢献者情報 |
| [AI開発メトリクス](etc/AI_DEVELOPMENT_METRICS.md) | AI支援開発の統計 |
| [アクセシビリティ指標](etc/ACCESSIBILITY_INDICATORS.md) | アクセシビリティ対応状況 |

### セキュリティ
| ドキュメント | 説明 |
|------------|------|
| [Dependabotアラート #1](security/alerts/dependabot-alert-1-glib.md) | glib脆弱性アラート |

---

## [Note] 関連リソース

- [ルートREADME](../README.md) - プロジェクトトップページ
- [日本語README](../README_ja.md) - 日本語ナビゲーションハブ
- [英語README](../README_en.md) - 英語ナビゲーションハブ
- [TODO](../TODO.md) - 開発タスクリスト
- [CHANGELOG](../CHANGELOG_ja.md) - 変更履歴
- [CONTRIBUTING](../CONTRIBUTING.md) - 貢献ガイドライン
- [CODE_OF_CONDUCT](../CODE_OF_CONDUCT_ja.md) - 行動規範

---

**Last Updated**: 2025-12-05 06:35 JST
