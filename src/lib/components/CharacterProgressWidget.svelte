<script lang="ts">
    import {
        character,
        characterStats,
        experienceInfo,
        attributeInfo,
    } from "$lib/stores/characterStore";
    import { CLASS_ICONS, CLASS_DESCRIPTIONS } from "$lib/types/character";

    // Get top 3 attributes
    $: topAttributes = $attributeInfo
        ? [
              {
                  name: "Siła",
                  value: $attributeInfo.strength,
                  color: "#dc2626",
              },
              {
                  name: "Intelekt",
                  value: $attributeInfo.intelligence,
                  color: "#2563eb",
              },
              {
                  name: "Charyzma",
                  value: $attributeInfo.charisma,
                  color: "#7c3aed",
              },
              {
                  name: "Zręczność",
                  value: $attributeInfo.dexterity,
                  color: "#16a34a",
              },
              {
                  name: "Mądrość",
                  value: $attributeInfo.wisdom,
                  color: "#ca8a04",
              },
              {
                  name: "Kondycja",
                  value: $attributeInfo.constitution,
                  color: "#ea580c",
              },
          ]
              .sort((a, b) => b.value - a.value)
              .slice(0, 3)
        : [];

    $: classIcon = $character ? CLASS_ICONS[$character.character_class] : "⚔️";
    $: classDescription = $character
        ? CLASS_DESCRIPTIONS[$character.character_class]
        : "Wojownik";
</script>

<div class="character-progress-widget">
    {#if $character}
        <div class="widget-header">
            <h3 class="widget-title">🏆 Postać</h3>
            <div class="level-badge">
                Lv. {$character.level}
            </div>
        </div>

        <div class="character-overview">
            <div class="character-avatar">
                <span class="avatar-icon">{classIcon}</span>
                <div class="character-info">
                    <div class="character-name">
                        {classDescription}
                    </div>
                    <div class="character-exp">
                        {$character.experience.toLocaleString()} EXP
                    </div>
                </div>
            </div>

            {#if $experienceInfo}
                <div class="exp-section">
                    <div class="exp-progress">
                        <div class="exp-label">
                            Postęp do poziomu {$character.level + 1}
                        </div>
                        <div class="progress-bar">
                            <div
                                class="progress-fill"
                                style="width: {$experienceInfo.percentage}%"
                            ></div>
                        </div>
                        <div class="exp-details">
                            {$experienceInfo.current.toLocaleString()} / {$experienceInfo.needed.toLocaleString()}
                            EXP ({Math.round($experienceInfo.percentage)}%)
                        </div>
                    </div>
                </div>
            {/if}
        </div>

        {#if topAttributes.length > 0}
            <div class="attributes-section">
                <h4 class="section-title">💪 Najwyższe Atrybuty</h4>
                <div class="attributes-list">
                    {#each topAttributes as attr, index}
                        <div class="attribute-item" class:first={index === 0}>
                            <div
                                class="attribute-dot"
                                style="background-color: {attr.color}"
                            ></div>
                            <div class="attribute-content">
                                <div class="attribute-name">{attr.name}</div>
                                <div class="attribute-value">
                                    {attr.value} punktów
                                </div>
                            </div>
                            <div class="attribute-rank">#{index + 1}</div>
                        </div>
                    {/each}
                </div>
            </div>
        {/if}

        {#if $attributeInfo}
            <div class="summary-section">
                <div class="summary-stat">
                    <span class="summary-icon">⭐</span>
                    <span class="summary-text">
                        Suma atrybutów: <strong>{$attributeInfo.total}</strong>
                    </span>
                </div>
                {#if $characterStats}
                    <div class="summary-stat">
                        <span class="summary-icon">📈</span>
                        <span class="summary-text">
                            Postęp: <strong
                                >{Math.round(
                                    $characterStats.levelProgress * 100,
                                )}%</strong
                            > do następnego poziomu
                        </span>
                    </div>
                {/if}
            </div>
        {/if}
    {:else}
        <div class="loading-state">
            <div class="loading-icon">⏳</div>
            <div class="loading-text">Ładowanie postaci...</div>
        </div>
    {/if}
</div>

<style>
    .character-progress-widget {
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

    .level-badge {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        padding: 0.25rem 0.75rem;
        border-radius: 1rem;
        font-size: 0.875rem;
        font-weight: 600;
    }

    .character-overview {
        margin-bottom: 1.5rem;
    }

    .character-avatar {
        display: flex;
        align-items: center;
        gap: 1rem;
        margin-bottom: 1rem;
    }

    .avatar-icon {
        font-size: 3rem;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        padding: 1rem;
        border-radius: 1rem;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    }

    .character-info {
        flex: 1;
    }

    .character-name {
        font-size: 1.25rem;
        font-weight: 600;
        color: #1e293b;
        margin-bottom: 0.25rem;
    }

    .character-exp {
        font-size: 0.875rem;
        color: #64748b;
        font-weight: 500;
    }

    .exp-section {
        padding: 1rem;
        background: #f8fafc;
        border-radius: 0.75rem;
    }

    .exp-label {
        font-size: 0.875rem;
        color: #374151;
        font-weight: 600;
        margin-bottom: 0.75rem;
    }

    .progress-bar {
        background: #e2e8f0;
        border-radius: 1rem;
        height: 0.75rem;
        overflow: hidden;
        margin-bottom: 0.5rem;
    }

    .progress-fill {
        height: 100%;
        background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
        border-radius: 1rem;
        transition: width 0.3s ease;
    }

    .exp-details {
        text-align: center;
        font-size: 0.75rem;
        color: #64748b;
        font-weight: 600;
    }

    .attributes-section {
        flex: 1;
        margin-bottom: 1rem;
    }

    .section-title {
        margin: 0 0 1rem 0;
        font-size: 1rem;
        font-weight: 600;
        color: #374151;
    }

    .attributes-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .attribute-item {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        padding: 0.75rem;
        background: #f8fafc;
        border-radius: 0.5rem;
        transition: all 0.2s ease;
    }

    .attribute-item:hover {
        transform: translateY(-1px);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    .attribute-item.first {
        background: linear-gradient(135deg, #fbbf24 0%, #f59e0b 100%);
        color: white;
    }

    .attribute-item.first .attribute-name,
    .attribute-item.first .attribute-value {
        color: white;
    }

    .attribute-dot {
        width: 0.75rem;
        height: 0.75rem;
        border-radius: 50%;
        flex-shrink: 0;
    }

    .attribute-content {
        flex: 1;
    }

    .attribute-name {
        font-weight: 600;
        color: #1e293b;
        font-size: 0.875rem;
        margin-bottom: 0.125rem;
    }

    .attribute-value {
        font-size: 0.75rem;
        color: #64748b;
        font-weight: 500;
    }

    .attribute-rank {
        font-size: 0.75rem;
        font-weight: 600;
        color: #64748b;
        background: rgba(255, 255, 255, 0.8);
        padding: 0.25rem 0.5rem;
        border-radius: 0.375rem;
    }

    .attribute-item.first .attribute-rank {
        background: rgba(255, 255, 255, 0.9);
        color: #92400e;
    }

    .summary-section {
        padding-top: 1rem;
        border-top: 1px solid #e2e8f0;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .summary-stat {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.5rem;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        border-radius: 0.5rem;
        font-size: 0.75rem;
    }

    .summary-icon {
        font-size: 1rem;
    }

    .loading-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        text-align: center;
    }

    .loading-icon {
        font-size: 3rem;
        margin-bottom: 1rem;
        opacity: 0.6;
    }

    .loading-text {
        color: #64748b;
        font-weight: 500;
    }

    /* Dark mode support */
    @media (prefers-color-scheme: dark) {
        .character-progress-widget {
            background: rgba(15, 23, 42, 0.95);
            border: 1px solid rgba(71, 85, 105, 0.3);
        }

        .widget-title {
            color: #f1f5f9;
        }

        .section-title {
            color: #cbd5e1;
        }

        .character-name {
            color: #f1f5f9;
        }

        .character-exp {
            color: #94a3b8;
        }

        .exp-section,
        .attribute-item {
            background: #1e293b;
        }

        .exp-label {
            color: #cbd5e1;
        }

        .exp-details {
            color: #94a3b8;
        }

        .attribute-name {
            color: #f1f5f9;
        }

        .attribute-value {
            color: #94a3b8;
        }

        .attribute-rank {
            background: rgba(71, 85, 105, 0.8);
            color: #cbd5e1;
        }

        .progress-bar {
            background: #374151;
        }

        .summary-section {
            border-top-color: #374151;
        }

        .loading-text {
            color: #94a3b8;
        }
    }

    /* Responsive design */
    @media (max-width: 640px) {
        .character-avatar {
            gap: 0.75rem;
        }

        .avatar-icon {
            font-size: 2.5rem;
            padding: 0.75rem;
        }

        .character-name {
            font-size: 1.125rem;
        }

        .exp-section {
            padding: 0.75rem;
        }

        .attribute-item {
            gap: 0.5rem;
            padding: 0.5rem;
        }

        .attribute-name {
            font-size: 0.8rem;
        }

        .attribute-value {
            font-size: 0.7rem;
        }

        .summary-stat {
            font-size: 0.7rem;
        }
    }
</style>
