<script lang="ts">
    import {
        habits,
        habitStats,
        habitsWithEntries,
    } from "$lib/stores/habitStore";

    // Get the top 3 habits with highest streaks
    $: topStreaks = $habits
        .filter((habit) => habit.current_streak > 0)
        .sort((a, b) => b.current_streak - a.current_streak)
        .slice(0, 3);

    // Get habits completed today
    $: completedToday = $habitsWithEntries.filter(
        (hw) => hw.completed_today,
    ).length;

    // Get emoji for streak milestone
    function getStreakEmoji(streak: number): string {
        if (streak >= 100) return "🔥🔥🔥";
        if (streak >= 50) return "🔥🔥";
        if (streak >= 30) return "🔥";
        if (streak >= 7) return "⭐";
        if (streak >= 3) return "🌟";
        return "✨";
    }

    // Get streak color based on length
    function getStreakColor(streak: number): string {
        if (streak >= 50) return "#dc2626"; // red-600
        if (streak >= 30) return "#ea580c"; // orange-600
        if (streak >= 14) return "#d97706"; // amber-600
        if (streak >= 7) return "#ca8a04"; // yellow-600
        return "#16a34a"; // green-600
    }
</script>

<div class="habit-streaks-widget">
    <div class="widget-header">
        <h3 class="widget-title">🎯 Nawyki & Streaki</h3>
        {#if completedToday > 0}
            <div class="today-badge">
                {completedToday} dzisiaj
            </div>
        {/if}
    </div>

    <div class="stats-overview">
        <div class="overview-item">
            <div class="overview-number">{$habitStats.total}</div>
            <div class="overview-label">Wszystkich nawyków</div>
        </div>

        <div class="overview-item">
            <div class="overview-number">{$habitStats.with_streak}</div>
            <div class="overview-label">Z aktywnym streak</div>
        </div>

        <div class="overview-item">
            <div class="overview-number">
                {$habitStats.average_streak.toFixed(1)}
            </div>
            <div class="overview-label">Średni streak</div>
        </div>
    </div>

    {#if topStreaks.length > 0}
        <div class="streaks-section">
            <h4 class="section-title">🏆 Najlepsze Streaki</h4>
            <div class="streaks-list">
                {#each topStreaks as habit, index}
                    <div class="streak-item" class:first={index === 0}>
                        <div class="streak-icon">
                            {getStreakEmoji(habit.current_streak)}
                        </div>
                        <div class="streak-content">
                            <div class="streak-title">{habit.title}</div>
                            <div
                                class="streak-days"
                                style="color: {getStreakColor(
                                    habit.current_streak,
                                )}"
                            >
                                {habit.current_streak}
                                {habit.current_streak === 1 ? "dzień" : "dni"} z
                                rzędu
                            </div>
                        </div>
                        <div class="streak-position">
                            #{index + 1}
                        </div>
                    </div>
                {/each}
            </div>
        </div>
    {:else if $habits.length > 0}
        <div class="no-streaks">
            <div class="no-streaks-icon">💪</div>
            <div class="no-streaks-text">
                Zacznij budować streaki!<br />
                <small>Oznacz nawyki jako wykonane dzisiaj</small>
            </div>
        </div>
    {:else}
        <div class="no-habits">
            <div class="no-habits-icon">🎯</div>
            <div class="no-habits-text">
                Brak nawyków<br />
                <small>Dodaj swój pierwszy nawyk</small>
            </div>
        </div>
    {/if}

    {#if $habitStats.longest_streak > 0}
        <div class="milestone-section">
            <div class="milestone-badge">
                <span class="milestone-icon">🏅</span>
                <span class="milestone-text">
                    Najdłuższy streak: <strong
                        >{$habitStats.longest_streak} dni</strong
                    >
                </span>
            </div>
        </div>
    {/if}
</div>

<style>
    .habit-streaks-widget {
        background: rgba(255, 255, 255, 0.95);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 1rem;
        padding: 1.5rem;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
        height: 100%;
        display: flex;
        flex-direction: column;
    }

    .widget-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 1.5rem;
    }

    .widget-title {
        margin: 0;
        font-size: 1.25rem;
        font-weight: 600;
        color: #1e293b;
    }

    .today-badge {
        background: #10b981;
        color: white;
        padding: 0.25rem 0.75rem;
        border-radius: 1rem;
        font-size: 0.875rem;
        font-weight: 600;
    }

    .stats-overview {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .overview-item {
        text-align: center;
        padding: 1rem;
        background: #f8fafc;
        border-radius: 0.75rem;
        transition: transform 0.2s ease;
    }

    .overview-item:hover {
        transform: translateY(-2px);
    }

    .overview-number {
        font-size: 1.5rem;
        font-weight: 700;
        color: #1e293b;
        margin-bottom: 0.25rem;
    }

    .overview-label {
        font-size: 0.75rem;
        color: #64748b;
        font-weight: 500;
    }

    .streaks-section {
        flex: 1;
    }

    .section-title {
        margin: 0 0 1rem 0;
        font-size: 1rem;
        font-weight: 600;
        color: #374151;
    }

    .streaks-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .streak-item {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 1rem;
        background: #f8fafc;
        border-radius: 0.75rem;
        transition: all 0.2s ease;
        border: 2px solid transparent;
    }

    .streak-item:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }

    .streak-item.first {
        background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
        color: white;
        border-color: #d97706;
    }

    .streak-item.first .streak-title {
        color: white;
    }

    .streak-icon {
        font-size: 1.5rem;
        flex-shrink: 0;
    }

    .streak-content {
        flex: 1;
    }

    .streak-title {
        font-weight: 600;
        color: #1e293b;
        margin-bottom: 0.25rem;
        font-size: 0.9rem;
    }

    .streak-days {
        font-size: 0.875rem;
        font-weight: 600;
    }

    .streak-position {
        font-size: 0.875rem;
        font-weight: 600;
        color: #64748b;
        background: rgba(255, 255, 255, 0.8);
        padding: 0.25rem 0.5rem;
        border-radius: 0.5rem;
    }

    .streak-item.first .streak-position {
        background: rgba(255, 255, 255, 0.9);
        color: #92400e;
    }

    .no-streaks,
    .no-habits {
        display: flex;
        flex-direction: column;
        align-items: center;
        text-align: center;
        padding: 2rem;
        flex: 1;
        justify-content: center;
    }

    .no-streaks-icon,
    .no-habits-icon {
        font-size: 3rem;
        margin-bottom: 1rem;
        opacity: 0.6;
    }

    .no-streaks-text,
    .no-habits-text {
        color: #64748b;
        font-weight: 500;
    }

    .no-streaks-text small,
    .no-habits-text small {
        display: block;
        margin-top: 0.5rem;
        opacity: 0.8;
    }

    .milestone-section {
        margin-top: 1.5rem;
        padding-top: 1rem;
        border-top: 1px solid #e2e8f0;
    }

    .milestone-badge {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.75rem;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        border-radius: 0.75rem;
        font-size: 0.875rem;
    }

    .milestone-icon {
        font-size: 1.125rem;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .habit-streaks-widget {
            background: rgba(15, 23, 42, 0.95);
            border: 1px solid rgba(71, 85, 105, 0.3);
        }

        .widget-title {
            color: #f1f5f9;
        }

        .section-title {
            color: #cbd5e1;
        }

        .overview-item,
        .streak-item {
            background: #1e293b;
        }

        .overview-number,
        .streak-title {
            color: #f1f5f9;
        }

        .overview-label {
            color: #94a3b8;
        }

        .streak-position {
            background: rgba(71, 85, 105, 0.8);
            color: #cbd5e1;
        }

        .no-streaks-text,
        .no-habits-text {
            color: #94a3b8;
        }

        .milestone-section {
            border-top-color: #374151;
        }
    }

    /* Responsive design */
    @media (max-width: 640px) {
        .stats-overview {
            grid-template-columns: repeat(2, 1fr);
            gap: 0.75rem;
        }

        .overview-item {
            padding: 0.75rem;
        }

        .overview-number {
            font-size: 1.25rem;
        }

        .overview-label {
            font-size: 0.7rem;
        }

        .streak-item {
            gap: 0.75rem;
            padding: 0.75rem;
        }

        .streak-icon {
            font-size: 1.25rem;
        }

        .streak-title {
            font-size: 0.8rem;
        }

        .streak-days {
            font-size: 0.75rem;
        }
    }
</style>
