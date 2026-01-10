<script lang="ts">
	import LightSwitch from '$lib/components/LightSwitch.svelte';
	import LocaleSelector from '$lib/components/LocaleSelector.svelte';
	import { Toaster } from 'svelte-sonner';
	import { ModeWatcher } from 'mode-watcher';
	import { t, waitLocale } from 'svelte-intl-precompile';
	import '../app.css';

	let { children } = $props();
</script>

<Toaster position="top-right" />

<ModeWatcher />

{#await waitLocale() then}
	<div class="container m-0 h-screen max-h-full max-w-full p-0">
		<nav
			data-sveltekit-reload
			class="flex h-14 flex-row items-center justify-between bg-secondary-foreground p-4 dark:bg-black"
		>
			<div
				class="basis-1/8 me-5 inline-block text-lg font-bold text-secondary dark:text-secondary-foreground"
			>
				<a href="/" title={$t('headerLabels.backToHomeHint')}>PW</a>
			</div>
			<div class="flex items-center gap-2">
				<LocaleSelector />
				<LightSwitch />
			</div>
		</nav>

		<div
			class="dark:bg-dark-gray container w-full max-w-5xl rounded-b bg-white pb-6 pe-4 ps-4 pt-4 shadow-lg lg:p-10"
		>
			{@render children()}
		</div>
	</div>
{/await}
