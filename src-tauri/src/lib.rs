// Moduły aplikacji
mod database;
mod models;
mod services;

use database::Database;
use models::{
    CreateHabitEntryRequest, CreateHabitRequest, CreateTaskRequest, Habit, HabitEntry, Task,
    UpdateHabitRequest,
};
use services::{habit_service, task_service};
use std::sync::Mutex;
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

/// Stan aplikacji zawierający połączenie z bazą danych
struct AppState {
    db: Mutex<Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicjalizacja bazy danych
    let db = database::initialize_database().expect("Failed to initialize database");

    println!("Database initialized successfully");

    // Tworzenie stanu aplikacji
    let app_state = AppState { db: Mutex::new(db) };

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
            get_habit_entries_for_habit
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
