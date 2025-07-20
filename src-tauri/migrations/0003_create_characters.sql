-- Migration: Create characters table
-- Tabela przechowująca dane postaci gracza w systemie RPG

CREATE TABLE IF NOT EXISTS characters (
    id INTEGER PRIMARY KEY CHECK (id = 1), -- Zawsze jedna postać na użytkownika
    level INTEGER NOT NULL DEFAULT 1,
    experience INTEGER NOT NULL DEFAULT 0,
    character_class TEXT NOT NULL CHECK (character_class IN ('Warrior', 'Mage', 'Bard', 'Rogue')),
    
    -- Atrybuty postaci
    strength INTEGER NOT NULL DEFAULT 10,
    intelligence INTEGER NOT NULL DEFAULT 10,
    charisma INTEGER NOT NULL DEFAULT 10,
    dexterity INTEGER NOT NULL DEFAULT 10,
    wisdom INTEGER NOT NULL DEFAULT 10,
    constitution INTEGER NOT NULL DEFAULT 10,
    
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Indeksy dla wydajności
CREATE INDEX IF NOT EXISTS idx_characters_level ON characters(level);
CREATE INDEX IF NOT EXISTS idx_characters_experience ON characters(experience);
CREATE INDEX IF NOT EXISTS idx_characters_class ON characters(character_class);

-- Wstaw domyślną postać jeśli nie istnieje (klasa Warrior)
INSERT OR IGNORE INTO characters (
    id, level, experience, character_class,
    strength, intelligence, charisma, dexterity, wisdom, constitution,
    created_at, updated_at
) VALUES (
    1, 1, 0, 'Warrior',
    10, 10, 10, 10, 10, 10,
    strftime('%s', 'now'), strftime('%s', 'now')
); 