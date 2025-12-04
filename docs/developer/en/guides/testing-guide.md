# Testing Guide for Contributors

**Version**: 1.0
**Last Updated**: 2025-12-04
**Target Audience**: Testers (No coding experience required)

---

## 📋 Table of Contents

- [Overview](#overview)
- [Why Your Testing Matters](#why-your-testing-matters)
- [Getting Started](#getting-started)
- [Platform-Specific Testing](#platform-specific-testing)
- [Test Scenarios](#test-scenarios)
- [How to Report Issues](#how-to-report-issues)
- [Testing Checklist](#testing-checklist)
- [FAQ](#faq)

---

## Overview

Thank you for your interest in testing KakeiBon! As a tester, you play a crucial role in improving the quality and reliability of KakeiBon across different platforms.

### Current Testing Status

| Platform | Status | Priority |
|----------|--------|----------|
| **Linux** | ✅ Verified | Low (well-tested) |
| **Windows** | ⚠️ Needs Testing | **HIGH** |
| **macOS** | ⚠️ Needs Testing | **HIGH** |

**We especially need Windows and macOS testers!**

---

## Why Your Testing Matters

### What Makes Your Testing Valuable

1. **Real-World Usage**: You test KakeiBon in real environments
2. **Platform Coverage**: Help verify Windows/macOS compatibility
3. **User Perspective**: Find UX issues developers might miss
4. **Quality Assurance**: Prevent bugs from reaching other users

### No Coding Required

You don't need programming skills! If you can:
- Install and run applications
- Follow step-by-step instructions
- Describe what you see and what happens
- Take screenshots

**You can be a valuable tester!**

---

## Getting Started

### Prerequisites

#### What You Need

1. **A computer** (Windows, macOS, or Linux)
2. **Basic computer skills** (installing apps, taking screenshots)
3. **30-60 minutes** for initial testing
4. **Patience** and attention to detail

#### What You'll Install

- **Build tools** (for compiling KakeiBon from source)
- **KakeiBon** (the app you'll test)

### Installation

Follow the installation guide for your platform:

- **[Windows Installation](../../../user/en/installation.md#windows)**
- **[macOS Installation](../../../user/en/installation.md#macos)**
- **[Linux Installation](../../../user/en/installation.md#linux)**

### Before You Start Testing

1. **Read the Quick Start Guide**
   - [Quick Start Guide](../../../user/en/quick-start.md)
   - Understand basic KakeiBon functionality

2. **Prepare a Testing Environment**
   - Use a test user account (not your real data)
   - Keep a notepad or text file for notes
   - Have a screenshot tool ready

3. **Join the Testing Community**
   - [Submit Testing Feedback Issue](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=testing-feedback.yml)
   - Introduce yourself and your test environment

---

## Platform-Specific Testing

### Windows Testing (HIGH PRIORITY)

#### Environment Information to Report

```
OS Version: Windows 11 23H2 (or Windows 10 22H2)
CPU: Intel Core i7-12700K (or your CPU)
RAM: 16GB (or your RAM)
Display: 1920x1080 (or your resolution)
```

#### Known Considerations

- **WebView2**: Required for Tauri apps on Windows
- **Permissions**: May need admin rights for installation
- **Antivirus**: Some antivirus software may flag Tauri apps

#### Critical Test Areas

1. **Installation**
   - Does the build process complete?
   - Can you launch the app?

2. **File System**
   - Database location: `%APPDATA%\KakeiBon\`
   - Can the app create and access files?

3. **UI Rendering**
   - Do fonts display correctly?
   - Are window controls working?
   - Does the UI scale properly with DPI settings?

---

### macOS Testing (HIGH PRIORITY)

#### Environment Information to Report

```
OS Version: macOS 14.2 Sonoma (or your macOS version)
Hardware: MacBook Pro (M2) or iMac (Intel)
RAM: 16GB (or your RAM)
Display: 2560x1600 (or your resolution)
```

#### Known Considerations

- **Apple Silicon vs Intel**: Different build processes
- **Gatekeeper**: May need to allow unsigned apps
- **Permissions**: May need to grant database access

#### Critical Test Areas

1. **Installation**
   - Does Homebrew setup work?
   - Do build tools install correctly?

2. **File System**
   - Database location: `~/Library/Application Support/KakeiBon/`
   - Permissions for reading/writing files

3. **UI Rendering**
   - Retina display compatibility
   - Menu bar integration
   - Window management

---

### Linux Testing (LOW PRIORITY)

Linux is already well-tested, but additional testing is still valuable!

#### Environment Information to Report

```
Distribution: Ubuntu 24.04 (or your distro)
Desktop Environment: GNOME 46 (or your DE)
Display Server: Wayland (or X11)
```

#### Test Areas

- Different distributions (Fedora, Arch, etc.)
- Different desktop environments (KDE, XFCE, etc.)
- Wayland vs X11

---

## Test Scenarios

### Scenario 1: First Launch (Critical)

**Goal**: Verify the app launches and initial setup works

**Steps**:
1. Launch KakeiBon
2. Observe the startup screen
3. Create an admin user
4. Log in with the new user

**Expected Result**:
- App launches without errors
- UI displays correctly
- User creation succeeds
- Login succeeds

**Report if**:
- App crashes or freezes
- UI elements are misaligned
- User creation fails
- Login fails

---

### Scenario 2: User Management

**Goal**: Test user creation, editing, and deletion

**Steps**:
1. Log in as admin
2. Navigate to User Management
3. Create a new user (non-admin)
4. Edit the user's information
5. Try to delete the user
6. Log out and log in as the new user

**Expected Result**:
- All operations complete successfully
- UI updates reflect changes
- New user can log in

**Report if**:
- Any operation fails or freezes
- UI doesn't update
- Data inconsistencies

---

### Scenario 3: Account Management

**Goal**: Test account creation and management

**Steps**:
1. Navigate to Account Management
2. Create a new account (e.g., "Bank Account")
3. Set initial balance
4. Edit the account
5. View account details

**Expected Result**:
- Account is created successfully
- Balance is saved correctly
- Changes are persisted

**Report if**:
- Account creation fails
- Balance is incorrect
- Changes are lost

---

### Scenario 4: Category Management

**Goal**: Test the 3-tier category system

**Steps**:
1. Navigate to Category Management
2. View existing categories
3. Create a new category (if allowed)
4. Edit category names
5. Test language switching (Japanese ↔ English)

**Expected Result**:
- Categories display correctly
- Language switching works
- Japanese and English names display properly

**Report if**:
- Categories don't display
- Language switching fails
- Japanese characters don't render

---

### Scenario 5: Transaction Input (Critical)

**Goal**: Test the core functionality—entering transactions

**Steps**:
1. Navigate to Transaction Input
2. Create a new transaction:
   - Select date
   - Select account
   - Select category
   - Enter amount
   - Add description
3. Add transaction details (items):
   - Select shop
   - Select product
   - Enter quantity and unit price
4. Save the transaction
5. View the transaction list

**Expected Result**:
- All form fields work correctly
- Dropdowns populate with data
- Calculations are correct
- Transaction is saved and appears in the list

**Report if**:
- Form fields don't work
- Dropdowns are empty
- Calculations are wrong
- Transaction isn't saved

---

### Scenario 6: Data Persistence

**Goal**: Ensure data is saved and persists across app restarts

**Steps**:
1. Create some test data (users, accounts, transactions)
2. Close the app completely
3. Relaunch the app
4. Log in
5. Verify all data is still there

**Expected Result**:
- All data persists
- No data loss

**Report if**:
- Data is lost
- Database errors occur

---

### Scenario 7: Language Switching

**Goal**: Test internationalization

**Steps**:
1. Log in
2. Navigate to Settings
3. Change language (Japanese → English or vice versa)
4. Navigate through different screens
5. Verify all text is translated

**Expected Result**:
- Language changes immediately
- All UI elements update
- No untranslated text

**Report if**:
- Language doesn't change
- Some text remains in the old language
- UI breaks after switching

---

### Scenario 8: Font Size Adjustment

**Goal**: Test accessibility feature

**Steps**:
1. Navigate to Settings
2. Change font size (100% → 150% → 200%)
3. Navigate through different screens
4. Verify readability

**Expected Result**:
- Font size changes apply immediately
- UI scales properly
- No text overflow or cutoff

**Report if**:
- Font size doesn't change
- UI breaks or overlaps
- Text is cut off

---

### Scenario 9: Backup and Restore

**Goal**: Test data backup functionality

**Steps**:
1. Create some test data
2. Locate the database file:
   - Windows: `%APPDATA%\KakeiBon\KakeiBonDB.sqlite3`
   - macOS: `~/Library/Application Support/KakeiBon/KakeiBonDB.sqlite3`
   - Linux: `~/.local/share/KakeiBon/KakeiBonDB.sqlite3`
3. Copy the database file to a backup location
4. Modify or delete some data in the app
5. Close the app
6. Restore the backup file
7. Relaunch the app
8. Verify data is restored

**Expected Result**:
- Database file can be located
- Backup and restore work correctly
- Data is fully restored

**Report if**:
- Can't locate database file
- Restore doesn't work
- Data corruption

---

### Scenario 10: Stress Testing

**Goal**: Test performance with large datasets

**Steps**:
1. Create many records:
   - 10+ accounts
   - 100+ transactions
2. Navigate through lists
3. Use filters and search
4. Observe performance

**Expected Result**:
- App remains responsive
- Lists load quickly
- No crashes or freezes

**Report if**:
- App slows down significantly
- Lists take too long to load
- App crashes with large data

---

## How to Report Issues

### What to Include in a Bug Report

1. **Platform and Environment**
   ```
   OS: Windows 11 23H2
   CPU: Intel Core i7
   RAM: 16GB
   Display: 1920x1080
   KakeiBon Version: v1.0.1
   ```

2. **Steps to Reproduce**
   ```
   1. Launch the app
   2. Navigate to Transaction Input
   3. Click "Save" without entering data
   4. App crashes
   ```

3. **Expected vs Actual Result**
   ```
   Expected: Validation error message
   Actual: App crashes completely
   ```

4. **Screenshots or Videos**
   - Attach screenshots showing the issue
   - Screen recordings are even better!

5. **Error Messages or Logs**
   - Copy any error messages
   - Check console logs (if applicable)

### How to Submit

**Method 1: GitHub Issue (Recommended)**

1. Go to [Issues](https://github.com/BonoJovi/KakeiBonByRust/issues)
2. Click "New Issue"
3. Select "Testing Feedback"
4. Fill in all fields
5. Submit

**Method 2: Email**

Send to: [bonojovi@zundou.org](mailto:bonojovi@zundou.org)
- Subject: `[Testing] Issue on [Platform]`
- Include all information listed above

---

## Testing Checklist

Use this checklist to track your testing progress:

### ✅ Installation & Setup
- [ ] Successfully built from source
- [ ] App launches without errors
- [ ] Initial admin user created
- [ ] Database file created in correct location

### ✅ User Management
- [ ] Admin user login
- [ ] Create new user
- [ ] Edit user
- [ ] Delete user
- [ ] Non-admin user login
- [ ] User permissions work correctly

### ✅ Master Data Management
- [ ] Account creation/editing
- [ ] Category browsing
- [ ] Shop creation/editing
- [ ] Maker creation/editing
- [ ] Product creation/editing

### ✅ Transaction Management
- [ ] Create transaction
- [ ] Add transaction details
- [ ] Edit transaction
- [ ] Delete transaction
- [ ] Filter transactions
- [ ] Pagination works

### ✅ UI/UX
- [ ] All buttons work
- [ ] Forms validate input
- [ ] Dropdowns populate correctly
- [ ] Date pickers work
- [ ] Tables display data correctly
- [ ] Modals/dialogs function properly

### ✅ Internationalization
- [ ] Language switching (ja ↔ en)
- [ ] All text translates
- [ ] Japanese characters display correctly
- [ ] No untranslated strings

### ✅ Settings
- [ ] Font size adjustment (100%, 150%, 200%)
- [ ] Language preference saves
- [ ] Settings persist across restarts

### ✅ Data Persistence
- [ ] Data saves correctly
- [ ] Data persists after app restart
- [ ] Database integrity maintained

### ✅ Performance
- [ ] App is responsive
- [ ] No freezing or hanging
- [ ] Handles large datasets
- [ ] No memory leaks

---

## FAQ

### Q: I found a bug. Is it already known?

**A**: Check [existing issues](https://github.com/BonoJovi/KakeiBonByRust/issues) first. If you don't find it, report it!

### Q: How much testing is expected?

**A**: Whatever you can contribute! Even testing a single scenario is valuable.

### Q: Can I test on a virtual machine?

**A**: Yes, but please note that in your report. Real hardware is preferred for accurate results.

### Q: What if I don't have time to test everything?

**A**: Focus on the areas marked **HIGH PRIORITY** or scenarios marked **Critical**.

### Q: Should I test with real data?

**A**: **No!** Always use test data. Never use your real financial information during testing.

### Q: Can I automate testing?

**A**: Manual testing is preferred for now, but automation contributions are welcome!

### Q: How will my testing be credited?

**A**: Testers are acknowledged in CHANGELOG.md and contributors section.

---

## Additional Resources

- **[Contributing Guide](../../../../CONTRIBUTING.md)**
- **[Translation Guide](translation-guide.md)**
- **[Installation Guide](../../../user/en/installation.md)**
- **[Quick Start Guide](../../../user/en/quick-start.md)**
- **[Submit Testing Feedback](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=testing-feedback.yml)**

---

## Need Help?

- **GitHub Issues**: [Testing Discussion](https://github.com/BonoJovi/KakeiBonByRust/issues?q=label%3Atesting)
- **Email**: [bonojovi@zundou.org](mailto:bonojovi@zundou.org)

---

**Thank you for helping improve KakeiBon's quality!**

Your testing contributions make KakeiBon better for everyone.

**- The KakeiBon Team**
