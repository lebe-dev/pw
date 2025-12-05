<script lang="ts">
	import Check from 'lucide-svelte/icons/check';
	import { Button } from '$lib/components/ui/button/index.js';
	import { locale, locales, t } from 'svelte-intl-precompile';
	import { onMount } from 'svelte';

	let isOpen = $state(false);
	let buttonRef: HTMLElement;

	const LOCALE_STORAGE_KEY = 'pw-preferred-locale';

	const localeOrder = ['en', 'ru', 'de', 'es', 'fr', 'ge', 'ch', 'ja'];

	const localeLabels: Record<string, string> = {
		en: 'EN',
		ru: 'RU',
		de: 'DE',
		es: 'ES',
		fr: 'FR',
		ge: 'GE',
		ch: 'CH',
		ja: 'JA'
	};

	function getSortedLocales(locales: string[]): string[] {
		return locales.sort((a, b) => {
			const indexA = localeOrder.indexOf(a);
			const indexB = localeOrder.indexOf(b);
			if (indexA === -1) return 1;
			if (indexB === -1) return -1;
			return indexA - indexB;
		});
	}

	function selectLocale(selectedLocale: string) {
		locale.set(selectedLocale);
		// Save to localStorage
		localStorage.setItem(LOCALE_STORAGE_KEY, selectedLocale);
		isOpen = false;
	}

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	function handleClickOutside(event: MouseEvent) {
		if (buttonRef && !buttonRef.contains(event.target as Node)) {
			isOpen = false;
		}
	}

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
		return () => {
			document.removeEventListener('click', handleClickOutside);
		};
	});
</script>

<div class="relative" bind:this={buttonRef}>
	<Button
		onclick={toggleDropdown}
		variant="outline"
		class="border-0 bg-transparent align-middle dark:bg-transparent px-3 min-w-[3rem]"
		title={$t('headerLabels.selectLanguage')}
	>
		<span class="text-sm font-medium text-secondary transition-all hover:text-primary dark:text-secondary-foreground dark:hover:text-primary">
			{localeLabels[$locale]}
		</span>
		<span class="sr-only">{$t('headerLabels.selectLanguage')}</span>
	</Button>

	{#if isOpen}
		<div
			class="absolute right-0 top-12 z-50 min-w-[8rem] overflow-hidden rounded-md border bg-white p-1 shadow-md dark:border-gray-700 dark:bg-gray-800"
		>
			{#each getSortedLocales([...$locales]) as localeOption}
				<button
					onclick={() => selectLocale(localeOption)}
					class="flex w-full items-center justify-between rounded-sm px-2 py-1.5 text-sm text-gray-900 outline-none transition-colors hover:bg-primary hover:text-white focus:bg-primary focus:text-white dark:text-gray-200 dark:hover:bg-gray-200 dark:hover:text-gray-900 dark:focus:bg-gray-200 dark:focus:text-gray-900"
				>
					<span class="flex items-center gap-2">
						<span class="font-medium">{localeLabels[localeOption]}</span>
						<span class="opacity-70">{$t(`languages.${localeOption}`)}</span>
					</span>
					{#if $locale === localeOption}
						<Check class="h-4 w-4" />
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>
