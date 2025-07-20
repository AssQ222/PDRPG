<script lang="ts">
    import {
        tasks,
        isLoading,
        taskError,
        taskActions,
        taskStats,
    } from "../stores/taskStore";
    import {
        characterActions,
        levelUpNotifications,
    } from "../stores/characterStore";
    import type { Task } from "../types/task";

    /**
     * Formatuje timestamp do czytelnej daty
     */
    function formatDate(timestamp: number): string {
        return new Date(timestamp * 1000).toLocaleDateString("pl-PL", {
            year: "numeric",
            month: "short",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
        });
    }

    /**
     * Obsługuje przełączenie statusu zadania
     */
    async function handleToggleTask(taskId: number) {
        // Znajdź zadanie przed toggle
        const task = $tasks.find((t) => t.id === taskId);
        const wasCompleted = task?.completed || false;

        // Toggle zadania
        await taskActions.toggleTaskStatus(taskId);

        // Jeśli zadanie zostało ukończone (przeszło z false na true), odśwież character
        if (!wasCompleted && task) {
            const updatedTask = $tasks.find((t) => t.id === taskId);
            if (updatedTask?.completed) {
                // Odśwież character store aby pobrać nowy EXP i poziom
                await characterActions.getCharacter();
                console.log(
                    "🎮 Character refreshed after task completion:",
                    updatedTask.title,
                );
            }
        }
    }

    /**
     * Obsługuje usunięcie zadania
     */
    async function handleDeleteTask(taskId: number) {
        if (confirm("Czy na pewno chcesz usunąć to zadanie?")) {
            await taskActions.deleteTask(taskId);
        }
    }

    /**
     * Obsługuje naciśnięcie klawisza na zadaniu
     */
    function handleTaskKeydown(event: KeyboardEvent, taskId: number) {
        if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            handleToggleTask(taskId);
        }
    }
</script>

<div class="task-list-container">
    <!-- Statystyki zadań -->
    <div class="task-stats">
        <div class="stat-item">
            <span class="stat-label">Wszystkie:</span>
            <span class="stat-value">{$taskStats.total}</span>
        </div>
        <div class="stat-item">
            <span class="stat-label">Do zrobienia:</span>
            <span class="stat-value pending">{$taskStats.pending}</span>
        </div>
        <div class="stat-item">
            <span class="stat-label">Ukończone:</span>
            <span class="stat-value completed">{$taskStats.completed}</span>
        </div>
    </div>

    <!-- Lista zadań -->
    {#if $isLoading}
        <div class="loading-container">
            <div class="loading-spinner"></div>
            <span>Ładowanie zadań...</span>
        </div>
    {:else if $taskError}
        <div class="error-container" role="alert">
            <span class="error-icon">⚠️</span>
            <span>{$taskError.message}</span>
            <button
                on:click={taskActions.clearError}
                class="clear-error-button"
            >
                Zamknij
            </button>
        </div>
    {:else if $tasks.length === 0}
        <div class="empty-state">
            <div class="empty-icon">📝</div>
            <h3>Brak zadań</h3>
            <p>Dodaj swoje pierwsze zadanie używając pola powyżej!</p>
        </div>
    {:else}
        <div class="task-list" role="list">
            {#each $tasks as task (task.id)}
                <div
                    class="task-item {task.completed ? 'completed' : 'pending'}"
                    role="listitem"
                >
                    <!-- Checkbox do toggle -->
                    <button
                        class="task-checkbox"
                        on:click={() => handleToggleTask(task.id)}
                        on:keydown={(e) => handleTaskKeydown(e, task.id)}
                        aria-label={task.completed
                            ? "Oznacz jako niezrobione"
                            : "Oznacz jako zrobione"}
                        tabindex="0"
                    >
                        {#if task.completed}
                            <span class="checkmark">✓</span>
                        {:else}
                            <span class="empty-check"></span>
                        {/if}
                    </button>

                    <!-- Treść zadania -->
                    <div class="task-content">
                        <h4 class="task-title">{task.title}</h4>
                        <div class="task-meta">
                            <span class="task-date">
                                Utworzono: {formatDate(task.created_at)}
                            </span>
                            {#if task.updated_at !== task.created_at}
                                <span class="task-date">
                                    Zaktualizowano: {formatDate(
                                        task.updated_at,
                                    )}
                                </span>
                            {/if}
                        </div>
                    </div>

                    <!-- Przycisk usuwania -->
                    <button
                        class="delete-button"
                        on:click={() => handleDeleteTask(task.id)}
                        aria-label="Usuń zadanie"
                        title="Usuń zadanie"
                    >
                        <span class="delete-icon">🗑️</span>
                    </button>
                </div>
            {/each}
        </div>
    {/if}
</div>

<style>
    .task-list-container {
        width: 100%;
    }

    .task-stats {
        display: flex;
        gap: 1rem;
        margin-bottom: 1.5rem;
        padding: 1rem;
        background-color: #f8fafc;
        border-radius: 0.5rem;
        border: 1px solid #e2e8f0;
    }

    .stat-item {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 0.25rem;
    }

    .stat-label {
        font-size: 0.875rem;
        color: #64748b;
        font-weight: 500;
    }

    .stat-value {
        font-size: 1.5rem;
        font-weight: bold;
        color: #1e293b;
    }

    .stat-value.pending {
        color: #f59e0b;
    }

    .stat-value.completed {
        color: #10b981;
    }

    .loading-container {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.75rem;
        padding: 2rem;
        color: #64748b;
    }

    .loading-spinner {
        width: 1.5rem;
        height: 1.5rem;
        border: 2px solid #e2e8f0;
        border-top: 2px solid #3b82f6;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    .error-container {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 1rem;
        background-color: #fef2f2;
        border: 1px solid #fecaca;
        border-radius: 0.5rem;
        color: #dc2626;
    }

    .error-icon {
        font-size: 1.25rem;
    }

    .clear-error-button {
        margin-left: auto;
        padding: 0.25rem 0.75rem;
        background-color: #dc2626;
        color: white;
        border: none;
        border-radius: 0.25rem;
        font-size: 0.875rem;
        cursor: pointer;
        transition: background-color 0.2s;
    }

    .clear-error-button:hover {
        background-color: #b91c1c;
    }

    .empty-state {
        text-align: center;
        padding: 3rem 1rem;
        color: #64748b;
    }

    .empty-icon {
        font-size: 3rem;
        margin-bottom: 1rem;
    }

    .empty-state h3 {
        margin: 0 0 0.5rem 0;
        font-size: 1.25rem;
        color: #374151;
    }

    .empty-state p {
        margin: 0;
        font-size: 1rem;
    }

    .task-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .task-item {
        display: flex;
        align-items: flex-start;
        gap: 0.75rem;
        padding: 1rem;
        background-color: white;
        border: 1px solid #e2e8f0;
        border-radius: 0.5rem;
        transition: all 0.2s ease;
    }

    .task-item:hover {
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        border-color: #cbd5e1;
    }

    .task-item.completed {
        background-color: #f0f9ff;
        border-color: #bae6fd;
    }

    .task-checkbox {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 1.5rem;
        height: 1.5rem;
        border: 2px solid #d1d5db;
        border-radius: 0.25rem;
        background-color: white;
        cursor: pointer;
        transition: all 0.2s ease;
        flex-shrink: 0;
        margin-top: 0.125rem;
    }

    .task-checkbox:hover {
        border-color: #3b82f6;
        background-color: #eff6ff;
    }

    .task-checkbox:focus {
        outline: none;
        ring: 2px solid #3b82f6;
        ring-offset: 2px;
    }

    .completed .task-checkbox {
        background-color: #10b981;
        border-color: #10b981;
    }

    .checkmark {
        color: white;
        font-weight: bold;
        font-size: 0.875rem;
    }

    .empty-check {
        width: 0.75rem;
        height: 0.75rem;
    }

    .task-content {
        flex: 1;
        min-width: 0;
    }

    .task-title {
        margin: 0 0 0.5rem 0;
        font-size: 1rem;
        font-weight: 500;
        color: #1f2937;
        word-wrap: break-word;
        transition: all 0.2s ease;
    }

    .completed .task-title {
        text-decoration: line-through;
        color: #6b7280;
    }

    .task-meta {
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .task-date {
        font-size: 0.75rem;
        color: #9ca3af;
    }

    .delete-button {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 2rem;
        height: 2rem;
        background-color: transparent;
        border: none;
        border-radius: 0.25rem;
        cursor: pointer;
        transition: all 0.2s ease;
        flex-shrink: 0;
        opacity: 0.6;
    }

    .delete-button:hover {
        background-color: #fef2f2;
        opacity: 1;
        transform: scale(1.1);
    }

    .delete-button:active {
        transform: scale(0.95);
    }

    .delete-icon {
        font-size: 1rem;
    }

    /* Responsive design */
    @media (max-width: 640px) {
        .task-stats {
            flex-direction: column;
            gap: 0.5rem;
        }

        .stat-item {
            flex-direction: row;
            justify-content: space-between;
        }

        .task-item {
            padding: 0.75rem;
        }

        .task-meta {
            font-size: 0.875rem;
        }
    }
</style>
