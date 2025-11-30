# Manufacturer & Product Management - User Guide

**Last Updated**: 2025-11-12 01:31 JST

## Overview

Manufacturer and Product Management features allow you to manage manufacturers and products used in your household accounting. These features include the "IS_DISABLED" functionality to hide items without deleting them.

---

## Basic Features

### Manufacturer Management

Manage manufacturers (brands/vendors).

**Key Features:**
- Add, edit, and delete manufacturers
- Record detailed information in memo field
- Flexible management with disabled status

### Product Management

Manage products (items). Products can be optionally associated with manufacturers.

**Key Features:**
- Add, edit, and delete products
- Associate with manufacturers (optional)
- Record detailed information in memo field
- Flexible management with disabled status

---

## IS_DISABLED Feature (Hide Feature)

### Overview

The IS_DISABLED feature allows you to hide items temporarily without permanently deleting them.

### How to Use

#### 1. Setting Disabled Status

**When Adding New:**
1. Click "Add Manufacturer" or "Add Product" button
2. Fill in required information
3. Check the "Disabled" checkbox if needed
4. Click "Save"

**When Editing:**
1. Click "Edit" button for the target item
2. Check/uncheck the "Disabled" checkbox
3. Click "Save"

#### 2. Show/Hide Disabled Items Toggle

Use the toggle button at the top of the screen to show or hide disabled items.

- **"Show Disabled Items"**: Display including disabled items
- **"Hide Disabled Items"**: Display only active items (default)

#### 3. Visual Appearance of Disabled Items

Disabled items are distinguished by:

- **Background Color**: Dark gray (#6c757d)
- **Text Color**: White (#ffffff)
- **Badge**: Yellow "[Disabled]" label
- **Action Buttons**: Displayed normally (can be edited/deleted)

### Difference from Delete Function

| Operation | Data Handling | Use Case | Reversibility |
|-----------|--------------|----------|---------------|
| **Disable** | Data is retained | Temporarily unused items | Easy to re-enable |
| **Delete** | Sets IS_DISABLED=1 | Discontinued items | Hidden from list view |

> **Note**: The "Delete" button performs logical deletion (sets IS_DISABLED=1). Data is not removed from the database.

### Use Cases

#### Seasonal Products
```
Example: "Cold Noodles"
Summer: Disabled OFF (active) → Shown in list
Winter: Disabled ON → Hidden from list
Spring: Disabled OFF → Shown again in list
```

#### Trial Period Manufacturers
```
Example: "New Manufacturer A"
During trial: Disabled OFF (active)
Trial ended (not adopted): Disabled ON
Reconsidering: Disabled OFF → Data remains available
```

#### Past Business Partners
```
Example: "Former Supplier"
During business: Disabled OFF (active)
After business ends: Delete → IS_DISABLED=1 (data retained)
History review: View with "Show Disabled Items"
```

---

## Operation Guide

### Adding a Manufacturer

1. Click "Add Manufacturer" button
2. **Manufacturer Name**: Required (e.g., Nissui)
3. **Memo**: Optional (e.g., Nippon Suisan Kaisha, Ltd.)
4. **Disabled**: Check if needed
5. Click "Save"

### Editing a Manufacturer

1. Click "Edit" button for the target manufacturer
2. Modify necessary fields
3. Click "Save"

### Deleting a Manufacturer

1. Click "Delete" button for the target manufacturer
2. Confirm in the dialog by clicking "Delete"

> **Note**: Deleted manufacturers are set to IS_DISABLED=1 (logical deletion)

### Adding a Product

1. Click "Add Product" button
2. **Product Name**: Required (e.g., Canned Mackerel)
3. **Manufacturer**: Optional (select from dropdown)
4. **Memo**: Optional (e.g., In Water)
5. **Disabled**: Check if needed
6. Click "Save"

### Editing a Product

1. Click "Edit" button for the target product
2. Modify necessary fields
3. Click "Save"

### Deleting a Product

1. Click "Delete" button for the target product
2. Confirm in the dialog by clicking "Delete"

> **Note**: Deleted products are set to IS_DISABLED=1 (logical deletion)

---

## Data Relationships

### Manufacturer and Product Relationship

- Products can be associated with manufacturers (optional)
- Product data is retained even if the manufacturer is disabled/deleted
- Products associated with disabled manufacturers will not show the manufacturer name in list view

### Display Order

- Manufacturers and products are automatically sorted by `DISPLAY_ORDER`
- Displayed in order of addition

---

## Validation Rules

### Manufacturer

| Field | Rule |
|-------|------|
| Manufacturer Name | Required, non-empty, unique |
| Memo | Optional |
| Disabled | 0 (active) or 1 (disabled) |

### Product

| Field | Rule |
|-------|------|
| Product Name | Required, non-empty, unique |
| Manufacturer | Optional |
| Memo | Optional |
| Disabled | 0 (active) or 1 (disabled) |

---

## Frequently Asked Questions (FAQ)

### Q1: Can deleted data be restored?

**A**: Yes, deletion is logical. Display with "Show Disabled Items" button, then uncheck "Disabled" in the edit form to restore.

### Q2: What's the difference between disable and delete?

**A**: 
- **Disable**: Manually check/uncheck the checkbox
- **Delete**: Execute with delete button (automatically sets to disabled)

Both retain data and are reversible.

### Q3: If I delete a manufacturer, are related products also deleted?

**A**: No, products are not deleted. However, products associated with disabled manufacturers will not show the manufacturer name in list view.

### Q4: Are disabled items included in search results?

**A**: No, when "Show Disabled Items" is off, disabled items are not included in search results.

### Q5: Can I permanently delete data?

**A**: In the current version, physical deletion from UI is not available. You need to delete directly from the database.

---

## Troubleshooting

### Issue: Disabled items are not showing

**Solution**: 
1. Verify "Show Disabled Items" button is clicked
2. Reload the view (navigate to another screen and back)
3. Check database connection

### Issue: Cannot delete

**Solution**:
1. Verify the target item exists
2. Verify logged-in user is correct
3. Check error messages

### Issue: Duplicate manufacturer/product name

**Solution**:
- Enter a different name from existing ones
- Duplicate check includes disabled items

---

## Related Documentation

- [IS_DISABLED Implementation Guide (For Developers)](./IS_DISABLED_IMPLEMENTATION_GUIDE.md)
- [Manufacturer API Specification](./API_MANUFACTURER.md)
- [Product API Specification](./API_PRODUCT.md)
- [Database Configuration](./DATABASE_CONFIGURATION.md)

---

**Change History**
- 2025-11-12: Initial version
