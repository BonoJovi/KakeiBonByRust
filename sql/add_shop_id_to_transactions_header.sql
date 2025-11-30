-- Add SHOP_ID column to TRANSACTIONS_HEADER table
-- Created: 2025-11-10
-- Purpose: Add shop/store identification field to transaction records

-- Add SHOP_ID column (nullable INTEGER)
ALTER TABLE TRANSACTIONS_HEADER ADD COLUMN SHOP_ID INTEGER;

-- Verify the column was added
PRAGMA table_info(TRANSACTIONS_HEADER);
