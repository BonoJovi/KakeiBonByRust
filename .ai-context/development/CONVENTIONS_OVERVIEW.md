# Coding Conventions Overview

**Purpose**: Quick reference for coding standards. Details: `archive/CONVENTIONS_DETAILED.md`

## Naming

| Language | Functions | Structs/Classes | Constants | Files |
|----------|-----------|-----------------|-----------|-------|
| Rust | `snake_case` | `PascalCase` | `UPPER_SNAKE_CASE` | - |
| JavaScript | `camelCase` | `PascalCase` | `UPPER_SNAKE_CASE` | `kebab-case.js` |

## Critical Rules

- **File edits**: Surgical — modify only necessary parts, no full rewrites
- **DB filename**: `KakeiBonDB.sqlite3` at `~/.kakeibon/`
- **SQL queries**: Must be in `src/sql_queries.rs`, never hardcoded in services
- **Test SQL**: Prefix with `TEST_`

## Patterns

### Tauri Command
```rust
#[tauri::command]
pub async fn cmd(param: String, state: tauri::State<'_, AppState>) -> Result<T, String> {
    service.method().await.map_err(|e| e.to_string())
}
```

### Tauri Invoke
```javascript
const result = await invoke('cmd', { camelCase: args });
```

## Error Handling

- Rust: Always `Result<T, E>`, never `unwrap()` in production
- JavaScript: Always `try/catch` with async functions

## Validation

- Frontend + Backend must enforce same rules
- Unicode: JS `.length` = char count, Rust `.len()` = byte length
