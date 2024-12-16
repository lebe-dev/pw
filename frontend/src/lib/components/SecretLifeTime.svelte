<script lang="ts">
	import { SecretTTL } from '$lib/secret';
	import { t } from 'svelte-intl-precompile';

	let { value = $bindable(SecretTTL.OneHour), disabled = $bindable(false), ...props } = $props();
</script>

<div class="mb-3">
	<a
		class="me-0 inline-block w-24 cursor-pointer rounded-s border p-2 text-sm"
		class:border-accent={value === SecretTTL.OneHour}
		onclick={() => (value = SecretTTL.OneHour)}
	>
		{$t('homePage.lifetime.oneHour')}
	</a>

	<a
		class="inline-block w-24 cursor-pointer border p-2 text-sm"
		class:border-accent={value === SecretTTL.TwoHours}
		onclick={() => (value = SecretTTL.TwoHours)}
	>
		{$t('homePage.lifetime.twoHours')}
	</a>

	<a
		class="mb-1 inline-block w-24 cursor-pointer border p-2 text-sm lg:mb-0"
		class:border-amber-500={value === SecretTTL.OneDay}
		onclick={() => (value = SecretTTL.OneDay)}
	>
		{$t('homePage.lifetime.oneDay')}
	</a>

	<a
		class="inline-block w-24 cursor-pointer rounded-e border p-2 text-sm"
		class:border-destructive={value === SecretTTL.OneWeek}
		onclick={() => (value = SecretTTL.OneWeek)}
	>
		{$t('homePage.lifetime.oneWeek')}
	</a>
</div>
