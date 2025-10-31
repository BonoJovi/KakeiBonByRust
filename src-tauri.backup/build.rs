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
    
    tauri_build::build()
}
