use maud::{Markup, PreEscaped, html};
use serde_json::Value;

use crate::i18n::Messages;
use crate::state::AppState;
use crate::view::world::{self, Island, PanelHead};
use crate::view::{icons, schema};

pub fn body(locale: &str, m: &Messages) -> Markup {
    let person = &m.structured_data.person;
    let (first, last) = person.name.rsplit_once(' ').unwrap_or((&person.name, ""));
    html! {
        section class="intro" {
            p class="eyebrow" { (person.job_title) " · " (m.hero.location) }
            h1 tabindex="-1" { (first) " " span { (last) } }
            p class="intro-lead" { (m.hero.description) }
            p class="intro-hint" { (m.world.intro) }
            div class="intro-foot" { (world::footer(locale, m)) }
        }
    }
}

pub fn basecamp_body(locale: &str, m: &Messages) -> Markup {
    let head = PanelHead {
        id: Island::Basecamp.key(),
        eyebrow: Island::Basecamp.label(m),
        title: &m.hero.title,
        ..PanelHead::default()
    };
    let content = html! {
        (world::profile(m, &m.hero.description))
        ul class="facts" {
            @if let Some(current) = m.experience.items.first() {
                li {
                    a href=(m.hero.current_employer_url) target="_blank" rel="noopener noreferrer" {
                        span { (m.hero.currently_at) }
                        b { (current.company) }
                        (PreEscaped(icons::EXTERNAL))
                    }
                }
            }
            li {
                a href=(m.social.github.href) target="_blank" rel="noopener noreferrer" {
                    span { (m.hero.open_source) }
                    b { (m.social.github.label) }
                    (PreEscaped(icons::EXTERNAL))
                }
            }
        }
        h2 class="section-title" { (m.basecamp.focus_title) }
        div class="grid-2" {
            @for area in &m.basecamp.focus {
                div class="card" {
                    h3 { (area.title) }
                    p { (area.description) }
                }
            }
        }
        div class="actions" {
            a class="btn" href=(Island::Projects.href(locale)) data-nav { (m.hero.cta_projects) }
            a class="btn btn-ghost" href=(Island::Contact.href(locale)) data-nav { (m.hero.cta_contact) }
        }
    };
    world::panel(locale, m, head, content)
}

pub fn extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    vec![
        schema::person(state, m, &m.skills),
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

pub fn basecamp_extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    vec![
        schema::web_page(
            state,
            locale,
            Island::Basecamp.path(),
            &m.basecamp.page_title,
            &m.basecamp.description,
            "ProfilePage",
            &m.structured_data.person.name,
        ),
        schema::breadcrumb(
            state,
            locale,
            &[
                (m.navigation.home.as_str(), ""),
                (m.basecamp.page_title.as_str(), Island::Basecamp.path()),
            ],
        ),
    ]
}
