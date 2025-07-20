-- Migration: Create quests and achievements tables
-- Tabele dla systemu questów tygodniowych i odznak

-- Tabela questów tygodniowych
CREATE TABLE IF NOT EXISTS quests (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    quest_type TEXT NOT NULL CHECK (quest_type IN ('Task', 'Habit', 'Character')),
    target_value INTEGER NOT NULL,
    current_progress INTEGER NOT NULL DEFAULT 0,
    category TEXT, -- Opcjonalna kategoria dla questów zadaniowych
    habit_id INTEGER, -- Opcjonalny ID nawyku dla questów nawykowych
    status TEXT NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Completed', 'Expired')),
    reward_exp INTEGER NOT NULL,
    deadline INTEGER, -- Unix timestamp, opcjonalny
    week TEXT NOT NULL, -- Format YYYY-WW
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    
    -- Foreign key constraint dla habit_id
    FOREIGN KEY (habit_id) REFERENCES habits (id) ON DELETE CASCADE
);

-- Tabela odznak/achievementów
CREATE TABLE IF NOT EXISTS achievements (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    achievement_type TEXT NOT NULL CHECK (achievement_type IN ('HabitStreak', 'TaskCount', 'CharacterLevel', 'QuestCount')),
    required_value INTEGER NOT NULL,
    icon TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Locked' CHECK (status IN ('Locked', 'Available', 'Earned')),
    earned_at INTEGER, -- Unix timestamp, tylko gdy status = 'Earned'
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Indeksy dla wydajności questów
CREATE INDEX IF NOT EXISTS idx_quests_status ON quests(status);
CREATE INDEX IF NOT EXISTS idx_quests_week ON quests(week);
CREATE INDEX IF NOT EXISTS idx_quests_type ON quests(quest_type);
CREATE INDEX IF NOT EXISTS idx_quests_habit_id ON quests(habit_id);
CREATE INDEX IF NOT EXISTS idx_quests_deadline ON quests(deadline);

-- Indeksy dla wydajności odznak
CREATE INDEX IF NOT EXISTS idx_achievements_type ON achievements(achievement_type);
CREATE INDEX IF NOT EXISTS idx_achievements_status ON achievements(status);
CREATE INDEX IF NOT EXISTS idx_achievements_earned_at ON achievements(earned_at);

-- Wstaw podstawowe odznaki za streaki nawyków
INSERT OR IGNORE INTO achievements (
    name, description, achievement_type, required_value, icon, status, created_at, updated_at
) VALUES
    ('Pierwsze Kroki', 'Utrzymaj nawyk przez 7 dni z rzędu', 'HabitStreak', 7, '🔥', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')),
    ('Miesięczny Mistrz', 'Utrzymaj nawyk przez 30 dni z rzędu', 'HabitStreak', 30, '⭐', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')),
    ('Kwartalny Bohater', 'Utrzymaj nawyk przez 90 dni z rzędu', 'HabitStreak', 90, '🏆', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')),
    ('Półroczny Legend', 'Utrzymaj nawyk przez 150 dni z rzędu', 'HabitStreak', 150, '💎', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')),
    ('Roczny Mistrz', 'Utrzymaj nawyk przez 230 dni z rzędu', 'HabitStreak', 230, '👑', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')),
    ('Starter Zadań', 'Ukończ 10 zadań', 'TaskCount', 10, '📝', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')),
    ('Wykonawca', 'Ukończ 50 zadań', 'TaskCount', 50, '✅', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')),
    ('Centurion Zadań', 'Ukończ 100 zadań', 'TaskCount', 100, '💪', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')),
    ('Wojownik Poziomu', 'Osiągnij poziom 5', 'CharacterLevel', 5, '⚔️', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')),
    ('Mistrzowski Awans', 'Osiągnij poziom 10', 'CharacterLevel', 10, '🛡️', 'Locked', strftime('%s', 'now'), strftime('%s', 'now')); 