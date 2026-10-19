export class AppConfig {
	messageMaxLength: number = 1024;
	fileUploadEnabled: boolean = false;
	fileMaxSize: number = 0;
	/** Locales allowed by the backend. Empty means all locales are available. */
	availableLocales: string[] = [];
}

/** Loads the app config from the backend, returns null when it is unavailable. */
export async function fetchAppConfig(fetchFn: typeof fetch = fetch): Promise<AppConfig | null> {
	try {
		const response = await fetchFn('/api/config', { method: 'GET' });
		if (response.status !== 200) {
			return null;
		}
		return Object.assign(new AppConfig(), await response.json());
	} catch (e) {
		console.error('unable to load config', e);
		return null;
	}
}
