# KakeiBon User Manual

**Version**: 1.0.1  
**Last Updated**: 2025-12-05 05:31 JST

---

## Table of Contents

1. [Introduction](#introduction)
2. [Initial Setup](#initial-setup)
3. [Basic Operations](#basic-operations)
4. [User Management](#user-management)
5. [Category Management](#category-management)
6. [Account Management](#account-management)
7. [Store Management](#store-management)
8. [Manufacturer & Product Management](#manufacturer--product-management)
9. [Transaction Management](#transaction-management)
10. [Aggregation Features](#aggregation-features)
11. [Settings](#settings)
12. [Troubleshooting](#troubleshooting)

---

## Introduction

KakeiBon is a desktop application designed to support personal finance management.

### Key Features

- **Transaction Management**: Record and manage income, expenses, and transfers
- **Category Management**: Classify expenses with 3-level hierarchical categories
- **Account Management**: Centrally manage multiple account balances
- **Store Management**: Register and manage frequently used stores
- **Product Management**: Record purchased products and manufacturers
- **Aggregation Features**: Daily, weekly, monthly, yearly, and custom period aggregation
- **Multilingual Support**: Japanese and English
- **Security**: Password encryption and data encryption

### System Requirements

- **OS**: Linux (Windows/macOS support planned)
- **Required Software**:
  - Rust 1.70 or higher
  - Node.js 18 or higher
  - SQLite 3.35 or higher
  - Various system libraries (see [Setup Guide](SETUP_GUIDE_en.md) for details)

---

## Initial Setup

### First Launch

1. **Launch the application**
   ```bash
   cargo tauri dev
   ```

2. **Admin Setup Screen**
   - Display language is automatically selected (Japanese/English)
   - Enter administrator information:
     - Username (4-20 characters, alphanumeric)
     - Password (minimum 16 characters)
     - Confirm password

3. **Click "Register" button**
   - User is created and initial setup is complete
   - The main screen is automatically displayed

### Login

- **Username**: Enter your registered username
- **Password**: Enter your password
- **Language Selection**: Select Japanese or English
- Click **"ログイン"/"Login"** button

---

## Basic Operations

### Main Screen Structure

```
┌─────────────────────────────────────────┐
│ Header: User Info | Language | Logout   │
├─────────────────────────────────────────┤
│ Menu (Left Side)                        │
│  - ユーザー管理 (User Management)        │
│  - 費目管理 (Category Management)        │
│  - 口座管理 (Account Management)         │
│  - 店舗管理 (Store Management)           │
│  - メーカー管理 (Manufacturer)           │
│  - 商品管理 (Product Management)         │
│  - 入出金管理 (Transaction Management)   │
│  - 集計 (Aggregation)                    │
│  - 設定 (Settings)                       │
├─────────────────────────────────────────┤
│ Content Area                             │
│  (Screen content displayed here)         │
└─────────────────────────────────────────┘
```

### Common Operations

#### Data Registration

1. Navigate to the target menu
2. Click **"新規登録"/"Add New"** button
3. Enter required information
4. Click **"登録"/"Register"** button

#### Data Editing

1. Select the data to edit from the list
2. Click **"編集"/"Edit"** button
3. Modify the information
4. Click **"更新"/"Update"** button

#### Data Deletion

1. Select the data to delete from the list
2. Click **"削除"/"Delete"** button
3. Confirm the deletion in the dialog
4. Click **"削除"/"Delete"** in confirmation dialog

---

## User Management

**Available to**: Administrator only

### User Registration

1. Click menu **"ユーザー管理"/"User Management"**
2. Click **"新規ユーザー登録"/"Add New User"** button
3. Enter user information:
   - **Username**: 4-20 characters, alphanumeric
   - **Password**: Minimum 16 characters
   - **Confirm Password**: Re-enter password
   - **Role**: Administrator (0) or Regular User (1)
4. Click **"登録"/"Register"** button

### User Editing

1. Select user from the list
2. Click **"編集"/"Edit"** button
3. Modify information (password change is optional)
4. Click **"更新"/"Update"** button

### User Deletion

1. Select user from the list
2. Click **"削除"/"Delete"** button
3. Confirm deletion

**Note**: Cannot delete yourself

---

## Category Management

**Available to**: Administrator and Regular User

Manage expense categories in 3-level hierarchy.

### Category Registration

1. Click menu **"費目管理"/"Category Management"**
2. Click **"新規費目登録"/"Add New Category"** button
3. Enter category information:
   - **Category Name**: Required
   - **Level**: Large (1), Medium (2), Small (3)
   - **Parent Category**: Required for Medium/Small levels
4. Click **"登録"/"Register"** button

### Hierarchical Structure

```
Large Category (Level 1)
  └─ Medium Category (Level 2)
      └─ Small Category (Level 3)
```

Example:
```
Food & Beverages
  └─ Groceries
      └─ Vegetables
```

---

## Account Management

**Available to**: Administrator and Regular User

Manage multiple financial accounts.

### Account Registration

1. Click menu **"口座管理"/"Account Management"**
2. Click **"新規口座登録"/"Add New Account"** button
3. Enter account information:
   - **Account Name**: Required
   - **Initial Balance**: Numeric value
   - **Description**: Optional
4. Click **"登録"/"Register"** button

### Account Balance

- Balance is automatically calculated based on transaction records
- View current balance in the account list

---

## Store Management

**Available to**: Administrator and Regular User

Manage frequently used stores for easier transaction entry.

### Store Registration

1. Click menu **"店舗管理"/"Store Management"**
2. Click **"新規店舗登録"/"Add New Store"** button
3. Enter store information:
   - **Store Name**: Required
   - **Description**: Optional
4. Click **"登録"/"Register"** button

---

## Manufacturer & Product Management

**Available to**: Administrator and Regular User

### Manufacturer Registration

1. Click menu **"メーカー管理"/"Manufacturer Management"**
2. Click **"新規メーカー登録"/"Add New Manufacturer"** button
3. Enter manufacturer name
4. Click **"登録"/"Register"** button

### Product Registration

1. Click menu **"商品管理"/"Product Management"**
2. Click **"新規商品登録"/"Add New Product"** button
3. Enter product information:
   - **Product Name**: Required
   - **Manufacturer**: Select from list
   - **Description**: Optional
4. Click **"登録"/"Register"** button

---

## Transaction Management

**Available to**: Administrator and Regular User

Record and manage income, expenses, and transfers.

### Transaction Types

- **Income**: Money received
- **Expense**: Money spent
- **Transfer**: Money moved between accounts

### Transaction Registration

1. Click menu **"入出金管理"/"Transaction Management"**
2. Click **"新規登録"/"Add New Transaction"** button
3. Select transaction type
4. Enter transaction information:
   - **Date**: Required
   - **Account**: Required
   - **Amount**: Required
   - **Category**: Required for expenses
   - **Store**: Optional
   - **Product**: Optional
   - **Memo**: Optional
5. Click **"登録"/"Register"** button

### Transaction List

- Displays registered transactions
- Can filter by date range, type, category, etc.
- Sort by date, amount, etc.

---

## Aggregation Features

**Available to**: Administrator and Regular User

Analyze income and expenses by period.

### Aggregation Types

1. **Daily**: Daily summary
2. **Weekly**: Weekly summary (Monday-Sunday)
3. **Monthly**: Monthly summary
4. **Yearly**: Yearly summary
5. **Custom Period**: Specify date range

### How to Aggregate

1. Click menu **"集計"/"Aggregation"**
2. Select aggregation type
3. Specify date range
4. Click **"集計"/"Aggregate"** button

### Aggregation Display

- Total income
- Total expenses
- Balance (Income - Expenses)
- Breakdown by category
- Graph display (if available)

---

## Settings

**Available to**: Administrator and Regular User

### Language Settings

- Switch between Japanese and English
- Available in header or settings screen

### Display Settings

- Color indicator mode (Low Vision Support)
- Theme settings (if available)

### Password Change

1. Navigate to settings screen
2. Select "Change Password"
3. Enter current password
4. Enter new password (minimum 16 characters)
5. Confirm new password
6. Click **"更新"/"Update"** button

---

## Troubleshooting

### Cannot Login

**Problem**: "Invalid username or password" error

**Solution**:
- Verify username and password are correct
- Check for typos
- Contact administrator if you forgot your password

### Data Not Saving

**Problem**: Data disappears after registration

**Solution**:
- Check for validation errors
- Verify database file permissions
- Check application logs

### Application Crashes

**Problem**: Application freezes or crashes

**Solution**:
1. Restart the application
2. Check system logs:
   ```bash
   journalctl -xe
   ```
3. Verify system requirements
4. Report issue on GitHub

### Database Errors

**Problem**: "Database error" message

**Solution**:
- Verify SQLite is installed correctly
- Check database file permissions
- Backup and restore database if corrupted

---

## Support

### GitHub Repository

- **Repository**: https://github.com/BonoJovi/KakeiBonByRust
- **Issues**: https://github.com/BonoJovi/KakeiBonByRust/issues
- **Documentation**: https://github.com/BonoJovi/KakeiBonByRust/docs

### Reporting Issues

1. Check existing issues first
2. Create a new issue with:
   - Clear description of the problem
   - Steps to reproduce
   - Expected vs actual behavior
   - System information
   - Error messages/logs

### Contributing

See [CONTRIBUTING.md](../../../CONTRIBUTING.md) for contribution guidelines.

---

**Document Version**: 1.0.1  
**Last Updated**: 2025-12-05 05:31 JST
