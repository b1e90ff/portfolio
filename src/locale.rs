use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};

use crate::i18n::Messages;
use crate::state::AppState;

#[derive(Clone)]
pub struct LocaleCtx {
    pub locale: String,
    pub messages: Arc<Messages>,
}

pub struct LocaleNotSupported;

impl IntoResponse for LocaleNotSupported {
    fn into_response(self) -> Response {
        axum::http::StatusCode::NOT_FOUND.into_response()
    }
}

impl FromRequestParts<AppState> for LocaleCtx {
    type Rejection = LocaleNotSupported;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let locale = parts
            .uri
            .path()
            .trim_start_matches('/')
            .split('/')
            .next()
            .unwrap_or("");

        if !state.i18n.has(locale) {
            return Err(LocaleNotSupported);
        }

        Ok(Self {
            locale: locale.to_string(),
            messages: state.i18n.get(locale),
        })
    }
}

pub fn alternate_links(state: &AppState, path: &str) -> Vec<(String, String)> {
    let base = &state.settings.base_url;
    let mut out = Vec::with_capacity(state.i18n.locales().len() + 1);
    for locale in state.i18n.locales() {
        out.push((locale.clone(), format!("{base}/{locale}{path}")));
    }
    out.push((
        "x-default".to_string(),
        format!("{base}/{}{path}", state.i18n.default_locale()),
    ));
    out
}
