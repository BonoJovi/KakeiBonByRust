-- Add UPDATE_DT column to MEMOS table
-- Date: 2024-11-09
-- Purpose: Track when memo was last updated

-- Add UPDATE_DT column to MEMOS table
ALTER TABLE MEMOS ADD COLUMN UPDATE_DT DATETIME;

