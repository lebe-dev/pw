import adapter from '@sveltejs/adapter-static';
import {vitePreprocess} from '@sveltejs/kit/vite';
import precompileIntl from "svelte-intl-precompile/sveltekit-plugin";

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess([precompileIntl('locales')]),

	kit: {
		adapter: adapter({
			fallback: 'index.html' // may differ from host to host
		}),
	}
};

export default config;
