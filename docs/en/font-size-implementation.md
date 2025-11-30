# Font Size Change Feature Implementation

## Overview

This feature dynamically changes the font size across the entire application and automatically adjusts the window size accordingly. It is modularized and reusable across multiple pages.

## Architecture

### Module Structure

```
res/js/
├── font-size.js         # Core font size functionality module
├── consts.js           # Constant definitions (font sizes, i18n keys)
├── menu.js             # Main menu (uses font-size.js)
└── user-management.js  # User management page (uses font-size.js)

src/
├── lib.rs              # Tauri command definitions
├── consts.rs           # Rust constant definitions
├── settings.rs         # Settings persistence
└── font_size_tests.rs  # Test suite (13 test cases)
```

### Key Functions

**JavaScript (font-size.js)**:
- `setupFontSizeMenuHandlers()` - Set up menu event handlers
- `setupFontSizeMenu()` - Generate and initialize menu items
- `applyFontSize()` - Apply font size and resize window
- `setupFontSizeModalHandlers()` - Set up custom settings modal handlers

**Rust (lib.rs)**:
- `set_font_size()` - Validate and save font size
- `get_font_size()` - Retrieve saved font size
- `adjust_window_size()` - Adjust window size

### Constant Definitions

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

## Applying to New Pages

Modularization makes it easy to apply the font size feature to new pages.

### 1. HTML Preparation

Add menu bar and font size settings modal:

```html
<!-- Add font size menu to menu bar -->
<div id="font-size-menu" class="menu-item">
    <span data-i18n="menu.font_size">Font Size</span>
    <div id="font-size-dropdown" class="dropdown">
        <!-- Generated dynamically by JavaScript -->
    </div>
</div>

<!-- Font size settings modal -->
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
                    <option value="small" data-i18n="font_size.small">Small</option>
                    <option value="medium" data-i18n="font_size.medium" selected>Medium</option>
                    <option value="large" data-i18n="font_size.large">Large</option>
                    <option value="custom" data-i18n="font_size.custom">Custom</option>
                </select>
            </div>
            <div class="form-group">
                <label for="font-size-percent" data-i18n="font_size.percentage">Percentage:</label>
                <input type="number" id="font-size-percent" min="50" max="200" step="5" value="100" />
            </div>
        </div>
        <div class="modal-footer">
            <button type="button" class="btn-secondary" id="font-size-cancel" data-i18n="common.cancel">Cancel</button>
            <button type="button" class="btn-primary" id="font-size-apply" data-i18n="common.apply">Apply</button>
        </div>
    </div>
</div>
```

### 2. JavaScript Implementation

Import and initialize the module:

```javascript
import { setupFontSizeMenuHandlers, setupFontSizeMenu, applyFontSize, setupFontSizeModalHandlers } from './font-size.js';

document.addEventListener('DOMContentLoaded', async function() {
    // Initialize font size feature
    setupFontSizeMenuHandlers();      // Set up menu event handlers
    await setupFontSizeMenu();         // Generate menu items
    setupFontSizeModalHandlers();      // Set up modal event handlers
    await applyFontSize();             // Apply saved font size
    
    // Other initialization...
});
```

### 3. CSS Verification

Ensure CSS uses em/rem units:

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
    max-width: 75em;  /* Use em instead of px */
    min-width: 25em;
}
```

## Test Suite

A comprehensive test suite is available for the font size feature.

### Test Structure

**File**: `src/font_size_tests.rs`

**Test Count**: 13

### Test Coverage

1. **Default Value Tests**
   - `test_font_size_default()` - Verify default font size

2. **Preset Size Tests**
   - `test_set_font_size_small()` - Set and retrieve small size
   - `test_set_font_size_medium()` - Set and retrieve medium size
   - `test_set_font_size_large()` - Set and retrieve large size
   - `test_validate_font_size_preset()` - Validate preset values

3. **Custom Percentage Tests**
   - `test_validate_font_size_custom_percentage()` - Valid percentages (50-200)
   - `test_invalid_font_size_custom_percentage()` - Invalid percentages
   - `test_font_size_custom_percentage_persistence()` - Custom value persistence

4. **Validation Tests**
   - `test_invalid_font_size_string()` - Reject invalid string values
   - `test_font_size_boundary_values()` - Handle boundary values (50, 200)

5. **Persistence Tests**
   - `test_font_size_persistence()` - Multiple set and retrieve operations
   - `test_font_size_overwrite()` - Value overwriting

6. **Constant Tests**
   - `test_font_size_constants()` - Verify constant values

### Running Tests

```bash
# Run font size tests only
cargo test font_size_tests --lib

# Run all tests
cargo test

# Run with detailed output
cargo test font_size_tests --lib -- --nocapture
```

### Test Implementation Example

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

## Troubleshooting

### Font Size Not Applied

**Cause**: Fixed sizes (px) used in CSS

**Solution**: Change to em/rem units

```css
/* ❌ Bad example */
.container {
    max-width: 450px;
}

/* ✅ Good example */
.container {
    max-width: 28em;
}
```

### Window Size Not Adjusted Correctly

**Cause**: Measuring size before layout update completes

**Solution**: Wait using `requestAnimationFrame()`

```javascript
await new Promise(resolve => {
    requestAnimationFrame(() => {
        requestAnimationFrame(resolve);
    });
});
```

### Menu Text Wrapping

**Cause**: `white-space: nowrap` not set

**Solution**: Add to CSS

```css
.menu-item,
.dropdown-item {
    white-space: nowrap;
}
```

## References

- [CSS Units: em vs rem](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units)
- [requestAnimationFrame API](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)
- [Tauri Window API](https://tauri.app/v1/api/js/window/)
- [getBoundingClientRect](https://developer.mozilla.org/en-US/docs/Web/API/Element/getBoundingClientRect)
