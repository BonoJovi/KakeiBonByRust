# フォントサイズ変更機能の実装

## 概要

本機能は、アプリケーション全体のフォントサイズを動的に変更し、ウィンドウサイズも自動的に調整する機能です。

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
