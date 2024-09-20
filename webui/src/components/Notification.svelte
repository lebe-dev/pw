<script>
	export let notification;
	export let onRemove;
</script>

<div class="flex flex-row p-3 justify-between rounded shadow-sm mb-1"
		 class:border={notification.type === 'info'}
		 class:border-primary={notification.type === 'info'}
		 class:text-dark={notification.type === 'warn'}
		 class:bg-light={notification.type === 'info'}
		 class:bg-warning={notification.type === 'warn'}
		 class:text-white={notification.type === 'error'}
		 class:bg-error={notification.type === 'error'}>
	<div class="col vertical-align">
		<div class="mt-2 mb-1">{notification.message}</div>
	</div>
	<div class="col-1 me-2">
		<button
			class="d-inline-block"
			class:text-primary={notification.type === 'info'}
			class:text-dark={notification.type === 'warn'}
			class:text-white={notification.type === 'error'}
			on:click={() => onRemove(notification.id)}>×</button>
	</div>
</div>

<style>
	.notification {
		font-size: 0.95rem;
	}

    button {
        background: none;
        border: none;
        font-size: 1.3rem;
        cursor: pointer;
    }
</style>