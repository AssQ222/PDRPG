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