<script lang="ts">
	import Check from 'lucide-svelte/icons/check';
	import { Button } from '$lib/components/ui/button/index.js';
	import { locale, t } from 'svelte-intl-precompile';
	import { onMount } from 'svelte';
	import { LOCALE_STORAGE_KEY } from '$lib/locale';

	let { availableLocales }: { availableLocales: string[] } = $props();

	let isOpen = $state(false);
	let buttonRef: HTMLElement | undefined = $state();

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

	function getLabel(localeId: string): string {
		return localeLabels[localeId] ?? localeId.toUpperCase();
	}

	function selectLocale(selectedLocale: string) {
		locale.set(selectedLocale);
		try {
			localStorage.setItem(LOCALE_STORAGE_KEY, selectedLocale);
		} catch (e) {
			console.error('unable to save preferred locale', e);
		}
		isOpen = false;
	}

	function toggleLocale() {
		const next = availableLocales.find((localeId) => localeId !== $locale) ?? availableLocales[0];
		selectLocale(next);
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

{#if availableLocales.length === 2}
	<Button
		onclick={toggleLocale}
		variant="outline"
		class="min-w-[3rem] gap-1 border-0 bg-transparent px-3 align-middle dark:bg-transparent"
		title={$t('headerLabels.selectLanguage')}
	>
		{#each availableLocales as localeOption, index}
			{#if index > 0}
				<span class="text-sm text-secondary opacity-50 dark:text-secondary-foreground">/</span>
			{/if}
			<span
				class="text-sm transition-all hover:text-primary dark:hover:text-primary {$locale ===
				localeOption
					? 'font-bold text-secondary dark:text-secondary-foreground'
					: 'font-medium text-secondary opacity-50 dark:text-secondary-foreground'}"
			>
				{getLabel(localeOption)}
			</span>
		{/each}
		<span class="sr-only">{$t('headerLabels.selectLanguage')}</span>
	</Button>
{:else if availableLocales.length > 2}
	<div class="relative" bind:this={buttonRef}>
		<Button
			onclick={toggleDropdown}
			variant="outline"
			class="min-w-[3rem] border-0 bg-transparent px-3 align-middle dark:bg-transparent"
			title={$t('headerLabels.selectLanguage')}
		>
			<span
				class="text-sm font-medium text-secondary transition-all hover:text-primary dark:text-secondary-foreground dark:hover:text-primary"
			>
				{getLabel($locale ?? availableLocales[0])}
			</span>
			<span class="sr-only">{$t('headerLabels.selectLanguage')}</span>
		</Button>

		{#if isOpen}
			<div
				class="absolute right-0 top-12 z-50 min-w-[8rem] overflow-hidden rounded-md border bg-white p-1 shadow-md dark:border-gray-700 dark:bg-gray-800"
			>
				{#each availableLocales as localeOption}
					<button
						onclick={() => selectLocale(localeOption)}
						class="flex w-full items-center justify-between rounded-sm px-2 py-1.5 text-sm text-gray-900 outline-none transition-colors hover:bg-primary hover:text-white focus:bg-primary focus:text-white dark:text-gray-200 dark:hover:bg-gray-200 dark:hover:text-gray-900 dark:focus:bg-gray-200 dark:focus:text-gray-900"
					>
						<span class="flex items-center gap-2">
							<span class="font-medium">{getLabel(localeOption)}</span>
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
{/if}
