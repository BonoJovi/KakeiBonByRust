-- Create SHOPS table
-- This table stores shop/store master data for each user
-- Created: 2025-11-10

CREATE TABLE IF NOT EXISTS SHOPS (
    SHOP_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    USER_ID INTEGER NOT NULL,
    SHOP_NAME TEXT NOT NULL,
    MEMO TEXT,
    DISPLAY_ORDER INTEGER NOT NULL DEFAULT 0,
    IS_DISABLED INTEGER DEFAULT 0,
    ENTRY_DT DATETIME NOT NULL DEFAULT (datetime('now')),
    UPDATE_DT DATETIME,
    -- Fable-5 review #11 — match the sibling ACCOUNTS / PRODUCTS /
    -- MANUFACTURERS / TRANSACTIONS_HEADER / MEMOS FK: cascade the row on
    -- user deletion. Without ON DELETE CASCADE, deleting a user with
    -- SHOPS rows fails with `FOREIGN KEY constraint failed` and rolls
    -- the whole DELETE back.
    FOREIGN KEY (USER_ID) REFERENCES USERS(USER_ID) ON DELETE CASCADE
);

-- Create indexes for SHOPS
CREATE INDEX IF NOT EXISTS idx_shops_user
    ON SHOPS(USER_ID, DISPLAY_ORDER);

CREATE INDEX IF NOT EXISTS idx_shops_name
    ON SHOPS(USER_ID, SHOP_NAME);
