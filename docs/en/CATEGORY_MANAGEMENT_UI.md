# Category Management Screen Implementation Document

## Overview

This document records the implementation details of KakeiBon's category management screen (Phase 4-1 to 4-3).

**Implementation Period**: 2025-10-28 ~ 2025-10-31  
**Last Updated**: 2025-11-04 20:04 JST

---

## Phase 4-1: Category List Display ✅

### Implementation Details

#### Basic Features
- **Tree Structure Display**: Hierarchical structure of major → middle → minor categories
- **Real Data Integration**: Retrieved from `get_category_tree_with_lang` API
- **Expand/Collapse**: Clickable expand icons (▶/▼)
- **Multilingual Support**: Display names according to current language

#### UI/UX Improvements

##### 1. Layout Improvements
- **Horizontal Name Placement**: Display category names horizontally as much as possible
- **Buttons on Next Line**: Action buttons are placed on the next line, aligned to the right (using `flex-basis: 100%`)
- **Line Break Control**: Prevent unnatural character-by-character line breaks in category names (`word-break: keep-all`)

##### 2. Expand Icon Improvements
- **Larger Size**: Icon container enlarged to 20px × 20px (`2em × 2em`)
- **Font Size Adjustment**: 
  - `▶` (collapsed): `font-size: 1.25em`
  - `▼` (expanded): `font-size: 1.5em` (compensating for visual size difference in Unicode characters)
- **Double-Click Expand**: Double-click on name to expand/collapse
- **Enhanced Visibility**: Optimized icon color and size

##### 3. Accessibility Improvements
- **Focus Management**: Smart focus display using `:focus-visible`
  - Show focus outline only during keyboard navigation
  - Hidden during mouse clicks
- **Double Border Display**: Visual feedback on focus/hover
  - 2px dark gray border (#444)
  - Only one element displays double border at a time (others are cleared)
- **Hover Behavior**: Temporarily hide focus outline on other elements during hover
  - Automatically restores on mouse out

#### Technical Implementation

**File Structure**:
- `res/category-management.html`: Screen HTML
- `res/js/category-management.js`: Business logic (801 lines)
- `res/css/category-management.css`: Style definitions (319 lines)

**Main Functions**:
```javascript
// Retrieve and display tree data
async loadCategoryTree()

// Rendering for each level
renderCategory1(cat1)
renderCategory2(cat2, parent1Code)
renderCategory3(cat3, parent1Code, parent2Code)

// Expand/collapse
toggleCategory(level, code)
```

**Main CSS Classes**:
```css
.category-name          /* Basic name style */
.category-name.expandable /* Expandable name */
.expand-icon            /* Expand icon */
.expand-icon.expanded   /* Expanded state (▼) */
.expand-icon.collapsed  /* Collapsed state (▶) */
.category-actions       /* Action button container (next line placement) */
.mouse-active           /* Hover marker */
```

#### Data Structure

**CategoryTree Structure**:
```javascript
{
  category1: {
    user_id: 1,
    category1_code: "EXPENSE",
    category1_name: "支出",
    category1_name_i18n: "Expense",
    display_order: 1,
    is_disabled: false
  },
  category2_list: [
    {
      user_id: 1,
      category2_code: "C2_E_1",
      category1_code: "EXPENSE",
      category2_name: "食費",
      category2_name_i18n: "Food",
      display_order: 1,
      is_disabled: false
    }
  ],
  category3_list: [
    {
      user_id: 1,
      category3_code: "C3_1",
      category2_code: "C2_E_1",
      category3_name: "食料品",
      category3_name_i18n: "Groceries",
      display_order: 1,
      is_disabled: false
    }
  ]
}
```

#### Constraints

1. **Fixed Major Categories**: EXPENSE, INCOME, TRANSFER
   - Users cannot add, edit, or delete major categories
   - Only subcategory addition is allowed

2. **Initial Data**: Automatically populated when user is created (`initialize_user_categories`)
   - 20 middle categories (CATEGORY2)
   - 126 minor categories (CATEGORY3)
   - All Japanese names populated (20 middle, 126 minor)
   - All English I18N data populated (20 middle, 126 minor) ✅

---

## Phase 4-2: Middle Category Add/Edit ✅

### Implementation Details

#### Modal Dialog Approach
- **Common Modal Class**: `res/js/modal.js` (ES Module)
- **Focus Trap**: Cycle within modal using TAB/SHIFT+TAB
- **ESC Key Support**: Close modal
- **Backdrop Click**: Close on clicking outside modal

#### Add Feature
**Trigger**: "Add Subcategory" button on major category

**Input Fields**:
- Name (Japanese): Required, duplicate check
- Name (English): Required, duplicate check

**Processing Flow**:
1. Display modal (showing parent category info)
2. Input validation
3. Call `add_category2` API
4. Reload tree
5. Show success message

#### Edit Feature
**Trigger**: "Edit" button on middle category

**Processing Flow**:
1. Retrieve existing data and display in modal
2. Input validation (duplicate check excluding self)
3. Call `update_category2` API
4. Reload tree

#### API Integration

**Add API**:
```javascript
await invoke('add_category2', {
  userId: this.userId,
  category1Code: parent1Code,
  category2NameJa: nameJa,
  category2NameEn: nameEn
});
```

**Edit API**:
```javascript
await invoke('update_category2', {
  userId: this.userId,
  category2Code: code,
  category2NameJa: nameJa,
  category2NameEn: nameEn
});
```

#### Validation

**Backend (Rust)**:
```sql
-- Duplicate check for add
SELECT COUNT(*) FROM CATEGORY2_I18N 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? 
AND LANG_CODE = ? AND CATEGORY2_NAME_I18N = ?

-- Duplicate check for edit (excluding self)
SELECT COUNT(*) FROM CATEGORY2_I18N 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? 
AND CATEGORY2_CODE != ? AND LANG_CODE = ? 
AND CATEGORY2_NAME_I18N = ?
```

---

## Phase 4-3: Minor Category Add/Edit ✅

### Implementation Details

#### Basic Structure
Adopts the same modal approach as middle categories.

#### Add Feature
**Trigger**: "Add Subcategory" button on middle category

**Input Fields**:
- Name (Japanese): Required, duplicate check
- Name (English): Required, duplicate check

**Processing Flow**:
1. Display modal (showing parent category info)
2. Input validation
3. Call `add_category3` API
4. Reload tree

#### Edit Feature
**Trigger**: "Edit" button on minor category

**Processing Flow**:
1. Retrieve existing data and display in modal
2. Input validation (duplicate check excluding self)
3. Call `update_category3` API
4. Reload tree

#### Implementation Fixes

**Data Attribute Addition**:
```javascript
// Add data-category3-code attribute to minor category button
button.setAttribute('data-category3-code', cat3.category3_code);
```

**Parent Category Code Passing**:
```javascript
// Passing from renderCategory2 to renderCategory3
renderCategory2(cat2, parent1Code) {
  // ...
  cat2.category3_list.forEach(cat3 => {
    renderCategory3(cat3, parent1Code, cat2.category2_code);
  });
}
```

**Event Handler Fix**:
```javascript
// Get appropriate category code
const category3Code = button.dataset.category3Code;
```

#### API Integration

**Add API**:
```javascript
await invoke('add_category3', {
  userId: this.userId,
  category2Code: parent2Code,
  category3NameJa: nameJa,
  category3NameEn: nameEn
});
```

**Edit API**:
```javascript
await invoke('update_category3', {
  userId: this.userId,
  category3Code: code,
  category3NameJa: nameJa,
  category3NameEn: nameEn
});
```

---

## Common Implementation Patterns

### Level Constants
```javascript
const LEVEL_CATEGORY1 = 1;
const LEVEL_CATEGORY2 = 2;
const LEVEL_CATEGORY3 = 3;
```

Changed from literal comparison to constant comparison to improve code readability.

### Error Handling
```javascript
try {
  await invoke('add_category2', params);
  await this.loadCategoryTree();
  alert(await this.i18n.t('common.save_success'));
} catch (error) {
  console.error('Error adding category:', error);
  alert(await this.i18n.t('common.error_occurred'));
}
```

### Modal Management
```javascript
// Open modal
this.modal.open();

// Close modal
this.modal.close();

// ESC key support (automatic)
// Backdrop click support (automatic)
```

---

## Performance Optimization

### Tree Redrawing
- Reload entire tree after add/edit
- Expansion state is not preserved (prioritizing simple implementation)
- Future improvement: Save and restore expansion state

### DOM Operations
- Using `innerHTML` for simple implementation
- Consider event delegation (under consideration)

---

## Testing

### Manual Testing (Completed)
- ✅ Middle category add/edit
- ✅ Minor category add/edit
- ✅ Duplicate check (add/edit)
- ✅ Language switching
- ✅ Expand/collapse
- ✅ Accessibility (keyboard navigation)

### Automated Testing (Not Implemented)
- To be implemented after introducing frontend testing framework
- Target: Modal operations, tree display, API integration

---

## Known Issues

### ~~English I18N Data Shortage~~ ✅ Resolved
- ✅ Japanese names: All populated (20 middle, 126 minor)
- ✅ English I18N: All populated (20 middle, 126 minor)
- **Resolution Date**: 2025-11-03 (Commit: fcf7696)

### Expand Icon Font Differences
- `▶` and `▼` Unicode characters have different visual sizes depending on font
- **Solution**: Individual `font-size` adjustment (▶: 1.25em, ▼: 1.5em) ensures visibility

---

## Future Extensions

### Phase 4-4: Display Order Changes

#### Up/Down Movement Feature
- [ ] Implementation of `moveCategoryUp()`
- [ ] Implementation of `moveCategoryDown()`
- [ ] Optimistic UI updates
- [ ] Rollback on error

#### Display Order Reset Feature
- [ ] **Implementation of "Reset to Initial Order" button**
  
**Design Policy**:
- Reassign DISPLAY_ORDER based on ENTRY_DT (registration date/time) order
- Based on registration date/time (registration order) to restore the order in which category data was automatically populated

**Implementation Method**:
```sql
-- Example for CATEGORY2
WITH ordered AS (
  SELECT USER_ID, CATEGORY1_CODE, CATEGORY2_CODE,
         ROW_NUMBER() OVER (
           PARTITION BY USER_ID, CATEGORY1_CODE 
           ORDER BY ENTRY_DT
         ) as new_order
  FROM CATEGORY2
  WHERE USER_ID = ?
)
UPDATE CATEGORY2 
SET DISPLAY_ORDER = (
  SELECT new_order FROM ordered 
  WHERE ordered.USER_ID = CATEGORY2.USER_ID 
    AND ordered.CATEGORY1_CODE = CATEGORY2.CATEGORY1_CODE
    AND ordered.CATEGORY2_CODE = CATEGORY2.CATEGORY2_CODE
)
WHERE USER_ID = ?;
```

**Benefits**:
- ENTRY_DT is unique for each record
- Registration order is clearly preserved
- No table structure changes required

### Phase 4-5: UI Adjustments
- [ ] Button enable/disable control
- [ ] Improved multilingual error messages
- [ ] Loading display improvements

---

## Reference Documents

- [Category Management API Documentation](API_CATEGORY.md)
- [Accessibility Indicators](ACCESSIBILITY_INDICATORS.md)
- [Testing Strategy](TEST_SUMMARY.md)

---

**Created**: 2025-11-02 21:44 JST
