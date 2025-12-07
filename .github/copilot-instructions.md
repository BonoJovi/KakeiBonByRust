# GitHub Copilot Instructions for KakeiBonByRust

This file provides context for GitHub Copilot CLI to understand the project structure, conventions, and guidelines.

---

## Project Overview

**Name**: KakeiBon  
**Type**: Desktop application (Tauri + Rust + HTML/JS)  
**Purpose**: Personal finance management (家計簿 - Kakeibo)

**Tech Stack**:
- **Backend**: Rust (Tauri framework)
- **Frontend**: Vanilla HTML/CSS/JavaScript (no framework)
- **Database**: SQLite (via sqlx)
- **Security**: Argon2 password hashing, AES-256-GCM encryption

---

## Important AI/LLM Guidelines

### Git Operations
- **AI/LLM scope**: Full git operations including `git push` are supported
- **Authentication**: HTTPS authentication (migrated from SSH for AI automation support)
- **Note**: Commits are automated through AI assistance

### Commit Message Rules
- **Language**: Always write in English
- **Format**: Follow conventional commits format
- **Example**: `feat(user-mgmt): add user deletion feature`

### Documentation Timestamps
- **User-facing documentation**: Always use Japan Standard Time (JST, UTC+9)
- **AI context documentation**: Can use UTC or JST
- **Example**: `Last Updated: 2024-10-26 13:21 JST`

### Documentation Policy
- **Reference**: See [docs/developer/en/guides/DOCUMENTATION_POLICY.md](../docs/developer/en/guides/DOCUMENTATION_POLICY.md)
- **Current Phase**: Development Phase (write freely, don't worry about granularity)
- **Future Phase**: Release Preparation (consolidate, audience-based restructuring)
- **Bilingual**: All user-facing docs must be in Japanese and English

### Release Workflow
**CRITICAL**: Always follow this workflow for releases:

1. **Code Development** (on `dev` branch)
   - Write/modify code
   - Write/update tests

2. **Local Testing**
   ```bash
   cargo test                    # Backend tests
   cd res && npm test           # Frontend tests
   ```

3. **Build Verification**
   ```bash
   cargo build --release
   ```

4. **Update Version Numbers** ⚠️ **CRITICAL**
   **Before creating a release tag, ALL THREE version files MUST be updated:**
   ```bash
   # Update these files with the same version number:
   # 1. Cargo.toml          → Used by Rust build
   # 2. package.json        → Used by release workflow (release name)
   # 3. tauri.conf.json     → Used by Tauri build (asset file names)
   ```
   
   **Example for v1.0.10:**
   ```toml
   # Cargo.toml
   version = "1.0.10"
   ```
   ```json
   // package.json
   "version": "1.0.10"
   ```
   ```json
   // tauri.conf.json
   "version": "1.0.10"
   ```
   
   **⚠️ Version Mismatch Consequences:**
   - If `package.json` is outdated → Wrong release name created
   - If `tauri.conf.json` is outdated → Wrong asset file names
   - Example: v1.0.9 tag with package.json=1.0.8 → Creates v1.0.8 release!

5. **Commit Changes**
   ```bash
   git add .
   git commit -m "type(scope): message"
   ```

6. **Push to dev**
   ```bash
   git push origin dev
   ```

7. **Merge to main**
   ```bash
   git checkout main
   git merge dev
   ```

8. **Push main branch FIRST**
   ```bash
   git push origin main
   ```
   ⚠️ **CRITICAL**: Push main BEFORE creating tag to ensure tag points to pushed commit

9. **Create and Push Tag**
   ```bash
   git pull origin main    # Ensure you have the latest
   git tag v1.0.x          # Create tag on the pushed commit
   git push origin v1.0.x  # Push tag to trigger CI/CD
   ```

10. **CI/CD Automation**
    - Tag push triggers GitHub Actions workflow
    - Runs tests, builds binaries, publishes release
    - No manual intervention needed

**⚠️ Never skip local testing before commit/push!**
**⚠️ Always verify version consistency across all 3 files before tagging!**

---

## Directory Structure

```
src/                      # Rust backend source
├── main.rs               # Entry point
├── lib.rs                # Tauri commands export
├── db.rs                 # Database operations
├── validation.rs         # Input validation
├── security.rs           # Password hashing
├── crypto.rs             # Encryption/decryption
└── services/             # Business logic modules
    ├── auth.rs           # Authentication
    ├── user_management.rs # User CRUD
    └── i18n.rs           # Internationalization

res/                      # Frontend resources
├── index.html            # Login/Admin setup screen
├── user-management.html  # User management screen
├── js/                   # JavaScript modules
├── css/                  # Stylesheets
├── locales/              # Translation files (ja, en)
└── tests/                # Test suites

.ai-context/              # Detailed AI context (see below)
├── PROJECT_STRUCTURE.md  # Detailed structure
├── CONVENTIONS.md        # Coding conventions
└── projects-guidelines.md # GitHub Projects guidelines
```

---

## Key Conventions

### General Principles
1. **DRY (Don't Repeat Yourself)**: Use common modules for shared logic
2. **Consistency**: Backend and frontend must use same validation rules
3. **Type Safety**: Use Rust's type system, avoid `any` in TypeScript
4. **Error Handling**: Always handle errors explicitly

### Rust Conventions
- **Functions**: `snake_case` (e.g., `verify_login`)
- **Structs**: `PascalCase` (e.g., `UserInfo`)
- **Constants**: `UPPER_SNAKE_CASE` (e.g., `ROLE_ADMIN`)
- **Error Handling**: Always use `Result<T, E>`, never `unwrap()` in production

### JavaScript Conventions
- **ES Modules**: Always include `.js` extension in imports
  ```javascript
  import { validatePassword } from './validation-helpers.js';
  ```
- **Async/Await**: Always use try/catch with async functions
- **Naming**: `camelCase` for functions, `PascalCase` for classes

### Validation Rules
- **Password**: Minimum 16 characters (enforced in both frontend and backend)
- **Unicode Handling**: 
  - JavaScript uses `.length` (UTF-16 code units)
  - Rust backend uses `.len()` (UTF-8 bytes)
  - For tests: Use ASCII for boundary tests, Unicode for character support tests

### Test Conventions
- **Pattern**: "should [expected behavior] when [condition]"
- **Structure**: AAA (Arrange, Act, Assert)
- **Common Modules**: Reuse test suites from `*-validation-tests.js`

---

## Important Constants

| Constant | Value | Location |
|----------|-------|----------|
| ROLE_ADMIN | 0 | `src/consts.rs`, `res/js/consts.js` |
| ROLE_USER | 1 | `src/consts.rs`, `res/js/consts.js` |
| MIN_PASSWORD_LENGTH | 16 | `src/validation.rs` |

---

## Common Tasks

### Adding a new screen
1. Create HTML file in `res/`
2. Create JS file in `res/js/`
3. Add Tauri commands in `src/` (if needed)
4. Create test file in `res/tests/` using common modules

### Modifying validation rules
1. Update `res/tests/validation-helpers.js`
2. Update backend in `src/validation.rs`
3. Run tests to verify all screens

---

## Test Architecture

**Frontend (JavaScript):**
- Common validation functions: `validation-helpers.js`
- Common test suites: `*-validation-tests.js`
- Screen-specific tests import and reuse common modules

**Backend (Rust):**
- Common test helpers: `src/test_helpers.rs`
- Common validation test suites: `src/validation_tests.rs`

**Test Count:**
- Rust backend: 201 tests
- JavaScript frontend: Manual testing (test runner not configured)
- **Total: 201+ tests**

---

## GitHub Projects Management

- **Features**: New functionality (granular level from TODO.md)
- **Issues**: Bugs and improvements
- **Projects URL**: https://github.com/users/BonoJovi/projects/5
- **Rule**: Always register Features/Issues to Projects when created

---

## Anti-Patterns to Avoid

❌ Don't:
- Duplicate validation logic across files
- Use `unwrap()` in production Rust code
- Ignore error handling
- Mix concerns (e.g., UI logic in validation functions)
- Hard-code strings (use i18n)
- Commit passwords or secrets

✅ Do:
- Use common modules for shared logic
- Handle all errors explicitly
- Separate concerns (UI, logic, data)
- Use i18n for all user-facing text

---

## Detailed Context Files

For more detailed information, refer to files in `.ai-context/`:
- **PROJECT_STRUCTURE.md**: Comprehensive project structure and module responsibilities
- **CONVENTIONS.md**: Detailed coding conventions and patterns
- **projects-guidelines.md**: GitHub Projects usage guidelines

---

**Last Updated**: 2025-12-05 18:40 JST
