mod models;
mod db;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::category::get_category_tree,
      commands::category::get_category_tree_with_lang,
      commands::category::add_category1,
      commands::category::update_category1,
      commands::category::move_category1_order,
      commands::category::delete_category1,
      commands::category::add_category2,
      commands::category::update_category2,
      commands::category::move_category2_order,
      commands::category::delete_category2,
      commands::category::add_category3,
      commands::category::update_category3,
      commands::category::move_category3_order,
      commands::category::delete_category3,
      commands::category::initialize_categories_for_new_user,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
