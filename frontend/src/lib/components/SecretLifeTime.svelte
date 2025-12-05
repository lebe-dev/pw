<script lang="ts">
	import { SecretTTL } from '$lib/secret';
	import { t } from 'svelte-intl-precompile';

	let { value = $bindable(SecretTTL.OneHour), disabled = $bindable(false), ...props } = $props();
</script>

<div class="mb-3">
	<a
		class="me-0 inline-block min-w-24 cursor-pointer rounded-s border p-2 text-sm"
		class:border-accent={value === SecretTTL.OneHour}
		class:cursor-not-allowed={disabled}
		class:opacity-50={disabled}
		onclick={() => !disabled && (value = SecretTTL.OneHour)}
	>
		{$t('homePage.lifetime.oneHour')}
	</a>

	<a
		class="inline-block min-w-24 cursor-pointer border p-2 text-sm"
		class:border-accent={value === SecretTTL.TwoHours}
		class:cursor-not-allowed={disabled}
		class:opacity-50={disabled}
		onclick={() => !disabled && (value = SecretTTL.TwoHours)}
	>
		{$t('homePage.lifetime.twoHours')}
	</a>

	<a
		class="mb-1 inline-block min-w-24 cursor-pointer border p-2 text-sm lg:mb-0"
		class:border-amber-500={value === SecretTTL.OneDay}
		class:cursor-not-allowed={disabled}
		class:opacity-50={disabled}
		onclick={() => !disabled && (value = SecretTTL.OneDay)}
	>
		{$t('homePage.lifetime.oneDay')}
	</a>

	<a
		class="inline-block min-w-24 cursor-pointer rounded-e border p-2 text-sm"
		class:border-destructive={value === SecretTTL.OneWeek}
		class:cursor-not-allowed={disabled}
		class:opacity-50={disabled}
		onclick={() => !disabled && (value = SecretTTL.OneWeek)}
	>
		{$t('homePage.lifetime.oneWeek')}
	</a>
</div>
