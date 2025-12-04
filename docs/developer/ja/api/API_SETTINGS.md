# 設定API リファレンス

**最終更新**: 2025-12-05 02:20 JST

## 概要

本ドキュメントは、設定画面（settings.html）で使用されるAPIの仕様を定義します。アプリケーション設定の取得・更新、言語切り替え、フォントサイズ変更などの機能を提供します。

---

## 目次

1. [汎用設定API](#汎用設定api)
2. [言語設定API](#言語設定api)
3. [フォントサイズ設定API](#フォントサイズ設定api)
4. [ウィンドウ調整API](#ウィンドウ調整api)
5. [データ構造](#データ構造)

---

## 汎用設定API

### get_setting

設定値を取得します（JSON形式）。

**パラメータ:**
- `key` (String): 設定キー

**戻り値:**
- `Option<serde_json::Value>`: 設定値（存在しない場合はnull）

**使用例:**
```javascript
const value = await invoke('get_setting', { key: 'language' });
if (value) {
    console.log(`Language: ${value}`);
}
```

---

### get_setting_string

文字列型の設定値を取得します。

**パラメータ:**
- `key` (String): 設定キー

**戻り値:**
- `String`: 設定値

**使用例:**
```javascript
const language = await invoke('get_setting_string', { key: 'language' });
```

**エラー:**
- `"Failed to get setting: ..."`: キーが存在しないまたは型不一致

---

### get_setting_int

整数型の設定値を取得します。

**パラメータ:**
- `key` (String): 設定キー

**戻り値:**
- `i64`: 設定値

**使用例:**
```javascript
const maxItems = await invoke('get_setting_int', { key: 'max_items' });
```

---

### get_setting_bool

真偽値型の設定値を取得します。

**パラメータ:**
- `key` (String): 設定キー

**戻り値:**
- `bool`: 設定値

**使用例:**
```javascript
const darkMode = await invoke('get_setting_bool', { key: 'dark_mode' });
```

---

### set_setting

設定値を保存します。

**パラメータ:**
- `key` (String): 設定キー
- `value` (serde_json::Value): 設定値（JSON）

**戻り値:** なし

**使用例:**
```javascript
await invoke('set_setting', {
    key: 'theme',
    value: 'dark'
});

// 数値の設定
await invoke('set_setting', {
    key: 'max_items',
    value: 100
});

// 真偽値の設定
await invoke('set_setting', {
    key: 'auto_save',
    value: true
});
```

**自動処理:**
- 設定ファイルに自動保存

**エラー:**
- `"Failed to set setting: ..."`: 設定エラー
- `"Failed to save settings: ..."`: ファイル保存エラー

---

### remove_setting

設定を削除します。

**パラメータ:**
- `key` (String): 設定キー

**戻り値:**
- `bool`: `true` - 削除成功、`false` - キーが存在しない

**使用例:**
```javascript
const removed = await invoke('remove_setting', { key: 'old_setting' });
if (removed) {
    console.log('設定を削除しました');
}
```

---

### list_setting_keys

すべての設定キーを取得します。

**パラメータ:** なし

**戻り値:**
- `Vec<String>`: 設定キーの配列

**使用例:**
```javascript
const keys = await invoke('list_setting_keys');
console.log('設定キー:', keys);
```

---

### reload_settings

設定をファイルから再読み込みします。

**パラメータ:** なし

**戻り値:** なし

**使用例:**
```javascript
await invoke('reload_settings');
console.log('設定を再読み込みしました');
```

**用途:**
- 外部で設定ファイルを編集した場合
- 設定の初期化

---

## 言語設定API

### set_language

言語を設定します。

**パラメータ:**
- `language` (String): 言語コードまたは言語名

**戻り値:**
- `String`: 成功メッセージ（多言語）

**使用例:**
```javascript
// 言語コードで指定
await invoke('set_language', { language: 'ja' });

// 言語名で指定
await invoke('set_language', { language: '日本語' });
await invoke('set_language', { language: 'English' });
```

**サポート言語:**
- `"ja"`, `"Japanese"`, `"日本語"` → 日本語
- `"en"`, `"English"` → 英語

**自動処理:**
1. 言語コードの正規化
2. バリデーション
3. 設定ファイルに保存

**エラー:**
- `"Unsupported language: ..."`: サポートされていない言語

---

### get_language

現在の言語設定を取得します。

**パラメータ:** なし

**戻り値:**
- `String`: 言語コード（"ja" or "en"）

**使用例:**
```javascript
const lang = await invoke('get_language');
console.log(`現在の言語: ${lang}`);
```

**デフォルト:**
- 設定がない場合は`"ja"`（LANG_DEFAULT）

---

### get_available_languages

利用可能な言語の一覧を取得します。

**パラメータ:** なし

**戻り値:**
- `Vec<String>`: 言語コードの配列

**使用例:**
```javascript
const languages = await invoke('get_available_languages');
console.log('利用可能な言語:', languages);
// ["ja", "en"]
```

---

### get_language_names

言語コードと表示名のペアを取得します。

**パラメータ:** なし

**戻り値:**
- `Vec<(String, String)>`: (言語コード, 表示名)の配列

**使用例:**
```javascript
const languageNames = await invoke('get_language_names');

// 言語選択のプルダウンを作成
languageNames.forEach(([code, name]) => {
    const option = document.createElement('option');
    option.value = code;
    option.textContent = name;
    selectElement.appendChild(option);
});

// 例: [["ja", "日本語"], ["en", "English"]]
```

**注意:**
- 現在の言語設定で表示名を取得
- I18Nテーブルから`lang.name.{code}`キーで取得

---

## フォントサイズ設定API

### set_font_size

フォントサイズを設定します。

**パラメータ:**
- `font_size` (String): フォントサイズ（"small", "medium", "large"）

**戻り値:**
- `String`: 成功メッセージ（多言語）

**使用例:**
```javascript
await invoke('set_font_size', { fontSize: 'large' });
```

**サポートサイズ:**
- `"small"`: 小
- `"medium"`: 中（デフォルト）
- `"large"`: 大

**自動処理:**
1. サイズのバリデーション
2. 設定ファイルに保存

**エラー:**
- `"Unsupported font size: ..."`: サポートされていないサイズ

---

### get_font_size

現在のフォントサイズ設定を取得します。

**パラメータ:** なし

**戻り値:**
- `String`: フォントサイズ（"small", "medium", "large"）

**使用例:**
```javascript
const fontSize = await invoke('get_font_size');
console.log(`フォントサイズ: ${fontSize}`);
```

**デフォルト:**
- 設定がない場合は`"medium"`（FONT_SIZE_DEFAULT）

---

## ウィンドウ調整API

### adjust_window_size

ウィンドウサイズを調整します。

**パラメータ:**
- `width` (f64): 幅（ピクセル）
- `height` (f64): 高さ（ピクセル）

**戻り値:** なし

**使用例:**
```javascript
// ウィンドウを1280x720にリサイズ
await invoke('adjust_window_size', {
    width: 1280,
    height: 720
});
```

**動作:**
- 現在のウィンドウサイズから相対的に調整
- 最小サイズ・最大サイズの制約あり（OS依存）

**エラー:**
- `"Failed to get window size: ..."`: ウィンドウサイズ取得エラー
- `"Failed to set window size: ..."`: ウィンドウサイズ設定エラー

---

## データ構造

### 設定ファイル形式

設定はJSON形式でファイルに保存されます。

**デフォルト保存場所:**
- Windows: `%APPDATA%\KakeiBon\settings.json`
- macOS: `~/Library/Application Support/KakeiBon/settings.json`
- Linux: `~/.config/KakeiBon/settings.json`

**設定ファイル例:**
```json
{
    "language": "ja",
    "font_size": "medium",
    "theme": "light",
    "auto_save": true,
    "max_items": 100
}
```

---

### 標準設定キー

| キー | 型 | デフォルト値 | 説明 |
|------|---|-------------|------|
| `language` | String | "ja" | 言語設定 |
| `font_size` | String | "medium" | フォントサイズ |
| `theme` | String | - | テーマ（未実装） |
| `auto_save` | bool | - | 自動保存（未実装） |

---

## エラーハンドリング

### 共通エラーパターン

| エラーメッセージ | 原因 | 対処方法 |
|----------------|------|---------|
| `"Failed to get setting: ..."` | キーが存在しないまたは型不一致 | キーを確認、または別のget関数を使用 |
| `"Failed to set setting: ..."` | 設定エラー | 設定値を確認 |
| `"Failed to save settings: ..."` | ファイル保存エラー | ディスク容量・権限を確認 |
| `"Failed to reload settings: ..."` | ファイル読み込みエラー | 設定ファイルの形式を確認 |
| `"Unsupported language: ..."` | サポートされていない言語 | "ja"または"en"を指定 |
| `"Unsupported font size: ..."` | サポートされていないサイズ | "small", "medium", "large"を指定 |

### フロントエンドでのエラーハンドリング例

```javascript
// 言語設定
async function changeLanguage(language) {
    try {
        const message = await invoke('set_language', { language });
        alert(message);
        
        // ページをリロードして新しい言語を反映
        location.reload();
    } catch (error) {
        if (error.includes('Unsupported language')) {
            alert('サポートされていない言語です');
        } else {
            alert(`エラー: ${error}`);
        }
    }
}

// フォントサイズ設定
async function changeFontSize(fontSize) {
    try {
        await invoke('set_font_size', { fontSize });
        
        // CSSを更新
        document.documentElement.style.fontSize = 
            fontSize === 'small' ? '14px' : 
            fontSize === 'large' ? '18px' : '16px';
        
        alert('フォントサイズを変更しました');
    } catch (error) {
        alert(`エラー: ${error}`);
    }
}
```

---

## 使用例：設定画面の実装

### 設定の読み込み

```javascript
async function loadSettings() {
    try {
        // 言語設定
        const language = await invoke('get_language');
        document.getElementById('language-select').value = language;
        
        // フォントサイズ設定
        const fontSize = await invoke('get_font_size');
        document.getElementById('font-size-select').value = fontSize;
        
        // すべての設定キーを表示（デバッグ用）
        const keys = await invoke('list_setting_keys');
        console.log('設定キー:', keys);
    } catch (error) {
        console.error('設定の読み込みエラー:', error);
    }
}

// ページ読み込み時に実行
document.addEventListener('DOMContentLoaded', loadSettings);
```

### 言語選択プルダウンの作成

```javascript
async function initializeLanguageSelect() {
    try {
        const languageNames = await invoke('get_language_names');
        const currentLang = await invoke('get_language');
        
        const select = document.getElementById('language-select');
        select.innerHTML = '';
        
        languageNames.forEach(([code, name]) => {
            const option = document.createElement('option');
            option.value = code;
            option.textContent = name;
            option.selected = (code === currentLang);
            select.appendChild(option);
        });
        
        // 変更イベント
        select.addEventListener('change', async (e) => {
            await changeLanguage(e.target.value);
        });
    } catch (error) {
        console.error('言語選択の初期化エラー:', error);
    }
}
```

### フォントサイズ変更

```javascript
async function handleFontSizeChange(event) {
    const fontSize = event.target.value;
    
    try {
        const message = await invoke('set_font_size', { fontSize });
        
        // CSSクラスを更新
        document.body.className = `font-${fontSize}`;
        
        alert(message);
    } catch (error) {
        alert(`エラー: ${error}`);
    }
}
```

### カスタム設定の保存

```javascript
async function saveCustomSettings() {
    try {
        // テーマ設定
        const theme = document.getElementById('theme-select').value;
        await invoke('set_setting', {
            key: 'theme',
            value: theme
        });
        
        // 自動保存設定
        const autoSave = document.getElementById('auto-save-checkbox').checked;
        await invoke('set_setting', {
            key: 'auto_save',
            value: autoSave
        });
        
        alert('設定を保存しました');
    } catch (error) {
        alert(`保存エラー: ${error}`);
    }
}
```

---

## 言語切り替えの仕組み

### フロー

```
1. ユーザーが言語を選択
   ↓
2. set_language() 呼び出し
   ↓
3. 言語コードの正規化
   ↓
4. バリデーション
   ↓
5. settings.jsonに保存
   ↓
6. ページリロード
   ↓
7. I18Nサービスが新しい言語を読み込み
```

### 実装例

```javascript
async function switchLanguage(newLang) {
    try {
        // 1. 言語を設定
        await invoke('set_language', { language: newLang });
        
        // 2. ページをリロード（新しい言語を反映）
        location.reload();
    } catch (error) {
        alert(`言語切り替えエラー: ${error}`);
    }
}

// または、リロードせずに動的に更新する場合
async function switchLanguageDynamic(newLang) {
    try {
        // 1. 言語を設定
        await invoke('set_language', { language: newLang });
        
        // 2. すべてのI18Nキーを再取得
        const translations = await invoke('get_translations', { language: newLang });
        
        // 3. UIのテキストを更新
        updateUITexts(translations);
        
    } catch (error) {
        alert(`言語切り替えエラー: ${error}`);
    }
}
```

---

## フォントサイズの適用

### CSS適用例

```css
/* デフォルト（medium） */
body {
    font-size: 16px;
}

/* Small */
body.font-small {
    font-size: 14px;
}

/* Large */
body.font-large {
    font-size: 18px;
}
```

### JavaScript適用例

```javascript
async function applyFontSize() {
    const fontSize = await invoke('get_font_size');
    
    // CSSクラスを適用
    document.body.className = `font-${fontSize}`;
    
    // または、スタイル直接適用
    const sizes = {
        small: '14px',
        medium: '16px',
        large: '18px'
    };
    document.documentElement.style.fontSize = sizes[fontSize];
}

// ページ読み込み時に適用
document.addEventListener('DOMContentLoaded', applyFontSize);
```

---

## セキュリティ考慮事項

### 設定ファイルの保護

1. **アクセス権限**: ユーザーのみアクセス可能
2. **バリデーション**: 設定値は必ずバリデーション
3. **デフォルト値**: 不正な値の場合はデフォルトを使用

### 入力検証

```javascript
// 不正な値を防止
async function setSafeSetting(key, value) {
    // バリデーション
    if (typeof value === 'string' && value.length > 1000) {
        throw new Error('設定値が長すぎます');
    }
    
    await invoke('set_setting', { key, value });
}
```

---

## テストカバレッジ

**SettingsService:**
- ✅ 設定の取得テスト（String, Int, Bool）
- ✅ 設定の保存テスト
- ✅ 設定の削除テスト
- ✅ 設定一覧取得テスト
- ✅ 設定のリロードテスト
- ✅ 言語切り替えテスト
- ✅ フォントサイズ変更テスト
- ✅ デフォルト値のフォールバックテスト

---

## 関連ドキュメント

### 実装ファイル

- 設定サービス: `src/services/settings.rs`
- I18Nサービス: `src/services/i18n.rs`
- Tauri Commands: `src/lib.rs`

### その他のAPIリファレンス

- [共通API](./API_COMMON.md) - I18n関連API
- [ユーザー管理API](./API_USER.md) - get_user_settings, update_user_settings

---

**変更履歴:**
- 2025-12-05: 初版作成（実装コードに基づく）
