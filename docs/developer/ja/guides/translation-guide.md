# 翻訳ガイド

**バージョン**: 1.0
**最終更新**: 2025-12-04
**対象読者**: 翻訳者（プログラミング経験不要）

---

## [List] 目次

- [概要](#概要)
- [はじめに](#はじめに)
- [翻訳システムのアーキテクチャ](#翻訳システムのアーキテクチャ)
- [新しい言語を追加する方法](#新しい言語を追加する方法)
- [翻訳ワークフロー](#翻訳ワークフロー)
- [翻訳ガイドライン](#翻訳ガイドライン)
- [翻訳のテスト](#翻訳のテスト)
- [翻訳の提出](#翻訳の提出)
- [よくある質問](#よくある質問)

---

## 概要

KakeiBonは、データベース駆動の国際化（i18n）システムを使用しています。すべてのユーザー向けテキストは、SQLiteデータベースの`I18N_RESOURCES`テーブルに保存されています。

### 現在の状況

- **サポート言語**: 日本語（ja）、英語（en）
- **翻訳キー総数**: 650+エントリ
- **翻訳カバレッジ**: jaとenで100%

### なぜ貢献するのか？

あなたの翻訳は、KakeiBonを世界中のユーザーが利用できるようにします。プログラミング知識は不要です—対象言語に堪能であれば大丈夫です！

---

## はじめに

### 前提条件

**プログラミング経験は不要です！** 必要なのは以下だけです：

1. 対象言語への堪能さ
2. 家計簿・財務用語の基本的な理解
3. テキストエディタ（メモ帳、テキストエディット、または任意のエディタ）
4. オプション: KakeiBonをソースからビルドする能力（テスト用）

### クイックスタート

1. **あなたの言語がすでにサポートされているか確認**
   - 現在: 日本語（ja）、英語（en）
   - [未対応の翻訳リクエストを確認](https://github.com/BonoJovi/KakeiBonByRust/issues?q=label%3Atranslation)

2. **関心を表明**
   - [翻訳リクエストを提出](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=translation.yml)
   - 翻訳したい言語を指定

3. **翻訳テンプレートを入手**
   - CSVまたはスプレッドシートのテンプレートを提供します
   - すべての英語キーと値が含まれています

4. **翻訳して提出**
   - 翻訳を記入
   - GitHub IssueまたはEmailで提出

---

## 翻訳システムのアーキテクチャ

### 仕組み

```
┌─────────────────────────────────────────────────────────────┐
│                    SQLiteデータベース                         │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐ │
│  │              I18N_RESOURCESテーブル                    │ │
│  ├──────────────┬──────────┬─────────────┬──────────────┤ │
│  │ RESOURCE_KEY │ LANG_CODE│   CATEGORY  │RESOURCE_VALUE│ │
│  ├──────────────┼──────────┼─────────────┼──────────────┤ │
│  │ app.title    │   en     │   general   │  KakeiBon    │ │
│  │ app.title    │   ja     │   general   │  家計簿      │ │
│  │ app.title    │   zh     │   general   │  记账本      │ │
│  └──────────────┴──────────┴─────────────┴──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↓
              ┌─────────────────────────────┐
              │   Rustバックエンド（Tauri）   │
              │   i18nサービス              │
              └─────────────────────────────┘
                            ↓
              ┌─────────────────────────────┐
              │   JavaScriptフロントエンド   │
              │   i18n.js                   │
              └─────────────────────────────┘
                            ↓
              ┌─────────────────────────────┐
              │   ユーザーインターフェース    │
              │   <span data-i18n="key">    │
              └─────────────────────────────┘
```

### データベーススキーマ

```sql
CREATE TABLE I18N_RESOURCES (
    RESOURCE_KEY   TEXT NOT NULL,   -- 一意のキー（例: "app.title"）
    LANG_CODE      TEXT NOT NULL,   -- 言語コード（ISO 639-1: en, ja, zh）
    CATEGORY       TEXT,             -- カテゴリ（general, menu, errorなど）
    RESOURCE_VALUE TEXT NOT NULL,   -- 翻訳されたテキスト
    PRIMARY KEY (RESOURCE_KEY, LANG_CODE)
);
```

### 翻訳カテゴリ

| カテゴリ | 説明 | キーの例 |
|----------|------|----------|
| `general` | 一般的なUI要素 | `app.title`, `app.description` |
| `menu` | メニュー項目 | `menu.file`, `menu.settings` |
| `button` | ボタンラベル | `btn.save`, `btn.cancel` |
| `label` | フォームラベル | `label.username`, `label.password` |
| `msg` | メッセージ | `msg.save_success`, `msg.error` |
| `error` | エラーメッセージ | `error.invalid_input` |
| `validation` | バリデーションメッセージ | `validation.required` |
| `placeholder` | 入力プレースホルダー | `placeholder.search` |

---

## 新しい言語を追加する方法

### ステップ1: 言語コードの選択

**ISO 639-1**言語コードを使用します：

| 言語 | コード | 例 |
|------|--------|-----|
| 中国語（簡体字） | `zh` | 记账本 |
| 韓国語 | `ko` | 가계부 |
| フランス語 | `fr` | Livre de comptes |
| ドイツ語 | `de` | Haushaltsbuch |
| スペイン語 | `es` | Libro de cuentas |
| ポルトガル語 | `pt` | Livro de contas |
| イタリア語 | `it` | Libro dei conti |
| ロシア語 | `ru` | Домашняя бухгалтерия |

### ステップ2: 翻訳テンプレートをリクエスト

1. [Issues](https://github.com/BonoJovi/KakeiBonByRust/issues)にアクセス
2. 「New Issue」→「Translation Request」をクリック
3. 以下を記入:
   - **対象言語**: 例：「中国語（簡体字）」
   - **言語コード**: 例：「zh」
   - **あなたの対応可能性**: 翻訳できる？レビューできる？

### ステップ3: テンプレートを受け取る

メンテナーが以下を提供します：

- **CSVファイル**または**Google Sheets**:
  - 列A: `RESOURCE_KEY`
  - 列B: `CATEGORY`
  - 列C: `RESOURCE_VALUE (English)`（参照用）
  - 列D: `RESOURCE_VALUE (Your Language)`（記入用）

CSVの構造例：

```csv
RESOURCE_KEY,CATEGORY,ENGLISH,YOUR_LANGUAGE
app.title,general,KakeiBon,
app.description,general,Household Budget Manager,
menu.file,menu,File,
menu.settings,menu,Settings,
btn.save,button,Save,
btn.cancel,button,Cancel,
```

---

## 翻訳ワークフロー

### 1. 英語リファレンスの確認

すべての英語翻訳を読んで理解する：
- アプリの機能
- 各キーのコンテキスト
- トーンと丁寧さのレベル

### 2. キーの翻訳

各行について：
1. **コンテキストを理解**
   - このキーは何を表していますか？
   - UIのどこで使われていますか？
   - ボタン、ラベル、メッセージのどれですか？

2. **正確に翻訳**
   - 元の意味を維持
   - あなたの言語で自然な表現を使用
   - 文化的な適切性を考慮

3. **一貫性を保つ**
   - 全体を通して同じ用語を使用
   - 同じトーンと丁寧さのレベルを保つ

### 3. 特殊なケースの処理

#### 文字列内のパラメータ

一部の文字列には`{0}`、`{1}`のようなプレースホルダーが含まれています：

```
English: "Language changed to {0}."
Japanese: "言語を{0}に変更しました。"
Chinese: "语言已更改为{0}。"
```

**重要**: `{0}`、`{1}`のプレースホルダーは、あなたの言語で意味が通る位置に保持してください。

#### HTMLタグ

一部の文字列にはHTMLタグが含まれている場合があります：

```
English: "Click <strong>Save</strong> to continue."
```

**重要**: HTMLタグは保持し、テキスト内容のみを翻訳してください。

#### 単位と数値

地域差を考慮：
- 日付形式（MM/DD/YYYY vs DD/MM/YYYY）
- 通貨記号（¥ vs $ vs €）
- 数値区切り（1,000.00 vs 1.000,00）

---

## 翻訳ガイドライン

### 1. 正確性

✅ **良い例**: 元の意味を保った正確な翻訳
```
English: "Delete this account?"
Chinese: "删除此账户？"
```

❌ **悪い例**: 誤訳または省略
```
English: "Delete this account?"
Chinese: "删除？"（「this account」が欠落）
```

### 2. 一貫性

全体を通して一貫した用語を維持：

| 用語 | 日本語 | 中国語 | 韓国語 |
|------|--------|--------|--------|
| Account | 口座 | 账户 | 계좌 |
| Transaction | 入出金 | 交易 | 거래 |
| Category | 費目 | 类别 | 카테고리 |
| Budget | 予算 | 预算 | 예산 |

### 3. 自然な言語

あなたの言語にとって自然な表現を使用：

✅ **良い例**: ネイティブな自然な表現
```
English: "Are you sure you want to delete?"
Japanese: "本当に削除しますか？"
```

❌ **悪い例**: 直訳（不自然に聞こえる）
```
Japanese: "あなたは削除したいですか確実ですか？"
```

### 4. 丁寧さのレベル

KakeiBonは**丁寧だが過度にフォーマルでない**言語を使用：

- カジュアルすぎない（例：「これ保存する？」）
- フォーマルすぎない（例：「保存していただけますでしょうか」）
- プロフェッショナルでフレンドリー

### 5. 文化的適切性

文化的コンテキストを考慮：
- 金融用語は地域によって異なる場合があります
- 一部の概念は直接翻訳できない場合があります
- 文化的に適切な例を使用

---

## 翻訳のテスト

### オプション1: 視覚的レビュー

コンテキストで翻訳を確認：
1. CSV/スプレッドシートを読む
2. 各テキストがUIでどのように表示されるか想像
3. 一貫性と自然さをチェック

### オプション2: アプリでテスト（推奨）

ソースからKakeiBonをビルドできる場合：

1. **依存関係をインストール**（[インストールガイド](../../../user/ja/installation.md)を参照）

2. **データベースに翻訳を追加**
   - メンテナーがこのステップをサポートします
   - または提供されたSQLスクリプトを使用

3. **ビルドして実行**
   ```bash
   cargo tauri build
   ./target/release/KakeiBon
   ```

4. **言語を切り替える**
   - 設定 → 言語 → あなたの言語を選択

5. **すべての画面を確認**
   - すべてのメニュー、ボタン、メッセージをチェック
   - 以下を探す：
     - 切り捨てられたテキスト（長すぎる）
     - ずれたUI要素
     - 誤った翻訳

6. **問題を報告**
   - スクリーンショットを撮る
   - 調整が必要なキーをメモ

---

## SQL INSERT文の作成

KakeiBonはすべての翻訳をSQLiteデータベースに保存します。翻訳が完了したら、INSERT文を含むSQLファイルを作成する必要があります。

### なぜSQL？

- **データベース駆動**: すべてのi18nリソースは`I18N_RESOURCES`テーブルに保存されます
- **自動読み込み**: `sql/init/i18n/`内のSQLファイルはデータベース初期化時に実行されます
- **バージョン管理**: SQLファイルで翻訳の変更を追跡できます

### SQLファイルの構造

各翻訳SQLファイルは以下のテンプレートに従います：

```sql
-- Translation resources for [カテゴリ]
-- Language: [言語名] ([言語コード])
-- Category: [カテゴリ]

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(RESOURCE_ID, 'resource.key', 'lang', '翻訳されたテキスト', 'category', '説明', datetime('now')),
(RESOURCE_ID, 'resource.key', 'lang', '翻訳されたテキスト', 'category', '説明', datetime('now'));
```

### ステップバイステップガイド

#### ステップ1: 最新のRESOURCE_IDを取得

SQLファイルを作成する前に、競合を避けるために次に使用可能なRESOURCE_IDを知る必要があります。

**現在の最高ID**: ~1290（2025-12-04時点）

**メンテナーに連絡**して、現在の最高IDを取得し、あなたの言語用の範囲を予約してください。

例：
```
あなたの言語: 中国語（zh）
予約されたID範囲: 1291-1940（650キー分の650 ID）
```

#### ステップ2: ファイル命名規則を選択

SQLファイルは説明的な名前にする必要があります：

```
sql/init/i18n/init_[カテゴリ]_i18n_[言語コード].sql
```

例：
- `init_app_i18n_zh.sql` - 中国語のアプリ翻訳
- `init_menu_i18n_ko.sql` - 韓国語のメニュー翻訳
- `init_common_i18n_fr.sql` - フランス語の共通翻訳

**完全な言語の場合**: カテゴリごとに1ファイル、または1つの大きなファイルを作成できます：

```
sql/init/i18n/init_all_i18n_zh.sql
```

#### ステップ3: CSVをSQLに変換

完成した翻訳CSVを使用して、INSERT文を作成します。

**例CSV**:
```csv
RESOURCE_KEY,CATEGORY,ENGLISH,CHINESE
app.title,general,KakeiBon,记账本
app.description,general,Household Budget Manager,家庭预算管理器
menu.file,menu,File,文件
btn.save,button,Save,保存
```

**SQLに変換**:
```sql
-- Translation resources for Chinese (Simplified)
-- Language: Chinese (zh)
-- Auto-generated from translation CSV

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(1291, 'app.title', 'zh', '记账本', 'general', 'Application title', datetime('now')),
(1292, 'app.description', 'zh', '家庭预算管理器', 'general', 'Application description', datetime('now')),
(1293, 'menu.file', 'zh', '文件', 'menu', 'File menu', datetime('now')),
(1294, 'btn.save', 'zh', '保存', 'button', 'Save button', datetime('now'));
```

#### ステップ4: ID一貫性の維持

**重要なルール**:

1. **連続したID**: 連続したRESOURCE_IDを使用（1291, 1292, 1293...）
2. **隙間なし**: 意図的でない限りIDをスキップしない
3. **重複なし**: 各RESOURCE_IDはすべての言語で一意でなければならない
4. **同じキー、異なるID**: 異なる言語の同じ`RESOURCE_KEY`は異なるIDを取得

**例**:
```sql
-- 英語
(401, 'app.title', 'en', 'KakeiBon', 'general', 'App title', datetime('now')),

-- 日本語
(402, 'app.title', 'ja', '家計簿', 'general', 'App title', datetime('now')),

-- 中国語（あなたの新しい翻訳）
(1291, 'app.title', 'zh', '记账本', 'general', 'App title', datetime('now')),
```

#### ステップ5: 特殊文字の処理

SQLではシングルクォートをエスケープする必要があります：

```sql
-- ❌ 間違い（SQLエラーになります）
'It's a test'

-- ✅ 正しい（シングルクォートを別のシングルクォートでエスケープ）
'It''s a test'
```

**例**:
```sql
(1295, 'msg.cant_delete', 'zh', '无法删除，因为它''s正在使用中', 'message', 'Cannot delete message', datetime('now')),
```

#### ステップ6: カテゴリ別に整理（オプション）

保守性を高めるために、カテゴリごとにINSERTをグループ化：

```sql
-- Translation resources for Chinese (Simplified)
-- Language: Chinese (zh)

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
-- General
(1291, 'app.title', 'zh', '记账本', 'general', 'App title', datetime('now')),
(1292, 'app.description', 'zh', '家庭预算管理器', 'general', 'App description', datetime('now')),

-- Menu
(1293, 'menu.file', 'zh', '文件', 'menu', 'File menu', datetime('now')),
(1294, 'menu.settings', 'zh', '设置', 'menu', 'Settings menu', datetime('now')),

-- Buttons
(1295, 'btn.save', 'zh', '保存', 'button', 'Save button', datetime('now')),
(1296, 'btn.cancel', 'zh', '取消', 'button', 'Cancel button', datetime('now'));
```

### ツールと自動化

#### CSVからSQLへの変換ツール（推奨）

メンテナーが変換スクリプトを提供できます：

```bash
# CSVをSQLに変換（メンテナーが提供するスクリプト）
python3 scripts/csv_to_sql.py translations_zh.csv --lang zh --start-id 1291 > init_all_i18n_zh.sql
```

**またはメンテナーにCSVを変換してもらうこともできます！**

#### 手動変換テンプレート

手動で変換する場合は、このテンプレートを使用：

```sql
-- Translation resources for [あなたの言語名]
-- Language: [あなたの言語名] ([言語コード])
-- Generated from: translations_[言語コード].csv
-- Date: [日付]

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
([開始ID], '[RESOURCE_KEY]', '[言語コード]', '[あなたの翻訳]', '[カテゴリ]', '[説明]', datetime('now')),
-- 各翻訳について繰り返し...
([終了ID], '[最後のキー]', '[言語コード]', '[最後の翻訳]', '[カテゴリ]', '[説明]', datetime('now'));
```

### 検証チェックリスト

SQLファイルを提出する前に、以下を確認：

- [ ] すべてのRESOURCE_IDが一意
- [ ] IDに隙間がない（意図的でない限り）
- [ ] IDが予約範囲内
- [ ] すべてのシングルクォートがエスケープされている（`''`）
- [ ] LANG_CODEが全体で一貫している
- [ ] ファイルがUTF-8エンコーディングを使用
- [ ] SQL構文が有効（末尾のカンマなど）
- [ ] `INSERT OR IGNORE`を使用（重複キーエラーを防ぐ）

### SQLファイルのテスト

ソースからKakeiBonをビルドできる場合：

1. **SQLファイルを配置** `sql/init/i18n/`に

2. **既存のデータベースを削除**（まずバックアップ！）
   ```bash
   # バックアップ
   cp ~/.local/share/KakeiBon/KakeiBonDB.sqlite3 ~/backup.db

   # 削除
   rm ~/.local/share/KakeiBon/KakeiBonDB.sqlite3
   ```

3. **リビルドして実行**
   ```bash
   cargo tauri build
   ./target/release/KakeiBon
   ```

4. **翻訳が読み込まれたことを確認**
   - 設定であなたの言語に切り替え
   - アプリ内を移動
   - すべてのテキストが正しく表示されることを確認

### よくある問題

#### 問題1: 重複IDエラー

```
Error: UNIQUE constraint failed: I18N_RESOURCES.RESOURCE_ID
```

**解決策**: SQLファイル内の重複IDまたは既存IDとの競合を確認してください。

#### 問題2: SQL構文エラー

```
Error: near line 42: syntax error
```

**解決策**: 以下を確認：
- 行間のカンマが欠けている
- 最後の行の末尾のカンマ（削除する）
- エスケープされていないシングルクォート

#### 問題3: エンコーディングの問題

```
Error: Invalid UTF-8 sequence
```

**解決策**: SQLファイルがUTF-8エンコーディングで保存されていることを確認してください。

---

## 翻訳の提出

### 方法1: GitHub Issue（推奨）

1. 翻訳CSV/スプレッドシートを完成させる
2. 元の翻訳リクエストIssueにアクセス
3. 完成したファイルを添付
4. 注意事項や質問を追加

### 方法2: プルリクエスト（上級者向け）

Gitに慣れている場合：

1. リポジトリをフォーク
2. `src-tauri/migrations/`に翻訳を追加
3. SQLマイグレーションファイルを作成（メンテナーがサポート）
4. プルリクエストを提出

### 方法3: Email

送信先: [bonojovi@zundou.org](mailto:bonojovi@zundou.org)
- 翻訳ファイルを添付
- 言語コードと注意事項を含める

---

## よくある質問

### Q: 650+のキーをすべて一度に翻訳する必要がありますか？

**A**: いいえ！段階的に翻訳できます：
1. 重要なキーから開始（アプリタイトル、メニュー項目、ボタン）
2. メッセージとエラーに移る
3. 使用頻度の低い文字列で完了

### Q: 翻訳が不確かな場合はどうすればよいですか？

**A**:
- GitHub Issueで質問する
- 複数の翻訳オプションを提供
- 不確実性をメモして、レビュアーが支援できるようにする

### Q: 既存の翻訳を改善できますか？

**A**: はい！現在の翻訳に問題を見つけた場合：
1. 翻訳改善Issueを提出
2. 現在の翻訳と提案する改善を提供
3. なぜあなたのバージョンがより良いか説明

### Q: 自分の言語に存在しない用語はどう扱いますか？

**A**: オプション：
1. 外来語を使用（例：「app」→「アプリ」）
2. 概念を説明する
3. コミュニティに提案を求める

### Q: 翻訳にクレジットされますか？

**A**: はい！コントリビューターは以下で認識されます：
- CHANGELOG.md
- コントリビューターセクション
- Gitコミット履歴

### Q: 間違えたらどうなりますか？

**A**: 心配ありません！すべての翻訳はレビューされます：
1. ネイティブスピーカーが翻訳をレビュー
2. メンテナーがアプリでテスト
3. ユーザーが問題を報告
4. 反復して改善

---

## 追加リソース

- **[コントリビューションガイド](../../../../CONTRIBUTING.md)**
- **[テストガイド](testing-guide.md)**
- **[インストールガイド](../../../user/ja/installation.md)**
- **[翻訳リクエストを提出](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=translation.yml)**

---

## サポートが必要ですか？

- **GitHub Issues**: [翻訳ディスカッション](https://github.com/BonoJovi/KakeiBonByRust/issues?q=label%3Atranslation)
- **Email**: [bonojovi@zundou.org](mailto:bonojovi@zundou.org)

---

**KakeiBonを世界中のユーザーに届けるお手伝いをしていただき、ありがとうございます！**

翻訳に貢献していただくあなたの時間と努力に深く感謝します。

**- KakeiBonチーム**
