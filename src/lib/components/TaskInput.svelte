<script lang="ts">
    import { taskActions, isLoading, taskError } from "../stores/taskStore";

    /** Wartość pola input */
    let inputValue = "";
    /** Referencja do elementu input */
    let inputElement: HTMLInputElement;

    /**
     * Obsługuje dodawanie nowego zadania
     */
    async function handleSubmit() {
        if (!inputValue.trim()) return;

        await taskActions.addTask(inputValue);

        // Wyczyść pole input po pomyślnym dodaniu
        if (!$taskError) {
            inputValue = "";
        }

        // Ustaw focus z powrotem na input
        inputElement?.focus();
    }

    /**
     * Obsługuje naciśnięcie klawisza Enter
     */
    function handleKeydown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            event.preventDefault();
            handleSubmit();
        }
    }

    /**
     * Czyści błędy gdy użytkownik zaczyna pisać
     */
    function handleInput() {
        if ($taskError) {
            taskActions.clearError();
        }
    }
</script>

<div class="task-input-container">
    <div class="input-group">
        <input
            bind:this={inputElement}
            bind:value={inputValue}
            on:keydown={handleKeydown}
            on:input={handleInput}
            placeholder="Dodaj nowe zadanie..."
            disabled={$isLoading}
            class="task-input"
            type="text"
            maxlength="100"
            aria-label="Nowe zadanie"
        />
        <button
            on:click={handleSubmit}
            disabled={$isLoading || !inputValue.trim()}
            class="add-button"
            type="button"
            aria-label="Dodaj zadanie"
        >
            {#if $isLoading}
                <span class="loading-spinner" aria-hidden="true"></span>
                Dodawanie...
            {:else}
                <span class="plus-icon" aria-hidden="true">+</span>
                Dodaj
            {/if}
        </button>
    </div>

    {#if $taskError}
        <div class="error-message" role="alert">
            <span class="error-icon" aria-hidden="true">⚠️</span>
            {$taskError.message}
        </div>
    {/if}
</div>

<style>
    .task-input-container {
        width: 100%;
        margin-bottom: 1.5rem;
    }

    .input-group {
        display: flex;
        gap: 0.5rem;
        align-items: stretch;
    }

    .task-input {
        flex: 1;
        padding: 0.75rem 1rem;
        border: 2px solid #e5e7eb;
        border-radius: 0.5rem;
        font-size: 1rem;
        transition:
            border-color 0.2s ease,
            box-shadow 0.2s ease;
        background-color: white;
    }

    .task-input:focus {
        outline: none;
        border-color: #3b82f6;
        box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
    }

    .task-input:disabled {
        background-color: #f9fafb;
        color: #9ca3af;
        cursor: not-allowed;
    }

    .add-button {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        padding: 0.75rem 1.5rem;
        background-color: #3b82f6;
        color: white;
        border: none;
        border-radius: 0.5rem;
        font-size: 1rem;
        font-weight: 500;
        cursor: pointer;
        transition:
            background-color 0.2s ease,
            transform 0.1s ease;
        white-space: nowrap;
    }

    .add-button:hover:not(:disabled) {
        background-color: #2563eb;
        transform: translateY(-1px);
    }

    .add-button:active:not(:disabled) {
        transform: translateY(0);
    }

    .add-button:disabled {
        background-color: #9ca3af;
        cursor: not-allowed;
        transform: none;
    }

    .plus-icon {
        font-size: 1.25rem;
        font-weight: bold;
        line-height: 1;
    }

    .loading-spinner {
        width: 1rem;
        height: 1rem;
        border: 2px solid rgba(255, 255, 255, 0.3);
        border-top: 2px solid white;
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

    .error-message {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        margin-top: 0.5rem;
        padding: 0.75rem;
        background-color: #fef2f2;
        border: 1px solid #fecaca;
        border-radius: 0.375rem;
        color: #dc2626;
        font-size: 0.875rem;
    }

    .error-icon {
        font-size: 1rem;
    }

    /* Responsive design */
    @media (max-width: 640px) {
        .input-group {
            flex-direction: column;
        }

        .add-button {
            justify-content: center;
        }
    }
</style>
