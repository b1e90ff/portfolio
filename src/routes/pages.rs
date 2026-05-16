use axum::extract::State;
use maud::{Markup, html};

use crate::locale::LocaleCtx;
use crate::state::AppState;
use crate::view::{Page, layout};

pub async fn home(State(state): State<AppState>, ctx: LocaleCtx) -> Markup {
    let m = ctx.messages.as_ref();
    let body = html! {
        section class="container mx-auto px-4 pt-32 pb-16 sm:pt-44 sm:pb-24" {
            div class="max-w-4xl mx-auto" {
                h1 class="t-display mb-6" {
                    (m.hero.title_prefix) " "
                    span class="text-aurora" { (m.hero.title_name) "." }
                }
                p class="t-lead max-w-xl" { (m.hero.description) }
            }
        }
    };

    let page = Page::new(
        &state,
        &ctx.locale,
        m,
        "",
        m.hero.page_title.clone(),
        m.hero.description.clone(),
        body,
    );
    layout(page)
}
