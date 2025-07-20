use crate::models::{
    CreateHabitEntryRequest, CreateHabitRequest, Habit, HabitEntry, HabitType, UpdateHabitRequest,
};
use crate::services::character_service;
use anyhow::Result;
use rusqlite::Connection;

/// Dodaje nowy nawyk do bazy danych
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `request` - Dane nowego nawyku
///
/// # Returns
/// * `Result<Habit>` - Nowo utworzony nawyk lub błąd
pub fn add_habit(conn: &Connection, request: CreateHabitRequest) -> Result<Habit> {
    // Validate title length
    if request.title.trim().is_empty() {
        return Err(anyhow::anyhow!("Habit title cannot be empty"));
    }
    if request.title.len() > 50 {
        return Err(anyhow::anyhow!(
            "Habit title is too long (max 50 characters)"
        ));
    }

    let mut habit = Habit::new(request.title, request.habit_type, request.target_value);

    let habit_type_str = match habit.habit_type {
        HabitType::Boolean => "Boolean",
        HabitType::Counter => "Counter",
    };

    let sql = "INSERT INTO habits (title, habit_type, target_value, current_streak, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
    conn.execute(
        sql,
        (
            &habit.title,
            habit_type_str,
            habit.target_value,
            habit.current_streak,
            habit.created_at,
            habit.updated_at,
        ),
    )?;

    // Pobierz ID nowo utworzonego nawyku
    habit.id = conn.last_insert_rowid() as i32;

    Ok(habit)
}

/// Pobiera wszystkie nawyki z bazy danych
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
///
/// # Returns
/// * `Result<Vec<Habit>>` - Lista wszystkich nawyków lub błąd
pub fn get_all_habits(conn: &Connection) -> Result<Vec<Habit>> {
    let sql = "SELECT id, title, habit_type, target_value, current_streak, created_at, updated_at FROM habits ORDER BY created_at DESC";
    let mut stmt = conn.prepare(sql)?;

    let habit_iter = stmt.query_map([], |row| {
        let habit_type_str: String = row.get(2)?;
        let habit_type = match habit_type_str.as_str() {
            "Boolean" => HabitType::Boolean,
            "Counter" => HabitType::Counter,
            _ => HabitType::Boolean, // Default fallback
        };

        Ok(Habit {
            id: row.get(0)?,
            title: row.get(1)?,
            habit_type,
            target_value: row.get(3)?,
            current_streak: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;

    let mut habits = Vec::new();
    for habit in habit_iter {
        habits.push(habit?);
    }

    Ok(habits)
}

/// Pobiera nawyk po ID
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `habit_id` - ID nawyku
///
/// # Returns
/// * `Result<Habit>` - Nawyk lub błąd
pub fn get_habit_by_id(conn: &Connection, habit_id: i32) -> Result<Habit> {
    let sql = "SELECT id, title, habit_type, target_value, current_streak, created_at, updated_at FROM habits WHERE id = ?1";
    let mut stmt = conn.prepare(sql)?;

    let habit = stmt.query_row([habit_id], |row| {
        let habit_type_str: String = row.get(2)?;
        let habit_type = match habit_type_str.as_str() {
            "Boolean" => HabitType::Boolean,
            "Counter" => HabitType::Counter,
            _ => HabitType::Boolean, // Default fallback
        };

        Ok(Habit {
            id: row.get(0)?,
            title: row.get(1)?,
            habit_type,
            target_value: row.get(3)?,
            current_streak: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    })?;

    Ok(habit)
}

/// Usuwa nawyk z bazy danych (wraz z wszystkimi wpisami)
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `habit_id` - ID nawyku do usunięcia
///
/// # Returns
/// * `Result<()>` - Sukces lub błąd
pub fn delete_habit(conn: &Connection, habit_id: i32) -> Result<()> {
    // Najpierw usuń wszystkie wpisy powiązane z nawykiem
    let delete_entries_sql = "DELETE FROM habit_entries WHERE habit_id = ?1";
    conn.execute(delete_entries_sql, [habit_id])?;

    // Następnie usuń sam nawyk
    let delete_habit_sql = "DELETE FROM habits WHERE id = ?1";
    let rows_affected = conn.execute(delete_habit_sql, [habit_id])?;

    if rows_affected == 0 {
        return Err(anyhow::anyhow!("Habit with id {} not found", habit_id));
    }

    Ok(())
}

/// Aktualizuje nawyk
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `habit_id` - ID nawyku do aktualizacji
/// * `request` - Dane do aktualizacji
///
/// # Returns
/// * `Result<Habit>` - Zaktualizowany nawyk lub błąd
pub fn update_habit(
    conn: &Connection,
    habit_id: i32,
    request: UpdateHabitRequest,
) -> Result<Habit> {
    let mut habit = get_habit_by_id(conn, habit_id)?;

    if let Some(title) = request.title {
        // Validate title length
        if title.trim().is_empty() {
            return Err(anyhow::anyhow!("Habit title cannot be empty"));
        }
        if title.len() > 50 {
            return Err(anyhow::anyhow!(
                "Habit title is too long (max 50 characters)"
            ));
        }
        habit.update_title(title);
    }

    if let Some(target_value) = request.target_value {
        habit.update_target_value(Some(target_value));
    }

    let sql = "UPDATE habits SET title = ?1, target_value = ?2, updated_at = ?3 WHERE id = ?4";
    conn.execute(
        sql,
        (&habit.title, habit.target_value, habit.updated_at, habit.id),
    )?;

    Ok(habit)
}

/// Dodaje wpis nawyku na konkretny dzień
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `request` - Dane wpisu nawyku
///
/// # Returns
/// * `Result<HabitEntry>` - Nowo utworzony wpis lub błąd
pub fn add_habit_entry(conn: &Connection, request: CreateHabitEntryRequest) -> Result<HabitEntry> {
    let completed = request.completed.unwrap_or(false);
    let value = request.value.unwrap_or(0);

    let mut entry = HabitEntry::new(request.habit_id, request.date.clone(), completed, value);

    let sql = "INSERT OR REPLACE INTO habit_entries (habit_id, date, completed, value, created_at) VALUES (?1, ?2, ?3, ?4, ?5)";
    conn.execute(
        sql,
        (
            entry.habit_id,
            &entry.date,
            entry.completed,
            entry.value,
            entry.created_at,
        ),
    )?;

    // Pobierz ID wpisu (może być nowy lub istniejący w przypadku REPLACE)
    let id_sql = "SELECT id FROM habit_entries WHERE habit_id = ?1 AND date = ?2";
    entry.id = conn.query_row(id_sql, (entry.habit_id, &entry.date), |row| {
        Ok(row.get::<_, i32>(0)?)
    })?;

    // Przelicz streak dla nawyku
    let new_streak = calculate_streak(conn, request.habit_id)?;
    update_habit_streak(conn, request.habit_id, new_streak)?;

    // Sprawdź czy wpis oznacza ukończenie nawyku na dzisiaj i dodaj EXP
    let habit = get_habit_by_id(conn, request.habit_id)?;
    let is_completed = match habit.habit_type {
        HabitType::Boolean => entry.completed,
        HabitType::Counter => {
            if let Some(target) = habit.target_value {
                entry.value >= target
            } else {
                entry.value > 0
            }
        }
    };

    if is_completed {
        // Przetwórz ukończenie nawyku i dodaj EXP
        match character_service::process_habit_completion(conn, &habit.title, new_streak) {
            Ok((_, level_up)) => {
                if level_up {
                    println!(
                        "Level up! Habit '{}' caused character to level up!",
                        habit.title
                    );
                }
            }
            Err(e) => {
                // Loguj błąd ale nie przerywaj operacji - wpis został już dodany
                eprintln!("Failed to process habit completion for EXP: {}", e);
            }
        }
    }

    Ok(entry)
}

/// Pobiera wpisy nawyków na konkretny dzień
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `date` - Data w formacie YYYY-MM-DD
///
/// # Returns
/// * `Result<Vec<HabitEntry>>` - Lista wpisów na dany dzień lub błąd
pub fn get_habit_entries_for_date(conn: &Connection, date: &str) -> Result<Vec<HabitEntry>> {
    let sql = "SELECT id, habit_id, date, completed, value, created_at FROM habit_entries WHERE date = ?1 ORDER BY created_at ASC";
    let mut stmt = conn.prepare(sql)?;

    let entry_iter = stmt.query_map([date], |row| {
        Ok(HabitEntry {
            id: row.get(0)?,
            habit_id: row.get(1)?,
            date: row.get(2)?,
            completed: row.get(3)?,
            value: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;

    let mut entries = Vec::new();
    for entry in entry_iter {
        entries.push(entry?);
    }

    Ok(entries)
}

/// Pobiera wszystkie wpisy dla konkretnego nawyku
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `habit_id` - ID nawyku
///
/// # Returns
/// * `Result<Vec<HabitEntry>>` - Lista wpisów dla nawyku lub błąd
pub fn get_habit_entries_for_habit(conn: &Connection, habit_id: i32) -> Result<Vec<HabitEntry>> {
    let sql = "SELECT id, habit_id, date, completed, value, created_at FROM habit_entries WHERE habit_id = ?1 ORDER BY date DESC, created_at DESC";
    let mut stmt = conn.prepare(sql)?;

    let entry_iter = stmt.query_map([habit_id], |row| {
        Ok(HabitEntry {
            id: row.get(0)?,
            habit_id: row.get(1)?,
            date: row.get(2)?,
            completed: row.get(3)?,
            value: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;

    let mut entries = Vec::new();
    for entry in entry_iter {
        entries.push(entry?);
    }

    Ok(entries)
}

/// Oblicza aktualny streak dla nawyku
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `habit_id` - ID nawyku
///
/// # Returns
/// * `Result<i32>` - Aktualny streak lub błąd
pub fn calculate_streak(conn: &Connection, habit_id: i32) -> Result<i32> {
    let habit = get_habit_by_id(conn, habit_id)?;
    let entries = get_habit_entries_for_habit(conn, habit_id)?;

    if entries.is_empty() {
        return Ok(0);
    }

    let mut streak = 0;
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let mut current_date = chrono::NaiveDate::parse_from_str(&today, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("Failed to parse date: {}", e))?;

    // Sprawdź ciągłość od dzisiaj wstecz
    loop {
        let date_str = current_date.format("%Y-%m-%d").to_string();

        // Znajdź najnowszy wpis dla tej daty (największy created_at)
        if let Some(entry) = entries
            .iter()
            .filter(|e| e.date == date_str)
            .max_by_key(|e| e.created_at)
        {
            let is_completed = match habit.habit_type {
                HabitType::Boolean => entry.completed,
                HabitType::Counter => {
                    if let Some(target) = habit.target_value {
                        entry.value >= target
                    } else {
                        entry.value > 0
                    }
                }
            };

            if is_completed {
                streak += 1;
                current_date = current_date - chrono::Duration::days(1);
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(streak)
}

/// Aktualizuje streak nawyku w bazie danych
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `habit_id` - ID nawyku
/// * `new_streak` - Nowy streak
///
/// # Returns
/// * `Result<()>` - Sukces lub błąd
fn update_habit_streak(conn: &Connection, habit_id: i32, new_streak: i32) -> Result<()> {
    let sql = "UPDATE habits SET current_streak = ?1, updated_at = ?2 WHERE id = ?3";
    let now = chrono::Utc::now().timestamp();
    conn.execute(sql, (new_streak, now, habit_id))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CreateHabitEntryRequest, CreateHabitRequest, HabitType};
    use rusqlite::{Connection, Result as SqliteResult};

    fn create_test_db() -> SqliteResult<Connection> {
        let conn = Connection::open_in_memory()?;

        // Utwórz tabele habits i habit_entries
        conn.execute(
            "CREATE TABLE habits (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                title TEXT NOT NULL,
                habit_type TEXT NOT NULL CHECK (habit_type IN ('Boolean', 'Counter')),
                target_value INTEGER,
                current_streak INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE habit_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                habit_id INTEGER NOT NULL,
                date TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0,
                value INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL,
                FOREIGN KEY (habit_id) REFERENCES habits(id) ON DELETE CASCADE,
                UNIQUE(habit_id, date)
            )",
            [],
        )?;

        Ok(conn)
    }

    #[test]
    fn test_add_habit() {
        let conn = create_test_db().unwrap();
        let request = CreateHabitRequest {
            title: "Daily meditation".to_string(),
            habit_type: HabitType::Boolean,
            target_value: None,
        };

        let habit = add_habit(&conn, request).unwrap();

        assert_eq!(habit.title, "Daily meditation");
        assert!(matches!(habit.habit_type, HabitType::Boolean));
        assert_eq!(habit.target_value, None);
        assert_eq!(habit.current_streak, 0);
        assert!(habit.id > 0);
    }

    #[test]
    fn test_add_counter_habit() {
        let conn = create_test_db().unwrap();
        let request = CreateHabitRequest {
            title: "Drink water".to_string(),
            habit_type: HabitType::Counter,
            target_value: Some(8),
        };

        let habit = add_habit(&conn, request).unwrap();

        assert_eq!(habit.title, "Drink water");
        assert!(matches!(habit.habit_type, HabitType::Counter));
        assert_eq!(habit.target_value, Some(8));
        assert_eq!(habit.current_streak, 0);
    }

    #[test]
    fn test_get_all_habits() {
        let conn = create_test_db().unwrap();

        let request1 = CreateHabitRequest {
            title: "Meditation".to_string(),
            habit_type: HabitType::Boolean,
            target_value: None,
        };
        let request2 = CreateHabitRequest {
            title: "Water".to_string(),
            habit_type: HabitType::Counter,
            target_value: Some(8),
        };

        add_habit(&conn, request1).unwrap();
        add_habit(&conn, request2).unwrap();

        let habits = get_all_habits(&conn).unwrap();

        assert_eq!(habits.len(), 2);
        let habit_titles: Vec<String> = habits.iter().map(|h| h.title.clone()).collect();
        assert!(habit_titles.contains(&"Meditation".to_string()));
        assert!(habit_titles.contains(&"Water".to_string()));
    }

    #[test]
    fn test_add_habit_entry() {
        let conn = create_test_db().unwrap();

        let habit_request = CreateHabitRequest {
            title: "Test habit".to_string(),
            habit_type: HabitType::Boolean,
            target_value: None,
        };
        let habit = add_habit(&conn, habit_request).unwrap();

        let entry_request = CreateHabitEntryRequest {
            habit_id: habit.id,
            date: "2025-01-20".to_string(),
            completed: Some(true),
            value: None,
        };

        let entry = add_habit_entry(&conn, entry_request).unwrap();

        assert_eq!(entry.habit_id, habit.id);
        assert_eq!(entry.date, "2025-01-20");
        assert!(entry.completed);
        assert_eq!(entry.value, 0);
    }
}
