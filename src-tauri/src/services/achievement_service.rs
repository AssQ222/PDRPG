use crate::models::{Achievement, AchievementStatus, AchievementType, CreateAchievementRequest};
use crate::services::character_service;
use anyhow::Result;
use rusqlite::Connection;

/// Pobiera wszystkie odznaki z bazy danych
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
///
/// # Returns
/// * `Result<Vec<Achievement>>` - Lista wszystkich odznak lub błąd
pub fn get_all_achievements(conn: &Connection) -> Result<Vec<Achievement>> {
    let sql = "SELECT id, name, description, achievement_type, required_value, icon, status, earned_at, created_at, updated_at 
               FROM achievements ORDER BY required_value ASC, id ASC";

    let mut stmt = conn.prepare(sql)?;
    let achievement_iter = stmt.query_map([], |row| {
        let achievement_type = match row.get::<_, String>(3)?.as_str() {
            "HabitStreak" => AchievementType::HabitStreak,
            "TaskCount" => AchievementType::TaskCount,
            "CharacterLevel" => AchievementType::CharacterLevel,
            "QuestCount" => AchievementType::QuestCount,
            _ => AchievementType::HabitStreak,
        };

        let status = match row.get::<_, String>(6)?.as_str() {
            "Locked" => AchievementStatus::Locked,
            "Available" => AchievementStatus::Available,
            "Earned" => AchievementStatus::Earned,
            _ => AchievementStatus::Locked,
        };

        Ok(Achievement {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            achievement_type,
            required_value: row.get(4)?,
            icon: row.get(5)?,
            status,
            earned_at: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    let mut achievements = Vec::new();
    for achievement in achievement_iter {
        achievements.push(achievement?);
    }

    Ok(achievements)
}

/// Pobiera odznaki o określonym statusie
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `status` - Status odznak do pobrania
///
/// # Returns
/// * `Result<Vec<Achievement>>` - Lista odznak o danym statusie lub błąd
pub fn get_achievements_by_status(
    conn: &Connection,
    status: AchievementStatus,
) -> Result<Vec<Achievement>> {
    let status_str = match status {
        AchievementStatus::Locked => "Locked",
        AchievementStatus::Available => "Available",
        AchievementStatus::Earned => "Earned",
    };

    let sql = "SELECT id, name, description, achievement_type, required_value, icon, status, earned_at, created_at, updated_at 
               FROM achievements WHERE status = ?1 ORDER BY required_value ASC, id ASC";

    let mut stmt = conn.prepare(sql)?;
    let achievement_iter = stmt.query_map([status_str], |row| {
        let achievement_type = match row.get::<_, String>(3)?.as_str() {
            "HabitStreak" => AchievementType::HabitStreak,
            "TaskCount" => AchievementType::TaskCount,
            "CharacterLevel" => AchievementType::CharacterLevel,
            "QuestCount" => AchievementType::QuestCount,
            _ => AchievementType::HabitStreak,
        };

        let status = match row.get::<_, String>(6)?.as_str() {
            "Locked" => AchievementStatus::Locked,
            "Available" => AchievementStatus::Available,
            "Earned" => AchievementStatus::Earned,
            _ => AchievementStatus::Locked,
        };

        Ok(Achievement {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            achievement_type,
            required_value: row.get(4)?,
            icon: row.get(5)?,
            status,
            earned_at: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    let mut achievements = Vec::new();
    for achievement in achievement_iter {
        achievements.push(achievement?);
    }

    Ok(achievements)
}

/// Sprawdza i aktualizuje wszystkie odznaki na podstawie aktualnych danych użytkownika
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
///
/// # Returns
/// * `Result<Vec<Achievement>>` - Lista nowo dostępnych/zdobytych odznak lub błąd
pub fn check_and_update_achievements(conn: &Connection) -> Result<Vec<Achievement>> {
    let mut updated_achievements = Vec::new();

    // Pobierz wszystkie locked i available achievements
    let locked_achievements = get_achievements_by_status(conn, AchievementStatus::Locked)?;
    let available_achievements = get_achievements_by_status(conn, AchievementStatus::Available)?;

    // Sprawdź locked achievements czy mogą być available
    for achievement in locked_achievements {
        if check_achievement_requirements(conn, &achievement)? {
            let mut updated = achievement.clone();
            updated.make_available();

            // Aktualizuj w bazie danych
            update_achievement_status(conn, updated.id, AchievementStatus::Available)?;
            updated_achievements.push(updated);
        }
    }

    // Sprawdź available achievements czy mogą być earned (automatyczne earning)
    for achievement in available_achievements {
        if check_achievement_requirements(conn, &achievement)? {
            let mut updated = achievement.clone();
            updated.mark_earned();

            // Aktualizuj w bazie danych
            update_achievement_status(conn, updated.id, AchievementStatus::Earned)?;

            // Dodaj bonus EXP za zdobycie odznaki
            let bonus_exp = calculate_achievement_bonus_exp(&updated);
            match character_service::add_experience(conn, bonus_exp) {
                Ok((_, level_up)) => {
                    if level_up {
                        println!(
                            "Level up! Achievement '{}' caused character to level up!",
                            updated.name
                        );
                    }
                    println!(
                        "Achievement earned: '{}' - {} bonus EXP!",
                        updated.name, bonus_exp
                    );
                }
                Err(e) => {
                    eprintln!("Failed to award achievement bonus EXP: {}", e);
                }
            }

            updated_achievements.push(updated);
        }
    }

    Ok(updated_achievements)
}

/// Sprawdza czy wymagania dla danej odznaki są spełnione
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `achievement` - Odznaka do sprawdzenia
///
/// # Returns
/// * `Result<bool>` - True jeśli wymagania są spełnione, false w przeciwnym razie
fn check_achievement_requirements(conn: &Connection, achievement: &Achievement) -> Result<bool> {
    match achievement.achievement_type {
        AchievementType::HabitStreak => {
            // Sprawdź najdłuższy streak nawyków
            let max_streak: i32 = conn
                .query_row(
                    "SELECT COALESCE(MAX(current_streak), 0) FROM habits",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            Ok(max_streak >= achievement.required_value)
        }
        AchievementType::TaskCount => {
            // Sprawdź liczbę ukończonych zadań
            let completed_count: i32 = conn
                .query_row(
                    "SELECT COUNT(*) FROM tasks WHERE completed = 1",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            Ok(completed_count >= achievement.required_value)
        }
        AchievementType::CharacterLevel => {
            // Sprawdź poziom postaci
            let character_level: i32 = conn
                .query_row("SELECT level FROM characters WHERE id = 1", [], |row| {
                    row.get(0)
                })
                .unwrap_or(1);

            Ok(character_level >= achievement.required_value)
        }
        AchievementType::QuestCount => {
            // Sprawdź liczbę ukończonych questów
            let completed_quests: i32 = conn
                .query_row(
                    "SELECT COUNT(*) FROM quests WHERE status = 'Completed'",
                    [],
                    |row| row.get(0),
                )
                .unwrap_or(0);

            Ok(completed_quests >= achievement.required_value)
        }
    }
}

/// Oblicza bonus EXP za zdobycie odznaki
///
/// # Arguments
/// * `achievement` - Odznaka do obliczenia bonusu
///
/// # Returns
/// * `i64` - Liczba punktów EXP do przyznania
fn calculate_achievement_bonus_exp(achievement: &Achievement) -> i64 {
    match achievement.achievement_type {
        AchievementType::HabitStreak => {
            // Bonusy za streaki: 7 dni = 25 EXP, 30 dni = 100 EXP, 90 dni = 300 EXP, etc.
            match achievement.required_value {
                7 => 25,
                30 => 100,
                90 => 300,
                150 => 500,
                230 => 1000,
                _ => achievement.required_value as i64 * 5, // Podstawowa formuła: 5 EXP na dzień
            }
        }
        AchievementType::TaskCount => {
            // Bonusy za zadania: 10 zadań = 50 EXP, 50 zadań = 200 EXP, 100 zadań = 500 EXP
            match achievement.required_value {
                10 => 50,
                50 => 200,
                100 => 500,
                _ => achievement.required_value as i64 * 2, // 2 EXP za zadanie
            }
        }
        AchievementType::CharacterLevel => {
            // Bonusy za poziomy: poziom 5 = 150 EXP, poziom 10 = 400 EXP
            match achievement.required_value {
                5 => 150,
                10 => 400,
                _ => achievement.required_value as i64 * 50, // 50 EXP za poziom
            }
        }
        AchievementType::QuestCount => {
            // Bonusy za questy
            achievement.required_value as i64 * 25 // 25 EXP za quest
        }
    }
}

/// Aktualizuje status odznaki w bazie danych
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `achievement_id` - ID odznaki
/// * `new_status` - Nowy status
///
/// # Returns
/// * `Result<()>` - Sukces lub błąd
fn update_achievement_status(
    conn: &Connection,
    achievement_id: i32,
    new_status: AchievementStatus,
) -> Result<()> {
    let status_str = match new_status {
        AchievementStatus::Locked => "Locked",
        AchievementStatus::Available => "Available",
        AchievementStatus::Earned => "Earned",
    };

    let now = chrono::Utc::now().timestamp();

    let sql = if matches!(new_status, AchievementStatus::Earned) {
        "UPDATE achievements SET status = ?1, earned_at = ?2, updated_at = ?3 WHERE id = ?4"
    } else {
        "UPDATE achievements SET status = ?1, updated_at = ?2 WHERE id = ?3 AND ?4 = ?4"
        // Dummy parameter for consistency
    };

    if matches!(new_status, AchievementStatus::Earned) {
        conn.execute(sql, (status_str, now, now, achievement_id))?;
    } else {
        conn.execute(
            "UPDATE achievements SET status = ?1, updated_at = ?2 WHERE id = ?3",
            (status_str, now, achievement_id),
        )?;
    }

    Ok(())
}

/// Ręcznie oznacza odznakę jako zdobytą (dla available achievements)
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `achievement_id` - ID odznaki do zdobycia
///
/// # Returns
/// * `Result<Achievement>` - Zaktualizowana odznaka lub błąd
pub fn earn_achievement(conn: &Connection, achievement_id: i32) -> Result<Achievement> {
    // Pobierz achievement
    let sql = "SELECT id, name, description, achievement_type, required_value, icon, status, earned_at, created_at, updated_at 
               FROM achievements WHERE id = ?1";

    let mut stmt = conn.prepare(sql)?;
    let mut achievement = stmt.query_row([achievement_id], |row| {
        let achievement_type = match row.get::<_, String>(3)?.as_str() {
            "HabitStreak" => AchievementType::HabitStreak,
            "TaskCount" => AchievementType::TaskCount,
            "CharacterLevel" => AchievementType::CharacterLevel,
            "QuestCount" => AchievementType::QuestCount,
            _ => AchievementType::HabitStreak,
        };

        let status = match row.get::<_, String>(6)?.as_str() {
            "Locked" => AchievementStatus::Locked,
            "Available" => AchievementStatus::Available,
            "Earned" => AchievementStatus::Earned,
            _ => AchievementStatus::Locked,
        };

        Ok(Achievement {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            achievement_type,
            required_value: row.get(4)?,
            icon: row.get(5)?,
            status,
            earned_at: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    })?;

    if !matches!(achievement.status, AchievementStatus::Available) {
        return Err(anyhow::anyhow!("Achievement is not available to earn"));
    }

    // Sprawdź czy wymagania są nadal spełnione
    if !check_achievement_requirements(conn, &achievement)? {
        return Err(anyhow::anyhow!(
            "Achievement requirements are no longer met"
        ));
    }

    // Oznacz jako zdobytą
    achievement.mark_earned();

    // Aktualizuj w bazie danych
    update_achievement_status(conn, achievement.id, AchievementStatus::Earned)?;

    // Daj bonus EXP
    let bonus_exp = calculate_achievement_bonus_exp(&achievement);
    match character_service::add_experience(conn, bonus_exp) {
        Ok((_, level_up)) => {
            if level_up {
                println!(
                    "Level up! Achievement '{}' caused character to level up!",
                    achievement.name
                );
            }
            println!(
                "Achievement earned: '{}' - {} bonus EXP!",
                achievement.name, bonus_exp
            );
        }
        Err(e) => {
            eprintln!("Failed to award achievement bonus EXP: {}", e);
        }
    }

    Ok(achievement)
}

/// Dodaje nową odznakę do systemu
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
/// * `request` - Dane nowej odznaki
///
/// # Returns
/// * `Result<Achievement>` - Nowo utworzona odznaka lub błąd
pub fn add_achievement(
    conn: &Connection,
    request: CreateAchievementRequest,
) -> Result<Achievement> {
    let achievement = Achievement::new(
        request.name,
        request.description,
        request.achievement_type,
        request.required_value,
        request.icon,
    );

    let sql = "INSERT INTO achievements (name, description, achievement_type, required_value, icon, status, created_at, updated_at)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)";

    conn.execute(
        sql,
        (
            &achievement.name,
            &achievement.description,
            match achievement.achievement_type {
                AchievementType::HabitStreak => "HabitStreak",
                AchievementType::TaskCount => "TaskCount",
                AchievementType::CharacterLevel => "CharacterLevel",
                AchievementType::QuestCount => "QuestCount",
            },
            achievement.required_value,
            &achievement.icon,
            match achievement.status {
                AchievementStatus::Locked => "Locked",
                AchievementStatus::Available => "Available",
                AchievementStatus::Earned => "Earned",
            },
            achievement.created_at,
            achievement.updated_at,
        ),
    )?;

    // Pobierz ID nowo utworzonej odznaki
    let mut new_achievement = achievement;
    new_achievement.id = conn.last_insert_rowid() as i32;

    Ok(new_achievement)
}

/// Pobiera statystyki odznak (ile zdobytych, dostępnych, zablokowanych)
///
/// # Arguments
/// * `conn` - Referencja do połączenia z bazą danych
///
/// # Returns
/// * `Result<(i32, i32, i32)>` - (earned, available, locked) lub błąd
pub fn get_achievement_stats(conn: &Connection) -> Result<(i32, i32, i32)> {
    let earned: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM achievements WHERE status = 'Earned'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let available: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM achievements WHERE status = 'Available'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let locked: i32 = conn
        .query_row(
            "SELECT COUNT(*) FROM achievements WHERE status = 'Locked'",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    Ok((earned, available, locked))
}
