-- Add AMOUNT_INCLUDING_TAX column to TRANSACTIONS_DETAIL table
-- Migration Date: 2025-11-17
-- Purpose: Store the tax-included amount separately to preserve exact input values
--          and avoid calculation errors when re-displaying data

-- Add AMOUNT_INCLUDING_TAX column (allows NULL for backward compatibility)
ALTER TABLE TRANSACTIONS_DETAIL 
ADD COLUMN AMOUNT_INCLUDING_TAX INTEGER;

-- For existing records, calculate AMOUNT_INCLUDING_TAX from AMOUNT + TAX_AMOUNT
UPDATE TRANSACTIONS_DETAIL 
SET AMOUNT_INCLUDING_TAX = AMOUNT + TAX_AMOUNT 
WHERE AMOUNT_INCLUDING_TAX IS NULL;

-- Note: New records should always set AMOUNT_INCLUDING_TAX explicitly
-- AMOUNT: tax-excluded amount (always stored)
-- TAX_AMOUNT: calculated tax amount
-- AMOUNT_INCLUDING_TAX: tax-included amount (preserves exact input)
