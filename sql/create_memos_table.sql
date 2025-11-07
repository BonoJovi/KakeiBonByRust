-- Create MEMOS table
-- This table stores reusable memo text for transactions
-- Created: 2025-01-07

CREATE TABLE IF NOT EXISTS MEMOS (
    MEMO_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    MEMO_TEXT TEXT NOT NULL,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID) ON DELETE CASCADE,
    CHECK (MEMO_TEXT != '')  -- Memo text cannot be empty
);

-- Create index for MEMOS
CREATE INDEX IF NOT EXISTS idx_memos_user 
    ON MEMOS(USER_ID);

-- Create index for frequently used memos (for autocomplete/suggestion)
CREATE INDEX IF NOT EXISTS idx_memos_text 
    ON MEMOS(USER_ID, MEMO_TEXT);
