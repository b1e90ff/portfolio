use std::time::Duration;

use axum::Router;
use axum::routing::{get, post};
use http::HeaderValue;
use http::header::CACHE_CONTROL;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::compression::CompressionLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::services::ServeDir;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::routes::{api, pages, seo};
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    let client_ip_source = state.settings.client_ip_source.clone();
    let mut pages_router = Router::new()
        .route("/healthz", get(health))
        .route("/", get(root_redirect))
        .route("/api/health", get(api::health))
        .route("/api/contact", post(api::contact))
        .route("/sitemap.xml", get(seo::sitemap))
        .route("/robots.txt", get(seo::robots))
        .route("/site.webmanifest", get(seo::manifest));

    // Register locale-prefixed routes per configured locale rather than via a
    // single `/{locale}` wildcard. The wildcard variant collides with static
    // prefixes (`/images`, `/css`, `/favicon.ico`) because matchit treats a
    // three-segment dynamic match as more specific than a two-segment nest.
    for locale in state.i18n.locales() {
        let prefix = format!("/{locale}");
        pages_router = pages_router
            .route(&prefix, get(pages::home))
            .route(&format!("{prefix}/about"), get(pages::about))
            .route(&format!("{prefix}/projects"), get(pages::projects_list))
            .route(
                &format!("{prefix}/projects/{{id}}"),
                get(pages::project_detail),
            )
            .route(&format!("{prefix}/contact"), get(pages::contact))
            .route(
                &format!("{prefix}/opengraph-image"),
                get(seo::opengraph_image),
            )
            .route(&format!("{prefix}/privacy"), get(pages::privacy))
            .route(&format!("{prefix}/impressum"), get(pages::impressum));
    }

    let pages_router = pages_router.with_state(state.clone());

    let images = Router::new()
        .fallback_service(serve_dir("public/images"))
        .layer(immutable_cache());

    let css = Router::new()
        .fallback_service(serve_dir("public/css"))
        .layer(immutable_cache());

    let fonts = Router::new()
        .fallback_service(serve_dir("assets/fonts"))
        .layer(immutable_cache());

    let not_found_router = Router::new()
        .fallback(pages::fallback_not_found)
        .with_state(state.clone());
    let public_serve = ServeDir::new("public")
        .precompressed_gzip()
        .precompressed_br()
        .append_index_html_on_directories(false)
        .not_found_service(not_found_router);
    let public_assets = Router::new()
        .fallback_service(public_serve)
        .layer(short_cache());

    pages_router
        .nest("/images", images)
        .nest("/css", css)
        .nest("/fonts", fonts)
        .fallback_service(public_assets)
        .layer(client_ip_source.into_extension())
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
    use std::net::SocketAddr;

    use crate::config::Settings;
    use crate::i18n::I18n;
    use crate::rate_limit::RateLimitConfig;
    use axum::body::Body;
    use axum::extract::ConnectInfo;
    use axum::http::request::Builder;
    use axum::http::{Request, StatusCode};
    use axum_client_ip::ClientIpSource;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    const CONTACT_BODY: &str = r#"{"name":"Alice","email":"a@example.com","subject":"Hello","message":"This is at least ten characters long."}"#;

    fn test_app() -> Router {
        test_app_with(ClientIpSource::ConnectInfo)
    }

    fn test_app_with(client_ip_source: ClientIpSource) -> Router {
        let settings = Settings {
            bind: "0.0.0.0:0".into(),
            base_url: "https://example.test".into(),
            default_locale: "en-US".into(),
            locales: vec!["en-US".into(), "de-DE".into()],
            client_ip_source,
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

    fn contact_request(headers: &[(&str, &str)]) -> Builder {
        headers.iter().fold(
            Request::builder()
                .method("POST")
                .uri("/api/contact")
                .header("content-type", "application/json"),
            |req, (name, value)| req.header(*name, *value),
        )
    }

    async fn send_contact(app: Router, req: Builder) -> StatusCode {
        app.oneshot(req.body(Body::from(CONTACT_BODY)).unwrap())
            .await
            .unwrap()
            .status()
    }

    async fn post_contact(app: Router, headers: &[(&str, &str)]) -> StatusCode {
        send_contact(app, contact_request(headers)).await
    }

    #[tokio::test]
    async fn contact_rate_limit_ignores_spoofed_forwarded_for() {
        let app = test_app_with(ClientIpSource::CfConnectingIp);
        for i in 0..RateLimitConfig::CONTACT_DEFAULT.max {
            let spoofed = format!("198.51.100.{i}");
            let headers = [
                ("cf-connecting-ip", "203.0.113.9"),
                ("x-forwarded-for", spoofed.as_str()),
            ];
            assert_eq!(
                post_contact(app.clone(), &headers).await,
                StatusCode::SERVICE_UNAVAILABLE
            );
        }
        let headers = [
            ("cf-connecting-ip", "203.0.113.9"),
            ("x-forwarded-for", "198.51.100.250"),
        ];
        assert_eq!(
            post_contact(app.clone(), &headers).await,
            StatusCode::TOO_MANY_REQUESTS
        );

        let other_client = [("cf-connecting-ip", "203.0.113.10")];
        assert_eq!(
            post_contact(app, &other_client).await,
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[tokio::test]
    async fn contact_without_configured_ip_header_is_rejected() {
        let app = test_app_with(ClientIpSource::CfConnectingIp);
        let headers = [("x-forwarded-for", "198.51.100.1")];
        assert_eq!(
            post_contact(app, &headers).await,
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[tokio::test]
    async fn contact_uses_socket_address_by_default() {
        let req =
            contact_request(&[]).extension(ConnectInfo(SocketAddr::from(([192, 0, 2, 1], 4000))));
        assert_eq!(
            send_contact(test_app(), req).await,
            StatusCode::SERVICE_UNAVAILABLE
        );
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
    async fn unknown_locale_returns_404() {
        let res = test_app()
            .oneshot(
                Request::builder()
                    .uri("/xx-XX")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn security_headers_present() {
        let res = test_app()
            .oneshot(
                Request::builder()
                    .uri("/healthz")
                    .body(Body::empty())
                    .unwrap(),
            )
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
