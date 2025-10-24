# Dynamic Language Menu Implementation

## 概要 (Overview)

言語選択メニューを、固定のハードコードから、データベースに登録された言語を動的に読み込む方式に変更しました。
言語コード順でソートされるため、言語を切り替えても順序が一定に保たれます。

Changed the language selection menu from hardcoded options to dynamically loading languages from the database.
Languages are sorted by language code to maintain consistent order.

## 変更内容 (Changes Made)

### 1. Backend Changes (Rust)

#### src/lib.rs

新しいTauriコマンドを2つ追加:

Added two new Tauri commands:

1. **`get_available_languages`** - データベースから利用可能な言語コードのリストを取得
   - Gets list of available language codes from database
   
2. **`get_language_names`** - 現在の言語で翻訳された言語名の配列を取得（言語コード順でソート）
   - Gets array of language codes to their localized names in the current language (sorted by language code)

These commands leverage the existing `I18nService::get_available_languages()` method which queries:
```sql
SELECT DISTINCT LANG_CODE FROM I18N_RESOURCES ORDER BY LANG_CODE
```

### 2. Frontend Changes (HTML/JavaScript)

#### res/index.html

メニューバーに言語選択ドロップダウンを追加:

Added language selection dropdown to menu bar:

```html
<div id="language-menu" class="menu-item">
    <span data-i18n="menu.language">Language</span>
    <div id="language-dropdown" class="dropdown">
        <!-- Language options will be populated dynamically -->
    </div>
</div>
```

#### res/js/menu.js

新しい関数を追加:

Added new functions:

1. **`setupLanguageMenu()`** - データベースから言語リストを取得し、動的にメニュー項目を生成
   - Fetches language list from database and dynamically generates menu items
   - Marks current language with bold font
   - Sets up event listeners for language switching

2. **`handleLanguageChange(langCode)`** - 言語変更を処理
   - Handles language change when user selects a language
   - Updates UI with new translations
   - Refreshes language menu to show updated names

初期化時に `setupLanguageMenu()` を呼び出すように変更:
Modified initialization to call `setupLanguageMenu()` during startup

## 利点 (Benefits)

1. **保守性の向上** - 言語を追加する際、データベースにデータを追加するだけで済む
   - Better maintainability - just add data to database to support new languages

2. **バグの削減** - ハードコードされた言語リストを複数箇所で管理する必要がなくなる
   - Fewer bugs - no need to maintain hardcoded language lists in multiple places

3. **一貫性** - すべての言語データが一箇所（データベース）で管理される
   - Consistency - all language data managed in one place (database)

4. **拡張性** - 新しい言語を追加する際のコード変更が不要
   - Extensibility - no code changes needed to add new languages

5. **順序の一貫性** - 言語コード順でソートされるため、言語切替時も順序が変わらない
   - Consistent ordering - languages sorted by code, order doesn't change when switching languages

## データベース構造 (Database Structure)

言語データは `I18N_RESOURCES` テーブルに格納されています:

Language data is stored in the `I18N_RESOURCES` table:

```sql
CREATE TABLE IF NOT EXISTS I18N_RESOURCES (
    RESOURCE_ID INTEGER NOT NULL,
    RESOURCE_KEY VARCHAR(256) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    RESOURCE_VALUE TEXT NOT NULL,
    CATEGORY VARCHAR(64),
    DESCRIPTION VARCHAR(512),
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(RESOURCE_ID),
    UNIQUE(RESOURCE_KEY, LANG_CODE)
);
```

### 言語名のリソースキー (Resource Keys for Language Names)

- `lang.name.en` - English language name in current language
- `lang.name.ja` - Japanese language name in current language

例 (Example):
- 英語で表示する場合: `lang.name.ja` → "Japanese"
- 日本語で表示する場合: `lang.name.ja` → "日本語"

## 新しい言語の追加方法 (How to Add New Languages)

1. `I18N_RESOURCES` テーブルに新しい言語のリソースを追加
   Add new language resources to `I18N_RESOURCES` table

2. すべてのリソースキー（menu.*, error.*, etc.）に対して新しい言語の翻訳を追加
   Add translations for all resource keys for the new language

3. `lang.name.{lang_code}` リソースを各言語で追加
   Add `lang.name.{lang_code}` resources in each language

4. アプリケーションを再起動すると、新しい言語が自動的にメニューに表示される
   Restart application and new language will automatically appear in menu

例: 中国語を追加する場合 (Example: Adding Chinese):

```sql
-- Language name resources
INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT)
VALUES 
(1001, 'lang.name.zh', 'en', 'Chinese', 'language', datetime('now')),
(1002, 'lang.name.zh', 'ja', '中国語', 'language', datetime('now')),
(1003, 'lang.name.zh', 'zh', '中文', 'language', datetime('now'));

-- Then add all other translations for zh language code
-- menu.file, menu.login, error messages, etc.
```

## テスト (Testing)

すべての既存のユニットテストが成功しています:
All existing unit tests pass:

```bash
cargo test --lib
# Result: ok. 90 passed; 0 failed; 0 ignored
```

## 互換性 (Compatibility)

既存の機能との後方互換性を保持しています。現在サポートされている言語:
Maintains backward compatibility with existing functionality. Currently supported languages:

- English (en)
- Japanese (ja)
