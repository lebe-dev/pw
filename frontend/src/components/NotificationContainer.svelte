<script>
	import {fade, fly} from 'svelte/transition';
	import Notification from './Notification.svelte';
	import {notifications} from "$lib/stores/notifications";

	export let position = 'top-right';
	export let max = 5;
</script>

<div class="notification-container {position} me-3">
	{#each $notifications.slice(0, max) as notification (notification.id)}
		<div in:fly="{{ y: -50, duration: 300 }}" out:fade="{{ duration: 300 }}">
			<Notification
				{notification}
				onRemove={(id) => notifications.remove(id)}
			/>
		</div>
	{/each}
</div>

<style>
    .notification-container {
        position: fixed;

        display: flex;
        flex-direction: column;
        padding: 10px;

        z-index: 1000;

				min-width: 300px;
				max-width: 550px;
    }

    .top-right {
        top: 0;
        right: 0;
    }

    .top-left {
        top: 0;
        left: 0;
    }

    .bottom-right {
        bottom: 0;
        right: 0;
    }

    .bottom-left {
        bottom: 0;
        left: 0;
    }
</style>