# Font Size Change Feature Implementation

## Overview

This feature dynamically changes the font size across the entire application and automatically adjusts the window size accordingly.

## Technical Challenges and Solutions

### 1. Automatic Window Resizing

#### Challenge
In HTML rendering engines, content size is affected by window size. Particularly when using CSS percentage specifications like `width: 90%` or flexbox layouts, content expands when the window is large, making it impossible to measure the accurate "natural size".

**Specific Problem**:
1. Increase font size → Content grows → Window expands
2. Decrease font size → Content shrinks but layout expands due to large window → Measurement shows large size → Window expands further
3. Window keeps growing with repeated changes

#### Solution
We adopted a technique of temporarily shrinking the window to minimum size (e.g., 400x300) before measuring the natural content size.

```javascript
async function adjustWindowSize() {
    // 1. First shrink to minimum size
    const minWidth = 400;
    const minHeight = 300;
    await invoke('adjust_window_size', { 
        width: minWidth, 
        height: minHeight 
    });
    
    // 2. Wait for layout update
    await new Promise(resolve => {
        requestAnimationFrame(() => {
            requestAnimationFrame(resolve);
        });
    });
    
    // 3. Measure natural content size
    // Get content size using getBoundingClientRect()
    
    // 4. Resize to final window size
}
```

**Advantages**:
- Compatible with flexible designs
- Accurate measurement even with complex layouts

**Disadvantages**:
- Visual issue (window briefly becomes small)
- Performance impact (two resize operations)

### 2. Window API in Tauri v2

#### Challenge
In Tauri v2, the import path for Window API from JavaScript has changed, and importing `@tauri-apps/api/window` directly in the frontend causes errors.

#### Solution
We implemented window resizing as a backend (Rust) command and call it from JavaScript using `invoke()`.

**Backend (Rust)**:
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

**Frontend (JavaScript)**:
```javascript
await invoke('adjust_window_size', { 
    width: newWidth, 
    height: newHeight 
});
```

### 3. Layout Update Timing

#### Challenge
Measuring content size immediately after changing CSS properties may occur before layout recalculation is complete.

#### Solution
Use nested `requestAnimationFrame()` twice to ensure layout update is complete before measurement.

```javascript
await new Promise(resolve => {
    requestAnimationFrame(() => {
        requestAnimationFrame(resolve);
    });
});
```

**Reason**:
- 1st frame: CSS changes are recognized by the browser engine
- 2nd frame: Layout recalculation and rendering preparation are complete

This technique ensures processing occurs when the browser engine has actually completed rendering preparation, regardless of graphics performance.

### 4. TAB Key Focus Trap

#### Challenge
In Tauri apps, the TAB key may be recognized as `e.key === 'Unidentified'`, making standard TAB key detection fail.

#### Solution
Check for `e.key === 'Unidentified'` and `e.code === 'Tab'` in addition to `e.key === 'Tab'`.

```javascript
const isTab = e.key === 'Tab' || (e.shiftKey && (e.key === 'Unidentified' || e.code === 'Tab'));
```

### 5. Preventing Multiple Resizes

#### Challenge
Multiple events may fire during font size changes, potentially executing resize operations multiple times.

#### Solution
Use a `resizeInProgress` flag to skip new resizes while one is in progress.

```javascript
let resizeInProgress = false;

async function applyFontSize() {
    if (resizeInProgress) {
        return;
    }
    
    resizeInProgress = true;
    
    try {
        // Resize processing
    } finally {
        resizeInProgress = false;
    }
}
```

## Implementation Notes

### CSS Unit Usage

- **Avoid fixed sizes (px)**: They don't scale with font size
- **Use relative units (em, rem)**: They scale proportionally with font size
- **Examples**:
  - ❌ `max-width: 450px`
  - ✅ `max-width: 28em`

### Content Size Measurement

Use `getBoundingClientRect()` to get the maximum size of all visible elements.

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

### Padding Settings

Add padding to prevent content from sticking to window edges.

```javascript
const paddingWidth = 40;   // Total 40px for left and right
const paddingHeight = 40;  // Total 40px for top and bottom

const newWidth = maxWidth + paddingWidth;
const newHeight = maxHeight + paddingHeight;
```

### Custom Percentage Value Handling

The backend accepts both presets (small/medium/large) and numeric percentages (50-200).

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

The frontend handles this similarly:

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

## Summary

Key considerations for implementing the font size change feature:

1. **Understand HTML rendering engine characteristics** (window size affects layout)
2. **Handle Tauri-specific issues** (Window API paths, TAB key recognition)
3. **Wait for layout updates properly** (requestAnimationFrame)
4. **Choose CSS units appropriately** (use em/rem)
5. **Prevent multiple executions** (flag-based control)

With these measures, we can achieve a flexible and robust font size change feature.
