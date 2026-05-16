use maud::{Markup, PreEscaped, html};
use serde_json::Value;

use crate::i18n::Messages;
use crate::state::AppState;
use crate::view::layout::asset;
use crate::view::schema;

pub fn body(_state: &AppState, locale: &str, m: &Messages) -> Markup {
    html! {
        section class="container mx-auto px-4 pt-32 pb-16 sm:pt-44 sm:pb-24" {
            div class="max-w-4xl mx-auto" {
                div class="flex flex-col sm:flex-row items-center sm:items-start gap-10 sm:gap-14" {
                    div class="shrink-0 order-1" {
                        div class="relative w-28 h-28 sm:w-32 sm:h-32 aspect-square" {
                            img src=(asset(&m.about.avatar.image))
                                alt=(m.about.avatar.alt)
                                width="128" height="128"
                                loading="eager" decoding="async"
                                class="rounded-full ring-soft object-cover w-full h-full";
                            span class="absolute bottom-0 right-1 w-3 h-3 rounded-full z-10"
                                 style="background-color: var(--success); border: 2px solid var(--background);" {}
                        }
                    }
                    div class="text-center sm:text-left flex-1 order-2 min-w-0" {
                        p class="t-caption mb-5" { (m.structured_data.person.job_title) }
                        h1 class="t-display mb-6 break-words" {
                            (m.hero.title_prefix) " "
                            span class="text-aurora" { (m.hero.title_name) "." }
                        }
                        @if !m.hero.description.is_empty() {
                            p class="t-lead mb-10 max-w-xl mx-auto sm:mx-0" { (m.hero.description) }
                        }
                        div class="flex flex-wrap gap-3 justify-center sm:justify-start" {
                            a href=(format!("/{locale}/projects")) class="btn btn-primary" data-magnetic {
                                (m.hero.cta_projects)
                            }
                            a href=(format!("/{locale}/contact")) class="btn btn-ghost" data-magnetic {
                                (m.hero.cta_contact)
                            }
                        }
                    }
                }
            }
        }

        section class="container mx-auto px-4 mb-12 sm:mb-16" {
            div class="max-w-3xl mx-auto" {
                div class="flex flex-col sm:flex-row items-center justify-center gap-4 sm:gap-10 py-4 px-6 border-y border-[var(--border-subtle)] text-sm" {
                    @if let Some(current) = m.experience.items.first() {
                        a href=(m.hero.current_employer_url) target="_blank" rel="noopener noreferrer"
                          class="group inline-flex items-center gap-2 text-[var(--text-secondary)] hover:text-[var(--foreground)] transition-colors" {
                            span class="text-[var(--text-tertiary)] text-xs uppercase tracking-widest" { (m.hero.currently_at) }
                            span class="font-medium text-[var(--foreground)]" { (current.company) }
                            (icon_arrow_up_right())
                        }
                    }
                    span class="hidden sm:block w-px h-5 bg-[var(--border-glass)]" aria-hidden="true" {}
                    a href=(m.social.github.href) target="_blank" rel="noopener noreferrer"
                      class="group inline-flex items-center gap-2 text-[var(--text-secondary)] hover:text-[var(--foreground)] transition-colors" {
                        (icon_github_small())
                        span class="text-[var(--text-tertiary)] text-xs uppercase tracking-widest" { (m.hero.open_source) }
                        span class="font-medium text-[var(--foreground)]" { (m.social.github.label) }
                        (icon_arrow_up_right())
                    }
                }
            }
        }

        div data-fade { (skills(m)) }
        div data-fade { (experience(m)) }
    }
}

pub fn extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    let skills_flat: Vec<String> = m
        .skills
        .categories
        .iter()
        .flat_map(|c| c.skills.iter().cloned())
        .collect();

    vec![
        schema::person(state, m, &skills_flat),
        schema::website(state, m),
        schema::portfolio(state, locale, m),
        schema::web_page(
            state,
            locale,
            "",
            &m.hero.page_title,
            &m.hero.description,
            "WebPage",
            &m.structured_data.person.name,
        ),
        schema::breadcrumb(state, locale, &[(m.navigation.home.as_str(), "")]),
    ]
}

fn skills(m: &Messages) -> Markup {
    html! {
        section class="container mx-auto px-4 section-y" {
            div class="max-w-5xl mx-auto" {
                h2 class="t-h2 heading-accent mb-12" { (m.skills.title) }
                div class="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-10" {
                    @for c in &m.skills.categories {
                        div class="border-t border-[var(--border-subtle)] pt-6" {
                            h3 class="text-[var(--foreground)] font-semibold mb-4 flex items-center gap-3" {
                                span class="inline-flex items-center justify-center w-7 h-7 rounded-md"
                                     style="color: var(--accent-warm);" {
                                    (skill_icon(&c.icon))
                                }
                                (c.name)
                            }
                            div class="flex flex-wrap gap-1.5" {
                                @for s in &c.skills {
                                    span class="px-2.5 py-1 text-[var(--text-secondary)] text-xs rounded-md border border-[var(--border-subtle)] hover:text-[var(--foreground)] hover:border-[var(--border-glass-hover)] transition-colors" {
                                        (s)
                                    }
                                }
                            }
                            p class="text-[11px] text-[var(--text-tertiary)] italic mt-2.5"
                              title=(m.skills.more_skills_tooltip) {
                                (c.more_skills_text) " — " (m.skills.more_skills_tooltip)
                            }
                        }
                    }
                }
            }
        }
    }
}

fn experience(m: &Messages) -> Markup {
    html! {
        section class="container mx-auto px-4 section-y" {
            div class="max-w-3xl mx-auto" {
                h2 class="t-h2 heading-accent mb-12" { (m.experience.title) }
                ol class="relative border-l border-[var(--border-subtle)] pl-8 sm:pl-10 space-y-12" {
                    @for item in &m.experience.items {
                        li class="relative" {
                            span class="absolute -left-[33px] top-2 w-3 h-3 rounded-full"
                                 style="background-color: var(--accent-warm); box-shadow: 0 0 0 4px var(--background);" {}
                            div class="flex flex-col sm:flex-row sm:items-baseline sm:justify-between gap-1 mb-2" {
                                h3 class="text-[var(--foreground)] font-semibold" { (item.title) }
                                span class="t-caption" { (item.period) }
                            }
                            p class="text-sm font-medium mb-3" style="color: var(--accent-warm);" { (item.company) }
                            p class="t-small leading-relaxed" { (item.description) }
                            @if !item.technologies.is_empty() {
                                div class="flex flex-wrap gap-1.5 mt-4" {
                                    @for tech in &item.technologies {
                                        span class="px-2.5 py-1 text-[var(--text-secondary)] text-xs rounded-md border border-[var(--border-subtle)]" { (tech) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn skill_icon(icon: &str) -> Markup {
    let svg = match icon {
        "languages" => {
            r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>"#
        }
        "frameworks" => {
            r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>"#
        }
        "devops" => {
            r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg>"#
        }
        "cloud" => {
            r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M17.5 19a4.5 4.5 0 1 0 0-9h-1.8A7 7 0 1 0 4 14.9"/></svg>"#
        }
        _ => {
            r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><circle cx="12" cy="12" r="9"/></svg>"#
        }
    };
    html! { (PreEscaped(svg)) }
}

fn icon_arrow_up_right() -> Markup {
    html! { (PreEscaped(r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="opacity-60 group-hover:opacity-100 transition-transform group-hover:translate-x-0.5 group-hover:-translate-y-0.5" aria-hidden="true"><line x1="7" y1="17" x2="17" y2="7"/><polyline points="7 7 17 7 17 17"/></svg>"#)) }
}

fn icon_github_small() -> Markup {
    html! { (PreEscaped(r#"<svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 .5C5.65.5.5 5.65.5 12c0 5.08 3.29 9.38 7.86 10.9.58.1.79-.25.79-.55v-2.1c-3.2.7-3.87-1.36-3.87-1.36-.52-1.33-1.28-1.68-1.28-1.68-1.05-.71.08-.7.08-.7 1.16.08 1.78 1.2 1.78 1.2 1.03 1.76 2.7 1.25 3.36.95.1-.75.4-1.25.73-1.54-2.55-.29-5.24-1.27-5.24-5.65 0-1.25.45-2.27 1.18-3.07-.12-.29-.51-1.46.11-3.04 0 0 .96-.31 3.15 1.17.92-.25 1.9-.38 2.88-.39.98.01 1.96.14 2.88.39 2.19-1.48 3.15-1.17 3.15-1.17.62 1.58.23 2.75.11 3.04.73.8 1.18 1.82 1.18 3.07 0 4.39-2.69 5.35-5.25 5.64.41.36.78 1.06.78 2.15v3.19c0 .31.21.66.8.55C20.21 21.38 23.5 17.07 23.5 12 23.5 5.65 18.35.5 12 .5z"/></svg>"#)) }
}
