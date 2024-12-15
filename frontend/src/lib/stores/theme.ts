import {writable} from 'svelte/store';
import {browser} from '$app/environment';

const storedTheme = browser ? localStorage.getItem('theme') : null;
export const theme = writable(storedTheme || 'day');

if (browser) {
	theme.subscribe(value => {
		localStorage.setItem('theme', value);
		document.documentElement.setAttribute('data-theme', value);
	});
}