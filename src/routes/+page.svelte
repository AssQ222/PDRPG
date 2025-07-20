<script lang="ts">
  import { onMount } from "svelte";
  import TaskInput from "$lib/components/TaskInput.svelte";
  import TaskList from "$lib/components/TaskList.svelte";
  import HabitInput from "$lib/components/HabitInput.svelte";
  import HabitTracker from "$lib/components/HabitTracker.svelte";
  import CharacterStatus from "$lib/components/CharacterStatus.svelte";
  import { taskActions } from "$lib/stores/taskStore";

  /**
   * Ładuje zadania przy inicjalizacji komponentu
   */
  onMount(async () => {
    await taskActions.loadTasks();
  });
</script>

<svelte:head>
  <title>PDRPG - Personal Development RPG</title>
  <meta
    name="description"
    content="Personal Development RPG - aplikacja do samorozwoju z elementami gry"
  />
</svelte:head>

<main class="container">
  <header class="app-header">
    <h1 class="app-title">
      <span class="title-icon">⚔️</span>
      PDRPG
    </h1>
    <p class="app-subtitle">Personal Development RPG</p>
    <div class="app-description">
      <p class="description-text">
        Rozwijaj się jak w grze RPG! Zarządzaj zadaniami, buduj nawyki i śledź
        swój postęp.
      </p>
    </div>
  </header>

  <!-- Character Status Dashboard -->
  <section class="character-section">
    <div class="section-header">
      <h2 class="section-title">🏆 Status Postaci</h2>
      <p class="section-description">
        Twój postęp w systemie RPG. Rozwijaj się i zdobywaj doświadczenie!
      </p>
    </div>

    <div class="character-status-wrapper">
      <CharacterStatus />
    </div>
  </section>

  <section class="tasks-section">
    <div class="section-header">
      <h2 class="section-title">📝 Tablica Zadań</h2>
      <p class="section-description">
        Zarządzaj swoimi zadaniami w stylu Inbox Zero. Dodawaj, ukończaj i
        organizuj swoje cele.
      </p>
    </div>

    <!-- Komponent do dodawania zadań -->
    <TaskInput />

    <!-- Lista zadań -->
    <TaskList />
  </section>

  <section class="habits-section">
    <div class="section-header">
      <h2 class="section-title">🎯 Habit Tracker</h2>
      <p class="section-description">
        Buduj lepsze nawyki dzień po dniu. Śledź postępy i rozwijaj streak!
      </p>
    </div>

    <!-- Formularz dodawania nawyków -->
    <div class="habit-input-wrapper">
      <HabitInput />
    </div>

    <!-- Tracker nawyków -->
    <HabitTracker />
  </section>

  <footer class="app-footer">
    <p class="footer-text">
      🎮 Rozwijaj się jak w grze RPG! Każde ukończone zadanie to doświadczenie w
      Twojej podróży samorozwoju.
    </p>
  </footer>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family:
      "Inter",
      -apple-system,
      BlinkMacSystemFont,
      "Segoe UI",
      "Roboto",
      "Oxygen",
      "Ubuntu",
      "Cantarell",
      sans-serif;
    font-size: 16px;
    line-height: 1.6;
    color: #1f2937;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    min-height: 100vh;
  }

  .container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem 1rem;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  .app-header {
    text-align: center;
    margin-bottom: 3rem;
    background-color: rgba(255, 255, 255, 0.95);
    padding: 2rem;
    border-radius: 1rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .app-title {
    margin: 0 0 0.5rem 0;
    font-size: 3rem;
    font-weight: 800;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .title-icon {
    font-size: 2.5rem;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
  }

  .app-subtitle {
    margin: 0 0 2rem 0;
    font-size: 1.125rem;
    color: #64748b;
    font-weight: 500;
  }

  .app-description {
    border-top: 1px solid #e2e8f0;
    padding-top: 2rem;
  }

  .description-text {
    margin: 0;
    color: #64748b;
    font-size: 1rem;
  }

  .section-header {
    margin-bottom: 2rem;
    text-align: center;
  }

  .section-title {
    margin: 0 0 0.5rem 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #1e293b;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
  }

  .section-description {
    margin: 0;
    color: #64748b;
    font-size: 0.95rem;
  }

  .character-section,
  .tasks-section,
  .habits-section {
    background-color: rgba(255, 255, 255, 0.95);
    padding: 2rem;
    border-radius: 1rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
    margin-bottom: 2rem;
  }

  .character-section {
    background: transparent;
  }

  .character-status-wrapper {
    display: flex;
    justify-content: center;
    margin-bottom: 1rem;
  }

  .habits-section {
    background: transparent;
  }

  .habit-input-wrapper {
    display: flex;
    justify-content: center;
    margin-bottom: 2rem;
  }

  .app-footer {
    text-align: center;
    background-color: rgba(255, 255, 255, 0.9);
    padding: 1.5rem;
    border-radius: 0.75rem;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .footer-text {
    margin: 0;
    color: #64748b;
    font-size: 0.95rem;
    font-weight: 500;
  }

  /* Responsive design */
  @media (max-width: 640px) {
    .container {
      padding: 1rem 0.75rem;
    }

    .app-header {
      padding: 1.5rem;
      margin-bottom: 2rem;
    }

    .app-title {
      font-size: 2.25rem;
      flex-direction: column;
      gap: 0.25rem;
    }

    .title-icon {
      font-size: 2rem;
    }

    .app-subtitle {
      font-size: 1rem;
    }

    .section-title {
      font-size: 1.25rem;
      flex-direction: column;
      gap: 0.25rem;
    }

    .character-section,
    .tasks-section,
    .habits-section {
      padding: 1.5rem;
    }

    .habit-input-wrapper {
      margin-bottom: 1.5rem;
    }

    .app-footer {
      padding: 1rem;
    }

    .footer-text {
      font-size: 0.875rem;
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    :global(body) {
      background: linear-gradient(135deg, #1e3a8a 0%, #312e81 100%);
    }

    .app-header,
    .tasks-section,
    .app-footer {
      background-color: rgba(15, 23, 42, 0.95);
      border: 1px solid rgba(71, 85, 105, 0.3);
    }

    .character-section,
    .habits-section {
      background: transparent;
    }

    .app-title {
      background: linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .section-title {
      color: #f1f5f9;
    }

    .app-subtitle,
    .description-text,
    .section-description,
    .footer-text {
      color: #94a3b8;
    }
  }
</style>
