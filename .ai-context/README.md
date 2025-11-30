# AI Context Directory

このディレクトリには、AI/LLMアシスタントがプロジェクトを理解するためのコンテキスト情報が含まれています。

## ファイル一覧

- **CONVENTIONS.md**: コーディング規約、命名規則、パターン
- **PROJECT_STRUCTURE.md**: プロジェクト構造、モジュール責務、データフロー
- **projects-guidelines.md**: GitHub Projects使用ガイドライン
- **init.zsh**: コンテキスト初期化スクリプト（レガシー）

## GitHub Copilot CLI統合

GitHub Copilot CLIは自動的に `.github/copilot-instructions.md` を読み込みます。
このファイルは `.ai-context/` の内容を要約したものです。

### 使用方法

1. **自動読み込み**: GitHub Copilot CLIを起動すると自動的に読み込まれます
2. **手動確認**: `cat .github/copilot-instructions.md` で内容を確認できます

### 従来の方法（レガシー）

```bash
# 従来はセッション開始時に手動実行が必要でした
ghcp-init
```

現在は不要です。GitHub Copilot CLIが自動的にコンテキストを読み込みます。

## 詳細情報の参照

詳細な情報が必要な場合は、このディレクトリ内のファイルを直接参照してください：

```bash
# 例：コーディング規約の確認
cat .ai-context/CONVENTIONS.md

# 例：プロジェクト構造の確認
cat .ai-context/PROJECT_STRUCTURE.md
```

---

**最終更新**: 2025-10-29
