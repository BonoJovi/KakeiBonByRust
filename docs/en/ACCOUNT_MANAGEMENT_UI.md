# Account Management Screen Implementation Document

## Overview

This document records the implementation details of KakeiBon's Account Management screen.

**Implementation Period**: 2025-11-05 ～ 2025-11-07  
**Last Updated**: 2025-11-08 07:40 JST

---

## Feature Overview

### Purpose of Account Management
- Manage bank accounts, credit cards, electronic money, etc. used by users
- Enable account selection when registering income/expense transactions
- Create accounts based on templates and customize actual account names

### Account Data Structure

#### Table Structure
1. **ACCOUNT_TEMPLATES** (Master Data)
   - Common templates for all users
   - TEMPLATE_CODE: NONE, CASH, BANK, CREDIT, EMONEY, OTHER
   - Read-only

2. **ACCOUNTS** (User-specific Accounts)
   - Managed by `USER_ID` + `ACCOUNT_CODE`
   - References `TEMPLATE_CODE` for template
   - Users can add, edit, and delete

#### Special Treatment of "Unspecified" Account
- **ACCOUNT_CODE**: `NONE`
- **Purpose**: Handle irregular transactions (transactions without specific account)
- **Auto-creation**: Automatically added when user is created
- **Cannot be deleted**: Delete button is grayed out
- **Editable**: Name customization is allowed

---

## Implementation Details

### 1. User Filtering Feature

#### Requirements
- **Admin (ROLE_ADMIN)**: Display all users' accounts
- **Regular User (ROLE_USER)**: Display only own accounts

#### Backend Implementation
```rust
// src/lib.rs
#[tauri::command]
async fn get_accounts(
    user_id: i64,
    user_role: i64,
    state: tauri::State<'_, AppState>
) -> Result<Vec<services::account::Account>, String> {
    let db = state.db.lock().await;
    
    // Admin can see all accounts, regular users see only their own
    if user_role == crate::consts::ROLE_ADMIN {
        services::account::get_all_accounts(db.pool()).await
    } else {
        services::account::get_accounts(db.pool(), user_id).await
    }
}
```

#### Frontend Implementation
```javascript
// res/js/account-management.js
const currentUserId = 1;  // TODO: Get from session/auth
const currentUserRole = ROLE_ADMIN;  // TODO: Get from session/auth

async function loadAccounts() {
    accounts = await invoke('get_accounts', {
        userId: currentUserId,
        userRole: currentUserRole
    });
    // ...
}
```

**Temporary Workaround**:
- `currentUserId` and `currentUserRole` are hardcoded as constants
- Must manually change values for testing
- Will be improved after session management implementation

### 2. NONE Account Auto-creation

#### Backend Implementation
```rust
// src/services/account.rs
pub async fn initialize_none_account(pool: &SqlitePool, user_id: i64) -> Result<(), String> {
    // Get NONE template
    let none_template = sqlx::query_as::<_, AccountTemplate>(
        "SELECT * FROM ACCOUNT_TEMPLATES WHERE TEMPLATE_CODE = 'NONE'"
    )
    .fetch_one(pool)
    .await?;
    
    // Create NONE account
    let request = AddAccountRequest {
        account_code: "NONE".to_string(),
        account_name: none_template.template_name_ja.clone(),
        template_code: "NONE".to_string(),
        initial_balance: 0,
    };
    
    add_account(pool, user_id, request).await
}
```

```rust
// src/services/auth.rs
impl AuthService {
    pub async fn create_general_user(&self, name: &str, password: &str) -> Result<(), AuthError> {
        // ...existing code...
        
        // Initialize NONE account for the new user
        crate::services::account::initialize_none_account(&self.pool, next_id).await?;
        
        Ok(())
    }
}
```

### 3. NONE Account Delete Prevention

#### Frontend Implementation
```javascript
// res/js/account-management.js
function createAccountRow(account) {
    // NONE account cannot be deleted
    const isNoneAccount = account.account_code === 'NONE';
    const deleteButtonHtml = isNoneAccount 
        ? '<button class="btn btn-danger" disabled style="opacity: 0.5; cursor: not-allowed;">Delete</button>'
        : `<button class="btn btn-danger delete-btn" data-code="${escapeHtml(account.account_code)}">Delete</button>`;
    
    // ...render row...
    
    // Delete button (only if not NONE account)
    if (!isNoneAccount) {
        row.querySelector('.delete-btn').addEventListener('click', () => {
            deleteAccount(account.account_code, account.account_name);
        });
    }
    
    return row;
}
```

### 4. Account CRUD Operations

#### Add
```javascript
await invoke('add_account', {
    userId: currentUserId,
    accountCode: accountCode,
    accountName: accountName,
    templateCode: templateCode,
    initialBalance: initialBalance
});
```

#### Update
```javascript
await invoke('update_account', {
    userId: currentUserId,
    accountCode: accountCode,
    accountName: accountName,
    templateCode: templateCode,
    initialBalance: initialBalance,
    displayOrder: displayOrder
});
```

#### Delete
```javascript
await invoke('delete_account', { 
    userId: currentUserId,
    accountCode: accountCode 
});
```

---

## File Structure

### Frontend
- `res/account-management.html`: Screen HTML
- `res/js/account-management.js`: Business logic
- `res/css/account-management.css`: Style definitions (uses common CSS)

### Backend
- `src/services/account.rs`: Account-related business logic
- `src/services/auth.rs`: NONE account creation on user creation
- `src/lib.rs`: Tauri command definitions

### Database
- `sql/create_accounts_table.sql`: Table definitions
- `sql/fix_transactions_foreign_keys.sql`: Foreign key constraint fixes
- `sql/insert_default_accounts_for_existing_users.sql`: NONE account addition for existing users

---

## Key Functions

### Backend (Rust)
```rust
// Get account list (with user filter)
pub async fn get_accounts(pool: &SqlitePool, user_id: i64) -> Result<Vec<Account>, String>

// Get all accounts (for admin)
pub async fn get_all_accounts(pool: &SqlitePool) -> Result<Vec<Account>, String>

// Add account
pub async fn add_account(pool: &SqlitePool, user_id: i64, request: AddAccountRequest) -> Result<String, String>

// Update account
pub async fn update_account(pool: &SqlitePool, user_id: i64, request: UpdateAccountRequest) -> Result<String, String>

// Delete account
pub async fn delete_account(pool: &SqlitePool, user_id: i64, account_code: &str) -> Result<String, String>

// Initialize NONE account
pub async fn initialize_none_account(pool: &SqlitePool, user_id: i64) -> Result<(), String>

// Get template list
pub async fn get_account_templates(pool: &SqlitePool) -> Result<Vec<AccountTemplate>, String>
```

### Frontend (JavaScript)
```javascript
// Load account list
async function loadAccounts()

// Load templates
async function loadTemplates()

// Modal open/close
function openModal(mode, account = null)
function closeModal()

// Create account row
function createAccountRow(account)

// Save operation
async function saveAccount()

// Delete operation
async function deleteAccount(accountCode, accountName)
```

---

## Data Structures

### AccountTemplate
```rust
pub struct AccountTemplate {
    pub template_id: i64,
    pub template_code: String,
    pub template_name_ja: String,
    pub template_name_en: String,
    pub display_order: Option<i64>,
    pub entry_dt: String,
}
```

### Account
```rust
pub struct Account {
    pub account_id: i64,
    pub user_id: i64,
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
    pub display_order: Option<i64>,
    pub is_disabled: i64,
    pub entry_dt: String,
    pub update_dt: Option<String>,
}
```

### AddAccountRequest
```rust
pub struct AddAccountRequest {
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
}
```

### UpdateAccountRequest
```rust
pub struct UpdateAccountRequest {
    pub account_code: String,
    pub account_name: String,
    pub template_code: String,
    pub initial_balance: i64,
    pub display_order: i64,
}
```

---

## Testing Methods

### Test as Admin
```javascript
// res/js/account-management.js
const currentUserId = 1;
const currentUserRole = ROLE_ADMIN;
```
**Expected Behavior**: All users' accounts are displayed

### Test as Regular User
```javascript
// res/js/account-management.js
const currentUserId = 2;
const currentUserRole = ROLE_USER;
```
**Expected Behavior**: Only USER_ID=2's accounts are displayed

### NONE Account Delete Prevention Test
1. Check "Unspecified" account in account list
2. Verify delete button is grayed out
3. Verify no response when clicked

### New User Creation Test
1. Login as admin
2. Create new user in user management screen
3. Login as new user
4. Open account management screen
5. Verify "Unspecified" account exists automatically

---

## Known Limitations

### Session Management Not Implemented
- **Current**: `currentUserId` and `currentUserRole` are hardcoded
- **Impact**: Manual code changes required for testing
- **Planned**: Implement session management to automatically retrieve logged-in user info

### Account Code Duplicate Check
- **Current**: UNIQUE constraint in backend but no frontend pre-check
- **Impact**: Error dialog shown on duplicate
- **Improvement**: Add real-time validation

### Template Selection Constraints
- **Current**: All templates are selectable
- **Improvement**: Implement per-template constraints (e.g., only one NONE)

---

## Future Improvements

### Phase 2: Session Management
- Session management using localStorage/sessionStorage
- Login state persistence
- Auto-logout feature

### Phase 3: Enhanced Validation
- Real-time duplicate check for account codes
- Character length check for account names
- Initial balance validity check

### Phase 4: UI/UX Improvements
- Drag & drop account reordering
- Account grouping feature (display only)
- Account enable/disable toggle

### Phase 5: Import/Export
- Export account data in CSV format
- Import account data in CSV format

---

## Related Documents

- [Account Management API Specification](./API_ACCOUNT.md)
- [Database Design](./DATABASE_CONFIGURATION.md)
- [Troubleshooting](./TROUBLESHOOTING.md)

---

**Last Updated**: 2025-11-08 07:40 JST

**Author**: AI Assistant  
**Supervisor**: Yoshihiro NAKAHARA (bonojovi@zundou.org)
