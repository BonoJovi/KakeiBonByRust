# Changelog

All notable changes to this project will be documented in this file.

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
