use maud::{Markup, PreEscaped, html};
use serde_json::Value;

use crate::i18n::Messages;
use crate::state::AppState;
use crate::view::world::{self, Island, PanelHead};
use crate::view::{icons, schema};

pub fn body(locale: &str, m: &Messages) -> Markup {
    let head = PanelHead {
        id: Island::About.key(),
        eyebrow: Island::About.label(m),
        title: &m.about.bio.title,
        ..PanelHead::default()
    };
    let content = html! {
        (world::profile(m, &m.about.bio.content))
        ul class="highlights" {
            @for item in &m.about.bio.highlights {
                li { (PreEscaped(icons::CHECK)) span { (item) } }
            }
        }
        h2 class="section-title" { (m.about.approach.title) }
        p class="lead" { (m.about.approach.content) }
        div class="grid-2" {
            @for principle in &m.about.approach.principles {
                div class="card" {
                    h3 { (principle.title) }
                    p { (principle.description) }
                }
            }
        }
        h2 class="section-title" { (m.about.interests.title) }
        p class="lead" { (m.about.interests.content) }
    };
    world::panel(locale, m, head, content)
}

pub fn extra_schemas(state: &AppState, locale: &str, m: &Messages) -> Vec<Value> {
    vec![
        schema::web_page(
            state,
            locale,
            Island::About.path(),
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
                (m.navigation.about.as_str(), Island::About.path()),
            ],
        ),
    ]
}
