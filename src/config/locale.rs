use std::collections::HashSet;

/// Locales the frontend ships translations for.
pub const SUPPORTED_LOCALES: [&str; 8] = ["en", "ru", "de", "es", "fr", "ge", "ch", "ja"];

/// Parses a comma-separated locale list, e.g. `PW_AVAILABLE_LOCALES=en,ru`.
pub fn parse_locales_list(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}

/// Normalizes the configured locales and rejects unsupported ones.
/// Order is preserved, duplicates are dropped. An empty list means "all supported locales".
pub fn normalize_available_locales(locales: &[String]) -> anyhow::Result<Vec<String>> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for locale in locales {
        let locale = locale.trim().to_lowercase();

        if locale.is_empty() {
            continue;
        }

        if !SUPPORTED_LOCALES.contains(&locale.as_str()) {
            return Err(anyhow::anyhow!(
                "Unsupported locale '{}' in available-locales, supported locales: {}",
                locale,
                SUPPORTED_LOCALES.join(", ")
            ));
        }

        if seen.insert(locale.clone()) {
            result.push(locale);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    #[test]
    fn parse_locales_list_splits_and_trims() {
        assert_eq!(
            parse_locales_list("en, ru ,de"),
            strings(&["en", "ru", "de"])
        );
    }

    #[test]
    fn parse_locales_list_skips_empty_items() {
        assert_eq!(parse_locales_list(" , en,,"), strings(&["en"]));
        assert!(parse_locales_list("").is_empty());
    }

    #[test]
    fn normalize_keeps_order_and_lowercases() {
        let result = normalize_available_locales(&strings(&["RU", " en "])).unwrap();
        assert_eq!(result, strings(&["ru", "en"]));
    }

    #[test]
    fn normalize_drops_duplicates() {
        let result = normalize_available_locales(&strings(&["en", "ru", "EN"])).unwrap();
        assert_eq!(result, strings(&["en", "ru"]));
    }

    #[test]
    fn normalize_empty_list_stays_empty() {
        assert!(normalize_available_locales(&[]).unwrap().is_empty());
    }

    #[test]
    fn normalize_rejects_unsupported_locale() {
        let error = normalize_available_locales(&strings(&["en", "xx"])).unwrap_err();
        assert!(error.to_string().contains("Unsupported locale 'xx'"));
    }
}
