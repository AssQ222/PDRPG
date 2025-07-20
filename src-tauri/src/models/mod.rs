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
    #[allow(dead_code)]
    pub fn mark_completed(&mut self) {
        self.completed = true;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Oznacza zadanie jako nieukończone
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub title: Option<String>,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub fn experience_to_next_level(&self) -> i64 {
        let next_level = self.level + 1;
        let required_exp = ((next_level - 1) * (next_level - 1) * 100) as i64;
        required_exp - self.experience
    }

    /// Oblicza postęp do następnego poziomu w procentach (0.0 - 1.0)
    #[allow(dead_code)]
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

/// Status questu tygodniowego
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestStatus {
    /// Quest aktywny - można go ukończyć
    Active,
    /// Quest ukończony
    Completed,
    /// Quest wygasł (minął deadline)
    Expired,
}

/// Typ questu określający jego kategorię
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestType {
    /// Quest związany z zadaniami (np. "Ukończ 5 zadań z kategorii 'nauka'")
    Task,
    /// Quest związany z nawykami (np. "Utrzymaj nawyk przez 7 dni z rzędu")
    Habit,
    /// Quest związany z postacią (np. "Zdobądź 100 EXP")
    Character,
}

/// Model reprezentujący quest tygodniowy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    /// Unikalny identyfikator questu
    pub id: i32,
    /// Tytuł questu
    pub title: String,
    /// Szczegółowy opis questu
    pub description: String,
    /// Typ questu
    pub quest_type: QuestType,
    /// Wartość docelowa (np. liczba zadań do ukończenia)
    pub target_value: i32,
    /// Aktualny postęp (np. ukończone zadania)
    pub current_progress: i32,
    /// Kategoria/tag dla questów zadaniowych (opcjonalna)
    pub category: Option<String>,
    /// ID nawyku dla questów nawykowych (opcjonalne)
    pub habit_id: Option<i32>,
    /// Status questu
    pub status: QuestStatus,
    /// Nagroda EXP za ukończenie
    pub reward_exp: i64,
    /// Deadline questu (Unix timestamp, opcjonalny)
    pub deadline: Option<i64>,
    /// Tydzień w którym quest został utworzony (YYYY-WW format)
    pub week: String,
    /// Timestamp utworzenia questu
    pub created_at: i64,
    /// Timestamp ostatniej aktualizacji
    pub updated_at: i64,
}

impl Quest {
    /// Tworzy nowy quest
    pub fn new(
        title: String,
        description: String,
        quest_type: QuestType,
        target_value: i32,
        category: Option<String>,
        habit_id: Option<i32>,
        reward_exp: i64,
        deadline: Option<i64>,
        week: String,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();

        Quest {
            id: 0, // Będzie ustawione przez bazę danych
            title,
            description,
            quest_type,
            target_value,
            current_progress: 0,
            category,
            habit_id,
            status: QuestStatus::Active,
            reward_exp,
            deadline,
            week,
            created_at: now,
            updated_at: now,
        }
    }

    /// Sprawdza czy quest jest ukończony
    pub fn is_completed(&self) -> bool {
        self.current_progress >= self.target_value
    }

    /// Aktualizuje postęp questu
    pub fn update_progress(&mut self, progress: i32) {
        self.current_progress = progress;
        if self.is_completed() && matches!(self.status, QuestStatus::Active) {
            self.status = QuestStatus::Completed;
        }
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// Oznacza quest jako wygasły
    pub fn mark_expired(&mut self) {
        self.status = QuestStatus::Expired;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

/// Typ odznaki/achievementu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementType {
    /// Odznaka za streak nawyków
    HabitStreak,
    /// Odznaka za liczbę ukończonych zadań
    TaskCount,
    /// Odznaka za poziom postaci
    CharacterLevel,
    /// Odznaka za ukończone questy
    QuestCount,
}

/// Status odznaki
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementStatus {
    /// Odznaka zablokowana (wymagania niespełnione)
    Locked,
    /// Odznaka dostępna (wymagania spełnione, ale nie odebrana)
    Available,
    /// Odznaka zdobyta (odebrana przez użytkownika)
    Earned,
}

/// Model reprezentujący odznakę/achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    /// Unikalny identyfikator odznaki
    pub id: i32,
    /// Nazwa odznaki
    pub name: String,
    /// Opis odznaki
    pub description: String,
    /// Typ odznaki
    pub achievement_type: AchievementType,
    /// Wymagana wartość do zdobycia odznaki (np. 30 dla 30-day streak)
    pub required_value: i32,
    /// Ikona odznaki (emoji lub kod)
    pub icon: String,
    /// Status odznaki
    pub status: AchievementStatus,
    /// Timestamp zdobycia odznaki (tylko gdy status = Earned)
    pub earned_at: Option<i64>,
    /// Timestamp utworzenia odznaki
    pub created_at: i64,
    /// Timestamp ostatniej aktualizacji
    pub updated_at: i64,
}

impl Achievement {
    /// Tworzy nową odznakę
    pub fn new(
        name: String,
        description: String,
        achievement_type: AchievementType,
        required_value: i32,
        icon: String,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();

        Achievement {
            id: 0, // Będzie ustawione przez bazę danych
            name,
            description,
            achievement_type,
            required_value,
            icon,
            status: AchievementStatus::Locked,
            earned_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Oznacza odznakę jako dostępną
    pub fn make_available(&mut self) {
        if matches!(self.status, AchievementStatus::Locked) {
            self.status = AchievementStatus::Available;
            self.updated_at = chrono::Utc::now().timestamp();
        }
    }

    /// Oznacza odznakę jako zdobytą
    pub fn mark_earned(&mut self) {
        let now = chrono::Utc::now().timestamp();
        self.status = AchievementStatus::Earned;
        self.earned_at = Some(now);
        self.updated_at = now;
    }
}

/// Struktura reprezentująca dane do utworzenia nowego questu
#[derive(Debug, Deserialize)]
pub struct CreateQuestRequest {
    pub title: String,
    pub description: String,
    pub quest_type: QuestType,
    pub target_value: i32,
    pub category: Option<String>,
    pub habit_id: Option<i32>,
    pub reward_exp: i64,
    pub deadline: Option<i64>,
}

/// Struktura reprezentująca dane do aktualizacji questu
#[derive(Debug, Deserialize)]
pub struct UpdateQuestRequest {
    pub current_progress: Option<i32>,
    pub status: Option<QuestStatus>,
}

/// Struktura reprezentująca dane do utworzenia nowej odznaki
#[derive(Debug, Deserialize)]
pub struct CreateAchievementRequest {
    pub name: String,
    pub description: String,
    pub achievement_type: AchievementType,
    pub required_value: i32,
    pub icon: String,
}
