use maud::{Markup, PreEscaped, html};
use serde_json::{Value, json};

use crate::i18n::{Messages, ProjectItem};
use crate::state::AppState;
use crate::view::layout::asset;
use crate::view::world::{self, Island, PanelHead};
use crate::view::{icons, schema};

pub fn list_body(locale: &str, m: &Messages) -> Markup {
    panel(locale, m, false)
}

pub fn detail_body(locale: &str, m: &Messages, project: &ProjectItem) -> Markup {
    html! {
        (panel(locale, m, true))
        (modal(locale, m, project))
    }
}

fn panel(locale: &str, m: &Messages, behind_modal: bool) -> Markup {
    let mut ordered: Vec<&ProjectItem> = m.projects.items.iter().collect();
    ordered.sort_by(|a, b| b.date.cmp(&a.date));
    let head = PanelHead {
        id: Island::Projects.key(),
        eyebrow: Island::Projects.label(m),
        title: &m.projects.heading,
        behind_modal,
    };
    let content = html! {
        p class="lead" { (m.projects.description) }
        ul class="projects" {
            @for project in ordered {
                li { (project_card(locale, m, project)) }
            }
        }
    };
    world::panel(locale, m, head, content)
}

pub fn list_extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    let base = &state.settings.base_url;
    let list_url = format!("{base}/{locale}/projects");

    let item_list = json!({
        "@context": "https://schema.org",
        "@type": "ItemList",
        "@id": format!("{list_url}#projects"),
        "name": m.projects.title,
        "itemListElement": m.projects.items.iter().enumerate().map(|(i, p)| {
            json!({
                "@type": "ListItem",
                "position": i + 1,
                "url": format!("{base}/{locale}/projects/{}", p.id),
                "name": p.title,
            })
        }).collect::<Vec<_>>(),
    });

    vec![
        schema::web_page(
            state,
            locale,
            Island::Projects.path(),
            &m.projects.title,
            &m.projects.description,
            "CollectionPage",
            &m.structured_data.person.name,
        ),
        schema::breadcrumb(
            state,
            locale,
            &[
                (m.navigation.home.as_str(), ""),
                (m.projects.title.as_str(), Island::Projects.path()),
            ],
        ),
        item_list,
    ]
}

pub fn detail_extra_schemas(
    state: &AppState,
    locale: &str,
    m: &Messages,
    project: &ProjectItem,
) -> Vec<Value> {
    let base = &state.settings.base_url;
    let url = format!("{base}/{locale}/projects/{}", project.id);
    let image = if project.image.starts_with("http") {
        project.image.clone()
    } else {
        format!("{base}{}", project.image)
    };

    let creative_work = json!({
        "@context": "https://schema.org",
        "@type": "CreativeWork",
        "@id": format!("{url}#work"),
        "url": url,
        "name": project.title,
        "description": project.description,
        "image": image,
        "dateCreated": project.date,
        "creator": { "@type": "Person", "name": m.structured_data.person.name },
        "keywords": project.technologies,
    });

    vec![
        schema::web_page(
            state,
            locale,
            &format!("/projects/{}", project.id),
            &project.title,
            &project.description,
            "WebPage",
            &m.structured_data.person.name,
        ),
        schema::breadcrumb(
            state,
            locale,
            &[
                (m.navigation.home.as_str(), ""),
                (m.projects.title.as_str(), Island::Projects.path()),
                (project.title.as_str(), &format!("/projects/{}", project.id)),
            ],
        ),
        creative_work,
    ]
}

fn project_card(locale: &str, m: &Messages, p: &ProjectItem) -> Markup {
    html! {
        a class="card project" href=(format!("/{locale}/projects/{}", p.id)) data-nav
          aria-label=(format!("{} – {}", p.title, m.projects.view_details)) {
            img src=(asset(&p.image)) alt="" width="224" height="152" loading="lazy" decoding="async";
            div {
                h2 { (p.title) }
                p { (p.description) }
                (tags(p, 2))
            }
        }
    }
}

fn tags(p: &ProjectItem, tech_limit: usize) -> Markup {
    html! {
        div class="tags" {
            span class="tag tag-accent" { (p.date.get(..4).unwrap_or(&p.date)) }
            span class="tag" { (p.status_label) }
            @for tech in p.technologies.iter().take(tech_limit) {
                span class="tag" { (tech) }
            }
        }
    }
}

fn modal(locale: &str, m: &Messages, project: &ProjectItem) -> Markup {
    let links = [
        (&project.live_url, &m.projects.view_live),
        (&project.code_url, &m.projects.view_code),
        (&project.docs_url, &m.projects.view_docs),
    ];
    html! {
        div class="modal" data-modal {
            a class="modal-scrim" href=(Island::Projects.href(locale)) data-nav data-modal-close
              tabindex="-1" aria-label=(m.world.close) {}
            article class="modal-card glass" role="dialog" aria-modal="true" aria-labelledby="modal-title" {
                a class="icon-btn glass" href=(Island::Projects.href(locale)) data-nav data-modal-close
                  aria-label=(m.world.close) {
                    (PreEscaped(icons::CLOSE))
                }
                div class="modal-scroll" {
                    img src=(asset(&project.image)) alt="" width="1280" height="640" decoding="async";
                    div class="modal-body" {
                        (tags(project, 0))
                        h1 #modal-title tabindex="-1" { (project.title) }
                        p class="lead" { (project.extended_description) }
                        @if !project.technologies.is_empty() {
                            h2 class="section-title" { (m.projects.technologies_title) }
                            div class="tags" {
                                @for tech in &project.technologies { span class="tag" { (tech) } }
                            }
                        }
                        div class="actions" {
                            @for (url, label) in links {
                                @if let Some(url) = url {
                                    a class="btn" href=(url) target="_blank" rel="noopener noreferrer" {
                                        (label) (PreEscaped(icons::EXTERNAL))
                                    }
                                }
                            }
                            a class="btn btn-ghost" href=(Island::Projects.href(locale)) data-nav data-modal-close {
                                (PreEscaped(icons::ARROW_LEFT)) (m.projects.back_to_projects)
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn find<'a>(m: &'a Messages, id: &str) -> Option<&'a ProjectItem> {
    m.projects.items.iter().find(|p| p.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Settings;
    use crate::i18n::I18n;
    use axum_client_ip::ClientIpSource;

    fn fixture_state() -> AppState {
        let settings = Settings {
            bind: "0.0.0.0:0".into(),
            base_url: "https://example.test".into(),
            default_locale: "en-US".into(),
            locales: vec!["en-US".into(), "de-DE".into()],
            client_ip_source: ClientIpSource::ConnectInfo,
            smtp: None,
        };
        let i18n = I18n::load(&settings.locales, &settings.default_locale).unwrap();
        AppState::new(settings, i18n)
    }

    #[test]
    fn find_returns_known_project() {
        let state = fixture_state();
        let m = state.i18n.get("en-US");
        assert!(find(&m, "eventfrog").is_some());
        assert!(find(&m, "does-not-exist").is_none());
    }

    #[test]
    fn list_schema_emits_itemlist() {
        let state = fixture_state();
        let m = state.i18n.get("en-US");
        let schemas = list_extra_schemas(&state, "en-US", &m);
        let list = schemas
            .iter()
            .find(|v| v["@type"] == "ItemList")
            .expect("ItemList present");
        assert!(!list["itemListElement"].as_array().unwrap().is_empty());
    }

    #[test]
    fn detail_schema_has_creativework_with_image() {
        let state = fixture_state();
        let m = state.i18n.get("en-US");
        let p = find(&m, "helm-repository").unwrap();
        let schemas = detail_extra_schemas(&state, "en-US", &m, p);
        let work = schemas
            .iter()
            .find(|v| v["@type"] == "CreativeWork")
            .expect("CreativeWork present");
        assert!(work["image"].as_str().unwrap().starts_with("https://"));
    }
}
