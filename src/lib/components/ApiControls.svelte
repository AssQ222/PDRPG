<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let apiPort = 3000;
    let isApiRunning = false;
    let isLoading = false;
    let message = "";

    async function checkApiStatus() {
        try {
            isApiRunning = await invoke("check_api_status", { port: apiPort });
        } catch (error) {
            console.error("Failed to check API status:", error);
            isApiRunning = false;
        }
    }

    async function startApiServer() {
        try {
            isLoading = true;
            message = "";

            const result = await invoke("start_api_server", { port: apiPort });
            message = result as string;

            // Wait a moment then check status
            setTimeout(async () => {
                await checkApiStatus();
                isLoading = false;
            }, 2000);
        } catch (error) {
            console.error("Failed to start API server:", error);
            message = `Error: ${error}`;
            isLoading = false;
        }
    }

    async function testApiEndpoint() {
        try {
            const response = await fetch(
                `http://localhost:${apiPort}/api/health`,
            );
            const data = await response.json();
            message = `API Test Success: ${JSON.stringify(data, null, 2)}`;
        } catch (error) {
            message = `API Test Failed: ${error}`;
        }
    }

    onMount(() => {
        checkApiStatus();
    });
</script>

<div class="bg-white/10 backdrop-blur-md rounded-lg border border-white/20 p-6">
    <h2 class="text-xl font-bold text-white mb-4">🌐 API Server Controls</h2>

    <div class="space-y-4">
        <!-- Port Configuration -->
        <div class="flex items-center gap-3">
            <label for="apiPort" class="text-white font-medium">Port:</label>
            <input
                id="apiPort"
                type="number"
                bind:value={apiPort}
                class="px-3 py-1 rounded bg-white/20 text-white border border-white/30 w-20"
                min="1000"
                max="65535"
            />
            <span class="text-white/70 text-sm">
                Status:
                <span
                    class="font-medium {isApiRunning
                        ? 'text-green-300'
                        : 'text-red-300'}"
                >
                    {isApiRunning ? "🟢 Running" : "🔴 Stopped"}
                </span>
            </span>
        </div>

        <!-- Controls -->
        <div class="flex gap-3 flex-wrap">
            <button
                on:click={startApiServer}
                disabled={isLoading || isApiRunning}
                class="px-4 py-2 bg-blue-500/80 hover:bg-blue-500 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
                {#if isLoading}
                    🔄 Starting...
                {:else if isApiRunning}
                    ✅ Running
                {:else}
                    🚀 Start API Server
                {/if}
            </button>

            <button
                on:click={checkApiStatus}
                class="px-4 py-2 bg-gray-500/80 hover:bg-gray-500 text-white rounded-lg font-medium transition-colors"
            >
                🔍 Check Status
            </button>

            <button
                on:click={testApiEndpoint}
                disabled={!isApiRunning}
                class="px-4 py-2 bg-green-500/80 hover:bg-green-500 text-white rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
                🧪 Test API
            </button>
        </div>

        <!-- Message Display -->
        {#if message}
            <div class="bg-black/30 rounded-lg p-3 border border-white/20">
                <pre
                    class="text-white/90 text-sm whitespace-pre-wrap">{message}</pre>
            </div>
        {/if}

        <!-- API Documentation -->
        {#if isApiRunning}
            <div
                class="bg-blue-500/20 rounded-lg p-4 border border-blue-300/30"
            >
                <h3 class="text-white font-medium mb-2">
                    📋 Available Endpoints:
                </h3>
                <ul class="text-white/80 text-sm space-y-1">
                    <li>
                        • <code class="bg-black/30 px-1 rounded"
                            >GET /api/health</code
                        > - Server health check
                    </li>
                    <li>
                        • <code class="bg-black/30 px-1 rounded"
                            >GET /api/tasks</code
                        > - Get all tasks
                    </li>
                    <li>
                        • <code class="bg-black/30 px-1 rounded"
                            >GET /api/habits</code
                        > - Get all habits with today's entries
                    </li>
                    <li>
                        • <code class="bg-black/30 px-1 rounded"
                            >GET /api/character</code
                        > - Get character data
                    </li>
                </ul>
                <p class="text-white/60 text-xs mt-2">
                    Base URL: <code class="bg-black/30 px-1 rounded"
                        >http://localhost:{apiPort}</code
                    >
                </p>
            </div>
        {/if}
    </div>
</div>
