# KakeiBonByRust
Household Budget App "KakeiBon" Built with Rust Language.

## Overview
A modern household budget management application built with Rust and Tauri framework.

## Features
- 💰 Expense and income tracking
- 👥 Multi-user support with role-based access control
- 🔐 Secure password management (Argon2id)
- 🔒 Data encryption (AES-256-GCM)
- 🌐 Multilingual support (English, Japanese)
- 📊 Hierarchical category management
- ⚙️ User settings management

## Technology Stack
- **Frontend**: HTML, CSS, JavaScript
- **Backend**: Rust
- **Framework**: Tauri v2.8.5
- **Database**: SQLite with WAL mode
- **Security**: Argon2id (password hashing), AES-256-GCM (data encryption)

## Documentation

📚 **[日本語版 (Japanese)](./README_ja.md)** is also available.

Detailed documentation is available in the [docs/en](./docs/en) directory:

- [User Management](./docs/en/USER_MANAGEMENT.md) - User registration, authentication, and management
- [Encryption Management](./docs/en/ENCRYPTION_MANAGEMENT.md) - Data encryption and re-encryption system
- [Settings Management](./docs/en/SETTINGS_MANAGEMENT.md) - User settings and preferences
- [I18N Implementation](./docs/en/I18N_IMPLEMENTATION.md) - Multilingual support system
- [Test Summary](./docs/en/TEST_SUMMARY.md) - Test results and coverage

## Getting Started

### Prerequisites
- Rust 1.70+
- Node.js (for Tauri development)

### Build
```bash
cargo build
```

### Run Tests
```bash
cargo test --lib
```

### Run Application
```bash
cargo tauri dev
```

## Project Structure
```
KakeiBonByRust/
├── src/               # Rust source code
│   ├── services/      # Business logic services
│   ├── db.rs          # Database management
│   ├── crypto.rs      # Encryption utilities
│   ├── consts.rs      # Application constants
│   └── ...
├── res/               # Resources
│   └── sql/           # SQL schema files
├── docs/              # Documentation
│   ├── en/            # English documentation
│   └── ja/            # Japanese documentation
└── $HOME/.kakeibon/   # User data directory
    ├── KakeiBonDB.sqlite3
    └── KakeiBon.json
```

## Test Results
```
Total Tests: 90
Passed: 90
Failed: 0
Success Rate: 100%
```

## Security Features
- Password hashing with Argon2id
- Data encryption with AES-256-GCM
- Password length: 16-128 characters
- Password complexity requirements enforced
- Re-encryption on password change
- Role-based access control

## License
See [LICENSE](./LICENSE) file for details.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## Contact
For questions or feedback, please open an issue on GitHub.
