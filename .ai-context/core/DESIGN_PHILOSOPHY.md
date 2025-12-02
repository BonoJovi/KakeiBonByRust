# KakeiBon - Design Philosophy

**Last Updated**: 2025-12-03 04:43 JST  
**Purpose**: Core design principles and architectural rationale

---

## Core Principles

### 1. Security-First Design

**Rationale**: Personal finance data is highly sensitive. Security cannot be an afterthought.

#### Password Security
- **Argon2 Hashing**: Memory-hard algorithm, resistant to GPU/ASIC attacks
- **Minimum Length**: 16 characters (enforced on frontend and backend)
- **No Plaintext Storage**: Passwords never stored or logged

#### Data Encryption
- **Algorithm**: AES-256-GCM (authenticated encryption)
- **Scope**: Sensitive user data at rest
- **Key Management**: Derived from user credentials

#### Defense in Depth
- **Frontend Validation**: Immediate user feedback, reduces server load
- **Backend Validation**: Final authority, prevents bypass
- **SQL Injection Prevention**: Prepared statements only, no string concatenation
- **Input Sanitization**: All user input validated and escaped

---

### 2. Bilingual by Default

**Rationale**: Japanese and English are equally important. Not translation, but parallel development.

#### Implementation
- **All UI Elements**: Defined in both languages from day one
- **Database Schema**: Bilingual name columns (name_ja, name_en)
- **i18n Architecture**: `res/locales/` with fallback mechanism
- **Validation Messages**: Bilingual error messages

#### Why Not Translation?
- **Consistency**: Avoid translation drift
- **Maintenance**: Easier to update both at once
- **Quality**: Native expression in both languages

---

### 3. Type Safety & Explicit Error Handling

**Rationale**: Rust's type system prevents entire classes of bugs. Use it.

#### Rust Backend
```rust
// ✅ Good: Explicit Result types
pub async fn verify_login(username: &str, password: &str) -> Result<UserInfo, String>

// ❌ Bad: Panics or unwrap() in production
let user = db.get_user().unwrap();  // Never do this
```

#### JavaScript Frontend
```javascript
// ✅ Good: Try-catch with proper error handling
try {
    const result = await invoke('verify_login', { username, password });
} catch (error) {
    showError(error);
}

// ❌ Bad: Unhandled promises
invoke('verify_login', { username, password });  // What if it fails?
```

---

### 4. Separation of Concerns

**Rationale**: Clear boundaries make code maintainable and testable.

#### Module Responsibilities

**Backend (Rust)**:
- `src/main.rs` - Entry point, Tauri setup
- `src/lib.rs` - Public API exports
- `src/db.rs` - Database operations only
- `src/validation.rs` - Input validation rules
- `src/security.rs` - Password hashing
- `src/crypto.rs` - Encryption/decryption
- `src/services/` - Business logic (auth, user management, etc.)

**Frontend (JavaScript)**:
- `res/js/` - Modular ES6 modules
- `res/css/` - Scoped stylesheets
- `res/locales/` - Translation resources
- `res/tests/` - Frontend tests

**Database**:
- `sql/` - Migration scripts only
- No ORM - direct SQL with prepared statements

---

### 5. Test-Driven Development (TDD)

**Rationale**: Tests define behavior, prevent regressions, enable refactoring.

#### Testing Strategy
- **Backend**: Inline tests with `#[cfg(test)]` modules
- **Test Helpers**: `src/test_helpers.rs` for common setup
- **Coverage Goal**: >80% for critical paths
- **Integration Tests**: `tests/` directory

#### Test Structure (AAA Pattern)
```rust
#[tokio::test]
async fn test_password_hashing() {
    // Arrange
    let password = "secure_password_16+";
    
    // Act
    let hash = hash_password(password).await.unwrap();
    let verified = verify_password(password, &hash).await.unwrap();
    
    // Assert
    assert!(verified);
}
```

---

### 6. Don't Repeat Yourself (DRY)

**Rationale**: Single source of truth prevents inconsistencies.

#### Common Patterns
- **Validation Rules**: Shared between frontend (`res/tests/validation-helpers.js`) and backend (`src/validation.rs`)
- **Constants**: Defined once, imported everywhere
- **Test Suites**: Reusable test modules (`*-validation-tests.js`)
- **i18n**: Centralized in `res/locales/`

#### Anti-Pattern
```javascript
// ❌ Bad: Duplicated validation logic
// index.html
if (password.length < 16) { /* error */ }

// user-management.html
if (password.length < 16) { /* error */ }

// ✅ Good: Shared validation module
import { validatePassword } from './validation-helpers.js';
validatePassword(password);
```

---

### 7. Progressive Enhancement

**Rationale**: Start simple, add complexity only when needed.

#### Implementation Philosophy
- **Vanilla JS First**: No framework unless complexity demands it
- **Tauri First**: Use Tauri APIs before adding external dependencies
- **SQLite First**: Relational database for structured data
- **Manual Tests First**: Automated tests when patterns stabilize

#### When to Add Complexity
- **Framework**: When state management becomes unwieldy
- **ORM**: Never (direct SQL is explicit and performant)
- **External Libraries**: When implementing from scratch is error-prone (e.g., crypto)

---

### 8. Documentation as Code

**Rationale**: Documentation lives with code, updated together, not as an afterthought.

#### Levels of Documentation
1. **Code Comments**: Only for non-obvious logic
2. **Function Docs**: Parameters, return types, examples
3. **Module Docs**: Purpose, responsibilities, usage
4. **AI Context**: High-level architecture, design decisions
5. **User Docs**: End-user guides (README, CHANGELOG)

#### Documentation Policy
- **Development Phase**: Write freely, don't worry about granularity
- **Release Phase**: Consolidate, restructure for audience

---

## Architecture Patterns

### Command Pattern (Tauri)
```rust
#[tauri::command]
async fn some_operation(arg: String) -> Result<DataType, String> {
    // Always return Result<T, String> for consistent error handling
}
```

### Repository Pattern (Database)
- `src/db.rs` is the single point of database access
- No SQL scattered across business logic
- Prepared statements prevent SQL injection

### Service Layer Pattern
- `src/services/` contains business logic
- Services call `db.rs` for persistence
- Services handle authorization checks

---

## Anti-Patterns to Avoid

### ❌ Don't
1. **Magic Numbers**: Use named constants
2. **God Objects**: Keep modules focused
3. **Implicit Behavior**: Make side effects explicit
4. **Silent Failures**: Always handle errors
5. **Hard-coded Strings**: Use i18n for all user-facing text
6. **Mixing Concerns**: UI logic in database layer, etc.

### ✅ Do
1. **Explicit is Better**: `Result<T, E>` over exceptions
2. **Single Responsibility**: One module, one job
3. **Fail Fast**: Validate early, fail loudly
4. **Type-Driven Design**: Let types guide implementation
5. **Test-First**: Write tests before implementation

---

## Design Trade-offs

### Chosen Trade-offs

| Decision | Pro | Con | Rationale |
|----------|-----|-----|-----------|
| Vanilla JS | No build step, simple | Verbose | Complexity doesn't justify framework yet |
| SQLite | Embedded, no server | Single-user | Desktop app, local-first |
| Argon2 | High security | Slower | Security > performance for passwords |
| Bilingual from start | Consistency | More upfront work | Avoids translation debt |
| No ORM | Explicit, fast | More boilerplate | Control & performance matter |

---

## Future Considerations

### When Project Grows
- **Frontend Framework**: Consider when state management becomes complex
- **Database Migration**: If multi-user needed, consider PostgreSQL
- **API Layer**: If remote access needed, add REST/GraphQL
- **Caching**: If performance bottlenecks appear

### Principles to Maintain
- Security-first mindset
- Bilingual parity
- Explicit error handling
- Test coverage

---

**These principles guide all development decisions. When in doubt, refer back to these fundamentals.**
