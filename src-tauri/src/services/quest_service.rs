use crate::models::{
    CreateQuestRequest, Habit, HabitEntry, Quest, QuestStatus, QuestType, Task, UpdateQuestRequest,
};
use crate::services::character_service;
use anyhow::Result;
use chrono::{Datelike, IsoWeek, Utc};
use rusqlite::Connection;

/// Pobiera aktualny tydzień w formacie YYYY-WW
fn get_current_week() -> String {
    let now = Utc::now();
    let iso_week = now.iso_week();
    format!("{}-{:02}", iso_week.year(), iso_week.week())
}

/// Generuje questy tygodniowe na podstawie danych użytkownika
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
///
/// # Returns
/// * `Result<Vec<Quest>>` - Lista nowo wygenerowanych questów lub błąd
pub fn generate_weekly_quests(conn: &Connection) -> Result<Vec<Quest>> {
    let current_week = get_current_week();

    // Sprawdź czy questy na ten tydzień już istnieją
    let existing_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM quests WHERE week = ?1",
        [&current_week],
        |row| row.get(0),
    )?;

    if existing_count > 0 {
        return Ok(Vec::new()); // Questy już wygenerowane
    }

    let mut new_quests = Vec::new();
    let now = Utc::now().timestamp();

    // Quest 1: Ukończ zadania (na podstawie nieukończonych zadań)
    let incomplete_tasks_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE completed = 0",
        [],
        |row| row.get(0),
    )?;

    if incomplete_tasks_count > 0 {
        let target = std::cmp::min(5, incomplete_tasks_count); // Max 5 zadań
        let quest = Quest::new(
            "Tygodniowy Wykonawca".to_string(),
            format!("Ukończ {} zadań w tym tygodniu", target),
            QuestType::Task,
            target,
            None,
            None,
            50,                           // 50 EXP nagrody
            Some(now + 7 * 24 * 60 * 60), // Deadline za tydzień
            current_week.clone(),
        );
        new_quests.push(quest);
    }

    // Quest 2: Utrzymaj najdłuższy streak nawyku
    let longest_habit_result: Result<(i32, String, i32), rusqlite::Error> = conn.query_row(
        "SELECT id, title, current_streak FROM habits ORDER BY current_streak DESC LIMIT 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    );

    if let Ok((habit_id, habit_title, current_streak)) = longest_habit_result {
        if current_streak >= 3 {
            // Tylko jeśli streak jest przynajmniej 3 dni
            let quest = Quest::new(
                "Mistrz Konsekwencji".to_string(),
                format!(
                    "Utrzymaj nawyk '{}' przez cały tydzień (7 dni z rzędu)",
                    habit_title
                ),
                QuestType::Habit,
                7,
                None,
                Some(habit_id),
                75, // 75 EXP nagrody
                Some(now + 7 * 24 * 60 * 60),
                current_week.clone(),
            );
            new_quests.push(quest);
        }
    }

    // Quest 3: Zdobądź EXP (quest związany z postacią)
    let character_result: Result<i32, rusqlite::Error> =
        conn.query_row("SELECT level FROM characters WHERE id = 1", [], |row| {
            row.get(0)
        });

    if let Ok(level) = character_result {
        let target_exp = 100 + (level * 25); // Więcej EXP dla wyższych poziomów
        let quest = Quest::new(
            "Tygodniowy Rozwój".to_string(),
            format!(
                "Zdobądź {} punktów doświadczenia w tym tygodniu",
                target_exp
            ),
            QuestType::Character,
            target_exp,
            None,
            None,
            100, // 100 EXP nagrody (bonusowe)
            Some(now + 7 * 24 * 60 * 60),
            current_week.clone(),
        );
        new_quests.push(quest);
    }

    // Quest 4: Dodatkowy quest dla konkretnej kategorii (jeśli istnieją zadania z kategoriami)
    let category_result: Result<String, rusqlite::Error> = conn.query_row(
        "SELECT title FROM tasks WHERE completed = 0 AND (
            LOWER(title) LIKE '%nauka%' OR 
            LOWER(title) LIKE '%sport%' OR 
            LOWER(title) LIKE '%praca%' OR 
            LOWER(title) LIKE '%projekt%'
        ) LIMIT 1",
        [],
        |row| row.get(0),
    );

    if let Ok(task_title) = category_result {
        let category = if task_title.to_lowercase().contains("nauka") {
            "nauka"
        } else if task_title.to_lowercase().contains("sport") {
            "sport"
        } else if task_title.to_lowercase().contains("praca") {
            "praca"
        } else {
            "projekt"
        };

        let quest = Quest::new(
            format!("Specjalista: {}", category.to_uppercase()),
            format!("Ukończ 3 zadania związane z kategorią '{}'", category),
            QuestType::Task,
            3,
            Some(category.to_string()),
            None,
            60, // 60 EXP nagrody
            Some(now + 7 * 24 * 60 * 60),
            current_week.clone(),
        );
        new_quests.push(quest);
    }

    // Zapisz questy do bazy danych
    for quest in &new_quests {
        let sql = "INSERT INTO quests (
            title, description, quest_type, target_value, current_progress,
            category, habit_id, status, reward_exp, deadline, week, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)";

        conn.execute(
            sql,
            (
                &quest.title,
                &quest.description,
                match quest.quest_type {
                    QuestType::Task => "Task",
                    QuestType::Habit => "Habit",
                    QuestType::Character => "Character",
                },
                quest.target_value,
                quest.current_progress,
                &quest.category,
                quest.habit_id,
                match quest.status {
                    QuestStatus::Active => "Active",
                    QuestStatus::Completed => "Completed",
                    QuestStatus::Expired => "Expired",
                },
                quest.reward_exp,
                quest.deadline,
                &quest.week,
                quest.created_at,
                quest.updated_at,
            ),
        )?;
    }

    Ok(new_quests)
}

/// Pobiera wszystkie questy dla danego tygodnia
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `week` - Tydzień w formacie YYYY-WW (opcjonalny, domyślnie aktualny)
///
/// # Returns
/// * `Result<Vec<Quest>>` - Lista questów lub błąd
pub fn get_quests_for_week(conn: &Connection, week: Option<String>) -> Result<Vec<Quest>> {
    let target_week = week.unwrap_or_else(get_current_week);

    let sql = "SELECT id, title, description, quest_type, target_value, current_progress,
                      category, habit_id, status, reward_exp, deadline, week, created_at, updated_at
               FROM quests WHERE week = ?1 ORDER BY created_at ASC";

    let mut stmt = conn.prepare(sql)?;
    let quest_iter = stmt.query_map([target_week], |row| {
        let quest_type = match row.get::<_, String>(3)?.as_str() {
            "Task" => QuestType::Task,
            "Habit" => QuestType::Habit,
            "Character" => QuestType::Character,
            _ => QuestType::Task,
        };

        let status = match row.get::<_, String>(8)?.as_str() {
            "Active" => QuestStatus::Active,
            "Completed" => QuestStatus::Completed,
            "Expired" => QuestStatus::Expired,
            _ => QuestStatus::Active,
        };

        Ok(Quest {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            quest_type,
            target_value: row.get(4)?,
            current_progress: row.get(5)?,
            category: row.get(6)?,
            habit_id: row.get(7)?,
            status,
            reward_exp: row.get(9)?,
            deadline: row.get(10)?,
            week: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
        })
    })?;

    let mut quests = Vec::new();
    for quest in quest_iter {
        quests.push(quest?);
    }

    Ok(quests)
}

/// Aktualizuje postęp wszystkich aktywnych questów na podstawie aktualnych danych
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
///
/// # Returns
/// * `Result<Vec<Quest>>` - Lista zaktualizowanych questów lub błąd
pub fn update_all_quest_progress(conn: &Connection) -> Result<Vec<Quest>> {
    let current_week = get_current_week();
    let mut updated_quests = Vec::new();

    // Pobierz wszystkie aktywne questy dla obecnego tygodnia
    let active_quests = get_quests_for_week(conn, Some(current_week))?
        .into_iter()
        .filter(|q| matches!(q.status, QuestStatus::Active))
        .collect::<Vec<_>>();

    for quest in active_quests {
        let new_progress = match quest.quest_type {
            QuestType::Task => {
                if let Some(category) = &quest.category {
                    // Zlicz ukończone zadania z określonej kategorii w tym tygodniu
                    let week_start = Utc::now().timestamp() - (7 * 24 * 60 * 60);
                    let category_pattern = format!("%{}%", category.to_lowercase());
                    conn.query_row(
                        "SELECT COUNT(*) FROM tasks WHERE completed = 1 AND updated_at >= ?1 AND LOWER(title) LIKE ?2",
                        (week_start, category_pattern),
                        |row| row.get::<_, i32>(0)
                    ).unwrap_or(0)
                } else {
                    // Zlicz wszystkie ukończone zadania w tym tygodniu
                    let week_start = Utc::now().timestamp() - (7 * 24 * 60 * 60);
                    conn.query_row(
                        "SELECT COUNT(*) FROM tasks WHERE completed = 1 AND updated_at >= ?1",
                        [week_start],
                        |row| row.get::<_, i32>(0),
                    )
                    .unwrap_or(0)
                }
            }
            QuestType::Habit => {
                if let Some(habit_id) = quest.habit_id {
                    // Sprawdź streak dla konkretnego nawyku
                    conn.query_row(
                        "SELECT current_streak FROM habits WHERE id = ?1",
                        [habit_id],
                        |row| row.get::<_, i32>(0),
                    )
                    .unwrap_or(0)
                } else {
                    0
                }
            }
            QuestType::Character => {
                // Sprawdź aktualny EXP postaci dla questów EXP
                match crate::services::character_service::get_character(conn) {
                    Ok(character) => character.experience as i32,
                    Err(_) => 0, // Fallback jeśli nie można pobrać postaci
                }
            }
        };

        // Aktualizuj quest jeśli progress się zmienił
        if new_progress != quest.current_progress {
            let mut updated_quest = quest.clone();
            updated_quest.update_progress(new_progress);

            // Zapisz do bazy danych
            let sql = "UPDATE quests SET current_progress = ?1, status = ?2, updated_at = ?3 WHERE id = ?4";
            conn.execute(
                sql,
                (
                    updated_quest.current_progress,
                    match updated_quest.status {
                        QuestStatus::Active => "Active",
                        QuestStatus::Completed => "Completed",
                        QuestStatus::Expired => "Expired",
                    },
                    updated_quest.updated_at,
                    updated_quest.id,
                ),
            )?;

            // Jeśli quest został ukończony, daj nagrodę EXP
            if updated_quest.is_completed()
                && matches!(updated_quest.status, QuestStatus::Completed)
            {
                match character_service::add_experience(conn, updated_quest.reward_exp) {
                    Ok((_, level_up)) => {
                        if level_up {
                            println!(
                                "Level up! Quest '{}' caused character to level up!",
                                updated_quest.title
                            );
                        }
                        println!(
                            "Quest completed: '{}' - {} EXP earned!",
                            updated_quest.title, updated_quest.reward_exp
                        );
                    }
                    Err(e) => {
                        eprintln!("Failed to award quest EXP: {}", e);
                    }
                }
            }

            updated_quests.push(updated_quest);
        }
    }

    Ok(updated_quests)
}

/// Oznacza przeterminowane questy jako expired
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
///
/// # Returns
/// * `Result<i32>` - Liczba questów oznaczonych jako expired lub błąd
pub fn expire_overdue_quests(conn: &Connection) -> Result<i32> {
    let now = Utc::now().timestamp();

    let sql = "UPDATE quests SET status = 'Expired', updated_at = ?1 
               WHERE status = 'Active' AND deadline IS NOT NULL AND deadline < ?2";

    let rows_affected = conn.execute(sql, [now, now])?;

    Ok(rows_affected as i32)
}

/// Pobiera wszystkie aktywne questy
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
///
/// # Returns
/// * `Result<Vec<Quest>>` - Lista aktywnych questów lub błąd
pub fn get_active_quests(conn: &Connection) -> Result<Vec<Quest>> {
    let sql = "SELECT id, title, description, quest_type, target_value, current_progress,
                      category, habit_id, status, reward_exp, deadline, week, created_at, updated_at
               FROM quests WHERE status = 'Active' ORDER BY deadline ASC";

    let mut stmt = conn.prepare(sql)?;
    let quest_iter = stmt.query_map([], |row| {
        let quest_type = match row.get::<_, String>(3)?.as_str() {
            "Task" => QuestType::Task,
            "Habit" => QuestType::Habit,
            "Character" => QuestType::Character,
            _ => QuestType::Task,
        };

        let status = match row.get::<_, String>(8)?.as_str() {
            "Active" => QuestStatus::Active,
            "Completed" => QuestStatus::Completed,
            "Expired" => QuestStatus::Expired,
            _ => QuestStatus::Active,
        };

        Ok(Quest {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            quest_type,
            target_value: row.get(4)?,
            current_progress: row.get(5)?,
            category: row.get(6)?,
            habit_id: row.get(7)?,
            status,
            reward_exp: row.get(9)?,
            deadline: row.get(10)?,
            week: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
        })
    })?;

    let mut quests = Vec::new();
    for quest in quest_iter {
        quests.push(quest?);
    }

    Ok(quests)
}

/// Ręcznie oznacza quest jako ukończony (np. przez użytkownika)
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `quest_id` - ID questu do ukończenia
///
/// # Returns
/// * `Result<Quest>` - Zaktualizowany quest lub błąd
pub fn complete_quest(conn: &Connection, quest_id: i32) -> Result<Quest> {
    // Pobierz quest
    let sql = "SELECT id, title, description, quest_type, target_value, current_progress,
                      category, habit_id, status, reward_exp, deadline, week, created_at, updated_at
               FROM quests WHERE id = ?1";

    let mut stmt = conn.prepare(sql)?;
    let mut quest = stmt.query_row([quest_id], |row| {
        let quest_type = match row.get::<_, String>(3)?.as_str() {
            "Task" => QuestType::Task,
            "Habit" => QuestType::Habit,
            "Character" => QuestType::Character,
            _ => QuestType::Task,
        };

        let status = match row.get::<_, String>(8)?.as_str() {
            "Active" => QuestStatus::Active,
            "Completed" => QuestStatus::Completed,
            "Expired" => QuestStatus::Expired,
            _ => QuestStatus::Active,
        };

        Ok(Quest {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            quest_type,
            target_value: row.get(4)?,
            current_progress: row.get(5)?,
            category: row.get(6)?,
            habit_id: row.get(7)?,
            status,
            reward_exp: row.get(9)?,
            deadline: row.get(10)?,
            week: row.get(11)?,
            created_at: row.get(12)?,
            updated_at: row.get(13)?,
        })
    })?;

    if !matches!(quest.status, QuestStatus::Active) {
        return Err(anyhow::anyhow!("Quest is not active"));
    }

    // Oznacz jako ukończony
    quest.current_progress = quest.target_value;
    quest.status = QuestStatus::Completed;
    quest.updated_at = Utc::now().timestamp();

    // Aktualizuj w bazie danych
    let update_sql =
        "UPDATE quests SET current_progress = ?1, status = ?2, updated_at = ?3 WHERE id = ?4";
    conn.execute(
        update_sql,
        (
            quest.current_progress,
            "Completed",
            quest.updated_at,
            quest.id,
        ),
    )?;

    // Daj nagrodę EXP
    match character_service::add_experience(conn, quest.reward_exp) {
        Ok((_, level_up)) => {
            if level_up {
                println!(
                    "Level up! Quest '{}' caused character to level up!",
                    quest.title
                );
            }
            println!(
                "Quest completed: '{}' - {} EXP earned!",
                quest.title, quest.reward_exp
            );
        }
        Err(e) => {
            eprintln!("Failed to award quest EXP: {}", e);
        }
    }

    Ok(quest)
}
