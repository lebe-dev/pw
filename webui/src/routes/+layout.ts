import {AppConfig} from "$lib/config";
import type {LayoutLoad} from './$types';

export const load: LayoutLoad = async () => {
    return {
        config: new AppConfig()
    };
};