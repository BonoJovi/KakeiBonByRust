# Setup Guide

**Last Updated**: 2024-12-05 04:59 JST

This guide explains how to install and set up KakeiBon.

---

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Installation](#installation)
3. [Initial Setup](#initial-setup)
4. [Troubleshooting](#troubleshooting)

---

## System Requirements

### Supported Operating Systems
- Linux (Ubuntu 20.04 or later recommended)
- Windows 10/11 (build from source)
- macOS 10.15 or later (build from source)

### Required Software
The following tools must be installed on your system:

| Software | Version | Purpose |
|----------|---------|---------|
| Rust | 1.70+ | Building the application |
| Node.js | 18+ | Frontend dependencies |
| npm | 9+ | Package manager |

---

## Installation

Currently, pre-built binaries are not available. You need to build from source code.

### 1. Install Prerequisites

#### Linux (Ubuntu/Debian)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Node.js and npm
sudo apt update
sudo apt install nodejs npm

# Install required libraries
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Windows
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Node.js](https://nodejs.org/)
3. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)

#### macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Node.js (using Homebrew)
brew install node
```

### 2. Download Source Code

```bash
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust
```

### 3. Build the Application

```bash
# Install frontend dependencies
npm install

# Build and run in development mode
npm run tauri dev

# Or build for production
npm run tauri build
```

### 4. Install the Application

#### Linux
After building, the executable is located at:
```
target/release/kakeibonbyrust
```

You can copy it to a location in your PATH:
```bash
sudo cp target/release/kakeibonbyrust /usr/local/bin/
```

#### Windows
After building, the installer is located at:
```
target\release\bundle\msi\KakeiBon_x.x.x_x64.msi
```

Double-click the installer to install.

#### macOS
After building, the app bundle is located at:
```
target/release/bundle/macos/KakeiBon.app
```

Copy it to your Applications folder:
```bash
cp -r target/release/bundle/macos/KakeiBon.app /Applications/
```

---

## Initial Setup

### 1. First Launch

When you launch KakeiBon for the first time:

1. **Administrator Setup Screen** appears
2. Create your administrator account:
   - Enter a username (4-20 characters)
   - Enter a secure password (16+ characters)
   - Confirm your password
3. Click "Create Admin & Login"

### 2. Language Settings

- The application automatically detects your system language
- Currently supports:
  - Japanese (日本語)
  - English
- You can change the language in Settings after login

### 3. Data Location

Application data is stored at:
- **Linux**: `~/.local/share/kakeibonbyrust/`
- **Windows**: `C:\Users\<username>\AppData\Roaming\kakeibonbyrust\`
- **macOS**: `~/Library/Application Support/kakeibonbyrust/`

---

## Troubleshooting

### Build Fails

**Problem**: Build errors during `npm run tauri build`

**Solutions**:
1. Ensure all prerequisites are installed
2. Update Rust: `rustup update`
3. Clean build cache: `cargo clean && npm run tauri build`
4. Check for detailed error messages in the console

### Application Won't Start

**Problem**: Application crashes on startup

**Solutions**:
1. Check if database file is corrupted
2. Verify file permissions on data directory
3. Check console output for error messages
4. Try removing the data directory (backup first!)

### Database Errors

**Problem**: "Database is locked" or similar errors

**Solutions**:
1. Ensure no other instance is running
2. Restart the application
3. Check file permissions on database file

### Language Issues

**Problem**: Wrong language or missing translations

**Solutions**:
1. Check system locale settings
2. Manually change language in Settings
3. Report missing translations on GitHub

---

## Getting Help

If you encounter issues not covered here:

1. Check existing [GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues)
2. Create a new issue with:
   - Your operating system and version
   - Steps to reproduce the problem
   - Error messages or screenshots
3. Review the [Developer Documentation](../../developer/en/) for more technical details

---

**Next Steps**: After installation, see the [User Guide](USER_GUIDE.md) for how to use KakeiBon.
