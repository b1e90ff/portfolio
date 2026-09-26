use maud::{Markup, html};

use crate::i18n::Messages;
use crate::view::world::{self, Island, PanelHead};

pub fn body(locale: &str, m: &Messages) -> Markup {
    let head = PanelHead {
        id: "not-found",
        eyebrow: &m.notfound.eyebrow,
        title: &m.notfound.title,
        ..PanelHead::default()
    };
    let content = html! {
        p class="lead" { (m.notfound.message) }
        div class="actions" {
            a class="btn" href=(Island::Overview.href(locale)) data-nav { (m.notfound.back) }
        }
    };
    world::panel(locale, m, head, content)
}
