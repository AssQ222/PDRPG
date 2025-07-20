// Moduły aplikacji
mod database;
mod models;

use database::Database;
use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Stan aplikacji zawierający połączenie z bazą danych
struct AppState {
    db: Mutex<Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicjalizacja bazy danych
    let db = database::initialize_database()
        .expect("Failed to initialize database");
    
    println!("Database initialized successfully");
    
    // Tworzenie stanu aplikacji
    let app_state = AppState {
        db: Mutex::new(db),
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
