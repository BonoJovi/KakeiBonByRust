#!/bin/bash

# Test script for Manufacturer and Product management

DB_PATH="$HOME/.kakeibon/KakeiBonDB.sqlite3"

echo "=== Testing Manufacturer and Product Management ==="
echo ""

# 1. Create admin user first
echo "1. Creating admin user..."
./db.sh "INSERT OR IGNORE INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) VALUES (1, 'admin', '\$argon2id\$v=19\$m=19456,t=2,p=1\$test\$testHashValue', 0, datetime('now'));"

# 2. Insert manufacturers
echo "2. Inserting manufacturers..."
./db.sh "INSERT INTO MANUFACTURERS (USER_ID, MANUFACTURER_NAME, MEMO, DISPLAY_ORDER) VALUES (1, 'ニッスイ', '日本水産株式会社', 1);"
./db.sh "INSERT INTO MANUFACTURERS (USER_ID, MANUFACTURER_NAME, MEMO, DISPLAY_ORDER) VALUES (1, 'マルハニチロ', 'マルハニチロ株式会社', 2);"
./db.sh "INSERT INTO MANUFACTURERS (USER_ID, MANUFACTURER_NAME, DISPLAY_ORDER) VALUES (1, '極洋', 3);"

# 3. Get manufacturer IDs
echo ""
echo "3. Listing manufacturers..."
./db.sh "SELECT MANUFACTURER_ID, MANUFACTURER_NAME, MEMO FROM MANUFACTURERS WHERE USER_ID = 1;"

# Get first manufacturer ID
NISSUI_ID=$(./db.sh "SELECT MANUFACTURER_ID FROM MANUFACTURERS WHERE USER_ID = 1 AND MANUFACTURER_NAME = 'ニッスイ';" | tail -1)
echo ""
echo "ニッスイのID: $NISSUI_ID"

# 4. Insert products
echo ""
echo "4. Inserting products..."
./db.sh "INSERT INTO PRODUCTS (USER_ID, PRODUCT_NAME, MANUFACTURER_ID, MEMO, DISPLAY_ORDER) VALUES (1, 'サバ缶 水煮', $NISSUI_ID, '190g缶', 1);"
./db.sh "INSERT INTO PRODUCTS (USER_ID, PRODUCT_NAME, MANUFACTURER_ID, DISPLAY_ORDER) VALUES (1, 'サンマ缶 味付', $NISSUI_ID, 2);"
./db.sh "INSERT INTO PRODUCTS (USER_ID, PRODUCT_NAME, DISPLAY_ORDER) VALUES (1, 'ツナ缶 オイル漬け', 3);"

# 5. List products with manufacturer names (JOIN query)
echo ""
echo "5. Listing products with manufacturers..."
./db.sh "
SELECT
    p.PRODUCT_ID,
    p.PRODUCT_NAME,
    COALESCE(m.MANUFACTURER_NAME, '(未指定)') as MANUFACTURER_NAME,
    p.MEMO
FROM PRODUCTS p
LEFT JOIN MANUFACTURERS m ON p.MANUFACTURER_ID = m.MANUFACTURER_ID
WHERE p.USER_ID = 1 AND p.IS_DISABLED = 0
ORDER BY p.DISPLAY_ORDER;
"

# 6. Test update manufacturer
echo ""
echo "6. Testing manufacturer update..."
./db.sh "UPDATE MANUFACTURERS SET MEMO = '日本水産株式会社 (更新)', UPDATE_DT = datetime('now') WHERE MANUFACTURER_ID = $NISSUI_ID;"
./db.sh "SELECT MANUFACTURER_NAME, MEMO, UPDATE_DT FROM MANUFACTURERS WHERE MANUFACTURER_ID = $NISSUI_ID;"

# 7. Test product update
echo ""
echo "7. Testing product update..."
PRODUCT_ID=$(./db.sh "SELECT PRODUCT_ID FROM PRODUCTS WHERE PRODUCT_NAME = 'ツナ缶 オイル漬け';" | tail -1)
./db.sh "UPDATE PRODUCTS SET MANUFACTURER_ID = $NISSUI_ID, UPDATE_DT = datetime('now') WHERE PRODUCT_ID = $PRODUCT_ID;"
echo "ツナ缶にメーカーを設定:"
./db.sh "
SELECT
    p.PRODUCT_NAME,
    m.MANUFACTURER_NAME
FROM PRODUCTS p
LEFT JOIN MANUFACTURERS m ON p.MANUFACTURER_ID = m.MANUFACTURER_ID
WHERE p.PRODUCT_ID = $PRODUCT_ID;
"

# 8. Test duplicate prevention
echo ""
echo "8. Testing duplicate prevention..."
echo "Trying to insert duplicate manufacturer (should fail):"
./db.sh "INSERT INTO MANUFACTURERS (USER_ID, MANUFACTURER_NAME, DISPLAY_ORDER) VALUES (1, 'ニッスイ', 4);" 2>&1 | grep -i "unique\|error" || echo "No error (unexpected)"

echo ""
echo "Trying to insert duplicate product (should fail):"
./db.sh "INSERT INTO PRODUCTS (USER_ID, PRODUCT_NAME, DISPLAY_ORDER) VALUES (1, 'サバ缶 水煮', 5);" 2>&1 | grep -i "unique\|error" || echo "No error (unexpected)"

# 9. Test logical deletion
echo ""
echo "9. Testing logical deletion..."
./db.sh "UPDATE MANUFACTURERS SET IS_DISABLED = 1, UPDATE_DT = datetime('now') WHERE MANUFACTURER_ID = $NISSUI_ID;"
echo "After disabling ニッスイ, active manufacturers:"
./db.sh "SELECT MANUFACTURER_NAME FROM MANUFACTURERS WHERE USER_ID = 1 AND IS_DISABLED = 0;"

echo ""
echo "Products still exist (MANUFACTURER_ID preserved):"
./db.sh "SELECT PRODUCT_NAME, MANUFACTURER_ID FROM PRODUCTS WHERE USER_ID = 1;"

echo ""
echo "=== Test completed ==="
