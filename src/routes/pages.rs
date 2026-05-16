use axum::extract::State;
use maud::Markup;

use crate::locale::LocaleCtx;
use crate::state::AppState;
use crate::view::{self, Page, layout};

pub async fn home(State(state): State<AppState>, ctx: LocaleCtx) -> Markup {
    let m = ctx.messages.as_ref();
    let mut page = Page::new(
        &state,
        &ctx.locale,
        m,
        "",
        m.hero.page_title.clone(),
        m.hero.description.clone(),
        view::home::body(&state, &ctx.locale, m),
    );
    page.extra_schemas = view::home::extra_schemas(&state, &ctx.locale, m);
    layout(page)
}

pub async fn about(State(state): State<AppState>, ctx: LocaleCtx) -> Markup {
    let m = ctx.messages.as_ref();
    let mut page = Page::new(
        &state,
        &ctx.locale,
        m,
        "/about",
        m.about.about_title.clone(),
        m.about.bio.content.clone(),
        view::about::body(&state, &ctx.locale, m),
    );
    page.og_type = "profile";
    page.extra_schemas = view::about::extra_schemas(&state, &ctx.locale, m);
    layout(page)
}
