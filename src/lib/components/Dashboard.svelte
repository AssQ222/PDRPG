<script lang="ts">
    import { onMount } from "svelte";
    import TaskSummaryWidget from "./TaskSummaryWidget.svelte";
    import HabitStreaksWidget from "./HabitStreaksWidget.svelte";
    import CharacterProgressWidget from "./CharacterProgressWidget.svelte";
    import QuestSummaryWidget from "./QuestSummaryWidget.svelte";
    import { taskActions } from "$lib/stores/taskStore";
    import { habitActions } from "$lib/stores/habitStore";
    import { characterActions } from "$lib/stores/characterStore";
    import { initializeQuestSystem } from "$lib/stores/questStore";

    // Props for navigation
    export let onNavigate: (view: string) => void = () => {};

    // Initialize all stores when component mounts
    onMount(async () => {
        await Promise.all([
            taskActions.loadTasks(),
            habitActions.initialize(),
            characterActions.initialize(),
            initializeQuestSystem(),
        ]);
    });

    function handleNavigateToTasks() {
        onNavigate("tasks");
    }

    function handleNavigateToHabits() {
        onNavigate("habits");
    }

    function handleNavigateToCharacter() {
        onNavigate("character");
    }

    function handleNavigateToQuests() {
        onNavigate("quests");
    }
</script>

<div class="dashboard">
    <header class="dashboard-header">
        <h1 class="dashboard-title">
            <span class="title-icon">📊</span>
            Dashboard
        </h1>
        <p class="dashboard-subtitle">
            Przegląd postępów i aktywności w Twojej podróży samorozwoju
        </p>
    </header>

    <div class="widgets-grid">
        <!-- Task Summary Widget -->
        <div class="widget-container">
            <TaskSummaryWidget />
            <div class="widget-footer">
                <button
                    class="widget-action-btn"
                    on:click={handleNavigateToTasks}
                >
                    <span class="btn-icon">📝</span>
                    Zarządzaj zadaniami
                </button>
            </div>
        </div>

        <!-- Habit Streaks Widget -->
        <div class="widget-container">
            <HabitStreaksWidget />
            <div class="widget-footer">
                <button
                    class="widget-action-btn"
                    on:click={handleNavigateToHabits}
                >
                    <span class="btn-icon">🎯</span>
                    Śledź nawyki
                </button>
            </div>
        </div>

        <!-- Character Progress Widget -->
        <div class="widget-container">
            <CharacterProgressWidget />
            <div class="widget-footer">
                <button
                    class="widget-action-btn"
                    on:click={handleNavigateToCharacter}
                >
                    <span class="btn-icon">⚔️</span>
                    Rozwój postaci
                </button>
            </div>
        </div>

        <!-- Quest Summary Widget -->
        <div class="widget-container">
            <QuestSummaryWidget />
            <div class="widget-footer">
                <button
                    class="widget-action-btn"
                    on:click={handleNavigateToQuests}
                >
                    <span class="btn-icon">🎯</span>
                    Zarządzaj questami
                </button>
            </div>
        </div>
    </div>

    <div class="quick-actions">
        <h3 class="quick-actions-title">⚡ Szybkie Akcje</h3>
        <div class="actions-grid">
            <button class="quick-action-card" on:click={handleNavigateToTasks}>
                <div class="action-icon">➕</div>
                <div class="action-text">
                    <div class="action-title">Dodaj zadanie</div>
                    <div class="action-subtitle">Nowy cel do osiągnięcia</div>
                </div>
            </button>

            <button class="quick-action-card" on:click={handleNavigateToHabits}>
                <div class="action-icon">🌟</div>
                <div class="action-text">
                    <div class="action-title">Nowy nawyk</div>
                    <div class="action-subtitle">
                        Buduj lepsze przyzwyczajenia
                    </div>
                </div>
            </button>

            <button
                class="quick-action-card"
                on:click={handleNavigateToCharacter}
            >
                <div class="action-icon">📈</div>
                <div class="action-text">
                    <div class="action-title">Zobacz postęp</div>
                    <div class="action-subtitle">Statystyki i osiągnięcia</div>
                </div>
            </button>

            <button class="quick-action-card" on:click={handleNavigateToQuests}>
                <div class="action-icon">🎯</div>
                <div class="action-text">
                    <div class="action-title">Questy tygodniowe</div>
                    <div class="action-subtitle">Wyzwania i odznaki</div>
                </div>
            </button>
        </div>
    </div>

    <footer class="dashboard-footer">
        <div class="footer-content">
            <div class="footer-text">
                🎮 Kontynuuj swoją podróż samorozwoju! Każde ukończone zadanie i
                nawyk to doświadczenie w Twojej osobistej grze RPG.
            </div>
            <div class="footer-tip">
                💡 <strong>Wskazówka:</strong> Regularne wykonywanie nawyków buduje
                potężne streaki i zwiększa Twój EXP!
            </div>
        </div>
    </footer>
</div>

<style>
    .dashboard {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        gap: 2rem;
    }

    .dashboard-header {
        text-align: center;
        background: rgba(255, 255, 255, 0.95);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 1rem;
        padding: 2rem;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    }

    .dashboard-title {
        margin: 0 0 0.5rem 0;
        font-size: 2.5rem;
        font-weight: 700;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.75rem;
    }

    .title-icon {
        font-size: 2rem;
        filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
    }

    .dashboard-subtitle {
        margin: 0;
        font-size: 1.125rem;
        color: #64748b;
        font-weight: 500;
    }

    .widgets-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
        gap: 2rem;
    }

    /* Layout optimization for 4 widgets */
    @media (min-width: 1200px) {
        .widgets-grid {
            grid-template-columns: repeat(2, 1fr);
        }
    }

    @media (min-width: 1600px) {
        .widgets-grid {
            grid-template-columns: repeat(4, 1fr);
        }
    }

    .widget-container {
        display: flex;
        flex-direction: column;
        background: transparent;
        border-radius: 1rem;
        overflow: hidden;
        min-height: 400px;
    }

    .widget-footer {
        margin-top: 1rem;
        padding: 0;
    }

    .widget-action-btn {
        width: 100%;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        border: none;
        padding: 0.75rem 1rem;
        border-radius: 0.75rem;
        font-size: 0.875rem;
        font-weight: 600;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .widget-action-btn:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(102, 126, 234, 0.3);
    }

    .btn-icon {
        font-size: 1rem;
    }

    .quick-actions {
        background: rgba(255, 255, 255, 0.95);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 1rem;
        padding: 2rem;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    }

    .quick-actions-title {
        margin: 0 0 1.5rem 0;
        font-size: 1.5rem;
        font-weight: 600;
        color: #1e293b;
        text-align: center;
    }

    .actions-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 1rem;
    }

    .quick-action-card {
        background: #f8fafc;
        border: 2px solid transparent;
        border-radius: 0.75rem;
        padding: 1.5rem;
        display: flex;
        align-items: center;
        gap: 1rem;
        cursor: pointer;
        transition: all 0.2s ease;
        text-align: left;
    }

    .quick-action-card:hover {
        background: #f1f5f9;
        border-color: #667eea;
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
    }

    .action-icon {
        font-size: 2rem;
        flex-shrink: 0;
    }

    .action-text {
        flex: 1;
    }

    .action-title {
        font-weight: 600;
        color: #1e293b;
        margin-bottom: 0.25rem;
    }

    .action-subtitle {
        font-size: 0.875rem;
        color: #64748b;
    }

    .dashboard-footer {
        background: rgba(255, 255, 255, 0.95);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 1rem;
        padding: 2rem;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
        text-align: center;
    }

    .footer-content {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .footer-text {
        font-size: 1rem;
        color: #64748b;
        font-weight: 500;
    }

    .footer-tip {
        font-size: 0.875rem;
        color: #16a34a;
        background: #f0fdf4;
        padding: 1rem;
        border-radius: 0.75rem;
        border: 1px solid #bbf7d0;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .dashboard-header,
        .quick-actions,
        .dashboard-footer {
            background: rgba(15, 23, 42, 0.95);
            border: 1px solid rgba(71, 85, 105, 0.3);
        }

        .dashboard-title {
            background: linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }

        .dashboard-subtitle {
            color: #94a3b8;
        }

        .quick-actions-title {
            color: #f1f5f9;
        }

        .quick-action-card {
            background: #1e293b;
            border-color: transparent;
        }

        .quick-action-card:hover {
            background: #334155;
            border-color: #60a5fa;
        }

        .action-title {
            color: #f1f5f9;
        }

        .action-subtitle {
            color: #94a3b8;
        }

        .footer-text {
            color: #94a3b8;
        }

        .footer-tip {
            color: #059669;
            background: #064e3b;
            border-color: #065f46;
        }
    }

    /* Responsive design */
    @media (max-width: 768px) {
        .widgets-grid {
            grid-template-columns: 1fr;
            gap: 1.5rem;
        }

        .dashboard-title {
            font-size: 2rem;
            flex-direction: column;
            gap: 0.5rem;
        }

        .title-icon {
            font-size: 1.75rem;
        }

        .dashboard-subtitle {
            font-size: 1rem;
        }

        .actions-grid {
            grid-template-columns: 1fr;
        }

        .quick-action-card {
            padding: 1rem;
        }

        .action-icon {
            font-size: 1.5rem;
        }

        .footer-content {
            gap: 0.75rem;
        }

        .footer-text {
            font-size: 0.875rem;
        }

        .footer-tip {
            font-size: 0.8rem;
            padding: 0.75rem;
        }
    }

    @media (max-width: 480px) {
        .dashboard-header,
        .quick-actions,
        .dashboard-footer {
            padding: 1.5rem;
        }

        .widget-container {
            min-height: 350px;
        }

        .dashboard-title {
            font-size: 1.75rem;
        }

        .quick-actions-title {
            font-size: 1.25rem;
        }
    }
</style>
