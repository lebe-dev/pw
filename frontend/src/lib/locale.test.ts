import { describe, expect, it } from 'vitest';
import { getAvailableLocales, resolveInitialLocale, sortLocales } from './locale';

describe('sortLocales', () => {
	it('orders known locales by preferred order and puts unknown ones last', () => {
		expect(sortLocales(['ja', 'xx', 'ru', 'en'])).toEqual(['en', 'ru', 'ja', 'xx']);
	});

	it('does not mutate the input', () => {
		const input = ['ru', 'en'];
		sortLocales(input);
		expect(input).toEqual(['ru', 'en']);
	});
});

describe('getAvailableLocales', () => {
	const registered = ['ru', 'en', 'de', 'ja'];

	it('returns all registered locales sorted when nothing is configured', () => {
		expect(getAvailableLocales([], registered)).toEqual(['en', 'ru', 'de', 'ja']);
	});

	it('keeps configured order and drops unregistered locales', () => {
		expect(getAvailableLocales(['de', 'xx', 'en'], registered)).toEqual(['de', 'en']);
	});

	it('falls back to all locales when no configured locale is registered', () => {
		expect(getAvailableLocales(['xx'], registered)).toEqual(['en', 'ru', 'de', 'ja']);
	});

	it('treats missing configuration as no restriction', () => {
		expect(getAvailableLocales(undefined, registered)).toEqual(['en', 'ru', 'de', 'ja']);
	});
});

describe('resolveInitialLocale', () => {
	it('prefers the saved locale when it is available', () => {
		expect(resolveInitialLocale(['en', 'ru'], 'ru', 'en-US')).toBe('ru');
	});

	it('ignores a saved locale that is no longer available', () => {
		expect(resolveInitialLocale(['en', 'ru'], 'de', 'ru-RU')).toBe('ru');
	});

	it('matches the navigator locale by language prefix', () => {
		expect(resolveInitialLocale(['en', 'ru'], null, 'ru-RU')).toBe('ru');
	});

	it('uses the first available locale when nothing matches', () => {
		expect(resolveInitialLocale(['ru', 'de'], null, 'fr-FR')).toBe('ru');
	});

	it('uses the first available locale without navigator locale', () => {
		expect(resolveInitialLocale(['de', 'ru'], null, undefined)).toBe('de');
	});
});
