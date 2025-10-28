use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use crate::consts::{DB_DIR_NAME, DB_FILE_NAME};

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(DB_DIR_NAME).join(DB_FILE_NAME)
}

pub fn get_all_translations(lang_code: &str) -> Result<HashMap<String, String>> {
    let conn = Connection::open(get_db_path())?;
    
    // First, check how many records exist
    let count_query = conn.query_row(
        "SELECT COUNT(*) FROM I18N_RESOURCES WHERE LANG_CODE = ?1",
        [lang_code],
        |row| row.get::<_, i64>(0)
    )?;
    
    // Log to file
    let log_path = std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("work")
        .join("db_i18n_debug.log");
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&log_path) {
        use std::io::Write;
        let _ = writeln!(file, "DB: COUNT query returned {} records for lang_code={}", count_query, lang_code);
    }
    
    let mut stmt = conn.prepare(
        "SELECT RESOURCE_KEY, RESOURCE_VALUE FROM I18N_RESOURCES WHERE LANG_CODE = ?1"
    )?;
    
    let rows = stmt.query_map([lang_code], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    
    let mut translations = HashMap::new();
    let mut count = 0;
    let mut found_menu_admin = false;
    for row_result in rows {
        let (key, value) = row_result?;
        if key == "menu.admin" {
            found_menu_admin = true;
            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&log_path) {
                use std::io::Write;
                let _ = writeln!(file, "DB: *** Found menu.admin: {}", value);
            }
        }
        if translations.insert(key.clone(), value.clone()).is_some() {
            if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&log_path) {
                use std::io::Write;
                let _ = writeln!(file, "DB: WARNING: Duplicate key detected: {}", key);
            }
        }
        count += 1;
    }
    
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(&log_path) {
        use std::io::Write;
        let _ = writeln!(file, "DB: Processed {} rows, HashMap size: {}, found_menu_admin: {}", count, translations.len(), found_menu_admin);
        let _ = writeln!(file, "DB: menu.admin in HashMap: {:?}", translations.get("menu.admin"));
        let _ = writeln!(file, "DB: menu.font_size in HashMap: {:?}", translations.get("menu.font_size"));
    }
    
    Ok(translations)
}
