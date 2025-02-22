# Localization

Locale files are located inside `frontend/locales` directory.

## Force specified locale

App auto-detects your locale from browser. You can override it in `frontend/src/routes/+layout.ts`, for example:

```typescript
init({ initialLocale: 'ru' ?? undefined, fallbackLocale: 'ru' });
```

## How to add a new locale

Locale files stored in `frontend/locales`. Use the existing as an example. For example add `ge.json` for Georgian language.

Update `frontend/src/routes/+layout.ts` if you'd like to override defaults.
