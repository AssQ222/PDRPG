// Types dla systemu postaci RPG

/// Klasy postaci reprezentujące różne obszary rozwoju
export enum CharacterClass {
    Warrior = "Warrior",  // Rozwój fizyczny, sport, zdrowie
    Mage = "Mage",        // Rozwój intelektualny, nauka, umiejętności
    Bard = "Bard",        // Kreatywność, umiejętności społeczne, komunikacja
    Rogue = "Rogue",      // Finanse, przedsiębiorczość, praktyczne umiejętności
}

/// Atrybuty postaci dla wykresu pajęczynowego
export interface CharacterAttributes {
    strength: number;      // Siła fizyczna (sport, trening, zdrowie)
    intelligence: number;  // Intelekt (nauka, czytanie, kursy)
    charisma: number;      // Charyzma (kontakty społeczne, prezentacje)
    dexterity: number;     // Zręczność (praktyczne umiejętności, hobby)
    wisdom: number;        // Mądrość (medytacja, refleksja, mindfulness)
    constitution: number;  // Konstytucja (sen, dieta, nawyki zdrowotne)
}

/// Model reprezentujący postać gracza w systemie RPG
export interface Character {
    id: number;                         // Unikalny identyfikator postaci (zawsze 1)
    level: number;                      // Aktualny poziom postaci
    experience: number;                 // Aktualny experience points
    character_class: CharacterClass;   // Klasa postaci
    attributes: CharacterAttributes;   // Atrybuty postaci
    created_at: number;                 // Timestamp utworzenia postaci
    updated_at: number;                 // Timestamp ostatniej modyfikacji
}

/// Dane do utworzenia nowej postaci
export interface CreateCharacterRequest {
    character_class: CharacterClass;
}

/// Dane do aktualizacji postaci
export interface UpdateCharacterRequest {
    character_class?: CharacterClass;
}

/// Statystyki postaci dla UI
export interface CharacterStats {
    currentLevel: number;
    currentExperience: number;
    experienceToNextLevel: number;
    levelProgress: number;  // 0.0 - 1.0
    totalAttributePoints: number;
    className: string;
    classDescription: string;
}

/// Błędy związane z postacią
export interface CharacterError {
    message: string;
    code?: string;
}

/// Resultado operacji z EXP i awansem poziomu
export interface ExperienceResult {
    character: Character;
    levelUp: boolean;
}

/// Mapa nazw atrybutów do ich wyświetlanych nazw
export const ATTRIBUTE_NAMES: Record<keyof CharacterAttributes, string> = {
    strength: "Siła",
    intelligence: "Intelekt",
    charisma: "Charyzma",
    dexterity: "Zręczność",
    wisdom: "Mądrość",
    constitution: "Kondycja",
};

/// Mapa klas postaci do opisów
export const CLASS_DESCRIPTIONS: Record<CharacterClass, string> = {
    [CharacterClass.Warrior]: "Skupia się na rozwoju fizycznym, sporcie i zdrowiu",
    [CharacterClass.Mage]: "Rozwija intelekt poprzez naukę i zdobywanie wiedzy",
    [CharacterClass.Bard]: "Stawia na kreatywność i umiejętności społeczne",
    [CharacterClass.Rogue]: "Koncentruje się na finansach i praktycznych umiejętnościach",
};

/// Ikony emoji dla klas postaci
export const CLASS_ICONS: Record<CharacterClass, string> = {
    [CharacterClass.Warrior]: "⚔️",
    [CharacterClass.Mage]: "🧙‍♂️",
    [CharacterClass.Bard]: "🎭",
    [CharacterClass.Rogue]: "🗡️",
};

/// Kolory dla klas postaci (Tailwind CSS)
export const CLASS_COLORS: Record<CharacterClass, string> = {
    [CharacterClass.Warrior]: "text-red-600",
    [CharacterClass.Mage]: "text-blue-600",
    [CharacterClass.Bard]: "text-purple-600",
    [CharacterClass.Rogue]: "text-green-600",
};

/// Funkcje pomocnicze

/// Oblicza EXP wymagane do następnego poziomu
export function calculateExperienceToNextLevel(currentLevel: number): number {
    return (currentLevel * currentLevel * 100);
}

/// Oblicza postęp do następnego poziomu (0.0 - 1.0)
export function calculateLevelProgress(currentLevel: number, currentExperience: number): number {
    const currentLevelExp = ((currentLevel - 1) * (currentLevel - 1) * 100);
    const nextLevelExp = (currentLevel * currentLevel * 100);
    const progressExp = currentExperience - currentLevelExp;
    const totalExpNeeded = nextLevelExp - currentLevelExp;

    if (totalExpNeeded > 0) {
        return Math.min(1.0, progressExp / totalExpNeeded);
    }
    return 0.0;
}

/// Oblicza poziom na podstawie doświadczenia
export function calculateLevel(experience: number): number {
    return Math.floor(Math.sqrt(experience / 100)) + 1;
}

/// Sumuje wszystkie punkty atrybutów
export function calculateTotalAttributePoints(attributes: CharacterAttributes): number {
    return attributes.strength + attributes.intelligence + attributes.charisma +
        attributes.dexterity + attributes.wisdom + attributes.constitution;
}

/// Generuje statystyki postaci na podstawie Character
export function generateCharacterStats(character: Character): CharacterStats {
    const experienceToNextLevel = character.experience < calculateExperienceToNextLevel(character.level)
        ? calculateExperienceToNextLevel(character.level) - character.experience
        : 0;

    return {
        currentLevel: character.level,
        currentExperience: character.experience,
        experienceToNextLevel,
        levelProgress: calculateLevelProgress(character.level, character.experience),
        totalAttributePoints: calculateTotalAttributePoints(character.attributes),
        className: character.character_class,
        classDescription: CLASS_DESCRIPTIONS[character.character_class],
    };
} 