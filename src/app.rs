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

use axum::routing::post;

use crate::routes::{api, pages};
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let pages = Router::new()
        .route("/healthz", get(health))
        .route("/", get(root_redirect))
        .route("/api/health", get(api::health))
        .route("/api/contact", post(api::contact))
        .route("/{locale}", get(pages::home))
        .route("/{locale}/about", get(pages::about))
        .route("/{locale}/projects", get(pages::projects_list))
        .route("/{locale}/projects/{id}", get(pages::project_detail))
        .route("/{locale}/contact", get(pages::contact))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Settings;
    use crate::i18n::I18n;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn test_app() -> Router {
        let settings = Settings {
            bind: "0.0.0.0:0".into(),
            base_url: "https://example.test".into(),
            default_locale: "en-US".into(),
            locales: vec!["en-US".into(), "de-DE".into()],
            smtp: None,
        };
        let i18n = I18n::load(&settings.locales, &settings.default_locale).unwrap();
        let state = AppState::new(settings, i18n);
        router(state)
    }

    async fn get(app: Router, path: &str) -> (StatusCode, String) {
        let res = app
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = res.status();
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap_or_default();
        (status, body)
    }

    #[tokio::test]
    async fn healthz_returns_ok() {
        let (status, body) = get(test_app(), "/healthz").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "ok");
    }

    #[tokio::test]
    async fn root_redirects_to_default_locale() {
        let res = test_app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(res.headers()["location"], "/en-US");
    }

    #[tokio::test]
    async fn en_us_home_renders() {
        let (status, body) = get(test_app(), "/en-US").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("<!DOCTYPE html>"));
        assert!(body.contains(r#"lang="en-US""#));
        assert!(body.contains("Niklas"));
        assert!(body.contains(r#"<link rel="canonical" href="https://example.test/en-US">"#));
    }

    #[tokio::test]
    async fn de_de_home_uses_german_copy() {
        let (status, body) = get(test_app(), "/de-DE").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains(r#"lang="de-DE""#));
        assert!(body.contains("Architektur, die hält"));
    }

    #[tokio::test]
    async fn about_page_renders() {
        let (status, body) = get(test_app(), "/en-US/about").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("About Me"));
        assert!(body.contains(r#"<link rel="canonical" href="https://example.test/en-US/about">"#));
    }

    #[tokio::test]
    async fn unknown_locale_falls_back() {
        let res = test_app()
            .oneshot(Request::builder().uri("/xx-XX").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::PERMANENT_REDIRECT);
    }

    #[tokio::test]
    async fn security_headers_present() {
        let res = test_app()
            .oneshot(Request::builder().uri("/healthz").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let h = res.headers();
        assert_eq!(h.get("x-frame-options").unwrap(), "DENY");
        assert_eq!(h.get("x-content-type-options").unwrap(), "nosniff");
        assert!(h.contains_key("referrer-policy"));
        assert!(h.contains_key("permissions-policy"));
    }

    #[tokio::test]
    async fn hreflang_alternates_present_on_home() {
        let (_, body) = get(test_app(), "/en-US").await;
        assert!(body.contains(r#"hreflang="en-US""#));
        assert!(body.contains(r#"hreflang="de-DE""#));
        assert!(body.contains(r#"hreflang="x-default""#));
    }
}
