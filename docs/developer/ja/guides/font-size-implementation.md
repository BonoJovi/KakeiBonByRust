# フォントサイズ変更機能の実装

## 概要

本機能は、アプリケーション全体のフォントサイズを動的に変更し、ウィンドウサイズも自動的に調整する機能です。モジュール化されており、複数のページで再利用可能です。

## アーキテクチャ

### モジュール構成

```
res/js/
├── font-size.js         # フォントサイズ機能のコアモジュール
├── consts.js           # 定数定義（フォントサイズ、i18nキー）
├── menu.js             # メインメニュー（font-size.jsを使用）
└── user-management.js  # ユーザ管理画面（font-size.jsを使用）

src/
├── lib.rs              # Tauriコマンド定義
├── consts.rs           # Rust側の定数定義
├── settings.rs         # 設定の永続化
└── font_size_tests.rs  # テストスイート（13テストケース）
```

### 主要な関数

**JavaScript (font-size.js)**:
- `setupFontSizeMenuHandlers()` - メニューのイベントハンドラー設定
- `setupFontSizeMenu()` - メニュー項目の生成と初期化
- `applyFontSize()` - フォントサイズの適用とウィンドウリサイズ
- `setupFontSizeModalHandlers()` - カスタム設定モーダルのハンドラー

**Rust (lib.rs)**:
- `set_font_size()` - フォントサイズの検証と保存
- `get_font_size()` - 保存されたフォントサイズの取得
- `adjust_window_size()` - ウィンドウサイズの調整

### 定数定義

**JavaScript (consts.js)**:
```javascript
export const FONT_SIZE_SMALL = 'small';
export const FONT_SIZE_MEDIUM = 'medium';
export const FONT_SIZE_LARGE = 'large';
export const FONT_SIZE_CUSTOM = 'custom';
export const FONT_SIZE_DEFAULT = FONT_SIZE_MEDIUM;

export const I18N_FONT_SIZE_SMALL = 'font_size.small';
export const I18N_FONT_SIZE_MEDIUM = 'font_size.medium';
export const I18N_FONT_SIZE_LARGE = 'font_size.large';
export const I18N_FONT_SIZE_CUSTOM = 'font_size.custom';

export const FONT_SIZE_OPTIONS = [
    { code: FONT_SIZE_SMALL, key: I18N_FONT_SIZE_SMALL },
    { code: FONT_SIZE_MEDIUM, key: I18N_FONT_SIZE_MEDIUM },
    { code: FONT_SIZE_LARGE, key: I18N_FONT_SIZE_LARGE },
    { code: FONT_SIZE_CUSTOM, key: I18N_FONT_SIZE_CUSTOM, action: 'modal' }
];
```

**Rust (consts.rs)**:
```rust
pub const FONT_SIZE_SMALL: &str = "small";
pub const FONT_SIZE_MEDIUM: &str = "medium";
pub const FONT_SIZE_LARGE: &str = "large";
pub const FONT_SIZE_DEFAULT: &str = FONT_SIZE_MEDIUM;
```

## 技術的な課題と解決策

### 1. ウィンドウの自動リサイズ

#### 課題
HTMLのレンダリングエンジンでは、コンテンツのサイズはウィンドウサイズに影響を受けます。特に、CSSで`width: 90%`などのパーセンテージ指定やflexboxレイアウトを使用している場合、ウィンドウが大きいとコンテンツも広がって測定されるため、正確な「自然なサイズ」が測定できません。

**問題の具体例**：
1. フォントサイズを大きくする → コンテンツが大きくなる → ウィンドウを拡大
2. フォントサイズを小さくする → コンテンツは小さくなるが、ウィンドウが大きいのでレイアウトが広がる → 測定すると大きいサイズになる → さらに拡大
3. 繰り返すごとにウィンドウがどんどん大きくなる

#### 解決策
測定前に一時的にウィンドウを最小サイズ（例：400x300）に縮小してから、コンテンツの自然なサイズを測定する手法を採用しました。

```javascript
async function adjustWindowSize() {
    // 1. まず最小サイズに縮小
    const minWidth = 400;
    const minHeight = 300;
    await invoke('adjust_window_size', { 
        width: minWidth, 
        height: minHeight 
    });
    
    // 2. レイアウト更新を待つ
    await new Promise(resolve => {
        requestAnimationFrame(() => {
            requestAnimationFrame(resolve);
        });
    });
    
    // 3. コンテンツの自然なサイズを測定
    // getBoundingClientRect()でコンテンツサイズを取得
    
    // 4. 最終的なウィンドウサイズにリサイズ
}
```

**メリット**：
- フレキシブルデザインに対応可能
- 複雑なレイアウトでも正確に測定できる

**デメリット**：
- 一瞬ウィンドウが小さくなる（視覚的な問題）
- リサイズが2回実行される（パフォーマンス）

### 2. Tauri v2でのWindow API

#### 課題
Tauri v2では、JavaScriptからのWindow APIのインポートパスが変更されており、フロントエンドで直接`@tauri-apps/api/window`をインポートするとエラーが発生しました。

#### 解決策
ウィンドウリサイズをバックエンド（Rust）のコマンドとして実装し、JavaScriptからは`invoke()`で呼び出す方式に変更しました。

**バックエンド（Rust）**：
```rust
#[tauri::command]
async fn adjust_window_size(
    width: f64,
    height: f64,
    window: tauri::Window
) -> Result<(), String> {
    use tauri::LogicalSize;
    
    let current_size = window.inner_size()
        .map_err(|e| format!("Failed to get window size: {}", e))?;
    
    let logical_current = current_size.to_logical::<f64>(
        window.scale_factor()
            .map_err(|e| format!("Failed to get scale factor: {}", e))?
    );
    
    // Resize to match content size (both expand and shrink)
    if (width - logical_current.width).abs() > 1.0 
        || (height - logical_current.height).abs() > 1.0 {
        window.set_size(LogicalSize::new(width, height))
            .map_err(|e| format!("Failed to resize window: {}", e))?;
    }
    
    Ok(())
}
```

**フロントエンド（JavaScript）**：
```javascript
await invoke('adjust_window_size', { 
    width: newWidth, 
    height: newHeight 
});
```

### 3. レイアウト更新のタイミング

#### 課題
CSSプロパティを変更した直後にコンテンツサイズを測定すると、レイアウトの再計算が完了していない可能性があります。

#### 解決策
`requestAnimationFrame()`を2回ネストして、確実にレイアウト更新が完了してから測定を行います。

```javascript
await new Promise(resolve => {
    requestAnimationFrame(() => {
        requestAnimationFrame(resolve);
    });
});
```

**理由**：
- 1回目：CSSの変更がブラウザエンジンに認識される
- 2回目：レイアウトの再計算と描画準備が完了する

この手法は、グラフィック性能に関わらず、ブラウザエンジンが実際に描画準備を完了したタイミングで処理を実行できます。

### 4. TABキーのフォーカストラップ

#### 課題
Tauriアプリでは、TABキーが`e.key === 'Unidentified'`として認識される場合があり、通常のTABキー判定では動作しません。

#### 解決策
`e.key === 'Tab'`に加えて、`e.key === 'Unidentified'`と`e.code === 'Tab'`もチェックします。

```javascript
const isTab = e.key === 'Tab' || (e.shiftKey && (e.key === 'Unidentified' || e.code === 'Tab'));
```

### 5. 複数回リサイズの防止

#### 課題
フォントサイズ変更時に、複数のイベントが発火してリサイズが複数回実行される可能性があります。

#### 解決策
`resizeInProgress`フラグを使用して、リサイズ中は新しいリサイズをスキップします。

```javascript
let resizeInProgress = false;

async function applyFontSize() {
    if (resizeInProgress) {
        return;
    }
    
    resizeInProgress = true;
    
    try {
        // リサイズ処理
    } finally {
        resizeInProgress = false;
    }
}
```

## 実装上の注意点

### CSS単位の使用

- **固定サイズ（px）は避ける**：フォントサイズに追従しないため
- **相対単位（em, rem）を推奨**：フォントサイズに比例して拡大縮小する
- **例**：
  - ❌ `max-width: 450px`
  - ✅ `max-width: 28em`

### コンテンツサイズの測定

測定には`getBoundingClientRect()`を使用し、表示されているすべての要素の最大サイズを取得します。

```javascript
const mainContent = document.getElementById('main-content');
const menuBar = document.getElementById('menu-bar');
const elements = [menuBar, mainContent];

let maxWidth = 0;
let maxHeight = 0;

for (const el of elements) {
    if (el && !el.classList.contains('hidden')) {
        const rect = el.getBoundingClientRect();
        maxWidth = Math.max(maxWidth, rect.right);
        maxHeight = Math.max(maxHeight, rect.bottom);
    }
}
```

### パディングの設定

コンテンツが窓枠にぴったりくっつかないように、パディングを追加します。

```javascript
const paddingWidth = 40;   // 左右合計40px
const paddingHeight = 40;  // 上下合計40px

const newWidth = maxWidth + paddingWidth;
const newHeight = maxHeight + paddingHeight;
```

### カスタムパーセンテージ値の処理

バックエンドでは、プリセット（small/medium/large）と数値パーセンテージ（50-200）の両方を受け入れます。

```rust
let size = match font_size.as_str() {
    FONT_SIZE_SMALL => FONT_SIZE_SMALL.to_string(),
    FONT_SIZE_MEDIUM => FONT_SIZE_MEDIUM.to_string(),
    FONT_SIZE_LARGE => FONT_SIZE_LARGE.to_string(),
    _ => {
        // Try to parse as custom percentage (50-200)
        match font_size.parse::<u32>() {
            Ok(percent) if percent >= 50 && percent <= 200 => font_size.clone(),
            _ => return Err("Invalid font size: must be 'small', 'medium', 'large', or a percentage between 50 and 200".to_string()),
        }
    }
};
```

フロントエンドでも同様に処理します：

```javascript
if (sizeMap[fontSize]) {
    // It's a preset size
    cssValue = sizeMap[fontSize];
} else {
    // It's a custom percentage value
    const percent = parseInt(fontSize);
    if (!isNaN(percent)) {
        cssValue = percent + '%';
    } else {
        cssValue = sizeMap['medium']; // fallback
    }
}
```

## まとめ

フォントサイズ変更機能の実装では、以下の点に注意が必要です：

1. **HTMLレンダリングエンジンの特性**を理解する（ウィンドウサイズがレイアウトに影響する）
2. **Tauri固有の問題**に対応する（Window APIのパス、TABキーの認識）
3. **レイアウト更新のタイミング**を適切に待つ（requestAnimationFrame）
4. **CSS単位**を適切に選択する（em/remの使用）
5. **複数回実行の防止**を実装する（フラグによる制御）

これらの対策により、フレキシブルで堅牢なフォントサイズ変更機能を実現できます。

## 新しいページへの適用方法

フォントサイズ機能をモジュール化したことで、新しいページへの適用が簡単になりました。

### 1. HTML側の準備

メニューバーとフォントサイズ設定モーダルを追加：

```html
<!-- メニューバーにフォントサイズメニューを追加 -->
<div id="font-size-menu" class="menu-item">
    <span data-i18n="menu.font_size">Font Size</span>
    <div id="font-size-dropdown" class="dropdown">
        <!-- JavaScript側で動的に生成 -->
    </div>
</div>

<!-- フォントサイズ設定モーダル -->
<div id="font-size-modal" class="modal hidden">
    <div class="modal-content">
        <div class="modal-header">
            <h2 data-i18n="font_size.modal_title">Font Size Settings</h2>
            <button class="close-btn" id="font-size-modal-close">&times;</button>
        </div>
        <div class="modal-body">
            <div class="form-group">
                <label for="font-size-preset" data-i18n="font_size.preset">Preset:</label>
                <select id="font-size-preset">
                    <option value="small" data-i18n="font_size.small">小</option>
                    <option value="medium" data-i18n="font_size.medium" selected>中</option>
                    <option value="large" data-i18n="font_size.large">大</option>
                    <option value="custom" data-i18n="font_size.custom">カスタム</option>
                </select>
            </div>
            <div class="form-group">
                <label for="font-size-percent" data-i18n="font_size.percentage">Percentage:</label>
                <input type="number" id="font-size-percent" min="50" max="200" step="5" value="100" />
            </div>
        </div>
        <div class="modal-footer">
            <button type="button" class="btn-secondary" id="font-size-cancel" data-i18n="common.cancel">キャンセル</button>
            <button type="button" class="btn-primary" id="font-size-apply" data-i18n="common.apply">変更</button>
        </div>
    </div>
</div>
```

### 2. JavaScript側の実装

モジュールをインポートして初期化：

```javascript
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';

document.addEventListener('DOMContentLoaded', async function() {
    // フォントサイズ機能の初期化
    setupFontSizeMenuHandlers();      // メニューのイベントハンドラー設定
    await setupFontSizeMenu();         // メニュー項目の生成
    setupFontSizeModalHandlers();      // モーダルのイベントハンドラー設定
    await applyFontSize();             // 保存されたフォントサイズを適用
    
    // 他の初期化処理...
});
```

### 3. CSSの確認

CSS側でem/rem単位を使用していることを確認：

```css
:root {
    --font-size-small: 85%;
    --font-size-medium: 100%;
    --font-size-large: 115%;
    --base-font-size: var(--font-size-medium);
}

body {
    font-size: var(--base-font-size);
}

.container {
    max-width: 75em;  /* px ではなく em を使用 */
    min-width: 25em;
}
```

## テストケース

フォントサイズ機能には包括的なテストスイートが用意されています。

### テスト構成

**ファイル**: `src/font_size_tests.rs`

**テストケース数**: 13

### テストカバレッジ

1. **デフォルト値のテスト**
   - `test_font_size_default()` - デフォルトフォントサイズの検証

2. **プリセットサイズのテスト**
   - `test_set_font_size_small()` - 小サイズの設定と取得
   - `test_set_font_size_medium()` - 中サイズの設定と取得
   - `test_set_font_size_large()` - 大サイズの設定と取得
   - `test_validate_font_size_preset()` - プリセット値の検証

3. **カスタムパーセンテージのテスト**
   - `test_validate_font_size_custom_percentage()` - 有効なパーセンテージ（50-200）
   - `test_invalid_font_size_custom_percentage()` - 無効なパーセンテージ
   - `test_font_size_custom_percentage_persistence()` - カスタム値の永続化

4. **バリデーションのテスト**
   - `test_invalid_font_size_string()` - 無効な文字列値の拒否
   - `test_font_size_boundary_values()` - 境界値（50, 200）の処理

5. **永続化のテスト**
   - `test_font_size_persistence()` - 複数回の設定と取得
   - `test_font_size_overwrite()` - 値の上書き

6. **定数のテスト**
   - `test_font_size_constants()` - 定数値の検証

### テストの実行

```bash
# フォントサイズテストのみ実行
cargo test font_size_tests --lib

# 全テストを実行
cargo test

# 詳細な出力で実行
cargo test font_size_tests --lib -- --nocapture
```

### テスト実装例

```rust
#[test]
fn test_set_font_size_small() {
    let (mut settings, temp_dir) = create_test_settings();
    
    // Set font size to small
    settings.set("font_size", FONT_SIZE_SMALL).unwrap();
    settings.save().unwrap();
    
    // Verify it was set correctly
    let size = settings.get_string("font_size").unwrap();
    assert_eq!(size, FONT_SIZE_SMALL);
    
    cleanup_test_dir(temp_dir);
}

#[test]
fn test_validate_font_size_custom_percentage() {
    // Test valid custom percentages
    let valid_percentages = vec!["50", "75", "100", "125", "150", "175", "200"];
    
    for percentage in valid_percentages {
        let percent: u32 = percentage.parse().unwrap();
        assert!(
            percent >= 50 && percent <= 200,
            "Percentage {} should be in range 50-200",
            percent
        );
    }
}
```

## トラブルシューティング

### フォントサイズが適用されない

**原因**: CSSで固定サイズ（px）が使用されている

**解決策**: em/rem単位に変更する

```css
/* ❌ 悪い例 */
.container {
    max-width: 450px;
}

/* ✅ 良い例 */
.container {
    max-width: 28em;
}
```

### ウィンドウサイズが正しく調整されない

**原因**: レイアウト更新が完了する前にサイズを測定している

**解決策**: `requestAnimationFrame()`を使用して待機する

```javascript
await new Promise(resolve => {
    requestAnimationFrame(() => {
        requestAnimationFrame(resolve);
    });
});
```

### メニューテキストが折り返される

**原因**: `white-space: nowrap`が設定されていない

**解決策**: CSSに追加する

```css
.menu-item,
.dropdown-item {
    white-space: nowrap;
}
```

## 参考リソース

- [CSS単位: em vs rem](https://developer.mozilla.org/ja/docs/Learn/CSS/Building_blocks/Values_and_units)
- [requestAnimationFrame API](https://developer.mozilla.org/ja/docs/Web/API/window/requestAnimationFrame)
- [Tauri Window API](https://tauri.app/v1/api/js/window/)
- [getBoundingClientRect](https://developer.mozilla.org/ja/docs/Web/API/Element/getBoundingClientRect)
