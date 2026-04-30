# Changelog

All notable changes to this project will be documented in this file.

## [v2.0.0] - 2026-05-01

Major release with a complete overhaul of the tax calculation logic. The previous approach (compute-then-round per detail line, then aggregate) accumulated rounding errors, occasionally causing aggregated totals to disagree with the receipt's actual amount (the trigger was a 107-yen discrepancy in April's food aggregation). The formula has been unified to: *within a single transaction* — `SUM(net) → tax calc → round per header's TAX_ROUNDING_TYPE → sum across rates*; *across transactions* — `SUM` the already-rounded integer values without further rounding.

### Breaking Changes

- **`AMOUNT` semantics**: Transaction detail `AMOUNT` is now treated as the net (tax-exclusive) value. `AMOUNT_INCLUDING_TAX` is retained both in the UI and DB as a derived tax-inclusive presentation value
- **Aggregation query rewrite** (commit `dc720521`): Detail-level subquery that decides tax-inclusive/exclusive and aggregates header-rounded values. Eliminates the double-counting bug and the accumulated rounding error

### New Features

- **Auto-recalculation of header `TOTAL_AMOUNT`** (commits `984987d5` `d600a2a4`): After editing details or the header, a prompt asks whether to overwrite with the auto-calculated value (OK) or keep the manual value (Cancel)
- **Real-time preview on tax setting change** (commit `31cadb09`): Switching tax method (exclusive/inclusive/rounding) in the header edit modal updates the total field instantly (frontend JS pure function with Jest tests)
- **Bulk recalculate + one-click rollback** (commits `83d90e78` `f7d61a13`): New "Data Maintenance" section on the dashboard. Backs up the DB file before running, so any unwanted outcome can be reverted with one click
- **Pattern-match recalc** (commit `b6cbab91`): Treats user-entered `TOTAL_AMOUNT` as authoritative, brute-forces 4 patterns (exclusive floor / half-up / ceil + inclusive) and corrects `TAX_ROUNDING_TYPE` / `TAX_INCLUDED_TYPE` when one matches; only overwrites `TOTAL_AMOUNT` when no pattern matches. Shows a detailed change log
- **Per-account balance snapshot** (commit `f888c8bc`): Per-account balance panel above the dashboard charts, useful for reconciling against source data

### Fixes

- **Ceil rounding lost 1 yen** (commit `7bd04550`): The `CAST(-x/100 AS INTEGER)` idiom in SQLite was actually floor (trunc-toward-zero), not ceil. Rewritten as `(x + 99) / 100`. Rust and JS implementations were correct from the start; only the SQL path was affected
- **Pin SQLite ATTACH / UPDATE / DETACH to a single connection** (commit `7d844f7c`): Fixes a bug where the bulk recalc held the attached schema on a different connection, causing DETACH to report NOT FOUND
- **Dashboard column scrollable** (commit `6e27e3f2`): The dashboard column now scrolls when content exceeds the viewport

### Developer Experience / Maintenance

- **Bump `@tauri-apps/cli` to ^2.11.0** (commit `d354b689`)

### Versioning

The change to `AMOUNT` semantics is an API-level breaking change, hence the MAJOR bump to v2.0.0. There is no database schema migration, so existing users can update without manual data migration. Aggregated values may differ from before; this is the rounding-error fix at work, and the new values match the receipt amounts.

---

## [v1.2.2] - 2026-04-30

### Fixes

- **Transaction List Filters**: Fix CATEGORY2/CATEGORY3 filters being silently ignored, causing every row of the parent CATEGORY1 to be returned
  - The `category2_code` / `category3_code` arguments to `get_transactions` were discarded via `let _ = ...`
  - Wired into the WHERE clause via EXISTS subqueries so header row counts stay stable (no DISTINCT needed)
  - Empty strings are now treated as "no filter" to match the frontend's unselected state
  - Added a regression test covering CATEGORY2-only, CATEGORY3 narrowing, and the empty-string passthrough

### CI / Security

- **GitHub Actions Workflow**: Declare minimal permissions (`contents: read`) on `feature-branch-test.yml`
- **Test Coverage Artifacts**: Remove `res/tests/coverage/` files that had been accidentally committed (already listed in `.gitignore`)
- **Code Scanning Baseline**: Triaged the 26 open alerts (dismissed false positives, fixed legitimate finding)
- **Dependabot Alert Triage**: Dismissed GHSA-cq8v-f236-94qc (rand transitive dep via tauri build deps) as `tolerable_risk`

### Documentation

- Compacted session-loaded context files (CLAUDE.md / `.github/copilot-instructions.md`)
- Added tax calculation refactor plan as design doc for the upcoming v2.0.0
- Added project-level slash commands (`/i18n-add`, `/release`)
- Fixed script name typo in release docs (`pre-release-check.sh` → `check-release.sh`)

---

## [v1.2.1] - 2026-04-24

### Fixes

- **Account Management**: Hide NONE account (internal "unspecified" account) from the list
  - Prevents users from accidentally editing the NONE account
- **Detail Management**: Improve focus behavior on add detail modal
  - Auto-focus on item name field when modal opens
  - Prevent scroll position shift (`preventScroll` support)

---

## [v1.2.0] - 2026-04-19

### New Features

- **Scheduled Transactions (IS_SCHEDULED)**: Add "scheduled" flag to transactions
  - Register scheduled transactions and confirm (convert scheduled → actual)
  - Visual distinction in list view (background color, badge)
  - Excluded by default, toggle checkbox to include
- **Aggregation Scheduled Filter**: All 5 aggregation types (monthly/daily/weekly/period/yearly)
- **Remove Future Date Restriction**: Allow aggregation of future-dated scheduled transactions
- **Weekly Aggregation Week Range Display**: Real-time target week range shown on date selection
- **Aggregation Amount Coloring**: Income (green) and expense (red) visual distinction

### Internationalization

- Aggregation error messages i18n (from English-only to bilingual Japanese/English)
- Added 34 bilingual i18n resource entries for scheduled transaction features

### Tests

- Backend: 248 tests
- Frontend: 216 tests

---

## [v1.1.2] - 2026-04-16

### Security

- **Upgrade rand crate**: v0.8 → v0.9 (GHSA-cq8v-f236-94qc)
  - Addresses Dependabot Alert #13
  - API migration: `thread_rng().gen()` → `rand::random()`

---

## [v1.1.1] - 2026-01-26

### Security & Stability

- **Dependency Updates**: Updated 76 packages to latest versions
- **Code Cleanup**: Removed dead code and resolved warnings
- **CI/CD Improvements**: Added release build integrity check
  - Added `test-release-build` job to workflow
  - Added `scripts/check-release.sh` script

### Tests

- Backend: 201 tests
- Frontend: 599 tests
- Total: 800 tests

---

## [v1.1.0] - 2026-01-07

### Added

- **Dashboard Feature**: Graph visualization with Chart.js
  - Monthly income/expense trend chart
  - Category-wise pie chart

---

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
