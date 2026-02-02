# KakeiBon Project Context

This file automatically loads **minimal** project context at the start of each Claude Code session.

**Token Optimization**: Only essential information is loaded by default. Load additional contexts as needed using `@` references.

---

## Always Load (Essential Context Only)

### Essential Information - Current Status & Critical Rules
@.ai-context/core/QUICK_REFERENCE.md

---

## Load When Needed (On-Demand Contexts)

### Project-Specific Context

**For Coding Tasks**:
- Conventions: `@.ai-context/development/CONVENTIONS_OVERVIEW.md`
- Conventions (detailed): `@.ai-context/development/archive/CONVENTIONS_DETAILED.md`
- Testing Strategy: `@.ai-context/development/TESTING_STRATEGY.md`

**For Architecture Tasks**:
- Project Structure: `@.ai-context/architecture/PROJECT_STRUCTURE.md`
- Tauri Integration: `@.ai-context/architecture/TAURI_DEVELOPMENT.md`

**For Workflow Tasks**:
- i18n Management: `@.ai-context/workflows/I18N_MANAGEMENT.md`

### Shared Context (via submodule)

**For Understanding Methodology** (rarely needed):
- AI Collaboration: `@.ai-context/shared/methodology/AI_COLLABORATION.md`
- Design Philosophy: `@.ai-context/shared/methodology/DESIGN_PHILOSOPHY.md`
- Scale & Architecture: `@.ai-context/shared/methodology/SCALE_ARCHITECTURE.md`

**Common Workflows**:
- Documentation Creation: `@.ai-context/shared/workflows/DOCUMENTATION_CREATION.md`
- GitHub Projects: `@.ai-context/shared/workflows/GITHUB_PROJECTS.md`

**Developer Profile** (for career/context reference):
- `@.ai-context/shared/developer/YOSHIHIRO_NAKAHARA_PROFILE.md`

**Analytics** (SEO tracking across all projects):
- `@.ai-context/shared/analytics/SEO_Keywords_Tracking.md`

**Insights** (optional reading):
- `@.ai-context/shared/insights/` - Various architectural and development insights

---

## Submodule Management

The `shared/` directory is a Git submodule pointing to:
https://github.com/BonoJovi/ai-context-shared

**Update shared context**:
```bash
git submodule update --remote
```

---

**Performance Note**: This configuration loads minimal context at session startup. Load other contexts only when needed.
