import {sveltekit} from '@sveltejs/kit/vite';
import {defineConfig} from 'vite';
import precompileIntl from 'svelte-intl-precompile/sveltekit-plugin'

export default defineConfig({
	plugins: [sveltekit(), precompileIntl('locales')],
	optimizeDeps:{
		// exclude: ['crypto-js']
	},
	build: {
		rollupOptions: {

		}
	}
});
