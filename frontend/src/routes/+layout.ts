import {AppConfig} from "$lib/config";
import type {LayoutLoad} from './$types';
import {getLocaleFromNavigator, init} from 'svelte-intl-precompile'
// @ts-ignore
import {registerAll} from '$locales';

registerAll()

init({ initialLocale: getLocaleFromNavigator() ?? undefined, fallbackLocale: 'en' });

export const ssr = false;

export const load: LayoutLoad = async () => {
    return {
        config: new AppConfig()
    };
};