# Localization

Locale files are located inside `webui/locales` directory.

## Force specified locale

App auto-detects your locale from browser. You can override it in `webui/src/routes/+layout.ts`, for example:

```typescript
init({ initialLocale: 'ru' ?? undefined, fallbackLocale: 'ru' });
```

## How to add a new locale

Locale files stored in `webui/locales`. Use the existing as an example. For example add `de.json` for German language.

Update `webui/src/routes/+layout.ts` if you'd like to override defaults.