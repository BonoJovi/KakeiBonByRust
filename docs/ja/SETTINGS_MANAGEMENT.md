# ユーザ設定管理システム

## 概要
ユーザカスタマイズ設定を`$HOME/.kakeibon/KakeiBon.json`ファイルに保存・管理するシステムです。

## 設定ファイル

### ファイルパス
```
$HOME/.kakeibon/KakeiBon.json
```
- Linuxの場合: `~/.kakeibon/KakeiBon.json`
- Windowsの場合: `%USERPROFILE%\.kakeibon\KakeiBon.json`

### ファイル形式
JSON形式でキー・バリュー形式のデータを保存します。

```json
{
  "language": "ja"
}
```

**注意**: 現在実装されている設定項目は`language`のみです。テーマやフォントサイズなどの設定項目は今後実装予定です。

## 実装ファイル

### `src/settings.rs`
設定管理サービスの実装

#### 主要構造体
- `UserSettings`: 設定データを保持
- `SettingsError`: エラー型
- `SettingsManager`: 設定管理サービス

#### 主要メソッド

##### `new() -> Result<Self, SettingsError>`
- SettingsManagerの新規作成
- 設定ファイルが存在しない場合は空のファイルを作成
- 自動的に設定ファイルを読み込み

##### `get(key: &str) -> Option<&serde_json::Value>`
- 指定したキーの設定値を取得（汎用型）
- 戻り値: `Option<&serde_json::Value>`

##### `get_string(key: &str) -> Result<String, SettingsError>`
- 文字列型の設定値を取得
- キーが存在しない場合はエラー

##### `get_int(key: &str) -> Result<i64, SettingsError>`
- 整数型の設定値を取得
- キーが存在しない場合はエラー

##### `get_bool(key: &str) -> Result<bool, SettingsError>`
- 真偽値型の設定値を取得
- キーが存在しない場合はエラー

##### `get_as<T>(key: &str) -> Result<T, SettingsError>`
- 任意の型の設定値を取得
- ジェネリック型対応

##### `set<T>(key: &str, value: T) -> Result<(), SettingsError>`
- 設定値を設定
- 任意の型をサポート（Serialize可能な型）

##### `remove(key: &str) -> Option<serde_json::Value>`
- 設定を削除
- 削除された値を返す

##### `save() -> Result<(), SettingsError>`
- メモリ上の設定をファイルに保存

##### `reload() -> Result<(), SettingsError>`
- ファイルから設定を再読み込み

##### `contains_key(key: &str) -> bool`
- キーの存在を確認

##### `keys() -> Vec<String>`
- 全キーのリストを取得

## Tauriコマンド

### `get_setting(key: String) -> Result<Option<serde_json::Value>, String>`
- 汎用的な設定値取得
- フロントエンド呼び出し: `invoke('get_setting', { key: 'theme' })`

### `get_setting_string(key: String) -> Result<String, String>`
- 文字列型の設定値取得
- フロントエンド呼び出し: `invoke('get_setting_string', { key: 'theme' })`

### `get_setting_int(key: String) -> Result<i64, String>`
- 整数型の設定値取得
- フロントエンド呼び出し: `invoke('get_setting_int', { key: 'window_width' })`

### `get_setting_bool(key: String) -> Result<bool, String>`
- 真偽値型の設定値取得
- フロントエンド呼び出し: `invoke('get_setting_bool', { key: 'auto_save' })`

### `set_setting(key: String, value: serde_json::Value) -> Result<(), String>`
- 設定値を設定して保存
- フロントエンド呼び出し: `invoke('set_setting', { key: 'theme', value: 'dark' })`

### `remove_setting(key: String) -> Result<bool, String>`
- 設定を削除
- 戻り値: 削除されたかどうか
- フロントエンド呼び出し: `invoke('remove_setting', { key: 'old_key' })`

### `list_setting_keys() -> Result<Vec<String>, String>`
- 全キーのリストを取得
- フロントエンド呼び出し: `invoke('list_setting_keys')`

### `reload_settings() -> Result<(), String>`
- 設定をファイルから再読み込み
- フロントエンド呼び出し: `invoke('reload_settings')`

## 使用例

### フロントエンド（JavaScript）
```javascript
// 設定を取得
const theme = await invoke('get_setting_string', { key: 'theme' });
console.log('Current theme:', theme);

// 設定を保存
await invoke('set_setting', { key: 'theme', value: 'dark' });
await invoke('set_setting', { key: 'window_width', value: 1920 });
await invoke('set_setting', { key: 'auto_save', value: true });

// 複雑なオブジェクトも保存可能
await invoke('set_setting', {
  key: 'window',
  value: {
    width: 1920,
    height: 1080,
    maximized: false
  }
});

// 全キーを取得
const keys = await invoke('list_setting_keys');
console.log('All setting keys:', keys);

// 設定を削除
await invoke('remove_setting', { key: 'old_setting' });

// 設定を再読み込み
await invoke('reload_settings');
```

### Rust（バックエンド）
```rust
use crate::settings::SettingsManager;

// 設定マネージャーを作成
let mut settings = SettingsManager::new()?;

// 設定を取得
let theme = settings.get_string("theme")?;
let width = settings.get_int("window_width")?;
let auto_save = settings.get_bool("auto_save")?;

// 設定を設定
settings.set("theme", "dark")?;
settings.set("window_width", 1920)?;
settings.set("auto_save", true)?;

// 複雑な型も対応
#[derive(Serialize, Deserialize)]
struct WindowSettings {
    width: i32,
    height: i32,
    maximized: bool,
}

let window = WindowSettings {
    width: 1920,
    height: 1080,
    maximized: false,
};
settings.set("window", &window)?;

// 保存
settings.save()?;

// 再読み込み
settings.reload()?;
```

## エラーハンドリング

### SettingsError
- `IoError`: ファイルI/Oエラー
- `JsonError`: JSON解析エラー
- `EntryNotFound`: キーが存在しない

## テスト

### 実装されたテスト（9件、全てパス）
1. `test_settings_manager_creation`: SettingsManager作成
2. `test_get_and_set_string`: 文字列の取得・設定
3. `test_get_and_set_int`: 整数の取得・設定
4. `test_get_and_set_bool`: 真偽値の取得・設定
5. `test_save_and_reload`: 保存と再読み込み
6. `test_remove_entry`: エントリ削除
7. `test_entry_not_found`: 存在しないキーのエラー
8. `test_complex_type`: 複雑な型のサポート
9. `test_keys_list`: キーリスト取得

### テスト実行方法
```bash
cargo test settings::tests --lib
```

## ファイル構造

```
$HOME/
  .kakeibon/
    ├── KakeiBonDB.sqlite3      # データベースファイル
    ├── KakeiBonDB.sqlite3-wal  # WALファイル
    ├── KakeiBonDB.sqlite3-shm  # 共有メモリファイル
    └── KakeiBon.json           # ユーザ設定ファイル（新規）
```

## 自動保存機能

`set_setting` Tauriコマンドは、設定変更後に自動的に`save()`を呼び出します。
手動で`save()`を呼ぶ必要はありません（Rustバックエンドから直接使用する場合を除く）。

## 設定例

### テーマ設定
```json
{
  "theme": "dark",
  "accent_color": "#007acc"
}
```

### ウィンドウ設定
```json
{
  "window": {
    "width": 1920,
    "height": 1080,
    "x": 100,
    "y": 100,
    "maximized": false
  }
}
```

### 言語・地域設定
```json
{
  "language": "ja",
  "timezone": "Asia/Tokyo",
  "date_format": "YYYY/MM/DD"
}
```

### アプリケーション設定
```json
{
  "auto_save": true,
  "auto_backup": true,
  "backup_interval_hours": 24,
  "show_tips": true
}
```

## まとめ

ユーザカスタマイズ設定管理システムを実装しました：
- ✅ JSON形式での設定保存
- ✅ 型安全な設定取得（String, i64, bool, カスタム型）
- ✅ 自動保存機能
- ✅ ファイルの自動作成
- ✅ 9つのTauriコマンド
- ✅ 9テスト全て成功

アプリケーション起動時に自動的に`$HOME/.kakeibon/KakeiBon.json`が作成され、
フロントエンドから簡単にユーザ設定を読み書きできます。
