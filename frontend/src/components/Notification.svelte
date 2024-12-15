<script>
	export let notification;
	export let onRemove;
</script>

<div class="notification flex flex-row p-3 justify-between rounded shadow-sm mb-1"
		 class:border={notification.type === 'info'}
		 class:border-primary={notification.type === 'info'}
		 class:text-dark={notification.type === 'warn'}
		 class:bg-light={notification.type === 'info'}
		 class:bg-warning={notification.type === 'warn'}
		 class:text-white={notification.type === 'error'}
		 class:bg-red-600={notification.type === 'error'}>
	<div class="col vertical-align">
		<div class="mt-2 mb-2">{notification.message}</div>
	</div>
	<div class="col-1 me-2">
		<button
			class="inline-block mb-0 pb-0 mt-0.5"
			class:text-primary={notification.type === 'info'}
			class:text-dark={notification.type === 'warn'}
			class:text-white={notification.type === 'error'}
			on:click={() => onRemove(notification.id)}>×</button>
	</div>
</div>

<style>
	.notification {
		font-size: 0.9rem;
	}

    button {
        background: none;
        border: none;
        font-size: 1.3rem;
        cursor: pointer;
    }
</style>