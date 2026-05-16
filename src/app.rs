use std::time::Duration;

use axum::Router;
use axum::routing::get;
use http::HeaderValue;
use http::header::CACHE_CONTROL;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::routes::pages;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let pages = Router::new()
        .route("/healthz", get(health))
        .route("/", get(root_redirect))
        .route("/{locale}", get(pages::home))
        .with_state(state);

    let images = Router::new()
        .fallback_service(serve_dir("public/images"))
        .layer(immutable_cache());

    let css = Router::new()
        .fallback_service(serve_dir("public/css"))
        .layer(immutable_cache());

    let public_assets = Router::new()
        .fallback_service(serve_dir("public"))
        .layer(short_cache());

    pages
        .nest("/images", images)
        .nest("/css", css)
        .fallback_service(public_assets)
        .layer(SetResponseHeaderLayer::if_not_present(
            http::header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            http::header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            http::header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            http::HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
        ))
        .layer(CompressionLayer::new().gzip(true).br(true))
        .layer(TimeoutLayer::with_status_code(
            http::StatusCode::GATEWAY_TIMEOUT,
            Duration::from_secs(15),
        ))
        .layer(CatchPanicLayer::new())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
}

fn serve_dir(path: &str) -> ServeDir {
    ServeDir::new(path)
        .precompressed_gzip()
        .precompressed_br()
        .append_index_html_on_directories(false)
}

fn immutable_cache() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000, immutable"),
    )
}

fn short_cache() -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=3600, must-revalidate"),
    )
}

async fn health() -> &'static str {
    "ok"
}

async fn root_redirect(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> axum::response::Redirect {
    axum::response::Redirect::permanent(&format!("/{}", state.settings.default_locale))
}
