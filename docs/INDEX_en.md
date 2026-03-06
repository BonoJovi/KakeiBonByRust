# KakeiBon Documentation Index

**Last Updated**: 2025-12-05 06:35 JST

This document provides quick access to all KakeiBon project documentation.

---

## [List] Table of Contents

- [Quick Start](#quick-start)
- [User Documentation](#user-documentation)
- [Developer Documentation](#developer-documentation)
- [API Reference](#api-reference)
- [Design Documentation](#design-documentation)
- [Screen-by-Screen Quick Reference](#screen-by-screen-quick-reference)
- [Keyword Index](#keyword-index)

---

## [Rocket] Quick Start

### For Users
- [Installation Guide](user/en/INSTALLATION_GUIDE.md) - How to install the application
- [Quick Start Guide](user/en/QUICK_START_GUIDE.md) - Get started in 5 minutes
- [Setup Guide](user/en/SETUP_GUIDE.md) - Initial configuration steps

### For Developers
- [Development Setup](developer/en/guides/DEVELOPMENT_SETUP.md) - Setting up development environment (Japanese only)
- [Developer Guide](developer/en/guides/DEVELOPER_GUIDE.md) - How to start developing

---

## [Users] User Documentation

### Basic Guides
| Document | Description |
|----------|-------------|
| [User Manual](user/en/USER_MANUAL_en.md) | Complete feature descriptions and operations |
| [Setup Guide](user/en/SETUP_GUIDE.md) | Detailed initial setup procedures |
| [Installation Guide](user/en/INSTALLATION_GUIDE.md) | Installation methods |
| [Quick Start Guide](user/en/QUICK_START_GUIDE.md) | 5-minute quick guide |

### Feature-Specific Guides
| Document | Description |
|----------|-------------|
| [Aggregation User Guide](user/en/AGGREGATION_USER_GUIDE.md) | How to use monthly, daily, weekly, yearly, and period aggregations |

### Troubleshooting
| Document | Description |
|----------|-------------|
| [Troubleshooting](user/en/TROUBLESHOOTING.md) | Problem-solving guide |

---

## [PC] Developer Documentation

### Setup & Environment
| Document | Description |
|----------|-------------|
| [Development Setup](developer/ja/setup/DEVELOPMENT_SETUP.md) | Rust, Node.js, Tauri environment setup (Japanese only) |
| [Database Configuration](developer/en/guides/DATABASE_CONFIGURATION.md) | SQLite database configuration |
| [Database Migration](developer/en/guides/DATABASE_MIGRATION.md) | Schema changes and migration procedures |

### Development Guides
| Document | Description |
|----------|-------------|
| [Developer Guide](developer/en/guides/DEVELOPER_GUIDE.md) | Development process and workflow |
| [Coding Standards](developer/en/guides/CODING_STANDARDS_en.md) | Rust/JavaScript/CSS coding standards |
| [Testing Guide](developer/en/guides/TESTING_GUIDE_en.md) | Test strategy and implementation |
| [Documentation Policy](developer/en/guides/DOCUMENTATION_POLICY.md) | Documentation creation rules |

### Feature Implementation Guides
| Document | Description |
|----------|-------------|
| [User Management UI](developer/en/guides/USER_MANAGEMENT_UI.md) | User management screen implementation |
| [Category Management UI](developer/en/guides/CATEGORY_MANAGEMENT_UI.md) | Category management screen implementation |
| [Account Management UI](developer/en/guides/ACCOUNT_MANAGEMENT_UI.md) | Account management screen implementation |
| [Transaction Management UI](developer/en/guides/TRANSACTION_MANAGEMENT_UI.md) | Transaction management screen implementation |
| [Transaction Management UI V2](developer/en/guides/TRANSACTION_MANAGEMENT_UI_V2.md) | Transaction management screen revised version |
| [IS_DISABLED Implementation Guide](developer/en/guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md) | Soft delete feature implementation pattern |
| [Input Screens Implementation](developer/en/guides/INPUT_SCREENS_IMPLEMENTATION.md) | Implementation of all input screens |
| [Frontend Design Phase 4](developer/en/guides/FRONTEND_DESIGN_PHASE4.md) | Frontend design and implementation |

### Internationalization & UI
| Document | Description |
|----------|-------------|
| [I18N Implementation](developer/en/guides/I18N_IMPLEMENTATION.md) | Multilingual support implementation |
| [I18N Resources](developer/en/guides/I18N_RESOURCES.md) | Translation resource statistics |
| [Translation Guide](developer/en/guides/translation-guide.md) | How to add and update translations |
| [Dynamic Language Menu](developer/en/guides/DYNAMIC_LANGUAGE_MENU.md) | Language switching menu implementation |
| [Font Size Implementation](developer/en/guides/font-size-implementation.md) | Font size change feature implementation |

### Security & Encryption
| Document | Description |
|----------|-------------|
| [Encryption Management](developer/en/guides/ENCRYPTION_MANAGEMENT.md) | AES-256-GCM encryption implementation |
| [Settings Management](developer/en/guides/SETTINGS_MANAGEMENT.md) | User settings management |

### Testing
| Document | Description |
|----------|-------------|
| [Testing Guide](developer/en/guides/testing-guide.md) | Test strategy and automated testing |
| [Testing Strategy](developer/en/testing/TESTING.md) | Comprehensive testing strategy |
| [Test Summary](developer/en/testing/TEST_SUMMARY.md) | Test results summary |
| [Session Management Test Report](developer/en/testing/session-management-test-report.md) | Session management test results |

---

## [Books] API Reference

### Screen-by-Screen API Documentation
| Document | Description | Screen |
|----------|-------------|--------|
| [Common API](developer/en/api/API_COMMON.md) | Session, I18N, Encryption, System, Validation | All screens |
| [Auth & Setup API](developer/en/api/API_AUTH.md) | Admin setup, Login | index.html |
| [User Management API](developer/en/api/API_USER.md) | User CRUD, Password change | user-management.html |
| [Category Management API](developer/en/api/API_CATEGORY.md) | 3-level category CRUD, Order change | category-management.html |
| [Account Management API](developer/en/api/API_ACCOUNT.md) | Account CRUD, Template management | account-management.html |
| [Transaction Management API](developer/en/api/API_TRANSACTION.md) | Header CRUD, Filters, Memo management | transaction-management.html |
| [Master Data API](developer/en/api/API_MASTER_DATA.md) | Shop, Manufacturer, Product CRUD | shop/manufacturer/product-management.html |
| [Aggregation API](developer/en/api/API_AGGREGATION.md) | Monthly, Daily, Weekly, Yearly, Period aggregations | aggregation-*.html |
| [Settings API](developer/en/api/API_SETTINGS.md) | Font size, Language, Encryption settings | settings.html |

### API Command List
**Total 100 Commands** (API Distribution):
- Common API: 24 commands
- Category Management: 16 commands
- Transaction Management: 14 commands
- User Management: 13 commands
- Settings: 10 commands
- Session: 9 commands
- Others: 14 commands

---

## [Build]️ Design Documentation

### Architecture Design
| Document | Description |
|----------|-------------|
| [Architecture Design](developer/ja/design/ARCHITECTURE.md) | Overall system architecture (Japanese only) |
| [Database Design](developer/en/design/DATABASE_DESIGN_en.md) | ERD, Table definitions, Indexes |
| [Security Design](design/SECURITY_DESIGN_en.md) | Authentication, Encryption, Password hashing |
| [UI Design](design/en/UI_DESIGN_en.md) | UI/UX design principles |

### Detailed Design
| Document | Description |
|----------|-------------|
| [Transaction Design V2](design/architecture/TRANSACTION_DESIGN_V2.md) | Detailed transaction feature design |
| [Session Management Spec](design/architecture/session-management-spec.md) | Session management specification |
| [Tax Calculation Logic](design/architecture/tax-calculation-logic.md) | Tax-inclusive/exclusive calculation, Rounding |

### Requirements & Problem Solving
| Document | Description |
|----------|-------------|
| [Transaction Requirements](design/requirements/TRANSACTION_REQUIREMENTS.md) | Transaction feature requirements |
| [Design Issues and Fixes](design/decisions/DESIGN_ISSUES_AND_FIXES.md) | Design problems and solutions |

---

## [Desktop]️ Screen-by-Screen Quick Reference

### Authentication & Setup (index.html)
- **API**: [Auth & Setup API](developer/en/api/API_AUTH.md)
- **Features**: Admin setup, Login, Session management

### User Management (user-management.html)
- **API**: [User Management API](developer/en/api/API_USER.md)
- **Implementation Guide**: [User Management UI](developer/en/guides/USER_MANAGEMENT_UI.md)
- **Features**: Add/Edit/Delete users, Password change

### Category Management (category-management.html)
- **API**: [Category Management API](developer/en/api/API_CATEGORY.md)
- **Implementation Guide**: [Category Management UI](developer/en/guides/CATEGORY_MANAGEMENT_UI.md)
- **Features**: 3-level category (Major/Middle/Minor) CRUD, Order change

### Account Management (account-management.html)
- **API**: [Account Management API](developer/en/api/API_ACCOUNT.md)
- **Implementation Guide**: [Account Management UI](developer/en/guides/ACCOUNT_MANAGEMENT_UI.md)
- **Features**: Account CRUD, Template selection, Initial balance

### Transaction Management (transaction-management.html)
- **API**: [Transaction Management API](developer/en/api/API_TRANSACTION.md)
- **Implementation Guide**: [Transaction Management UI](developer/en/guides/TRANSACTION_MANAGEMENT_UI.md)
- **Features**: Header CRUD, Filters, Pagination, Memo management

### Shop Management (shop-management.html)
- **API**: [Master Data API](developer/en/api/API_MASTER_DATA.md) (Shop section)
- **Features**: Shop CRUD, IS_DISABLED feature

### Manufacturer Management (manufacturer-management.html)
- **API**: [Master Data API](developer/en/api/API_MASTER_DATA.md) (Manufacturer section)
- **Features**: Manufacturer CRUD, IS_DISABLED feature

### Product Management (product-management.html)
- **API**: [Master Data API](developer/en/api/API_MASTER_DATA.md) (Product section)
- **Features**: Product CRUD, Manufacturer integration, IS_DISABLED feature

### Aggregation Screens (aggregation-*.html)
- **API**: [Aggregation API](developer/en/api/API_AGGREGATION.md)
- **User Guide**: [Aggregation User Guide](user/en/AGGREGATION_USER_GUIDE.md)
- **Features**: Monthly, Daily, Weekly, Yearly, Period aggregations

### Settings Screen (settings.html)
- **API**: [Settings API](developer/en/api/API_SETTINGS.md)
- **Implementation Guide**: [Settings Management](developer/en/guides/SETTINGS_MANAGEMENT.md)
- **Features**: Font size, Language, Encryption key management

---

## [Search] Keyword Index

### A
- **Account**: [Account Management API](developer/en/api/API_ACCOUNT.md), [Account Management UI](developer/en/guides/ACCOUNT_MANAGEMENT_UI.md)
- **Aggregation**: [Aggregation API](developer/en/api/API_AGGREGATION.md), [Aggregation User Guide](user/en/AGGREGATION_USER_GUIDE.md)
- **API**: See [API Reference](#api-reference) section
- **Architecture**: [Architecture Design](developer/ja/design/ARCHITECTURE.md) (Japanese only)
- **Authentication**: [Auth & Setup API](developer/en/api/API_AUTH.md)

### C
- **Category**: [Category Management API](developer/en/api/API_CATEGORY.md), [Category Management UI](developer/en/guides/CATEGORY_MANAGEMENT_UI.md)
- **Coding Standards**: [Coding Standards](developer/en/guides/CODING_STANDARDS_en.md)
- **CRUD**: See each management screen API reference

### D
- **Database**: [Database Design](developer/en/design/DATABASE_DESIGN_en.md), [Database Configuration](developer/en/guides/DATABASE_CONFIGURATION.md)
- **Development Environment**: [Development Setup](developer/ja/setup/DEVELOPMENT_SETUP.md) (Japanese only)

### E
- **Encryption**: [Encryption Management](developer/en/guides/ENCRYPTION_MANAGEMENT.md), [Security Design](design/SECURITY_DESIGN_en.md)

### F
- **Font Size**: [Font Size Implementation](developer/en/guides/font-size-implementation.md), [Settings API](developer/en/api/API_SETTINGS.md)

### I
- **I18N**: [I18N Implementation](developer/en/guides/I18N_IMPLEMENTATION.md)
- **IS_DISABLED**: [IS_DISABLED Implementation Guide](developer/en/guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md)

### L
- **Login**: [Auth & Setup API](developer/en/api/API_AUTH.md)

### M
- **Manufacturer**: [Master Data API](developer/en/api/API_MASTER_DATA.md) (Manufacturer section)
- **Master Data**: [Master Data API](developer/en/api/API_MASTER_DATA.md)
- **Memo**: [Transaction Management API](developer/en/api/API_TRANSACTION.md) (Memo management section)
- **Migration**: [Database Migration](developer/en/guides/DATABASE_MIGRATION.md)
- **Multilingual**: [I18N Implementation](developer/en/guides/I18N_IMPLEMENTATION.md), [Translation Guide](developer/en/guides/translation-guide.md)

### P
- **Product**: [Master Data API](developer/en/api/API_MASTER_DATA.md) (Product section)

### R
- **Rust**: [Coding Standards](developer/en/guides/CODING_STANDARDS_en.md)

### S
- **Security**: [Security Design](design/SECURITY_DESIGN_en.md), [Encryption Management](developer/en/guides/ENCRYPTION_MANAGEMENT.md)
- **Session**: [Session Management Spec](design/architecture/session-management-spec.md), [Common API](developer/en/api/API_COMMON.md)
- **Setup**: [Setup Guide](user/en/SETUP_GUIDE.md), [Development Setup](developer/ja/setup/DEVELOPMENT_SETUP.md) (Japanese only)
- **Shop**: [Master Data API](developer/en/api/API_MASTER_DATA.md) (Shop section)
- **Soft Delete**: [IS_DISABLED Implementation Guide](developer/en/guides/IS_DISABLED_IMPLEMENTATION_GUIDE.md)
- **SQLite**: [Database Configuration](developer/en/guides/DATABASE_CONFIGURATION.md)

### T
- **Tauri**: [Development Setup](developer/ja/setup/DEVELOPMENT_SETUP.md) (Japanese only)
- **Tax Calculation**: [Tax Calculation Logic](design/architecture/tax-calculation-logic.md)
- **Testing**: [Testing Guide](developer/en/guides/testing-guide.md), [Test Summary](developer/en/testing/TEST_SUMMARY.md)
- **Transaction**: [Transaction Management API](developer/en/api/API_TRANSACTION.md), [Transaction Management UI](developer/en/guides/TRANSACTION_MANAGEMENT_UI.md)
- **Translation**: [Translation Guide](developer/en/guides/translation-guide.md)
- **Troubleshooting**: [Troubleshooting](user/en/TROUBLESHOOTING.md)

### U
- **UI**: [UI Design](design/en/UI_DESIGN_en.md)
- **User Management**: [User Management API](developer/en/api/API_USER.md), [User Management UI](developer/en/guides/USER_MANAGEMENT_UI.md)

### V
- **Validation**: [Common API](developer/en/api/API_COMMON.md) (Validation section)

---

## [Book] Other Documentation

### Project Information
| Document | Description |
|----------|-------------|
| [Project Participants](etc/PROJECT_PARTICIPANTS.md) | Developer and contributor information |
| [AI Development Metrics](etc/AI_DEVELOPMENT_METRICS.md) | AI-assisted development statistics |
| [Accessibility Indicators](etc/ACCESSIBILITY_INDICATORS.md) | Accessibility compliance status |
| [Manufacturer Product Management](etc/MANUFACTURER_PRODUCT_MANAGEMENT.md) | Manufacturer and product management overview |

### Security
| Document | Description |
|----------|-------------|
| [Dependabot Alert #1](security/alerts/dependabot-alert-1-glib.md) | glib vulnerability alert |

---

## [Note] Related Resources

- [Root README](../README.md) - Project top page
- [Japanese README](../README_ja.md) - Japanese navigation hub
- [English README](../README_en.md) - English navigation hub
- [TODO](../TODO.md) - Development task list
- [CHANGELOG (English)](../CHANGELOG_en.md) - Change history
- [CONTRIBUTING](../CONTRIBUTING.md) - Contribution guidelines
- [CODE_OF_CONDUCT (English)](../CODE_OF_CONDUCT_en.md) - Code of conduct

---

**Last Updated**: 2025-12-05 06:35 JST
