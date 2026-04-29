# KakeiBon Project Context

## Project

- **Type**: Tauri v2 desktop app (Rust backend + Vanilla JS frontend + SQLite)
- **Purpose**: Household budget app (家計簿) with bilingual support (JP/EN)
- **Version**: v1.2.0
- **Branch**: `dev` for development, `main` for releases

## Essential Rules

1. **All code changes on `dev` branch** — merge to `main` for releases only
2. **Bilingual**: All user-facing text via i18n (never hardcode strings)
3. **i18n RESOURCE_ID**: Always check MAX ID before adding — `INSERT OR IGNORE` silently skips duplicates (use `/i18n-add`)
4. **Release**: Update 3 version files (Cargo.toml, tauri.conf.json, package.json) + run `check-release.sh` (use `/release`)
5. **SQL**: All queries in `src/sql_queries.rs`, prepared statements only
6. **No `unwrap()` in production Rust** — always use `Result<T, E>`
7. **ES Modules**: Include `.js` extension in imports
8. **Commit messages**: English, conventional format `type(scope): description`
9. **Tauri app**: Tell user to "restart app" not "reload browser"
10. **Password**: Min 16 chars, validated in both frontend and backend

## Key Constants

```
ROLE_ADMIN = 0, ROLE_USER = 1, MIN_PASSWORD_LENGTH = 16
DB: ~/.kakeibon/KakeiBonDB.sqlite3
```

## Commands

```bash
cargo tauri dev          # Development
cargo test               # Backend tests
cd res && npm test       # Frontend tests
./scripts/check-release.sh  # Pre-release verification
```

## On-Demand Context

Load with `@` when needed:
- Architecture: `@.ai-context/architecture/PROJECT_STRUCTURE.md`
- Conventions (detailed): `@.ai-context/development/archive/CONVENTIONS_DETAILED.md`
- Testing: `@.ai-context/development/TESTING_STRATEGY.md`
- i18n details: `@.ai-context/workflows/I18N_MANAGEMENT.md`
