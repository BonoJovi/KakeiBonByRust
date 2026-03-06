# Translation Guide

**Version**: 1.0
**Last Updated**: 2025-12-04
**Target Audience**: Translators (No coding experience required)

---

## [List] Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
- [Translation System Architecture](#translation-system-architecture)
- [How to Add a New Language](#how-to-add-a-new-language)
- [Translation Workflow](#translation-workflow)
- [Translation Guidelines](#translation-guidelines)
- [Testing Your Translation](#testing-your-translation)
- [Submitting Your Translation](#submitting-your-translation)
- [FAQ](#faq)

---

## Overview

KakeiBon uses a database-driven internationalization (i18n) system. All user-facing text is stored in the `I18N_RESOURCES` table in the SQLite database.

### Current Status

- **Supported Languages**: Japanese (ja), English (en)
- **Total Translation Keys**: 650+ entries
- **Translation Coverage**: 100% for ja and en

### Why Contribute?

Your translation helps make KakeiBon accessible to users worldwide. You don't need programming knowledge—just fluency in your target language!

---

## Getting Started

### Prerequisites

**No coding experience required!** You only need:

1. Fluency in your target language
2. Basic understanding of household finance terminology
3. A text editor (Notepad, TextEdit, or any editor)
4. Optional: Ability to build KakeiBon from source (for testing)

### Quick Start

1. **Check if your language is already supported**
   - Currently: Japanese (ja), English (en)
   - [See open translation requests](https://github.com/BonoJovi/KakeiBonByRust/issues?q=label%3Atranslation)

2. **Express your interest**
   - [Submit a translation request](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=translation.yml)
   - Indicate which language you want to translate

3. **Get the translation template**
   - We'll provide you with a CSV or spreadsheet template
   - Contains all English keys and values

4. **Translate and submit**
   - Fill in your translations
   - Submit via GitHub Issue or email

---

## Translation System Architecture

### How It Works

```
┌─────────────────────────────────────────────────────────────┐
│                    SQLite Database                          │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐ │
│  │              I18N_RESOURCES Table                     │ │
│  ├──────────────┬──────────┬─────────────┬──────────────┤ │
│  │ RESOURCE_KEY │ LANG_CODE│   CATEGORY  │RESOURCE_VALUE│ │
│  ├──────────────┼──────────┼─────────────┼──────────────┤ │
│  │ app.title    │   en     │   general   │  KakeiBon    │ │
│  │ app.title    │   ja     │   general   │  家計簿      │ │
│  │ app.title    │   zh     │   general   │  记账本      │ │
│  └──────────────┴──────────┴─────────────┴──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↓
              ┌─────────────────────────────┐
              │   Rust Backend (Tauri)      │
              │   i18n Service              │
              └─────────────────────────────┘
                            ↓
              ┌─────────────────────────────┐
              │   JavaScript Frontend       │
              │   i18n.js                   │
              └─────────────────────────────┘
                            ↓
              ┌─────────────────────────────┐
              │   User Interface (HTML)     │
              │   <span data-i18n="key">    │
              └─────────────────────────────┘
```

### Database Schema

```sql
CREATE TABLE I18N_RESOURCES (
    RESOURCE_KEY   TEXT NOT NULL,   -- Unique key (e.g., "app.title")
    LANG_CODE      TEXT NOT NULL,   -- Language code (ISO 639-1: en, ja, zh)
    CATEGORY       TEXT,             -- Category (general, menu, error, etc.)
    RESOURCE_VALUE TEXT NOT NULL,   -- Translated text
    PRIMARY KEY (RESOURCE_KEY, LANG_CODE)
);
```

### Translation Categories

| Category | Description | Example Keys |
|----------|-------------|--------------|
| `general` | General UI elements | `app.title`, `app.description` |
| `menu` | Menu items | `menu.file`, `menu.settings` |
| `button` | Button labels | `btn.save`, `btn.cancel` |
| `label` | Form labels | `label.username`, `label.password` |
| `msg` | Messages | `msg.save_success`, `msg.error` |
| `error` | Error messages | `error.invalid_input` |
| `validation` | Validation messages | `validation.required` |
| `placeholder` | Input placeholders | `placeholder.search` |

---

## How to Add a New Language

### Step 1: Language Code Selection

Use **ISO 639-1** language codes:

| Language | Code | Example |
|----------|------|---------|
| Chinese (Simplified) | `zh` | 记账本 |
| Korean | `ko` | 가계부 |
| French | `fr` | Livre de comptes |
| German | `de` | Haushaltsbuch |
| Spanish | `es` | Libro de cuentas |
| Portuguese | `pt` | Livro de contas |
| Italian | `it` | Libro dei conti |
| Russian | `ru` | Домашняя бухгалтерия |

### Step 2: Request Translation Template

1. Go to [Issues](https://github.com/BonoJovi/KakeiBonByRust/issues)
2. Click "New Issue" → "Translation Request"
3. Fill in:
   - **Target Language**: e.g., "Chinese (Simplified)"
   - **Language Code**: e.g., "zh"
   - **Your availability**: Can you translate? Review?

### Step 3: Receive Template

The maintainer will provide you with:

- **CSV file** or **Google Sheets** with:
  - Column A: `RESOURCE_KEY`
  - Column B: `CATEGORY`
  - Column C: `RESOURCE_VALUE (English)` (reference)
  - Column D: `RESOURCE_VALUE (Your Language)` (for you to fill)

Example CSV structure:

```csv
RESOURCE_KEY,CATEGORY,ENGLISH,YOUR_LANGUAGE
app.title,general,KakeiBon,
app.description,general,Household Budget Manager,
menu.file,menu,File,
menu.settings,menu,Settings,
btn.save,button,Save,
btn.cancel,button,Cancel,
```

---

## Translation Workflow

### 1. Review English Reference

Read through all English translations to understand:
- App functionality
- Context of each key
- Tone and formality level

### 2. Translate Keys

For each row:
1. **Understand the context**
   - What does this key represent?
   - Where is it used in the UI?
   - Is it a button, label, or message?

2. **Translate accurately**
   - Maintain the original meaning
   - Use natural phrasing in your language
   - Consider cultural appropriateness

3. **Maintain consistency**
   - Use the same terminology throughout
   - Keep the same tone and formality level

### 3. Handle Special Cases

#### Parameters in Strings

Some strings contain placeholders like `{0}`, `{1}`:

```
English: "Language changed to {0}."
Japanese: "言語を{0}に変更しました。"
Chinese: "语言已更改为{0}。"
```

**Important**: Keep the `{0}`, `{1}` placeholders in the same position where they make sense in your language.

#### HTML Tags

Some strings may contain HTML tags:

```
English: "Click <strong>Save</strong> to continue."
```

**Important**: Keep the HTML tags, translate only the text content.

#### Units and Numbers

Consider regional differences:
- Date formats (MM/DD/YYYY vs DD/MM/YYYY)
- Currency symbols (¥ vs $ vs €)
- Number separators (1,000.00 vs 1.000,00)

---

## Translation Guidelines

### 1. Accuracy

✅ **Good**: Accurate translation maintaining original meaning
```
English: "Delete this account?"
Chinese: "删除此账户？"
```

❌ **Bad**: Mistranslation or omission
```
English: "Delete this account?"
Chinese: "删除？" (Missing "this account")
```

### 2. Consistency

Maintain consistent terminology throughout:

| Term | Japanese | Chinese | Korean |
|------|----------|---------|--------|
| Account | 口座 | 账户 | 계좌 |
| Transaction | 入出金 | 交易 | 거래 |
| Category | 費目 | 类别 | 카테고리 |
| Budget | 予算 | 预算 | 예산 |

### 3. Natural Language

Use natural phrasing for your language:

✅ **Good**: Natural native expression
```
English: "Are you sure you want to delete?"
Japanese: "本当に削除しますか？"
```

❌ **Bad**: Word-for-word translation (sounds unnatural)
```
Japanese: "あなたは削除したいですか確実ですか？"
```

### 4. Formality Level

KakeiBon uses **polite but not overly formal** language:

- Not too casual (e.g., "Yo, save this?")
- Not too formal (e.g., "Would you be so kind as to save?")
- Professional and friendly

### 5. Cultural Appropriateness

Consider cultural context:
- Financial terms may vary by region
- Some concepts may not translate directly
- Use culturally appropriate examples

---

## Testing Your Translation

### Option 1: Visual Review

Review your translations in context:
1. Read through the CSV/spreadsheet
2. Imagine how each text appears in the UI
3. Check for consistency and naturalness

### Option 2: Test in the App (Recommended)

If you can build KakeiBon from source:

1. **Install dependencies** (see [Installation Guide](../../../user/en/installation.md))

2. **Add your translations to the database**
   - The maintainer will help with this step
   - Or use the provided SQL script

3. **Build and run**
   ```bash
   cargo tauri build
   ./target/release/KakeiBon
   ```

4. **Switch to your language**
   - Settings → Language → Select your language

5. **Navigate through all screens**
   - Check every menu, button, and message
   - Look for:
     - Truncated text (too long)
     - Misaligned UI elements
     - Incorrect translations

6. **Report issues**
   - Take screenshots
   - Note which keys need adjustment

---

## Creating SQL INSERT Statements

KakeiBon stores all translations in a SQLite database. Once your translation is complete, you'll need to create a SQL file with INSERT statements.

### Why SQL?

- **Database-driven**: All i18n resources are stored in the `I18N_RESOURCES` table
- **Auto-loaded**: SQL files in `sql/init/i18n/` are executed during database initialization
- **Version control**: SQL files track translation changes over time

### SQL File Structure

Each translation SQL file follows this template:

```sql
-- Translation resources for [category]
-- Language: [language_name] ([language_code])
-- Category: [category]

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(RESOURCE_ID, 'resource.key', 'lang', 'Translated Text', 'category', 'Description', datetime('now')),
(RESOURCE_ID, 'resource.key', 'lang', 'Translated Text', 'category', 'Description', datetime('now'));
```

### Step-by-Step Guide

#### Step 1: Get the Latest RESOURCE_ID

Before creating your SQL file, you need to know the next available RESOURCE_ID to avoid conflicts.

**Current highest ID**: ~1290 (as of 2025-12-04)

**Contact the maintainer** to get the current highest ID and reserve a range for your language.

Example:
```
Your language: Chinese (zh)
Reserved ID range: 1291-1940 (650 IDs for 650 keys)
```

#### Step 2: Choose a File Naming Convention

SQL files should be named descriptively:

```
sql/init/i18n/init_[category]_i18n_[lang_code].sql
```

Examples:
- `init_app_i18n_zh.sql` - Chinese app translations
- `init_menu_i18n_ko.sql` - Korean menu translations
- `init_common_i18n_fr.sql` - French common translations

**For a complete language**: You may create one file per category, or one large file:

```
sql/init/i18n/init_all_i18n_zh.sql
```

#### Step 3: Convert CSV to SQL

Using your completed translation CSV, create INSERT statements.

**Example CSV**:
```csv
RESOURCE_KEY,CATEGORY,ENGLISH,CHINESE
app.title,general,KakeiBon,记账本
app.description,general,Household Budget Manager,家庭预算管理器
menu.file,menu,File,文件
btn.save,button,Save,保存
```

**Converted to SQL**:
```sql
-- Translation resources for Chinese (Simplified)
-- Language: Chinese (zh)
-- Auto-generated from translation CSV

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
(1291, 'app.title', 'zh', '记账本', 'general', 'Application title', datetime('now')),
(1292, 'app.description', 'zh', '家庭预算管理器', 'general', 'Application description', datetime('now')),
(1293, 'menu.file', 'zh', '文件', 'menu', 'File menu', datetime('now')),
(1294, 'btn.save', 'zh', '保存', 'button', 'Save button', datetime('now'));
```

#### Step 4: Maintain ID Consistency

**Important rules**:

1. **Sequential IDs**: Use sequential RESOURCE_IDs (1291, 1292, 1293...)
2. **No gaps**: Don't skip IDs unless intentional
3. **No duplicates**: Each RESOURCE_ID must be unique across ALL languages
4. **Same key, different ID**: The same `RESOURCE_KEY` for different languages gets different IDs

**Example**:
```sql
-- English
(401, 'app.title', 'en', 'KakeiBon', 'general', 'App title', datetime('now')),

-- Japanese
(402, 'app.title', 'ja', '家計簿', 'general', 'App title', datetime('now')),

-- Chinese (your new translation)
(1291, 'app.title', 'zh', '记账本', 'general', 'App title', datetime('now')),
```

#### Step 5: Handle Special Characters

SQL requires escaping single quotes:

```sql
-- ❌ Wrong (will cause SQL error)
'It's a test'

-- ✅ Correct (escape single quote with another single quote)
'It''s a test'
```

**Example**:
```sql
(1295, 'msg.cant_delete', 'zh', '无法删除，因为它''s正在使用中', 'message', 'Cannot delete message', datetime('now')),
```

#### Step 6: Organize by Category (Optional)

For better maintainability, group INSERTs by category:

```sql
-- Translation resources for Chinese (Simplified)
-- Language: Chinese (zh)

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
-- General
(1291, 'app.title', 'zh', '记账本', 'general', 'App title', datetime('now')),
(1292, 'app.description', 'zh', '家庭预算管理器', 'general', 'App description', datetime('now')),

-- Menu
(1293, 'menu.file', 'zh', '文件', 'menu', 'File menu', datetime('now')),
(1294, 'menu.settings', 'zh', '设置', 'menu', 'Settings menu', datetime('now')),

-- Buttons
(1295, 'btn.save', 'zh', '保存', 'button', 'Save button', datetime('now')),
(1296, 'btn.cancel', 'zh', '取消', 'button', 'Cancel button', datetime('now'));
```

### Tools and Automation

#### CSV to SQL Converter (Recommended)

The maintainer can provide a conversion script:

```bash
# Convert CSV to SQL (script provided by maintainer)
python3 scripts/csv_to_sql.py translations_zh.csv --lang zh --start-id 1291 > init_all_i18n_zh.sql
```

**Or request the maintainer to convert your CSV for you!**

#### Manual Conversion Template

If converting manually, use this template:

```sql
-- Translation resources for [YOUR_LANGUAGE_NAME]
-- Language: [YOUR_LANGUAGE_NAME] ([LANG_CODE])
-- Generated from: translations_[LANG_CODE].csv
-- Date: [DATE]

INSERT OR IGNORE INTO I18N_RESOURCES (RESOURCE_ID, RESOURCE_KEY, LANG_CODE, RESOURCE_VALUE, CATEGORY, DESCRIPTION, ENTRY_DT)
VALUES
([START_ID], '[RESOURCE_KEY]', '[LANG_CODE]', '[YOUR_TRANSLATION]', '[CATEGORY]', '[DESCRIPTION]', datetime('now')),
-- Repeat for each translation...
([END_ID], '[LAST_KEY]', '[LANG_CODE]', '[LAST_TRANSLATION]', '[CATEGORY]', '[DESCRIPTION]', datetime('now'));
```

### Validation Checklist

Before submitting your SQL file, verify:

- [ ] All RESOURCE_IDs are unique
- [ ] No ID gaps (unless intentional)
- [ ] IDs are within your reserved range
- [ ] All single quotes are escaped (`''`)
- [ ] LANG_CODE is consistent throughout
- [ ] File uses UTF-8 encoding
- [ ] SQL syntax is valid (no trailing commas, etc.)
- [ ] `INSERT OR IGNORE` is used (prevents duplicate key errors)

### Testing Your SQL File

If you can build KakeiBon from source:

1. **Place your SQL file** in `sql/init/i18n/`

2. **Delete existing database** (backup first!)
   ```bash
   # Backup
   cp ~/.local/share/KakeiBon/KakeiBonDB.sqlite3 ~/backup.db

   # Delete
   rm ~/.local/share/KakeiBon/KakeiBonDB.sqlite3
   ```

3. **Rebuild and run**
   ```bash
   cargo tauri build
   ./target/release/KakeiBon
   ```

4. **Verify your translations loaded**
   - Switch to your language in Settings
   - Navigate through the app
   - Check that all text displays correctly

### Common Issues

#### Issue 1: Duplicate ID Error

```
Error: UNIQUE constraint failed: I18N_RESOURCES.RESOURCE_ID
```

**Solution**: Check for duplicate IDs in your SQL file or conflicts with existing IDs.

#### Issue 2: SQL Syntax Error

```
Error: near line 42: syntax error
```

**Solution**: Check for:
- Missing commas between rows
- Trailing comma on last row (remove it)
- Unescaped single quotes

#### Issue 3: Encoding Issues

```
Error: Invalid UTF-8 sequence
```

**Solution**: Ensure your SQL file is saved with UTF-8 encoding.

---

## Submitting Your Translation

### Method 1: GitHub Issue (Recommended)

1. Complete your translation CSV/spreadsheet
2. Go to the original translation request issue
3. Attach your completed file
4. Add any notes or questions

### Method 2: Pull Request (Advanced)

If you're comfortable with Git:

1. Fork the repository
2. Add your translations to `src-tauri/migrations/`
3. Create SQL migration file (maintainer can help)
4. Submit a pull request

### Method 3: Email

Send to: [bonojovi@zundou.org](mailto:bonojovi@zundou.org)
- Attach your translation file
- Include your language code and any notes

---

## FAQ

### Q: Do I need to translate all 650+ keys at once?

**A**: No! You can translate incrementally:
1. Start with critical keys (app title, menu items, buttons)
2. Move to messages and errors
3. Finish with less common strings

### Q: What if I'm not sure about a translation?

**A**:
- Ask questions in the GitHub issue
- Provide multiple translation options
- Note uncertainty so reviewers can help

### Q: Can I improve existing translations?

**A**: Yes! If you find issues with current translations:
1. Submit a translation improvement issue
2. Provide the current translation and your suggested improvement
3. Explain why your version is better

### Q: How do I handle terms that don't exist in my language?

**A**: Options:
1. Use a loan word (e.g., "app" → "アプリ")
2. Describe the concept
3. Ask the community for suggestions

### Q: Will my translation be credited?

**A**: Yes! Contributors are acknowledged in:
- CHANGELOG.md
- Contributors section
- Git commit history

### Q: What if I make a mistake?

**A**: Don't worry! All translations are reviewed:
1. Native speakers review translations
2. Maintainers test in the app
3. Users report issues
4. We iterate and improve

---

## Additional Resources

- **[Contributing Guide](../../../../CONTRIBUTING.md)**
- **[Testing Guide](testing-guide.md)**
- **[Installation Guide](../../../user/en/installation.md)**
- **[Submit Translation Request](https://github.com/BonoJovi/KakeiBonByRust/issues/new?template=translation.yml)**

---

## Need Help?

- **GitHub Issues**: [Translation Discussion](https://github.com/BonoJovi/KakeiBonByRust/issues?q=label%3Atranslation)
- **Email**: [bonojovi@zundou.org](mailto:bonojovi@zundou.org)

---

**Thank you for helping make KakeiBon accessible to users worldwide!**

We deeply appreciate your time and effort in contributing translations.

**- The KakeiBon Team**
