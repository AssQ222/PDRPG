use serde::{Deserialize, Serialize};

/// Model reprezentujący zadanie w aplikacji
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unikalny identyfikator zadania
    pub id: i32,
    /// Tytuł/nazwa zadania
    pub title: String,
    /// Status ukończenia zadania
    pub completed: bool,
    /// Timestamp utworzenia zadania (Unix timestamp)
    pub created_at: i64,
    /// Timestamp ostatniej modyfikacji (Unix timestamp)
    pub updated_at: i64,
}

impl Task {
    /// Tworzy nowe zadanie z podanym tytułem
    ///
    /// # Arguments
    /// * `title` - Tytuł zadania
    ///
    /// # Returns
    /// * `Task` - Nowa instancja zadania z wartościami domyślnymi
    pub fn new(title: String) -> Self {
        let now = chrono::Utc::now().timestamp();

        Task {
            id: 0, // Będzie ustawione przez bazę danych
            title,
            completed: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// Oznacza zadanie jako ukończone
    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Oznacza zadanie jako nieukończone
    pub fn mark_incomplete(&mut self) {
        self.completed = false;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Przełącza status ukończenia zadania
    pub fn toggle_completed(&mut self) {
        self.completed = !self.completed;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Aktualizuje tytuł zadania
    ///
    /// # Arguments
    /// * `new_title` - Nowy tytuł zadania
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

/// Struktura reprezentująca dane do utworzenia nowego zadania
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
}

/// Struktura reprezentująca dane do aktualizacji zadania
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

/// Typ nawyku określający jak jest śledzony
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HabitType {
    /// Nawyk typu tak/nie (np. "Czy medytowałem dzisiaj?")
    Boolean,
    /// Nawyk typu licznik (np. "Ile szklanek wody wypiłem?")
    Counter,
}

/// Model reprezentujący nawyk w aplikacji
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Habit {
    /// Unikalny identyfikator nawyku
    pub id: i32,
    /// Nazwa nawyku
    pub title: String,
    /// Typ nawyku (boolean vs counter)
    pub habit_type: HabitType,
    /// Wartość docelowa (dla typu counter, np. 8 szklanek wody)
    pub target_value: Option<i32>,
    /// Aktualny ciąg dni (streak)
    pub current_streak: i32,
    /// Timestamp utworzenia nawyku (Unix timestamp)
    pub created_at: i64,
    /// Timestamp ostatniej modyfikacji (Unix timestamp)
    pub updated_at: i64,
}

impl Habit {
    /// Tworzy nowy nawyk z podaną nazwą i typem
    ///
    /// # Arguments
    /// * `title` - Nazwa nawyku
    /// * `habit_type` - Typ nawyku (Boolean lub Counter)
    /// * `target_value` - Wartość docelowa (tylko dla typu Counter)
    ///
    /// # Returns
    /// * `Habit` - Nowa instancja nawyku z wartościami domyślnymi
    pub fn new(title: String, habit_type: HabitType, target_value: Option<i32>) -> Self {
        let now = chrono::Utc::now().timestamp();

        Habit {
            id: 0, // Będzie ustawione przez bazę danych
            title,
            habit_type,
            target_value,
            current_streak: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Aktualizuje tytuł nawyku
    ///
    /// # Arguments
    /// * `new_title` - Nowy tytuł nawyku
    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Aktualizuje wartość docelową nawyku
    ///
    /// # Arguments
    /// * `new_target` - Nowa wartość docelowa
    pub fn update_target_value(&mut self, new_target: Option<i32>) {
        self.target_value = new_target;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Aktualizuje aktualny streak
    ///
    /// # Arguments
    /// * `new_streak` - Nowy streak
    pub fn update_streak(&mut self, new_streak: i32) {
        self.current_streak = new_streak;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

/// Model reprezentujący wpis nawyku na konkretny dzień
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HabitEntry {
    /// Unikalny identyfikator wpisu
    pub id: i32,
    /// ID nawyku do którego należy wpis
    pub habit_id: i32,
    /// Data wpisu (YYYY-MM-DD format)
    pub date: String,
    /// Czy nawyk został wykonany (dla typu Boolean)
    pub completed: bool,
    /// Wartość dla nawyków typu Counter
    pub value: i32,
    /// Timestamp utworzenia wpisu
    pub created_at: i64,
}

impl HabitEntry {
    /// Tworzy nowy wpis nawyku
    ///
    /// # Arguments
    /// * `habit_id` - ID nawyku
    /// * `date` - Data w formacie YYYY-MM-DD
    /// * `completed` - Czy wykonano (dla typu Boolean)
    /// * `value` - Wartość (dla typu Counter)
    ///
    /// # Returns
    /// * `HabitEntry` - Nowa instancja wpisu nawyku
    pub fn new(habit_id: i32, date: String, completed: bool, value: i32) -> Self {
        let now = chrono::Utc::now().timestamp();

        HabitEntry {
            id: 0, // Będzie ustawione przez bazę danych
            habit_id,
            date,
            completed,
            value,
            created_at: now,
        }
    }
}

/// Struktura reprezentująca dane do utworzenia nowego nawyku
#[derive(Debug, Deserialize)]
pub struct CreateHabitRequest {
    pub title: String,
    pub habit_type: HabitType,
    pub target_value: Option<i32>,
}

/// Struktura reprezentująca dane do aktualizacji nawyku
#[derive(Debug, Deserialize)]
pub struct UpdateHabitRequest {
    pub title: Option<String>,
    pub target_value: Option<i32>,
}

/// Struktura reprezentująca dane do utworzenia wpisu nawyku
#[derive(Debug, Deserialize)]
pub struct CreateHabitEntryRequest {
    pub habit_id: i32,
    pub date: String,
    pub completed: Option<bool>,
    pub value: Option<i32>,
}

/// Klasy postaci reprezentujące różne obszary rozwoju
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterClass {
    /// Wojownik - rozwój fizyczny, sport, zdrowie
    Warrior,
    /// Mag - rozwój intelektualny, nauka, umiejętności
    Mage,
    /// Bard - kreatywność, umiejętności społeczne, komunikacja
    Bard,
    /// Łotrzyk - finanse, przedsiębiorczość, praktyczne umiejętności
    Rogue,
}

/// Atrybuty postaci dla wykresu pajęczynowego
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAttributes {
    /// Siła fizyczna (sport, trening, zdrowie)
    pub strength: i32,
    /// Intelekt (nauka, czytanie, kursy)
    pub intelligence: i32,
    /// Charyzma (kontakty społeczne, prezentacje)
    pub charisma: i32,
    /// Zręczność (praktyczne umiejętności, hobby)
    pub dexterity: i32,
    /// Mądrość (medytacja, refleksja, mindfulness)
    pub wisdom: i32,
    /// Konstytucja (sen, dieta, nawyki zdrowotne)
    pub constitution: i32,
}

impl CharacterAttributes {
    /// Tworzy nowe atrybuty z wartościami startowymi
    pub fn new() -> Self {
        CharacterAttributes {
            strength: 10,
            intelligence: 10,
            charisma: 10,
            dexterity: 10,
            wisdom: 10,
            constitution: 10,
        }
    }

    /// Dodaje punkty do określonego atrybutu
    pub fn add_points(&mut self, attribute: &str, points: i32) {
        match attribute {
            "strength" => self.strength += points,
            "intelligence" => self.intelligence += points,
            "charisma" => self.charisma += points,
            "dexterity" => self.dexterity += points,
            "wisdom" => self.wisdom += points,
            "constitution" => self.constitution += points,
            _ => {}
        }
    }
}

/// Model reprezentujący postać gracza w systemie RPG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// Unikalny identyfikator postaci (zawsze 1 - jedna postać na użytkownika)
    pub id: i32,
    /// Aktualny poziom postaci
    pub level: i32,
    /// Aktualny experience points
    pub experience: i64,
    /// Klasa postaci
    pub character_class: CharacterClass,
    /// Atrybuty postaci
    pub attributes: CharacterAttributes,
    /// Timestamp utworzenia postaci
    pub created_at: i64,
    /// Timestamp ostatniej modyfikacji
    pub updated_at: i64,
}

impl Character {
    /// Tworzy nową postać z domyślnymi wartościami
    pub fn new(character_class: CharacterClass) -> Self {
        let now = chrono::Utc::now().timestamp();

        Character {
            id: 1, // Zawsze jedna postać na użytkownika
            level: 1,
            experience: 0,
            character_class,
            attributes: CharacterAttributes::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Dodaje punkty doświadczenia i sprawdza awanse poziomów
    ///
    /// # Arguments
    /// * `exp_points` - Ilość punktów doświadczenia do dodania
    ///
    /// # Returns
    /// * `bool` - True jeśli nastąpił awans poziomu
    pub fn add_experience(&mut self, exp_points: i64) -> bool {
        self.experience += exp_points;
        self.updated_at = chrono::Utc::now().timestamp();

        let old_level = self.level;
        self.level = self.calculate_level();

        old_level < self.level
    }

    /// Oblicza poziom na podstawie aktualnego doświadczenia
    /// Formuła: level = floor(sqrt(experience / 100)) + 1
    pub fn calculate_level(&self) -> i32 {
        ((self.experience as f64 / 100.0).sqrt().floor() as i32) + 1
    }

    /// Oblicza doświadczenie wymagane do następnego poziomu
    pub fn experience_to_next_level(&self) -> i64 {
        let next_level = self.level + 1;
        let required_exp = ((next_level - 1) * (next_level - 1) * 100) as i64;
        required_exp - self.experience
    }

    /// Oblicza postęp do następnego poziomu w procentach (0.0 - 1.0)
    pub fn level_progress(&self) -> f64 {
        let current_level_exp = ((self.level - 1) * (self.level - 1) * 100) as i64;
        let next_level_exp = (self.level * self.level * 100) as i64;
        let progress_exp = self.experience - current_level_exp;
        let total_exp_needed = next_level_exp - current_level_exp;

        if total_exp_needed > 0 {
            progress_exp as f64 / total_exp_needed as f64
        } else {
            0.0
        }
    }

    /// Dodaje punkty do atrybutu
    pub fn add_attribute_points(&mut self, attribute: &str, points: i32) {
        self.attributes.add_points(attribute, points);
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

/// Struktura reprezentująca dane do utworzenia nowej postaci
#[derive(Debug, Deserialize)]
pub struct CreateCharacterRequest {
    pub character_class: CharacterClass,
}

/// Struktura reprezentująca dane do aktualizacji postaci
#[derive(Debug, Deserialize)]
pub struct UpdateCharacterRequest {
    pub character_class: Option<CharacterClass>,
}
