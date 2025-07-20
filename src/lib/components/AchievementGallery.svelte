<script lang="ts">
    import { onMount } from "svelte";
    import {
        achievements,
        earnedAchievements,
        availableAchievements,
        lockedAchievements,
        achievementStats,
        isAchievementsLoading,
        achievementError,
        achievementActions,
    } from "../stores/questStore";
    import type { Achievement, AchievementStatus } from "../types/quest";
    import { getAchievementStatusColor } from "../types/quest";

    // Reactive state
    $: achievementsData = $achievements;
    $: earnedAchievementsData = $earnedAchievements;
    $: availableAchievementsData = $availableAchievements;
    $: lockedAchievementsData = $lockedAchievements;
    $: achievementStatsData = $achievementStats;
    $: loading = $isAchievementsLoading;
    $: error = $achievementError;

    // UI state
    let selectedFilter: AchievementStatus | "All" = "All";
    let sortBy: "name" | "required_value" | "earned_at" = "required_value";

    onMount(async () => {
        await achievementActions.loadAllAchievements();
        await achievementActions.checkAndUpdateAchievements();
    });

    // Filtered and sorted achievements
    $: filteredAchievements = (() => {
        let filtered = achievementsData;

        if (selectedFilter !== "All") {
            filtered = achievementsData.filter(
                (achievement) => achievement.status === selectedFilter,
            );
        }

        // Sort achievements
        return filtered.sort((a, b) => {
            switch (sortBy) {
                case "name":
                    return a.name.localeCompare(b.name);
                case "required_value":
                    return a.required_value - b.required_value;
                case "earned_at":
                    if (!a.earned_at && !b.earned_at) return 0;
                    if (!a.earned_at) return 1;
                    if (!b.earned_at) return -1;
                    return b.earned_at - a.earned_at;
                default:
                    return 0;
            }
        });
    })();

    async function handleEarnAchievement(achievement: Achievement) {
        if (achievement.status !== "Available") return;

        try {
            const earned = await achievementActions.earnAchievement(
                achievement.id,
            );
            if (earned) {
                console.log(`Achievement earned: ${earned.name}!`);
            }
        } catch (err) {
            console.error("Failed to earn achievement:", err);
        }
    }

    function getAchievementStatusText(status: AchievementStatus): string {
        switch (status) {
            case "Locked":
                return "Zablokowana";
            case "Available":
                return "Dostępna";
            case "Earned":
                return "Zdobyta";
        }
    }

    function getAchievementTypeText(type: string): string {
        switch (type) {
            case "HabitStreak":
                return "Streak Nawyków";
            case "TaskCount":
                return "Liczba Zadań";
            case "CharacterLevel":
                return "Poziom Postaci";
            case "QuestCount":
                return "Liczba Questów";
            default:
                return type;
        }
    }

    function formatEarnedDate(timestamp?: number): string {
        if (!timestamp) return "Nie zdobyta";

        const date = new Date(timestamp * 1000);
        return date.toLocaleDateString("pl-PL", {
            year: "numeric",
            month: "short",
            day: "numeric",
            hour: "2-digit",
            minute: "2-digit",
        });
    }

    function getAchievementCardClass(achievement: Achievement): string {
        const baseClass =
            "achievement-card rounded-xl p-6 border transition-all duration-300 hover:scale-105";

        switch (achievement.status) {
            case "Earned":
                return `${baseClass} bg-gradient-to-br from-yellow-500/20 to-gold-600/20 border-yellow-400/50 shadow-lg shadow-yellow-500/20`;
            case "Available":
                return `${baseClass} bg-gradient-to-br from-green-500/20 to-blue-500/20 border-green-400/50 shadow-md hover:shadow-lg shadow-green-500/10`;
            case "Locked":
                return `${baseClass} bg-white/5 border-gray-600/30 opacity-60`;
        }
    }
</script>

<div
    class="achievement-gallery p-6 bg-white/20 backdrop-blur-sm rounded-xl border border-white/30 shadow-lg"
>
    <div class="flex justify-between items-center mb-6">
        <h2 class="text-2xl font-bold text-white flex items-center gap-2">
            🏆 Galeria Odznak
        </h2>

        <div class="flex gap-3">
            <select
                bind:value={selectedFilter}
                class="bg-gray-700 text-white border border-gray-600 rounded-lg px-3 py-2 text-sm"
            >
                <option value="All">Wszystkie</option>
                <option value="Earned">Zdobyte</option>
                <option value="Available">Dostępne</option>
                <option value="Locked">Zablokowane</option>
            </select>

            <select
                bind:value={sortBy}
                class="bg-gray-700 text-white border border-gray-600 rounded-lg px-3 py-2 text-sm"
            >
                <option value="required_value">Wymagania</option>
                <option value="name">Nazwa</option>
                <option value="earned_at">Data Zdobycia</option>
            </select>
        </div>
    </div>

    <!-- Achievement Statistics -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <div class="bg-white/10 rounded-lg p-4 text-center">
            <div class="text-2xl font-bold text-yellow-400">
                {achievementStatsData.earned}
            </div>
            <div class="text-sm text-gray-300">Zdobyte</div>
        </div>
        <div class="bg-white/10 rounded-lg p-4 text-center">
            <div class="text-2xl font-bold text-green-400">
                {achievementStatsData.available}
            </div>
            <div class="text-sm text-gray-300">Dostępne</div>
        </div>
        <div class="bg-white/10 rounded-lg p-4 text-center">
            <div class="text-2xl font-bold text-gray-400">
                {achievementStatsData.locked}
            </div>
            <div class="text-sm text-gray-300">Zablokowane</div>
        </div>
        <div class="bg-white/10 rounded-lg p-4 text-center">
            <div class="text-2xl font-bold text-purple-400">
                {achievementStatsData.earnedPercentage.toFixed(1)}%
            </div>
            <div class="text-sm text-gray-300">Ukończenie</div>
        </div>
    </div>

    <!-- Error Display -->
    {#if error}
        <div class="bg-red-500/20 border border-red-500/30 rounded-lg p-4 mb-4">
            <div class="text-red-300 font-medium">❌ Błąd: {error.message}</div>
            <button
                on:click={() => achievementActions.clearError()}
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
            <span class="ml-3 text-gray-300">Ładowanie odznak...</span>
        </div>
    {/if}

    <!-- Achievements Grid -->
    {#if !loading && filteredAchievements.length > 0}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {#each filteredAchievements as achievement (achievement.id)}
                <div class={getAchievementCardClass(achievement)}>
                    <!-- Achievement Icon and Status -->
                    <div class="flex justify-between items-start mb-4">
                        <div class="text-4xl">{achievement.icon}</div>
                        <div class="text-right">
                            <div
                                class="text-xs {getAchievementStatusColor(
                                    achievement.status,
                                )} font-medium"
                            >
                                {getAchievementStatusText(achievement.status)}
                            </div>
                            <div class="text-xs text-gray-400 mt-1">
                                {getAchievementTypeText(
                                    achievement.achievement_type,
                                )}
                            </div>
                        </div>
                    </div>

                    <!-- Achievement Details -->
                    <div class="mb-4">
                        <h3 class="font-bold text-lg text-white mb-2">
                            {achievement.name}
                        </h3>
                        <p class="text-gray-300 text-sm mb-3">
                            {achievement.description}
                        </p>

                        <div
                            class="flex items-center justify-between text-xs text-gray-400"
                        >
                            <span>Wymagane: {achievement.required_value}</span>
                            {#if achievement.status === "Earned" && achievement.earned_at}
                                <span
                                    >{formatEarnedDate(
                                        achievement.earned_at,
                                    )}</span
                                >
                            {/if}
                        </div>
                    </div>

                    <!-- Action Button -->
                    {#if achievement.status === "Available"}
                        <button
                            on:click={() => handleEarnAchievement(achievement)}
                            class="w-full bg-green-500 hover:bg-green-600 text-white py-2 px-4 rounded-lg transition-colors duration-200 font-medium text-sm"
                        >
                            🏆 Odbierz Odznakę
                        </button>
                    {:else if achievement.status === "Earned"}
                        <div
                            class="w-full bg-yellow-500/30 text-yellow-300 py-2 px-4 rounded-lg text-center font-medium text-sm"
                        >
                            ✅ Zdobyta!
                        </div>
                    {:else}
                        <div
                            class="w-full bg-gray-600/30 text-gray-400 py-2 px-4 rounded-lg text-center font-medium text-sm"
                        >
                            🔒 Zablokowana
                        </div>
                    {/if}
                </div>
            {/each}
        </div>
    {/if}

    <!-- Empty State -->
    {#if !loading && filteredAchievements.length === 0}
        <div class="text-center py-8">
            <div class="text-4xl mb-4">🏆</div>
            <h3 class="text-lg font-semibold text-white mb-2">
                {selectedFilter === "All"
                    ? "Brak odznak"
                    : `Brak odznak: ${getAchievementStatusText(selectedFilter)}`}
            </h3>
            <p class="text-gray-300 mb-4">
                {selectedFilter === "Available"
                    ? "Kontynuuj wykonywanie zadań i utrzymywanie nawyków aby odblokować odznaki!"
                    : selectedFilter === "Earned"
                      ? "Jeszcze nie zdobyłeś żadnych odznak. Czas zacząć!"
                      : "Zmień filtr aby zobaczyć dostępne odznaki."}
            </p>
            {#if selectedFilter !== "All"}
                <button
                    on:click={() => (selectedFilter = "All")}
                    class="bg-purple-500 hover:bg-purple-600 text-white px-6 py-3 rounded-lg transition-colors duration-200 font-medium"
                >
                    Pokaż Wszystkie Odznaki
                </button>
            {/if}
        </div>
    {/if}

    <!-- Quick Stats Summary -->
    {#if !loading && achievementsData.length > 0}
        <div class="mt-8 p-4 bg-white/10 rounded-lg">
            <h4 class="text-sm font-semibold text-white mb-2">
                🎯 Cel Zbierania Odznak
            </h4>
            <div class="w-full bg-gray-700 rounded-full h-2 mb-2">
                <div
                    class="bg-gradient-to-r from-purple-500 to-yellow-500 h-2 rounded-full transition-all duration-500"
                    style="width: {achievementStatsData.earnedPercentage}%"
                ></div>
            </div>
            <p class="text-xs text-gray-300">
                Masz {achievementStatsData.earned} z {achievementStatsData.total}
                odznak ({achievementStatsData.earnedPercentage.toFixed(1)}%
                ukończone)
            </p>
        </div>
    {/if}
</div>

<style>
    .achievement-gallery {
        /* Custom scrollbar styling */
        scrollbar-width: thin;
        scrollbar-color: rgba(168, 85, 247, 0.5) transparent;
    }

    .achievement-gallery::-webkit-scrollbar {
        width: 6px;
    }

    .achievement-gallery::-webkit-scrollbar-track {
        background: transparent;
    }

    .achievement-gallery::-webkit-scrollbar-thumb {
        background-color: rgba(168, 85, 247, 0.5);
        border-radius: 3px;
    }

    .achievement-gallery::-webkit-scrollbar-thumb:hover {
        background-color: rgba(168, 85, 247, 0.7);
    }

    .achievement-card {
        cursor: pointer;
    }

    .achievement-card:hover {
        transform: translateY(-2px);
    }
</style>
