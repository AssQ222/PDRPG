use anyhow::Result;
use rusqlite::{params, Connection, Row};

use crate::models::{
    Character, CharacterAttributes, CharacterClass, CreateCharacterRequest, UpdateCharacterRequest,
};

/// Konwertuje wiersz bazy danych na obiekt Character
fn row_to_character(row: &Row) -> Result<Character, rusqlite::Error> {
    let class_str: String = row.get("character_class")?;
    let character_class = match class_str.as_str() {
        "Warrior" => CharacterClass::Warrior,
        "Mage" => CharacterClass::Mage,
        "Bard" => CharacterClass::Bard,
        "Rogue" => CharacterClass::Rogue,
        _ => CharacterClass::Warrior, // fallback
    };

    let attributes = CharacterAttributes {
        strength: row.get("strength")?,
        intelligence: row.get("intelligence")?,
        charisma: row.get("charisma")?,
        dexterity: row.get("dexterity")?,
        wisdom: row.get("wisdom")?,
        constitution: row.get("constitution")?,
    };

    Ok(Character {
        id: row.get("id")?,
        level: row.get("level")?,
        experience: row.get("experience")?,
        character_class,
        attributes,
        created_at: row.get("created_at")?,
        updated_at: row.get("updated_at")?,
    })
}

/// Pobiera postać gracza (zawsze ID = 1)
pub fn get_character(connection: &Connection) -> Result<Character> {
    let mut stmt = connection.prepare(
        "SELECT id, level, experience, character_class, 
                strength, intelligence, charisma, dexterity, wisdom, constitution,
                created_at, updated_at 
         FROM characters WHERE id = 1",
    )?;

    let character = stmt.query_row([], |row| row_to_character(row))?;
    Ok(character)
}

/// Tworzy nową postać (jeśli nie istnieje)
pub fn create_character(
    connection: &Connection,
    request: CreateCharacterRequest,
) -> Result<Character> {
    let character = Character::new(request.character_class.clone());

    let class_str = match request.character_class {
        CharacterClass::Warrior => "Warrior",
        CharacterClass::Mage => "Mage",
        CharacterClass::Bard => "Bard",
        CharacterClass::Rogue => "Rogue",
    };

    connection.execute(
        "INSERT OR REPLACE INTO characters 
         (id, level, experience, character_class, 
          strength, intelligence, charisma, dexterity, wisdom, constitution,
          created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            character.id,
            character.level,
            character.experience,
            class_str,
            character.attributes.strength,
            character.attributes.intelligence,
            character.attributes.charisma,
            character.attributes.dexterity,
            character.attributes.wisdom,
            character.attributes.constitution,
            character.created_at,
            character.updated_at
        ],
    )?;

    get_character(connection)
}

/// Aktualizuje postać
pub fn update_character(
    connection: &Connection,
    request: UpdateCharacterRequest,
) -> Result<Character> {
    if let Some(character_class) = request.character_class {
        let class_str = match character_class {
            CharacterClass::Warrior => "Warrior",
            CharacterClass::Mage => "Mage",
            CharacterClass::Bard => "Bard",
            CharacterClass::Rogue => "Rogue",
        };

        let now = chrono::Utc::now().timestamp();

        connection.execute(
            "UPDATE characters SET character_class = ?1, updated_at = ?2 WHERE id = 1",
            params![class_str, now],
        )?;
    }

    get_character(connection)
}

/// Dodaje punkty doświadczenia do postaci
///
/// # Arguments
/// * `connection` - Połączenie z bazą danych
/// * `exp_points` - Ilość punktów doświadczenia do dodania
///
/// # Returns
/// * `Result<(Character, bool)>` - Aktualna postać i informacja czy nastąpił awans poziomu
pub fn add_experience(connection: &Connection, exp_points: i64) -> Result<(Character, bool)> {
    let mut character = get_character(connection)?;
    let level_up = character.add_experience(exp_points);

    // Aktualizuj bazę danych
    connection.execute(
        "UPDATE characters SET level = ?1, experience = ?2, updated_at = ?3 WHERE id = 1",
        params![character.level, character.experience, character.updated_at],
    )?;

    Ok((character, level_up))
}

/// Dodaje punkty do atrybutu postaci
///
/// # Arguments
/// * `connection` - Połączenie z bazą danych
/// * `attribute` - Nazwa atrybutu (strength, intelligence, charisma, dexterity, wisdom, constitution)
/// * `points` - Ilość punktów do dodania
pub fn add_attribute_points(
    connection: &Connection,
    attribute: &str,
    points: i32,
) -> Result<Character> {
    let mut character = get_character(connection)?;
    character.add_attribute_points(attribute, points);

    // Aktualizuj bazę danych
    let sql = match attribute {
        "strength" => "UPDATE characters SET strength = ?1, updated_at = ?2 WHERE id = 1",
        "intelligence" => "UPDATE characters SET intelligence = ?1, updated_at = ?2 WHERE id = 1",
        "charisma" => "UPDATE characters SET charisma = ?1, updated_at = ?2 WHERE id = 1",
        "dexterity" => "UPDATE characters SET dexterity = ?1, updated_at = ?2 WHERE id = 1",
        "wisdom" => "UPDATE characters SET wisdom = ?1, updated_at = ?2 WHERE id = 1",
        "constitution" => "UPDATE characters SET constitution = ?1, updated_at = ?2 WHERE id = 1",
        _ => return Err(anyhow::anyhow!("Invalid attribute name: {}", attribute)),
    };

    let attribute_value = match attribute {
        "strength" => character.attributes.strength,
        "intelligence" => character.attributes.intelligence,
        "charisma" => character.attributes.charisma,
        "dexterity" => character.attributes.dexterity,
        "wisdom" => character.attributes.wisdom,
        "constitution" => character.attributes.constitution,
        _ => return Err(anyhow::anyhow!("Invalid attribute name: {}", attribute)),
    };

    connection.execute(sql, params![attribute_value, character.updated_at])?;

    Ok(character)
}

/// Oblicza punkty EXP za ukończenie zadania
///
/// # Arguments
/// * `task_title` - Tytuł zadania (może zawierać tagi klas)
/// * `is_goal_related` - Czy zadanie jest powiązane z celem
///
/// # Returns
/// * `(i64, Option<String>)` - Punkty EXP i opcjonalny atrybut do zwiększenia
pub fn calculate_task_exp(task_title: &str, is_goal_related: bool) -> (i64, Option<String>) {
    let base_exp = if is_goal_related { 25 } else { 15 };
    let title_lower = task_title.to_lowercase();

    // Analiza tagów/słów kluczowych w tytule zadania
    let attribute = if title_lower.contains("sport")
        || title_lower.contains("trening")
        || title_lower.contains("ćwiczenia")
        || title_lower.contains("fitness")
    {
        Some("strength".to_string())
    } else if title_lower.contains("nauka")
        || title_lower.contains("książka")
        || title_lower.contains("kurs")
        || title_lower.contains("czytanie")
    {
        Some("intelligence".to_string())
    } else if title_lower.contains("prezentacja")
        || title_lower.contains("spotkanie")
        || title_lower.contains("kontakt")
        || title_lower.contains("rozmowa")
    {
        Some("charisma".to_string())
    } else if title_lower.contains("hobby")
        || title_lower.contains("praktyka")
        || title_lower.contains("umiejętność")
        || title_lower.contains("projekt")
    {
        Some("dexterity".to_string())
    } else if title_lower.contains("medytacja")
        || title_lower.contains("refleksja")
        || title_lower.contains("mindfulness")
        || title_lower.contains("planowanie")
    {
        Some("wisdom".to_string())
    } else if title_lower.contains("sen")
        || title_lower.contains("dieta")
        || title_lower.contains("zdrowie")
        || title_lower.contains("nawyk")
    {
        Some("constitution".to_string())
    } else {
        None
    };

    (base_exp, attribute)
}

/// Oblicza punkty EXP za ukończenie nawyku
///
/// # Arguments
/// * `habit_title` - Tytuł nawyku
/// * `streak_bonus` - Bonus za streak (0-100%)
///
/// # Returns
/// * `(i64, Option<String>)` - Punkty EXP i opcjonalny atrybut do zwiększenia
pub fn calculate_habit_exp(habit_title: &str, streak_bonus: f64) -> (i64, Option<String>) {
    let base_exp = 10;
    let title_lower = habit_title.to_lowercase();

    // Bonus za streak (0-50% dodatkowego EXP)
    let streak_multiplier = 1.0 + (streak_bonus * 0.5);
    let final_exp = (base_exp as f64 * streak_multiplier).round() as i64;

    // Analiza atrybutów podobnie jak w zadaniach
    let attribute = if title_lower.contains("sport")
        || title_lower.contains("trening")
        || title_lower.contains("ćwiczenia")
        || title_lower.contains("fitness")
    {
        Some("strength".to_string())
    } else if title_lower.contains("nauka")
        || title_lower.contains("książka")
        || title_lower.contains("czytanie")
        || title_lower.contains("kurs")
    {
        Some("intelligence".to_string())
    } else if title_lower.contains("prezentacja")
        || title_lower.contains("spotkanie")
        || title_lower.contains("kontakt")
        || title_lower.contains("rozmowa")
    {
        Some("charisma".to_string())
    } else if title_lower.contains("hobby")
        || title_lower.contains("praktyka")
        || title_lower.contains("umiejętność")
        || title_lower.contains("gra")
    {
        Some("dexterity".to_string())
    } else if title_lower.contains("medytacja")
        || title_lower.contains("refleksja")
        || title_lower.contains("mindfulness")
        || title_lower.contains("planowanie")
    {
        Some("wisdom".to_string())
    } else if title_lower.contains("sen")
        || title_lower.contains("dieta")
        || title_lower.contains("zdrowie")
        || title_lower.contains("woda")
    {
        Some("constitution".to_string())
    } else {
        None
    };

    (final_exp, attribute)
}

/// Przetwarza ukończenie zadania - dodaje EXP i atrybuty
pub fn process_task_completion(
    connection: &Connection,
    task_title: &str,
    is_goal_related: bool,
) -> Result<(Character, bool)> {
    let (exp_points, attribute) = calculate_task_exp(task_title, is_goal_related);

    // Dodaj EXP
    let (mut character, level_up) = add_experience(connection, exp_points)?;

    // Dodaj punkty atrybutu jeśli wykryto kategorię
    if let Some(attr) = attribute {
        character = add_attribute_points(connection, &attr, 1)?;
    }

    Ok((character, level_up))
}

/// Przetwarza ukończenie nawyku - dodaje EXP i atrybuty
pub fn process_habit_completion(
    connection: &Connection,
    habit_title: &str,
    current_streak: i32,
) -> Result<(Character, bool)> {
    // Oblicz bonus streak (0-100% za streaki 1-30+ dni)
    let streak_bonus = (current_streak as f64 / 30.0).min(1.0);
    let (exp_points, attribute) = calculate_habit_exp(habit_title, streak_bonus);

    // Dodaj EXP
    let (mut character, level_up) = add_experience(connection, exp_points)?;

    // Dodaj punkty atrybutu jeśli wykryto kategorię
    if let Some(attr) = attribute {
        character = add_attribute_points(connection, &attr, 1)?;
    }

    Ok((character, level_up))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let mut conn = Connection::open_in_memory().unwrap();

        // Stwórz tabele dla testów
        conn.execute(
            "CREATE TABLE characters (
                id INTEGER PRIMARY KEY,
                level INTEGER NOT NULL DEFAULT 1,
                experience INTEGER NOT NULL DEFAULT 0,
                character_class TEXT NOT NULL,
                strength INTEGER NOT NULL DEFAULT 10,
                intelligence INTEGER NOT NULL DEFAULT 10,
                charisma INTEGER NOT NULL DEFAULT 10,
                dexterity INTEGER NOT NULL DEFAULT 10,
                wisdom INTEGER NOT NULL DEFAULT 10,
                constitution INTEGER NOT NULL DEFAULT 10,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_create_and_get_character() {
        let conn = setup_test_db();

        let request = CreateCharacterRequest {
            character_class: CharacterClass::Mage,
        };

        let character = create_character(&conn, request).unwrap();
        assert_eq!(character.level, 1);
        assert_eq!(character.experience, 0);

        let retrieved = get_character(&conn).unwrap();
        assert_eq!(retrieved.id, character.id);
    }

    #[test]
    fn test_add_experience() {
        let conn = setup_test_db();

        let request = CreateCharacterRequest {
            character_class: CharacterClass::Warrior,
        };
        create_character(&conn, request).unwrap();

        let (character, level_up) = add_experience(&conn, 150).unwrap();
        assert_eq!(character.experience, 150);
        assert_eq!(character.level, 2); // sqrt(150/100) + 1 = 2
        assert!(level_up);
    }

    #[test]
    fn test_calculate_task_exp() {
        let (exp, attr) = calculate_task_exp("Trening na siłowni", false);
        assert_eq!(exp, 15);
        assert_eq!(attr, Some("strength".to_string()));

        let (exp, attr) = calculate_task_exp("Nauka języka Python", true);
        assert_eq!(exp, 25);
        assert_eq!(attr, Some("intelligence".to_string()));
    }

    #[test]
    fn test_calculate_habit_exp() {
        let (exp, attr) = calculate_habit_exp("Codzienna medytacja", 0.5);
        assert_eq!(exp, 13); // 10 * (1 + 0.5 * 0.5) = 12.5 -> 13
        assert_eq!(attr, Some("wisdom".to_string()));
    }
}
