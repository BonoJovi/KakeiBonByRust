# 変更履歴

このファイルには、プロジェクトのすべての重要な変更が記録されます。

## [未リリース] - 2025-11-10

### 追加

**feat(i18n): CATEGORY2_I18NとCATEGORY3_I18Nに日本語リソースを追加**

#### データベース初期化:
- CATEGORY2_I18Nテーブルに日本語リソースを追加（ユーザーごとに20件）
- CATEGORY3_I18Nテーブルに日本語リソースを追加（ユーザーごとに126件）
- `res/sql/default_categories_seed.sql`を更新し、日本語と英語の両方のリソースを含めた
- I18Nエントリー総数: CATEGORY2_I18N 40件(ja+en)、CATEGORY3_I18N 252件(ja+en)

#### 一貫性の向上:
- すべてのカテゴリI18Nテーブル（CATEGORY1_I18N、CATEGORY2_I18N、CATEGORY3_I18N）で一貫したバイリンガルサポートを実現
- データベース初期化時に日本語リソースが自動投入されるようになった
- 費目管理UIで日本語名が正しく表示されるようになった

### 修正

**fix(account): AccountとAccountTemplate構造体にsqlxマッピング属性を追加**

#### バグ修正:
- `src/services/account.rs`の`AccountTemplate`構造体に`#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]`属性を追加
- `src/services/account.rs`の`Account`構造体に`#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]`属性を追加
- SQLiteの大文字カラム名とRustの小文字フィールド名のマッピング問題を修正
- ユーザー登録時の「no column found for name: template_id」エラーを解決

---

## [初期リリース] - 2025-10-25

### 追加 - Commit 1 (8e20ede3b7dfd7cdd3c5bec0590bbca15e252f40)

**docs: ドキュメントを言語別に整理 & consts.defをconsts.rsにリネーム**

#### ドキュメントの整理:
- 日英混在のドキュメントを言語別ディレクトリに分離
- `docs/ja/`に日本語版を配置
- `docs/en/`に英語版を配置
- 将来的な多言語対応に備えた構成

#### ドキュメントの改善:
- テストセクションを「実装済み」と「今後の実装予定」に明確に分離
- 非選択状態の説明を正確に修正（グレーの枠線 #666）
- 「視力の弱い方（ロービジョンの方）」の表記に統一
- より柔らかい表現に修正

#### ファイルリネーム:
- `res/consts.def` → `res/consts.rs` (Rustの命名規則に準拠)

#### ドキュメントポリシー:
今後、新しい言語のドキュメントは対応する言語ディレクトリに配置
- 例: 中国語 → `docs/zh/`
