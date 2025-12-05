# Localization

The application uses `svelte-intl-precompile` for internationalization with automatic locale discovery.

## Supported Locales

Currently supported languages:

- `en` - English
- `de` - Deutsch (German)
- `es` - Español (Spanish)
- `ru` - Русский (Russian)
- `ge` - ქართული (Georgian)
- `fr` - Français (French)
- `ja` - 日本語 (Japanese)

Locale files are located in `frontend/locales/` directory.

## How Localization Works

1. **Auto-Discovery**: The Vite plugin (`precompileIntl('locales')` in `vite.config.ts`) automatically discovers all JSON files in the `frontend/locales/` directory at build time
2. **Locale Selection**: Users can select their preferred language via the locale selector in the header
3. **Persistence**: Selected locale is saved to `localStorage` (key: `pw-preferred-locale`)
4. **Browser Detection**: If no locale is saved, the app auto-detects locale from browser settings

## Force Specified Locale

App auto-detects your locale from browser. You can override it in `frontend/src/routes/+layout.ts`, for example:

```typescript
init({ initialLocale: 'ru' ?? undefined, fallbackLocale: 'ru' });
```

## How to Add a New Locale

To add a new locale (e.g., French `fr`):

### 1. Create Locale File

Create `frontend/locales/fr.json` following the structure of existing files:

```json
{
  "id": "fr",
  "headerLabels": { ... },
  "languages": {
    "en": "Anglais",
    "de": "Allemand",
    "es": "Espagnol",
    "ru": "Russe",
    "ge": "Géorgien",
    "fr": "Français"
  },
  ...
}
```

### 2. Update LocaleSelector Component

Add the locale abbreviation to `frontend/src/lib/components/LocaleSelector.svelte`:

```typescript
const localeLabels: Record<string, string> = {
  en: 'EN',
  de: 'DE',
  es: 'ES',
  ru: 'RU',
  ge: 'GE',
  fr: 'FR'  // ADD NEW LOCALE
};
```

### 3. Update All Existing Locale Files

Add the new locale to the `languages` object in **all** existing locale files:

- `frontend/locales/en.json` - Add `"fr": "French"`
- `frontend/locales/de.json` - Add `"fr": "Französisch"`
- `frontend/locales/es.json` - Add `"fr": "Francés"`
- `frontend/locales/ru.json` - Add `"fr": "Французский"`
- `frontend/locales/ge.json` - Add `"fr": "ფრანგული"`

**Why**: This ensures language names display correctly when any locale is active.

### 4. Create FAQ Documentation

Create `docs/faq/FAQ.fr.md` following the structure of existing FAQ files.

### 5. Update FAQ Index

Add link in `docs/FAQ.md`:

```markdown
- [Français](faq/FAQ.fr.md)
```

### 6. Restart Dev Server

After adding the new locale file, restart the development server to ensure auto-discovery picks up the new locale:

```bash
npm run dev
```

## File Structure

```
frontend/
├── locales/
│   ├── en.json
│   ├── de.json
│   ├── es.json
│   ├── ru.json
│   └── ge.json
├── src/
│   ├── lib/components/
│   │   └── LocaleSelector.svelte
│   └── routes/
│       └── +layout.ts
└── vite.config.ts

docs/
├── faq/
│   ├── FAQ.en.md
│   ├── FAQ.de.md
│   ├── FAQ.es.md
│   ├── FAQ.ru.md
│   └── FAQ.ge.md
└── FAQ.md
```
