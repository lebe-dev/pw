<script lang="ts">
	import { copy } from 'svelte-copy';
	import { Button } from './ui/button';

	let { data, label } = $props();

	let flag = $state(false);

	function toggle() {
		flag = true;
		setTimeout(() => {
			flag = false;
		}, 500);
	}
</script>

<div use:copy={data}>
	<Button size="sm" class="h-8" onclick={toggle} disabled={flag}>
		<div class="flex items-center">
			{#if flag}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="me-1 inline-block h-3.5 w-3.5"
				>
					<path stroke-linecap="round" stroke-linejoin="round" d="m4.5 12.75 6 6 9-13.5" />
				</svg>{label}
			{:else}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="24"
					height="24"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					stroke-linecap="round"
					stroke-linejoin="round"
					class="lucide lucide-clipboard me-1 inline-block"
					><rect width="8" height="4" x="8" y="2" rx="1" ry="1" /><path
						d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"
					/></svg
				>{label}
			{/if}
		</div>
	</Button>
</div>
