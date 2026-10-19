export const LOCALE_STORAGE_KEY = 'pw-preferred-locale';
export const FALLBACK_LOCALE = 'en';

const localeOrder = ['en', 'ru', 'de', 'es', 'fr', 'ge', 'ch', 'ja'];

export function sortLocales(locales: string[]): string[] {
	return [...locales].sort((a, b) => {
		const indexA = localeOrder.indexOf(a);
		const indexB = localeOrder.indexOf(b);
		if (indexA === -1) return 1;
		if (indexB === -1) return -1;
		return indexA - indexB;
	});
}

/**
 * Locales offered to the user: the ones configured on the backend (in configured order),
 * limited to locales the frontend has translations for. Empty configuration means all.
 */
export function getAvailableLocales(
	configured: string[] | undefined,
	registered: string[]
): string[] {
	const available = (configured ?? []).filter((locale) => registered.includes(locale));
	if (available.length > 0) {
		return available;
	}
	return sortLocales(registered);
}

export function resolveInitialLocale(
	available: string[],
	saved: string | null,
	navigatorLocale: string | null | undefined
): string {
	if (saved && available.includes(saved)) {
		return saved;
	}

	if (navigatorLocale) {
		const language = navigatorLocale.toLowerCase().split('-')[0];
		if (available.includes(language)) {
			return language;
		}
	}

	return available[0] ?? FALLBACK_LOCALE;
}
