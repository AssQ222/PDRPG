<script lang="ts">
    import { habitActions, habitError, isLoading } from "../stores/habitStore";
    import type { HabitType } from "../types/habit";

    // State variables
    let title = "";
    let habitType: HabitType = "Boolean";
    let targetValue = 1;
    let showTargetValue = false;
    let isSubmitting = false;

    // Reactive statements
    $: showTargetValue = habitType === "Counter";
    $: canSubmit = title.trim().length > 0 && !isSubmitting && !$isLoading;

    // Event handlers
    async function handleSubmit() {
        if (!canSubmit) return;

        isSubmitting = true;

        try {
            await habitActions.addHabit(
                title.trim(),
                habitType,
                showTargetValue ? targetValue : undefined,
            );

            // Clear form on success
            title = "";
            habitType = "Boolean";
            targetValue = 1;
        } catch (error) {
            console.error("Failed to add habit:", error);
        } finally {
            isSubmitting = false;
        }
    }

    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter" && canSubmit) {
            handleSubmit();
        }
    }

    function clearError() {
        if ($habitError) {
            habitActions.clearError();
        }
    }
</script>

<div class="habit-input-container">
    <div class="glass-card p-6">
        <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-2">
            <span class="text-2xl">🎯</span>
            Add New Habit
        </h3>

        <!-- Error Display -->
        {#if $habitError}
            <div class="error-alert mb-4">
                <div class="flex items-center gap-2">
                    <span class="text-red-400">⚠️</span>
                    <span class="text-red-300">{$habitError.message}</span>
                    <button
                        on:click={clearError}
                        class="ml-auto text-red-300 hover:text-red-100 transition-colors"
                        aria-label="Clear error"
                    >
                        ✕
                    </button>
                </div>
            </div>
        {/if}

        <form on:submit|preventDefault={handleSubmit} class="space-y-4">
            <!-- Title Input -->
            <div class="form-group">
                <label for="habit-title" class="form-label"> Habit Name </label>
                <input
                    id="habit-title"
                    type="text"
                    bind:value={title}
                    on:keydown={handleKeydown}
                    on:input={clearError}
                    placeholder="e.g., Daily meditation, Drink water, Read books..."
                    class="form-input"
                    disabled={isSubmitting || $isLoading}
                    maxlength="50"
                    required
                />
                <p class="form-help-text">
                    Maximum 50 characters for better display
                </p>
            </div>

            <!-- Habit Type Selection -->
            <div class="form-group">
                <label for="habit-type" class="form-label"> Habit Type </label>
                <select
                    id="habit-type"
                    bind:value={habitType}
                    class="form-select"
                    disabled={isSubmitting || $isLoading}
                >
                    <option value="Boolean">Yes/No (Did I do it today?)</option>
                    <option value="Counter">Counter (How many times?)</option>
                </select>
            </div>

            <!-- Target Value (only for Counter type) -->
            {#if showTargetValue}
                <div class="form-group">
                    <label for="target-value" class="form-label">
                        Daily Target
                    </label>
                    <input
                        id="target-value"
                        type="number"
                        bind:value={targetValue}
                        min="1"
                        max="100"
                        class="form-input"
                        disabled={isSubmitting || $isLoading}
                        placeholder="e.g., 8 glasses of water"
                        required
                    />
                    <p class="form-help-text">
                        How many times per day do you want to achieve this
                        habit?
                    </p>
                </div>
            {/if}

            <!-- Submit Button -->
            <button
                type="submit"
                disabled={!canSubmit}
                class="submit-button"
                class:loading={isSubmitting || $isLoading}
            >
                {#if isSubmitting || $isLoading}
                    <div class="loading-spinner"></div>
                    Adding...
                {:else}
                    <span class="text-lg">➕</span>
                    Add Habit
                {/if}
            </button>
        </form>

        <!-- Help Text -->
        <div class="help-section mt-6 pt-4 border-t border-white/20">
            <h4 class="text-sm font-semibold text-white/80 mb-2">💡 Tips:</h4>
            <ul class="text-sm text-white/60 space-y-1">
                <li>
                    <strong>Yes/No habits:</strong> Perfect for daily practices like
                    meditation, exercise, journaling
                </li>
                <li>
                    <strong>Counter habits:</strong> Great for tracking quantities
                    like glasses of water, pages read, pushups
                </li>
                <li>
                    Start small and be consistent - even 1% improvement daily
                    compounds over time!
                </li>
            </ul>
        </div>
    </div>
</div>

<style>
    .habit-input-container {
        max-width: 500px;
        width: 100%;
    }

    .glass-card {
        background: rgba(255, 255, 255, 0.1);
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255, 255, 255, 0.2);
        border-radius: 16px;
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    }

    .form-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .form-label {
        font-weight: 600;
        color: rgba(255, 255, 255, 0.9);
        font-size: 0.875rem;
    }

    .form-input,
    .form-select {
        padding: 0.75rem 1rem;
        background: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.3);
        border-radius: 8px;
        color: white;
        font-size: 1rem;
        transition: all 0.2s ease;
    }

    .form-input:focus,
    .form-select:focus {
        outline: none;
        border-color: rgba(124, 58, 237, 0.8);
        box-shadow: 0 0 0 3px rgba(124, 58, 237, 0.3);
        background: rgba(255, 255, 255, 0.15);
    }

    .form-input::placeholder {
        color: rgba(255, 255, 255, 0.5);
    }

    .form-input:disabled,
    .form-select:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .form-help-text {
        font-size: 0.75rem;
        color: rgba(255, 255, 255, 0.6);
        margin-top: 0.25rem;
    }

    .submit-button {
        width: 100%;
        padding: 0.875rem 1.5rem;
        background: linear-gradient(135deg, #7c3aed, #a855f7);
        border: none;
        border-radius: 8px;
        color: white;
        font-weight: 600;
        font-size: 1rem;
        cursor: pointer;
        transition: all 0.2s ease;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5rem;
        min-height: 48px;
    }

    .submit-button:hover:not(:disabled) {
        background: linear-gradient(135deg, #6d28d9, #9333ea);
        transform: translateY(-1px);
        box-shadow: 0 4px 16px rgba(124, 58, 237, 0.4);
    }

    .submit-button:disabled {
        opacity: 0.6;
        cursor: not-allowed;
        transform: none;
        box-shadow: none;
    }

    .submit-button.loading {
        background: linear-gradient(135deg, #6d28d9, #9333ea);
    }

    .loading-spinner {
        width: 16px;
        height: 16px;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top: 2px solid white;
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    .error-alert {
        background: rgba(239, 68, 68, 0.1);
        border: 1px solid rgba(239, 68, 68, 0.3);
        border-radius: 8px;
        padding: 0.75rem;
    }

    .help-section {
        font-size: 0.875rem;
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }

    /* Mobile responsiveness */
    @media (max-width: 640px) {
        .glass-card {
            padding: 1rem;
        }

        .form-input,
        .form-select {
            padding: 0.625rem 0.875rem;
        }

        .submit-button {
            padding: 0.75rem 1.25rem;
        }
    }
</style>
