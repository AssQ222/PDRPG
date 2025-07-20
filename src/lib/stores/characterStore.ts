import { writable, derived, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { CharacterClass, generateCharacterStats } from '../types/character';
import type {
    Character,
    CharacterStats,
    CharacterError,
    CreateCharacterRequest,
    UpdateCharacterRequest,
    ExperienceResult,
    CharacterAttributes
} from '../types/character';

// Reactive stores
const characterData = writable<Character | null>(null);
const isLoading = writable(false);
const characterError = writable<CharacterError | null>(null);

/// Character actions - funkcje do komunikacji z backendem
export const characterActions = {
    /// Pobiera aktualną postać gracza
    async getCharacter(): Promise<void> {
        isLoading.set(true);
        characterError.set(null);

        try {
            const previousCharacter = get(characterData);
            const character = await invoke<Character>('get_character');

            // Sprawdź czy nastąpił level up
            if (previousCharacter && character.level > previousCharacter.level) {
                console.log(`🎉 Level up detected! ${previousCharacter.level} → ${character.level}`);
                levelUpNotifications.add(character.level);
            }

            characterData.set(character);
        } catch (error) {
            console.error('Failed to get character:', error);
            characterError.set({
                message: error instanceof Error ? error.message : 'Failed to get character',
                code: 'GET_CHARACTER_ERROR'
            });
        } finally {
            isLoading.set(false);
        }
    },

    /// Tworzy nową postać
    async createCharacter(request: CreateCharacterRequest): Promise<void> {
        isLoading.set(true);
        characterError.set(null);

        try {
            const character = await invoke<Character>('create_character', { request });
            characterData.set(character);
        } catch (error) {
            console.error('Failed to create character:', error);
            characterError.set({
                message: error instanceof Error ? error.message : 'Failed to create character',
                code: 'CREATE_CHARACTER_ERROR'
            });
        } finally {
            isLoading.set(false);
        }
    },

    /// Aktualizuje postać
    async updateCharacter(request: UpdateCharacterRequest): Promise<void> {
        isLoading.set(true);
        characterError.set(null);

        try {
            const character = await invoke<Character>('update_character', { request });
            characterData.set(character);
        } catch (error) {
            console.error('Failed to update character:', error);
            characterError.set({
                message: error instanceof Error ? error.message : 'Failed to update character',
                code: 'UPDATE_CHARACTER_ERROR'
            });
        } finally {
            isLoading.set(false);
        }
    },

    /// Dodaje punkty doświadczenia
    async addExperience(expPoints: number): Promise<boolean> {
        characterError.set(null);

        try {
            const previousCharacter = get(characterData);
            const result = await invoke<ExperienceResult>('add_experience', { expPoints });

            // Sprawdź czy nastąpił level up
            if (result.levelUp && previousCharacter) {
                console.log(`🎉 Level up via addExperience! ${previousCharacter.level} → ${result.character.level}`);
                levelUpNotifications.add(result.character.level);
            }

            characterData.set(result.character);
            return result.levelUp;
        } catch (error) {
            console.error('Failed to add experience:', error);
            characterError.set({
                message: error instanceof Error ? error.message : 'Failed to add experience',
                code: 'ADD_EXPERIENCE_ERROR'
            });
            return false;
        }
    },

    /// Dodaje punkty do atrybutu
    async addAttributePoints(attribute: keyof CharacterAttributes, points: number): Promise<void> {
        characterError.set(null);

        try {
            const character = await invoke<Character>('add_attribute_points', {
                attribute,
                points
            });
            characterData.set(character);
        } catch (error) {
            console.error('Failed to add attribute points:', error);
            characterError.set({
                message: error instanceof Error ? error.message : 'Failed to add attribute points',
                code: 'ADD_ATTRIBUTE_POINTS_ERROR'
            });
        }
    },

    /// Czyści błędy
    clearError(): void {
        characterError.set(null);
    },

    /// Inicjalizuje postać przy starcie aplikacji
    async initialize(): Promise<void> {
        try {
            await this.getCharacter();
        } catch (error) {
            // Jeśli postać nie istnieje, stwórz domyślną
            console.log('Character not found, creating default character...');
            await this.createCharacter({ character_class: CharacterClass.Warrior });
        }
    }
};

// Derived stores - obliczane automatycznie na podstawie characterData
export const character = derived(characterData, ($characterData) => $characterData);

export const characterStats = derived(
    characterData,
    ($characterData): CharacterStats | null => {
        if (!$characterData) return null;
        return generateCharacterStats($characterData);
    }
);

export const levelProgress = derived(
    characterStats,
    ($characterStats) => $characterStats?.levelProgress ?? 0
);

export const experienceInfo = derived(
    character,
    ($character) => {
        if (!$character) return null;

        const currentLevelExp = (($character.level - 1) * ($character.level - 1) * 100);
        const nextLevelExp = ($character.level * $character.level * 100);
        const progressExp = $character.experience - currentLevelExp;
        const totalExpNeeded = nextLevelExp - currentLevelExp;

        return {
            current: progressExp,
            needed: totalExpNeeded,
            total: $character.experience,
            nextLevel: nextLevelExp,
            percentage: totalExpNeeded > 0 ? (progressExp / totalExpNeeded * 100) : 0
        };
    }
);

export const attributeInfo = derived(
    character,
    ($character) => {
        if (!$character) return null;

        const attributes = $character.attributes;
        const total = attributes.strength + attributes.intelligence + attributes.charisma +
            attributes.dexterity + attributes.wisdom + attributes.constitution;

        return {
            total,
            strength: attributes.strength,
            intelligence: attributes.intelligence,
            charisma: attributes.charisma,
            dexterity: attributes.dexterity,
            wisdom: attributes.wisdom,
            constitution: attributes.constitution,
            // Procenty dla wykresu pajęczynowego (max 50 punktów na atrybut)
            strengthPercent: Math.min(100, (attributes.strength / 50) * 100),
            intelligencePercent: Math.min(100, (attributes.intelligence / 50) * 100),
            charismaPercent: Math.min(100, (attributes.charisma / 50) * 100),
            dexterityPercent: Math.min(100, (attributes.dexterity / 50) * 100),
            wisdomPercent: Math.min(100, (attributes.wisdom / 50) * 100),
            constitutionPercent: Math.min(100, (attributes.constitution / 50) * 100),
        };
    }
);

export const isCharacterLoaded = derived(
    character,
    ($character) => $character !== null
);

// Re-export podstawowych stores
export { isLoading, characterError };

// Funkcje pomocnicze do notyfikacji o awansach poziomu
export const levelUpNotifications = (() => {
    const notifications = writable<Array<{ id: string, level: number, timestamp: number }>>([]);

    return {
        subscribe: notifications.subscribe,

        add(level: number) {
            const notification = {
                id: `levelup-${Date.now()}`,
                level,
                timestamp: Date.now()
            };

            notifications.update(current => [...current, notification]);

            // Auto remove after 5 seconds
            setTimeout(() => {
                notifications.update(current =>
                    current.filter(n => n.id !== notification.id)
                );
            }, 5000);
        },

        remove(id: string) {
            notifications.update(current =>
                current.filter(n => n.id !== id)
            );
        },

        clear() {
            notifications.set([]);
        }
    };
})();

// Funkcje pomocnicze dla testów i debugowania
export const characterDebug = {
    /// Pobiera aktualny stan postaci synchronicznie
    getCurrentCharacter(): Character | null {
        return get(characterData);
    },

    /// Resetuje stan store'a
    reset(): void {
        characterData.set(null);
        characterError.set(null);
        isLoading.set(false);
    },

    /// Ustawia stan postaci (tylko do testów)
    setCharacter(character: Character): void {
        characterData.set(character);
    }
};

// Auto-initialize character store when module loads
// characterActions.initialize(); 