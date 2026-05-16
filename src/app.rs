use std::time::Duration;

use axum::Router;
use axum::routing::get;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::config::Settings;

pub fn router(settings: Settings) -> Router {
    let security_headers = [
        ("x-frame-options", "DENY"),
        ("x-content-type-options", "nosniff"),
        ("referrer-policy", "strict-origin-when-cross-origin"),
        (
            "permissions-policy",
            "camera=(), microphone=(), geolocation=()",
        ),
    ];

    let mut router = Router::new()
        .route("/healthz", get(health))
        .route("/", get(root_redirect));

    for (name, value) in security_headers {
        router = router.layer(SetResponseHeaderLayer::if_not_present(
            name.parse().expect("static header name"),
            value.parse().expect("static header value"),
        ));
    }

    router
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(15)))
        .layer(CatchPanicLayer::new())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .with_state(settings)
}

async fn health() -> &'static str {
    "ok"
}

async fn root_redirect(
    axum::extract::State(settings): axum::extract::State<Settings>,
) -> axum::response::Redirect {
    axum::response::Redirect::permanent(&format!("/{}", settings.default_locale))
}
