<script lang="ts">
    import { onMount } from "svelte";
    import {
        quests,
        activeQuests,
        completedQuests,
        questStats,
        isQuestsLoading,
        questError,
        questActions,
        initializeQuestSystem,
    } from "../stores/questStore";
    import type { Quest } from "../types/quest";
    import {
        getQuestStatusColor,
        getQuestStatusIcon,
        getQuestTypeIcon,
        formatDeadline,
        calculateQuestProgress,
        formatQuestProgress,
    } from "../types/quest";

    // Reactive state
    $: questsData = $quests;
    $: activeQuestsData = $activeQuests;
    $: completedQuestsData = $completedQuests;
    $: questStatsData = $questStats;
    $: loading = $isQuestsLoading;
    $: error = $questError;

    // UI state
    let showCompleted = false;
    let generatingQuests = false;

    onMount(() => {
        let interval: number;

        // Initialize quest system and start auto-refresh
        (async () => {
            await initializeQuestSystem();

            // Automatyczne odświeżanie questów co 30 sekund
            interval = setInterval(async () => {
                try {
                    await questActions.updateQuestProgress();
                } catch (error) {
                    console.warn(
                        "Failed to auto-update quest progress:",
                        error,
                    );
                }
            }, 30000);
        })();

        // Cleanup interval on component destroy
        return () => {
            if (interval) clearInterval(interval);
        };
    });

    async function handleGenerateQuests() {
        generatingQuests = true;
        try {
            const newQuests = await questActions.generateWeeklyQuests();
            if (newQuests.length > 0) {
                console.log(`Generated ${newQuests.length} new quests!`);
            } else {
                console.log(
                    "No new quests generated - already exist for this week",
                );
            }
        } catch (err) {
            console.error("Failed to generate quests:", err);
        } finally {
            generatingQuests = false;
        }
    }

    async function handleCompleteQuest(quest: Quest) {
        if (quest.status !== "Active") return;

        try {
            const completed = await questActions.completeQuest(quest.id);
            if (completed) {
                console.log(
                    `Quest completed: ${completed.title} - ${completed.reward_exp} EXP!`,
                );
            }
        } catch (err) {
            console.error("Failed to complete quest:", err);
        }
    }

    function getQuestProgressColor(progress: number): string {
        if (progress >= 100) return "bg-green-500";
        if (progress >= 75) return "bg-blue-500";
        if (progress >= 50) return "bg-yellow-500";
        if (progress >= 25) return "bg-orange-500";
        return "bg-gray-300";
    }
</script>

<div
    class="quest-tracker p-6 bg-white/20 backdrop-blur-sm rounded-xl border border-white/30 shadow-lg"
>
    <div class="flex justify-between items-center mb-6">
        <h2 class="text-2xl font-bold text-white flex items-center gap-2">
            🎯 Questy Tygodniowe
        </h2>

        <div class="flex gap-3">
            <button
                on:click={handleGenerateQuests}
                disabled={generatingQuests || loading}
                class="bg-purple-500 hover:bg-purple-600 disabled:bg-gray-400 text-white px-4 py-2 rounded-lg transition-colors duration-200 text-sm font-medium"
            >
                {#if generatingQuests}
                    🔄 Generowanie...
                {:else}
                    ✨ Generuj Questy
                {/if}
            </button>

            <button
                on:click={() => (showCompleted = !showCompleted)}
                class="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-lg transition-colors duration-200 text-sm font-medium"
            >
                {showCompleted ? "👁️ Ukryj Ukończone" : "📋 Pokaż Ukończone"}
            </button>
        </div>
    </div>

    <!-- Quest Statistics -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <div class="bg-white/10 rounded-lg p-4 text-center">
            <div class="text-2xl font-bold text-blue-400">
                {questStatsData.active}
            </div>
            <div class="text-sm text-gray-300">Aktywne</div>
        </div>
        <div class="bg-white/10 rounded-lg p-4 text-center">
            <div class="text-2xl font-bold text-green-400">
                {questStatsData.completed}
            </div>
            <div class="text-sm text-gray-300">Ukończone</div>
        </div>
        <div class="bg-white/10 rounded-lg p-4 text-center">
            <div class="text-2xl font-bold text-red-400">
                {questStatsData.expired}
            </div>
            <div class="text-sm text-gray-300">Wygasłe</div>
        </div>
        <div class="bg-white/10 rounded-lg p-4 text-center">
            <div class="text-2xl font-bold text-purple-400">
                {questStatsData.completionRate.toFixed(1)}%
            </div>
            <div class="text-sm text-gray-300">Wskaźnik Ukończenia</div>
        </div>
    </div>

    <!-- Error Display -->
    {#if error}
        <div class="bg-red-500/20 border border-red-500/30 rounded-lg p-4 mb-4">
            <div class="text-red-300 font-medium">❌ Błąd: {error.message}</div>
            <button
                on:click={() => questActions.clearError()}
                class="text-red-200 text-sm hover:text-red-100 mt-2"
            >
                Zamknij
            </button>
        </div>
    {/if}

    <!-- Loading State -->
    {#if loading}
        <div class="flex justify-center items-center py-8">
            <div
                class="animate-spin rounded-full h-8 w-8 border-b-2 border-purple-400"
            ></div>
            <span class="ml-3 text-gray-300">Ładowanie questów...</span>
        </div>
    {/if}

    <!-- Active Quests -->
    {#if !loading && activeQuestsData.length > 0}
        <div class="space-y-4 mb-6">
            <h3 class="text-lg font-semibold text-white">🎯 Aktywne Questy</h3>
            {#each activeQuestsData as quest (quest.id)}
                <div
                    class="bg-white/10 border border-white/20 rounded-lg p-4 hover:bg-white/15 transition-colors"
                >
                    <div class="flex justify-between items-start mb-3">
                        <div class="flex-1">
                            <div class="flex items-center gap-2 mb-1">
                                <span class="text-lg"
                                    >{getQuestTypeIcon(quest.quest_type)}</span
                                >
                                <h4 class="font-semibold text-white">
                                    {quest.title}
                                </h4>
                                <span class="text-lg"
                                    >{getQuestStatusIcon(quest.status)}</span
                                >
                            </div>
                            <p class="text-gray-300 text-sm mb-2">
                                {quest.description}
                            </p>

                            <!-- Progress Bar -->
                            <div class="flex items-center gap-3 mb-2">
                                <div
                                    class="flex-1 bg-gray-700 rounded-full h-2"
                                >
                                    <div
                                        class="h-2 rounded-full transition-all duration-300 {getQuestProgressColor(
                                            calculateQuestProgress(quest),
                                        )}"
                                        style="width: {calculateQuestProgress(
                                            quest,
                                        )}%"
                                    ></div>
                                </div>
                                <span class="text-sm text-gray-300 min-w-0"
                                    >{formatQuestProgress(quest)}</span
                                >
                            </div>

                            <div
                                class="flex items-center gap-4 text-xs text-gray-400"
                            >
                                <span>💰 {quest.reward_exp} EXP</span>
                                <span>⏰ {formatDeadline(quest.deadline)}</span>
                                {#if quest.category}
                                    <span
                                        class="bg-purple-500/30 px-2 py-1 rounded text-purple-300"
                                        >#{quest.category}</span
                                    >
                                {/if}
                            </div>
                        </div>

                        <div class="flex gap-2 ml-4">
                            {#if quest.current_progress >= quest.target_value}
                                <button
                                    on:click={() => handleCompleteQuest(quest)}
                                    class="bg-green-500 hover:bg-green-600 text-white px-3 py-1 rounded text-sm transition-colors"
                                >
                                    ✅ Odbierz
                                </button>
                            {:else}
                                <div class="text-gray-400 text-sm px-3 py-1">
                                    {Math.round(calculateQuestProgress(quest))}%
                                    ukończone
                                </div>
                            {/if}
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    {/if}

    <!-- Completed Quests (when toggled) -->
    {#if showCompleted && completedQuestsData.length > 0}
        <div class="space-y-4">
            <h3 class="text-lg font-semibold text-white">
                ✅ Ukończone Questy
            </h3>
            {#each completedQuestsData as quest (quest.id)}
                <div
                    class="bg-green-500/10 border border-green-500/30 rounded-lg p-4"
                >
                    <div class="flex items-center gap-2 mb-1">
                        <span class="text-lg"
                            >{getQuestTypeIcon(quest.quest_type)}</span
                        >
                        <h4 class="font-semibold text-green-300">
                            {quest.title}
                        </h4>
                        <span class="text-lg">✅</span>
                    </div>
                    <p class="text-gray-300 text-sm mb-2">
                        {quest.description}
                    </p>
                    <div class="flex items-center gap-4 text-xs text-gray-400">
                        <span>💰 {quest.reward_exp} EXP zdobyte</span>
                        <span>🎯 {formatQuestProgress(quest)} ukończone</span>
                    </div>
                </div>
            {/each}
        </div>
    {/if}

    <!-- Empty State -->
    {#if !loading && activeQuestsData.length === 0}
        <div class="text-center py-8">
            <div class="text-4xl mb-4">🎯</div>
            <h3 class="text-lg font-semibold text-white mb-2">
                Brak aktywnych questów
            </h3>
            <p class="text-gray-300 mb-4">
                Wygeneruj nowe questy tygodniowe aby rozpocząć przygodę!
            </p>
            <button
                on:click={handleGenerateQuests}
                disabled={generatingQuests}
                class="bg-purple-500 hover:bg-purple-600 disabled:bg-gray-400 text-white px-6 py-3 rounded-lg transition-colors duration-200 font-medium"
            >
                {#if generatingQuests}
                    🔄 Generowanie...
                {:else}
                    ✨ Generuj Pierwsze Questy
                {/if}
            </button>
        </div>
    {/if}
</div>

<style>
    .quest-tracker {
        /* Custom scrollbar styling */
        scrollbar-width: thin;
        scrollbar-color: rgba(168, 85, 247, 0.5) transparent;
    }

    .quest-tracker::-webkit-scrollbar {
        width: 6px;
    }

    .quest-tracker::-webkit-scrollbar-track {
        background: transparent;
    }

    .quest-tracker::-webkit-scrollbar-thumb {
        background-color: rgba(168, 85, 247, 0.5);
        border-radius: 3px;
    }

    .quest-tracker::-webkit-scrollbar-thumb:hover {
        background-color: rgba(168, 85, 247, 0.7);
    }
</style>
