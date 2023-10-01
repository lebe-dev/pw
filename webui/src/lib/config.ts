import {Locale} from "$lib/locale";

export class AppConfig {
    messageMaxLength: number = 1024;
    locale: Locale = new Locale();
}