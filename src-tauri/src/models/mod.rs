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
