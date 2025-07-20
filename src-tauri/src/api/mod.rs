use axum::{extract::State, http::Method, response::Json, routing::get, Router};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::database::Database;
use crate::services::{character_service, habit_service, task_service};

/// Shared application state for API endpoints
#[derive(Clone)]
pub struct ApiState {
    pub database: Arc<Mutex<Database>>,
}

/// Starts the HTTP API server on a background thread
/// Returns the server handle and port number
pub async fn start_api_server(
    database: Arc<Mutex<Database>>,
    port: u16,
) -> Result<tokio::task::JoinHandle<()>, anyhow::Error> {
    let state = ApiState { database };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/tasks", get(get_tasks))
        .route("/api/habits", get(get_habits))
        .route("/api/character", get(get_character))
        .layer(ServiceBuilder::new().layer(cors))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to port {}: {}", port, e))?;

    println!("🌐 Local API Server started on http://127.0.0.1:{}", port);
    println!("📋 Available endpoints:");
    println!("  GET /api/health    - Server health check");
    println!("  GET /api/tasks     - Get all tasks");
    println!("  GET /api/habits    - Get all habits with today's entries");
    println!("  GET /api/character - Get character data");

    let handle = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("❌ API Server error: {}", e);
        }
    });

    Ok(handle)
}

/// Health check endpoint
async fn health_check() -> Json<Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "PDRPG API",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Get all tasks endpoint
async fn get_tasks(State(state): State<ApiState>) -> Json<Value> {
    match state.database.lock() {
        Ok(db) => match task_service::get_all_tasks(db.connection()) {
            Ok(tasks) => Json(serde_json::json!({
                "success": true,
                "data": tasks,
                "count": tasks.len()
            })),
            Err(e) => Json(serde_json::json!({
                "success": false,
                "error": format!("Failed to get tasks: {}", e)
            })),
        },
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": format!("Database lock error: {}", e)
        })),
    }
}

/// Get all habits with today's entries endpoint
async fn get_habits(State(state): State<ApiState>) -> Json<Value> {
    match state.database.lock() {
        Ok(db) => {
            let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

            match habit_service::get_all_habits(db.connection()) {
                Ok(habits) => {
                    // Get today's entries for each habit
                    let mut habits_with_entries = Vec::new();

                    for habit in habits {
                        let entry =
                            habit_service::get_habit_entries_for_date(db.connection(), &today)
                                .unwrap_or_default()
                                .into_iter()
                                .find(|e| e.habit_id == habit.id);

                        habits_with_entries.push(serde_json::json!({
                            "habit": habit,
                            "today_entry": entry,
                            "today_completed": entry.as_ref()
                                .map(|e| match habit.habit_type {
                                    crate::models::HabitType::Boolean => e.completed,
                                    crate::models::HabitType::Counter => {
                                        if let Some(target) = habit.target_value {
                                            e.value >= target
                                        } else {
                                            e.value > 0
                                        }
                                    }
                                })
                                .unwrap_or(false)
                        }));
                    }

                    Json(serde_json::json!({
                        "success": true,
                        "data": habits_with_entries,
                        "count": habits_with_entries.len(),
                        "date": today
                    }))
                }
                Err(e) => Json(serde_json::json!({
                    "success": false,
                    "error": format!("Failed to get habits: {}", e)
                })),
            }
        }
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": format!("Database lock error: {}", e)
        })),
    }
}

/// Get character data endpoint
async fn get_character(State(state): State<ApiState>) -> Json<Value> {
    match state.database.lock() {
        Ok(db) => match character_service::get_character(db.connection()) {
            Ok(character) => {
                // Calculate level progress
                let current_level_exp = (character.level as f64 - 1.0).powi(2) * 100.0;
                let next_level_exp = (character.level as f64).powi(2) * 100.0;
                let progress = if next_level_exp > current_level_exp {
                    ((character.experience as f64 - current_level_exp)
                        / (next_level_exp - current_level_exp)
                        * 100.0)
                        .min(100.0)
                        .max(0.0)
                } else {
                    100.0
                };

                Json(serde_json::json!({
                    "success": true,
                    "data": {
                        "character": character,
                        "level_progress": {
                            "current_level_exp": current_level_exp as i64,
                            "next_level_exp": next_level_exp as i64,
                            "progress_percentage": progress,
                            "exp_to_next_level": (next_level_exp as i64) - character.experience
                        }
                    }
                }))
            }
            Err(e) => Json(serde_json::json!({
                "success": false,
                "error": format!("Failed to get character: {}", e)
            })),
        },
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": format!("Database lock error: {}", e)
        })),
    }
}
