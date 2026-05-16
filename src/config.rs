use std::env;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Settings {
    pub bind: String,
    pub base_url: String,
    pub default_locale: String,
    pub locales: Vec<String>,
}

impl Settings {
    pub fn from_env() -> Result<Self> {
        let bind = env::var("PORTFOLIO_BIND").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

        let base_url = env::var("PORTFOLIO_BASE_URL")
            .or_else(|_| env::var("NEXT_PUBLIC_BASE_URL"))
            .or_else(|_| env::var("BASE_URL"))
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .trim_end_matches('/')
            .to_string();

        let default_locale =
            env::var("PORTFOLIO_DEFAULT_LOCALE").unwrap_or_else(|_| "en-US".to_string());

        let locales = env::var("PORTFOLIO_LOCALES")
            .unwrap_or_else(|_| "en-US,de-DE".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        anyhow::ensure!(!locales.is_empty(), "PORTFOLIO_LOCALES must list at least one");
        anyhow::ensure!(
            locales.contains(&default_locale),
            "default locale {default_locale} not in PORTFOLIO_LOCALES"
        );

        Ok(Self {
            bind,
            base_url,
            default_locale,
            locales,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_clean_env<F: FnOnce()>(f: F) {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
        // SAFETY: tests are serialised via ENV_LOCK so the unsoundness of
        // env::remove_var across threads cannot trigger here.
        unsafe {
            for k in [
                "PORTFOLIO_BIND",
                "PORTFOLIO_BASE_URL",
                "NEXT_PUBLIC_BASE_URL",
                "BASE_URL",
                "PORTFOLIO_DEFAULT_LOCALE",
                "PORTFOLIO_LOCALES",
            ] {
                std::env::remove_var(k);
            }
        }
        f();
    }

    #[test]
    fn defaults_are_sensible() {
        with_clean_env(|| {
            let s = Settings::from_env().expect("should build defaults");
            assert_eq!(s.bind, "0.0.0.0:3000");
            assert_eq!(s.base_url, "http://localhost:3000");
            assert_eq!(s.default_locale, "en-US");
            assert!(s.locales.iter().any(|l| l == "en-US"));
        });
    }

    #[test]
    fn base_url_trailing_slash_is_trimmed() {
        with_clean_env(|| {
            // SAFETY: serialised via ENV_LOCK.
            unsafe {
                std::env::set_var("PORTFOLIO_BASE_URL", "https://example.com/");
            }
            let s = Settings::from_env().unwrap();
            assert_eq!(s.base_url, "https://example.com");
        });
    }

    #[test]
    fn default_locale_must_be_in_locales() {
        with_clean_env(|| {
            // SAFETY: serialised via ENV_LOCK.
            unsafe {
                std::env::set_var("PORTFOLIO_DEFAULT_LOCALE", "fr-FR");
                std::env::set_var("PORTFOLIO_LOCALES", "en-US,de-DE");
            }
            assert!(Settings::from_env().is_err());
        });
    }

    #[test]
    fn locales_list_is_split_on_commas() {
        with_clean_env(|| {
            // SAFETY: serialised via ENV_LOCK.
            unsafe {
                std::env::set_var("PORTFOLIO_LOCALES", "en-US, de-DE , fr-FR");
                std::env::set_var("PORTFOLIO_DEFAULT_LOCALE", "en-US");
            }
            let s = Settings::from_env().unwrap();
            assert_eq!(s.locales, vec!["en-US", "de-DE", "fr-FR"]);
        });
    }
}
