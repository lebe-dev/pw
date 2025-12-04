import { AppConfig } from '$lib/config';
import type { LayoutLoad } from './$types';
import { getLocaleFromNavigator, init } from 'svelte-intl-precompile';
import { registerAll } from '$locales';

registerAll();

const LOCALE_STORAGE_KEY = 'pw-preferred-locale';

function getInitialLocale(): string | undefined {
	if (typeof window !== 'undefined') {
		const savedLocale = localStorage.getItem(LOCALE_STORAGE_KEY);
		if (savedLocale) {
			return savedLocale;
		}
	}
	return getLocaleFromNavigator() ?? undefined;
}

init({ initialLocale: getInitialLocale(), fallbackLocale: 'en' });

export const ssr = false;

export const load: LayoutLoad = async () => {
	return {
		config: new AppConfig()
	};
};
