<!-- CharacterStatus.svelte - Komponent wyświetlający status postaci gracza -->
<script lang="ts">
    import { onMount } from "svelte";
    import {
        character,
        characterStats,
        experienceInfo,
        attributeInfo,
        isLoading,
        characterError,
        characterActions,
        levelUpNotifications,
        isCharacterLoaded,
    } from "../stores/characterStore";
    import {
        CLASS_ICONS,
        CLASS_COLORS,
        CLASS_DESCRIPTIONS,
        ATTRIBUTE_NAMES,
        CharacterClass,
    } from "../types/character";
    import type { CharacterClass as CharacterClassType } from "../types/character";

    // Reactive statements
    $: classIcon = $character ? CLASS_ICONS[$character.character_class] : "⚔️";
    $: classColor = $character
        ? CLASS_COLORS[$character.character_class]
        : "text-gray-600";
    $: classDescription = $character
        ? CLASS_DESCRIPTIONS[$character.character_class]
        : "";

    // Inicjalizacja przy montowaniu komponentu
    onMount(async () => {
        if (!$isCharacterLoaded) {
            await characterActions.initialize();
        }
    });

    // Funkcja do formatowania liczb z separatorami tysięcy
    function formatNumber(num: number): string {
        return new Intl.NumberFormat("pl-PL").format(num);
    }

    // Funkcja do obsługi notyfikacji level up
    function handleLevelUpNotification(level: number) {
        // Pokazuje toast notification lub modal
        alert(`🎉 Gratulacje! Osiągnąłeś poziom ${level}!`);
    }

    // Nasłuchuj na notyfikacje level up
    $: if ($levelUpNotifications.length > 0) {
        const latest = $levelUpNotifications[$levelUpNotifications.length - 1];
        handleLevelUpNotification(latest.level);
    }

    // Funkcja resetowania postaci
    async function resetCharacter() {
        if (
            confirm(
                "🔄 Czy na pewno chcesz zresetować postać? Ta akcja jest nieodwracalna!",
            )
        ) {
            try {
                await characterActions.createCharacter({
                    character_class: CharacterClass.Warrior,
                });
                console.log("🔄 Character reset successful!");
            } catch (error) {
                console.error("Failed to reset character:", error);
            }
        }
    }

    // Wybór klasy postaci
    let showClassSelector = false;

    async function handleClassChange(newClass: CharacterClass) {
        await characterActions.updateCharacter({ character_class: newClass });
        showClassSelector = false;
        console.log("🎭 Character class changed to:", newClass);
    }
</script>

<!-- Main Character Status Component -->
<div class="w-full max-w-md mx-auto">
    <!-- Loading State -->
    {#if $isLoading}
        <div
            class="bg-white/10 backdrop-blur-md rounded-xl p-6 border border-white/20 shadow-xl"
        >
            <div class="flex items-center justify-center space-x-2">
                <div
                    class="animate-spin rounded-full h-6 w-6 border-b-2 border-white"
                ></div>
                <span class="text-white/80">Ładowanie postaci...</span>
            </div>
        </div>

        <!-- Error State -->
    {:else if $characterError}
        <div
            class="bg-red-500/10 backdrop-blur-md rounded-xl p-6 border border-red-500/20 shadow-xl"
        >
            <div class="flex items-center space-x-2">
                <span class="text-red-400">❌</span>
                <span class="text-red-300">{$characterError.message}</span>
            </div>
            <button
                on:click={() => characterActions.clearError()}
                class="mt-2 text-sm text-red-400 hover:text-red-300 underline"
            >
                Spróbuj ponownie
            </button>
        </div>

        <!-- Main Character Display -->
    {:else if $character && $characterStats && $experienceInfo}
        <div
            class="bg-white/10 backdrop-blur-md rounded-xl p-6 border border-white/20 shadow-xl"
        >
            <!-- Header with Class and Level -->
            <div class="flex items-center justify-between mb-4">
                <div class="flex items-center space-x-3">
                    <!-- Avatar/Class Icon -->
                    <div
                        class="w-12 h-12 bg-white/20 rounded-full flex items-center justify-center text-2xl"
                    >
                        {classIcon}
                    </div>

                    <!-- Character Info -->
                    <div>
                        <h3 class="text-white font-semibold text-lg">
                            Poziom {$character.level}
                        </h3>
                        <div class="flex items-center space-x-2">
                            <p class="text-white/70 text-sm {classColor}">
                                {$character.character_class}
                            </p>
                            <button
                                on:click={() =>
                                    (showClassSelector = !showClassSelector)}
                                class="text-white/60 hover:text-white/80 text-xs px-1 py-0.5 rounded transition-colors"
                                title="Zmień klasę postaci"
                            >
                                ⚙️
                            </button>
                        </div>
                    </div>
                </div>

                <!-- Total Attribute Points -->
                {#if $attributeInfo}
                    <div class="text-right">
                        <div class="text-white text-lg font-bold">
                            {$attributeInfo.total}
                        </div>
                        <div class="text-white/60 text-xs">
                            punktów atrybutów
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Class Selector -->
            {#if showClassSelector}
                <div
                    class="mb-4 p-3 bg-white/5 rounded-lg border border-white/20"
                >
                    <h4 class="text-white text-sm font-semibold mb-2">
                        Wybierz klasę postaci:
                    </h4>
                    <div class="grid grid-cols-2 gap-2">
                        {#each [CharacterClass.Warrior, CharacterClass.Mage, CharacterClass.Bard, CharacterClass.Rogue] as className}
                            <button
                                on:click={() => handleClassChange(className)}
                                class="flex items-center space-x-2 p-2 bg-white/10 hover:bg-white/20 rounded-lg transition-colors"
                                class:ring-2={$character?.character_class ===
                                    className}
                                class:ring-purple-400={$character?.character_class ===
                                    className}
                            >
                                <span class="text-lg"
                                    >{CLASS_ICONS[className]}</span
                                >
                                <div class="text-left">
                                    <div class="text-white text-sm font-medium">
                                        {className}
                                    </div>
                                    <div class="text-white/60 text-xs">
                                        {CLASS_DESCRIPTIONS[className].slice(
                                            0,
                                            30,
                                        )}...
                                    </div>
                                </div>
                            </button>
                        {/each}
                    </div>
                    <button
                        on:click={() => (showClassSelector = false)}
                        class="mt-2 text-white/60 hover:text-white/80 text-xs"
                    >
                        Anuluj
                    </button>
                </div>
            {/if}

            <!-- Experience Progress Bar -->
            <div class="mb-4">
                <div class="flex items-center justify-between mb-2">
                    <span class="text-white/80 text-sm">Doświadczenie</span>
                    <span class="text-white/60 text-xs">
                        {formatNumber($experienceInfo.current)} / {formatNumber(
                            $experienceInfo.needed,
                        )} EXP
                    </span>
                </div>

                <!-- Progress Bar -->
                <div
                    class="w-full bg-white/10 rounded-full h-3 overflow-hidden"
                >
                    <div
                        class="h-full bg-gradient-to-r from-purple-500 to-blue-500 transition-all duration-500 ease-out"
                        style="width: {Math.max(
                            0,
                            Math.min(100, $experienceInfo.percentage),
                        )}%"
                    ></div>
                </div>

                <!-- EXP to Next Level -->
                <div class="mt-1 text-center">
                    <span class="text-white/50 text-xs">
                        {formatNumber($character.experience)} EXP do następnego poziomu:
                        {formatNumber(
                            $experienceInfo.nextLevel - $character.experience,
                        )}
                    </span>
                </div>
            </div>

            <!-- Attributes Mini Display -->
            {#if $attributeInfo}
                <div class="grid grid-cols-3 gap-2 mb-4">
                    {#each Object.entries(ATTRIBUTE_NAMES) as [key, name]}
                        <div class="bg-white/5 rounded-lg p-2 text-center">
                            <div class="text-white text-sm font-semibold">
                                {$attributeInfo[
                                    key as keyof typeof $attributeInfo
                                ]}
                            </div>
                            <div class="text-white/60 text-xs">
                                {name}
                            </div>
                        </div>
                    {/each}
                </div>
            {/if}

            <!-- Character Class Description -->
            <div class="text-center">
                <p class="text-white/60 text-xs italic">
                    {classDescription}
                </p>
            </div>

            <!-- Character Controls -->
            <div class="mt-4 pt-4 border-t border-white/10">
                <button
                    on:click={resetCharacter}
                    class="w-full bg-red-500/20 hover:bg-red-500/30 text-red-300 text-xs py-2 px-3 rounded-lg transition-colors"
                >
                    🔄 Reset postaci
                </button>
            </div>
        </div>

        <!-- No Character State -->
    {:else}
        <div
            class="bg-white/10 backdrop-blur-md rounded-xl p-6 border border-white/20 shadow-xl text-center"
        >
            <div class="mb-4">
                <span class="text-4xl">👤</span>
            </div>
            <h3 class="text-white mb-2">Brak postaci</h3>
            <p class="text-white/60 text-sm mb-4">
                Stwórz swoją postać, aby rozpocząć przygodę!
            </p>
            <button
                on:click={() => characterActions.initialize()}
                class="bg-purple-500 hover:bg-purple-600 text-white px-4 py-2 rounded-lg transition-colors"
            >
                Utwórz postać
            </button>
        </div>
    {/if}
</div>

<!-- Level Up Notifications -->
{#if $levelUpNotifications.length > 0}
    <div class="fixed top-4 right-4 z-50 space-y-2">
        {#each $levelUpNotifications as notification (notification.id)}
            <div
                class="bg-gradient-to-r from-yellow-400 to-orange-500 text-white px-6 py-3 rounded-lg shadow-lg animate-bounce"
                role="alert"
            >
                <div class="flex items-center space-x-2">
                    <span class="text-xl">🎉</span>
                    <div>
                        <div class="font-bold">Level Up!</div>
                        <div class="text-sm">
                            Osiągnąłeś poziom {notification.level}!
                        </div>
                    </div>
                    <button
                        on:click={() =>
                            levelUpNotifications.remove(notification.id)}
                        class="ml-2 text-white/80 hover:text-white"
                    >
                        ✕
                    </button>
                </div>
            </div>
        {/each}
    </div>
{/if}

<style>
    /* Dodatkowe style dla animacji */
    @keyframes pulse-glow {
        0%,
        100% {
            box-shadow: 0 0 5px rgba(168, 85, 247, 0.4);
        }
        50% {
            box-shadow: 0 0 20px rgba(168, 85, 247, 0.8);
        }
    }

    .level-up-glow {
        animation: pulse-glow 2s infinite;
    }

    /* Gradient animation dla progress bar */
    @keyframes gradient-shift {
        0% {
            background-position: 0% 50%;
        }
        50% {
            background-position: 100% 50%;
        }
        100% {
            background-position: 0% 50%;
        }
    }

    .exp-progress {
        background: linear-gradient(-45deg, #667eea, #764ba2, #667eea, #764ba2);
        background-size: 400% 400%;
        animation: gradient-shift 3s ease infinite;
    }
</style>
