use kakebo_data_model::connection::{DbPoolState, create_pool, run_migrations};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust! ATTE: YM2", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let db_pool = create_pool().expect("Failed to create database pool");

    tauri::Builder::default()
        .manage(DbPoolState(db_pool.clone()))
        .setup(move |_app| {
            run_migrations(&db_pool).expect("Failed to run migrations");
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
