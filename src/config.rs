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
