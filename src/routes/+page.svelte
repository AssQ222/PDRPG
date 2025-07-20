<script lang="ts">
  import { onMount } from "svelte";
  import Dashboard from "$lib/components/Dashboard.svelte";
  import TaskInput from "$lib/components/TaskInput.svelte";
  import TaskList from "$lib/components/TaskList.svelte";
  import HabitInput from "$lib/components/HabitInput.svelte";
  import HabitTracker from "$lib/components/HabitTracker.svelte";
  import CharacterStatus from "$lib/components/CharacterStatus.svelte";
  import ApiControls from "$lib/components/ApiControls.svelte";
  import { taskActions } from "$lib/stores/taskStore";
  import { habitActions } from "$lib/stores/habitStore";
  import { characterActions } from "$lib/stores/characterStore";

  // Current view state
  let currentView: string = "dashboard";

  // Navigation handler
  function handleNavigate(view: string) {
    currentView = view;
  }

  // Initialize all stores on mount
  onMount(async () => {
    await Promise.all([
      taskActions.loadTasks(),
      habitActions.initialize(),
      characterActions.initialize(),
    ]);
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
  <!-- Navigation Header -->
  <header class="app-header">
    <div class="header-content">
      <div class="logo-section">
        <h1 class="app-title">
          <span class="title-icon">⚔️</span>
          PDRPG
        </h1>
        <p class="app-subtitle">Personal Development RPG</p>
      </div>

      <nav class="main-nav">
        <button
          class="nav-btn"
          class:active={currentView === "dashboard"}
          on:click={() => handleNavigate("dashboard")}
        >
          <span class="nav-icon">📊</span>
          Dashboard
        </button>
        <button
          class="nav-btn"
          class:active={currentView === "tasks"}
          on:click={() => handleNavigate("tasks")}
        >
          <span class="nav-icon">📝</span>
          Zadania
        </button>
        <button
          class="nav-btn"
          class:active={currentView === "habits"}
          on:click={() => handleNavigate("habits")}
        >
          <span class="nav-icon">🎯</span>
          Nawyki
        </button>
        <button
          class="nav-btn"
          class:active={currentView === "character"}
          on:click={() => handleNavigate("character")}
        >
          <span class="nav-icon">⚔️</span>
          Postać
        </button>
        <button
          class="nav-btn"
          class:active={currentView === "api"}
          on:click={() => handleNavigate("api")}
        >
          <span class="nav-icon">🌐</span>
          API
        </button>
      </nav>
    </div>
  </header>

  <!-- Main Content -->
  <div class="main-content">
    {#if currentView === "dashboard"}
      <Dashboard onNavigate={handleNavigate} />
    {:else if currentView === "tasks"}
      <section class="module-section">
        <div class="module-header">
          <h2 class="module-title">📝 Zarządzanie Zadaniami</h2>
          <p class="module-description">
            Organizuj swoje cele i zadania w stylu Inbox Zero. Każde ukończone
            zadanie przybliża Cię do sukcesu!
          </p>
        </div>

        <div class="module-content">
          <TaskInput />
          <TaskList />
        </div>
      </section>
    {:else if currentView === "habits"}
      <section class="module-section habits-module">
        <div class="module-header habits-header">
          <h2 class="module-title habits-title">🎯 Habit Tracker</h2>
          <p class="module-description habits-description">
            Buduj lepsze nawyki dzień po dniu. Regularne wykonywanie nawyków to
            klucz do długoterminowego sukcesu!
          </p>
        </div>

        <div class="module-content">
          <div class="habit-input-wrapper">
            <HabitInput />
          </div>
          <HabitTracker />
        </div>
      </section>
    {:else if currentView === "character"}
      <section class="module-section">
        <div class="module-header">
          <h2 class="module-title">⚔️ Rozwój Postaci</h2>
          <p class="module-description">
            Śledź swój postęp w systemie RPG. Zdobywaj doświadczenie, poziomy i
            rozwijaj atrybuty!
          </p>
        </div>

        <div class="module-content character-content">
          <CharacterStatus />

          <div class="character-tips">
            <div class="tip-card">
              <h3 class="tip-title">💡 Jak zdobywać EXP?</h3>
              <ul class="tip-list">
                <li>Ukończ zadanie - 15 EXP</li>
                <li>Oznacz nawyk jako wykonany - 10 EXP + bonus za streak</li>
                <li>Zadania związane z celami - bonus EXP</li>
                <li>Dłuższe streaki nawyków - do 50% więcej EXP</li>
              </ul>
            </div>

            <div class="tip-card">
              <h3 class="tip-title">🎯 System atrybutów</h3>
              <ul class="tip-list">
                <li><strong>Siła:</strong> Sport, trening, zdrowie</li>
                <li><strong>Intelekt:</strong> Nauka, czytanie, kursy</li>
                <li><strong>Charyzma:</strong> Kontakty, prezentacje</li>
                <li><strong>Zręczność:</strong> Praktyczne umiejętności</li>
                <li><strong>Mądrość:</strong> Medytacja, refleksja</li>
                <li><strong>Kondycja:</strong> Sen, dieta, nawyki zdrowotne</li>
              </ul>
            </div>
          </div>
        </div>
      </section>
    {:else if currentView === "api"}
      <section class="module-section">
        <div class="module-header">
          <h2 class="module-title">🌐 API Server</h2>
          <p class="module-description">
            Uruchom lokalny HTTP API server umożliwiający zewnętrznym narzędziom
            (np. skryptom AI) dostęp do danych aplikacji.
          </p>
        </div>

        <div class="module-content">
          <ApiControls />
        </div>
      </section>
    {/if}
  </div>

  <!-- Footer -->
  <footer class="app-footer">
    <p class="footer-text">
      🎮 Rozwijaj się jak w grze RPG! Każde ukończone zadanie i nawyk to
      doświadczenie w Twojej podróży samorozwoju.
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
    background-color: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 1rem;
    padding: 2rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    margin-bottom: 2rem;
  }

  .header-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-wrap: wrap;
    gap: 2rem;
  }

  .logo-section {
    text-align: left;
  }

  .app-title {
    margin: 0 0 0.5rem 0;
    font-size: 2.5rem;
    font-weight: 800;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .title-icon {
    font-size: 2rem;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
  }

  .app-subtitle {
    margin: 0;
    font-size: 1.125rem;
    color: #64748b;
    font-weight: 500;
  }

  .main-nav {
    display: flex;
    gap: 0.5rem;
    background: #f8fafc;
    padding: 0.5rem;
    border-radius: 0.75rem;
  }

  .nav-btn {
    background: transparent;
    border: none;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    font-size: 0.875rem;
    font-weight: 600;
    color: #64748b;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .nav-btn:hover {
    background: rgba(102, 126, 234, 0.1);
    color: #667eea;
  }

  .nav-btn.active {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  }

  .nav-icon {
    font-size: 1rem;
  }

  .main-content {
    flex: 1;
    margin-bottom: 2rem;
  }

  .module-section {
    background-color: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 1rem;
    padding: 2rem;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  }

  .module-header {
    text-align: center;
    margin-bottom: 2rem;
    border-bottom: 1px solid #e2e8f0;
    padding-bottom: 2rem;
  }

  .module-title {
    margin: 0 0 0.5rem 0;
    font-size: 2rem;
    font-weight: 600;
    color: #1e293b;
  }

  .module-description {
    margin: 0;
    color: #64748b;
    font-size: 1rem;
    max-width: 600px;
    margin: 0 auto;
  }

  .module-content {
    max-width: 800px;
    margin: 0 auto;
  }

  .habit-input-wrapper {
    display: flex;
    justify-content: center;
    margin-bottom: 2rem;
  }

  /* Special dark theme for habits module */
  .habits-module {
    background-color: rgba(15, 23, 42, 0.95) !important;
    border: 1px solid rgba(71, 85, 105, 0.3) !important;
  }

  .habits-title {
    color: #f1f5f9 !important;
  }

  .habits-description {
    color: #94a3b8 !important;
  }

  .habits-header {
    border-bottom-color: #374151 !important;
  }

  .character-content {
    max-width: 1000px;
  }

  .character-tips {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 2rem;
    margin-top: 2rem;
  }

  .tip-card {
    background: #f8fafc;
    border-radius: 0.75rem;
    padding: 1.5rem;
    border: 1px solid #e2e8f0;
  }

  .tip-title {
    margin: 0 0 1rem 0;
    font-size: 1.125rem;
    font-weight: 600;
    color: #1e293b;
  }

  .tip-list {
    margin: 0;
    padding-left: 1.25rem;
    color: #64748b;
  }

  .tip-list li {
    margin-bottom: 0.5rem;
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

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    :global(body) {
      background: linear-gradient(135deg, #1e3a8a 0%, #312e81 100%);
    }

    .app-header,
    .module-section,
    .app-footer {
      background-color: rgba(15, 23, 42, 0.95);
      border: 1px solid rgba(71, 85, 105, 0.3);
    }

    .app-title {
      background: linear-gradient(135deg, #60a5fa 0%, #a78bfa 100%);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
    }

    .app-subtitle,
    .module-description,
    .footer-text {
      color: #94a3b8;
    }

    .module-title {
      color: #f1f5f9;
    }

    .main-nav {
      background: #1e293b;
    }

    .nav-btn {
      color: #94a3b8;
    }

    .nav-btn:hover {
      background: rgba(96, 165, 250, 0.1);
      color: #60a5fa;
    }

    .module-header {
      border-bottom-color: #374151;
    }

    .tip-card {
      background: #1e293b;
      border-color: #374151;
    }

    .tip-title {
      color: #f1f5f9;
    }

    .tip-list {
      color: #94a3b8;
    }
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .container {
      padding: 1rem 0.75rem;
    }

    .app-header {
      padding: 1.5rem;
    }

    .header-content {
      flex-direction: column;
      text-align: center;
      gap: 1.5rem;
    }

    .logo-section {
      text-align: center;
    }

    .app-title {
      font-size: 2rem;
      justify-content: center;
    }

    .title-icon {
      font-size: 1.75rem;
    }

    .main-nav {
      flex-wrap: wrap;
      justify-content: center;
    }

    .nav-btn {
      font-size: 0.8rem;
      padding: 0.5rem 0.75rem;
    }

    .module-title {
      font-size: 1.5rem;
    }

    .character-tips {
      grid-template-columns: 1fr;
      gap: 1.5rem;
    }

    .tip-card {
      padding: 1rem;
    }
  }

  @media (max-width: 480px) {
    .nav-btn {
      flex-direction: column;
      gap: 0.25rem;
      font-size: 0.75rem;
      padding: 0.5rem;
    }

    .nav-icon {
      font-size: 1.125rem;
    }
  }
</style>
