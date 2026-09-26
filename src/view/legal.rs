use maud::{Markup, html};
use serde_json::Value;

use crate::i18n::{ImpressumSection, Messages, PrivacySection, PrivacySubsection};
use crate::state::AppState;
use crate::view::schema;
use crate::view::world::{self, PanelHead};

pub fn privacy_body(locale: &str, m: &Messages) -> Markup {
    let head = PanelHead {
        id: "privacy",
        eyebrow: &m.footer.copyright,
        title: &m.privacy.title,
        ..PanelHead::default()
    };
    let content = html! {
        div class="legal" {
            @for section in &m.privacy.sections { (privacy_section(section)) }
        }
    };
    world::panel(locale, m, head, content)
}

pub fn impressum_body(locale: &str, m: &Messages) -> Markup {
    let head = PanelHead {
        id: "impressum",
        eyebrow: &m.footer.copyright,
        title: &m.impressum.title,
        ..PanelHead::default()
    };
    let content = html! {
        div class="legal" {
            @for section in &m.impressum.sections { (impressum_section(section)) }
        }
    };
    world::panel(locale, m, head, content)
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
            section {
                h2 { (title) }
                p class="prewrap" { (content) }
            }
        },
        PrivacySection::TextList {
            title,
            content,
            items,
        } => html! {
            section {
                h2 { (title) }
                p class="prewrap" { (content) }
                ul {
                    @for item in items { li { (item) } }
                }
            }
        },
        PrivacySection::Subsection { title, subsections } => html! {
            section {
                h2 { (title) }
                div class="legal-sub" {
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
                h3 { (title) }
                p class="prewrap" { (content) }
            }
        },
        PrivacySubsection::TextList {
            title,
            content,
            items,
        } => html! {
            div {
                h3 { (title) }
                p class="prewrap" { (content) }
                ul {
                    @for item in items { li { (item) } }
                }
            }
        },
    }
}

fn impressum_section(section: &ImpressumSection) -> Markup {
    match section {
        ImpressumSection::Contact { title, lines } => html! {
            section {
                h2 { (title) }
                address {
                    @for line in lines {
                        @if line.is_empty() { p { "\u{00A0}" } }
                        @else { p { (line) } }
                    }
                }
            }
        },
        ImpressumSection::Text { title, content } => html! {
            section {
                h2 { (title) }
                p class="prewrap" { (content) }
            }
        },
    }
}
