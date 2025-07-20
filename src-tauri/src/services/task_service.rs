use crate::models::{CreateTaskRequest, Task};
use crate::services::character_service;
use anyhow::Result;
use rusqlite::Connection;

/// Dodaje nowe zadanie do bazy danych
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `request` - Dane nowego zadania
///
/// # Returns
/// * `Result<Task>` - Nowo utworzone zadanie lub błąd
pub fn add_task(conn: &Connection, request: CreateTaskRequest) -> Result<Task> {
    // Validate title length
    if request.title.trim().is_empty() {
        return Err(anyhow::anyhow!("Task title cannot be empty"));
    }
    if request.title.len() > 100 {
        return Err(anyhow::anyhow!(
            "Task title is too long (max 100 characters)"
        ));
    }

    let mut task = Task::new(request.title);

    let sql =
        "INSERT INTO tasks (title, completed, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)";
    conn.execute(
        sql,
        (
            &task.title,
            task.completed,
            task.created_at,
            task.updated_at,
        ),
    )?;

    // Pobierz ID nowo utworzonego zadania
    task.id = conn.last_insert_rowid() as i32;

    Ok(task)
}

/// Pobiera wszystkie zadania z bazy danych
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
///
/// # Returns
/// * `Result<Vec<Task>>` - Lista wszystkich zadań lub błąd
pub fn get_all_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let sql =
        "SELECT id, title, completed, created_at, updated_at FROM tasks ORDER BY created_at DESC";
    let mut stmt = conn.prepare(sql)?;

    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }

    Ok(tasks)
}

/// Przełącza status ukończenia zadania
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `task_id` - ID zadania do przełączenia
///
/// # Returns
/// * `Result<Task>` - Zaktualizowane zadanie lub błąd
pub fn toggle_task_status(conn: &Connection, task_id: i32) -> Result<Task> {
    // Pobierz aktualne zadanie
    let sql = "SELECT id, title, completed, created_at, updated_at FROM tasks WHERE id = ?1";
    let mut stmt = conn.prepare(sql)?;

    let mut task = stmt.query_row([task_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    let was_completed = task.completed;

    // Przełącz status
    task.toggle_completed();

    // Zaktualizuj w bazie danych
    let update_sql = "UPDATE tasks SET completed = ?1, updated_at = ?2 WHERE id = ?3";
    conn.execute(update_sql, (task.completed, task.updated_at, task.id))?;

    // Jeśli zadanie zostało ukończone (przeszło z false na true), dodaj EXP
    if !was_completed && task.completed {
        // TODO: W przyszłości można dodać logikę wykrywania czy zadanie jest powiązane z celem
        let is_goal_related = false;

        // Przetwórz ukończenie zadania i dodaj EXP
        match character_service::process_task_completion(conn, &task.title, is_goal_related) {
            Ok((_, level_up)) => {
                if level_up {
                    println!(
                        "Level up! Task '{}' caused character to level up!",
                        task.title
                    );
                }
            }
            Err(e) => {
                // Loguj błąd ale nie przerywaj operacji - zadanie zostało już zaktualizowane
                eprintln!("Failed to process task completion for EXP: {}", e);
            }
        }
    }

    Ok(task)
}

/// Usuwa zadanie z bazy danych
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `task_id` - ID zadania do usunięcia
///
/// # Returns
/// * `Result<()>` - Sukces lub błąd
pub fn delete_task(conn: &Connection, task_id: i32) -> Result<()> {
    let sql = "DELETE FROM tasks WHERE id = ?1";
    let rows_affected = conn.execute(sql, [task_id])?;

    if rows_affected == 0 {
        return Err(anyhow::anyhow!("Task with id {} not found", task_id));
    }

    Ok(())
}

/// Pobiera zadanie po ID
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `task_id` - ID zadania
///
/// # Returns
/// * `Result<Task>` - Zadanie lub błąd
#[cfg(test)]
pub fn get_task_by_id(conn: &Connection, task_id: i32) -> Result<Task> {
    let sql = "SELECT id, title, completed, created_at, updated_at FROM tasks WHERE id = ?1";
    let mut stmt = conn.prepare(sql)?;

    let task = stmt.query_row([task_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    Ok(task)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateTaskRequest;
    use rusqlite::{Connection, Result as SqliteResult};

    fn create_test_db() -> SqliteResult<Connection> {
        let conn = Connection::open_in_memory()?;

        // Utwórz tabelę tasks
        conn.execute(
            "CREATE TABLE tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        Ok(conn)
    }

    #[test]
    fn test_add_task() {
        let conn = create_test_db().unwrap();
        let request = CreateTaskRequest {
            title: "Test task".to_string(),
        };

        let task = add_task(&conn, request).unwrap();

        assert_eq!(task.title, "Test task");
        assert!(!task.completed);
        assert!(task.id > 0);
    }

    #[test]
    fn test_get_all_tasks() {
        let conn = create_test_db().unwrap();

        // Dodaj kilka zadań
        let request1 = CreateTaskRequest {
            title: "Task 1".to_string(),
        };
        let request2 = CreateTaskRequest {
            title: "Task 2".to_string(),
        };

        add_task(&conn, request1).unwrap();

        // Dodaj małe opóźnienie żeby mieć różne timestampy
        std::thread::sleep(std::time::Duration::from_millis(10));

        add_task(&conn, request2).unwrap();

        let tasks = get_all_tasks(&conn).unwrap();

        assert_eq!(tasks.len(), 2);

        // Sprawdź że wszystkie zadania są obecne
        let task_titles: Vec<String> = tasks.iter().map(|t| t.title.clone()).collect();
        assert!(task_titles.contains(&"Task 1".to_string()));
        assert!(task_titles.contains(&"Task 2".to_string()));

        // Sprawdź że zadania są sortowane według created_at DESC (najnowsze pierwsze)
        assert!(tasks[0].created_at >= tasks[1].created_at);
    }

    #[test]
    fn test_toggle_task_status() {
        let conn = create_test_db().unwrap();
        let request = CreateTaskRequest {
            title: "Toggle test".to_string(),
        };

        let task = add_task(&conn, request).unwrap();
        assert!(!task.completed);

        let toggled_task = toggle_task_status(&conn, task.id).unwrap();
        assert!(toggled_task.completed);

        let toggled_again = toggle_task_status(&conn, task.id).unwrap();
        assert!(!toggled_again.completed);
    }

    #[test]
    fn test_delete_task() {
        let conn = create_test_db().unwrap();
        let request = CreateTaskRequest {
            title: "Delete test".to_string(),
        };

        let task = add_task(&conn, request).unwrap();

        // Sprawdź, że zadanie istnieje
        let result = get_task_by_id(&conn, task.id);
        assert!(result.is_ok());

        // Usuń zadanie
        delete_task(&conn, task.id).unwrap();

        // Sprawdź, że zadanie nie istnieje
        let result = get_task_by_id(&conn, task.id);
        assert!(result.is_err());
    }
}
