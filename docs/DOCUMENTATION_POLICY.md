# Documentation Policy / ドキュメント作成ポリシー

## Overview / 概要

This document defines the documentation creation and maintenance policy for the KakeiBot project.

本ドキュメントは、KakeiBonプロジェクトにおけるドキュメント作成・管理のポリシーを定義します。

**作成日 / Created**: 2025-11-05 23:39 JST  
**最終更新 / Last Updated**: 2025-11-05 23:39 JST

---

## Core Principles / 基本方針

### 1. Bilingual Support / 二言語対応

**Policy / 方針**:
- All user-facing documentation must be provided in both Japanese and English
- 全てのユーザー向けドキュメントは日本語・英語の両方で提供する

**Rationale / 理由**:
- This project targets both Japanese and international users
- 本プロジェクトは日本人ユーザーと国際ユーザーの両方を対象とする

**Impact / 影響**:
- Documentation volume doubles (acceptable trade-off)
- ドキュメント量が2倍になる（許容範囲内のトレードオフ）

### 2. Phased Approach / 段階的アプローチ

Documentation maintenance is divided into two phases:
ドキュメント管理を2つのフェーズに分割する：

#### Phase A: Development Phase (Current) / 開発フェーズ（現在）

**Duration / 期間**: Until all features are implemented / 全機能実装完了まで

**Guidelines / ガイドライン**:
- ✅ Write freely as implementation notes / 実装メモとして自由に記述
- ✅ Don't worry about granularity / 粒度は気にしない
- ✅ Allow duplication across documents / ドキュメント間の重複を許容
- ✅ Include temporary development information / 開発時の一時的な情報も含める
- ✅ Mix code explanations and feature descriptions / コード解説と機能説明の混在OK

**Purpose / 目的**:
- Support rapid development / 迅速な開発をサポート
- Capture implementation context while fresh / 実装時の文脈を記録
- Avoid premature optimization / 早すぎる最適化を避ける

#### Phase B: Release Preparation Phase / リリース準備フェーズ

**Timing / タイミング**: After all features are implemented / 全機能実装後

**Sub-phases / サブフェーズ**:

##### Phase B-1: Inventory / 棚卸し
- Review all documentation files / 全ドキュメントファイルをレビュー
- Identify duplicate content / 重複コンテンツの特定
- Identify temporary/unnecessary information / 一時的・不要な情報の特定
- Redefine role of each document / 各ドキュメントの役割を再定義

##### Phase B-2: Consolidation / 統合・分離
- Consolidate documents where appropriate / 適切な箇所でドキュメントを統合
- Separate only when truly necessary / 本当に必要な場合のみ分離
- Minimize total file count / ファイル総数を最小化
- Remove redundant information / 冗長な情報を削除

##### Phase B-3: Audience-based Restructuring / 視点別再構成
- Separate by target audience / 対象読者別に分離
- **End Users**: How to use features / エンドユーザー: 機能の使い方
- **Developers**: Implementation details / 開発者: 実装詳細
- Clear separation of concerns / 関心事の明確な分離

---

## Document Structure / ドキュメント構造

### Current Structure (Development Phase) / 現在の構造（開発フェーズ）

```
docs/
  ├── ja/                           # Japanese documentation
  │   ├── *.md                      # Various implementation docs
  │   └── ...
  ├── en/                           # English documentation
  │   ├── *.md                      # Various implementation docs
  │   └── ...
  ├── TRANSACTION_REQUIREMENTS.md   # Shared requirements docs
  └── DOCUMENTATION_POLICY.md       # This file
```

**Characteristics / 特徴**:
- Many granular documents / 多数の細かいドキュメント
- Mixed purposes / 用途が混在
- Development-focused / 開発重視

### Target Structure (Release Phase) / 目標構造（リリースフェーズ）

```
docs/
  ├── ja/
  │   ├── USER_GUIDE.md           # User manual
  │   ├── DEVELOPER_GUIDE.md      # Developer documentation
  │   ├── API_REFERENCE.md        # API specifications
  │   └── ARCHITECTURE.md         # Design philosophy
  ├── en/
  │   ├── USER_GUIDE.md
  │   ├── DEVELOPER_GUIDE.md
  │   ├── API_REFERENCE.md
  │   └── ARCHITECTURE.md
  └── DOCUMENTATION_POLICY.md     # This file
```

**Characteristics / 特徴**:
- Consolidated, focused documents / 統合された焦点の絞られたドキュメント
- Clear audience separation / 明確な読者の分離
- User-focused / ユーザー重視

**Note / 注記**: This structure is tentative and may be adjusted based on needs that arise during development.
この構造は暫定的であり、開発中に生じるニーズに応じて調整される可能性があります。

---

## Status Indicators / ステータス表示

### Implementation Status / 実装状況

Use the following markers to indicate implementation status:
実装状況を示すために以下のマーカーを使用する：

- ✅ **Complete / 完了**: Implementation finished
- ⏳ **In Progress / 進行中**: Currently being implemented
- ❌ **Not Started / 未着手**: Not yet started
- ⚠️ **Needs Attention / 要注意**: Issues or blockers

### Test Status / テスト状況

When implementation is complete but testing is incomplete, add notes:
実装完了だがテストが未完了の場合、注記を追加する：

**Examples / 例**:
- `✅ Pagination (50 items/page) (テスト未実施)` / `(test not implemented)`
- `**Test Status**: Manual testing only (automated tests not implemented)`
- `**注記**: フロントエンド自動テストは未実施（手動動作確認のみ）`

**Purpose / 目的**:
- Maintain document clarity / ドキュメントの明確性を維持
- Avoid misleading readers / 読者を誤解させない
- Keep implementation and testing status separate / 実装とテストの状況を分離

---

## Timestamp Format / タイムスタンプ形式

### User-facing Documentation / ユーザー向けドキュメント

**Format / 形式**: Japan Standard Time (JST, UTC+9)

**Examples / 例**:
```markdown
**Last Updated**: 2025-11-05 23:39 JST
**完了日**: 2025-11-05 18:12 JST
```

**Rationale / 理由**:
- Primary audience is in Japan / 主な読者が日本在住
- Easier to understand for Japanese users / 日本人ユーザーにとって理解しやすい

### AI Context Documentation / AIコンテキストドキュメント

**Format / 形式**: Either UTC or JST is acceptable / UTCまたはJSTのいずれも可

**Rationale / 理由**:
- Internal documentation for AI/LLM / AI/LLM用の内部ドキュメント
- Flexibility is acceptable / 柔軟性が許容される

---

## Review and Updates / レビューと更新

### When to Update This Policy / 本ポリシーの更新タイミング

- When new documentation needs arise / 新しいドキュメントニーズが生じた時
- When current policy proves inadequate / 現在のポリシーが不十分と判明した時
- Before entering Release Preparation Phase / リリース準備フェーズに入る前
- When project scope significantly changes / プロジェクトスコープが大きく変わった時

### Review Frequency / レビュー頻度

- Minimum: Before release / 最低: リリース前
- Recommended: At major milestones / 推奨: 主要マイルストーン毎

---

## References / 参考

### Related Documents / 関連ドキュメント
- [TODO.md](../TODO.md) - Development task management
- [.ai-context/CONVENTIONS.md](../.ai-context/CONVENTIONS.md) - Coding conventions
- [.github/copilot-instructions.md](../.github/copilot-instructions.md) - AI guidelines

### External Resources / 外部リソース
- [Write the Docs](https://www.writethedocs.org/) - Documentation best practices
- [Documentation Guide by Google](https://google.github.io/styleguide/docguide/) - Style guide

---

**Approved by / 承認者**: Development Team  
**Next Review / 次回レビュー**: Before Release / リリース前
