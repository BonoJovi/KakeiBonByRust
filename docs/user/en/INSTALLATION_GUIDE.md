# Installation Guide

**Version**: 1.0.1  
**Last Updated**: 2025-12-03 14:40 JST

---

## Table of Contents

- [System Requirements](#system-requirements)
- [Linux Installation](#linux-installation)
- [Windows Installation](#windows-installation)
- [macOS Installation](#macos-installation)
- [Building from Source](#building-from-source)
- [First Launch](#first-launch)
- [Troubleshooting](#troubleshooting)

---

## System Requirements

### Minimum Requirements
- **OS**: Linux (Ubuntu 20.04+, Debian 11+), Windows 10+, macOS 10.15+
- **RAM**: 512MB
- **Disk Space**: 100MB
- **Display**: 1024x768 or higher

### Recommended
- **RAM**: 1GB or more
- **Display**: 1920x1080 or higher

---

## Linux Installation

### Method 1: Using Pre-built Binary (Recommended)

**Note**: Currently, we do not distribute pre-built binaries. Please use Method 2 (build from source).

### Method 2: Build from Source

#### Prerequisites

KakeiBon uses Tauri v2, which requires the following system dependencies:

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

# Fedora/RHEL
sudo dnf install \
  webkit2gtk4.1-devel \
  openssl-devel \
  curl \
  wget \
  file \
  libappindicator-gtk3-devel \
  librsvg2-devel

# Arch Linux
sudo pacman -S \
  webkit2gtk \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  gtk3 \
  libappindicator-gtk3 \
  librsvg \
  libvips
```

**Important Notes:**
- Package names may vary between distributions
- If build fails due to missing dependencies, please report via [GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues)

#### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### Build KakeiBon

```bash
# Clone repository
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust

# Checkout stable version
git checkout v1.0.1

# Build
cargo tauri build

# Built binary location
# ./target/release/bundle/
```

#### Install Binary

```bash
# Copy binary to local bin (example)
sudo cp target/release/kakeibon /usr/local/bin/

# Or install .deb package (if available)
sudo dpkg -i target/release/bundle/deb/kakeibon_1.0.1_amd64.deb
```

---

## Windows Installation

### Method 1: Using Installer (Coming Soon)

Pre-built Windows installer will be available in future releases.

### Method 2: Build from Source

#### Prerequisites

1. **Install Rust**: Download from [rustup.rs](https://rustup.rs/)
2. **Install Build Tools**: Visual Studio Build Tools or Visual Studio Community
3. **Install WebView2**: Usually pre-installed on Windows 11 (download from [Microsoft](https://developer.microsoft.com/microsoft-edge/webview2/) if needed)

#### Build

```powershell
# Clone repository
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust

# Checkout stable version
git checkout v1.0.1

# Build
cargo tauri build

# Built binary location
# .\target\release\
```

**Note**: Windows build instructions are based on Tauri documentation and have not been verified in actual environment. If you encounter issues, please report via [GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues).

---

## macOS Installation

### Method 1: Using DMG (Coming Soon)

Pre-built macOS DMG will be available in future releases.

### Method 2: Build from Source

#### Prerequisites

1. **Install Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```

2. **Install Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

#### Build

```bash
# Clone repository
git clone https://github.com/BonoJovi/KakeiBonByRust.git
cd KakeiBonByRust

# Checkout stable version
git checkout v1.0.1

# Build
cargo tauri build

# Built binary location
# ./target/release/bundle/
```

**Note**: macOS build instructions are based on Tauri documentation and have not been verified in actual environment. If you encounter issues, please report via [GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues).

---

## Building from Source

For detailed build instructions, see platform-specific sections above.

### Development Build

```bash
cargo tauri dev
```

### Production Build

```bash
cargo tauri build
```

---

## First Launch

### 1. Initial Setup

When you launch KakeiBon for the first time:

1. **Admin Account Creation Screen** will appear
2. Enter administrator username (3-20 characters)
3. Enter password (minimum 16 characters)
4. Click "Create Administrator"

### 2. Database Location

KakeiBon stores data in:
- **Linux**: `$HOME/.kakeibon/KakeiBonDB.sqlite3`
- **Windows**: `%USERPROFILE%\.kakeibon\KakeiBonDB.sqlite3`
- **macOS**: `$HOME/.kakeibon/KakeiBonDB.sqlite3`

### 3. Next Steps

After login, you can:
- Create regular user accounts (Admin only)
- Set up category hierarchy
- Add accounts (bank accounts, cash, credit cards)
- Start recording transactions

For detailed usage, see [Quick Start Guide](QUICK_START_GUIDE.md) (coming soon).

---

## Troubleshooting

### Build Fails with Missing Dependencies

**Linux**: Check error messages for missing libraries and install them via package manager.

Example:
```bash
# If error mentions "webkit2gtk"
sudo apt install libwebkit2gtk-4.1-dev

# If error mentions "libssl"
sudo apt install libssl-dev
```

### Database Permission Issues

Ensure `~/.kakeibon/` directory has correct permissions:

```bash
chmod 700 ~/.kakeibon/
chmod 600 ~/.kakeibon/*.sqlite3
```

### Application Won't Start

Check logs:
```bash
# Linux
journalctl -xe
tail -f ~/.local/share/kakeibon/logs/*

# Check console output when running from terminal
./kakeibon
```

### For More Issues

See [Troubleshooting Guide](TROUBLESHOOTING.md) or report via [GitHub Issues](https://github.com/BonoJovi/KakeiBonByRust/issues).

---

## Contributing

If you encounter issues not covered in this guide, please:
1. Open an [issue](https://github.com/BonoJovi/KakeiBonByRust/issues) with details
2. Specify your OS, version, and error messages
3. Your feedback helps improve this guide!

---

**Next**: [Quick Start Guide](QUICK_START_GUIDE.md) (coming soon)
