import {AppConfig} from "$lib/config";
import type {LayoutLoad} from './$types';

export const ssr = false;

export const load: LayoutLoad = async () => {
    const localeFiles = import.meta.glob('../lib/locale/*.json');
    return {
        config: new AppConfig(),
        localeFiles: localeFiles
    };
};