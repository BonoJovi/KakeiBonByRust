# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased] - 2025-11-10

### Added

**feat(i18n): Add Japanese resources to CATEGORY2_I18N and CATEGORY3_I18N**

#### Database Initialization:
- Added Japanese language resources to CATEGORY2_I18N table (20 entries per user)
- Added Japanese language resources to CATEGORY3_I18N table (126 entries per user)
- Updated `res/sql/default_categories_seed.sql` to include both Japanese and English resources
- Total I18N entries: CATEGORY2_I18N 40 (ja+en), CATEGORY3_I18N 252 (ja+en)

#### Consistency Improvement:
- All category I18N tables (CATEGORY1_I18N, CATEGORY2_I18N, CATEGORY3_I18N) now have consistent bilingual support
- Japanese resources are automatically populated during database initialization
- Category management UI now displays Japanese names correctly

### Fixed

**fix(account): Add sqlx mapping attributes to Account and AccountTemplate structs**

#### Bug Fix:
- Added `#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]` attribute to `AccountTemplate` struct in `src/services/account.rs`
- Added `#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]` attribute to `Account` struct in `src/services/account.rs`
- Fixed column name mapping issue between SQLite uppercase column names and Rust lowercase field names
- Resolved "no column found for name: template_id" error during user registration

---

### Original Japanese Summary:

```
feat(i18n): CATEGORY2_I18NとCATEGORY3_I18Nに日本語リソースを追加

データベース初期化:
- CATEGORY2_I18Nテーブルに日本語リソースを追加（ユーザーごとに20件）
- CATEGORY3_I18Nテーブルに日本語リソースを追加（ユーザーごとに126件）
- res/sql/default_categories_seed.sqlを更新し、日本語と英語の両方のリソースを含めた
- I18Nエントリー総数: CATEGORY2_I18N 40件(ja+en)、CATEGORY3_I18N 252件(ja+en)

一貫性の向上:
- すべてのカテゴリI18Nテーブル（CATEGORY1_I18N、CATEGORY2_I18N、CATEGORY3_I18N）で一貫したバイリンガルサポートを実現
- データベース初期化時に日本語リソースが自動投入されるようになった
- 費目管理UIで日本語名が正しく表示されるようになった

fix(account): AccountとAccountTemplate構造体にsqlxマッピング属性を追加

バグ修正:
- src/services/account.rsのAccountTemplate構造体に#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]属性を追加
- src/services/account.rsのAccount構造体に#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]属性を追加
- SQLiteの大文字カラム名とRustの小文字フィールド名のマッピング問題を修正
- ユーザー登録時の「no column found for name: template_id」エラーを解決
```

---

## [Initial Release] - 2025-10-25

### Added - Commit 1 (8e20ede3b7dfd7cdd3c5bec0590bbca15e252f40)

**docs: Organize documents by language & rename consts.def to consts.rs**

#### Document Organization:
- Separated mixed Japanese-English documents into language-specific directories
- Placed Japanese version in docs/ja/
- Placed English version in docs/en/
- Structure prepared for future multilingual support

#### Document Improvements:
- Clearly separated test section into "Implemented" and "Future Implementation"
- Corrected description of non-selected state (gray border #666)
- Standardized terminology to use "people with low vision"
- Modified to softer expressions

#### File Rename:
- res/consts.def → res/consts.rs (compliant with Rust naming conventions)

#### Document Policy:
In the future, documents for new languages will be placed in the corresponding language directory
- Example: Chinese → docs/zh/

---

### Original Japanese Commit Message:

```
docs: ドキュメントを言語別に整理 & consts.defをconsts.rsにリネーム

ドキュメントの整理:
- 日英混在のドキュメントを言語別ディレクトリに分離
- docs/ja/ に日本語版を配置
- docs/en/ に英語版を配置
- 将来的な多言語対応に備えた構成

ドキュメントの改善:
- テストセクションを「実装済み」と「今後の実装予定」に明確に分離
- 非選択状態の説明を正確に修正（グレーの枠線 #666）
- 「視力の弱い方（ロービジョンの方）」の表記に統一
- より柔らかい表現に修正

ファイルリネーム:
- res/consts.def → res/consts.rs (Rustの命名規則に準拠)

ドキュメントポリシー:
今後、新しい言語のドキュメントは対応する言語ディレクトリに配置
例: 中国語 → docs/zh/
```
