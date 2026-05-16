use maud::{Markup, PreEscaped, html};
use serde_json::{Value, json};

use crate::i18n::Messages;
use crate::state::AppState;

pub fn json_ld(value: &Value) -> Markup {
    let mut json = serde_json::to_string(value).unwrap_or_else(|_| "{}".into());
    json = json.replace("</", "<\\/");
    html! { script type="application/ld+json" { (PreEscaped(json)) } }
}

pub fn organization(state: &AppState, locale: &str) -> Value {
    let base = &state.settings.base_url;
    json!({
        "@context": "https://schema.org",
        "@type": "ProfessionalService",
        "@id": format!("{base}#organization"),
        "name": "Niklas Tat",
        "url": format!("{base}/{locale}"),
        "image": format!("{base}/tat.webp"),
        "logo": format!("{base}/tat.webp"),
        "sameAs": [
            "https://github.com/b1e90ff",
            "https://linkedin.com/in/niklas-tat-5219a024b",
        ],
        "founder": { "@type": "Person", "name": "Niklas Tat" },
        "areaServed": { "@type": "Country", "name": "Switzerland" },
        "knowsAbout": [
            "Backend Development",
            "DevOps",
            "IT Management",
            "Kubernetes",
            "Cloud Infrastructure",
            "Security & Compliance",
        ],
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
    let image = if p.image.starts_with("http") {
        p.image.clone()
    } else {
        format!("{base}{}", p.image)
    };
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

pub fn portfolio(state: &AppState, m: &Messages) -> Value {
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
        "url": format!("{base}/projects"),
    })
}
