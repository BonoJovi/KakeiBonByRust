# AI Context Directory

**Last Updated**: 2025-12-03 04:41 JST  
**Project**: KakeiBon (Personal Finance Manager)  
**Purpose**: Hierarchical AI context for efficient information retrieval

---

## Directory Structure

```
.ai-context/
├── README.md                          # This file
│
├── core/                              # [Always Load] Critical context
│   ├── QUICK_REFERENCE.md            # Fast lookup, current status
│   └── DESIGN_PHILOSOPHY.md          # Core design principles
│
├── development/                       # [When Coding] Development practices
│   ├── METHODOLOGY.md                # AI collaboration methodology
│   ├── CONVENTIONS.md                # Coding standards
│   └── TESTING_STRATEGY.md           # TDD strategies
│
├── architecture/                      # [When Designing] System design
│   ├── PROJECT_STRUCTURE.md          # Module organization
│   └── TAURI_DEVELOPMENT.md          # Tauri framework specifics
│
└── workflows/                         # [When Managing] Project workflows
    ├── GITHUB_PROJECTS.md            # Issue & feature tracking
    └── I18N_MANAGEMENT.md            # Localization management
```

---

## Usage Guidelines for AI

### When Starting a Session
**Always read**:
- `core/QUICK_REFERENCE.md` - Current phase status, critical decisions
- `core/DESIGN_PHILOSOPHY.md` - Design principles, security architecture

### When Implementing Code
**Read**:
- `development/METHODOLOGY.md` - AI collaboration patterns
- `development/CONVENTIONS.md` - Coding standards (Rust, JS, validation rules)
- `development/TESTING_STRATEGY.md` - Testing approach

### When Making Design Decisions
**Read**:
- `architecture/PROJECT_STRUCTURE.md` - Module responsibilities
- `architecture/TAURI_DEVELOPMENT.md` - Framework constraints

### When Managing Tasks
**Read**:
- `workflows/GITHUB_PROJECTS.md` - Issue tracking guidelines
- `workflows/I18N_MANAGEMENT.md` - Translation workflows

---

## File Priority Reference

| File | Category | Priority | Description |
|------|----------|----------|-------------|
| QUICK_REFERENCE.md | Core | **High** | Current status, version, phase |
| DESIGN_PHILOSOPHY.md | Core | **High** | Security-first design |
| METHODOLOGY.md | Development | Medium | AI collaboration patterns |
| CONVENTIONS.md | Development | Medium | Code style, validation rules |
| TESTING_STRATEGY.md | Development | Medium | Test architecture |
| PROJECT_STRUCTURE.md | Architecture | Low | Module organization |
| TAURI_DEVELOPMENT.md | Architecture | Low | Tauri specifics |
| GITHUB_PROJECTS.md | Workflow | Low | Task management |
| I18N_MANAGEMENT.md | Workflow | Low | i18n workflows |

---

## Design Rationale

### Why Hierarchical Structure?

1. **Token Efficiency**: Load only necessary context for current task
2. **Scalability**: Easy to add new documentation as project grows
3. **Maintainability**: Clear separation of concerns
4. **Discoverability**: Intuitive categorization for AI navigation

### Priority Levels

- **High**: Core context, always relevant
- **Medium**: Development context, frequently needed when coding
- **Low**: Specialized context, needed for specific tasks

---

## Maintenance Policy

### When to Update

- **core/**: When fundamental design decisions or project phase changes
- **development/**: When coding patterns or methodologies evolve
- **architecture/**: When module structure or framework integration changes
- **workflows/**: When project management practices change

### File Size Guidelines

- **core/**: Keep QUICK_REFERENCE concise (< 200 lines)
- **development/**: Split if exceeds 2,000 lines
- **architecture/**: Split by subsystem if needed
- **workflows/**: One file per workflow type

---

## GitHub Copilot CLI Integration

**Automatic Loading**: GitHub Copilot CLI automatically loads contexts referenced in `.github/copilot-instructions.md` at the root of the repository.

**No Manual Initialization Required**: The hierarchical structure is transparent to Copilot CLI.

---

## Project-Specific Notes

### KakeiBon Context Highlights

- **Security Focus**: Argon2 password hashing, AES-256-GCM encryption
- **Bilingual**: All user-facing content in Japanese and English
- **Test Coverage**: 201+ tests (Rust backend comprehensive, frontend manual)
- **Current Version**: v1.0.1
- **Framework**: Tauri v2 (Rust + HTML/CSS/JS)

---

**This structure ensures AI assistants can efficiently access the right information at the right time.**
