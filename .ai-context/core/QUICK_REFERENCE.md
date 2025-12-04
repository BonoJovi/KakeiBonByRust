# KakeiBon - Quick Reference

**Last Updated**: 2025-12-03 04:42 JST  
**Version**: v1.0.1  
**Status**: Released (Maintenance & Enhancement)

---

## Current Project Status

### Released Version
- **v1.0.1**: Stable release with core features
- **Platform**: Desktop (Windows, macOS, Linux via Tauri)
- **Database**: SQLite with encryption

### Active Development
- **Branch**: `dev` (feature development)
- **Focus**: Documentation improvement, security monitoring

---

## Core Features (Implemented)

### Phase 1-4: Production Features ✅
- ✅ User Management (Admin/User roles, Argon2 password hashing)
- ✅ Authentication & Authorization
- ✅ Category Management (2-level hierarchy with bilingual names)
- ✅ Transaction Management (Income/Expense with filters)
- ✅ Data Encryption (AES-256-GCM for sensitive data)
- ✅ Internationalization (Japanese & English)

### Test Coverage
- **Backend**: 201 tests (comprehensive)
- **Frontend**: Manual testing (automated tests pending)

---

## Technology Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| Framework | Tauri | v2.9.3 |
| Backend | Rust | 1.77.2+ |
| Frontend | Vanilla JS/HTML/CSS | ES6+ |
| Database | SQLite | via sqlx 0.8.6 |
| Security | Argon2, AES-256-GCM | - |

---

## Key Design Decisions

### Security-First Architecture
1. **Password Hashing**: Argon2 (memory-hard, resistant to GPU attacks)
2. **Data Encryption**: AES-256-GCM for sensitive data at rest
3. **Validation**: Enforced on both frontend and backend
4. **SQL Injection Prevention**: Prepared statements only

### Validation Rules
- **Password Minimum**: 16 characters (enforced in both FE & BE)
- **Username**: 3-20 characters, alphanumeric + underscore
- **Bilingual Names**: Required for categories (ja, en)

### Code Organization
- **Backend**: `src/` - Rust modules with clear separation
- **Frontend**: `res/` - Vanilla JS with ES modules
- **Tests**: Inline tests for Rust, `res/tests/` for frontend
- **Database**: `sql/` - Migration scripts

---

## Critical Constants

```rust
// src/consts.rs & res/js/consts.js
ROLE_ADMIN = 0
ROLE_USER = 1
MIN_PASSWORD_LENGTH = 16
```

---

## Common Tasks

### Running the Application
```bash
cargo tauri dev      # Development mode
cargo tauri build    # Production build
```

### Running Tests
```bash
cargo test                    # All Rust tests
cargo test --lib             # Library tests only
cargo test user_management   # Specific module
```

### Database Operations
```bash
./db.sh                      # Interactive DB management
```

---

## Security Monitoring

### Dependabot Alerts
- **Active Alert #1**: glib VariantStrIter (Medium severity)
  - Status: Monitoring (indirect dependency, no direct usage)
  - See: `docs/security/alerts/dependabot-alert-1-glib.md`

---

## Documentation

### User-Facing
- `README.md` - English (with badge stats)
- `README_ja.md` - Japanese
- `CHANGELOG.md` - Version history

### Developer
- `.ai-context/` - AI assistant context (hierarchical)
- `docs/` - Technical documentation
- `SECURITY.md` - Security policy

### AI Context
- Always read: `core/QUICK_REFERENCE.md` (this file), `core/DESIGN_PHILOSOPHY.md`
- When coding: `development/` directory
- When designing: `architecture/` directory
- When managing: `workflows/` directory

---

## Issue & Feature Tracking

**GitHub Projects**: https://github.com/users/BonoJovi/projects/5

### Rules
1. All Features/Issues must be registered in Projects
2. Use appropriate labels (enhancement, bug, testing, etc.)
3. Link to Project when creating Issue/Feature

---

## Recent Changes

### 2025-12-03
- Restructured AI context to hierarchical format
- Updated issue templates (bilingual, desktop app-specific)
- Updated SECURITY.md to reflect v1.0.x support
- Added security documentation for Dependabot alerts

### 2025-11-30
- Released v1.0.1
- Added repository traffic stats automation
- Improved documentation structure

---

## Next Steps

### Short Term
- Document refactoring (in progress)
- Frontend test automation setup (pending)

### Medium Term
- Phase 5: Additional features based on user feedback
- Performance optimization
- Enhanced analytics

---

**For detailed information, refer to category-specific documents in `.ai-context/` subdirectories.**
