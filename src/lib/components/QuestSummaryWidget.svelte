<script lang="ts">
    import {
        activeQuests,
        questStats,
        isQuestsLoading,
        questError,
    } from "../stores/questStore";
    import {
        getQuestStatusColor,
        getQuestTypeIcon,
        calculateQuestProgress,
    } from "../types/quest";

    $: quests = $activeQuests;
    $: stats = $questStats;
    $: loading = $isQuestsLoading;
    $: error = $questError;

    // Pokaż tylko pierwsze 3 najważniejsze questy
    $: topQuests = quests
        .sort((a, b) => {
            // Sortuj według postępu (najmniej ukończone najpierw)
            const progressA = calculateQuestProgress(a);
            const progressB = calculateQuestProgress(b);
            if (progressA === progressB) {
                // Jeśli równy postęp, sortuj według nagrody EXP
                return b.reward_exp - a.reward_exp;
            }
            return progressA - progressB;
        })
        .slice(0, 3);
</script>

<div class="quest-summary-widget">
    <div class="widget-header">
        <h3 class="widget-title">
            <span class="title-icon">🎯</span>
            Aktywne Questy
        </h3>
        <div class="quest-stats">
            <span class="stat-item">
                <span class="stat-label">Aktywne:</span>
                <span class="stat-value">{stats.active}</span>
            </span>
            <span class="stat-item">
                <span class="stat-label">Ukończone:</span>
                <span class="stat-value">{stats.completed}</span>
            </span>
        </div>
    </div>

    <div class="widget-content">
        {#if loading}
            <div class="loading-state">
                <div class="loading-spinner"></div>
                <p>Ładowanie questów...</p>
            </div>
        {:else if error}
            <div class="error-state">
                <span class="error-icon">⚠️</span>
                <p>Błąd ładowania questów</p>
            </div>
        {:else if topQuests.length === 0}
            <div class="empty-state">
                <span class="empty-icon">🎲</span>
                <p>Brak aktywnych questów</p>
                <p class="empty-hint">Wygeneruj nowe questy tygodniowe!</p>
            </div>
        {:else}
            <div class="quest-list">
                {#each topQuests as quest}
                    {@const progress = calculateQuestProgress(quest)}
                    <div class="quest-item">
                        <div class="quest-info">
                            <div class="quest-header">
                                <span class="quest-type-icon">
                                    {getQuestTypeIcon(quest.quest_type)}
                                </span>
                                <span class="quest-title">{quest.title}</span>
                                <span class="quest-reward"
                                    >💰 {quest.reward_exp} EXP</span
                                >
                            </div>
                            <div class="quest-progress">
                                <div class="progress-bar">
                                    <div
                                        class="progress-fill"
                                        style="width: {progress}%; background-color: {getQuestStatusColor(
                                            quest.status,
                                        )};"
                                    ></div>
                                </div>
                                <span class="progress-text">
                                    {quest.current_progress}/{quest.target_value}
                                    ({progress.toFixed(0)}%)
                                </span>
                            </div>
                        </div>
                    </div>
                {/each}
            </div>
        {/if}
    </div>
</div>

<style>
    .quest-summary-widget {
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    .widget-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 1rem;
        padding-bottom: 0.75rem;
        border-bottom: 1px solid #e5e7eb;
    }

    .widget-title {
        margin: 0;
        font-size: 1.125rem;
        font-weight: 600;
        color: #374151;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .title-icon {
        font-size: 1.25rem;
    }

    .quest-stats {
        display: flex;
        gap: 1rem;
        font-size: 0.875rem;
    }

    .stat-item {
        display: flex;
        align-items: center;
        gap: 0.25rem;
    }

    .stat-label {
        color: #6b7280;
    }

    .stat-value {
        font-weight: 600;
        color: #1f2937;
        background: #f3f4f6;
        padding: 0.125rem 0.375rem;
        border-radius: 0.375rem;
        min-width: 1.5rem;
        text-align: center;
    }

    .widget-content {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    .loading-state,
    .error-state,
    .empty-state {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
        color: #6b7280;
        gap: 0.5rem;
    }

    .loading-spinner {
        width: 2rem;
        height: 2rem;
        border: 2px solid #e5e7eb;
        border-top: 2px solid #3b82f6;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .error-icon,
    .empty-icon {
        font-size: 2rem;
        opacity: 0.6;
    }

    .empty-hint {
        font-size: 0.75rem;
        color: #9ca3af;
        margin-top: 0.25rem;
    }

    .quest-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .quest-item {
        background: #f9fafb;
        border: 1px solid #e5e7eb;
        border-radius: 0.5rem;
        padding: 0.75rem;
        transition: all 0.2s ease;
    }

    .quest-item:hover {
        background: #f3f4f6;
        border-color: #d1d5db;
    }

    .quest-header {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 0.5rem;
    }

    .quest-type-icon {
        font-size: 1rem;
    }

    .quest-title {
        flex: 1;
        font-weight: 500;
        color: #374151;
        font-size: 0.875rem;
        line-height: 1.2;
    }

    .quest-reward {
        font-size: 0.75rem;
        color: #059669;
        font-weight: 600;
        background: #ecfdf5;
        padding: 0.125rem 0.375rem;
        border-radius: 0.25rem;
    }

    .quest-progress {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .progress-bar {
        flex: 1;
        height: 0.375rem;
        background: #e5e7eb;
        border-radius: 0.1875rem;
        overflow: hidden;
    }

    .progress-fill {
        height: 100%;
        transition: width 0.3s ease;
        border-radius: 0.1875rem;
    }

    .progress-text {
        font-size: 0.75rem;
        color: #6b7280;
        font-weight: 500;
        min-width: fit-content;
    }
</style>
