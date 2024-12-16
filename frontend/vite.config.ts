import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import precompileIntl from 'svelte-intl-precompile/sveltekit-plugin';

export default defineConfig({
	plugins: [sveltekit(), precompileIntl('locales')],

	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
