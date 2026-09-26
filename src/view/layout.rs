use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::LazyLock;

use maud::{DOCTYPE, Markup, PreEscaped, html};
use serde_json::Value;

use crate::i18n::{I18n, Messages};
use crate::keywords;
use crate::locale::alternate_links;
use crate::state::AppState;
use crate::view::world::{self, Island};
use crate::view::{nav, schema};

pub const THREE_VENDOR: &str = "/vendor/three-0.186.1";

pub fn asset(url: &str) -> String {
    if url.starts_with("http") || url.contains('?') {
        url.to_string()
    } else {
        format!("{url}?v={}", env!("CARGO_PKG_VERSION"))
    }
}

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
    pub island: Option<Island>,
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
            island: None,
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
        .unwrap_or_else(|| format!("{base_url}/{}/opengraph-image", p.locale));
    let import_map = format!(
        r#"{{"imports":{{"three":"{THREE_VENDOR}/three.module.js","three/addons/":"{THREE_VENDOR}/addons/"}}}}"#
    );

    let island = p.island.unwrap_or(Island::Overview);

    let mut schemas = vec![
        schema::organization(p.state, p.locale, p.messages),
        schema::site_navigation(p.state, p.locale, p.messages),
    ];
    schemas.extend(p.extra_schemas);

    html! {
        (DOCTYPE)
        html lang=(p.locale) data-mood=(nav::DEFAULT_MOOD) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover";
                title { (templated_title) }
                meta name="description" content=(p.description);
                meta name="keywords" content=(keywords_str);
                meta name="format-detection" content="telephone=no, address=no, email=no";
                meta name="robots" content="index, follow, max-image-preview:large";
                meta name="theme-color" content="#1f1840";
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

                link rel="preload" href="/fonts/bricolage-grotesque-latin-wght-normal.woff2" as="font" type="font/woff2" crossorigin;
                link rel="stylesheet" href=(stylesheet_href());

                script { (PreEscaped(mood_init())) }
                script type="importmap" { (PreEscaped(import_map)) }
                script type="module" src=(asset("/js/app.js")) {}
                @for s in &schemas {
                    (schema::json_ld(s))
                }
            }
            body data-island=(island.key()) {
                a href="#main-content" class="skip-link" { (p.messages.navigation.skip_to_content) }
                div class="backdrop" aria-hidden="true" {}
                canvas #world class="world" role="img"
                       aria-label=(p.messages.world.scene_label)
                       data-signposts=(world::signposts(p.messages)) {}
                (nav::topbar(p.state, p.locale, p.path, p.messages))
                (nav::pins(p.locale, p.messages))
                main #main-content data-island=(island.key()) {
                    (p.body)
                }
                (nav::dock(p.locale, p.messages, p.island))
                p class="loading" data-loading hidden { (p.messages.world.loading) }
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

// The stylesheet is cached immutably, so its URL must change whenever its content does.
pub fn stylesheet_href() -> &'static str {
    static HREF: LazyLock<String> = LazyLock::new(|| match fs::read(STYLESHEET) {
        Ok(bytes) => format!("/css/main.css?v={}", content_version(&bytes)),
        Err(err) => {
            tracing::warn!(%err, path = STYLESHEET, "stylesheet unreadable, falling back to version query");
            asset("/css/main.css")
        }
    });
    &HREF
}

fn content_version(bytes: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

const STYLESHEET: &str = "public/css/main.css";

fn mood_init() -> &'static str {
    static SCRIPT: LazyLock<String> = LazyLock::new(|| {
        let moods = nav::MOODS.join("|");
        format!(
            r#"(function(){{var d=document.documentElement;d.classList.add('js');try{{var m=localStorage.getItem('mood');if(/^({moods})$/.test(m))d.dataset.mood=m;}}catch(e){{}}}})();"#
        )
    });
    &SCRIPT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_version_changes_with_content() {
        let a = content_version(b"body { color: red }");
        assert_eq!(a.len(), 16);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(a, content_version(b"body { color: red }"));
        assert_ne!(a, content_version(b"body { color: blue }"));
    }
}
