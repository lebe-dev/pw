import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import precompileIntl from 'svelte-intl-precompile/sveltekit-plugin';

export default defineConfig({
	plugins: [sveltekit(), precompileIntl('locales')],

	server: {
		allowedHosts: ['test.home']
	},

	test: {
		include: ['src/**/*.{test,spec}.{js,ts}'],
		environment: 'happy-dom',

		coverage: {
			provider: 'v8',
			// lcov feeds SonarQube (see sonar.javascript.lcov.reportPaths); text is for humans.
			reporter: ['text', 'lcov'],
			reportsDirectory: 'coverage',
			// Without an explicit include the report also covers build output and config
			// files. Keep it to our own sources and mirror sonar.exclusions so both tools
			// agree on what counts: vendored shadcn components are not ours to test.
			include: ['src/**/*.{js,ts,svelte}'],
			exclude: ['src/lib/components/ui/**', 'src/**/*.{test,spec}.{js,ts}', 'src/app.d.ts']
		}
	}
});
