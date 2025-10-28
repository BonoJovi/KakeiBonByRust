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
    
    let mut stmt = conn.prepare(
        "SELECT RESOURCE_KEY, RESOURCE_VALUE FROM I18N_RESOURCES WHERE LANG_CODE = ?1"
    )?;
    
    let rows = stmt.query_map([lang_code], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    })?;
    
    let mut translations = HashMap::new();
    for row_result in rows {
        let (key, value) = row_result?;
        translations.insert(key, value);
    }
    
    Ok(translations)
}

