use maud::{Markup, html};
use serde_json::Value;

use crate::i18n::{ImpressumSection, Messages, PrivacySection, PrivacySubsection};
use crate::state::AppState;
use crate::view::schema;

pub fn privacy_body(_state: &AppState, _locale: &str, m: &Messages) -> Markup {
    html! {
        section class="container mx-auto px-4 pt-32 sm:pt-40 pb-16" {
            div class="max-w-3xl mx-auto" {
                header class="mb-12" {
                    h1 class="t-h1 mb-4" { span class="text-aurora" { (m.privacy.title) } }
                }
                div class="space-y-10" {
                    @for section in &m.privacy.sections {
                        (privacy_section(section))
                    }
                }
            }
        }
    }
}

pub fn impressum_body(_state: &AppState, _locale: &str, m: &Messages) -> Markup {
    html! {
        section class="container mx-auto px-4 pt-32 sm:pt-40 pb-16" {
            div class="max-w-3xl mx-auto" {
                header class="mb-12" {
                    h1 class="t-h1 mb-4" { span class="text-aurora" { (m.impressum.title) } }
                }
                div class="space-y-10" {
                    @for section in &m.impressum.sections {
                        (impressum_section(section))
                    }
                }
            }
        }
    }
}

pub fn privacy_extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    vec![
        schema::web_page(
            state,
            locale,
            "/privacy",
            &m.privacy.title,
            &m.footer.privacy,
            "WebPage",
            &m.structured_data.person.name,
        ),
        schema::breadcrumb(
            state,
            locale,
            &[
                (m.navigation.home.as_str(), ""),
                (m.privacy.title.as_str(), "/privacy"),
            ],
        ),
    ]
}

pub fn impressum_extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    vec![
        schema::web_page(
            state,
            locale,
            "/impressum",
            &m.impressum.title,
            &m.footer.impressum,
            "WebPage",
            &m.structured_data.person.name,
        ),
        schema::breadcrumb(
            state,
            locale,
            &[
                (m.navigation.home.as_str(), ""),
                (m.impressum.title.as_str(), "/impressum"),
            ],
        ),
    ]
}

fn privacy_section(section: &PrivacySection) -> Markup {
    match section {
        PrivacySection::Text { title, content } => html! {
            section data-fade {
                h2 class="text-sm font-semibold tracking-wide uppercase mb-3"
                   style="color: var(--accent-warm);" { (title) }
                p class="t-body whitespace-pre-line" { (content) }
            }
        },
        PrivacySection::TextList {
            title,
            content,
            items,
        } => html! {
            section data-fade {
                h2 class="text-sm font-semibold tracking-wide uppercase mb-3"
                   style="color: var(--accent-warm);" { (title) }
                p class="t-body whitespace-pre-line mb-4" { (content) }
                ul class="t-body list-disc pl-5 space-y-1" {
                    @for item in items { li { (item) } }
                }
            }
        },
        PrivacySection::Subsection { title, subsections } => html! {
            section data-fade {
                h2 class="text-sm font-semibold tracking-wide uppercase mb-3"
                   style="color: var(--accent-warm);" { (title) }
                div class="space-y-6 mt-2" {
                    @for sub in subsections { (privacy_subsection(sub)) }
                }
            }
        },
    }
}

fn privacy_subsection(sub: &PrivacySubsection) -> Markup {
    match sub {
        PrivacySubsection::Text { title, content } => html! {
            div {
                h3 class="text-[var(--foreground)] font-semibold mb-2" { (title) }
                p class="t-body whitespace-pre-line" { (content) }
            }
        },
        PrivacySubsection::TextList {
            title,
            content,
            items,
        } => html! {
            div {
                h3 class="text-[var(--foreground)] font-semibold mb-2" { (title) }
                p class="t-body whitespace-pre-line mb-3" { (content) }
                ul class="t-body list-disc pl-5 space-y-1" {
                    @for item in items { li { (item) } }
                }
            }
        },
    }
}

fn impressum_section(section: &ImpressumSection) -> Markup {
    match section {
        ImpressumSection::Contact { title, lines } => html! {
            section data-fade {
                h2 class="text-sm font-semibold tracking-wide uppercase mb-3"
                   style="color: var(--accent-warm);" { (title) }
                div class="t-body space-y-0.5" {
                    @for line in lines {
                        @if line.is_empty() { p { "\u{00A0}" } }
                        @else { p { (line) } }
                    }
                }
            }
        },
        ImpressumSection::Text { title, content } => html! {
            section data-fade {
                h2 class="text-sm font-semibold tracking-wide uppercase mb-3"
                   style="color: var(--accent-warm);" { (title) }
                p class="t-body whitespace-pre-line" { (content) }
            }
        },
    }
}
