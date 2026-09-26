use axum::extract::{Path, State};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use maud::Markup;

use crate::locale::LocaleCtx;
use crate::state::AppState;
use crate::view::world::Island;
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
        view::home::body(&ctx.locale, m),
    );
    page.island = Some(Island::Overview);
    page.extra_schemas = view::home::extra_schemas(&state, &ctx.locale, m);
    layout(page)
}

pub async fn basecamp(State(state): State<AppState>, ctx: LocaleCtx) -> Markup {
    let m = ctx.messages.as_ref();
    let mut page = Page::new(
        &state,
        &ctx.locale,
        m,
        Island::Basecamp.path(),
        m.basecamp.page_title.clone(),
        m.basecamp.description.clone(),
        view::home::basecamp_body(&ctx.locale, m),
    );
    page.island = Some(Island::Basecamp);
    page.og_type = "profile";
    page.extra_schemas = view::home::basecamp_extra_schemas(&state, &ctx.locale, m);
    layout(page)
}

pub async fn experience(State(state): State<AppState>, ctx: LocaleCtx) -> Markup {
    let m = ctx.messages.as_ref();
    let mut page = Page::new(
        &state,
        &ctx.locale,
        m,
        Island::Experience.path(),
        m.experience.title.clone(),
        m.experience.description.clone(),
        view::experience::body(&ctx.locale, m),
    );
    page.island = Some(Island::Experience);
    page.extra_schemas = view::experience::extra_schemas(&state, &ctx.locale, m);
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
        view::about::body(&ctx.locale, m),
    );
    page.island = Some(Island::About);
    page.og_type = "profile";
    page.extra_schemas = view::about::extra_schemas(&state, &ctx.locale, m);
    layout(page)
}

pub async fn contact(State(state): State<AppState>, ctx: LocaleCtx) -> Markup {
    let m = ctx.messages.as_ref();
    let mut page = Page::new(
        &state,
        &ctx.locale,
        m,
        "/contact",
        m.contact.title.clone(),
        m.contact.description.clone(),
        view::contact::body(&ctx.locale, m),
    );
    page.island = Some(Island::Contact);
    page.extra_schemas = view::contact::extra_schemas(&state, &ctx.locale, m);
    layout(page)
}

pub async fn projects_list(State(state): State<AppState>, ctx: LocaleCtx) -> Markup {
    let m = ctx.messages.as_ref();
    let mut page = Page::new(
        &state,
        &ctx.locale,
        m,
        "/projects",
        m.projects.title.clone(),
        m.projects.description.clone(),
        view::projects::list_body(&ctx.locale, m),
    );
    page.island = Some(Island::Projects);
    page.extra_schemas = view::projects::list_extra_schemas(&state, &ctx.locale, m);
    layout(page)
}

pub async fn project_detail(
    State(state): State<AppState>,
    ctx: LocaleCtx,
    Path(id): Path<String>,
) -> Response {
    let m = ctx.messages.as_ref();
    let path = format!("/projects/{id}");
    let Some(project) = view::projects::find(m, &id) else {
        let body = view::not_found::body(&ctx.locale, m);
        let page = Page::new(
            &state,
            &ctx.locale,
            m,
            &path,
            m.notfound.title.clone(),
            m.notfound.message.clone(),
            body,
        );
        return (StatusCode::NOT_FOUND, layout(page)).into_response();
    };

    let mut page = Page::new(
        &state,
        &ctx.locale,
        m,
        &path,
        project.title.clone(),
        project.description.clone(),
        view::projects::detail_body(&ctx.locale, m, project),
    );
    page.island = Some(Island::Projects);
    page.og_type = "article";
    page.og_image = Some(if project.image.starts_with("http") {
        project.image.clone()
    } else {
        format!("{}{}", state.settings.base_url, project.image)
    });
    page.extra_schemas = view::projects::detail_extra_schemas(&state, &ctx.locale, m, project);
    layout(page).into_response()
}

pub async fn privacy(State(state): State<AppState>, ctx: LocaleCtx) -> Markup {
    let m = ctx.messages.as_ref();
    let mut page = Page::new(
        &state,
        &ctx.locale,
        m,
        "/privacy",
        m.privacy.title.clone(),
        m.footer.privacy.clone(),
        view::legal::privacy_body(&ctx.locale, m),
    );
    page.extra_schemas = view::legal::privacy_extra_schemas(&state, &ctx.locale, m);
    layout(page)
}

pub async fn impressum(State(state): State<AppState>, ctx: LocaleCtx) -> Markup {
    let m = ctx.messages.as_ref();
    let mut page = Page::new(
        &state,
        &ctx.locale,
        m,
        "/impressum",
        m.impressum.title.clone(),
        m.footer.impressum.clone(),
        view::legal::impressum_body(&ctx.locale, m),
    );
    page.extra_schemas = view::legal::impressum_extra_schemas(&state, &ctx.locale, m);
    layout(page)
}

pub async fn fallback_not_found(State(state): State<AppState>, uri: Uri) -> Response {
    let requested = uri.path().split('/').nth(1).unwrap_or_default();
    let locale = if state.i18n.has(requested) {
        requested.to_string()
    } else {
        state.settings.default_locale.clone()
    };
    let messages = state.i18n.get(&locale);
    let m = messages.as_ref();
    let body = view::not_found::body(&locale, m);
    let page = Page::new(
        &state,
        &locale,
        m,
        "/404",
        m.notfound.title.clone(),
        m.notfound.message.clone(),
        body,
    );
    (StatusCode::NOT_FOUND, layout(page)).into_response()
}
