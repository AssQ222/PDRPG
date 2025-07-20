import { writable, derived, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import type { Quest, Achievement, QuestError, AchievementError, AchievementStatus } from '../types/quest';

// Stores dla questów
const questsStore = writable<Quest[]>([]);
const questsLoading = writable<boolean>(false);
const questsError = writable<QuestError | null>(null);

// Stores dla odznak
const achievementsStore = writable<Achievement[]>([]);
const achievementsLoading = writable<boolean>(false);
const achievementsError = writable<AchievementError | null>(null);

// Derived stores dla questów
export const quests = derived(questsStore, $quests => $quests);
export const activeQuests = derived(questsStore, $quests =>
    $quests.filter(quest => quest.status === 'Active')
);
export const completedQuests = derived(questsStore, $quests =>
    $quests.filter(quest => quest.status === 'Completed')
);
export const expiredQuests = derived(questsStore, $quests =>
    $quests.filter(quest => quest.status === 'Expired')
);

export const questStats = derived(questsStore, $quests => {
    const total = $quests.length;
    const active = $quests.filter(q => q.status === 'Active').length;
    const completed = $quests.filter(q => q.status === 'Completed').length;
    const expired = $quests.filter(q => q.status === 'Expired').length;

    return {
        total,
        active,
        completed,
        expired,
        completionRate: total > 0 ? (completed / total) * 100 : 0
    };
});

// Derived stores dla odznak
export const achievements = derived(achievementsStore, $achievements => $achievements);
export const earnedAchievements = derived(achievementsStore, $achievements =>
    $achievements.filter(achievement => achievement.status === 'Earned')
);
export const availableAchievements = derived(achievementsStore, $achievements =>
    $achievements.filter(achievement => achievement.status === 'Available')
);
export const lockedAchievements = derived(achievementsStore, $achievements =>
    $achievements.filter(achievement => achievement.status === 'Locked')
);

export const achievementStats = derived(achievementsStore, $achievements => {
    const total = $achievements.length;
    const earned = $achievements.filter(a => a.status === 'Earned').length;
    const available = $achievements.filter(a => a.status === 'Available').length;
    const locked = $achievements.filter(a => a.status === 'Locked').length;

    return {
        total,
        earned,
        available,
        locked,
        earnedPercentage: total > 0 ? (earned / total) * 100 : 0
    };
});

// Loading and error states
export const isQuestsLoading = derived(questsLoading, $loading => $loading);
export const questError = derived(questsError, $error => $error);
export const isAchievementsLoading = derived(achievementsLoading, $loading => $loading);
export const achievementError = derived(achievementsError, $error => $error);

// Quest Actions
export const questActions = {
    // Generuj questy tygodniowe
    async generateWeeklyQuests(): Promise<Quest[]> {
        questsLoading.set(true);
        questsError.set(null);

        try {
            const newQuests = await invoke<Quest[]>('generate_weekly_quests');

            // Odśwież listę questów
            await this.loadActiveQuests();

            return newQuests;
        } catch (error) {
            const questError: QuestError = {
                message: error as string,
                code: 'GENERATE_QUESTS_ERROR'
            };
            questsError.set(questError);
            console.error('Failed to generate weekly quests:', error);
            return [];
        } finally {
            questsLoading.set(false);
        }
    },

    // Pobierz questy dla tygodnia
    async loadQuestsForWeek(week?: string): Promise<void> {
        questsLoading.set(true);
        questsError.set(null);

        try {
            const quests = await invoke<Quest[]>('get_quests_for_week', { week });
            questsStore.set(quests);
        } catch (error) {
            const questError: QuestError = {
                message: error as string,
                code: 'LOAD_QUESTS_ERROR'
            };
            questsError.set(questError);
            console.error('Failed to load quests for week:', error);
        } finally {
            questsLoading.set(false);
        }
    },

    // Pobierz aktywne questy
    async loadActiveQuests(): Promise<void> {
        questsLoading.set(true);
        questsError.set(null);

        try {
            const quests = await invoke<Quest[]>('get_active_quests');
            questsStore.set(quests);
        } catch (error) {
            const questError: QuestError = {
                message: error as string,
                code: 'LOAD_ACTIVE_QUESTS_ERROR'
            };
            questsError.set(questError);
            console.error('Failed to load active quests:', error);
        } finally {
            questsLoading.set(false);
        }
    },

    // Aktualizuj postęp questów
    async updateQuestProgress(): Promise<Quest[]> {
        try {
            const updatedQuests = await invoke<Quest[]>('update_quest_progress');

            // Odśwież listę questów
            await this.loadActiveQuests();

            return updatedQuests;
        } catch (error) {
            console.error('Failed to update quest progress:', error);
            return [];
        }
    },

    // Ukończ quest
    async completeQuest(questId: number): Promise<Quest | null> {
        try {
            const completedQuest = await invoke<Quest>('complete_quest', { questId });

            // Aktualizuj local store
            const currentQuests = get(questsStore);
            const updatedQuests = currentQuests.map(quest =>
                quest.id === questId ? completedQuest : quest
            );
            questsStore.set(updatedQuests);

            // Sprawdź czy można zaktualizować achievements
            await achievementActions.checkAndUpdateAchievements();

            return completedQuest;
        } catch (error) {
            const questError: QuestError = {
                message: error as string,
                code: 'COMPLETE_QUEST_ERROR'
            };
            questsError.set(questError);
            console.error('Failed to complete quest:', error);
            return null;
        }
    },

    // Wygaś przeterminowane questy
    async expireOverdueQuests(): Promise<number> {
        try {
            const expiredCount = await invoke<number>('expire_overdue_quests');

            // Odśwież listę questów
            await this.loadActiveQuests();

            return expiredCount;
        } catch (error) {
            console.error('Failed to expire overdue quests:', error);
            return 0;
        }
    },

    // Wyczyść błędy
    clearError(): void {
        questsError.set(null);
    }
};

// Achievement Actions
// Quick action for external use
export const updateQuestProgress = questActions.updateQuestProgress;

export const achievementActions = {
    // Pobierz wszystkie odznaki
    async loadAllAchievements(): Promise<void> {
        achievementsLoading.set(true);
        achievementsError.set(null);

        try {
            const achievements = await invoke<Achievement[]>('get_all_achievements');
            achievementsStore.set(achievements);
        } catch (error) {
            const achievementError: AchievementError = {
                message: error as string,
                code: 'LOAD_ACHIEVEMENTS_ERROR'
            };
            achievementsError.set(achievementError);
            console.error('Failed to load achievements:', error);
        } finally {
            achievementsLoading.set(false);
        }
    },

    // Pobierz odznaki według statusu
    async loadAchievementsByStatus(status: AchievementStatus): Promise<Achievement[]> {
        try {
            const achievements = await invoke<Achievement[]>('get_achievements_by_status', { status });
            return achievements;
        } catch (error) {
            console.error('Failed to load achievements by status:', error);
            return [];
        }
    },

    // Sprawdź i zaktualizuj odznaki
    async checkAndUpdateAchievements(): Promise<Achievement[]> {
        try {
            const updatedAchievements = await invoke<Achievement[]>('check_and_update_achievements');

            // Odśwież listę odznak
            await this.loadAllAchievements();

            return updatedAchievements;
        } catch (error) {
            console.error('Failed to check and update achievements:', error);
            return [];
        }
    },

    // Zdobądź odznakę
    async earnAchievement(achievementId: number): Promise<Achievement | null> {
        try {
            const earnedAchievement = await invoke<Achievement>('earn_achievement', { achievementId });

            // Aktualizuj local store
            const currentAchievements = get(achievementsStore);
            const updatedAchievements = currentAchievements.map(achievement =>
                achievement.id === achievementId ? earnedAchievement : achievement
            );
            achievementsStore.set(updatedAchievements);

            return earnedAchievement;
        } catch (error) {
            const achievementError: AchievementError = {
                message: error as string,
                code: 'EARN_ACHIEVEMENT_ERROR'
            };
            achievementsError.set(achievementError);
            console.error('Failed to earn achievement:', error);
            return null;
        }
    },

    // Pobierz statystyki odznak
    async getAchievementStats(): Promise<{ earned: number; available: number; locked: number } | null> {
        try {
            const [earned, available, locked] = await invoke<[number, number, number]>('get_achievement_stats');
            return { earned, available, locked };
        } catch (error) {
            console.error('Failed to get achievement stats:', error);
            return null;
        }
    },

    // Wyczyść błędy
    clearError(): void {
        achievementsError.set(null);
    }
};

// Auto-initialization funkcje
export async function initializeQuestSystem(): Promise<void> {
    try {
        // Wygaś przeterminowane questy
        await questActions.expireOverdueQuests();

        // Załaduj aktywne questy
        await questActions.loadActiveQuests();

        // Aktualizuj postęp questów
        await questActions.updateQuestProgress();

        // Załaduj odznaki
        await achievementActions.loadAllAchievements();

        // Sprawdź i zaktualizuj odznaki
        await achievementActions.checkAndUpdateAchievements();

    } catch (error) {
        console.error('Failed to initialize quest system:', error);
    }
} 