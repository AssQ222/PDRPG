<script lang="ts">
    import {
        taskStats,
        pendingTasks,
        completedTasks,
    } from "$lib/stores/taskStore";

    // Get today's date for filtering today's tasks
    const today = new Date().toDateString();

    // Filter tasks created today (approximation)
    $: todayTasks = $pendingTasks.filter((task) => {
        const taskDate = new Date(task.created_at * 1000).toDateString();
        return taskDate === today;
    });

    $: todayCompleted = $completedTasks.filter((task) => {
        const taskDate = new Date(task.created_at * 1000).toDateString();
        return taskDate === today;
    });

    $: completionRate =
        $taskStats.total > 0
            ? Math.round(($taskStats.completed / $taskStats.total) * 100)
            : 0;
</script>

<div class="task-summary-widget">
    <div class="widget-header">
        <h3 class="widget-title">📝 Zadania</h3>
        <div
            class="completion-badge"
            class:high={completionRate >= 70}
            class:medium={completionRate >= 40 && completionRate < 70}
        >
            {completionRate}%
        </div>
    </div>

    <div class="stats-grid">
        <div class="stat-card total">
            <div class="stat-number">{$taskStats.total}</div>
            <div class="stat-label">Wszystkich</div>
        </div>

        <div class="stat-card pending">
            <div class="stat-number">{$taskStats.pending}</div>
            <div class="stat-label">Do zrobienia</div>
        </div>

        <div class="stat-card completed">
            <div class="stat-number">{$taskStats.completed}</div>
            <div class="stat-label">Ukończonych</div>
        </div>
    </div>

    <div class="today-section">
        <h4 class="today-title">Dzisiaj</h4>
        <div class="today-stats">
            <div class="today-item">
                <span class="today-icon">➕</span>
                <span class="today-text">{todayTasks.length} nowych zadań</span>
            </div>
            <div class="today-item">
                <span class="today-icon">✅</span>
                <span class="today-text"
                    >{todayCompleted.length} ukończonych</span
                >
            </div>
        </div>
    </div>

    {#if $taskStats.pending > 0}
        <div class="progress-section">
            <div class="progress-bar">
                <div
                    class="progress-fill"
                    style="width: {completionRate}%"
                ></div>
            </div>
            <div class="progress-text">
                {$taskStats.pending} zadań do ukończenia
            </div>
        </div>
    {/if}
</div>

<style>
    .task-summary-widget {
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

    .completion-badge {
        background: #e2e8f0;
        color: #64748b;
        padding: 0.25rem 0.75rem;
        border-radius: 1rem;
        font-size: 0.875rem;
        font-weight: 600;
        transition: all 0.2s ease;
    }

    .completion-badge.medium {
        background: #fbbf24;
        color: #92400e;
    }

    .completion-badge.high {
        background: #10b981;
        color: #065f46;
    }

    .stats-grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1rem;
        margin-bottom: 1.5rem;
    }

    .stat-card {
        text-align: center;
        padding: 1rem;
        border-radius: 0.75rem;
        transition: transform 0.2s ease;
    }

    .stat-card:hover {
        transform: translateY(-2px);
    }

    .stat-card.total {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
    }

    .stat-card.pending {
        background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
        color: white;
    }

    .stat-card.completed {
        background: linear-gradient(135deg, #10b981 0%, #059669 100%);
        color: white;
    }

    .stat-number {
        font-size: 2rem;
        font-weight: 700;
        line-height: 1;
        margin-bottom: 0.25rem;
    }

    .stat-label {
        font-size: 0.875rem;
        opacity: 0.9;
        font-weight: 500;
    }

    .today-section {
        margin-bottom: 1.5rem;
    }

    .today-title {
        margin: 0 0 1rem 0;
        font-size: 1rem;
        font-weight: 600;
        color: #374151;
    }

    .today-stats {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .today-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.5rem;
        background: #f8fafc;
        border-radius: 0.5rem;
    }

    .today-icon {
        font-size: 1.125rem;
    }

    .today-text {
        font-size: 0.875rem;
        color: #64748b;
        font-weight: 500;
    }

    .progress-section {
        margin-top: auto;
    }

    .progress-bar {
        background: #e2e8f0;
        border-radius: 1rem;
        height: 0.5rem;
        overflow: hidden;
        margin-bottom: 0.5rem;
    }

    .progress-fill {
        height: 100%;
        background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
        border-radius: 1rem;
        transition: width 0.3s ease;
    }

    .progress-text {
        text-align: center;
        font-size: 0.75rem;
        color: #64748b;
        font-weight: 500;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .task-summary-widget {
            background: rgba(15, 23, 42, 0.95);
            border: 1px solid rgba(71, 85, 105, 0.3);
        }

        .widget-title {
            color: #f1f5f9;
        }

        .today-title {
            color: #cbd5e1;
        }

        .today-item {
            background: #1e293b;
        }

        .today-text {
            color: #94a3b8;
        }

        .completion-badge {
            background: #374151;
            color: #9ca3af;
        }

        .progress-bar {
            background: #374151;
        }

        .progress-text {
            color: #9ca3af;
        }
    }

    /* Responsive design */
    @media (max-width: 640px) {
        .stats-grid {
            grid-template-columns: repeat(2, 1fr);
            gap: 0.75rem;
        }

        .stat-card {
            padding: 0.75rem;
        }

        .stat-number {
            font-size: 1.5rem;
        }

        .stat-label {
            font-size: 0.75rem;
        }
    }
</style>
