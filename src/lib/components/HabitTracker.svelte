<script lang="ts">
    import { onMount } from "svelte";
    import {
        habitActions,
        habitsWithEntries,
        habitStats,
        habitError,
        isLoading,
    } from "../stores/habitStore";
    import { characterActions } from "../stores/characterStore";
    import { updateQuestProgress } from "../stores/questStore";
    import type { HabitWithEntry } from "../types/habit";

    // Reactive state
    $: loading = $isLoading;
    $: error = $habitError;
    $: habitsData = $habitsWithEntries;
    $: stats = $habitStats;

    // Initialize habits on mount with delay and error handling
    onMount(async () => {
        try {
            // Small delay to ensure Tauri is ready
            await new Promise((resolve) => setTimeout(resolve, 500));
            await habitActions.initialize();
        } catch (error) {
            console.error("Failed to initialize habits on mount:", error);
            // Don't crash the app, just log the error
        }
    });

    // Event handlers
    async function handleBooleanHabit(habitWithEntry: HabitWithEntry) {
        const wasCompleted = habitWithEntry.completed_today;
        const newStatus = !wasCompleted;

        await habitActions.markHabitToday(habitWithEntry.habit.id, newStatus);

        // Jeśli nawyk został oznaczony jako wykonany, odśwież character store
        if (!wasCompleted && newStatus) {
            await characterActions.getCharacter();

            // Aktualizuj postęp questów po ukończeniu nawyku
            await updateQuestProgress();

            console.log(
                "🎮 Character refreshed after habit completion:",
                habitWithEntry.habit.title,
            );
        }
    }

    async function handleCounterChange(
        habitWithEntry: HabitWithEntry,
        newValue: number,
    ) {
        if (newValue < 0) newValue = 0;
        if (newValue > 999) newValue = 999; // reasonable limit

        const oldValue = habitWithEntry.today_value;

        await habitActions.markHabitToday(
            habitWithEntry.habit.id,
            undefined,
            newValue,
        );

        // Sprawdź czy nawyk został ukończony (osiągnięta wartość docelowa)
        const targetValue = habitWithEntry.habit.target_value;
        const wasCompleted = targetValue
            ? oldValue >= targetValue
            : oldValue > 0;
        const isNowCompleted = targetValue
            ? newValue >= targetValue
            : newValue > 0;

        // Jeśli nawyk przeszedł z nieukończonego na ukończony, odśwież character
        if (!wasCompleted && isNowCompleted) {
            await characterActions.getCharacter();

            // Aktualizuj postęp questów po ukończeniu nawyku
            await updateQuestProgress();

            console.log(
                "🎮 Character refreshed after habit completion:",
                habitWithEntry.habit.title,
            );
        }
    }

    async function deleteHabit(habitId: number, habitTitle: string) {
        if (
            confirm(
                `Are you sure you want to delete "${habitTitle}"? This action cannot be undone.`,
            )
        ) {
            await habitActions.deleteHabit(habitId);
        }
    }

    function getStreakEmoji(streak: number): string {
        if (streak === 0) return "⚪";
        if (streak < 7) return "🔥";
        if (streak < 30) return "💪";
        if (streak < 100) return "⭐";
        return "👑";
    }

    function formatDateShort(): string {
        return new Date().toLocaleDateString("en-US", {
            weekday: "short",
            month: "short",
            day: "numeric",
        });
    }
</script>

<div class="habit-tracker-container">
    <!-- Header with Stats -->
    <div class="glass-card p-6 mb-6">
        <div class="flex items-center justify-between mb-4">
            <h2
                class="text-2xl font-bold text-white flex items-center gap-2"
                style="text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.8);"
            >
                <span class="text-3xl">📊</span>
                Today's Habits
            </h2>
            <div class="flex items-center gap-2">
                <button
                    on:click={() => habitActions.initialize()}
                    class="text-xs text-white/90 hover:text-white px-2 py-1 rounded"
                    style="text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.8);"
                    title="Reload habits"
                >
                    🔄
                </button>
                <div
                    class="text-sm text-white/90"
                    style="text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.8);"
                >
                    {formatDateShort()}
                </div>
            </div>
        </div>

        <!-- Stats Row -->
        {#if stats.total > 0}
            <div class="stats-grid">
                <div class="stat-item">
                    <div class="stat-value stat-total">{stats.total}</div>
                    <div class="stat-label">Total Habits</div>
                </div>
                <div class="stat-item">
                    <div class="stat-value stat-active">
                        {stats.with_streak}
                    </div>
                    <div class="stat-label">Active Streaks</div>
                </div>
                <div class="stat-item">
                    <div class="stat-value stat-average">
                        {stats.average_streak.toFixed(1)}
                    </div>
                    <div class="stat-label">Avg Streak</div>
                </div>
                <div class="stat-item">
                    <div class="stat-value stat-best">
                        {stats.longest_streak}
                    </div>
                    <div class="stat-label">Best Streak</div>
                </div>
            </div>
        {/if}
    </div>

    <!-- Error Display -->
    {#if error}
        <div class="error-alert mb-6">
            <div class="flex items-center gap-2">
                <span class="text-red-400">⚠️</span>
                <span class="text-red-300">{error.message}</span>
                <button
                    on:click={() => habitActions.clearError()}
                    class="ml-auto text-red-300 hover:text-red-100 transition-colors"
                    aria-label="Clear error"
                >
                    ✕
                </button>
            </div>
        </div>
    {/if}

    <!-- Loading State -->
    {#if loading && habitsData.length === 0}
        <div class="glass-card p-8">
            <div class="loading-container">
                <div class="loading-spinner large"></div>
                <p class="text-white/90 mt-4">Loading your habits...</p>
            </div>
        </div>
    {/if}

    <!-- Empty State -->
    {#if !loading && habitsData.length === 0}
        <div class="glass-card p-8">
            <div class="empty-state">
                <div class="text-6xl mb-4">🌱</div>
                <h3 class="text-xl font-bold text-white mb-2">
                    No habits yet!
                </h3>
                <p class="text-white/90 text-center max-w-md">
                    Start building better habits by adding your first one above.
                    Small daily actions lead to big transformations.
                </p>
            </div>
        </div>
    {/if}

    <!-- Habits List -->
    {#if habitsData.length > 0}
        <div class="habits-grid">
            {#each habitsData as habitWithEntry (habitWithEntry.habit.id)}
                <div class="habit-card glass-card">
                    <div class="habit-header">
                        <h3 class="habit-title">
                            {habitWithEntry.habit.title}
                        </h3>
                        <div class="habit-controls">
                            <div class="streak-indicator">
                                <span class="streak-emoji"
                                    >{getStreakEmoji(
                                        habitWithEntry.habit.current_streak,
                                    )}</span
                                >
                                <span class="streak-number"
                                    >{habitWithEntry.habit.current_streak}</span
                                >
                            </div>
                            <button
                                on:click={() =>
                                    deleteHabit(
                                        habitWithEntry.habit.id,
                                        habitWithEntry.habit.title,
                                    )}
                                class="delete-button"
                                aria-label="Delete habit"
                                title="Delete habit"
                            >
                                🗑️
                            </button>
                        </div>
                    </div>

                    <div class="habit-content">
                        {#if habitWithEntry.habit.habit_type === "Boolean"}
                            <!-- Boolean Habit -->
                            <button
                                on:click={() =>
                                    handleBooleanHabit(habitWithEntry)}
                                class="boolean-button"
                                class:completed={habitWithEntry.completed_today}
                                disabled={loading}
                            >
                                <div class="checkbox">
                                    {#if habitWithEntry.completed_today}
                                        <span class="checkmark">✓</span>
                                    {/if}
                                </div>
                                <span class="boolean-text">
                                    {habitWithEntry.completed_today
                                        ? "Completed today!"
                                        : "Mark as done"}
                                </span>
                            </button>
                        {:else}
                            <!-- Counter Habit -->
                            <div class="counter-container">
                                <div class="counter-display">
                                    <input
                                        type="number"
                                        min="0"
                                        max="999"
                                        value={habitWithEntry.today_value}
                                        on:input={(e) => {
                                            const newValue =
                                                parseInt(
                                                    e.currentTarget.value,
                                                ) || 0;
                                            handleCounterChange(
                                                habitWithEntry,
                                                newValue,
                                            );
                                        }}
                                        class="counter-input"
                                        disabled={loading}
                                    />
                                    {#if habitWithEntry.habit.target_value}
                                        <span class="counter-target"
                                            >/ {habitWithEntry.habit
                                                .target_value}</span
                                        >
                                    {/if}
                                </div>

                                <div class="counter-progress">
                                    {#if habitWithEntry.habit.target_value}
                                        {@const progress = Math.min(
                                            (habitWithEntry.today_value /
                                                habitWithEntry.habit
                                                    .target_value) *
                                                100,
                                            100,
                                        )}
                                        <div class="progress-bar">
                                            <div
                                                class="progress-fill"
                                                style="width: {progress}%"
                                            ></div>
                                        </div>
                                        <span class="progress-text"
                                            >{progress.toFixed(0)}%</span
                                        >
                                    {:else}
                                        <span class="counter-simple"
                                            >Count: {habitWithEntry.today_value}</span
                                        >
                                    {/if}
                                </div>
                            </div>
                        {/if}
                    </div>

                    <!-- Habit Type Badge -->
                    <div class="habit-badge">
                        {habitWithEntry.habit.habit_type === "Boolean"
                            ? "✓ Yes/No"
                            : "# Counter"}
                    </div>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .habit-tracker-container {
        width: 100%;
        max-width: 1200px;
    }

    .glass-card {
        background: rgba(15, 23, 42, 0.85);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 16px;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    }

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
        gap: 1rem;
    }

    .stat-item {
        text-align: center;
        padding: 1rem;
        background: rgba(255, 255, 255, 0.05);
        border-radius: 8px;
    }

    .stat-value {
        font-size: 1.5rem;
        font-weight: bold;
        color: white;
    }

    .stat-total {
        color: #60a5fa; /* blue */
    }

    .stat-active {
        color: #34d399; /* green */
    }

    .stat-average {
        color: #fbbf24; /* yellow */
    }

    .stat-best {
        color: #f87171; /* red */
    }

    .stat-label {
        font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.9);
        margin-top: 0.25rem;
    }

    .loading-container,
    .empty-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
    }

    .loading-spinner {
        width: 24px;
        height: 24px;
        border: 3px solid rgba(255, 255, 255, 0.3);
        border-top: 3px solid white;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    .loading-spinner.large {
        width: 32px;
        height: 32px;
        border-width: 4px;
    }

    .habits-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 1.5rem;
    }

    .habit-card {
        padding: 1.5rem;
        transition: all 0.2s ease;
        position: relative;
        overflow: hidden;
        min-width: 0; /* Allow flex children to shrink */
    }

    .habit-card:hover {
        transform: translateY(-2px);
        box-shadow: 0 12px 40px rgba(0, 0, 0, 0.4);
    }

    .habit-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 1rem;
    }

    .habit-title {
        font-size: 1.125rem;
        font-weight: 600;
        color: white;
        flex: 1;
        margin-right: 1rem;
        line-height: 1.3;
        /* Handle long text properly */
        word-wrap: break-word;
        word-break: break-word;
        overflow-wrap: break-word;
        hyphens: auto;
        max-width: 100%;
    }

    .habit-controls {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .streak-indicator {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        background: rgba(255, 255, 255, 0.1);
        padding: 0.25rem 0.5rem;
        border-radius: 12px;
        font-size: 0.875rem;
    }

    .streak-emoji {
        font-size: 1rem;
    }

    .streak-number {
        color: #a855f7;
        font-weight: 600;
    }

    .delete-button {
        background: none;
        border: none;
        cursor: pointer;
        padding: 0.25rem;
        border-radius: 4px;
        opacity: 0.6;
        transition: opacity 0.2s ease;
    }

    .delete-button:hover {
        opacity: 1;
        background: rgba(239, 68, 68, 0.2);
    }

    .habit-content {
        margin-bottom: 1rem;
    }

    .boolean-button {
        width: 100%;
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 1rem;
        background: rgba(255, 255, 255, 0.05);
        border: 2px solid rgba(255, 255, 255, 0.2);
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s ease;
        color: white;
    }

    .boolean-button:hover {
        background: rgba(255, 255, 255, 0.1);
        border-color: rgba(124, 58, 237, 0.5);
    }

    .boolean-button.completed {
        background: rgba(34, 197, 94, 0.2);
        border-color: rgba(34, 197, 94, 0.5);
    }

    .checkbox {
        width: 24px;
        height: 24px;
        border: 2px solid rgba(255, 255, 255, 0.4);
        border-radius: 4px;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        transition: all 0.2s ease;
    }

    .boolean-button.completed .checkbox {
        background: #22c55e;
        border-color: #22c55e;
    }

    .checkmark {
        color: white;
        font-weight: bold;
        font-size: 16px;
    }

    .boolean-text {
        font-weight: 500;
    }

    .counter-container {
        gap: 1rem;
    }

    .counter-display {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-bottom: 0.75rem;
    }

    .counter-input {
        padding: 0.5rem 0.75rem;
        background: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.3);
        border-radius: 6px;
        color: white;
        font-size: 1.125rem;
        font-weight: 600;
        width: 80px;
        text-align: center;
    }

    .counter-input:focus {
        outline: none;
        border-color: rgba(124, 58, 237, 0.8);
        box-shadow: 0 0 0 2px rgba(124, 58, 237, 0.3);
    }

    .counter-target {
        color: rgba(255, 255, 255, 0.85);
        font-size: 1rem;
    }

    .counter-progress {
        display: flex;
        align-items: center;
        gap: 0.75rem;
    }

    .progress-bar {
        flex: 1;
        height: 8px;
        background: rgba(255, 255, 255, 0.2);
        border-radius: 4px;
        overflow: hidden;
    }

    .progress-fill {
        height: 100%;
        background: linear-gradient(90deg, #7c3aed, #a855f7);
        transition: width 0.3s ease;
        border-radius: 4px;
    }

    .progress-text {
        font-size: 0.875rem;
        color: rgba(255, 255, 255, 0.8);
        font-weight: 500;
        min-width: 40px;
    }

    .counter-simple {
        color: rgba(255, 255, 255, 0.8);
        font-size: 0.875rem;
    }

    .habit-badge {
        position: absolute;
        top: 0.75rem;
        right: 0.75rem;
        font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.9);
        background: rgba(255, 255, 255, 0.1);
        padding: 0.25rem 0.5rem;
        border-radius: 8px;
        opacity: 0;
        transition: opacity 0.2s ease;
    }

    .habit-card:hover .habit-badge {
        opacity: 1;
    }

    .error-alert {
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.3);
        border-radius: 8px;
        padding: 0.75rem;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    /* Mobile responsiveness */
    @media (max-width: 640px) {
        .habits-grid {
            grid-template-columns: 1fr;
        }

        .stats-grid {
            grid-template-columns: repeat(2, 1fr);
        }

        .habit-card {
            padding: 1rem;
        }

        .habit-title {
            font-size: 1rem;
        }
    }
</style>
