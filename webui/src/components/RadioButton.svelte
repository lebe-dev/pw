<script lang="ts">
    export let text: string;
    export let enabled: boolean;

    export let toggle = () => {};
</script>

<div class="inline-block cursor-pointer select-none me-1" on:click={toggle}>
    {#if enabled}
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
             class="bi bi-record-circle text-accent inline mb-0.5 me-0.5" viewBox="0 0 16 16">
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14m0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16"/>
            <path d="M11 8a3 3 0 1 1-6 0 3 3 0 0 1 6 0"/>
        </svg> {text}
    {:else}
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor"
             class="bi bi-circle text-accent inline mb-0.5 me-0.5" viewBox="0 0 16 16">
            <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14m0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16"/>
        </svg> {text}
    {/if}
</div>