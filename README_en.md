# [Book] KakeiBon (Household Budget App)

<div align="center">

> **A Modern Household Budget App with Focus on Readability and Usability**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2.9.3-blue.svg)](https://tauri.app/)
[![Tests](https://img.shields.io/badge/tests-527%20passing-brightgreen.svg)](#test-results)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[[J][P] Japanese Version](README_ja.md) | [[Globe] Bilingual README](README.md)

</div>

---

## [Love] Message from Developer

<div style="border: 3px solid #4a90e2; padding: 20px; margin: 20px 0; background-color: #f8f9fa; font-size: 1.1em;">

### To All Beloved KakeiBon Users

Thank you for your continued interest in KakeiBon.
I'm BonoJovi (Yoshihiro NAKAHARA), the project initiator.

**We have officially released Ver.1.0.7!**

The major version jump from Ver.1.0.1 to Ver.1.0.7 reflects the significant impact of introducing a CI/CD pipeline, enabling multi-platform support (Windows/macOS/Linux) and automated releases. This large-scale infrastructure improvement has dramatically enhanced development efficiency and enables smooth continuous feature improvements and releases going forward.

The transaction data input functionality is now complete, and KakeiBon is ready to be used as a basic household budget application.
If you would like to use the stable release version, please refer to the [main branch](https://github.com/BonoJovi/KakeiBonByRust/tree/main).

The dev branch you are currently viewing is the development version, where we are working on features for the next release.
If you want to try the latest features early, please use this dev branch.

We plan to proceed with implementing aggregation and reporting features next. We will continue to add various features incrementally, so please look forward to continuous enhancements.
We welcome messages via GitHub issues or email, whether it's words of encouragement or suggestions for features you'd like to see in the future—any feedback is appreciated.

Thank you for your continued support of KakeiBon.

**2025-11-30 (JST) Written by Yoshihiro NAKAHARA**

</div>

---

## [Bookmark] Table of Contents

- [[WIP] Development Status](#-development-status)
- [[Chart] Repository Statistics](#-repository-statistics)
- [[Books] Legacy Version](#-legacy-version)
- [✨ Key Features](#-key-features)
- [[Rocket] Current Features](#-current-features)
- [[PC] Technology Stack](#-technology-stack)
- [[Package] Installation](#-installation)
- [[Test] Test Results](#-test-results)
- [[Books] Documentation](#-documentation)
- [[Handshake] Contributing](#-contributing)
- [[Doc] License](#-license)
- [[Star] Development Roadmap](#-development-roadmap)

---

## [WIP] Development Status

**[Fire] Actively Under Development**

Development is progressing smoothly, and we strive to update daily!

**Project Started**: 2025-10-22 (JST)  
**Last Updated**: 2025-11-30 (JST)

> **[Robot] AI-Assisted Development**  
> This project's source code and documentation are **100% generated** with the assistance of generative AI (GitHub Copilot, Claude), supervised and reviewed by the developer. This demonstrates the potential of AI-assisted development.
> 
> [Chart] **[See AI Development Metrics & Quality Analysis →](docs/etc/AI_DEVELOPMENT_METRICS.md)**

<!-- STATS_START -->
## [Chart] Repository Statistics

<div align="center">

### [TrendUp] Daily Traffic

![Daily Traffic Stats](docs/stats_graph_daily.png)

### [Chart] Cumulative Traffic

![Cumulative Traffic Stats](docs/stats_graph_cumulative.png)

| Metric | Count |
|--------|-------|
| [Eye]️ **Total Views** | **660** |
| [Package] **Total Clones** | **214** |

*Last Updated: 2025-11-30 12:09 UTC*

</div>
<!-- STATS_END -->

---

## [Books] Legacy Version

**Looking for the stable Lazarus/Free Pascal version?**

[Point] **[KakeiBon (Original)](https://github.com/BonoJovi/KakeiBon)** - Ready to use now!

The original KakeiBon is a **fully functional household budget app** ready to use right now!

**Key Differences:**
- ✅ **Stable & Production-Ready**
- [Package] **Pre-built Binaries Available** ([Releases](https://github.com/BonoJovi/KakeiBon/releases/))
- [J][P] **Japanese Interface Only**
- [Desktop]️ **Linux & Windows Support**
- [Text] **Large Fonts & Accessibility**

**Why Rust Version?**

This Rust rewrite offers:
- ⚡ **Better Performance**
- [Lock] **Enhanced Security** (Argon2 + AES-256-GCM)
- [Globe] **Full Multilingual Support**
- [Art] **Modern Architecture**
- [Crystal] **Future Expandability**

[Idea] **Try both and choose what works best for you!**

---

## ✨ Key Features

### [Art] NOT Vibe Coding
Built with **proper planning and documentation first**, not vibes

### [User] Clear User-First Policy
Every feature is designed with **explicit user needs and usability** in mind

### [Text] Large, Easy-to-Read Text
Designed with high visibility in mind - comfortable for long-term use

### [Build]️ Enterprise-Grade Architecture
**Session-Based Authentication** throughout all 52 API functions

- [Key] **Secure Session Management**
- [Users] **User Isolation**
- ✅ **Zero Hardcoded User IDs**
- [Test] **527 Tests (100% Pass)**

### [Target] Intuitive User Interface
Simple and clear UI that anyone can master quickly

### ♿ Accessibility Support
- **Font Size Adjustment**: Small/Medium/Large/Custom (10-30px)
- **Keyboard Navigation**: Fully supported
- **Focus Indicators**: Clear visual feedback

### [Globe] Multilingual Support
Switch between Japanese and English seamlessly

### [Lock] Strong Security
- Argon2id password hashing
- AES-256-GCM data encryption
- Role-based access control

---

## [Rocket] Current Features

| Feature | Description | Status |
|---------|-------------|--------|
| [Key] **Session Management** | In-memory session state management | ✅ Complete |
| [Money] **Category Management** | Hierarchical category system (Major/Middle/Minor) | ✅ Complete |
| [Users] **User Management** | Multi-user support (Admin/General) | ✅ Complete |
| [Bank] **Account Management** | Account master data management | ✅ Complete |
| [Shop] **Shop Management** | Shop master data management | ✅ Complete |
| [Factory] **Manufacturer Management** | Manufacturer master data with IS_DISABLED feature | ✅ Complete |
| [Package] **Product Management** | Product master data with manufacturer linkage | ✅ Complete |
| [World] **Multilingual** | Dynamic language switching (JP/EN) - 992 resources | ✅ Complete |
| [Fix] **Customization** | Font size, language preferences | ✅ Complete |
| [Note] **Transaction Management** | Header-level CRUD, filters, pagination | ✅ Complete |
| [Receipt] **Transaction Details** | CRUD operations with smart tax calculation, automatic rounding detection | ✅ Complete |
| [Chart] **Reports** | Monthly/annual summaries, graphs | [WIP] In Progress |

---

## [PC] Technology Stack

| Category | Technology | Details |
|----------|------------|---------|
| **Frontend** | Vanilla JavaScript + HTML5 + CSS3 | ES6 Modules |
| **Backend** | Rust + Tauri | v2.8.5 |
| **Database** | SQLite | WAL mode |
| **Security** | Argon2id + AES-256-GCM | Password hashing + Data encryption |
| **Testing** | Jest + Cargo Test | 527 tests passing (Rust: 201, JS: 326) |
| **i18n Resources** | JSON-based | 992 resources (496 unique keys, 2 languages) |
| **Code Lines** | Total | ~35,478 lines (Rust: 13,870, JS: 8,810, HTML: 3,355, CSS: 6,109, SQL: 3,334) |

---

## [Package] Installation

### Prerequisites
- Rust 1.70+ (Install via [rustup](https://rustup.rs/))
- Node.js 18+ (for Tauri CLI)
- SQLite3 native library
  - **Windows**: Download and install from [sqlite.org](https://www.sqlite.org/download.html)
    - Download `sqlite-dll-win-x64-*.zip` (64-bit DLL)
    - Extract `sqlite3.dll` to `C:\Windows\System32\` (or add to PATH)
  - **macOS**: Pre-installed (or install via Homebrew: `brew install sqlite3`)
  - **Linux**: Install via package manager
    - Ubuntu/Debian: `sudo apt-get install libsqlite3-dev`
    - Fedora/RHEL: `sudo dnf install sqlite-devel`
    - Arch: `sudo pacman -S sqlite`

### Build & Run

```bash
# Clone repository
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust

# Run in development mode
cargo tauri dev

# Production build
cargo tauri build
```

---

## [Test] Test Results

```
Backend (Rust):       201 passing ✅
Frontend (JavaScript): 326 passing ✅
Total Tests:          527 passing ✅
Success Rate:         100%
```

**Recent Improvements**:
- ✅ **Session Management Integration** (2025-11-30)
  - All 52 API functions now use session-based authentication
  - Enhanced security with proper user isolation
  - Removed hardcoded user IDs throughout the codebase

- ✅ **Test Quality Enhancement** (2025-11-30)
  - Added explicit assertions to delegated tests
  - Improved test readability and maintainability
  - Enterprise-grade test structure achieved

**Test Count Methodology** (Updated 2025-11-30):
- **Previous count (613)**: Included nested `describe` blocks and test structure
- **Current count (527)**: Counts only actual executable test cases
- **Reason for change**: Improved accuracy and industry-standard methodology
- **Note**: No tests were removed; this is purely a measurement refinement

See [Test Overview](docs/testing/en/TEST_OVERVIEW.md) for details

---

## [Books] Documentation

### [Book] Documentation Index
- [Files]️ **[Complete Documentation Index](docs/INDEX_en.md)** - Quick access to all documentation

### [Target] Getting Started

#### Installation & Setup
- [Package] **[Setup Guide](docs/user/en/SETUP_GUIDE.md)** - How to install the app

#### User Manual
- [Book] **[User Manual](docs/user/en/USER_MANUAL_en.md)** - How to use features
- [Fix] **[Troubleshooting](docs/user/en/TROUBLESHOOTING.md)** - Problem resolution guide

---

### [Man]‍[PC] For Developers

#### Design Documents
- [Build]️ **[Architecture](docs/developer/en/design/ARCHITECTURE.md)** - System architecture overview
- [Lock] **[Security Design](docs/developer/en/design/SECURITY_DESIGN.md)** - Security implementation
- [Cabinet]️ **[Database Design](docs/developer/en/design/DATABASE_DESIGN.md)** - DB schema and ER diagrams
- [Art] **[UI Design](docs/developer/en/design/UI_DESIGN.md)** - User interface design

#### Development Guides
- [Rocket] **[Development Setup](docs/developer/en/guides/DEVELOPMENT_SETUP.md)** - Setting up dev environment
- [Note] **[Coding Standards](docs/developer/en/guides/CODING_STANDARDS.md)** - Code style guide
- [Test] **Testing Documentation**
  - [Book] **[Test Overview](docs/testing/en/TEST_OVERVIEW.md)** - Test strategy and execution guide
  - [BlueBook] **[Backend Test Index](docs/testing/en/BACKEND_TEST_INDEX.md)** - Complete Rust test list (201 tests)
  - [GreenBook] **[Frontend Test Index](docs/testing/en/FRONTEND_TEST_INDEX.md)** - Complete JavaScript test list (262+ tests)

#### API Documentation
- [Link] **[Common API](docs/developer/en/api/API_COMMON.md)** - Auth, session, i18n
- [Users] **[User Management API](docs/developer/en/api/API_USER.md)** - User CRUD operations
- [Folder] **[Category Management API](docs/developer/en/api/API_CATEGORY.md)** - Hierarchical category management
- [Money] **[Transaction Management API](docs/developer/en/api/API_TRANSACTION.md)** - Transaction data management
- [Bank] **[Account Management API](docs/developer/en/api/API_ACCOUNT.md)** - Account master management
- [Office] **[Master Data API](docs/developer/en/api/API_MASTER_DATA.md)** - Shops, manufacturers, products
- [Chart] **[Aggregation API](docs/developer/en/api/API_AGGREGATION.md)** - Reports and statistics
- ⚙️ **[Settings API](docs/developer/en/api/API_SETTINGS.md)** - User settings management

---

### [List] Project Information
- [Users] **[Project Participants](docs/etc/PROJECT_PARTICIPANTS.md)** - Contributors list
- [Chart] **[AI Development Metrics](docs/etc/AI_DEVELOPMENT_METRICS.md)** - AI-assisted development analysis

---

## [Handshake] Contributing

Contributions are welcome!

1. Fork this repository
2. Create a feature branch  
   `git checkout -b feature/AmazingFeature`
3. Commit your changes  
   `git commit -m 'Add some AmazingFeature'`
4. Push to the branch  
   `git push origin feature/AmazingFeature`
5. Open a Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for details

---

## [Doc] License

This project is licensed under the terms in the [LICENSE](LICENSE) file.

---

## [Star] Development Roadmap

- [x] User management
- [x] Category management
- [x] Multilingual support
- [x] Accessibility features
- [x] Transaction management
- [x] Monthly/annual reports
- [ ] Data export (CSV)
- [ ] Backup & restore

---

<div align="center">

**Made with ❤️ and Rust**

[Report Bug](https://github.com/BonoJovi/KakeiBonByRust/issues) · [Request Feature](https://github.com/BonoJovi/KakeiBonByRust/issues)

</div>
