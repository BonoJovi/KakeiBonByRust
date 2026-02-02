# Coding Conventions Overview

**Last Updated**: 2026-02-03
**Purpose**: Quick reference for coding standards (detailed version: `archive/CONVENTIONS_DETAILED.md`)

---

## General Principles

| Principle | Description |
|-----------|-------------|
| **DRY** | Use common modules for shared logic |
| **Consistency** | Backend/frontend must use same validation rules |
| **Type Safety** | Use Rust's type system, avoid `any` in TypeScript |
| **Error Handling** | Always handle errors explicitly |

---

## Critical Rules

### File Modification
- **PROHIBITED**: Full file rewrite (unless absolutely necessary)
- **RECOMMENDED**: Surgical edits - modify only necessary parts

### Database
- **Filename**: `KakeiBonDB.sqlite3` (NOT `kakeibo.db`)
- **Location**: `~/.kakeibon/KakeiBonDB.sqlite3`
- **Access**: Use `./db.sh` script

### SQL Management
- **All SQL queries** must be in `src/sql_queries.rs`
- **No hardcoded SQL** in service/command code
- **Test SQL**: Prefix with `TEST_`

---

## Naming Conventions

| Language | Functions | Structs/Classes | Constants | Files |
|----------|-----------|-----------------|-----------|-------|
| **Rust** | `snake_case` | `PascalCase` | `UPPER_SNAKE_CASE` | - |
| **JavaScript** | `camelCase` | `PascalCase` | `UPPER_SNAKE_CASE` | `kebab-case.js` |

---

## AI/LLM Guidelines

### Git Operations
```
✅ AI can do: git status, git add, git commit, git log, git diff
❌ AI must NOT do: git push (requires hardware key)
```

### Commit Messages
- **Language**: Always English
- **Format**: `type(scope): short description`
- **Types**: feat, fix, refactor, test, docs, style, chore

### Session Limit
- Notify user at 90% of request budget

---

## Error Handling Patterns

**Rust**:
```rust
// ✅ Good
pub async fn get_user(id: i64) -> Result<UserInfo, Error> { }

// ❌ Bad - unwrap in production
db.query().await.unwrap()
```

**JavaScript**:
```javascript
// ✅ Good - always try/catch with async
async function loadData() {
    try {
        const data = await invoke('command');
    } catch (error) {
        console.error('Failed:', error);
    }
}
```

---

## ES Modules (Important!)

```javascript
// ✅ Good - include .js extension
import { func } from './module.js';

// ❌ Bad - missing extension
import { func } from './module';
```

---

## Validation

### Frontend/Backend Must Match
- Same rules in `res/tests/validation-helpers.js` and `src/validation.rs`
- Same error messages

### Unicode Warning
- JavaScript `.length` = character count
- Rust `.len()` = byte length (UTF-8)
- **Use ASCII for boundary tests**

---

## Test Conventions

| Item | Standard |
|------|----------|
| **Naming** | `should [expected] when [condition]` |
| **Pattern** | AAA (Arrange, Act, Assert) |
| **Reuse** | Common test suites across features |
| **Pass rate** | Always 100% |

**Current**: ~800 tests (Backend + Frontend)

---

## Quick Reference

### Tauri Command Pattern
```rust
#[tauri::command]
pub async fn command_name(
    param: String,
    state: tauri::State<'_, AppState>,
) -> Result<T, String> {
    service.method().await.map_err(|e| e.to_string())
}
```

### Tauri Invocation
```javascript
const result = await invoke('command_name', { camelCase: args });
```

---

## When in Doubt

1. Check existing code for patterns
2. Check `archive/CONVENTIONS_DETAILED.md` for full examples
3. Ask rather than guess
4. Document new patterns

---

**For detailed conventions with full examples**: See `archive/CONVENTIONS_DETAILED.md`
