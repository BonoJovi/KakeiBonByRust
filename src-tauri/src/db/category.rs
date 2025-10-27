use rusqlite::{Connection, Result};
use crate::models::category::{Category1, Category2, Category3, CategoryTree, Category2WithChildren};

/// Get database connection
pub fn get_connection() -> Result<Connection> {
    Connection::open("src-tauri/kakeibo.db")
}

/// Get all categories for a user as a tree structure
pub fn get_category_tree(user_id: i64) -> Result<Vec<CategoryTree>> {
    let conn = get_connection()?;
    
    // Get all Category1
    let mut stmt1 = conn.prepare(
        "SELECT user_id, category1_code, display_order, category1_name, is_disabled, entry_dt, update_dt 
         FROM CATEGORY1 
         WHERE user_id = ?1 AND is_disabled = 0
         ORDER BY display_order"
    )?;
    
    let cat1_iter = stmt1.query_map([user_id], |row| {
        Ok(Category1 {
            user_id: row.get(0)?,
            category1_code: row.get(1)?,
            display_order: row.get(2)?,
            category1_name: row.get(3)?,
            is_disabled: row.get(4)?,
            entry_dt: row.get(5)?,
            update_dt: row.get(6)?,
        })
    })?;
    
    let mut result = Vec::new();
    
    for cat1 in cat1_iter {
        let cat1 = cat1?;
        let cat1_code = cat1.category1_code.clone();
        
        // Get Category2 for this Category1
        let mut stmt2 = conn.prepare(
            "SELECT user_id, category1_code, category2_code, display_order, category2_name, is_disabled, entry_dt, update_dt 
             FROM CATEGORY2 
             WHERE user_id = ?1 AND category1_code = ?2 AND is_disabled = 0
             ORDER BY display_order"
        )?;
        
        let cat2_iter = stmt2.query_map([user_id, cat1_code.parse().unwrap()], |row| {
            Ok(Category2 {
                user_id: row.get(0)?,
                category1_code: row.get(1)?,
                category2_code: row.get(2)?,
                display_order: row.get(3)?,
                category2_name: row.get(4)?,
                is_disabled: row.get(5)?,
                entry_dt: row.get(6)?,
                update_dt: row.get(7)?,
            })
        })?;
        
        let mut cat2_with_children = Vec::new();
        
        for cat2 in cat2_iter {
            let cat2 = cat2?;
            let cat2_code = cat2.category2_code.clone();
            
            // Get Category3 for this Category2
            let mut stmt3 = conn.prepare(
                "SELECT user_id, category1_code, category2_code, category3_code, display_order, category3_name, is_disabled, entry_dt, update_dt 
                 FROM CATEGORY3 
                 WHERE user_id = ?1 AND category1_code = ?2 AND category2_code = ?3 AND is_disabled = 0
                 ORDER BY display_order"
            )?;
            
            let cat3_iter = stmt3.query_map([user_id.to_string(), cat1_code.clone(), cat2_code], |row| {
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
                })
            })?;
            
            let cat3_list: Result<Vec<_>> = cat3_iter.collect();
            
            cat2_with_children.push(Category2WithChildren {
                category2: cat2,
                children: cat3_list?,
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
    
    // Commit transaction
    conn.execute("COMMIT", [])?;
    
    Ok(())
}
