// Moduły aplikacji
mod api;
mod database;
mod models;
mod services;

use database::Database;
use models::{
    Character, CreateCharacterRequest, CreateHabitEntryRequest, CreateHabitRequest,
    CreateTaskRequest, Habit, HabitEntry, Task, UpdateCharacterRequest, UpdateHabitRequest,
};
use services::{character_service, habit_service, task_service};
use std::sync::{Arc, Mutex};
use tauri::State;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Tauri command do dodawania nowego zadania
#[tauri::command]
fn add_task(title: String, state: State<AppState>) -> Result<Task, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    let request = CreateTaskRequest { title };

    task_service::add_task(conn, request).map_err(|e| format!("Failed to add task: {}", e))
}

/// Tauri command do pobierania wszystkich zadań
#[tauri::command]
fn get_all_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    task_service::get_all_tasks(conn).map_err(|e| format!("Failed to get tasks: {}", e))
}

/// Tauri command do przełączania statusu zadania
#[tauri::command]
fn toggle_task_status(task_id: i32, state: State<AppState>) -> Result<Task, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    task_service::toggle_task_status(conn, task_id)
        .map_err(|e| format!("Failed to toggle task status: {}", e))
}

/// Tauri command do usuwania zadania
#[tauri::command]
fn delete_task(task_id: i32, state: State<AppState>) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    task_service::delete_task(conn, task_id).map_err(|e| format!("Failed to delete task: {}", e))
}

// ==== HABIT COMMANDS ====

/// Tauri command do dodawania nowego nawyku
#[tauri::command]
fn add_habit(request: CreateHabitRequest, state: State<AppState>) -> Result<Habit, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    habit_service::add_habit(conn, request).map_err(|e| format!("Failed to add habit: {}", e))
}

/// Tauri command do pobierania wszystkich nawyków
#[tauri::command]
fn get_all_habits(state: State<AppState>) -> Result<Vec<Habit>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    habit_service::get_all_habits(conn).map_err(|e| format!("Failed to get habits: {}", e))
}

/// Tauri command do usuwania nawyku
#[tauri::command]
fn delete_habit(id: i32, state: State<AppState>) -> Result<(), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    habit_service::delete_habit(conn, id).map_err(|e| format!("Failed to delete habit: {}", e))
}

/// Tauri command do aktualizacji nawyku
#[tauri::command]
fn update_habit(
    habit_id: i32,
    request: UpdateHabitRequest,
    state: State<AppState>,
) -> Result<Habit, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    habit_service::update_habit(conn, habit_id, request)
        .map_err(|e| format!("Failed to update habit: {}", e))
}

/// Tauri command do dodawania wpisu nawyku
#[tauri::command]
fn add_habit_entry(
    request: CreateHabitEntryRequest,
    state: State<AppState>,
) -> Result<HabitEntry, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    habit_service::add_habit_entry(conn, request)
        .map_err(|e| format!("Failed to add habit entry: {}", e))
}

/// Tauri command do pobierania wpisów nawyków na konkretny dzień
#[tauri::command]
fn get_habit_entries_for_date(
    date: String,
    state: State<AppState>,
) -> Result<Vec<HabitEntry>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    habit_service::get_habit_entries_for_date(conn, &date)
        .map_err(|e| format!("Failed to get habit entries for date: {}", e))
}

/// Tauri command do pobierania wszystkich wpisów dla konkretnego nawyku
#[tauri::command]
fn get_habit_entries_for_habit(
    habit_id: i32,
    state: State<AppState>,
) -> Result<Vec<HabitEntry>, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    habit_service::get_habit_entries_for_habit(conn, habit_id)
        .map_err(|e| format!("Failed to get habit entries: {}", e))
}

// ==== CHARACTER COMMANDS ====

/// Tauri command do pobierania postaci gracza
#[tauri::command]
fn get_character(state: State<AppState>) -> Result<Character, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    character_service::get_character(conn).map_err(|e| format!("Failed to get character: {}", e))
}

/// Tauri command do tworzenia nowej postaci
#[tauri::command]
fn create_character(
    request: CreateCharacterRequest,
    state: State<AppState>,
) -> Result<Character, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    character_service::create_character(conn, request)
        .map_err(|e| format!("Failed to create character: {}", e))
}

/// Tauri command do aktualizacji postaci
#[tauri::command]
fn update_character(
    request: UpdateCharacterRequest,
    state: State<AppState>,
) -> Result<Character, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    character_service::update_character(conn, request)
        .map_err(|e| format!("Failed to update character: {}", e))
}

/// Tauri command do dodawania punktów doświadczenia
#[tauri::command]
fn add_experience(exp_points: i64, state: State<AppState>) -> Result<(Character, bool), String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    character_service::add_experience(conn, exp_points)
        .map_err(|e| format!("Failed to add experience: {}", e))
}

/// Tauri command do dodawania punktów atrybutu
#[tauri::command]
fn add_attribute_points(
    attribute: String,
    points: i32,
    state: State<AppState>,
) -> Result<Character, String> {
    let db = state
        .db
        .lock()
        .map_err(|e| format!("Database lock error: {}", e))?;
    let conn = db.connection();

    character_service::add_attribute_points(conn, &attribute, points)
        .map_err(|e| format!("Failed to add attribute points: {}", e))
}

/// Tauri command do uruchamiania API server
#[tauri::command]
async fn start_api_server(port: u16) -> Result<String, String> {
    // Utwórz osobną instancję bazy danych dla API
    let api_db = database::initialize_database()
        .map_err(|e| format!("Failed to initialize API database: {}", e))?;
    let db_arc = Arc::new(Mutex::new(api_db));

    // Uruchom API server w osobnym tokio task
    tokio::spawn(async move {
        match api::start_api_server(db_arc, port).await {
            Ok(server_handle) => {
                println!("✅ API Server started on port {}", port);
                if let Err(e) = server_handle.await {
                    eprintln!("❌ API Server task failed: {}", e);
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to start API server: {}", e);
            }
        }
    });

    Ok(format!(
        "API Server starting on port {}. Check console for status.",
        port
    ))
}

/// Tauri command do sprawdzania statusu API server
#[tauri::command]
async fn check_api_status(port: u16) -> Result<bool, String> {
    // Sprawdź czy port jest zajęty
    match tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await {
        Ok(_) => Ok(false), // Port wolny - API server nie działa
        Err(_) => Ok(true), // Port zajęty - prawdopodobnie API server działa
    }
}

/// Stan aplikacji zawierający połączenie z bazą danych
struct AppState {
    db: Mutex<Database>,
    api_server_handle: Option<tokio::task::JoinHandle<()>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicjalizacja bazy danych
    let db = database::initialize_database().expect("Failed to initialize database");

    println!("Database initialized successfully");

    // Tworzenie stanu aplikacji (API server zostanie uruchomiony na żądanie)
    let app_state = AppState {
        db: Mutex::new(db),
        api_server_handle: None,
    };

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            add_task,
            get_all_tasks,
            toggle_task_status,
            delete_task,
            add_habit,
            get_all_habits,
            delete_habit,
            update_habit,
            add_habit_entry,
            get_habit_entries_for_date,
            get_habit_entries_for_habit,
            get_character,
            create_character,
            update_character,
            add_experience,
            add_attribute_points,
            start_api_server,
            check_api_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
