use std::fs;
use std::path::Path;

fn main() {
    // Copy sql_queries.rs from workspace root to src-tauri/src before build
    let src = Path::new("../src/sql_queries.rs");
    let dest = Path::new("src/sql_queries.rs");
    
    if src.exists() {
        fs::copy(src, dest).expect("Failed to copy sql_queries.rs");
        println!("cargo:rerun-if-changed=../src/sql_queries.rs");
    }

    // dbaccess.sql is embedded via include_str! in src/db.rs; cargo does not
    // auto-track include_str! sources once build.rs prints any
    // rerun-if-changed line, so SQL-only edits (e.g. new I18N_RESOURCES rows)
    // would otherwise produce an unchanged binary on `cargo tauri dev`.
    println!("cargo:rerun-if-changed=../res/sql/dbaccess.sql");
    println!("cargo:rerun-if-changed=../res/sql/default_categories_seed.sql");

    tauri_build::build()
}
