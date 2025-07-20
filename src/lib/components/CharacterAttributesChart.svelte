<!-- CharacterAttributesChart.svelte - Wykres pajęczynowy atrybutów postaci -->
<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { Chart, registerables } from "chart.js";
    import {
        character,
        characterActions,
        isLoading,
    } from "../stores/characterStore";
    import { ATTRIBUTE_NAMES } from "../types/character";
    import type { CharacterAttributes } from "../types/character";

    // Register Chart.js components
    Chart.register(...registerables);

    // Chart instance and canvas reference
    let chartCanvas: HTMLCanvasElement;
    let chartInstance: Chart | null = null;

    // Chart configuration
    const chartConfig = {
        type: "radar" as const,
        data: {
            labels: Object.values(ATTRIBUTE_NAMES),
            datasets: [
                {
                    label: "Atrybuty Postaci",
                    data: [10, 10, 10, 10, 10, 10], // Domyślne wartości
                    backgroundColor: "rgba(168, 85, 247, 0.1)", // Purple with transparency
                    borderColor: "rgba(168, 85, 247, 1)", // Solid purple
                    borderWidth: 2,
                    pointBackgroundColor: "rgba(168, 85, 247, 1)",
                    pointBorderColor: "#fff",
                    pointBorderWidth: 2,
                    pointRadius: 5,
                    pointHoverRadius: 7,
                    fill: true,
                },
            ],
        },
        options: {
            responsive: true,
            maintainAspectRatio: true,
            plugins: {
                title: {
                    display: true,
                    text: "Wykres Atrybutów Postaci",
                    color: "rgba(0, 0, 0, 1.0)",
                    font: {
                        size: 18,
                        weight: "bold" as const,
                    },
                },
                legend: {
                    display: false, // Ukrywamy legendę bo mamy jeden dataset
                },
                tooltip: {
                    backgroundColor: "rgba(0, 0, 0, 0.8)",
                    titleColor: "white",
                    bodyColor: "white",
                    borderColor: "rgba(168, 85, 247, 1)",
                    borderWidth: 1,
                    callbacks: {
                        label: function (context: any) {
                            const label = context.label || "";
                            const value = context.parsed.r || 0;
                            return `${label}: ${value} punktów`;
                        },
                    },
                },
            },
            scales: {
                r: {
                    beginAtZero: true,
                    min: 0,
                    max: 100, // Maksymalna wartość na wykresie
                    ticks: {
                        display: false, // Ukrywamy cyfry na wykresie
                        stepSize: 20,
                    },
                    grid: {
                        display: true,
                        color: "rgba(0, 0, 0, 0.3)",
                        lineWidth: 1,
                        circular: true,
                    },
                    angleLines: {
                        display: true,
                        color: "rgba(0, 0, 0, 0.4)",
                        lineWidth: 1,
                    },
                    pointLabels: {
                        color: "rgba(0, 0, 0, 1.0)",
                        font: {
                            size: 14,
                            weight: "bolder" as const,
                        },
                    },
                },
            },
            elements: {
                line: {
                    tension: 0.1, // Lekko zakrzywione linie
                },
            },
            interaction: {
                intersect: false,
            },
        },
    };

    // Function to update chart data
    function updateChartData(attributes: CharacterAttributes) {
        if (!chartInstance) return;

        const data = [
            attributes.strength,
            attributes.intelligence,
            attributes.charisma,
            attributes.dexterity,
            attributes.wisdom,
            attributes.constitution,
        ];

        chartInstance.data.datasets[0].data = data;
        chartInstance.update(); // Force update of chart
    }

    // Function to get attribute color based on value
    function getAttributeColor(value: number): string {
        if (value >= 30) return "rgba(34, 197, 94, 0.8)"; // Green for high values
        if (value >= 20) return "rgba(168, 85, 247, 0.8)"; // Purple for medium values
        if (value >= 15) return "rgba(59, 130, 246, 0.8)"; // Blue for decent values
        return "rgba(156, 163, 175, 0.8)"; // Gray for low values
    }

    // Initialize chart
    onMount(() => {
        if (chartCanvas) {
            chartInstance = new Chart(chartCanvas, chartConfig);

            // Load character data if available
            if ($character) {
                updateChartData($character.attributes);
            } else {
                // Initialize character if not loaded
                characterActions.initialize();
            }
        }
    });

    // Update chart when character data changes
    $: if (chartInstance && $character) {
        updateChartData($character.attributes);
    }

    // Clean up chart on component destroy
    onDestroy(() => {
        if (chartInstance) {
            chartInstance.destroy();
            chartInstance = null;
        }
    });

    // Calculate total attribute points
    $: totalPoints = $character
        ? Object.values($character.attributes).reduce(
              (sum, value) => sum + value,
              0,
          )
        : 60;

    // Get highest attribute
    $: highestAttribute = $character
        ? Object.entries($character.attributes).reduce(
              (max, [key, value]) => (value > max.value ? { key, value } : max),
              { key: "strength", value: 0 },
          )
        : { key: "strength", value: 10 };

    // Get attribute name in Polish
    $: highestAttributeName =
        ATTRIBUTE_NAMES[highestAttribute.key as keyof typeof ATTRIBUTE_NAMES];
</script>

<!-- Main Chart Container -->
<div class="w-full max-w-4xl mx-auto">
    <!-- Loading State -->
    {#if $isLoading}
        <div
            class="bg-white/10 backdrop-blur-md rounded-xl p-8 border border-white/20 shadow-xl"
        >
            <div class="flex items-center justify-center space-x-3">
                <div
                    class="animate-spin rounded-full h-8 w-8 border-b-2 border-purple-400"
                ></div>
                <span class="text-white/80 text-lg">Ładowanie atrybutów...</span
                >
            </div>
        </div>

        <!-- Chart Display -->
    {:else if $character}
        <div
            class="bg-white/10 backdrop-blur-md rounded-xl p-6 border border-white/20 shadow-xl"
        >
            <!-- Chart Header -->
            <div class="mb-6 text-center">
                <h2 class="text-2xl font-bold text-white mb-2">
                    🕷️ Wykres Atrybutów Postaci
                </h2>
                <p class="text-white/70 text-sm">
                    Wizualizacja rozwoju wszystkich atrybutów twojej postaci
                </p>
            </div>

            <!-- Chart Canvas Container -->
            <div class="relative mb-6">
                <div class="w-full max-w-md mx-auto bg-black/20 rounded-xl p-4">
                    <canvas
                        bind:this={chartCanvas}
                        class="w-full h-full"
                        style="max-height: 400px;"
                    ></canvas>
                </div>
            </div>

            <!-- Statistics Summary -->
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mt-6">
                <!-- Total Points -->
                <div class="bg-white/5 rounded-lg p-4 text-center">
                    <div class="text-2xl font-bold text-purple-400">
                        {totalPoints}
                    </div>
                    <div class="text-white/60 text-sm">
                        Suma wszystkich atrybutów
                    </div>
                </div>

                <!-- Highest Attribute -->
                <div class="bg-white/5 rounded-lg p-4 text-center">
                    <div class="text-2xl font-bold text-green-400">
                        {highestAttribute.value}
                    </div>
                    <div class="text-white/60 text-sm">
                        Najwyższy: {highestAttributeName}
                    </div>
                </div>

                <!-- Average -->
                <div class="bg-white/5 rounded-lg p-4 text-center">
                    <div class="text-2xl font-bold text-blue-400">
                        {Math.round((totalPoints / 6) * 10) / 10}
                    </div>
                    <div class="text-white/60 text-sm">
                        Średnia wartość atrybutu
                    </div>
                </div>
            </div>

            <!-- Attribute Details -->
            <div class="grid grid-cols-2 md:grid-cols-3 gap-3 mt-6">
                {#each Object.entries(ATTRIBUTE_NAMES) as [key, name]}
                    {@const value =
                        $character.attributes[
                            key as keyof typeof $character.attributes
                        ]}
                    {@const percentage = (value / 100) * 100}
                    <div class="bg-white/5 rounded-lg p-3">
                        <div class="flex items-center justify-between mb-2">
                            <span class="text-white/80 text-sm font-medium"
                                >{name}</span
                            >
                            <span class="text-white font-bold">{value}</span>
                        </div>
                        <!-- Progress Bar -->
                        <div class="w-full bg-white/10 rounded-full h-2">
                            <div
                                class="h-2 rounded-full transition-all duration-300"
                                class:bg-green-500={value >= 30}
                                class:bg-purple-500={value >= 20 && value < 30}
                                class:bg-blue-500={value >= 15 && value < 20}
                                class:bg-gray-500={value < 15}
                                style="width: {Math.min(100, percentage)}%"
                            ></div>
                        </div>
                    </div>
                {/each}
            </div>

            <!-- Helpful Tips -->
            <div class="mt-6 bg-white/5 rounded-lg p-4">
                <h3 class="text-white font-semibold mb-2">
                    💡 Wskazówki rozwoju:
                </h3>
                <div class="text-white/70 text-sm space-y-1">
                    <p>
                        • <strong>Siła:</strong> Dodaj zadania ze słowami "sport",
                        "trening", "fitness"
                    </p>
                    <p>
                        • <strong>Intelekt:</strong> Zadania z "nauka", "książka",
                        "kurs", "czytanie"
                    </p>
                    <p>
                        • <strong>Charyzma:</strong> Zadania z "prezentacja", "spotkanie",
                        "kontakt"
                    </p>
                    <p>
                        • <strong>Zręczność:</strong> Zadania z "hobby", "praktyka",
                        "projekt"
                    </p>
                    <p>
                        • <strong>Mądrość:</strong> Zadania z "medytacja", "refleksja",
                        "planowanie"
                    </p>
                    <p>
                        • <strong>Kondycja:</strong> Nawyki "sen", "dieta", "zdrowie",
                        "woda"
                    </p>
                </div>
            </div>
        </div>

        <!-- No Character State -->
    {:else}
        <div
            class="bg-white/10 backdrop-blur-md rounded-xl p-8 border border-white/20 shadow-xl text-center"
        >
            <div class="mb-4">
                <span class="text-6xl">📊</span>
            </div>
            <h3 class="text-white text-xl mb-2">Brak danych postaci</h3>
            <p class="text-white/60 mb-4">
                Najpierw utwórz postać, aby zobaczyć wykres atrybutów.
            </p>
            <button
                on:click={() => characterActions.initialize()}
                class="bg-purple-500 hover:bg-purple-600 text-white px-6 py-3 rounded-lg transition-colors"
            >
                Załaduj postać
            </button>
        </div>
    {/if}
</div>

<style>
    /* Dodatkowe style dla canvas */
    canvas {
        filter: drop-shadow(0 4px 6px rgba(0, 0, 0, 0.1));
    }

    /* Animation dla loading spinner */
    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .animate-spin {
        animation: spin 1s linear infinite;
    }

    /* Responsive adjustments */
    @media (max-width: 768px) {
        canvas {
            max-height: 300px;
        }
    }
</style>
