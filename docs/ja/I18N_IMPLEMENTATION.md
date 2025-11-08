# 多言語対応システム (I18N) - 実装ドキュメント

## 概要
データベースベースの多言語対応システムを実装しました。システムリソース（メニュー、メッセージ、ラベル）と費目（カテゴリ）の多言語化に対応しています。

## 実装日
2024-10-24

## データベーステーブル

### 1. I18N_RESOURCES（システムリソース）
システム全体の多言語リソースを管理するテーブル。

```sql
CREATE TABLE I18N_RESOURCES (
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

**用途**: メニュー、メッセージ、ラベルなどのUI要素
**例**: 
- `menu.file` → "File" (en), "ファイル" (ja)
- `msg.lang_changed` → "Language changed to {0}." (en), "言語を{0}に変更しました。" (ja)

### 2. CATEGORY1/2/3（費目マスタ）
階層構造を持つ費目（カテゴリ）を管理するテーブル。

```sql
CREATE TABLE CATEGORY1 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY1_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE)
);

CREATE TABLE CATEGORY2 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY2_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE) REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE)
);

CREATE TABLE CATEGORY3 (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    CATEGORY2_CODE VARCHAR(64) NOT NULL,
    CATEGORY3_CODE VARCHAR(64) NOT NULL,
    DISPLAY_ORDER INTEGER NOT NULL,
    CATEGORY3_NAME VARCHAR(128) NOT NULL,
    IS_DISABLED INTEGER NOT NULL DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, CATEGORY3_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE) 
        REFERENCES CATEGORY2(USER_ID, CATEGORY1_CODE, CATEGORY2_CODE)
);
```

**設計の特徴**:
- **CODE識別**: `CATEGORY_CODE`で可読性の高い識別（例: "FOOD", "TRANSPORT"）
- **ユーザ別管理**: 各ユーザが独自の費目を持てる
- **階層構造**: CATEGORY1（大分類）→ CATEGORY2（中分類）→ CATEGORY3（小分類）
- **表示順序**: `DISPLAY_ORDER`で並び順を制御
- **無効化フラグ**: `IS_DISABLED`で論理削除

### 3. CATEGORY1/2/3_I18N（費目多言語）
費目の多言語名称を管理するテーブル。

```sql
CREATE TABLE CATEGORY1_I18N (
    USER_ID INTEGER NOT NULL,
    CATEGORY1_CODE VARCHAR(64) NOT NULL,
    LANG_CODE VARCHAR(10) NOT NULL,
    CATEGORY1_NAME_I18N VARCHAR(256) NOT NULL,
    ENTRY_DT DATETIME NOT NULL,
    UPDATE_DT DATETIME,
    PRIMARY KEY(USER_ID, CATEGORY1_CODE, LANG_CODE),
    FOREIGN KEY(USER_ID, CATEGORY1_CODE) REFERENCES CATEGORY1(USER_ID, CATEGORY1_CODE)
);
```

**特徴**:
- 各費目レベルごとに独立したI18Nテーブル
- データベースサイズの最適化（正規化）
- メンテナンス性の向上

## サービス層

### I18nService (`src/services/i18n.rs`)

多言語リソースを管理するサービス。

#### 主要メソッド

##### `get(key: &str, lang_code: &str) -> Result<String, I18nError>`
リソースキーと言語コードからリソース値を取得。
- 指定言語が見つからない場合、デフォルト言語（ja）にフォールバック

##### `get_with_params(key: &str, lang_code: &str, params: &[&str]) -> Result<String, I18nError>`
パラメータ置換付きでリソースを取得。
```rust
// "言語を{0}に変更しました。" → "言語を日本語に変更しました。"
i18n.get_with_params("msg.lang_changed", "ja", &["日本語"]).await
```

##### `get_all(lang_code: &str) -> Result<HashMap<String, String>, I18nError>`
特定言語の全リソースを取得。

##### `get_by_category(lang_code: &str, category: &str) -> Result<HashMap<String, String>, I18nError>`
カテゴリ別にリソースを取得（例: "menu", "message"）。

### CategoryService (`src/services/category.rs`)

費目（カテゴリ）を管理するサービス。

#### 主要メソッド

##### `initialize_user_categories(user_id: i64) -> Result<(), CategoryError>`
新規ユーザ登録時にデフォルト費目を初期化。
- 既に初期化済みの場合はスキップ
- トランザクション処理で整合性を保証

##### `get_category1_list(user_id: i64, lang_code: &str) -> Result<Vec<Category1>, CategoryError>`
大分類（CATEGORY1）の一覧を多言語で取得。
- I18Nテーブルと結合して翻訳名を取得
- 翻訳がない場合はデフォルト名にフォールバック

## Tauriコマンド

### 言語設定関連

#### `set_language(language: String) -> Result<String, String>`
言語を設定し、確認メッセージを返す。

**引数**:
- `language`: "English", "en", "日本語", "Japanese", "ja"

**戻り値**: 選択した言語での確認メッセージ
- 英語: "Language changed to English."
- 日本語: "言語を日本語に変更しました。"

**処理フロー**:
1. 言語コードを正規化（en/ja）
2. KakeiBon.jsonに保存
3. データベースから言語名を取得
4. 確認メッセージをパラメータ置換して返す

#### `get_language() -> Result<String, String>`
現在の言語設定を取得。

**デフォルト**: "ja"（日本語）

### リソース取得関連

#### `get_i18n_resource(key: String) -> Result<String, String>`
リソースキーからリソース値を取得。
- 現在の言語設定を自動的に使用

**例**:
```javascript
const fileMenu = await invoke('get_i18n_resource', { key: 'menu.file' });
// 日本語設定の場合: "ファイル"
// 英語設定の場合: "File"
```

#### `get_i18n_resources_by_category(category: String) -> Result<HashMap<String, String>, String>`
カテゴリ別に複数のリソースを一括取得。

**例**:
```javascript
const menuResources = await invoke('get_i18n_resources_by_category', { category: 'menu' });
// { "menu.file": "ファイル", "menu.settings": "設定", ... }
```

## デフォルトリソース

### メニュー
| リソースキー | 英語 | 日本語 |
|-------------|------|--------|
| menu.file | File | ファイル |
| menu.settings | Settings | 設定 |
| menu.language | Language | 言語 |
| menu.quit | Quit | 終了 |

### 言語オプション
| リソースキー | 英語 | 日本語 |
|-------------|------|--------|
| lang.english | English | English |
| lang.japanese | 日本語 (Japanese) | 日本語 |
| lang.name.en | English | 英語 |
| lang.name.ja | Japanese | 日本語 |

### メッセージ
| リソースキー | 英語 | 日本語 |
|-------------|------|--------|
| msg.lang_changed | Language changed to {0}. | 言語を{0}に変更しました。 |
| msg.error | Error | エラー |
| msg.success | Success | 成功 |
| msg.info | Information | 情報 |

## テスト

### I18nServiceテスト
- ✅ `test_get_resource`: リソース取得
- ✅ `test_get_with_params`: パラメータ置換
- ✅ `test_fallback_to_default`: デフォルト言語へのフォールバック
- ✅ `test_get_by_category`: カテゴリ別取得

### CategoryServiceテスト
- ✅ `test_initialize_user_categories`: ユーザ費目初期化
- ✅ `test_get_category1_list`: 多言語費目一覧取得

### 総合テスト結果
```
テスト総数: 90
成功: 90
失敗: 0
成功率: 100%
```

## 使用例

### フロントエンド（JavaScript）

#### 言語切り替え
```javascript
// 言語を日本語に変更
const message = await invoke('set_language', { language: '日本語' });
alert(message); // "言語を日本語に変更しました。"

// 言語を英語に変更
const message = await invoke('set_language', { language: 'English' });
alert(message); // "Language changed to English."
```

#### リソース取得
```javascript
// 単一リソース取得
const fileMenuLabel = await invoke('get_i18n_resource', { key: 'menu.file' });

// カテゴリ別一括取得
const menuResources = await invoke('get_i18n_resources_by_category', { category: 'menu' });
console.log(menuResources['menu.file']);
console.log(menuResources['menu.settings']);
```

### バックエンド（Rust）

```rust
use crate::services::i18n::I18nService;

let i18n = I18nService::new(pool);

// リソース取得
let menu_file = i18n.get("menu.file", "ja").await?;

// パラメータ置換
let message = i18n.get_with_params(
    "msg.lang_changed", 
    "ja", 
    &["日本語"]
).await?;
// "言語を日本語に変更しました。"
```

## バックエンドAPI仕様

### `get_translations` コマンド

全翻訳リソースを一括取得するTauriコマンド。

#### 関数シグネチャ
```rust
#[tauri::command]
pub fn get_translations(language: String) -> Result<HashMap<String, String>, String>
```

#### パラメータ
- **language** (String): 言語コード
  - 有効な値: `"ja"` (日本語), `"en"` (英語)
  - 大文字小文字を区別しない

#### 戻り値
- **成功時**: `HashMap<String, String>`
  - キー: リソースキー（例: `"menu.file"`, `"common.save"`）
  - 値: 翻訳されたテキスト（例: `"ファイル"`, `"保存"`）
- **失敗時**: `String` - エラーメッセージ

#### データベースクエリ
```sql
SELECT RESOURCE_KEY, RESOURCE_VALUE 
FROM I18N_RESOURCES 
WHERE LANG_CODE = ?1
```

#### 実装詳細
1. データベースパスを取得（`$HOME/.kakeibon/KakeiBonDB.sqlite3`）
2. SQLite接続を開く
3. 指定言語のすべてのリソースを取得
4. HashMapに変換して返す

#### エラーハンドリング

**エラーケース**:
1. **データベース接続エラー**
   - ファイルが存在しない
   - アクセス権限がない
   - ファイルが破損している
   ```
   "Failed to get translations: unable to open database file"
   ```

2. **クエリ実行エラー**
   - テーブルが存在しない
   - カラム定義が不正
   ```
   "Failed to get translations: no such table: I18N_RESOURCES"
   ```

3. **データ変換エラー**
   - データ型の不一致
   ```
   "Failed to get translations: Invalid column type"
   ```

**エラーハンドリング方針**:
- すべてのエラーは文字列メッセージとして返される
- フロントエンドでキャッチして適切に処理する
- エラー時は空のHashMapまたはフォールバック動作を推奨

#### パフォーマンス考慮事項

**初回ロード**:
- アプリケーション起動時に1回だけ呼び出す
- 全リソースを一括取得（典型的には100～500エントリ）
- 取得時間: 通常10～50ms

**キャッシング戦略**:
- フロントエンドでメモリにキャッシュする（`i18n.translations`）
- 言語変更時のみ再取得
- ページリロード時は再取得が必要

**最適化のヒント**:
- 頻繁に使用するリソースはローカル変数に保存
- 大量のDOM更新時は`requestAnimationFrame`を使用
- 言語変更時はバッチ更新を行う

### フロントエンドからの呼び出し例

#### 基本的な使用方法

```javascript
import { invoke } from '@tauri-apps/api/core';

// 日本語の翻訳を取得
const translations = await invoke('get_translations', { 
    language: 'ja' 
});

console.log(translations);
// {
//   "menu.file": "ファイル",
//   "menu.settings": "設定",
//   "common.save": "保存",
//   ...
// }

// 特定のリソースにアクセス
const fileMenu = translations['menu.file']; // "ファイル"
```

#### I18nクラスでの統合

```javascript
class I18n {
    constructor() {
        this.currentLanguage = 'ja';
        this.translations = {};
    }

    async loadTranslations() {
        try {
            const translations = await invoke('get_translations', { 
                language: this.currentLanguage 
            });
            this.translations = translations;
        } catch (error) {
            console.error('Failed to load translations:', error);
            this.translations = {};
        }
    }

    t(key, params = {}) {
        let text = this.translations[key] || key;
        
        // パラメータ置換
        Object.keys(params).forEach(paramKey => {
            text = text.replace(
                new RegExp(`{${paramKey}}`, 'g'), 
                params[paramKey]
            );
        });
        
        return text;
    }
}
```

#### エラーハンドリングの例

```javascript
async function switchLanguage(newLang) {
    try {
        // 翻訳を取得
        const translations = await invoke('get_translations', { 
            language: newLang 
        });
        
        // 成功: UIを更新
        updateAllUIElements(translations);
        
    } catch (error) {
        // エラー: フォールバック処理
        console.error('Translation load failed:', error);
        
        // デフォルト言語で再試行
        if (newLang !== 'ja') {
            const fallbackTranslations = await invoke('get_translations', { 
                language: 'ja' 
            });
            updateAllUIElements(fallbackTranslations);
        }
        
        // ユーザーに通知
        showErrorMessage('言語の読み込みに失敗しました');
    }
}
```

#### パラメータ置換の例

```javascript
// データベース: "msg.welcome" = "ようこそ、{name}さん！"
const welcomeMsg = i18n.t('msg.welcome', { name: 'ボノ' });
// 結果: "ようこそ、ボノさん！"

// 複数パラメータ
// データベース: "msg.date_range" = "{start}から{end}まで"
const dateMsg = i18n.t('msg.date_range', { 
    start: '2024-10-01', 
    end: '2024-10-31' 
});
// 結果: "2024-10-01から2024-10-31まで"
```

#### DOM要素の自動更新

```html
<!-- HTML -->
<button data-i18n="common.save">保存</button>
<input data-i18n-placeholder="common.search" placeholder="検索">
<div data-i18n-title="common.help" title="ヘルプ">?</div>
```

```javascript
// 言語変更時にすべての要素を自動更新
function updateUI() {
    // data-i18n属性を持つ要素
    document.querySelectorAll('[data-i18n]').forEach(element => {
        const key = element.getAttribute('data-i18n');
        element.textContent = i18n.t(key);
    });

    // data-i18n-placeholder属性を持つ要素
    document.querySelectorAll('[data-i18n-placeholder]').forEach(element => {
        const key = element.getAttribute('data-i18n-placeholder');
        element.placeholder = i18n.t(key);
    });

    // data-i18n-title属性を持つ要素
    document.querySelectorAll('[data-i18n-title]').forEach(element => {
        const key = element.getAttribute('data-i18n-title');
        element.title = i18n.t(key);
    });
}
```

## 次のステップ（未実装）

### 1. メニュー実装
- File > Settings > Language メニューの追加
- 言語選択サブメニュー（English / 日本語）

### 2. 言語変更ダイアログ
- 言語変更確認ダイアログの実装
- メッセージ表示

### 3. カテゴリデータ移行
- 既存のSQLから費目データを移行
- CATEGORY1/2/3テーブルへのデータ投入
- CATEGORY1/2/3_I18Nテーブルへの翻訳データ投入

### 4. 動的メニュー更新
- 言語変更時にメニューを動的に更新
- アプリケーション全体の再描画

## ファイル構成

```
src/
  ├── services/
  │   ├── i18n.rs          # 多言語リソース管理
  │   └── category.rs      # 費目管理
  ├── lib.rs               # Tauriコマンド定義
  └── ...

res/
  └── sql/
      └── dbaccess.sql     # データベーススキーマ + 初期データ

$HOME/.kakeibon/
  ├── KakeiBonDB.sqlite3   # データベース
  └── KakeiBon.json        # ユーザ設定（言語設定を含む）
```

## まとめ

多言語対応システムのバックエンド実装が完了しました：

✅ データベーステーブル設計完了
✅ サービス層実装完了
✅ Tauriコマンド実装完了
✅ テスト実装完了（90/90成功）
✅ ドキュメント作成完了

次のフェーズでフロントエンド（メニューとダイアログ）を実装予定です。

---

**作成者**: AI Assistant  
**監修**: Yoshihiro NAKAHARA (bonojovi@zundou.org)
