# GitHub Copilot Instructions for KakeiBonByRust

## Project

**KakeiBon** — Tauri v2 desktop app (Rust + Vanilla JS + SQLite)
Personal finance management (家計簿)

## Rules

- **Branch**: All code changes on `dev`, merge to `main` for releases
- **Commits**: English, conventional format: `type(scope): description`
- **Timestamps**: User-facing docs use JST (UTC+9)
- **Bilingual**: All user-facing text via i18n (JP/EN)
- **No `unwrap()` in production** — use `Result<T, E>`
- **ES Modules**: Include `.js` extension in imports
- **SQL**: All queries in `src/sql_queries.rs`, prepared statements only

## Release (3-file version sync required)

Before tagging, update ALL three to the same version:
1. `Cargo.toml` — Rust build
2. `package.json` — release name
3. `tauri.conf.json` — asset filenames

Then: `./scripts/check-release.sh` → commit → merge to main → push main → tag → push tag

## Naming

| Lang | Functions | Structs | Constants |
|------|-----------|---------|-----------|
| Rust | `snake_case` | `PascalCase` | `UPPER_SNAKE_CASE` |
| JS | `camelCase` | `PascalCase` | `UPPER_SNAKE_CASE` |

## Key Constants

```
ROLE_ADMIN = 0, ROLE_USER = 1, MIN_PASSWORD_LENGTH = 16
```

## Structure

```
src/           # Rust backend (lib.rs, db.rs, services/, sql_queries.rs)
res/           # Frontend (HTML, js/, css/, sql/, tests/)
.ai-context/   # AI context files
```

## Tests

- Backend: `cargo test` (248 tests)
- Frontend: `cd res && npm test` (216 tests)
- Pattern: "should [expected] when [condition]", AAA structure
