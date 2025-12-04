# 📖 KakeiBon (Household Budget App)

<div align="center">

> **A Modern Household Budget App with Focus on Readability and Usability**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2.9.3-blue.svg)](https://tauri.app/)
[![Tests](https://img.shields.io/badge/tests-527%20passing-brightgreen.svg)](#test-results)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[🇯🇵 Japanese Version](README_ja.md) | [🌐 Bilingual README](README.md)

</div>

---

## 💌 Message from Developer

<div style="border: 3px solid #4a90e2; padding: 20px; margin: 20px 0; background-color: #f8f9fa; font-size: 1.1em;">

### To All Beloved KakeiBon Users

Thank you for your continued interest in KakeiBon.
I'm BonoJovi (Yoshihiro NAKAHARA), the project initiator.

**We have officially released Ver.1.0.1!**

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

## 📑 Table of Contents

- [🚧 Development Status](#-development-status)
- [📊 Repository Statistics](#-repository-statistics)
- [📚 Legacy Version](#-legacy-version)
- [✨ Key Features](#-key-features)
- [🚀 Current Features](#-current-features)
- [💻 Technology Stack](#-technology-stack)
- [📦 Installation](#-installation)
- [🧪 Test Results](#-test-results)
- [📚 Documentation](#-documentation)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)
- [🌟 Development Roadmap](#-development-roadmap)

---

## 🚧 Development Status

**🔥 Actively Under Development**

Development is progressing smoothly, and we strive to update daily!

**Project Started**: 2025-10-22 (JST)  
**Last Updated**: 2025-11-30 (JST)

> **🤖 AI-Assisted Development**  
> This project's source code and documentation are **100% generated** with the assistance of generative AI (GitHub Copilot, Claude), supervised and reviewed by the developer. This demonstrates the potential of AI-assisted development.
> 
> 📊 **[See AI Development Metrics & Quality Analysis →](docs/etc/AI_DEVELOPMENT_METRICS.md)**

<!-- STATS_START -->
## 📊 Repository Statistics

<div align="center">

### 📈 Daily Traffic

![Daily Traffic Stats](docs/stats_graph_daily.png)

### 📊 Cumulative Traffic

![Cumulative Traffic Stats](docs/stats_graph_cumulative.png)

| Metric | Count |
|--------|-------|
| 👁️ **Total Views** | **660** |
| 📦 **Total Clones** | **214** |

*Last Updated: 2025-11-30 12:09 UTC*

</div>
<!-- STATS_END -->

---

## 📚 Legacy Version

**Looking for the stable Lazarus/Free Pascal version?**

👉 **[KakeiBon (Original)](https://github.com/BonoJovi/KakeiBon)** - Ready to use now!

The original KakeiBon is a **fully functional household budget app** ready to use right now!

**Key Differences:**
- ✅ **Stable & Production-Ready**
- 📦 **Pre-built Binaries Available** ([Releases](https://github.com/BonoJovi/KakeiBon/releases/))
- 🇯🇵 **Japanese Interface Only**
- 🖥️ **Linux & Windows Support**
- 🔤 **Large Fonts & Accessibility**

**Why Rust Version?**

This Rust rewrite offers:
- ⚡ **Better Performance**
- 🔒 **Enhanced Security** (Argon2 + AES-256-GCM)
- 🌐 **Full Multilingual Support**
- 🎨 **Modern Architecture**
- 🔮 **Future Expandability**

💡 **Try both and choose what works best for you!**

---

## ✨ Key Features

### 🎨 NOT Vibe Coding
Built with **proper planning and documentation first**, not vibes

### 👤 Clear User-First Policy
Every feature is designed with **explicit user needs and usability** in mind

### 🔤 Large, Easy-to-Read Text
Designed with high visibility in mind - comfortable for long-term use

### 🏗️ Enterprise-Grade Architecture
**Session-Based Authentication** throughout all 52 API functions

- 🔐 **Secure Session Management**
- 👥 **User Isolation**
- ✅ **Zero Hardcoded User IDs**
- 🧪 **527 Tests (100% Pass)**

### 🎯 Intuitive User Interface
Simple and clear UI that anyone can master quickly

### ♿ Accessibility Support
- **Font Size Adjustment**: Small/Medium/Large/Custom (10-30px)
- **Keyboard Navigation**: Fully supported
- **Focus Indicators**: Clear visual feedback

### 🌐 Multilingual Support
Switch between Japanese and English seamlessly

### 🔒 Strong Security
- Argon2id password hashing
- AES-256-GCM data encryption
- Role-based access control

---

## 🚀 Current Features

| Feature | Description | Status |
|---------|-------------|--------|
| 🔐 **Session Management** | In-memory session state management | ✅ Complete |
| 💰 **Category Management** | Hierarchical category system (Major/Middle/Minor) | ✅ Complete |
| 👥 **User Management** | Multi-user support (Admin/General) | ✅ Complete |
| 🏦 **Account Management** | Account master data management | ✅ Complete |
| 🏪 **Shop Management** | Shop master data management | ✅ Complete |
| 🏭 **Manufacturer Management** | Manufacturer master data with IS_DISABLED feature | ✅ Complete |
| 📦 **Product Management** | Product master data with manufacturer linkage | ✅ Complete |
| 🌍 **Multilingual** | Dynamic language switching (JP/EN) - 992 resources | ✅ Complete |
| 🔧 **Customization** | Font size, language preferences | ✅ Complete |
| 📝 **Transaction Management** | Header-level CRUD, filters, pagination | ✅ Complete |
| 🧾 **Transaction Details** | CRUD operations with smart tax calculation, automatic rounding detection | ✅ Complete |
| 📊 **Reports** | Monthly/annual summaries, graphs | 🚧 In Progress |

---

## 💻 Technology Stack

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

## 📦 Installation

### Prerequisites
- Rust 1.70+ (Install via [rustup](https://rustup.rs/))
- Node.js 18+ (for Tauri CLI)

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

## 🧪 Test Results

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

See [TEST_SUMMARY.md](docs/developer/en/testing/TEST_SUMMARY.md) for details

---

## 📚 Documentation

### 🎯 Getting Started

#### Installation & Setup
- 📦 **[Setup Guide](docs/user/en/SETUP_GUIDE.md)** - How to install the app

#### User Manual
- 📖 **[User Manual](docs/user/en/USER_MANUAL.md)** - How to use features
- ❓ **[FAQ](docs/user/en/FAQ.md)** - Frequently asked questions
- 🔧 **[Troubleshooting](docs/user/en/TROUBLESHOOTING.md)** - Problem resolution guide

---

### 👨‍💻 For Developers

#### Design Documents
- 🏗️ **[Architecture](docs/developer/en/design/ARCHITECTURE.md)** - System architecture overview
- 🔒 **[Security Design](docs/developer/en/design/SECURITY_DESIGN.md)** - Security implementation
- 🗄️ **[Database Design](docs/developer/en/design/DATABASE_DESIGN.md)** - DB schema and ER diagrams
- 🎨 **[UI Design](docs/developer/en/design/UI_DESIGN.md)** - User interface design

#### Development Guides
- 🚀 **[Development Setup](docs/developer/en/guides/DEVELOPMENT_SETUP.md)** - Setting up dev environment
- 📝 **[Coding Standards](docs/developer/en/guides/CODING_STANDARDS.md)** - Code style guide
- 🧪 **[Testing Guide](docs/developer/en/guides/TESTING_GUIDE.md)** - Testing strategy and execution

#### API Documentation
- 🔗 **[Common API](docs/developer/en/api/API_COMMON.md)** - Auth, session, i18n
- 👥 **[User Management API](docs/developer/en/api/API_USER.md)** - User CRUD operations
- 📁 **[Category Management API](docs/developer/en/api/API_CATEGORY.md)** - Hierarchical category management
- 💰 **[Transaction Management API](docs/developer/en/api/API_TRANSACTION.md)** - Transaction data management
- 🏦 **[Account Management API](docs/developer/en/api/API_ACCOUNT.md)** - Account master management
- 🏢 **[Master Data API](docs/developer/en/api/API_MASTER_DATA.md)** - Shops, manufacturers, products
- 📊 **[Aggregation API](docs/developer/en/api/API_AGGREGATION.md)** - Reports and statistics
- ⚙️ **[Settings API](docs/developer/en/api/API_SETTINGS.md)** - User settings management

---

### 📋 Project Information
- 👥 **[Project Participants](docs/etc/PROJECT_PARTICIPANTS.md)** - Contributors list
- 📊 **[AI Development Metrics](docs/etc/AI_DEVELOPMENT_METRICS.md)** - AI-assisted development analysis

---

## 🤝 Contributing

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

## 📄 License

This project is licensed under the terms in the [LICENSE](LICENSE) file.

---

## 🌟 Development Roadmap

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
