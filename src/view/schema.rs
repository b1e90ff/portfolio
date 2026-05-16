use maud::{Markup, PreEscaped, html};
use serde_json::{Value, json};

use crate::i18n::Messages;
use crate::state::AppState;

pub fn json_ld(value: &Value) -> Markup {
    let mut json = serde_json::to_string(value).unwrap_or_else(|_| "{}".into());
    json = json.replace("</", "<\\/");
    html! { script type="application/ld+json" { (PreEscaped(json)) } }
}

pub fn organization(state: &AppState, locale: &str, m: &Messages) -> Value {
    let base = &state.settings.base_url;
    let image = absolute(base, &m.structured_data.person.image);
    let same_as: Vec<&str> = [
        m.social.github.href.as_str(),
        m.social.linkedin.href.as_str(),
    ]
    .iter()
    .copied()
    .filter(|s| s.starts_with("http"))
    .collect();

    json!({
        "@context": "https://schema.org",
        "@type": "ProfessionalService",
        "@id": format!("{base}#organization"),
        "name": m.structured_data.person.name,
        "url": format!("{base}/{locale}"),
        "image": image,
        "logo": image,
        "sameAs": same_as,
        "founder": { "@type": "Person", "name": m.structured_data.person.name },
        "areaServed": { "@type": "Country", "name": area_served_name(locale) },
        "knowsAbout": m.structured_data.portfolio.about,
    })
}

pub fn site_navigation(state: &AppState, locale: &str, m: &Messages) -> Value {
    let base = &state.settings.base_url;
    let items: [(&str, &str); 4] = [
        (&m.navigation.home, ""),
        (&m.navigation.projects, "/projects"),
        (&m.navigation.about, "/about"),
        (&m.navigation.contact, "/contact"),
    ];

    json!({
        "@context": "https://schema.org",
        "@type": "SiteNavigationElement",
        "name": items.iter().map(|(n, _)| n.to_string()).collect::<Vec<_>>(),
        "url":  items.iter().map(|(_, p)| format!("{base}/{locale}{p}")).collect::<Vec<_>>(),
    })
}

pub fn breadcrumb(state: &AppState, locale: &str, items: &[(&str, &str)]) -> Value {
    let base = &state.settings.base_url;
    let list: Vec<Value> = items
        .iter()
        .enumerate()
        .map(|(i, (name, path))| {
            json!({
                "@type": "ListItem",
                "position": i + 1,
                "name": name,
                "item": format!("{base}/{locale}{path}"),
            })
        })
        .collect();

    json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": list,
    })
}

pub fn web_page(
    state: &AppState,
    locale: &str,
    path: &str,
    name: &str,
    description: &str,
    page_type: &str,
    author_name: &str,
) -> Value {
    let base = &state.settings.base_url;
    let url = format!("{base}/{locale}{path}");
    json!({
        "@context": "https://schema.org",
        "@type": page_type,
        "@id": format!("{url}#webpage"),
        "url": url,
        "name": name,
        "description": description,
        "inLanguage": locale,
        "isPartOf": { "@type": "WebSite", "@id": format!("{base}#website") },
        "about": { "@type": "Person", "name": author_name },
    })
}

pub fn person(state: &AppState, m: &Messages, skills_flat: &[String]) -> Value {
    let base = &state.settings.base_url;
    let p = &m.structured_data.person;
    let image = absolute(base, &p.image);
    json!({
        "@context": "https://schema.org",
        "@type": "Person",
        "name": p.name,
        "jobTitle": p.job_title,
        "description": p.description,
        "image": image,
        "url": base,
        "email": p.email,
        "sameAs": p.same_as,
        "knowsAbout": skills_flat,
    })
}

pub fn website(state: &AppState, m: &Messages) -> Value {
    let base = &state.settings.base_url;
    let w = &m.structured_data.website;
    json!({
        "@context": "https://schema.org",
        "@type": "WebSite",
        "@id": format!("{base}#website"),
        "name": w.name,
        "description": w.description,
        "url": base,
    })
}

pub fn portfolio(state: &AppState, locale: &str, m: &Messages) -> Value {
    let base = &state.settings.base_url;
    let p = &m.structured_data.portfolio;
    json!({
        "@context": "https://schema.org",
        "@type": "CreativeWork",
        "name": p.name,
        "description": p.description,
        "dateCreated": p.date_created,
        "genre": p.genre,
        "about": p.about,
        "author": { "@type": "Person", "name": m.structured_data.person.name },
        "url": format!("{base}/{locale}/projects"),
    })
}

fn absolute(base: &str, url: &str) -> String {
    if url.starts_with("http") {
        url.to_string()
    } else {
        format!("{base}{url}")
    }
}

fn area_served_name(locale: &str) -> &'static str {
    match locale {
        "de-DE" => "Schweiz",
        _ => "Switzerland",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Settings;
    use crate::i18n::I18n;

    fn fixture_state() -> AppState {
        let settings = Settings {
            bind: "0.0.0.0:3000".into(),
            base_url: "https://example.test".into(),
            default_locale: "en-US".into(),
            locales: vec!["en-US".into(), "de-DE".into()],
            smtp: None,
        };
        let i18n = I18n::load(&settings.locales, &settings.default_locale).unwrap();
        AppState::new(settings, i18n)
    }

    #[test]
    fn organization_contains_name_and_same_as() {
        let state = fixture_state();
        let m = state.i18n.get("en-US");
        let v = organization(&state, "en-US", &m);
        assert_eq!(v["@type"], "ProfessionalService");
        assert!(!v["sameAs"].as_array().unwrap().is_empty());
        assert_eq!(v["url"], "https://example.test/en-US");
    }

    #[test]
    fn breadcrumb_uses_one_based_positions() {
        let state = fixture_state();
        let v = breadcrumb(&state, "en-US", &[("Home", ""), ("Projects", "/projects")]);
        let list = v["itemListElement"].as_array().unwrap();
        assert_eq!(list[0]["position"], 1);
        assert_eq!(list[1]["position"], 2);
        assert_eq!(list[1]["item"], "https://example.test/en-US/projects");
    }

    #[test]
    fn web_page_carries_inlanguage() {
        let state = fixture_state();
        let v = web_page(
            &state,
            "de-DE",
            "/about",
            "About",
            "About description",
            "AboutPage",
            "Niklas Tat",
        );
        assert_eq!(v["inLanguage"], "de-DE");
        assert_eq!(v["@type"], "AboutPage");
    }
}
