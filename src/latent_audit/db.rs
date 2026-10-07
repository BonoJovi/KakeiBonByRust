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
    db.cleanup_orphan_user_categories().await.expect("cleanup_orphan_user_categories");
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

/// M3 (existing data): category rows left behind by user deletes before the
/// fix are swept at startup, while a live user's categories are untouched.
#[tokio::test]
async fn latent_m3_startup_removes_orphan_user_categories() {
    let db = memory_db().await;
    run_startup(&db).await;
    let pool = db.pool();

    for stmt in [
        "INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) VALUES (2, 'live_user', 'hash', 1, datetime('now'))",
        // Live user 2 and deleted user 3 (no USERS row) each own a category tree.
        "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, DISPLAY_ORDER, CATEGORY1_NAME, ENTRY_DT) VALUES (2, 'EXPENSE', 1, '支出', datetime('now'))",
        "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, DISPLAY_ORDER, CATEGORY1_NAME, ENTRY_DT) VALUES (3, 'EXPENSE', 1, '支出', datetime('now'))",
        "INSERT INTO CATEGORY1_I18N (USER_ID, CATEGORY1_CODE, LANG_CODE, CATEGORY1_NAME_I18N, ENTRY_DT) VALUES (3, 'EXPENSE', 'ja', '支出', datetime('now'))",
        "INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, DISPLAY_ORDER, CATEGORY2_NAME, ENTRY_DT) VALUES (2, 'EXPENSE', 'C2_E_1', 1, '食費', datetime('now'))",
        "INSERT INTO CATEGORY2 (USER_ID, CATEGORY1_CODE, CATEGORY2_CODE, DISPLAY_ORDER, CATEGORY2_NAME, ENTRY_DT) VALUES (3, 'EXPENSE', 'C2_E_1', 1, '独自費目', datetime('now'))",
    ] {
        sqlx::query(stmt).execute(pool).await.expect(stmt);
    }

    // Next app start.
    run_startup(&db).await;

    let count = |table: &'static str, user: i64| async move {
        let sql = format!("SELECT COUNT(*) FROM {} WHERE USER_ID = ?", table);
        sqlx::query_scalar::<_, i64>(&sql).bind(user).fetch_one(pool).await.unwrap()
    };
    for table in ["CATEGORY1", "CATEGORY1_I18N", "CATEGORY2"] {
        assert_eq!(count(table, 3).await, 0, "{}: orphan rows of deleted USER_ID 3 must be removed", table);
    }
    assert_eq!(count("CATEGORY1", 2).await, 1, "live user's CATEGORY1 must be kept");
    assert_eq!(count("CATEGORY2", 2).await, 1, "live user's CATEGORY2 must be kept");
}

/// M2 (existing data): a header whose category1 was changed after its
/// details were entered is set back to the details' category1 at startup,
/// with its account moved to the side that category uses. A header whose
/// details mix category1 values, one whose details are a transfer (its
/// other account is unknown), and a consistent header are left alone.
#[tokio::test]
async fn latent_m2_startup_repairs_header_category1_mismatch() {
    let db = memory_db().await;
    run_startup(&db).await;
    let pool = db.pool();

    for stmt in [
        "INSERT INTO USERS (USER_ID, NAME, PAW, ROLE, ENTRY_DT) VALUES (2, 'latent_user', 'hash', 1, datetime('now'))",
        "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, DISPLAY_ORDER, CATEGORY1_NAME, ENTRY_DT) VALUES (2, 'EXPENSE', 1, '支出', datetime('now'))",
        "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, DISPLAY_ORDER, CATEGORY1_NAME, ENTRY_DT) VALUES (2, 'INCOME', 2, '収入', datetime('now'))",
        "INSERT INTO CATEGORY1 (USER_ID, CATEGORY1_CODE, DISPLAY_ORDER, CATEGORY1_NAME, ENTRY_DT) VALUES (2, 'TRANSFER', 3, '振替', datetime('now'))",
        "INSERT INTO ACCOUNTS (USER_ID, ACCOUNT_CODE, ACCOUNT_NAME, TEMPLATE_CODE) VALUES (2, 'NONE', '指定なし', 'NONE')",
        "INSERT INTO ACCOUNTS (USER_ID, ACCOUNT_CODE, ACCOUNT_NAME, TEMPLATE_CODE) VALUES (2, 'CASH', '現金', 'CASH')",
    ] {
        sqlx::query(stmt).execute(pool).await.expect(stmt);
    }

    async fn header(pool: &SqlitePool, category1: &str, from: &str, to: &str) -> i64 {
        sqlx::query(
            "INSERT INTO TRANSACTIONS_HEADER (USER_ID, CATEGORY1_CODE, FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE, \
             TRANSACTION_DATE, TOTAL_AMOUNT, TAX_ROUNDING_TYPE, TAX_INCLUDED_TYPE) \
             VALUES (2, ?, ?, ?, '2026-01-01 10:00:00', 100, 0, 1)",
        )
        .bind(category1)
        .bind(from)
        .bind(to)
        .execute(pool)
        .await
        .expect("insert header")
        .last_insert_rowid()
    }
    async fn detail(pool: &SqlitePool, transaction_id: i64, category1: &str) {
        sqlx::query(
            "INSERT INTO TRANSACTIONS_DETAIL (TRANSACTION_ID, USER_ID, CATEGORY1_CODE, ITEM_NAME, AMOUNT) \
             VALUES (?, 2, ?, 'item', 100)",
        )
        .bind(transaction_id)
        .bind(category1)
        .execute(pool)
        .await
        .expect("insert detail");
    }

    // Entered as an expense from CASH, then switched to income (TO = CASH).
    let mismatched = header(pool, "INCOME", "NONE", "CASH").await;
    detail(pool, mismatched, "EXPENSE").await;
    detail(pool, mismatched, "EXPENSE").await;
    // Details disagree with each other: nothing to go back to.
    let mixed = header(pool, "INCOME", "NONE", "CASH").await;
    detail(pool, mixed, "EXPENSE").await;
    detail(pool, mixed, "INCOME").await;
    // Consistent.
    let consistent = header(pool, "EXPENSE", "CASH", "NONE").await;
    detail(pool, consistent, "EXPENSE").await;
    // Details are a transfer, but the expense header holds only FROM: the
    // transfer's destination is unknown, so it is not repaired.
    let to_transfer = header(pool, "EXPENSE", "CASH", "NONE").await;
    detail(pool, to_transfer, "TRANSFER").await;

    // Next app start.
    run_startup(&db).await;

    let header_row = |id: i64| async move {
        let row = sqlx::query(
            "SELECT CATEGORY1_CODE, FROM_ACCOUNT_CODE, TO_ACCOUNT_CODE FROM TRANSACTIONS_HEADER WHERE TRANSACTION_ID = ?",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap();
        (row.get::<String, _>(0), row.get::<String, _>(1), row.get::<String, _>(2))
    };
    assert_eq!(
        header_row(mismatched).await,
        ("EXPENSE".to_string(), "CASH".to_string(), "NONE".to_string()),
        "the header must go back to the details' category1, spending from CASH"
    );
    assert_eq!(
        header_row(mixed).await,
        ("INCOME".to_string(), "NONE".to_string(), "CASH".to_string()),
        "a header whose details mix category1 values is left alone"
    );
    assert_eq!(
        header_row(consistent).await,
        ("EXPENSE".to_string(), "CASH".to_string(), "NONE".to_string())
    );
    assert_eq!(
        header_row(to_transfer).await,
        ("EXPENSE".to_string(), "CASH".to_string(), "NONE".to_string()),
        "a header that would become a transfer with an unknown destination is left alone"
    );
}

/// M2: the account moves to the side the new category1 uses; a header that
/// would become a transfer is not repaired (its other account is unknown).
#[test]
fn latent_m2_accounts_follow_category1_side() {
    let s = |v: &str| v.to_string();
    assert_eq!(
        accounts_for_category1_change("INCOME", "EXPENSE", s("NONE"), s("CASH")),
        Some((s("CASH"), s("NONE")))
    );
    assert_eq!(
        accounts_for_category1_change("EXPENSE", "INCOME", s("CASH"), s("NONE")),
        Some((s("NONE"), s("CASH")))
    );
    assert_eq!(
        accounts_for_category1_change("TRANSFER", "EXPENSE", s("BANK"), s("CASH")),
        Some((s("BANK"), s("NONE")))
    );
    assert_eq!(
        accounts_for_category1_change("TRANSFER", "INCOME", s("BANK"), s("CASH")),
        Some((s("NONE"), s("CASH")))
    );
    assert_eq!(
        accounts_for_category1_change("EXPENSE", "TRANSFER", s("CASH"), s("NONE")),
        None,
        "a transfer needs both accounts; an expense header had only one"
    );
    assert_eq!(accounts_for_category1_change("INCOME", "TRANSFER", s("NONE"), s("CASH")), None);
}

/// scan2-R3: jpholiday 0.1.4 lacks the 2021 Olympic holiday moves, so the
/// seeded JP holidays for 2021 are wrong (海の日 07-19, 山の日 08-11,
/// スポーツの日 10-11 instead of 07-22, 07-23, 08-08 and the substitute
/// holiday 08-09), and the INSERT OR IGNORE seeding can never correct rows
/// that an earlier build already stored.
/// Expected: a fresh DB has the real 2021 holidays, and re-running the
/// startup seeding on an existing DB that holds the wrong rows repairs them.
#[tokio::test]
async fn latent_scan2_r3_2021_holidays_are_correct_and_repaired() {
    use chrono::{Datelike, Local};

    // A fresh DB seeds 2021 only while it is inside the sliding window
    // (until 2026); the repair of an existing DB is checked in any year.
    let this_year = Local::now().year();
    let seeds_2021 = ((this_year - crate::consts::HOLIDAY_SEED_YEARS_BACK)
        ..=(this_year + crate::consts::HOLIDAY_SEED_YEARS_AHEAD))
        .contains(&2021);

    const WRONG: [&str; 3] = ["2021-07-19", "2021-08-11", "2021-10-11"];
    const RIGHT: [&str; 4] = ["2021-07-22", "2021-07-23", "2021-08-08", "2021-08-09"];

    async fn jp_2021(pool: &SqlitePool) -> Vec<String> {
        sqlx::query_scalar(
            "SELECT HOLIDAY_DATE FROM HOLIDAYS_STANDARD \
             WHERE LOCALE = 'JP' AND HOLIDAY_DATE BETWEEN '2021-01-01' AND '2021-12-31' \
             ORDER BY HOLIDAY_DATE",
        )
        .fetch_all(pool)
        .await
        .expect("read 2021 holidays")
    }
    fn check(stage: &str, dates: &[String]) {
        let wrong: Vec<&str> = WRONG.iter().copied().filter(|d| dates.iter().any(|x| x == d)).collect();
        let missing: Vec<&str> = RIGHT.iter().copied().filter(|d| !dates.iter().any(|x| x == d)).collect();
        assert!(
            wrong.is_empty() && missing.is_empty(),
            "{}: wrong 2021 holidays present {:?}, real 2021 holidays missing {:?}",
            stage,
            wrong,
            missing
        );
    }

    let db = memory_db().await;
    run_startup(&db).await;
    let pool = db.pool();

    // 1. Fresh DB.
    let fresh = jp_2021(pool).await;

    // 2. Existing DB seeded by an older build: the wrong rows are stored and
    //    the real ones are absent; the next startup must repair it.
    for d in RIGHT {
        sqlx::query("DELETE FROM HOLIDAYS_STANDARD WHERE LOCALE = 'JP' AND HOLIDAY_DATE = ?")
            .bind(d)
            .execute(pool)
            .await
            .expect("delete");
    }
    for (d, name) in [("2021-07-19", "海の日"), ("2021-08-11", "山の日"), ("2021-10-11", "スポーツの日")] {
        sqlx::query(
            "INSERT OR IGNORE INTO HOLIDAYS_STANDARD (LOCALE, HOLIDAY_DATE, HOLIDAY_NAME) VALUES ('JP', ?, ?)",
        )
        .bind(d)
        .bind(name)
        .execute(pool)
        .await
        .expect("insert wrong row");
    }
    db.migrate_recurring().await.expect("re-run migrate_recurring");
    let repaired = jp_2021(pool).await;

    if seeds_2021 {
        check("fresh DB", &fresh);
    }
    check("existing DB after startup", &repaired);
}
