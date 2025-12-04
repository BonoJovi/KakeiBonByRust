# Account Management API Reference

**Last Updated**: 2025-12-05 02:32 JST

## Overview

This document defines APIs used in the account management screen (account-management.html). Provides functionality for users to manage multiple accounts (cash, bank accounts, etc.).

---

## Table of Contents

1. [Account Template API](#account-template-api)
2. [Account Management API](#account-management-api)
3. [Data Structures](#data-structures)

---

## Account Template API

### get_account_templates

Retrieves the list of account templates.

**Parameters:** None

**Return Value:**
- `Vec<AccountTemplate>`: Array of templates

**AccountTemplate Structure:**
```javascript
{
    template_id: number,
    template_code: string,      // "CASH", "BANK", "CREDIT_CARD", etc.
    template_name_ja: string,   // "現金", "銀行口座", "クレジットカード"
    template_name_en: string,   // "Cash", "Bank Account", "Credit Card"
    display_order: number,
    entry_dt: string
}
```

**Usage Example:**
```javascript
const templates = await invoke('get_account_templates');

// Create dropdown for template selection
templates.forEach(template => {
    const option = document.createElement('option');
    option.value = template.template_code;
    option.textContent = template.template_name_en;
    selectElement.appendChild(option);
});
```

**Purpose:**
- Template selection when creating accounts
- Determining display icons based on template

**Note:**
- Templates are fixed data (cannot be added/edited by users)
- Retrieved from ACCOUNT_TEMPLATES table

---

## Account Management API

### get_accounts

Retrieves the list of accounts for the logged-in user.

**Parameters:** None

**Return Value:**
- `Vec<Account>`: Array of accounts

**Account Structure:**
```javascript
{
    account_id: number,
    user_id: number,
    account_code: string,       // "CASH", "MIZUHO_001", etc.
    account_name: string,       // "Cash", "Mizuho Bank Savings"
    template_code: string,      // "CASH", "BANK", etc.
    initial_balance: number,    // Initial balance
    display_order: number,
    is_disabled: number,        // 0=active, 1=disabled
    entry_dt: string,
    update_dt: string | null
}
```

**Usage Example:**
```javascript
const accounts = await invoke('get_accounts');

accounts.forEach(account => {
    console.log(`${account.account_name}: ¥${account.initial_balance}`);
});
```

**Role-based Behavior:**
- **Admin (role=0)**: Retrieves all users' accounts
- **General User (role=1)**: Retrieves only own accounts

**Filter:**
- Only retrieves where `IS_DISABLED = 0` (excludes logically deleted)

**Sort:**
- By `DISPLAY_ORDER`, `ACCOUNT_CODE`

---

### add_account

Adds a new account.

**Parameters:**
- `account_code` (String): Account code (auto-converted to uppercase)
- `account_name` (String): Account name
- `template_code` (String): Template code
- `initial_balance` (i64): Initial balance

**Return Value:**
- `String`: "Account added successfully"

**Usage Example:**
```javascript
try {
    await invoke('add_account', {
        accountCode: 'MIZUHO_001',
        accountName: 'Mizuho Bank Savings',
        templateCode: 'BANK',
        initialBalance: 100000
    });
    
    alert('Account added');
    await loadAccounts();
} catch (error) {
    alert(`Addition failed: ${error}`);
}
```

**Automatic Processing:**
1. **Code normalization**: Lowercase→uppercase, trim
2. **Display order auto-assignment**: Max value + 1
3. **is_disabled**: Set to 0 (active)

**Validation:**
- Account code duplicate check (within same user)
- Account name required

**Errors:**
- `"Account code 'XXX' already exists"`: Code duplicate
- `"Failed to add account: ..."`: Database error

---

### update_account

Updates account information.

**Parameters:**
- `account_code` (String): Account code (immutable, for identification)
- `account_name` (String): New account name
- `template_code` (String): New template code
- `initial_balance` (i64): New initial balance
- `display_order` (i64): New display order

**Return Value:**
- `String`: "Account updated successfully"

**Usage Example:**
```javascript
await invoke('update_account', {
    accountCode: 'MIZUHO_001',
    accountName: 'Mizuho Bank Savings (Updated)',
    templateCode: 'BANK',
    initialBalance: 150000,
    displayOrder: 2
});
```

**Note:**
- `account_code` cannot be changed (primary key)
- All other fields can be updated

**Validation:**
- Duplicate account code check excluding self

---

### delete_account

Logically deletes an account.

**Parameters:**
- `account_code` (String): Account code

**Return Value:**
- `String`: "Account deleted successfully"

**Usage Example:**
```javascript
if (confirm('Are you sure you want to delete this account?')) {
    try {
        await invoke('delete_account', { accountCode: 'MIZUHO_001' });
        alert('Account deleted');
        await loadAccounts();
    } catch (error) {
        alert(`Deletion failed: ${error}`);
    }
}
```

**Behavior:**
- Logical deletion (`IS_DISABLED = 1`)
- Not physical deletion

**Note:**
- Accounts in use by transactions can also be deleted
- Transaction history is retained after deletion

---

## Data Structures

### AccountTemplate

```rust
pub struct AccountTemplate {
    pub template_id: i64,
    pub template_code: String,        // "CASH", "BANK", "CREDIT_CARD"...
    pub template_name_ja: String,     // Japanese name
    pub template_name_en: String,     // English name
    pub display_order: i64,
    pub entry_dt: String,
}
```

**Standard Templates (Example):**
- CASH: 現金 / Cash
- BANK: 銀行口座 / Bank Account
- CREDIT_CARD: クレジットカード / Credit Card
- E_MONEY: 電子マネー / Electronic Money
- SECURITIES: 証券口座 / Securities Account

---

### Account

```rust
pub struct Account {
    pub account_id: i64,
    pub user_id: i64,
    pub account_code: String,       // Uppercase (auto-normalized)
    pub account_name: String,
    pub template_code: String,      // Template reference
    pub initial_balance: i64,       // Initial balance
    pub display_order: i64,
    pub is_disabled: i64,           // 0=active, 1=disabled
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

**account_code Naming Convention (Recommended):**
- CASH: Cash
- {bank_name}_{number}: e.g., MIZUHO_001, UFJ_002
- CREDIT_{card_name}: e.g., CREDIT_RAKUTEN

---

### AddAccountRequest

```rust
pub struct AddAccountRequest {
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
}
```

---

### UpdateAccountRequest

```rust
pub struct UpdateAccountRequest {
    pub account_code: String,       // For identification (immutable)
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
    pub display_order: i64,
}
```

---

## Error Handling

### Common Error Patterns

| Error Message | Cause | Solution |
|--------------|-------|----------|
| `"User not authenticated"` | Session not authenticated | Login required |
| `"Account code 'XXX' already exists"` | Code duplicate | Use different code |
| `"Failed to get account templates: ..."` | DB connection error | Check database |
| `"Failed to add account: ..."` | Addition error | Check database |
| `"Failed to update account: ..."` | Update error | Check database |
| `"Failed to delete account: ..."` | Deletion error | Check database |

### Frontend Error Handling Example

```javascript
// Add account
async function addAccount(code, name, templateCode, initialBalance) {
    try {
        await invoke('add_account', {
            accountCode: code,
            accountName: name,
            templateCode,
            initialBalance
        });
        
        alert('Account added');
        return true;
    } catch (error) {
        if (error.includes('already exists')) {
            alert('This account code is already in use');
        } else {
            alert(`Error: ${error}`);
        }
        return false;
    }
}

// Update account
async function updateAccount(code, name, templateCode, initialBalance, displayOrder) {
    try {
        await invoke('update_account', {
            accountCode: code,
            accountName: name,
            templateCode,
            initialBalance,
            displayOrder
        });
        
        alert('Account updated');
        return true;
    } catch (error) {
        alert(`Update error: ${error}`);
        return false;
    }
}
```

---

## Account Code Normalization

### normalize_account_code Function

Account codes are automatically normalized:

```rust
fn normalize_account_code(code: &str) -> String {
    code.trim().to_uppercase()
}
```

**Examples:**
- Input: `"mizuho_001"` → Saved: `"MIZUHO_001"`
- Input: `" cash "` → Saved: `"CASH"`
- Input: `"Ufj_002"` → Saved: `"UFJ_002"`

**Reasons:**
- Ensure code consistency
- Improve duplicate check accuracy
- Prevent duplicates due to case differences

---

## Usage Example: Account Management Screen Implementation

### Account List Display

```javascript
async function loadAccounts() {
    try {
        const accounts = await invoke('get_accounts');
        const templates = await invoke('get_account_templates');
        
        // Map to get template name from code
        const templateMap = new Map();
        templates.forEach(t => {
            templateMap.set(t.template_code, t.template_name_en);
        });
        
        const tbody = document.getElementById('account-table-body');
        tbody.innerHTML = '';
        
        accounts.forEach(account => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td>${account.account_code}</td>
                <td>${account.account_name}</td>
                <td>${templateMap.get(account.template_code) || account.template_code}</td>
                <td>¥${account.initial_balance.toLocaleString()}</td>
                <td>
                    <button onclick="editAccount('${account.account_code}')">Edit</button>
                    <button onclick="deleteAccount('${account.account_code}')">Delete</button>
                </td>
            `;
            tbody.appendChild(row);
        });
    } catch (error) {
        console.error('Account list loading error:', error);
    }
}
```

### Add Account Form

```javascript
async function handleAddAccount(event) {
    event.preventDefault();
    
    const code = document.getElementById('account-code').value;
    const name = document.getElementById('account-name').value;
    const templateCode = document.getElementById('template-select').value;
    const balance = parseInt(document.getElementById('initial-balance').value);
    
    try {
        await invoke('add_account', {
            accountCode: code,
            accountName: name,
            templateCode,
            initialBalance: balance
        });
        
        alert('Account added');
        event.target.reset();
        await loadAccounts();
    } catch (error) {
        alert(`Error: ${error}`);
    }
}
```

---

## Security Considerations

### User Isolation

1. **Session User ID**: Auto-retrieved in each API
2. **Data Isolation**: General users can only access their own accounts
3. **Admin Permission**: Admins can view all users' accounts

### Code Uniqueness

1. **Per User**: Code cannot be duplicated within same user
2. **Different Users**: Same code can be used
3. **Uppercase Normalization**: Prevents duplicates due to case differences

### Logical Deletion

1. **Data Retention**: Data retained after deletion
2. **Transaction Integrity**: Past transactions protected
3. **Reactivation**: Can be restored by setting IS_DISABLED=0 if needed

---

## Test Coverage

**AccountService:**
- ✅ Template retrieval test
- ✅ Account list retrieval test (per user)
- ✅ Account addition test
- ✅ Account update test
- ✅ Account deletion test
- ✅ Code duplicate check
- ✅ Code normalization test

---

## Related Documents

### Implementation Files

- Account Service: `src/services/account.rs`
- SQL Definitions: `src/sql_queries.rs`
- Tauri Commands: `src/lib.rs`

### Other API References

- [Common API](./API_COMMON.md) - Session management
- [Transaction Management API](./API_TRANSACTION.md) - Account usage

---

**Change History:**
- 2025-12-05: Created (based on implementation code)
