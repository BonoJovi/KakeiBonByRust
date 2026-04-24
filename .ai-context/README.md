# AI Context Directory

## Structure

```
.ai-context/
├── core/QUICK_REFERENCE.md          # Project status & tech stack
├── development/
│   ├── CONVENTIONS_OVERVIEW.md      # Coding standards (quick)
│   ├── TESTING_STRATEGY.md          # Test approach
│   └── archive/CONVENTIONS_DETAILED.md  # Full conventions
├── architecture/
│   ├── PROJECT_STRUCTURE.md         # Codebase structure
│   └── TAURI_DEVELOPMENT.md         # Tauri-specific notes
├── workflows/I18N_MANAGEMENT.md     # i18n resource management
└── shared/                          # Git submodule (ai-context-shared)
```

## Shared Submodule

`shared/` → https://github.com/BonoJovi/ai-context-shared

```bash
git submodule update --remote  # Update shared context
```
