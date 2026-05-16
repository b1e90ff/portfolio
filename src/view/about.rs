use maud::{Markup, PreEscaped, html};
use serde_json::Value;

use crate::i18n::Messages;
use crate::state::AppState;
use crate::view::schema;

pub fn body(_state: &AppState, _locale: &str, m: &Messages) -> Markup {
    html! {
        section class="container mx-auto px-4 pt-32 pb-16 sm:pt-40" {
            div class="max-w-3xl mx-auto" {
                div class="flex flex-col sm:flex-row items-center sm:items-start gap-8 mb-16" {
                    div class="shrink-0" {
                        div class="relative w-24 h-24 aspect-square" {
                            img src=(m.about.avatar.image)
                                alt=(m.about.avatar.alt)
                                width="96" height="96"
                                loading="eager" decoding="async"
                                class="rounded-full ring-soft object-cover w-full h-full";
                        }
                    }
                    div class="text-center sm:text-left flex-1 min-w-0" {
                        p class="t-caption mb-3" { (m.structured_data.person.job_title) }
                        h1 class="t-h1 mb-4" {
                            span class="text-aurora" { (m.about.about_title) }
                        }
                    }
                }

                section class="mb-16 border-t border-[var(--border-subtle)] pt-8" data-fade {
                    h2 class="t-caption mb-5" { (m.about.bio.title) }
                    p class="t-lead mb-8 whitespace-pre-line" { (m.about.bio.content) }
                    ul class="space-y-3" {
                        @for item in &m.about.bio.highlights {
                            li class="flex items-start gap-3 t-small" {
                                span class="shrink-0 mt-1" style="color: var(--accent-warm);" { (icon_check()) }
                                span { (item) }
                            }
                        }
                    }
                }

                section class="mb-16 border-t border-[var(--border-subtle)] pt-8" data-fade {
                    h2 class="t-caption mb-5" { (m.about.approach.title) }
                    p class="t-lead mb-8 whitespace-pre-line" { (m.about.approach.content) }
                    div class="grid grid-cols-1 sm:grid-cols-2 gap-x-10 gap-y-8" {
                        @for principle in &m.about.approach.principles {
                            div {
                                h3 class="text-sm font-semibold mb-2"
                                   style="color: var(--accent-warm);" { (principle.title) }
                                p class="t-small leading-relaxed" { (principle.description) }
                            }
                        }
                    }
                }

                section class="border-t border-[var(--border-subtle)] pt-8" data-fade {
                    h2 class="t-caption mb-5" { (m.about.interests.title) }
                    p class="t-lead whitespace-pre-line" { (m.about.interests.content) }
                }
            }
        }
    }
}

pub fn extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    vec![
        schema::web_page(
            state,
            locale,
            "/about",
            &m.about.about_title,
            &m.about.bio.content,
            "AboutPage",
            &m.structured_data.person.name,
        ),
        schema::breadcrumb(
            state,
            locale,
            &[
                (m.navigation.home.as_str(), ""),
                (m.navigation.about.as_str(), "/about"),
            ],
        ),
    ]
}

fn icon_check() -> Markup {
    html! {
        (PreEscaped(r#"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><polyline points="20 6 9 17 4 12"/></svg>"#))
    }
}
