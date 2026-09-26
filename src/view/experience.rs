use maud::{Markup, html};
use serde_json::Value;

use crate::i18n::Messages;
use crate::state::AppState;
use crate::view::schema;
use crate::view::world::{self, Island, PanelHead};

pub fn body(locale: &str, m: &Messages) -> Markup {
    let head = PanelHead {
        id: Island::Experience.key(),
        eyebrow: Island::Experience.label(m),
        title: &m.experience.heading,
        ..PanelHead::default()
    };
    let content = html! {
        h2 class="section-title" { (m.skills.title) }
        div class="stack" {
            @for category in &m.skills.categories {
                div class="stack-group" {
                    h3 { (category.name) }
                    div class="tags" {
                        @for skill in &category.skills { span class="tag" { (skill) } }
                    }
                }
            }
            p class="stack-more" { (m.skills.more) }
        }
        h2 class="section-title" { (m.experience.roles_title) }
        ol class="timeline" {
            @for item in &m.experience.items {
                li class="card" {
                    span class="tag tag-accent" { (item.period) }
                    h3 { (item.company) }
                    p class="role" { (item.title) }
                    p { (item.description) }
                }
            }
        }
    };
    world::panel(locale, m, head, content)
}

pub fn extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    vec![
        schema::web_page(
            state,
            locale,
            Island::Experience.path(),
            &m.experience.title,
            &m.experience.description,
            "WebPage",
            &m.structured_data.person.name,
        ),
        schema::breadcrumb(
            state,
            locale,
            &[
                (m.navigation.home.as_str(), ""),
                (m.experience.title.as_str(), Island::Experience.path()),
            ],
        ),
    ]
}
