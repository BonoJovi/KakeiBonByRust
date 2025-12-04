# Master Data Management API Reference

**Last Updated**: 2025-12-05 02:34 JST

## Overview

This document defines APIs used in the master data management screens (shop-management.html, manufacturer-management.html, product-management.html). Manages three master data types: shops, manufacturers, and products.

---

## Table of Contents

1. [Shop Management API](#shop-management-api)
2. [Manufacturer Management API](#manufacturer-management-api)
3. [Product Management API](#product-management-api)
4. [Data Structures](#data-structures)

---

## Shop Management API

### get_shops

Retrieves the list of shops.

**Parameters:** None

**Return Value:**
- `Vec<Shop>`: Array of shops (non-disabled only)

**Shop Structure:**
```javascript
{
    shop_id: number,
    user_id: number,
    shop_name: string,
    memo: string | null,
    display_order: number,
    is_disabled: number,    // 0=active, 1=disabled
    entry_dt: string,
    update_dt: string | null
}
```

**Usage Example:**
```javascript
const shops = await invoke('get_shops');
shops.forEach(shop => {
    console.log(`${shop.shop_name}`);
});
```

**Note:**
- Session user ID automatically retrieved
- Only retrieves where `IS_DISABLED = 0` (excludes disabled)
- Sorted by `DISPLAY_ORDER`

---

### add_shop

Adds a new shop.

**Parameters:**
- `shop_name` (String): Shop name (required)
- `memo` (Option<String>): Memo

**Return Value:**
- `String`: "Shop added successfully"

**Usage Example:**
```javascript
try {
    await invoke('add_shop', {
        shopName: 'AEON Shinjuku',
        memo: 'Double points on Tuesdays'
    });
    
    alert('Shop added');
    await loadShops();
} catch (error) {
    alert(`Addition failed: ${error}`);
}
```

**Automatic Processing:**
1. **Display order auto-assignment**: Max value + 1
2. **is_disabled**: Set to 0 (active)

**Validation:**
- Shop name required
- Name duplicate check within same user

**Errors:**
- `"Shop name cannot be empty"`: Shop name is empty
- `"Shop name 'XXX' already exists"`: Name duplicate
- `"Failed to add shop: ..."`: Database error

---

### update_shop

Updates shop information.

**Parameters:**
- `shop_id` (i64): Shop ID
- `shop_name` (String): New shop name
- `memo` (Option<String>): New memo
- `display_order` (i64): New display order

**Return Value:**
- `String`: "Shop updated successfully"

**Usage Example:**
```javascript
await invoke('update_shop', {
    shopId: 1,
    shopName: 'AEON Shinjuku (Updated)',
    memo: null,
    displayOrder: 2
});
```

**Validation:**
- Name duplicate check excluding self

---

### delete_shop

Logically deletes a shop.

**Parameters:**
- `shop_id` (i64): Shop ID

**Return Value:**
- `String`: "Shop deleted successfully"

**Usage Example:**
```javascript
if (confirm('Are you sure you want to delete this shop?')) {
    await invoke('delete_shop', { shopId: 1 });
    alert('Shop deleted');
    await loadShops();
}
```

**Behavior:**
- Logical deletion (`IS_DISABLED = 1`)
- Can be deleted even if in use by transactions
- Past transaction history retained

---

## Manufacturer Management API

### get_manufacturers

Retrieves the list of manufacturers.

**Parameters:**
- `include_disabled` (bool): Whether to include disabled

**Return Value:**
- `Vec<Manufacturer>`: Array of manufacturers

**Manufacturer Structure:**
```javascript
{
    manufacturer_id: number,
    user_id: number,
    manufacturer_name: string,
    memo: string | null,
    display_order: number,
    is_disabled: number,
    entry_dt: string,
    update_dt: string | null
}
```

**Usage Example:**
```javascript
// Get active manufacturers only
const manufacturers = await invoke('get_manufacturers', { 
    includeDisabled: false 
});

// Get all (including disabled)
const allManufacturers = await invoke('get_manufacturers', { 
    includeDisabled: true 
});
```

**Purpose:**
- `includeDisabled = false`: Options for product registration
- `includeDisabled = true`: List display in manufacturer management screen

---

### add_manufacturer

Adds a new manufacturer.

**Parameters:**
- `manufacturer_name` (String): Manufacturer name (required)
- `memo` (Option<String>): Memo
- `is_disabled` (Option<i64>): Disabled flag (default: 0)

**Return Value:**
- `String`: "Manufacturer added successfully"

**Usage Example:**
```javascript
await invoke('add_manufacturer', {
    manufacturerName: 'Kirin',
    memo: null,
    isDisabled: 0
});
```

**Automatic Processing:**
1. **Display order auto-assignment**: Max value + 1
2. **is_disabled**: Default 0 (if omitted)

**Validation:**
- Manufacturer name required
- Name duplicate check within same user

---

### update_manufacturer

Updates manufacturer information.

**Parameters:**
- `manufacturer_id` (i64): Manufacturer ID
- `manufacturer_name` (String): New manufacturer name
- `memo` (Option<String>): New memo
- `display_order` (i64): New display order
- `is_disabled` (i64): Disabled flag

**Return Value:**
- `String`: "Manufacturer updated successfully"

**Usage Example:**
```javascript
await invoke('update_manufacturer', {
    manufacturerId: 1,
    manufacturerName: 'Kirin Beverage',
    memo: 'Updated memo',
    displayOrder: 1,
    isDisabled: 0
});
```

---

### delete_manufacturer

Logically deletes a manufacturer.

**Parameters:**
- `manufacturer_id` (i64): Manufacturer ID

**Return Value:**
- `String`: "Manufacturer deleted successfully"

**Usage Example:**
```javascript
await invoke('delete_manufacturer', { manufacturerId: 1 });
```

**Behavior:**
- Logical deletion (`IS_DISABLED = 1`)
- Related products are NOT deleted
- Product's `manufacturer_id` retained but name becomes `null` on retrieval

**Note:**
- Deleting manufacturer doesn't delete products referencing it
- Manufacturer name not displayed in product list (LEFT JOIN returns null)

---

## Product Management API

### get_products

Retrieves the list of products (with manufacturer name).

**Parameters:**
- `include_disabled` (bool): Whether to include disabled

**Return Value:**
- `Vec<Product>`: Array of products

**Product Structure:**
```javascript
{
    product_id: number,
    user_id: number,
    product_name: string,
    manufacturer_id: number | null,
    manufacturer_name: string | null,  // Retrieved by LEFT JOIN
    memo: string | null,
    display_order: number,
    is_disabled: number,
    entry_dt: string,
    update_dt: string | null
}
```

**Usage Example:**
```javascript
const products = await invoke('get_products', { 
    includeDisabled: false 
});

products.forEach(product => {
    const maker = product.manufacturer_name || '(Unknown manufacturer)';
    console.log(`${product.product_name} - ${maker}`);
});
```

**Manufacturer Name Retrieval:**
- LEFT JOIN with MANUFACTURERS table
- Manufacturer not set → `manufacturer_name = null`
- Manufacturer deleted → `manufacturer_name = null`

---

### add_product

Adds a new product.

**Parameters:**
- `product_name` (String): Product name (required)
- `manufacturer_id` (Option<i64>): Manufacturer ID
- `memo` (Option<String>): Memo
- `is_disabled` (Option<i64>): Disabled flag (default: 0)

**Return Value:**
- `String`: "Product added successfully"

**Usage Example:**
```javascript
await invoke('add_product', {
    productName: 'Kirin Ichiban Shibori',
    manufacturerId: 1,
    memo: '350ml can',
    isDisabled: 0
});

// Can also be added without manufacturer
await invoke('add_product', {
    productName: 'No-brand product',
    manufacturerId: null,
    memo: null,
    isDisabled: 0
});
```

**Automatic Processing:**
- Display order auto-assignment
- is_disabled default value assignment

**Validation:**
- Product name required
- Name duplicate check within same user

---

### update_product

Updates product information.

**Parameters:**
- `product_id` (i64): Product ID
- `product_name` (String): New product name
- `manufacturer_id` (Option<i64>): New manufacturer ID
- `memo` (Option<String>): New memo
- `display_order` (i64): New display order
- `is_disabled` (i64): Disabled flag

**Return Value:**
- `String`: "Product updated successfully"

**Usage Example:**
```javascript
await invoke('update_product', {
    productId: 1,
    productName: 'Kirin Ichiban Shibori (Updated)',
    manufacturerId: 2,  // Change manufacturer
    memo: '500ml can',
    displayOrder: 1,
    isDisabled: 0
});
```

---
### delete_product

Logically deletes a product.

**Parameters:**
- `product_id` (i64): Product ID

**Return Value:**
- `String`: "Product deleted successfully"

**Usage Example:**
```javascript
await invoke('delete_product', { productId: 1 });
```

**Behavior:**
- Logical deletion (`IS_DISABLED = 1`)
- Can be deleted even if in use by transaction details

---

## Data Structures

### Shop

```rust
pub struct Shop {
    pub shop_id: i64,
    pub user_id: i64,
    pub shop_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

---

### Manufacturer

```rust
pub struct Manufacturer {
    pub manufacturer_id: i64,
    pub user_id: i64,
    pub manufacturer_name: String,
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

---

### Product

```rust
pub struct Product {
    pub product_id: i64,
    pub user_id: i64,
    pub product_name: String,
    pub manufacturer_id: Option<i64>,
    pub manufacturer_name: Option<String>,  // Retrieved by LEFT JOIN
    pub memo: Option<String>,
    pub display_order: i64,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

**Cases where manufacturer_name is null:**
1. `manufacturer_id` is `null` (manufacturer not set)
2. Manufacturer is logically deleted (`IS_DISABLED = 1`)

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"User not authenticated"` | Session not authenticated | Login required |
| `"Shop name cannot be empty"` | Shop name is empty | Enter shop name |
| `"Shop name 'XXX' already exists"` | Name duplicate | Use different name |
| `"Manufacturer name 'XXX' already exists"` | Name duplicate | Use different name |
| `"Product name 'XXX' already exists"` | Name duplicate | Use different name |
| `"Failed to add ...: ..."` | Database error | Check database |

### Frontend Error Handling Example

```javascript
// Add shop
async function addShop(name, memo) {
    try {
        await invoke('add_shop', {
            shopName: name,
            memo
        });
        
        alert('Shop added');
        return true;
    } catch (error) {
        if (error.includes('already exists')) {
            alert('This shop name is already in use');
        } else if (error.includes('cannot be empty')) {
            alert('Please enter shop name');
        } else {
            alert(`Error: ${error}`);
        }
        return false;
    }
}

// Add product (with manufacturer)
async function addProductWithManufacturer(productName, manufacturerId, memo) {
    try {
        await invoke('add_product', {
            productName,
            manufacturerId,
            memo,
            isDisabled: 0
        });
        
        alert('Product added');
        return true;
    } catch (error) {
        alert(`Error: ${error}`);
        return false;
    }
}
```

---

## Utilizing IS_DISABLED Functionality

### Benefits of Logical Deletion

1. **Data Preservation**: Protects past transaction history
2. **Reactivation**: Can restore if deleted by mistake
3. **Audit Trail**: Maintains deletion history

### Show/Hide Toggle

```javascript
// Implement toggle button in management screen
async function toggleShowDisabled() {
    const showDisabled = document.getElementById('show-disabled-toggle').checked;
    
    const manufacturers = await invoke('get_manufacturers', {
        includeDisabled: showDisabled
    });
    
    renderManufacturerList(manufacturers);
}
```

### Reactivation Example

```javascript
// Reactivate a logically deleted manufacturer
async function reactivateManufacturer(manufacturerId) {
    try {
        // Get current information
        const manufacturers = await invoke('get_manufacturers', { 
            includeDisabled: true 
        });
        const manufacturer = manufacturers.find(m => m.manufacturer_id === manufacturerId);
        
        if (!manufacturer) {
            alert('Manufacturer not found');
            return;
        }
        
        // Update is_disabled to 0
        await invoke('update_manufacturer', {
            manufacturerId,
            manufacturerName: manufacturer.manufacturer_name,
            memo: manufacturer.memo,
            displayOrder: manufacturer.display_order,
            isDisabled: 0  // Reactivate
        });
        
        alert('Manufacturer reactivated');
        await loadManufacturers();
    } catch (error) {
        alert(`Error: ${error}`);
    }
}
```

---

## Usage Example: Master Data Management Screen Implementation

### Shop List Display

```javascript
async function loadShops() {
    try {
        const shops = await invoke('get_shops');
        
        const tbody = document.getElementById('shop-table-body');
        tbody.innerHTML = '';
        
        shops.forEach(shop => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td>${shop.shop_id}</td>
                <td>${shop.shop_name}</td>
                <td>${shop.memo || '-'}</td>
                <td>
                    <button onclick="editShop(${shop.shop_id})">Edit</button>
                    <button onclick="deleteShop(${shop.shop_id})">Delete</button>
                </td>
            `;
            tbody.appendChild(row);
        });
    } catch (error) {
        console.error('Shop list loading error:', error);
    }
}
```

### Add Product Form (with Manufacturer Selection)

```javascript
async function initializeProductForm() {
    // Load manufacturer options
    const manufacturers = await invoke('get_manufacturers', { 
        includeDisabled: false 
    });
    
    const select = document.getElementById('manufacturer-select');
    select.innerHTML = '<option value="">(No manufacturer)</option>';
    
    manufacturers.forEach(m => {
        const option = document.createElement('option');
        option.value = m.manufacturer_id;
        option.textContent = m.manufacturer_name;
        select.appendChild(option);
    });
}

async function handleAddProduct(event) {
    event.preventDefault();
    
    const productName = document.getElementById('product-name').value;
    const manufacturerId = document.getElementById('manufacturer-select').value || null;
    const memo = document.getElementById('memo').value || null;
    
    try {
        await invoke('add_product', {
            productName,
            manufacturerId: manufacturerId ? parseInt(manufacturerId) : null,
            memo,
            isDisabled: 0
        });
        
        alert('Product added');
        event.target.reset();
        await loadProducts();
    } catch (error) {
        alert(`Error: ${error}`);
    }
}
```

---

## Data Relationships

### Manufacturer-Product Relationship

```
MANUFACTURERS (Manufacturers)
    ↓ 1-to-many
PRODUCTS (Products)
    ↓ Reference
TRANSACTION_DETAILS (Details)
```

**Deletion Behavior:**
1. **Manufacturer deletion**: Products not deleted (manufacturer_id retained)
2. **Product deletion**: Details not deleted (product_id retained)

### Shop-Transaction Relationship

```
SHOPS (Shops)
    ↓ Reference
TRANSACTION_HEADERS (Headers)
```

**Deletion Behavior:**
- Transactions retained after shop deletion
- Shop name remains as historical record

---

## Test Coverage

**ShopService:**
- ✅ Shop list retrieval test
- ✅ Shop addition test
- ✅ Shop update test
- ✅ Shop deletion test
- ✅ Name duplicate check

**ManufacturerService:**
- ✅ Manufacturer list retrieval test (include_disabled)
- ✅ Manufacturer addition test
- ✅ Manufacturer update test
- ✅ Manufacturer deletion test
- ✅ Name duplicate check

**ProductService:**
- ✅ Product list retrieval test (with manufacturer name)
- ✅ Product addition test
- ✅ Product update test
- ✅ Product deletion test
- ✅ Manufacturer relationship test
- ✅ Name duplicate check

---

## Related Documents

### Implementation Files

- Shop Service: `src/services/shop.rs`
- Manufacturer Service: `src/services/manufacturer.rs`
- Product Service: `src/services/product.rs`
- SQL Definitions: `src/sql_queries.rs`
- Tauri Commands: `src/lib.rs`

### Other API References

- [Common API](./API_COMMON.md) - Session management
- [Transaction Management API](./API_TRANSACTION.md) - Usage of shops and products

### Guide Documents

- [IS_DISABLED Implementation Guide](../guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md) - Logical deletion details

---

**Change History:**
- 2025-12-05: Created (based on implementation code, Shop/Manufacturer/Product integrated)
