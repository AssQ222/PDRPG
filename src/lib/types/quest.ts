// Quest and Achievement types for PDRPG frontend

export type QuestStatus = 'Active' | 'Completed' | 'Expired';
export type QuestType = 'Task' | 'Habit' | 'Character';

export interface Quest {
    id: number;
    title: string;
    description: string;
    quest_type: QuestType;
    target_value: number;
    current_progress: number;
    category?: string;
    habit_id?: number;
    status: QuestStatus;
    reward_exp: number;
    deadline?: number; // Unix timestamp
    week: string; // YYYY-WW format
    created_at: number;
    updated_at: number;
}

export type AchievementStatus = 'Locked' | 'Available' | 'Earned';
export type AchievementType = 'HabitStreak' | 'TaskCount' | 'CharacterLevel' | 'QuestCount';

export interface Achievement {
    id: number;
    name: string;
    description: string;
    achievement_type: AchievementType;
    required_value: number;
    icon: string;
    status: AchievementStatus;
    earned_at?: number; // Unix timestamp
    created_at: number;
    updated_at: number;
}

export interface CreateQuestRequest {
    title: string;
    description: string;
    quest_type: QuestType;
    target_value: number;
    category?: string;
    habit_id?: number;
    reward_exp: number;
    deadline?: number;
}

export interface CreateAchievementRequest {
    name: string;
    description: string;
    achievement_type: AchievementType;
    required_value: number;
    icon: string;
}

export interface QuestError {
    message: string;
    code?: string;
}

export interface AchievementError {
    message: string;
    code?: string;
}

// Helper functions
export function getQuestStatusColor(status: QuestStatus): string {
    switch (status) {
        case 'Active':
            return 'text-blue-600';
        case 'Completed':
            return 'text-green-600';
        case 'Expired':
            return 'text-red-600';
        default:
            return 'text-gray-600';
    }
}

export function getQuestStatusIcon(status: QuestStatus): string {
    switch (status) {
        case 'Active':
            return '🎯';
        case 'Completed':
            return '✅';
        case 'Expired':
            return '⏰';
        default:
            return '❓';
    }
}

export function getQuestTypeIcon(type: QuestType): string {
    switch (type) {
        case 'Task':
            return '📋';
        case 'Habit':
            return '🔄';
        case 'Character':
            return '⚔️';
        default:
            return '📝';
    }
}

export function getAchievementStatusColor(status: AchievementStatus): string {
    switch (status) {
        case 'Locked':
            return 'text-gray-400';
        case 'Available':
            return 'text-yellow-600';
        case 'Earned':
            return 'text-gold-600';
        default:
            return 'text-gray-600';
    }
}

export function formatDeadline(deadline?: number): string {
    if (!deadline) return 'Brak terminu';

    const date = new Date(deadline * 1000);
    const now = new Date();
    const diffMs = date.getTime() - now.getTime();
    const diffDays = Math.ceil(diffMs / (1000 * 60 * 60 * 24));

    if (diffDays < 0) {
        return 'Przekroczony';
    } else if (diffDays === 0) {
        return 'Dzisiaj';
    } else if (diffDays === 1) {
        return 'Jutro';
    } else {
        return `Za ${diffDays} dni`;
    }
}

export function calculateQuestProgress(quest: Quest): number {
    if (quest.target_value === 0) return 0;
    return Math.min(100, (quest.current_progress / quest.target_value) * 100);
}

export function formatQuestProgress(quest: Quest): string {
    return `${quest.current_progress}/${quest.target_value}`;
}

export function formatWeek(week: string): string {
    const [year, weekNum] = week.split('-');
    return `Tydzień ${weekNum}/${year}`;
} 