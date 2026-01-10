<script lang="ts">
	import { SecretTTL } from '$lib/secret';
	import { t } from 'svelte-intl-precompile';

	let { value = $bindable(SecretTTL.OneHour), disabled = $bindable(false) } = $props();
</script>

<div class="mb-3">
	<button
		type="button"
		class="me-0 inline-block min-w-24 cursor-pointer rounded-s border p-2 text-sm"
		class:border-accent={value === SecretTTL.OneHour}
		class:cursor-not-allowed={disabled}
		class:opacity-50={disabled}
		onclick={() => !disabled && (value = SecretTTL.OneHour)}
		{disabled}
	>
		{$t('homePage.lifetime.oneHour')}
	</button>

	<button
		type="button"
		class="inline-block min-w-24 cursor-pointer border p-2 text-sm"
		class:border-accent={value === SecretTTL.TwoHours}
		class:cursor-not-allowed={disabled}
		class:opacity-50={disabled}
		onclick={() => !disabled && (value = SecretTTL.TwoHours)}
		{disabled}
	>
		{$t('homePage.lifetime.twoHours')}
	</button>

	<button
		type="button"
		class="mb-1 inline-block min-w-24 cursor-pointer border p-2 text-sm lg:mb-0"
		class:border-amber-500={value === SecretTTL.OneDay}
		class:cursor-not-allowed={disabled}
		class:opacity-50={disabled}
		onclick={() => !disabled && (value = SecretTTL.OneDay)}
		{disabled}
	>
		{$t('homePage.lifetime.oneDay')}
	</button>

	<button
		type="button"
		class="inline-block min-w-24 cursor-pointer rounded-e border p-2 text-sm"
		class:border-destructive={value === SecretTTL.OneWeek}
		class:cursor-not-allowed={disabled}
		class:opacity-50={disabled}
		onclick={() => !disabled && (value = SecretTTL.OneWeek)}
		{disabled}
	>
		{$t('homePage.lifetime.oneWeek')}
	</button>
</div>
