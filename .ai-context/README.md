# AI Context Directory

**Last Updated**: 2025-12-15 JST
**Project**: KakeiBon (Personal Finance Manager)
**Purpose**: Hierarchical AI context with shared submodule
**Keywords**: AI context, AIコンテキスト, context directory, コンテキストディレクトリ, hierarchical structure, 階層構造, documentation, ドキュメント, KakeiBon, 家計簿, personal finance, 家計管理, Tauri, Rust, project overview, プロジェクト概要, file organization, ファイル構成, submodule, サブモジュール
**Related**: @core/QUICK_REFERENCE.md, @shared/methodology/DESIGN_PHILOSOPHY.md, @development/CONVENTIONS.md

---

## Directory Structure

```
.ai-context/
├── core/               # [Tier 1] Always load (project status)
│   └── QUICK_REFERENCE.md
│
├── development/        # [Tier 2] Project-specific coding
│   ├── CONVENTIONS.md
│   └── TESTING_STRATEGY.md
│
├── architecture/       # [Tier 2] Project-specific design
│   ├── PROJECT_STRUCTURE.md
│   └── TAURI_DEVELOPMENT.md
│
├── workflows/          # [Tier 2] Project-specific workflows
│   └── I18N_MANAGEMENT.md
│
├── shared/             # [Submodule] Shared across projects
│   ├── developer/
│   │   └── YOSHIHIRO_NAKAHARA_PROFILE.md
│   ├── analytics/
│   │   └── SEO_Keywords_Tracking.md
│   ├── methodology/
│   │   ├── AI_COLLABORATION.md
│   │   ├── DESIGN_PHILOSOPHY.md
│   │   └── SCALE_ARCHITECTURE.md
│   ├── insights/
│   │   └── ... (9 insight documents)
│   └── workflows/
│       ├── DOCUMENTATION_CREATION.md
│       └── GITHUB_PROJECTS.md
│
└── README.md
```

---

## Shared Submodule

The `shared/` directory is a Git submodule pointing to:
**https://github.com/BonoJovi/ai-context-shared**

### Why Submodule?

- **Single source of truth**: Common files managed in one place
- **Cross-project consistency**: KakeiBonByRust and Promps share the same context
- **Easy updates**: `git submodule update --remote` syncs all projects

### Submodule Commands

```bash
# Update to latest shared context
git submodule update --remote

# Clone project with submodules
git clone --recurse-submodules <repo-url>

# Initialize submodules after clone
git submodule init && git submodule update
```

---

## Usage by Task

| Task | Load Files |
|------|-----------|
| **Session Start** | `core/QUICK_REFERENCE.md` |
| **Coding** | + `development/CONVENTIONS.md`, `development/TESTING_STRATEGY.md` |
| **Design** | + `architecture/PROJECT_STRUCTURE.md`, `architecture/TAURI_DEVELOPMENT.md` |
| **Project Mgmt** | + `shared/workflows/GITHUB_PROJECTS.md`, `workflows/I18N_MANAGEMENT.md` |
| **Methodology** | + `shared/methodology/AI_COLLABORATION.md`, `shared/methodology/DESIGN_PHILOSOPHY.md` |

---

## Keyword Search System

All files include searchable keywords (English/Japanese) for better discoverability.

**Examples**:
- Status: `version`, `バージョン`, `current status`, `現在の状態`
- Security: `Argon2`, `AES-256-GCM`, `encryption`, `暗号化`
- Testing: `TDD`, `テスト`, `cargo test`, `unit tests`
- i18n: `internationalization`, `国際化`, `translation`, `翻訳`

---

## Project Highlights

- **Security**: Argon2 + AES-256-GCM encryption
- **Bilingual**: Japanese & English (全機能日英対応)
- **Version**: v1.0.1 (201+ backend tests)
- **Stack**: Tauri v2 + Rust + Vanilla JS

---

## Migration Notes

### 2025-12-15 - Submodule Migration
- Created `ai-context-shared` repository
- Moved common files to shared submodule:
  - developer/, analytics/
  - core/DESIGN_PHILOSOPHY.md → shared/methodology/
  - development/METHODOLOGY.md → shared/methodology/AI_COLLABORATION.md
  - workflows/DOCUMENTATION_CREATION.md, GITHUB_PROJECTS.md → shared/workflows/
- Project-specific files remain in place

---

**GitHub Copilot CLI automatically loads this context via `.github/copilot-instructions.md`.**
