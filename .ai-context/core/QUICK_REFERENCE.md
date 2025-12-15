# KakeiBon - Quick Reference

**Last Updated**: 2025-12-11 03:59 JST  
**Version**: v1.0.1  
**Status**: Released (Maintenance & Enhancement)  
**Purpose**: Fast lookup for current project status, version, tech stack, and key decisions  
**Keywords**: quick reference, クイックリファレンス, current status, 現在の状態, version, バージョン, v1.0.1, release, リリース, tech stack, 技術スタック, Tauri, Rust, SQLite, features, 機能, user management, ユーザー管理, authentication, 認証, encryption, 暗号化, AES-256-GCM, Argon2, i18n, internationalization, 国際化, bilingual, バイリンガル, Japanese, English, 日本語, 英語, project status, プロジェクトステータス, development phase, 開発フェーズ  
**Related**: @core/DESIGN_PHILOSOPHY.md, @development/CONVENTIONS.md, @development/TESTING_STRATEGY.md, @architecture/PROJECT_STRUCTURE.md

---

## Current Status

- **Version**: v1.0.1 (Stable)
- **Platform**: Desktop (Tauri - Windows, macOS, Linux)
- **Branch**: `dev` (documentation improvement, security monitoring)
- **Tests**: 201 backend tests (comprehensive), frontend manual

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Framework | Tauri v2.9.3 |
| Backend | Rust 1.77.2+ |
| Frontend | Vanilla JS/HTML/CSS (ES6+) |
| Database | SQLite (sqlx 0.8.6) |
| Security | Argon2, AES-256-GCM |

---

## Core Features (Phase 1-4 ✅)

- User Management (Admin/User roles)
- Authentication & Authorization
- Category Management (2-level bilingual)
- Transaction Management (Income/Expense)
- Data Encryption (AES-256-GCM)
- Internationalization (Japanese & English)

---

## Critical Values

```rust
ROLE_ADMIN = 0
ROLE_USER = 1
MIN_PASSWORD_LENGTH = 16
```

---

## Quick Commands

```bash
cargo tauri dev       # Development
cargo tauri build     # Production build
cargo test            # Run tests
./db.sh              # Database management
```

---

## Key Decisions

1. **Security-First**: Argon2 password hashing, AES-256-GCM encryption
2. **Bilingual**: All user-facing content in Japanese & English
3. **Validation**: Frontend + Backend (16+ char passwords)
4. **SQL Safety**: Prepared statements only

---

## Project Links

- **GitHub Projects**: https://github.com/users/BonoJovi/projects/5
- **Security Alert #1**: `docs/security/alerts/dependabot-alert-1-glib.md`

---

**See @core/DESIGN_PHILOSOPHY.md for design principles, @architecture/PROJECT_STRUCTURE.md for codebase structure.**
