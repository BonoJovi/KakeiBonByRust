# AI Context: Coding Conventions

**Purpose**: Coding standards and patterns for AI assistants to follow.
**Last Updated**: 2025-11-09

---

## General Principles

1. **DRY (Don't Repeat Yourself)**: Use common modules for shared logic
2. **Consistency**: Backend and frontend must use same validation rules
3. **Type Safety**: Use Rust's type system, avoid `any` in TypeScript (if added)
4. **Error Handling**: Always handle errors explicitly
5. **Documentation**: Update AI context docs when adding major features

## File Modification Guidelines

### ❌ PROHIBITED: Full File Rewrite
- **Never rewrite an entire file** when making changes
- **Exception**: Only when explicitly required by the nature of the change
- **Reason**: Minimizes risk of unintended changes and merge conflicts

### ✅ RECOMMENDED: Surgical Edits
- **Always modify only the necessary parts** of files
- Use targeted edits to change specific lines or blocks
- Preserve surrounding code and context unchanged
- **Benefits**: 
  - Clearer change history in version control
  - Reduced risk of introducing bugs
  - Easier code review process
  - Better merge conflict handling

## AI/LLM Specific Guidelines

### Documentation Timestamps
- **User-facing documentation**: Always use Japan Standard Time (JST, UTC+9)
- **AI context documentation**: Can use UTC or JST
- **Example**: `Last Updated: 2024-10-26 13:21 JST`

### Git Commit Messages
- **Language**: Always write in English
- **Format**: Follow conventional commits format
- **Example**: `feat(user-mgmt): add user deletion feature`

### Git Operations
- **AI/LLM scope**: Up to `git commit` only
- **Manual operation**: `git push` must be done by developer
- **Reason**: GitHub operations require hardware key authentication
- **Workflow**:
  1. AI/LLM creates/modifies files
  2. AI/LLM stages changes: `git add`
  3. AI/LLM commits: `git commit -m "message"`
  4. **Developer pushes**: `git push` (manual, with hardware key)

### Example Workflow
```bash
# AI/LLM can do:
git add src/new-feature.rs
git commit -m "feat(feature): add new feature implementation"

# Developer must do:
git push origin main  # Requires hardware key authentication
```

### Session Limit Notification
- **Threshold**: 90% of session request budget
- **Action**: Notify user when request count exceeds 90%
- **Message format**: Clear notification stating current usage percentage
- **Purpose**: Allow user to save work or prepare for session end
- **Example**: "⚠️ Session limit notice: Currently at 92% of request budget (46/50 requests used)"

---

## Database File Naming

### ❌ FORBIDDEN: Using "kakeibo.db" as Database Filename
- **Never use**: `kakeibo.db` as the database filename
- **Correct filename**: `KakeiBonDB.sqlite3` (defined in `src/consts.rs`)
- **Location**: `~/.kakeibon/KakeiBonDB.sqlite3`
- **Access script**: Use `./db.sh` to open the database with sqlite3
- **Reason**: Incorrect database names cause confusion and data inconsistency

### Database Access
```bash
# ✓ Correct - Use the db.sh script
./db.sh "SELECT * FROM USERS;"

# ✗ Wrong - Don't specify filename manually
sqlite3 work/kakeibo.db "SELECT * FROM USERS;"
sqlite3 ~/.local/share/kakeibo/kakeibo.db "SELECT * FROM USERS;"
```

---

## SQL Management

### SQL Queries Centralization
- **All SQL queries**: Must be defined as constants in `src/sql_queries.rs`
- **No hardcoded SQL**: SQL strings must not be embedded directly in service/command code
- **Naming convention**: 
  - Production queries: `{SCOPE}_{ACTION}` (e.g., `CATEGORY2_UPDATE`, `USER_INSERT`)
  - Test queries: Prefix with `TEST_` (e.g., `TEST_CATEGORY_GET_CATEGORY2_NAME`)
- **Benefits**: 
  - Single source of truth for all SQL
  - Easier maintenance and testing
  - Prevents SQL injection via consistent parameterization
  - Better SQL review and optimization

### Example
```rust
// In src/sql_queries.rs
pub const CATEGORY2_UPDATE: &str = r#"
UPDATE CATEGORY2 
SET CATEGORY2_NAME = ?, UPDATE_DT = datetime('now') 
WHERE USER_ID = ? AND CATEGORY1_CODE = ? AND CATEGORY2_CODE = ?
"#;

// In service code
use crate::sql_queries::CATEGORY2_UPDATE;
conn.execute(CATEGORY2_UPDATE, params![name, user_id, cat1, cat2])?;
```

---

## Rust Conventions

### Naming
- **Functions**: `snake_case` (e.g., `verify_login`)
- **Structs**: `PascalCase` (e.g., `UserInfo`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g., `ROLE_ADMIN`)
- **Modules**: `snake_case` (e.g., `user_management`)

### Error Handling
```rust
// ✓ Good - explicit Result type
pub async fn get_user(id: i64) -> Result<UserInfo, UserManagementError> {
    // ...
}

// ✗ Bad - unwrap in production code
pub async fn get_user(id: i64) -> UserInfo {
    db.query().await.unwrap()  // Don't do this
}
```

### Tauri Commands
```rust
// Pattern: #[tauri::command] + pub async fn + Result
#[tauri::command]
pub async fn create_general_user(
    username: String,
    password: String,
    state: tauri::State<'_, AppState>,
) -> Result<i64, String> {
    // Implementation
    service
        .register_general_user(&username, &password)
        .await
        .map_err(|e| e.to_string())  // Convert to String for Tauri
}
```

### Database Queries

#### SQL Constants (IMPORTANT!)
⚠️ **ALL SQL queries must be defined as constants in `src/sql_queries.rs`**

**Rules:**
1. Never hardcode SQL in business logic files
2. Define all SQL as constants in `src/sql_queries.rs`
3. Use descriptive names based on functionality (not numeric IDs)
4. Separate test SQL with `TEST_` prefix

**Example Structure:**
```rust
// src/sql_queries.rs

// Production SQL queries
pub const SELECT_USER_BY_NAME: &str = 
    "SELECT * FROM USERS WHERE NAME = ?";

pub const INSERT_CATEGORY: &str = 
    "INSERT INTO CATEGORIES (CODE, NAME_JA, NAME_EN) VALUES (?, ?, ?)";

pub const UPDATE_CATEGORY_NAME: &str = 
    "UPDATE CATEGORIES SET NAME_JA = ?, NAME_EN = ? WHERE CODE = ?";

// Test SQL queries (with TEST_ prefix)
pub const TEST_INSERT_USER: &str = 
    "INSERT INTO USERS (NAME, PAW, ROLE) VALUES (?, ?, ?)";

pub const TEST_CREATE_CATEGORIES_TABLE: &str = 
    "CREATE TABLE IF NOT EXISTS CATEGORIES (...)";
```

**Usage in Code:**
```rust
use crate::sql_queries::SELECT_USER_BY_NAME;

// ✓ Good - uses SQL constant
let user = sqlx::query(SELECT_USER_BY_NAME)
    .bind(username)
    .fetch_optional(&pool)
    .await?;

// ✗ Bad - hardcoded SQL
let user = sqlx::query("SELECT * FROM USERS WHERE NAME = ?")
    .bind(username)
    .fetch_optional(&pool)
    .await?;
```

**Benefits:**
- Central management of all SQL queries
- Easy to review and maintain
- Consistent naming across the project
- Clear separation between production and test SQL

#### Database Path Helper (IMPORTANT!)
⚠️ **Always use the `get_db_path()` helper function from `src/db.rs`**

**Database File Location:**
- Production DB: `$HOME/.kakeibon/KakeiBonDB.sqlite3`
- Constants: `DB_DIR_NAME = ".kakeibon"`, `DB_FILE_NAME = "KakeiBonDB.sqlite3"`

**Rules:**
1. Never hardcode database file paths
2. Use `get_db_path()` function (defined in `src/db.rs`) to get the correct path
3. The function handles both Unix (`$HOME`) and Windows (`$USERPROFILE`) environments

**Example Usage:**
```rust
// ✓ Good - uses helper function
use crate::db::get_db_path;
let db_path = get_db_path();
let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

// ✗ Bad - hardcoded path
let db_path = "kakeibo.db";  // Wrong!
let db_path = "/home/user/.kakeibon/KakeiBonDB.sqlite3";  // Too specific!
```

**Note:** The `get_db_path()` function is private to `src/db.rs`. For most use cases, use the `Database::new()` method which internally calls `get_db_path()`.

---

## JavaScript Conventions

### Naming
- **Functions**: `camelCase` (e.g., `handleLogin`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g., `ROLE_ADMIN`)
- **Classes**: `PascalCase` (e.g., `I18nService`)
- **Files**: `kebab-case` (e.g., `user-management.js`)

### ES Modules (Important!)
```javascript
// ✓ Good - always include .js extension
import { validatePassword } from './validation-helpers.js';

// ✗ Bad - missing extension
import { validatePassword } from './validation-helpers';
```

### Async/Await Pattern
```javascript
// ✓ Good - always use try/catch with async
async function loadUsers() {
    try {
        const users = await invoke('list_users');
        // ... handle success
    } catch (error) {
        console.error('Failed to load users:', error);
        showMessage('error', 'Failed to load users');
    }
}

// ✗ Bad - no error handling
async function loadUsers() {
    const users = await invoke('list_users');  // What if this fails?
    displayUsers(users);
}
```

### Tauri Invocations
```javascript
// Pattern: invoke('command_name', { camelCase: args })
const result = await invoke('create_general_user', {
    username: 'john',
    password: 'securepassword123456'
});
```

---

## Test Conventions

### Test Naming
```javascript
// Pattern: "should [expected behavior] when [condition]"
test('should reject password when shorter than 16 characters', () => {
    // ...
});

// ✓ Good - descriptive
test('should accept valid username with special characters', () => {});

// ✗ Bad - vague
test('test1', () => {});
```

### Test Structure (AAA Pattern)
```javascript
test('should validate correct input', () => {
    // Arrange
    const username = 'testuser';
    const password = '1234567890123456';
    
    // Act
    const result = validateUserAddition(username, password, password);
    
    // Assert
    expect(result.valid).toBe(true);
    expect(result.message).toBe('');
});
```

### Common Module Usage
```javascript
// ✓ Good - reuse common modules
import { runAllPasswordTests } from './password-validation-tests.js';
runAllPasswordTests(validatePassword, 'Screen Name');

// ✗ Bad - duplicating tests
describe('Password Tests', () => {
    test('empty password', () => { /* duplicate code */ });
    test('short password', () => { /* duplicate code */ });
    // ... 26 more duplicated tests
});
```

---

## HTML/CSS Conventions

### HTML Structure
```html
<!-- Use semantic HTML -->
<div class="section">
    <div class="section-header">
        <h2 data-i18n="section.title">Title</h2>
    </div>
    <div class="section-content">
        <!-- Content -->
    </div>
    <div class="section-footer">
        <button class="btn-primary" data-i18n="action.save">Save</button>
    </div>
</div>
```

### i18n Attributes
```html
<!-- Use data-i18n for translatable text -->
<button data-i18n="user_mgmt.add_user">Add User</button>
<label for="username" data-i18n="user_mgmt.username">Username:</label>
```

### CSS Classes
```css
/* Pattern: component-modifier */
.btn-primary { }       /* Primary button */
.btn-secondary { }     /* Secondary button */
.btn-danger { }        /* Danger button */

.section-header { }    /* Section header */
.section-content { }   /* Section content */

.message-success { }   /* Success message */
.message-error { }     /* Error message */
```

---

## Validation Conventions

### Frontend and Backend Must Match
```javascript
// Frontend: res/tests/validation-helpers.js
if (password.length < 16) {
    return { valid: false, message: 'Password must be at least 16 characters long!' };
}
```

```rust
// Backend: src/validation.rs
pub const MIN_PASSWORD_LENGTH: usize = 16;

if password.len() < MIN_PASSWORD_LENGTH {
    return Err(ValidationError::PasswordTooShort);
}
```

### Unicode Handling (IMPORTANT!)

#### JavaScript (Frontend)
- Uses `.length` property which counts UTF-16 code units
- **Most Unicode characters count as 1 character**
- Example: `"パスワード1234567890".length` = 15 characters

#### Rust (Backend)
- `.len()` returns **byte length** (UTF-8 encoding)
- `.chars().count()` returns **character count**
- **For validation, Rust uses `.len()` (bytes)**
- Example: `"パスワード1234567890".len()` = 25 bytes (not 15!)

#### Test Case Guidelines
⚠️ **When writing test cases with Unicode:**

1. **Frontend tests**: Use 16+ characters as counted by JavaScript `.length`
   ```javascript
   const password = 'パスワード12345678901'; // 16 chars in JS
   ```

2. **Rust tests**: Ensure byte length is 16+ bytes
   ```rust
   let password = "1234567890123456"; // 16 bytes (ASCII)
   let password = "パスワード1234567890123"; // 28 bytes (Unicode + numbers)
   ```

3. **Best practice for cross-platform tests**: 
   - Use ASCII characters for length boundary tests
   - Use Unicode only for character support tests (not length tests)
   - If using Unicode, ensure it's well over 16 characters/bytes

#### Example: Safe Unicode Test Cases
```javascript
// ✅ Good - uses enough characters
const password = 'パスワード12345678901'; // 16 chars in JS

// ✅ Good - ASCII for boundary test
const password = '1234567890123456'; // Exactly 16 in both JS and Rust

// ❌ Bad - Unicode at boundary
const password = 'パスワード1234567890'; // 15 chars in JS, fails!
```

### Error Messages Should Match
- Frontend: `'Password cannot be empty!'`
- Backend: `ValidationError::EmptyPassword` → "Password cannot be empty!"
- Tests verify exact string match

---

## Security Conventions

### Password Handling
```rust
// ✓ Good - hash immediately, never log
let hashed = hash_password(&password)?;
sqlx::query("INSERT INTO USERS (PAW) VALUES (?)")
    .bind(hashed)
    .execute(&pool)
    .await?;

// ✗ Bad - logging passwords
println!("User password: {}", password);  // Never do this
```

### SQL Injection Prevention
```rust
// ✓ Good - use parameter binding
sqlx::query("SELECT * FROM USERS WHERE NAME = ?")
    .bind(username)
    .fetch_optional(&pool)
    .await?;

// ✗ Bad - string concatenation
let query = format!("SELECT * FROM USERS WHERE NAME = '{}'", username);
```

---

## Documentation Conventions

### Code Comments
```rust
// Use /// for public API documentation
/// Validates a password against security requirements.
///
/// # Arguments
/// * `password` - The password to validate
///
/// # Returns
/// * `Ok(())` if valid
/// * `Err(ValidationError)` if invalid
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    // Implementation
}
```

### Inline Comments
```javascript
// Use comments sparingly - code should be self-documenting
// Only comment WHY, not WHAT

// ✓ Good - explains why
// Re-encrypt data because password changed
await reencryptData(userId, oldPassword, newPassword);

// ✗ Bad - obvious from code
// Call the login function
handleLogin();
```

### Documentation Files
- **README.md**: High-level overview, getting started
- **DESIGN.md**: Architecture, design decisions
- **API.md**: API reference (if applicable)
- **.ai-context/**: AI-specific context (not for users)

### Documentation Timestamp Format
```markdown
<!-- ✅ Good - User-facing documentation -->
Last Updated: 2024-10-26 13:21 JST

<!-- ✅ Good - AI context documentation (either format) -->
Last Updated: 2024-10-26
Last Updated: 2024-10-26 13:21 JST

<!-- ❌ Bad - User-facing with UTC -->
Last Updated: 2024-10-26 04:21 UTC  # Don't use UTC for user docs
```

---

## Git Commit Conventions

### Important: Language and Operations
- ⚠️ **All commit messages MUST be in English**
- ⚠️ **AI/LLM operations stop at `git commit`**
- ⚠️ **Never execute `git push`** - developer will do manually

### Commit Message Format
```
type(scope): short description

Detailed explanation if needed.

- Bullet points for changes
- Reference issues: Fixes #123
```

### Types
- **feat**: New feature
- **fix**: Bug fix
- **refactor**: Code refactoring
- **test**: Add/update tests
- **docs**: Documentation changes
- **style**: Code style (formatting, no logic change)
- **chore**: Maintenance tasks

### Examples (All in English)
```
feat(user-mgmt): add user addition screen with validation

- Created user-addition.test.js with 49 tests
- Refactored validation into common modules
- Updated documentation structure

refactor(tests): extract common validation tests

- Created validation-helpers.js
- Created password-validation-tests.js
- Updated admin-setup.test.js to use common modules
- All 136 tests passing

docs(ai-context): add AI-specific conventions

- Added timestamp format guidelines (JST for user docs)
- Added commit message language requirement (English only)
- Added Git push restriction for AI/LLM
```

### Git Workflow for AI/LLM
```bash
# ✅ AI/LLM CAN do these:
git status
git add <files>
git commit -m "commit message in English"
git log
git diff

# ❌ AI/LLM MUST NOT do these:
git push                    # Requires hardware key
git push origin main        # Requires hardware key
git push --force            # Requires hardware key

# After commit, AI/LLM should inform:
"Changes have been committed. Please run 'git push' manually with your hardware key."
```

---

## File Organization Conventions

### Directory Structure Rules
1. **Backend code**: Always in `src/`
2. **Frontend code**: Always in `res/`
3. **Tests**: 
   - Frontend tests: `res/tests/`
   - Backend tests: Inline with code (Rust convention)
4. **Documentation**: 
   - User docs: Project root or `docs/`
   - AI docs: `.ai-context/`

### Module Organization
```
src/services/
├── auth.rs          # Authentication (login, logout)
├── user_management.rs  # User CRUD
├── encryption.rs    # Encryption/decryption
└── i18n.rs          # Internationalization

res/js/
├── menu.js          # Main menu / admin setup
├── user-management.js  # User management UI
├── i18n.js          # i18n client
└── consts.js        # Constants
```

---

## Common Patterns to Follow

### Service Layer Pattern (Backend)
```rust
// 1. Define error types
pub enum UserManagementError { }

// 2. Define service struct
pub struct UserManagementService {
    pool: SqlitePool,
}

// 3. Implement methods
impl UserManagementService {
    pub async fn method_name(&self) -> Result<T, Error> { }
}
```

### UI Component Pattern (Frontend)
```javascript
// 1. Setup event listeners
function setupEventListeners() { }

// 2. API calls
async function loadData() { }

// 3. UI updates
function updateUI(data) { }

// 4. Initialize on DOM ready
document.addEventListener('DOMContentLoaded', async () => {
    setupEventListeners();
    await loadData();
});
```

### Test Module Pattern
```javascript
// 1. Import common modules
import { validateX } from './validation-helpers.js';
import { runAllTests } from './common-tests.js';

// 2. Reuse common tests
runAllTests(validateX, 'Screen Name');

// 3. Add screen-specific tests
describe('Screen Specific', () => {
    test('unique case', () => { });
});
```

---

## Anti-Patterns to Avoid

### ❌ Don't
- Duplicate validation logic across files
- Use `unwrap()` in production Rust code
- Ignore error handling
- Mix concerns (e.g., UI logic in validation functions)
- Create circular dependencies
- Use global mutable state
- Hard-code strings (use i18n)
- Commit passwords or secrets

### ✅ Do
- Use common modules for shared logic
- Handle all errors explicitly
- Separate concerns (UI, logic, data)
- Use dependency injection
- Keep functions pure when possible
- Use i18n for all user-facing text
- Use environment variables for config

---

## When in Doubt

1. **Check existing code**: Follow the pattern used in similar files
2. **Check this file**: See if there's a convention defined
3. **Ask**: Better to ask than guess and create inconsistent code
4. **Document**: If you establish a new pattern, document it here

---

---

## Implementation Patterns (Added 2025-11-09)

### Modal Pattern - Common Modal Class

**Use Case**: All modal dialogs in the application

**Implementation**:
- Use `res/js/modal.js` Modal class for all modals
- Built-in features: ESC key, backdrop click, focus trap, keyboard navigation
- Replace manual event listeners with Modal callbacks

**Example**:
```javascript
const modal = new Modal('modal-id', {
    formId: 'form-id',
    closeButtonId: 'close-btn',
    cancelButtonId: 'cancel-btn',
    onOpen: (mode, data) => { /* setup modal */ },
    onSave: async (formData) => { /* save data */ }
});
```

**Applied to**:
- transaction-management.js
- user-management.js
- category-management.js
- account-management.js

**Benefits**:
- Consistent behavior across all screens
- Built-in accessibility features
- Reduced code duplication

---

### Transaction Edit Pattern

**Use Case**: Editing transaction data with memo and account management

**Key Implementation Details**:

1. **Memo Management**:
   - Empty memo → `memo_id = NULL` (display as '-' in UI)
   - Existing memo → Reuse same `memo_id` if content matches
   - Shared memo → Create new `memo_id` when content changes
   - Reason: Prevent unintended changes to other transactions using same memo

2. **Account Code "NONE"**:
   - **NOT** converted to NULL
   - Stored as string "NONE" in database
   - Special code meaning "not specified"
   - Database has ACCOUNT_CODE='NONE', ACCOUNT_NAME='指定なし'

3. **Category Change Side Effect**:
   - When category type changes (e.g., TRANSFER → INCOME)
   - Auto-reset incompatible account fields to "NONE"
   - Prevents invalid account selections in UI

**Example (memo handling)**:
```rust
// Empty memo
memo: None → memo_id = NULL

// Shared memo editing
Transaction A, B both use memo_id=10 ("買い物")
Edit Transaction A memo to "食材購入"
Result:
  - Create new memo_id=20 ("食材購入")
  - Transaction A → memo_id=20
  - Transaction B → memo_id=10 (unchanged)
```

**Validation**:
- Date format: YYYY-MM-DD HH:MM:SS (19 characters, strict)
- Amount range: 0 ≤ amount ≤ 999,999,999 (zero allowed, negative not allowed)
- Tax rounding: 0=round down, 1=round half up, 2=round up

---

### Test Patterns and Statistics

**Current Test Count** (as of 2025-11-09):
- Backend (Rust): 121 tests
- Frontend (JavaScript): 404 tests
- **Total: 525 tests** ✅
- Success rate: 100%

**Test File Naming**:
- Unit tests: `*.test.js` (Jest framework)
- Test helpers: `validation-helpers.js`, `*-validation-tests.js`
- Common test suites: Reusable test logic in separate files

**Test Organization**:
```
res/tests/
├── validation-helpers.js       # Common validation functions
├── password-validation-tests.js # Reusable password test suite
├── username-validation-tests.js # Reusable username test suite
├── login.test.js               # Login functionality (58 tests)
├── user-addition.test.js       # User creation (49 tests)
├── user-deletion.test.js       # User deletion (46 tests)
├── transaction-edit.test.js    # Transaction edit (96 tests)
└── ...
```

**Test Principles**:
- Always maintain 100% pass rate
- Follow AAA pattern: Arrange, Act, Assert
- Test naming: "should [expected] when [condition]"
- Reuse common test suites across similar features
- Add tests for new features before merging

---

## Version Notes

- **2025-11-09**: Added implementation patterns
  - Modal pattern with common Modal class
  - Transaction edit pattern (memo management, account handling)
  - Test patterns and current statistics (525 tests)

- **2024-10-26**: Initial conventions established
  - Common module pattern for tests introduced
  - ES Modules with `.js` extensions required
  - AI/LLM specific guidelines added:
    - User documentation timestamps in JST
    - Commit messages in English only
    - Git push operations restricted to developer (hardware key required)
