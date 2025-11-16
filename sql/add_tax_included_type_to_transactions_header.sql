-- Add TAX_INCLUDED_TYPE column to TRANSACTIONS_HEADER table
-- Created: 2025-11-16
-- Purpose: Add tax inclusion type field (0=Tax Included 内税, 1=Tax Excluded 外税)

-- Add TAX_INCLUDED_TYPE column with default value 1 (Tax Excluded)
-- Default to tax excluded as it's the more common case for detailed tracking
ALTER TABLE TRANSACTIONS_HEADER ADD COLUMN TAX_INCLUDED_TYPE INTEGER DEFAULT 1 NOT NULL;

-- Add comment for clarity (SQLite doesn't support column comments in ALTER TABLE)
-- TAX_INCLUDED_TYPE values:
-- 0 = Tax Included (内税) - tax is already included in the item prices
-- 1 = Tax Excluded (外税) - tax is calculated separately and added to the total

-- Verify the column was added
PRAGMA table_info(TRANSACTIONS_HEADER);
