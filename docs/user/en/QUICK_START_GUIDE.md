# Quick Start Guide

**Version**: 1.0.1  
**Last Updated**: 2025-12-03 15:28 JST

---

## Before You Start

Make sure you have already installed KakeiBon. If not, see [Installation Guide](INSTALLATION_GUIDE.md).

---

## 5-Minute Quick Start

### Step 1: Launch KakeiBon

```bash
# Run from build directory
cd /path/to/KakeiBonByRust
./target/release/KakeiBon

# Or if you've copied to a bin directory
./KakeiBon
```

**Note**: v1.0.1 requires building from source. Pre-built packages are planned for future releases.

### Step 2: Create Administrator Account

On first launch, you'll see the **Admin Setup Screen**:

1. Enter **Username** (3-20 characters, alphanumeric + underscore)
   - Example: `admin`, `myname`, `user_admin`
2. Enter **Password** (minimum 16 characters)
   - Tip: Use a strong password with mix of letters, numbers, symbols
   - Example: `MySecurePass2024!@`
3. Click **"Create Administrator"** button

✅ **You're now logged in as administrator!**

### Step 3: Set Up Categories

Categories help organize your income and expenses.

**Built-in Categories:**
KakeiBon comes with default categories, but you can customize them.

1. Click **"Category Management"** from the menu
2. View the 3-level category hierarchy:
   - Level 1: Main categories (e.g., "Food", "Transportation")
   - Level 2: Subcategories (e.g., "Groceries", "Dining Out")
   - Level 3: Detailed categories (optional)

**To Add a New Category:**
1. Select parent category (or none for Level 1)
2. Enter category name in **Japanese** and **English**
3. Click **"Add Category"**

### Step 4: Add Accounts

Accounts represent where your money is (bank, cash, credit card, etc.).

1. Click **"Account Management"** from the menu
2. Click **"Add Account"** button
3. Enter:
   - **Account Name**: E.g., "Main Bank", "Cash Wallet", "Credit Card"
   - **Account Type**: Cash, Bank, Credit Card, etc.
   - **Initial Balance**: Starting amount (optional)
4. Click **"Save"**

💡 **Tip**: Add at least one account before recording transactions.

### Step 5: Record Your First Transaction

1. Click **"Transaction Management"** from the menu
2. Click **"Add Transaction"** button
3. Fill in:
   - **Date**: Transaction date
   - **Type**: Income or Expense
   - **Account**: Which account to use
   - **Category**: What category it belongs to
   - **Amount**: Transaction amount
   - **Description**: Optional note
4. Click **"Save"**

✅ **Congratulations! You've recorded your first transaction!**

---

## What's Next?

### Create Regular Users (Admin Only)

If multiple people will use KakeiBon:

1. Click **"User Management"** from the menu
2. Click **"Add User"** button
3. Enter username and password
4. Assign **"User"** role (not Admin)
5. Click **"Save"**

💡 Regular users can record transactions but cannot manage other users.

### View Aggregations

Analyze your spending patterns:

1. Click **"Aggregation"** from the menu
2. Choose aggregation type:
   - **Monthly**: View income/expenses by month
   - **Daily**: View by specific date
   - **Weekly**: View by week
   - **Yearly**: View by year
   - **Period**: View custom date range
3. Select parameters (e.g., year and month)
4. Click **"Aggregate"**
5. View results grouped by category, account, or shop

### Customize Categories

Build a category structure that fits your lifestyle:

1. **Add detailed subcategories**
   - Example: Food → Groceries → Vegetables
   - Example: Transportation → Public Transport → Train

2. **Disable unused categories**
   - Mark categories as "disabled" instead of deleting
   - Preserves historical data

### Set Up Shops (Optional)

Track which stores you frequent:

1. Click **"Shop Management"** (if available)
2. Add shops you regularly visit
3. When recording transactions, assign a shop
4. Use aggregation to see spending per shop

---

## Tips for Effective Use

### 🎯 Daily Habits
- **Record transactions immediately** - Don't wait until end of day
- **Use descriptive notes** - Your future self will thank you
- **Check balance regularly** - Keep track of account accuracy

### 📊 Monthly Review
- **Run monthly aggregation** - See where money went
- **Compare with previous months** - Spot trends
- **Adjust categories** - Add/modify as needed

### 🔐 Security Best Practices
- **Use strong passwords** (16+ characters)
- **Don't share admin account** - Create separate users
- **Backup database regularly** (see below)

### 💾 Backup Your Data

Your database is stored at:
- **Linux**: `~/.kakeibon/KakeiBonDB.sqlite3`
- **Windows**: `%USERPROFILE%\.kakeibon\KakeiBonDB.sqlite3`
- **macOS**: `~/.kakeibon/KakeiBonDB.sqlite3`

**Backup regularly:**
```bash
# Linux/macOS
cp ~/.kakeibon/KakeiBonDB.sqlite3 ~/backups/kakeibon_$(date +%Y%m%d).sqlite3

# Windows (PowerShell)
Copy-Item "$env:USERPROFILE\.kakeibon\KakeiBonDB.sqlite3" "$env:USERPROFILE\backups\kakeibon_$(Get-Date -Format 'yyyyMMdd').sqlite3"
```

---

## Common Questions

### Q: Can I change the language?
**A:** Yes! Click the language menu (🌐) and select Japanese (日本語) or English.

### Q: I forgot my password. What do I do?
**A:** Currently, there's no password reset feature. Keep your password safe!

### Q: Can multiple people use KakeiBon at the same time?
**A:** KakeiBon is designed for local use. Multiple users can have accounts, but they should use the app one at a time on the same computer.

### Q: Can I import data from other apps?
**A:** Import feature is planned for future releases. For now, manual entry is required.

### Q: My transaction list is getting long. How do I filter it?
**A:** Use the filter options in Transaction Management to view by date range, category, or account.

---

## Getting Help

### Documentation
- [Installation Guide](INSTALLATION_GUIDE.md) - Setup instructions
- [Troubleshooting Guide](TROUBLESHOOTING.md) - Common issues
- [Aggregation User Guide](AGGREGATION_USER_GUIDE.md) - Detailed aggregation features

### Support
- **GitHub Issues**: [Report bugs or request features](https://github.com/BonoJovi/KakeiBonByRust/issues)
- **Discussions**: [Ask questions or share ideas](https://github.com/BonoJovi/KakeiBonByRust/discussions)

---

## You're All Set! 🎉

You now know the basics of KakeiBon. Start recording your transactions and watch your financial awareness grow!

Happy budgeting! 💰📊

---

**Next**: Explore [Aggregation User Guide](AGGREGATION_USER_GUIDE.md) for advanced analysis features.
