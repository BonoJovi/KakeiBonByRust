//! Latent-audit regression tests (Opus 5.5, 2026-09-26) — DB startup migrations.
//!
//! Lives under `db` so it can build a `Database` around an in-memory pool
//! (the `pool` field is private). `#[ignore]`d until the fix lands; run with
//! `cargo test --lib latent_ -- --ignored`.

use super::*;
use sqlx::Row;

async fn memory_db() -> Database {
    let pool = connect_db(crate::test_helpers::database::TEST_DB_URL)
        .await
        .expect("Failed to connect to in-memory database");
    Database { pool }
}

/// Mirror of the startup sequence in `lib.rs` (initialize + every migrate_*).
async fn run_startup(db: &Database) {
    db.initialize().await.expect("initialize");
    db.migrate_transactions().await.expect("migrate_transactions");
    db.migrate_recurring().await.expect("migrate_recurring");
    db.migrate_period_customization().await.expect("migrate_period_customization");
    db.migrate_period_holiday_shift().await.expect("migrate_period_holiday_shift");
    db.migrate_encryption_salt().await.expect("migrate_encryption_salt");
    db.migrate_shops_unique().await.expect("migrate_shops_unique");
    db.migrate_shops_user_id_cascade().await.expect("migrate_shops_user_id_cascade");
}

/// H5-migration: AMOUNT_INCLUDING_TAX IS NULL の旧明細 (AMOUNT は税抜) が補完されない。
/// Expected: 起動時マイグレーションで AMOUNT_INCLUDING_TAX = AMOUNT + TAX_AMOUNT を backfill。
/// TAX_AMOUNT=0 かつ税率>0 の行はヘッダーの TAX_ROUNDING_TYPE で税額を計算して補完する。
#[tokio::test]
async fn latent_h5_migration_backfills_null_amount_including_tax() {
    let db = memory_db().await;
    run_startup(&db).await;
    let pool = db.pool();

    for stmt in [
        "INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) VALUES (2, 'latent_user', 'hash', 1, datetime('now'))",
        "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, DISPLAY_ORDER, CATEGORY1_NAME, ENTRY_DT) VALUES (2, 'EXPENSE', 1, '支出', datetime('now'))",
        "INSERT INTO ACCOUNTS (USER_ID, ACCOUNT_CODE, ACCOUNT_NAME, TEMPLATE_CODE) VALUES (2, 'CASH', '現金', 'CASH')",
    ] {
        sqlx::query(stmt).execute(pool).await.expect(stmt);
    }
    // Tax-excluded header, HALF_UP rounding.
    let txn_id = sqlx::query(
        "INSERT INTO TRANSACTIONS_HEADER (USER_ID, CATEGORY1_CODE, FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE, \
         TRANSACTION_DATE, TOTAL_AMOUNT, TAX_ROUNDING_TYPE, TAX_INCLUDED_TYPE) \
         VALUES (2, 'EXPENSE', 'CASH', 'CASH', '2020-01-01 10:00:00', 1660, ?, ?)",
    )
    .bind(crate::consts::TAX_ROUND_HALF_UP)
    .bind(crate::consts::TAX_EXCLUDED)
    .execute(pool)
    .await
    .expect("insert header")
    .last_insert_rowid();

    // (AMOUNT, TAX_AMOUNT, TAX_RATE, expected AMOUNT_INCLUDING_TAX)
    let rows: [(i64, i64, i64, i64); 3] = [
        (1000, 100, 10, 1100), // TAX_AMOUNT present → AMOUNT + TAX_AMOUNT
        (333, 0, 8, 360),      // TAX_AMOUNT 0, rate 8 → 333*0.08=26.64 → HALF_UP 27
        (200, 0, 0, 200),      // rate 0 → no tax
    ];
    let mut ids = Vec::new();
    for (amount, tax, rate, _) in rows {
        let id = sqlx::query(
            "INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, ITEM_NAME, \
             AMOUNT, TAX_AMOUNT, TAX_RATE, AMOUNT_INCLUDING_TAX) VALUES (?, 2, 'EXPENSE', 'legacy', ?, ?, ?, NULL)",
        )
        .bind(txn_id)
        .bind(amount)
        .bind(tax)
        .bind(rate)
        .execute(pool)
        .await
        .expect("insert legacy detail")
        .last_insert_rowid();
        ids.push(id);
    }

    // Next app start.
    run_startup(&db).await;

    for (id, (amount, tax, rate, expected)) in ids.iter().zip(rows) {
        let row = sqlx::query("SELECT AMOUNT, AMOUNT_INCLUDING_TAX FROM TRANSACTIONS_DETAIL WHERE DETAIL_ID = ?")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
        let stored_amount: i64 = row.get(0);
        let incl: Option<i64> = row.get(1);
        assert_eq!(stored_amount, amount, "AMOUNT (tax-excluded) must not be rewritten");
        assert_eq!(
            incl,
            Some(expected),
            "AMOUNT_INCLUDING_TAX must be backfilled for AMOUNT={} TAX={} RATE={}",
            amount, tax, rate
        );
    }
}
