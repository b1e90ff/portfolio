use std::collections::BTreeSet;

use maud::{Markup, PreEscaped, html};
use serde_json::{Value, json};

use crate::i18n::{Messages, ProjectItem};
use crate::state::AppState;
use crate::view::layout::asset;
use crate::view::schema;

pub fn list_body(_state: &AppState, locale: &str, m: &Messages) -> Markup {
    let statuses: BTreeSet<(&str, &str)> = m
        .projects
        .items
        .iter()
        .map(|p| (p.status.as_str(), p.status_label.as_str()))
        .collect();
    let technologies: BTreeSet<&str> = m
        .projects
        .items
        .iter()
        .flat_map(|p| p.technologies.iter().map(|s| s.as_str()))
        .collect();

    html! {
        section class="container mx-auto px-4 pt-32 sm:pt-40 pb-16" {
            div class="max-w-3xl mx-auto" {
                header class="mb-10" {
                    p class="t-caption mb-3" { (m.projects.title) }
                    h1 class="t-h1 mb-4" {
                        span class="text-aurora" { (m.projects.title) }
                    }
                    p class="t-lead" { (m.projects.description) }
                }

                div class="flex flex-col sm:flex-row flex-wrap gap-3 mb-6"
                    data-projects-filters {
                    input type="search"
                          name="q"
                          placeholder=(m.projects.search_placeholder)
                          autocomplete="off"
                          class="w-full sm:flex-1 px-4 py-2.5 rounded-full text-sm border border-[var(--border-subtle)] focus:outline-none focus:border-[var(--accent-warm)] transition-colors"
                          style="background-color: var(--surface-1); color: var(--foreground);"
                          data-projects-search;
                    select name="status"
                           class="w-full sm:w-auto px-4 py-2.5 rounded-full text-sm border border-[var(--border-subtle)] focus:outline-none focus:border-[var(--accent-warm)] transition-colors"
                           style="background-color: var(--surface-1); color: var(--foreground);"
                           data-projects-status {
                        option value="" { (m.projects.status_all) }
                        @for (value, label) in &statuses {
                            option value=(value) { (label) }
                        }
                    }
                    select name="technology"
                           class="w-full sm:w-auto px-4 py-2.5 rounded-full text-sm border border-[var(--border-subtle)] focus:outline-none focus:border-[var(--accent-warm)] transition-colors"
                           style="background-color: var(--surface-1); color: var(--foreground);"
                           data-projects-technology {
                        option value="" { (m.projects.technology_all) }
                        @for tech in &technologies {
                            option value=(tech) { (tech) }
                        }
                    }
                    button type="button"
                           class="hidden sm:inline-flex items-center gap-2 px-4 py-2.5 rounded-full text-sm text-[var(--text-secondary)] hover:text-[var(--foreground)] transition-colors"
                           data-projects-reset { (m.projects.reset_filters) }
                }

                div class="space-y-4" data-projects-grid {
                    @for project in &m.projects.items {
                        (project_card(locale, m, project))
                    }
                }

                p class="t-lead text-center mt-12 hidden" data-projects-empty {
                    (m.projects.no_projects) " " (m.projects.no_projects_hint)
                }
            }
        }
    }
}

pub fn detail_body(_state: &AppState, locale: &str, m: &Messages, project: &ProjectItem) -> Markup {
    html! {
        section class="container mx-auto px-4 pt-32 sm:pt-40 pb-16" {
            div class="max-w-3xl mx-auto" {
                nav class="flex flex-wrap items-center gap-1.5 t-caption mb-6" aria-label="Breadcrumb" {
                    a href=(format!("/{locale}")) class="hover:text-[var(--foreground)] transition-colors" { (m.navigation.home) }
                    span aria-hidden="true" { "/" }
                    a href=(format!("/{locale}/projects")) class="hover:text-[var(--foreground)] transition-colors" { (m.projects.title) }
                    span aria-hidden="true" { "/" }
                    span class="text-[var(--foreground)]" { (project.title) }
                }

                a href=(format!("/{locale}/projects"))
                  class="inline-flex items-center gap-2 text-sm text-[var(--text-secondary)] hover:text-[var(--foreground)] transition-colors mb-8" {
                    (icon_arrow_left())
                    span { (m.projects.back_to_projects) }
                }

                h1 class="t-h1 mb-4" {
                    span class="text-aurora" { (project.title) }
                }
                p class="t-lead mb-10" { (project.description) }

                figure class="aspect-video relative mb-12 rounded-xl overflow-hidden border border-[var(--border-subtle)]" {
                    img src=(asset(&project.image))
                        alt=(project.title)
                        width="1280" height="720"
                        loading="eager" decoding="async"
                        class="w-full h-full object-cover";
                }

                section class="border-t border-[var(--border-subtle)] pt-8 mb-12" data-fade {
                    p class="t-body whitespace-pre-line" { (project.extended_description) }
                }

                @if !project.technologies.is_empty() {
                    section class="border-t border-[var(--border-subtle)] pt-8 mb-12" data-fade {
                        h2 class="t-caption mb-4" { (m.projects.technologies_title) }
                        div class="flex flex-wrap gap-1.5" {
                            @for tech in &project.technologies {
                                span class="px-2.5 py-1 text-xs rounded-md text-[var(--text-secondary)] border border-[var(--border-subtle)]" { (tech) }
                            }
                        }
                    }
                }

                div class="flex flex-wrap gap-3 border-t border-[var(--border-subtle)] pt-8" {
                    @if let Some(url) = &project.live_url {
                        a href=(url) target="_blank" rel="noopener noreferrer" class="btn btn-primary" {
                            (icon_external()) span { (m.projects.view_live) }
                        }
                    }
                    @if let Some(url) = &project.code_url {
                        a href=(url) target="_blank" rel="noopener noreferrer" class="btn btn-ghost" {
                            (icon_external()) span { (m.projects.view_code) }
                        }
                    }
                    @if let Some(url) = &project.docs_url {
                        a href=(url) target="_blank" rel="noopener noreferrer" class="btn btn-ghost" {
                            (icon_external()) span { (m.projects.view_docs) }
                        }
                    }
                }
            }
        }
    }
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
            "/projects",
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
                (m.projects.title.as_str(), "/projects"),
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
                (m.projects.title.as_str(), "/projects"),
                (project.title.as_str(), &format!("/projects/{}", project.id)),
            ],
        ),
        creative_work,
    ]
}

fn project_card(locale: &str, m: &Messages, p: &ProjectItem) -> Markup {
    let tech_data = p.technologies.join(",");
    html! {
        article class="card card-interactive lift-on-hover overflow-hidden"
                data-project-card
                data-status=(p.status)
                data-technologies=(tech_data)
                data-title=(p.title.to_lowercase())
                data-description=(p.description.to_lowercase()) {
            div class="flex flex-col sm:flex-row gap-0 relative z-[2]" {
                figure class="shrink-0 overflow-hidden sm:w-56 aspect-video sm:aspect-auto sm:self-stretch border-b sm:border-b-0 sm:border-r border-[var(--border-subtle)]" {
                    img src=(asset(&p.image))
                        alt=(p.title)
                        width="448" height="252"
                        loading="lazy" decoding="async"
                        class="w-full h-full object-cover";
                }
                div class="flex-1 p-5 min-w-0" {
                    div class="flex items-baseline justify-between gap-3 mb-2" {
                        a href=(format!("/{locale}/projects/{}", p.id))
                          class="text-[var(--foreground)] font-semibold hover:text-[var(--accent-warm)] transition-colors" {
                            (p.title)
                        }
                        span class="t-caption shrink-0" { (p.status_label) }
                    }
                    p class="t-small leading-relaxed mb-4" { (p.description) }
                    @if !p.technologies.is_empty() {
                        div class="flex flex-wrap gap-1.5 mb-4" {
                            @for tech in &p.technologies {
                                span class="px-2 py-0.5 text-[10px] rounded-md text-[var(--text-secondary)] border border-[var(--border-subtle)]" { (tech) }
                            }
                        }
                    }
                    div class="flex flex-wrap gap-4 text-sm" {
                        a href=(format!("/{locale}/projects/{}", p.id))
                          class="inline-flex items-center gap-1.5 text-[var(--text-secondary)] hover:text-[var(--accent-warm)] transition-colors" {
                            (m.projects.view_details) (icon_arrow_right())
                        }
                        @if let Some(url) = &p.live_url {
                            a href=(url) target="_blank" rel="noopener noreferrer"
                              class="inline-flex items-center gap-1.5 text-[var(--text-secondary)] hover:text-[var(--accent-warm)] transition-colors" {
                                (m.projects.view_live) (icon_external())
                            }
                        }
                        @if let Some(url) = &p.code_url {
                            a href=(url) target="_blank" rel="noopener noreferrer"
                              class="inline-flex items-center gap-1.5 text-[var(--text-secondary)] hover:text-[var(--accent-warm)] transition-colors" {
                                (m.projects.view_code) (icon_external())
                            }
                        }
                        @if let Some(url) = &p.docs_url {
                            a href=(url) target="_blank" rel="noopener noreferrer"
                              class="inline-flex items-center gap-1.5 text-[var(--text-secondary)] hover:text-[var(--accent-warm)] transition-colors" {
                                (m.projects.view_docs) (icon_external())
                            }
                        }
                    }
                }
            }
        }
    }
}

fn icon_arrow_right() -> Markup {
    html! { (PreEscaped(r#"<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>"#)) }
}

fn icon_arrow_left() -> Markup {
    html! { (PreEscaped(r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="19" y1="12" x2="5" y2="12"/><polyline points="12 19 5 12 12 5"/></svg>"#)) }
}

fn icon_external() -> Markup {
    html! { (PreEscaped(r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/><polyline points="15 3 21 3 21 9"/><line x1="10" y1="14" x2="21" y2="3"/></svg>"#)) }
}

pub fn find<'a>(m: &'a Messages, id: &str) -> Option<&'a ProjectItem> {
    m.projects.items.iter().find(|p| p.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Settings;
    use crate::i18n::I18n;

    fn fixture_state() -> AppState {
        let settings = Settings {
            bind: "0.0.0.0:0".into(),
            base_url: "https://example.test".into(),
            default_locale: "en-US".into(),
            locales: vec!["en-US".into(), "de-DE".into()],
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
