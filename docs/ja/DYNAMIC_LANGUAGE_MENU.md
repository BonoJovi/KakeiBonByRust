# 動的言語メニュー実装

## 概要

言語選択メニューを、固定のハードコードから、データベースに登録された言語を動的に読み込む方式に変更しました。
言語コード順でソートされるため、言語を切り替えても順序が一定に保たれます。

## 変更内容

### 1. バックエンドの変更（Rust）

#### src/lib.rs

新しいTauriコマンドを2つ追加:

1. **`get_available_languages`** - データベースから利用可能な言語コードのリストを取得
   
2. **`get_language_names`** - 現在の言語で翻訳された言語名の配列を取得（言語コード順でソート）

これらのコマンドは既存の `I18nService::get_available_languages()` メソッドを活用します：

```sql
SELECT DISTINCT LANG_CODE FROM I18N_RESOURCES ORDER BY LANG_CODE
```

### 2. フロントエンドの変更（HTML/JavaScript）

#### res/index.html

メニューバーに言語選択ドロップダウンを追加:

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

1. **`setupLanguageMenu()`** - データベースから言語リストを取得し、動的にメニュー項目を生成
   - 現在の言語をマークする（緑の塗りつぶしの丸）
   - 言語切り替えのイベントリスナーを設定

2. **`handleLanguageChange(langCode)`** - 言語変更を処理
   - UIを新しい翻訳で更新
   - 言語メニューを更新して名前を表示

初期化時に `setupLanguageMenu()` を呼び出すように変更

## 利点

1. **保守性の向上** - 言語を追加する際、データベースにデータを追加するだけで済む

2. **バグの削減** - ハードコードされた言語リストを複数箇所で管理する必要がなくなる

3. **一貫性** - すべての言語データが一箇所（データベース）で管理される

4. **拡張性** - 新しい言語を追加する際のコード変更が不要

5. **順序の一貫性** - 言語コード順でソートされるため、言語切替時も順序が変わらない

## データベース構造

言語データは `I18N_RESOURCES` テーブルに格納されています:

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

### 言語名のリソースキー

- `lang.name.en` - 英語の言語名（現在の言語で表示）
- `lang.name.ja` - 日本語の言語名（現在の言語で表示）

例:
- 英語で表示する場合: `lang.name.ja` → "Japanese"
- 日本語で表示する場合: `lang.name.ja` → "日本語"

## 新しい言語の追加方法

1. `I18N_RESOURCES` テーブルに新しい言語のリソースを追加

2. すべてのリソースキー（menu.*, error.*, etc.）に対して新しい言語の翻訳を追加

3. `lang.name.{lang_code}` リソースを各言語で追加

4. アプリケーションを再起動すると、新しい言語が自動的にメニューに表示される

### 例: 中国語を追加する場合

```sql
-- 言語名のリソース
INSERT INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, ENTRY_DT)
VALUES 
(1001, 'lang.name.zh', 'en', 'Chinese', 'language', datetime('now')),
(1002, 'lang.name.zh', 'ja', '中国語', 'language', datetime('now')),
(1003, 'lang.name.zh', 'zh', '中文', 'language', datetime('now'));

-- その他すべての翻訳を zh 言語コードで追加
-- menu.file, menu.login, エラーメッセージなど
```

## テスト

すべての既存のユニットテストが成功しています:

```bash
cargo test --lib
# 結果: ok. 90 passed; 0 failed; 0 ignored
```

## 互換性

既存の機能との後方互換性を保持しています。現在サポートされている言語:

- English (en)
- Japanese (ja)

## 実装ファイル

### 変更したファイル

1. **src/lib.rs**
   - 新しいTauriコマンドの追加
   - `get_available_languages()` - 利用可能な言語のリスト
   - `get_language_names()` - ローカライズされた言語名

2. **res/index.html**
   - 言語ドロップダウンメニューの追加

3. **res/js/menu.js**
   - 動的言語メニューの実装
   - `setupLanguageMenu()` - メニュー項目の生成
   - `setupLanguageMenuHandlers()` - イベントハンドラーの設定
   - `handleLanguageChange()` - 言語変更の処理

4. **res/css/menu.css**
   - 言語メニューのスタイル

## 技術的詳細

### 言語の順序保証

バックエンドで `Vec<(String, String)>` として返すことで、順序を保証：

```rust
let mut language_names = Vec::new();
for lang_code in lang_codes {
    let key = format!("lang.name.{}", lang_code);
    if let Ok(name) = i18n.get(&key, &current_lang).await {
        language_names.push((lang_code, name));
    }
}

// 言語コードでソート
language_names.sort_by(|a, b| a.0.cmp(&b.0));
```

フロントエンドでは配列として受け取り、順序を保持：

```javascript
for (const [langCode, langName] of languageNames) {
    // 配列の順序がそのまま保持される
}
```

## 今後の展開

この実装により、以下が可能になります：

- データベースに言語データを追加するだけで新しい言語をサポート
- コードの変更なしに言語を追加・削除
- 翻訳の管理をデータベースで一元化
- 翻訳管理ツールとの統合が容易

## 変更履歴

### 2024-10-24
- 初版作成
- 動的言語メニューの実装
- 言語順序のソート機能追加
