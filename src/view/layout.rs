use maud::{DOCTYPE, Markup, PreEscaped, html};
use serde_json::Value;

use crate::i18n::{I18n, Messages};
use crate::keywords;
use crate::locale::alternate_links;
use crate::state::AppState;
use crate::view::{nav, schema};

pub struct Page<'a> {
    pub state: &'a AppState,
    pub locale: &'a str,
    pub messages: &'a Messages,
    pub path: &'a str,
    pub title: String,
    pub description: String,
    pub og_type: &'a str,
    pub og_image: Option<String>,
    pub extra_schemas: Vec<Value>,
    pub body: Markup,
}

impl<'a> Page<'a> {
    pub fn new(
        state: &'a AppState,
        locale: &'a str,
        messages: &'a Messages,
        path: &'a str,
        title: impl Into<String>,
        description: impl Into<String>,
        body: Markup,
    ) -> Self {
        Self {
            state,
            locale,
            messages,
            path,
            title: title.into(),
            description: description.into(),
            og_type: "website",
            og_image: None,
            extra_schemas: Vec::new(),
            body,
        }
    }
}

pub fn layout(p: Page<'_>) -> Markup {
    let base_url = &p.state.settings.base_url;
    let canonical = format!("{base_url}/{}{}", p.locale, p.path);
    let og_locale = I18n::og_locale(p.locale);
    let alt_locales: Vec<String> = p
        .state
        .i18n
        .locales()
        .iter()
        .filter(|l| l.as_str() != p.locale)
        .map(|l| I18n::og_locale(l))
        .collect();
    let alternates = alternate_links(p.state, p.path);
    let templated_title = templated(&p.messages.metadata.title_template, &p.title);
    let keywords_str = keywords::comma_separated(p.locale);
    let og_image_url = p
        .og_image
        .clone()
        .unwrap_or_else(|| format!("{base_url}/tat.webp"));

    let mut schemas = vec![
        schema::organization(p.state, p.locale, p.messages),
        schema::site_navigation(p.state, p.locale, p.messages),
    ];
    schemas.extend(p.extra_schemas);

    html! {
        (DOCTYPE)
        html lang=(p.locale) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover";
                title { (templated_title) }
                meta name="description" content=(p.description);
                meta name="keywords" content=(keywords_str);
                meta name="format-detection" content="telephone=no, address=no, email=no";
                meta name="robots" content="index, follow, max-image-preview:large";
                meta name="theme-color" content="#060608" media="(prefers-color-scheme: dark)";
                meta name="theme-color" content="#fafafa" media="(prefers-color-scheme: light)";
                meta name="color-scheme" content="dark light";

                link rel="canonical" href=(canonical);
                @for (lang, href) in &alternates {
                    link rel="alternate" hreflang=(lang) href=(href);
                }

                meta property="og:type" content=(p.og_type);
                meta property="og:url" content=(canonical);
                meta property="og:site_name" content=(p.messages.structured_data.person.name);
                meta property="og:title" content=(templated_title);
                meta property="og:description" content=(p.description);
                meta property="og:locale" content=(og_locale);
                meta property="og:image" content=(og_image_url);
                meta property="og:image:width" content="1200";
                meta property="og:image:height" content="630";
                @for alt in &alt_locales {
                    meta property="og:locale:alternate" content=(alt);
                }
                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:title" content=(templated_title);
                meta name="twitter:description" content=(p.description);
                meta name="twitter:image" content=(og_image_url);

                link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png";
                link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png";
                link rel="icon" href="/favicon.ico";
                link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
                link rel="manifest" href="/site.webmanifest";

                link rel="stylesheet" href="/css/main.css";

                script {
                    (PreEscaped(THEME_INIT))
                }
                @for s in &schemas {
                    (schema::json_ld(s))
                }
            }
            body class="min-h-screen" {
                a href="#main-content" class="skip-link" { (p.messages.navigation.skip_to_content) }
                div class="aurora-bg" aria-hidden="true" {
                    div class="aurora-blob aurora-warm" {}
                    div class="aurora-blob aurora-cool" {}
                    div class="aurora-grid" {}
                    div class="aurora-vignette" {}
                }
                (nav::header(p.state, p.locale, p.path, p.messages))
                main #main-content class="relative z-10 pt-14" {
                    (p.body)
                }
                (nav::footer(p.state, p.locale, p.messages))
                script src="/js/app.js" defer {}
            }
        }
    }
}

fn templated(template: &str, title: &str) -> String {
    if title.is_empty() {
        template.replace("%s | ", "").replace(" | %s", "")
    } else {
        template.replace("%s", title)
    }
}

const THEME_INIT: &str = r#"(function(){try{var s=localStorage.getItem('theme');var p=window.matchMedia('(prefers-color-scheme: dark)').matches?'dark':'light';var t=(s==='dark'||s==='light')?s:p;document.documentElement.setAttribute('data-theme',t);}catch(e){document.documentElement.setAttribute('data-theme','dark');}})();"#;
