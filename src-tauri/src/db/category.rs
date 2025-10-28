use rusqlite::{Connection, Result};
use crate::models::category::{Category1, Category2, Category3, CategoryTree, Category2WithChildren};

/// Get database connection
pub fn get_connection() -> Result<Connection> {
    Connection::open("src-tauri/kakeibo.db")
}

/// Get all categories for a user as a tree structure with optional language support
pub fn get_category_tree(user_id: i64) -> Result<Vec<CategoryTree>> {
    get_category_tree_with_lang(user_id, None)
}

/// Get all categories for a user as a tree structure with language support
pub fn get_category_tree_with_lang(user_id: i64, lang_code: Option<String>) -> Result<Vec<CategoryTree>> {
    let conn = get_connection()?;
    
    // Get all Category1 with optional I18N
    let sql = if lang_code.is_some() {
        "SELECT c1.user_id, c1.category1_code, c1.display_order, c1.category1_name, c1.is_disabled, c1.entry_dt, c1.update_dt,
                i18n.category1_name_i18n
         FROM CATEGORY1 c1
         LEFT JOIN CATEGORY1_I18N i18n ON c1.user_id = i18n.user_id 
            AND c1.category1_code = i18n.category1_code 
            AND i18n.lang_code = ?2
         WHERE c1.user_id = ?1 AND c1.is_disabled = 0
         ORDER BY c1.display_order"
    } else {
        "SELECT user_id, category1_code, display_order, category1_name, is_disabled, entry_dt, update_dt, NULL as category1_name_i18n
         FROM CATEGORY1
         WHERE user_id = ?1 AND is_disabled = 0
         ORDER BY display_order"
    };
    
    let mut stmt1 = conn.prepare(sql)?;
    
    let cat1_list: Vec<Category1> = if let Some(ref lang) = lang_code {
        stmt1.query_map([user_id.to_string(), lang.clone()], |row| {
            Ok(Category1 {
                user_id: row.get(0)?,
                category1_code: row.get(1)?,
                display_order: row.get(2)?,
                category1_name: row.get(3)?,
                is_disabled: row.get(4)?,
                entry_dt: row.get(5)?,
                update_dt: row.get(6)?,
                category1_name_i18n: row.get(7)?,
            })
        })?.collect::<Result<Vec<_>>>()?
    } else {
        stmt1.query_map([user_id], |row| {
            Ok(Category1 {
                user_id: row.get(0)?,
                category1_code: row.get(1)?,
                display_order: row.get(2)?,
                category1_name: row.get(3)?,
                is_disabled: row.get(4)?,
                entry_dt: row.get(5)?,
                update_dt: row.get(6)?,
                category1_name_i18n: None,
            })
        })?.collect::<Result<Vec<_>>>()?
    };
    
    let mut result = Vec::new();
    
    for cat1 in cat1_list {
        let cat1_code = cat1.category1_code.clone();
        
        // Get Category2 for this Category1 with optional I18N
        let sql2 = if lang_code.is_some() {
            "SELECT c2.user_id, c2.category1_code, c2.category2_code, c2.display_order, c2.category2_name, c2.is_disabled, c2.entry_dt, c2.update_dt,
                    i18n.category2_name_i18n
             FROM CATEGORY2 c2
             LEFT JOIN CATEGORY2_I18N i18n ON c2.user_id = i18n.user_id 
                AND c2.category1_code = i18n.category1_code 
                AND c2.category2_code = i18n.category2_code
                AND i18n.lang_code = ?3
             WHERE c2.user_id = ?1 AND c2.category1_code = ?2 AND c2.is_disabled = 0
             ORDER BY c2.display_order"
        } else {
            "SELECT user_id, category1_code, category2_code, display_order, category2_name, is_disabled, entry_dt, update_dt, NULL as category2_name_i18n
             FROM CATEGORY2
             WHERE user_id = ?1 AND category1_code = ?2 AND is_disabled = 0
             ORDER BY display_order"
        };
        
        let mut stmt2 = conn.prepare(sql2)?;
        
        let cat2_list: Vec<Category2> = if let Some(ref lang) = lang_code {
            stmt2.query_map([user_id.to_string(), cat1_code.clone(), lang.clone()], |row| {
                Ok(Category2 {
                    user_id: row.get(0)?,
                    category1_code: row.get(1)?,
                    category2_code: row.get(2)?,
                    display_order: row.get(3)?,
                    category2_name: row.get(4)?,
                    is_disabled: row.get(5)?,
                    entry_dt: row.get(6)?,
                    update_dt: row.get(7)?,
                    category2_name_i18n: row.get(8)?,
                })
            })?.collect::<Result<Vec<_>>>()?
        } else {
            stmt2.query_map([user_id, cat1_code.parse().unwrap()], |row| {
                Ok(Category2 {
                    user_id: row.get(0)?,
                    category1_code: row.get(1)?,
                    category2_code: row.get(2)?,
                    display_order: row.get(3)?,
                    category2_name: row.get(4)?,
                    is_disabled: row.get(5)?,
                    entry_dt: row.get(6)?,
                    update_dt: row.get(7)?,
                    category2_name_i18n: None,
                })
            })?.collect::<Result<Vec<_>>>()?
        };
        
        let mut cat2_with_children = Vec::new();
        
        for cat2 in cat2_list {
            let cat2_code = cat2.category2_code.clone();
            
            // Get Category3 for this Category2 with optional I18N
            let sql3 = if lang_code.is_some() {
                "SELECT c3.user_id, c3.category1_code, c3.category2_code, c3.category3_code, c3.display_order, c3.category3_name, c3.is_disabled, c3.entry_dt, c3.update_dt,
                        i18n.category3_name_i18n
                 FROM CATEGORY3 c3
                 LEFT JOIN CATEGORY3_I18N i18n ON c3.user_id = i18n.user_id 
                    AND c3.category1_code = i18n.category1_code 
                    AND c3.category2_code = i18n.category2_code
                    AND c3.category3_code = i18n.category3_code
                    AND i18n.lang_code = ?4
                 WHERE c3.user_id = ?1 AND c3.category1_code = ?2 AND c3.category2_code = ?3 AND c3.is_disabled = 0
                 ORDER BY c3.display_order"
            } else {
                "SELECT user_id, category1_code, category2_code, category3_code, display_order, category3_name, is_disabled, entry_dt, update_dt, NULL as category3_name_i18n
                 FROM CATEGORY3
                 WHERE user_id = ?1 AND category1_code = ?2 AND category2_code = ?3 AND is_disabled = 0
                 ORDER BY display_order"
            };
            
            let mut stmt3 = conn.prepare(sql3)?;
            
            let cat3_list: Vec<Category3> = if let Some(ref lang) = lang_code {
                stmt3.query_map([user_id.to_string(), cat1_code.clone(), cat2_code.clone(), lang.clone()], |row| {
                    Ok(Category3 {
                        user_id: row.get(0)?,
                        category1_code: row.get(1)?,
                        category2_code: row.get(2)?,
                        category3_code: row.get(3)?,
                        display_order: row.get(4)?,
                        category3_name: row.get(5)?,
                        is_disabled: row.get(6)?,
                        entry_dt: row.get(7)?,
                        update_dt: row.get(8)?,
                        category3_name_i18n: row.get(9)?,
                    })
                })?.collect::<Result<Vec<_>>>()?
            } else {
                stmt3.query_map([user_id.to_string(), cat1_code.clone(), cat2_code], |row| {
                    Ok(Category3 {
                        user_id: row.get(0)?,
                        category1_code: row.get(1)?,
                        category2_code: row.get(2)?,
                        category3_code: row.get(3)?,
                        display_order: row.get(4)?,
                        category3_name: row.get(5)?,
                        is_disabled: row.get(6)?,
                        entry_dt: row.get(7)?,
                        update_dt: row.get(8)?,
                        category3_name_i18n: None,
                    })
                })?.collect::<Result<Vec<_>>>()?
            };
            
            cat2_with_children.push(Category2WithChildren {
                category2: cat2,
                children: cat3_list,
            });
        }
        
        result.push(CategoryTree {
            category1: cat1,
            children: cat2_with_children,
        });
    }
    
    Ok(result)
}

/// Get Category1 list for a user
pub fn get_category1_list(user_id: i64) -> Result<Vec<Category1>> {
    let conn = get_connection()?;
    
    let mut stmt = conn.prepare(
        "SELECT user_id, category1_code, display_order, category1_name, is_disabled, entry_dt, update_dt 
         FROM CATEGORY1 
         WHERE user_id = ?1 AND is_disabled = 0
         ORDER BY display_order"
    )?;
    
    let cat1_iter = stmt.query_map([user_id], |row| {
        Ok(Category1 {
            user_id: row.get(0)?,
            category1_code: row.get(1)?,
            display_order: row.get(2)?,
            category1_name: row.get(3)?,
            is_disabled: row.get(4)?,
            entry_dt: row.get(5)?,
            update_dt: row.get(6)?,
            category1_name_i18n: None,
        })
    })?;
    
    cat1_iter.collect()
}

/// Add new Category1
pub fn add_category1(user_id: i64, code: String, name: String) -> Result<()> {
    let conn = get_connection()?;
    
    // Get max display_order
    let max_order: i32 = conn.query_row(
        "SELECT COALESCE(MAX(display_order), 0) FROM CATEGORY1 WHERE user_id = ?1",
        [user_id],
        |row| row.get(0)
    )?;
    
    conn.execute(
        "INSERT INTO CATEGORY1 (user_id, category1_code, display_order, category1_name, is_disabled, entry_dt) 
         VALUES (?1, ?2, ?3, ?4, 0, datetime('now'))",
        [user_id.to_string(), code, (max_order + 1).to_string(), name],
    )?;
    
    Ok(())
}

/// Update Category1
pub fn update_category1(user_id: i64, code: String, name: String) -> Result<()> {
    let conn = get_connection()?;
    
    conn.execute(
        "UPDATE CATEGORY1 SET category1_name = ?1, update_dt = datetime('now') 
         WHERE user_id = ?2 AND category1_code = ?3",
        [name, user_id.to_string(), code],
    )?;
    
    Ok(())
}

/// Move Category1 order (up or down)
pub fn move_category1_order(user_id: i64, code: String, direction: i32) -> Result<()> {
    let conn = get_connection()?;
    
    // Get current order
    let current_order: i32 = conn.query_row(
        "SELECT display_order FROM CATEGORY1 WHERE user_id = ?1 AND category1_code = ?2",
        [user_id.to_string(), code.clone()],
        |row| row.get(0)
    )?;
    
    let new_order = current_order + direction;
    
    if new_order < 1 {
        return Ok(()); // Cannot move up from first position
    }
    
    // Swap with adjacent item
    conn.execute(
        "UPDATE CATEGORY1 SET display_order = ?1 WHERE user_id = ?2 AND display_order = ?3",
        [current_order.to_string(), user_id.to_string(), new_order.to_string()],
    )?;
    
    conn.execute(
        "UPDATE CATEGORY1 SET display_order = ?1 WHERE user_id = ?2 AND category1_code = ?3",
        [new_order.to_string(), user_id.to_string(), code],
    )?;
    
    Ok(())
}

/// Delete Category1 and all its children (CASCADE)
pub fn delete_category1(user_id: i64, code: String) -> Result<()> {
    let conn = get_connection()?;
    
    conn.execute(
        "DELETE FROM CATEGORY1 WHERE user_id = ?1 AND category1_code = ?2",
        [user_id.to_string(), code],
    )?;
    
    Ok(())
}

/// Add new Category2
pub fn add_category2(user_id: i64, category1_code: String, category2_code: String, name: String) -> Result<()> {
    let conn = get_connection()?;
    
    // Get max display_order for this parent
    let max_order: i32 = conn.query_row(
        "SELECT COALESCE(MAX(display_order), 0) FROM CATEGORY2 
         WHERE user_id = ?1 AND category1_code = ?2",
        [user_id.to_string(), category1_code.clone()],
        |row| row.get(0)
    )?;
    
    conn.execute(
        "INSERT INTO CATEGORY2 (user_id, category1_code, category2_code, display_order, category2_name, is_disabled, entry_dt) 
         VALUES (?1, ?2, ?3, ?4, ?5, 0, datetime('now'))",
        [user_id.to_string(), category1_code, category2_code, (max_order + 1).to_string(), name],
    )?;
    
    Ok(())
}

/// Update Category2
pub fn update_category2(user_id: i64, category1_code: String, category2_code: String, name: String) -> Result<()> {
    let conn = get_connection()?;
    
    conn.execute(
        "UPDATE CATEGORY2 SET category2_name = ?1, update_dt = datetime('now') 
         WHERE user_id = ?2 AND category1_code = ?3 AND category2_code = ?4",
        [name, user_id.to_string(), category1_code, category2_code],
    )?;
    
    Ok(())
}

/// Move Category2 order (up or down)
pub fn move_category2_order(user_id: i64, category1_code: String, category2_code: String, direction: i32) -> Result<()> {
    let conn = get_connection()?;
    
    // Get current order
    let current_order: i32 = conn.query_row(
        "SELECT display_order FROM CATEGORY2 
         WHERE user_id = ?1 AND category1_code = ?2 AND category2_code = ?3",
        [user_id.to_string(), category1_code.clone(), category2_code.clone()],
        |row| row.get(0)
    )?;
    
    let new_order = current_order + direction;
    
    if new_order < 1 {
        return Ok(()); // Cannot move up from first position
    }
    
    // Swap with adjacent item
    conn.execute(
        "UPDATE CATEGORY2 SET display_order = ?1 
         WHERE user_id = ?2 AND category1_code = ?3 AND display_order = ?4",
        [current_order.to_string(), user_id.to_string(), category1_code.clone(), new_order.to_string()],
    )?;
    
    conn.execute(
        "UPDATE CATEGORY2 SET display_order = ?1 
         WHERE user_id = ?2 AND category1_code = ?3 AND category2_code = ?4",
        [new_order.to_string(), user_id.to_string(), category1_code, category2_code],
    )?;
    
    Ok(())
}

/// Delete Category2 and all its children (CASCADE)
pub fn delete_category2(user_id: i64, category1_code: String, category2_code: String) -> Result<()> {
    let conn = get_connection()?;
    
    conn.execute(
        "DELETE FROM CATEGORY2 WHERE user_id = ?1 AND category1_code = ?2 AND category2_code = ?3",
        [user_id.to_string(), category1_code, category2_code],
    )?;
    
    Ok(())
}

/// Add new Category3
pub fn add_category3(user_id: i64, category1_code: String, category2_code: String, category3_code: String, name: String) -> Result<()> {
    let conn = get_connection()?;
    
    // Get max display_order for this parent
    let max_order: i32 = conn.query_row(
        "SELECT COALESCE(MAX(display_order), 0) FROM CATEGORY3 
         WHERE user_id = ?1 AND category1_code = ?2 AND category2_code = ?3",
        [user_id.to_string(), category1_code.clone(), category2_code.clone()],
        |row| row.get(0)
    )?;
    
    conn.execute(
        "INSERT INTO CATEGORY3 (user_id, category1_code, category2_code, category3_code, display_order, category3_name, is_disabled, entry_dt) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, datetime('now'))",
        [user_id.to_string(), category1_code, category2_code, category3_code, (max_order + 1).to_string(), name],
    )?;
    
    Ok(())
}

/// Update Category3
pub fn update_category3(user_id: i64, category1_code: String, category2_code: String, category3_code: String, name: String) -> Result<()> {
    let conn = get_connection()?;
    
    conn.execute(
        "UPDATE CATEGORY3 SET category3_name = ?1, update_dt = datetime('now') 
         WHERE user_id = ?2 AND category1_code = ?3 AND category2_code = ?4 AND category3_code = ?5",
        [name, user_id.to_string(), category1_code, category2_code, category3_code],
    )?;
    
    Ok(())
}

/// Move Category3 order (up or down)
pub fn move_category3_order(user_id: i64, category1_code: String, category2_code: String, category3_code: String, direction: i32) -> Result<()> {
    let conn = get_connection()?;
    
    // Get current order
    let current_order: i32 = conn.query_row(
        "SELECT display_order FROM CATEGORY3 
         WHERE user_id = ?1 AND category1_code = ?2 AND category2_code = ?3 AND category3_code = ?4",
        [user_id.to_string(), category1_code.clone(), category2_code.clone(), category3_code.clone()],
        |row| row.get(0)
    )?;
    
    let new_order = current_order + direction;
    
    if new_order < 1 {
        return Ok(()); // Cannot move up from first position
    }
    
    // Swap with adjacent item
    conn.execute(
        "UPDATE CATEGORY3 SET display_order = ?1 
         WHERE user_id = ?2 AND category1_code = ?3 AND category2_code = ?4 AND display_order = ?5",
        [current_order.to_string(), user_id.to_string(), category1_code.clone(), category2_code.clone(), new_order.to_string()],
    )?;
    
    conn.execute(
        "UPDATE CATEGORY3 SET display_order = ?1 
         WHERE user_id = ?2 AND category1_code = ?3 AND category2_code = ?4 AND category3_code = ?5",
        [new_order.to_string(), user_id.to_string(), category1_code, category2_code, category3_code],
    )?;
    
    Ok(())
}

/// Delete Category3
pub fn delete_category3(user_id: i64, category1_code: String, category2_code: String, category3_code: String) -> Result<()> {
    let conn = get_connection()?;
    
    conn.execute(
        "DELETE FROM CATEGORY3 WHERE user_id = ?1 AND category1_code = ?2 AND category2_code = ?3 AND category3_code = ?4",
        [user_id.to_string(), category1_code, category2_code, category3_code],
    )?;
    
    Ok(())
}

/// Initialize categories for a new user by copying from template user (USER_ID=1)
pub fn initialize_categories_for_new_user(new_user_id: i64) -> Result<()> {
    let conn = get_connection()?;
    
    // Start transaction
    conn.execute("BEGIN TRANSACTION", [])?;
    
    // Copy CATEGORY1
    conn.execute(
        "INSERT INTO CATEGORY1 (user_id, category1_code, display_order, category1_name, is_disabled, entry_dt)
         SELECT ?1, category1_code, display_order, category1_name, is_disabled, datetime('now')
         FROM CATEGORY1
         WHERE user_id = 1",
        [new_user_id],
    )?;
    
    // Copy CATEGORY2
    conn.execute(
        "INSERT INTO CATEGORY2 (user_id, category1_code, category2_code, display_order, category2_name, is_disabled, entry_dt)
         SELECT ?1, category1_code, category2_code, display_order, category2_name, is_disabled, datetime('now')
         FROM CATEGORY2
         WHERE user_id = 1",
        [new_user_id],
    )?;
    
    // Copy CATEGORY3
    conn.execute(
        "INSERT INTO CATEGORY3 (user_id, category1_code, category2_code, category3_code, display_order, category3_name, is_disabled, entry_dt)
         SELECT ?1, category1_code, category2_code, category3_code, display_order, category3_name, is_disabled, datetime('now')
         FROM CATEGORY3
         WHERE user_id = 1",
        [new_user_id],
    )?;
    
    // Copy CATEGORY1_I18N
    conn.execute(
        "INSERT INTO CATEGORY1_I18N (user_id, category1_code, lang_code, category1_name_i18n, entry_dt)
         SELECT ?1, category1_code, lang_code, category1_name_i18n, datetime('now')
         FROM CATEGORY1_I18N
         WHERE user_id = 1",
        [new_user_id],
    )?;
    
    // Copy CATEGORY2_I18N
    conn.execute(
        "INSERT INTO CATEGORY2_I18N (user_id, category1_code, category2_code, lang_code, category2_name_i18n, entry_dt)
         SELECT ?1, category1_code, category2_code, lang_code, category2_name_i18n, datetime('now')
         FROM CATEGORY2_I18N
         WHERE user_id = 1",
        [new_user_id],
    )?;
    
    // Copy CATEGORY3_I18N
    conn.execute(
        "INSERT INTO CATEGORY3_I18N (user_id, category1_code, category2_code, category3_code, lang_code, category3_name_i18n, entry_dt)
         SELECT ?1, category1_code, category2_code, category3_code, lang_code, category3_name_i18n, datetime('now')
         FROM CATEGORY3_I18N
         WHERE user_id = 1",
        [new_user_id],
    )?;
    
    // Commit transaction
    conn.execute("COMMIT", [])?;
    
    Ok(())
}
