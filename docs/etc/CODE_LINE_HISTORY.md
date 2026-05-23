# KakeiBon Code & Documentation Line Count History

**Last updated:** 2026-05-23
**Maintained by:** [count_lines.sh](../../count_lines.sh) + git tag history

---

## 概要 / Overview

このドキュメントは KakeiBon by Rust の各リリースタグ時点における **コード行数とドキュメント行数の履歴** を、`count_lines.sh` による再現可能な計測値で記録したものです。

This document records the historical **code and documentation line counts** at each release tag of KakeiBon by Rust, measured reproducibly via `count_lines.sh`.

過去に公開された数値（記事・LinkedIn 投稿・社内ドキュメント等）に **誤った値（特に「v1.0.0 時点で 35,000 行」）** が含まれていた可能性があるため、本ドキュメントを **一次情報** として参照してください。

Some externally published numbers (articles, LinkedIn posts, internal documents) may have contained incorrect values (notably "35,000 lines at v1.0.0"). Please refer to this document as the **canonical source**.

---

## 計測方法 / Methodology

`count_lines.sh` は以下のディレクトリ／拡張子を集計します：

`count_lines.sh` measures the following directories / extensions:

| Category | Pattern |
|---|---|
| Rust | `src/**/*.rs` |
| JavaScript | `res/js/**/*.js` |
| HTML | `res/**/*.html` |
| CSS | `res/css/**/*.css` |
| SQL | `sql/**/*.sql` + `res/sql/**/*.sql` |
| Markdown (root) | `./*.md` |
| Documentation | `docs/**/*.md` |

**Excluded:** `node_modules/`, binary files (images, fonts), build artifacts (`target/`, `tarpaulin-report.html`), generated stats (`stats_data.json`).

**Code total** = Rust + JavaScript + HTML + CSS + SQL
**Documentation total** = Markdown (root) + Documentation (docs/)
**Grand total** = Code total + Documentation total

---

## 全タグの行数履歴 / Line count by tag

各タグを `git worktree` で展開し、最新の `count_lines.sh` を適用して計測した結果。

Measured by checking out each tag via `git worktree` and running the latest `count_lines.sh`.

| Tag | Date (UTC) | Rust | JS | HTML | CSS | SQL | MD-root | docs/ | **Code** | **Docs** | **Grand** |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| v1.0.0 | 2025-11-21 | 3,624 | 2,930 | 519 | 1,072 | 2,249 | 2 | 0 | **10,394** | **2** | **10,396** |
| v1.0.1 | (early v1) | 3,657 | 3,367 | 519 | 1,072 | 2,249 | 397 | 1,082 | 10,864 | 1,479 | 12,343 |
| **v1.0.2** | (early v1) | **13,958** | **8,769** | **3,475** | **6,109** | **6,251** | **3,657** | **42,122** | **38,562** | **45,779** | **84,341** |
| v1.0.7 | (mid v1) | 13,958 | 8,769 | 3,475 | 6,109 | 6,251 | 3,657 | 42,122 | 38,562 | 45,779 | 84,341 |
| v1.0.8 | (mid v1) | 13,961 | 8,769 | 3,475 | 6,109 | 6,251 | 3,724 | 43,910 | 38,565 | 47,634 | 86,199 |
| v1.0.9 | (mid v1) | 13,975 | 8,769 | 3,475 | 6,109 | 6,251 | 3,753 | 43,910 | 38,579 | 47,663 | 86,242 |
| v1.1.0 | (mid v1) | 13,975 | 9,667 | 3,629 | 6,498 | 6,649 | 3,761 | 44,137 | 40,418 | 47,898 | 88,316 |
| v1.1.1 | (mid v1) | 13,968 | 9,667 | 3,629 | 6,498 | 6,649 | 3,930 | 44,454 | 40,411 | 48,384 | 88,795 |
| v1.1.2 | (late v1) | 15,400 | 9,795 | 3,632 | 6,559 | 6,661 | 4,009 | 44,454 | 42,047 | 48,463 | 90,510 |
| v1.2.0 | (late v1) | 15,603 | 9,935 | 3,682 | 6,603 | 6,735 | 4,059 | 44,454 | 42,558 | 48,513 | 91,071 |
| v1.2.1 | (late v1) | 15,603 | 9,941 | 3,682 | 6,603 | 6,735 | 4,083 | 44,454 | 42,564 | 48,537 | 91,101 |
| v1.2.2 | 2026-04-29 | 15,734 | 9,941 | 3,582 | 6,603 | 6,735 | 4,114 | 44,454 | 42,595 | 48,568 | 91,163 |
| v2.0.0 | 2026-04-30 | 17,156 | 10,359 | 3,628 | 6,683 | 6,787 | 4,216 | 44,454 | 44,613 | 48,670 | 93,283 |
| v2.0.1 | 2026-05-01 | 17,125 | 10,379 | 3,628 | 6,683 | 6,787 | 4,289 | 44,454 | 44,602 | 48,743 | 93,345 |
| v2.1.0 | 2026-05-04 | 19,277 | 10,851 | 4,007 | 6,683 | 7,049 | 4,367 | 44,454 | 47,867 | 48,821 | 96,688 |
| v2.1.1 | 2026-05-13 | 19,277 | 10,851 | 4,007 | 6,683 | 7,049 | 4,429 | 44,629 | 47,867 | 49,058 | 96,925 |
| v2.2.0 | 2026-05-19 | 20,173 | 11,796 | 4,011 | 6,683 | 7,126 | 4,454 | 44,639 | 49,789 | 49,093 | 98,882 |
| v2.3.0 | 2026-05-23 | 20,697 | 12,033 | 4,050 | 6,620 | 7,186 | 4,521 | 44,639 | 50,586 | 49,160 | 99,746 |

**Note on dates:** v1.0.0 = 2025-11-21 (initial release). Tag UTC dates for v1.2.2 onward are confirmed via `gh release view`. Mid-v1 tags are placed chronologically; precise dates can be retrieved via `git log <tag> -1 --format=%aI`.

---

## マイルストーン / Milestones

### v1.0.0 → v1.0.2: ドキュメント整備の起点 / Documentation foundation

- v1.0.0 時点では `docs/` ディレクトリが存在せず、ルートに `README.md` のみ（2 行）
- v1.0.1 で `docs/` が新設され、初期ドキュメント 1,082 行を配置
- **v1.0.2 で `docs/` が一気に 42,122 行**（前バージョンの 38 倍以上）に拡張。Rust/JS/HTML/CSS/SQL/Markdown root もそれぞれ 3〜6 倍に拡張
- Code total: 10,864 → 38,562、Grand total: 12,343 → 84,341 と短期間で **約 7 倍** に成長
- v1.0.2 は KakeiBon の「ドキュメント駆動開発体制」の起点となるタグ

At v1.0.0 there was no `docs/` directory and only `README.md` (2 lines) at root. v1.0.1 introduced `docs/` with 1,082 initial lines. v1.0.2 expanded `docs/` to 42,122 lines (38× growth) and tripled-to-sextupled every other category, marking the start of KakeiBon's documentation-driven development phase.

### v1.0.2 → v2.0.0: 安定的な成長 / Steady growth

- 約 5 ヶ月で Grand total 84,341 → 93,283（+8,942 行、+10.6%）
- Documentation total はほぼ横ばい（45,779 → 48,670）、コードが主に成長

### v2.0.0: 税計算リファクタ / Tax calculation refactor

- AMOUNT セマンティクスを「税抜固定」に変更するメジャーリリース
- Rust が +1,422（v1.2.2 → v2.0.0）増加、SQL も +52
- 詳細は [CHANGELOG_ja.md](../../CHANGELOG_ja.md) / [CHANGELOG_en.md](../../CHANGELOG_en.md)

### v2.1.0: 繰り返し予定入出金 / Recurring scheduled transactions

- 純粋関数（services/recurring.rs）+ Cycle ↔ DB row converter + 一括登録ロジック + UI を追加
- Rust +2,152（17,125 → 19,277）、JS +472、HTML +379、SQL +262
- Documentation total が初めて Code total を抜く（48,821 > 47,867）
- 詳細は [CHANGELOG_ja.md](../../CHANGELOG_ja.md) / [CHANGELOG_en.md](../../CHANGELOG_en.md)

### v2.1.1: errata + セキュリティ対応 / Errata + security response

- **コード変更ゼロ**のリリース（Tauri 2.11.1 バンプは `Cargo.toml`/`Cargo.lock`、CI 更新は `.github/workflows/` で、いずれも本ドキュメントの計測対象外）
- ドキュメントのみ +237 行：MD-root +62（README に Errata / Disclosure セクション新設）、`docs/` +175（本ドキュメント `CODE_LINE_HISTORY.md` 自体の追加）
- Tauri 2.11.1（GHSA-7gmj-67g7-phm9）への迅速対応 + v1.0.0 行数訂正の正式公開
- 詳細は [CHANGELOG_ja.md](../../CHANGELOG_ja.md) / [CHANGELOG_en.md](../../CHANGELOG_en.md)

A documentation-only release. Code total unchanged because the Tauri 2.11.1 bump (security fix for GHSA-7gmj-67g7-phm9) lives in `Cargo.toml`/`Cargo.lock` and CI updates in `.github/workflows/`, none of which are measured by `count_lines.sh`. The +237 documentation lines consist of the README Errata/Disclosure section (+62) and this `CODE_LINE_HISTORY.md` document itself (+175).

### v2.2.0: toast 通知・バリデーション基盤 / Toast notifications + validation foundation

- `alert()` → `showToast()` 全 20+ 箇所移行 + 文字数カウンター付きバリデーション基盤を全管理画面に展開
- Rust +896（19,277 → 20,173）、JS +945（10,851 → 11,796）、HTML +4、SQL +77、MD-root +25、docs/ +10
- **Code total が初めて 49,000 行を突破**（47,867 → 49,789、+1,922）
- 詳細は [CHANGELOG_ja.md](../../CHANGELOG_ja.md) / [CHANGELOG_en.md](../../CHANGELOG_en.md)

Migrated 20+ `alert()` calls to the non-blocking `showToast()` utility and rolled out a live character-counter validation layer across every management screen. Code total crossed 49k lines for the first time (47,867 → 49,789, +1,922).

### v2.3.0: 集計起算日カスタマイズ / Period start day customization

- 月次・年次集計の起算日をユーザー設定可能に + 全集計画面のウィンドウ自動フィット + ダッシュボード時点ラベル + fcitx5 IME OFF (Linux)
- Rust +524（20,173 → 20,697）、JS +237、HTML +39、SQL +60、MD-root +67、docs/ ±0、CSS は -63（main-content/section の flex 解除等で減少）
- 新規モジュール `services/period.rs`（純粋関数、テスト 16 件）、`res/js/window-fit.js`、`res/js/login-ime.js`、`res/js/period.js`
- **Grand total が 99,746 行で 100k 行に肉薄**
- 詳細は [CHANGELOG_ja.md](../../CHANGELOG_ja.md) / [CHANGELOG_en.md](../../CHANGELOG_en.md)

Added user-configurable monthly/yearly aggregation start days (Issue #24), auto-fitting and centering of aggregation/user-management windows, the dashboard "as of" balance label, and fcitx5 IME deactivation on the login screen for Linux. The new `services/period.rs` pure-function module (with 16 unit tests) anchors the cycle-boundary math and absorbs the duplicate `end_of_month` from `recurring.rs`. Grand total reached 99,746 lines, just shy of 100k.

---

## 過去に公開された誤った数値の訂正記録 / Errata for past published figures

過去のドキュメント・記事・LinkedIn 投稿等で、KakeiBon の規模を **「35,000 行」（特に v1.0.0 リリース時の規模として）** と表記した箇所がありますが、これは**誤りです**。

In several past documents, articles, and LinkedIn posts, KakeiBon's scale was stated as **"35,000 lines"** (particularly as the size at the v1.0.0 release). **This figure is incorrect.**

### 誤った記述 / Incorrect figures

- v1.0.0 時点で 35,000 行
- 「Rust 版：35,000 行 ÷ 1 ヶ月 = 35,000 行/月」
- 生産性比 11.67 倍（35,000 ÷ 3,000）

### 正しい記述 / Correct figures

| 主張 / Claim | 誤り / Incorrect | 正しい / Correct |
|---|---|---|
| v1.0.0 時点の Code total | 35,000 lines | **10,394 lines** |
| v1.0.0 時点の Grand total | 35,000 lines | **10,396 lines** |
| v1.0.2 時点の Code total | — | 38,562 lines |
| v1.0.2 時点の Grand total | — | 84,341 lines |
| v2.1.1 時点の Code total | — | 47,867 lines |
| v2.1.1 時点の Grand total | — | 96,925 lines |
| v2.3.0 時点の Code total | — | 50,586 lines |
| v2.3.0 時点の Grand total | — | 99,746 lines |

### 「35,000 行」の出所推定 / Likely origin of the 35,000 figure

実数を見ると、**v1.0.2 時点の Code total が 38,562 行** で 35,000 に最も近い値となっています。v1.0.0 リリース直後に v1.0.2 でドキュメントとコードを大幅拡張したため、**v1.0.2 時点の数値を v1.0.0 リリース時の実績として認識・公開してしまった**可能性が高いと推定されます。

The actual v1.0.2 Code total of 38,562 lines is closest to 35,000. The likely cause is that the v1.0.2-era figure was misattributed to v1.0.0 itself, given that v1.0.2 was released shortly after v1.0.0 with a large expansion of code and documentation.

### 影響範囲 / Scope of correction

ローカル `.ai-context/shared/insights/` 配下の以下のドキュメントに「35,000 行」の記載があります（ローカルのみ、未公開）：

The following documents under `.ai-context/shared/insights/` (local-only, not published externally) contain the "35,000 lines" figure:

- `INSIGHTS_OVERVIEW.md`
- `archive/AI_COGNITIVE_AUGMENTATION.md`
- `archive/CONTENT_STRATEGY.md`
- `archive/ONLINE_TO_REAL_WORLD.md`
- `archive/SOFTWARE_AS_ORGANISM.md`

これらは順次訂正されます。外部公開済みのコンテンツ（Qiita 記事・LinkedIn 投稿など）に同じ数値が含まれている場合は、**本ドキュメントを参照して訂正**してください。

These will be corrected sequentially. If externally published content (Qiita articles, LinkedIn posts) contains the same figure, please refer to this document for correction.

### 開示 / Disclosure

この誤認の発生について、可能性として **発信者の統合失調症由来の認知の歪み（誤認）** が関与した可能性があります。今後は本ドキュメントおよび `count_lines.sh` を**一次情報源**とし、数値を引用する際は再計測の上で記載します。

The author of KakeiBon publicly discloses that **cognitive distortions related to schizophrenia (the author has been openly diagnosed and is undergoing treatment)** may have contributed to this misattribution. Going forward, this document and `count_lines.sh` will be the canonical source, and any cited figures will be re-measured before publication.

---

## 再現方法 / How to reproduce

```bash
# 現在の状態を計測 / Measure current state
./count_lines.sh

# JSON で出力 / Output as JSON
./count_lines.sh --json

# 特定タグでの計測 / Measure at a specific tag
git worktree add --detach /tmp/kakei-snap <tag>
cp count_lines.sh /tmp/kakei-snap/
( cd /tmp/kakei-snap && bash count_lines.sh )
git worktree remove --force /tmp/kakei-snap
```

このドキュメントの更新は、**新しいリリースタグを切るたびに行います**。

This document is updated at every new release tag.
