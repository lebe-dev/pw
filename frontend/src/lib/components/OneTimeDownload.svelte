<script lang="ts">
	import { Checkbox } from '$lib/components/ui/checkbox/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { t } from 'svelte-intl-precompile';

	let { checked, disabled = $bindable(false), click } = $props();
</script>

<div class="flex items-center space-x-2">
	<Checkbox
		id="one-time-download"
		bind:checked
		aria-labelledby="one-time-download-label"
		class="rounded dark:checked:border-accent dark:checked:bg-accent"
		onclick={() => click()}
		{disabled}
	/>
	<span class:text-amber-500={!checked}>
		<Label
			id="one-time-download-label"
			for="one-time-download"
			class="cursor-pointer text-sm font-normal leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
		>
			{$t('homePage.lifetime.oneTimeDownload')}
		</Label>
	</span>
</div>
