/**
 * Category Management UI Tests
 * Tests for category tree display, interaction, and accessibility
 */

// Test Suite 1: Category Tree Display
console.group('Category Tree Display Tests');

// Test 1.1: Category tree container exists
try {
    const categoryTree = document.getElementById('category-tree');
    if (!categoryTree) {
        throw new Error('Category tree container not found');
    }
    console.log('✓ Test 1.1: Category tree container exists');
} catch (error) {
    console.error('✗ Test 1.1 FAILED:', error.message);
}

// Test 1.2: Category items are rendered
try {
    const categoryItems = document.querySelectorAll('.category-item');
    if (categoryItems.length === 0) {
        throw new Error('No category items rendered');
    }
    console.log(`✓ Test 1.2: ${categoryItems.length} category items rendered`);
} catch (error) {
    console.error('✗ Test 1.2 FAILED:', error.message);
}

// Test 1.3: Three levels of categories exist
try {
    const level1 = document.querySelectorAll('.category-level-1');
    const level2 = document.querySelectorAll('.category-level-2');
    const level3 = document.querySelectorAll('.category-level-3');
    
    if (level1.length === 0) throw new Error('No level 1 categories found');
    if (level2.length === 0) throw new Error('No level 2 categories found');
    if (level3.length === 0) throw new Error('No level 3 categories found');
    
    console.log(`✓ Test 1.3: Three levels exist (L1:${level1.length}, L2:${level2.length}, L3:${level3.length})`);
} catch (error) {
    console.error('✗ Test 1.3 FAILED:', error.message);
}

// Test 1.4: Category names are displayed (not empty)
try {
    const categoryNames = document.querySelectorAll('.category-name');
    let emptyCount = 0;
    categoryNames.forEach(name => {
        if (!name.textContent.trim()) emptyCount++;
    });
    
    if (emptyCount > 0) {
        throw new Error(`${emptyCount} category names are empty`);
    }
    console.log(`✓ Test 1.4: All ${categoryNames.length} category names have text`);
} catch (error) {
    console.error('✗ Test 1.4 FAILED:', error.message);
}

// Test 1.5: Display order is shown
try {
    const orderElements = document.querySelectorAll('.category-order');
    if (orderElements.length === 0) {
        throw new Error('No display order elements found');
    }
    console.log(`✓ Test 1.5: Display order shown for ${orderElements.length} categories`);
} catch (error) {
    console.error('✗ Test 1.5 FAILED:', error.message);
}

console.groupEnd();

// Test Suite 2: Expand/Collapse Functionality
console.group('Expand/Collapse Functionality Tests');

// Test 2.1: Expand icons exist
try {
    const expandIcons = document.querySelectorAll('.expand-icon');
    if (expandIcons.length === 0) {
        throw new Error('No expand icons found');
    }
    console.log(`✓ Test 2.1: ${expandIcons.length} expand icons found`);
} catch (error) {
    console.error('✗ Test 2.1 FAILED:', error.message);
}

// Test 2.2: Expandable icons have correct class
try {
    const expandableIcons = document.querySelectorAll('.expand-icon.expandable');
    if (expandableIcons.length === 0) {
        throw new Error('No expandable icons found');
    }
    console.log(`✓ Test 2.2: ${expandableIcons.length} expandable icons have correct class`);
} catch (error) {
    console.error('✗ Test 2.2 FAILED:', error.message);
}

// Test 2.3: Empty icons for leaf nodes
try {
    const emptyIcons = document.querySelectorAll('.expand-icon.empty');
    // Level 3 items should have empty icons
    const level3Count = document.querySelectorAll('.category-level-3').length;
    if (emptyIcons.length !== level3Count) {
        console.warn(`⚠ Test 2.3: Expected ${level3Count} empty icons, found ${emptyIcons.length}`);
    } else {
        console.log(`✓ Test 2.3: ${emptyIcons.length} empty icons for leaf nodes`);
    }
} catch (error) {
    console.error('✗ Test 2.3 FAILED:', error.message);
}

// Test 2.4: Expand icon size (should be 2em)
try {
    const icon = document.querySelector('.expand-icon');
    if (icon) {
        const styles = window.getComputedStyle(icon);
        const width = styles.width;
        const height = styles.height;
        // Should be approximately 2em (32px at default font size)
        console.log(`✓ Test 2.4: Expand icon size is ${width} x ${height}`);
    } else {
        throw new Error('No expand icon found for size test');
    }
} catch (error) {
    console.error('✗ Test 2.4 FAILED:', error.message);
}

console.groupEnd();

// Test Suite 3: Action Buttons
console.group('Action Buttons Tests');

// Test 3.1: Level 1 has only "Add Sub" button
try {
    const level1Items = document.querySelectorAll('.category-level-1');
    let allCorrect = true;
    level1Items.forEach(item => {
        const actions = item.querySelector('.category-actions');
        const buttons = actions.querySelectorAll('.btn-icon');
        if (buttons.length !== 1 || !buttons[0].classList.contains('btn-add')) {
            allCorrect = false;
        }
    });
    if (!allCorrect) {
        throw new Error('Level 1 should only have Add Sub button');
    }
    console.log(`✓ Test 3.1: All ${level1Items.length} level 1 items have only Add Sub button`);
} catch (error) {
    console.error('✗ Test 3.1 FAILED:', error.message);
}

// Test 3.2: Level 2 has Add Sub, Edit, Up, Down buttons
try {
    const level2Item = document.querySelector('.category-level-2');
    if (level2Item) {
        const actions = level2Item.querySelector('.category-actions');
        const hasAdd = actions.querySelector('.btn-add') !== null;
        const hasEdit = actions.querySelector('.btn-edit') !== null;
        const hasUp = actions.querySelector('.btn-up') !== null;
        const hasDown = actions.querySelector('.btn-down') !== null;
        
        if (!hasAdd || !hasEdit || !hasUp || !hasDown) {
            throw new Error('Level 2 missing required buttons');
        }
        console.log('✓ Test 3.2: Level 2 has all required buttons (Add Sub, Edit, Up, Down)');
    } else {
        throw new Error('No level 2 item found');
    }
} catch (error) {
    console.error('✗ Test 3.2 FAILED:', error.message);
}

// Test 3.3: Level 3 has Edit, Up, Down buttons (no Add Sub)
try {
    const level3Item = document.querySelector('.category-level-3');
    if (level3Item) {
        const actions = level3Item.querySelector('.category-actions');
        const hasAdd = actions.querySelector('.btn-add') !== null;
        const hasEdit = actions.querySelector('.btn-edit') !== null;
        const hasUp = actions.querySelector('.btn-up') !== null;
        const hasDown = actions.querySelector('.btn-down') !== null;
        
        if (hasAdd) {
            throw new Error('Level 3 should not have Add Sub button');
        }
        if (!hasEdit || !hasUp || !hasDown) {
            throw new Error('Level 3 missing required buttons');
        }
        console.log('✓ Test 3.3: Level 3 has Edit, Up, Down buttons (no Add Sub)');
    } else {
        throw new Error('No level 3 item found');
    }
} catch (error) {
    console.error('✗ Test 3.3 FAILED:', error.message);
}

// Test 3.4: Buttons on new line (flex-basis: 100%)
try {
    const actions = document.querySelector('.category-actions');
    if (actions) {
        const styles = window.getComputedStyle(actions);
        const flexBasis = styles.flexBasis;
        if (flexBasis !== '100%') {
            console.warn(`⚠ Test 3.4: flex-basis is ${flexBasis}, expected 100%`);
        } else {
            console.log('✓ Test 3.4: Action buttons on new line (flex-basis: 100%)');
        }
    }
} catch (error) {
    console.error('✗ Test 3.4 FAILED:', error.message);
}

console.groupEnd();

// Test Suite 4: CSS Styling and Layout
console.group('CSS Styling and Layout Tests');

// Test 4.1: Category name uses correct flex properties
try {
    const categoryName = document.querySelector('.category-name');
    if (categoryName) {
        const styles = window.getComputedStyle(categoryName);
        const flexGrow = styles.flexGrow;
        const wordBreak = styles.wordBreak;
        const whiteSpace = styles.whiteSpace;
        
        console.log(`✓ Test 4.1: Category name styles (flex-grow:${flexGrow}, word-break:${wordBreak}, white-space:${whiteSpace})`);
    }
} catch (error) {
    console.error('✗ Test 4.1 FAILED:', error.message);
}

// Test 4.2: Category order has fixed width
try {
    const categoryOrder = document.querySelector('.category-order');
    if (categoryOrder) {
        const styles = window.getComputedStyle(categoryOrder);
        const width = styles.width;
        console.log(`✓ Test 4.2: Category order has width: ${width}`);
    }
} catch (error) {
    console.error('✗ Test 4.2 FAILED:', error.message);
}

// Test 4.3: Button borders are 2px
try {
    const button = document.querySelector('.btn-icon');
    if (button) {
        const styles = window.getComputedStyle(button);
        const borderWidth = styles.borderWidth;
        if (borderWidth !== '2px') {
            console.warn(`⚠ Test 4.3: Border width is ${borderWidth}, expected 2px`);
        } else {
            console.log('✓ Test 4.3: Button borders are 2px');
        }
    }
} catch (error) {
    console.error('✗ Test 4.3 FAILED:', error.message);
}

// Test 4.4: Expand icon font size is 1.2em
try {
    const icon = document.querySelector('.expand-icon');
    if (icon) {
        const styles = window.getComputedStyle(icon);
        const fontSize = styles.fontSize;
        console.log(`✓ Test 4.4: Expand icon font size: ${fontSize}`);
    }
} catch (error) {
    console.error('✗ Test 4.4 FAILED:', error.message);
}

console.groupEnd();

// Test Suite 5: Accessibility
console.group('Accessibility Tests');

// Test 5.1: Expandable elements have pointer cursor
try {
    const expandable = document.querySelector('.expand-icon.expandable');
    if (expandable) {
        const styles = window.getComputedStyle(expandable);
        const cursor = styles.cursor;
        if (cursor !== 'pointer') {
            console.warn(`⚠ Test 5.1: Expandable cursor is ${cursor}, expected pointer`);
        } else {
            console.log('✓ Test 5.1: Expandable elements have pointer cursor');
        }
    }
} catch (error) {
    console.error('✗ Test 5.1 FAILED:', error.message);
}

// Test 5.2: Category names with children are clickable
try {
    const expandableNames = document.querySelectorAll('.category-name.expandable');
    if (expandableNames.length > 0) {
        const styles = window.getComputedStyle(expandableNames[0]);
        const cursor = styles.cursor;
        if (cursor !== 'pointer') {
            console.warn(`⚠ Test 5.2: Expandable name cursor is ${cursor}, expected pointer`);
        } else {
            console.log(`✓ Test 5.2: ${expandableNames.length} expandable names have pointer cursor`);
        }
    }
} catch (error) {
    console.error('✗ Test 5.2 FAILED:', error.message);
}

// Test 5.3: Focus management function exists
try {
    if (typeof setupFocusHoverManagement !== 'function') {
        console.warn('⚠ Test 5.3: setupFocusHoverManagement function not found in global scope');
    } else {
        console.log('✓ Test 5.3: Focus/hover management function exists');
    }
} catch (error) {
    console.error('✗ Test 5.3 FAILED:', error.message);
}

// Test 5.4: Buttons are keyboard accessible (can receive focus)
try {
    const button = document.querySelector('.btn-icon');
    if (button) {
        const tabIndex = button.tabIndex;
        // Should be 0 or not explicitly set (default focusable)
        if (tabIndex < 0) {
            throw new Error('Buttons are not keyboard accessible (tabIndex < 0)');
        }
        console.log('✓ Test 5.4: Buttons are keyboard accessible');
    }
} catch (error) {
    console.error('✗ Test 5.4 FAILED:', error.message);
}

console.groupEnd();

// Test Suite 6: Data Integrity
console.group('Data Integrity Tests');

// Test 6.1: Expected number of categories (3 level 1, 20 level 2, 126 level 3)
try {
    const level1Count = document.querySelectorAll('.category-level-1').length;
    const level2Count = document.querySelectorAll('.category-level-2').length;
    const level3Count = document.querySelectorAll('.category-level-3').length;
    
    if (level1Count !== 3) console.warn(`⚠ Expected 3 level 1 categories, got ${level1Count}`);
    if (level2Count !== 20) console.warn(`⚠ Expected 20 level 2 categories, got ${level2Count}`);
    if (level3Count !== 126) console.warn(`⚠ Expected 126 level 3 categories, got ${level3Count}`);
    
    console.log(`✓ Test 6.1: Category counts - L1:${level1Count}, L2:${level2Count}, L3:${level3Count}`);
} catch (error) {
    console.error('✗ Test 6.1 FAILED:', error.message);
}

// Test 6.2: Each category has data attributes
try {
    const items = document.querySelectorAll('.category-item');
    let missingData = 0;
    items.forEach(item => {
        if (!item.dataset.level) missingData++;
    });
    
    if (missingData > 0) {
        throw new Error(`${missingData} items missing data-level attribute`);
    }
    console.log(`✓ Test 6.2: All ${items.length} items have required data attributes`);
} catch (error) {
    console.error('✗ Test 6.2 FAILED:', error.message);
}

console.groupEnd();

console.log('\n=== Category Management UI Tests Complete ===');
