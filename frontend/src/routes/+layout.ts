import { fetchAppConfig } from '$lib/config';
import {
	getAvailableLocales,
	resolveInitialLocale,
	FALLBACK_LOCALE,
	LOCALE_STORAGE_KEY
} from '$lib/locale';
import type { LayoutLoad } from './$types';
import { getLocaleFromNavigator, init, waitLocale } from 'svelte-intl-precompile';
import { availableLocales as registeredLocales, registerAll } from '$locales';

registerAll();

export const ssr = false;

function getSavedLocale(): string | null {
	try {
		return localStorage.getItem(LOCALE_STORAGE_KEY);
	} catch {
		return null;
	}
}

// Config and locale are resolved before the first render, so the page appears once
// in its final state instead of re-rendering when the config or translations arrive.
export const load: LayoutLoad = async ({ fetch }) => {
	const config = await fetchAppConfig(fetch);

	const availableLocales = getAvailableLocales(config?.availableLocales, registeredLocales);
	const initialLocale = resolveInitialLocale(
		availableLocales,
		getSavedLocale(),
		getLocaleFromNavigator()
	);

	init({ initialLocale, fallbackLocale: FALLBACK_LOCALE });
	await waitLocale();

	return {
		config,
		availableLocales
	};
};
