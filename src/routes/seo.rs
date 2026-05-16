use std::sync::OnceLock;

use axum::Json;
use axum::extract::State;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::locale::LocaleCtx;
use crate::og;
use crate::state::AppState;

const STATIC_PATHS: &[(&str, &str, &str)] = &[
    ("", "1.0", "weekly"),
    ("/about", "0.8", "monthly"),
    ("/projects", "0.9", "weekly"),
    ("/contact", "0.8", "monthly"),
    ("/privacy", "0.3", "yearly"),
    ("/impressum", "0.3", "yearly"),
];

fn site_lastmod() -> &'static str {
    static V: OnceLock<String> = OnceLock::new();
    V.get_or_init(|| chrono::Utc::now().format("%Y-%m-%d").to_string())
}

pub async fn sitemap(State(state): State<AppState>) -> Response {
    let base = &state.settings.base_url;
    let mut xml = String::with_capacity(8192);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str(
        "<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" \
          xmlns:xhtml=\"http://www.w3.org/1999/xhtml\" \
          xmlns:image=\"http://www.google.com/schemas/sitemap-image/1.1\">\n",
    );

    let locales = state.i18n.locales();
    let default_locale = state.i18n.default_locale();
    let site_mod = site_lastmod();

    for locale in locales {
        let m = state.i18n.get(locale);
        for (path, priority, changefreq) in STATIC_PATHS {
            url_entry(
                &mut xml,
                base,
                locale,
                locales,
                default_locale,
                path,
                priority,
                changefreq,
                site_mod,
                None,
            );
        }
        for project in &m.projects.items {
            let path = format!("/projects/{}", project.id);
            let image = if project.image.starts_with("http") {
                project.image.clone()
            } else {
                format!("{base}{}", project.image)
            };
            url_entry(
                &mut xml,
                base,
                locale,
                locales,
                default_locale,
                &path,
                "0.7",
                "monthly",
                &project.date,
                Some(&image),
            );
        }
    }

    xml.push_str("</urlset>\n");

    (
        [
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/xml; charset=utf-8"),
            ),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, s-maxage=86400, stale-while-revalidate=604800"),
            ),
        ],
        xml,
    )
        .into_response()
}

#[allow(clippy::too_many_arguments)]
fn url_entry(
    out: &mut String,
    base: &str,
    locale: &str,
    locales: &[String],
    default_locale: &str,
    path: &str,
    priority: &str,
    changefreq: &str,
    lastmod: &str,
    image: Option<&str>,
) {
    out.push_str("  <url>\n");
    out.push_str(&format!("    <loc>{base}/{locale}{path}</loc>\n"));
    out.push_str(&format!("    <lastmod>{lastmod}</lastmod>\n"));
    out.push_str(&format!("    <changefreq>{changefreq}</changefreq>\n"));
    out.push_str(&format!("    <priority>{priority}</priority>\n"));
    for alt in locales {
        out.push_str(&format!(
            "    <xhtml:link rel=\"alternate\" hreflang=\"{alt}\" href=\"{base}/{alt}{path}\"/>\n"
        ));
    }
    out.push_str(&format!(
        "    <xhtml:link rel=\"alternate\" hreflang=\"x-default\" href=\"{base}/{default_locale}{path}\"/>\n"
    ));
    if let Some(img) = image {
        out.push_str("    <image:image>\n");
        out.push_str(&format!("      <image:loc>{img}</image:loc>\n"));
        out.push_str("    </image:image>\n");
    }
    out.push_str("  </url>\n");
}

pub async fn robots(State(state): State<AppState>) -> Response {
    let body = format!(
        "User-agent: *\n\
         Allow: /\n\
         Disallow: /api/\n\
         Disallow: /admin/\n\
         Disallow: /impressum/\n\
         Disallow: /privacy/\n\
         Disallow: /favicon.ico\n\
         Disallow: /site.webmanifest\n\
         \n\
         Sitemap: {base}/sitemap.xml\n\
         Host: {base}\n",
        base = state.settings.base_url,
    );

    (
        [
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/plain; charset=utf-8"),
            ),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, s-maxage=86400, stale-while-revalidate=604800"),
            ),
        ],
        body,
    )
        .into_response()
}

pub async fn opengraph_image(State(_state): State<AppState>, ctx: LocaleCtx) -> Response {
    let m = ctx.messages.as_ref();
    match og::render_png(&ctx.locale, m) {
        Ok(bytes) => (
            [
                (header::CONTENT_TYPE, HeaderValue::from_static("image/png")),
                (
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("public, max-age=3600, stale-while-revalidate=86400"),
                ),
            ],
            bytes,
        )
            .into_response(),
        Err(err) => {
            tracing::error!(?err, "og image render failed");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn manifest(State(state): State<AppState>) -> Response {
    let m = state.i18n.get(&state.settings.default_locale);
    let manifest = json!({
        "name": m.metadata.manifest.name,
        "short_name": m.metadata.manifest.short_name,
        "description": m.metadata.manifest.description,
        "start_url": format!("/{}", state.settings.default_locale),
        "scope": "/",
        "display": "standalone",
        "orientation": "portrait",
        "background_color": "#060608",
        "theme_color": "#060608",
        "icons": [
            {
                "src": "/android-chrome-192x192.png",
                "sizes": "192x192",
                "type": "image/png",
                "purpose": "any maskable"
            },
            {
                "src": "/android-chrome-512x512.png",
                "sizes": "512x512",
                "type": "image/png",
                "purpose": "any maskable"
            },
            {
                "src": "/apple-touch-icon.png",
                "sizes": "180x180",
                "type": "image/png"
            }
        ]
    });

    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/manifest+json"),
        )],
        Json(manifest),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use crate::app::router;
    use crate::config::Settings;
    use crate::i18n::I18n;
    use crate::state::AppState;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn app() -> axum::Router {
        let settings = Settings {
            bind: "0.0.0.0:0".into(),
            base_url: "https://example.test".into(),
            default_locale: "en-US".into(),
            locales: vec!["en-US".into(), "de-DE".into()],
            smtp: None,
        };
        let i18n = I18n::load(&settings.locales, &settings.default_locale).unwrap();
        router(AppState::new(settings, i18n))
    }

    async fn body_of(app: axum::Router, path: &str) -> (StatusCode, String, axum::http::HeaderMap) {
        let res = app
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = res.status();
        let headers = res.headers().clone();
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        (
            status,
            String::from_utf8(bytes.to_vec()).unwrap_or_default(),
            headers,
        )
    }

    #[tokio::test]
    async fn sitemap_lists_every_locale_and_every_static_path() {
        let (status, body, h) = body_of(app(), "/sitemap.xml").await;
        assert_eq!(status, StatusCode::OK);
        assert!(
            h.get("content-type")
                .unwrap()
                .to_str()
                .unwrap()
                .contains("application/xml"),
            "content-type was {:?}",
            h.get("content-type")
        );
        for locale in ["en-US", "de-DE"] {
            for path in [
                "",
                "/about",
                "/projects",
                "/contact",
                "/privacy",
                "/impressum",
            ] {
                let expected = format!("<loc>https://example.test/{locale}{path}</loc>");
                assert!(body.contains(&expected), "missing {expected}");
            }
        }
        assert!(body.contains("/projects/eventfrog"));
        assert!(body.contains(r#"hreflang="de-DE""#));
        assert!(body.contains(r#"hreflang="x-default""#));
        assert!(body.contains("<lastmod>"));
        assert!(body.contains("<image:image>"));
        assert!(body.contains("xmlns:image="));
    }

    #[tokio::test]
    async fn robots_points_at_sitemap_and_blocks_api() {
        let (status, body, _) = body_of(app(), "/robots.txt").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("Disallow: /api/"));
        assert!(body.contains("Sitemap: https://example.test/sitemap.xml"));
    }

    #[tokio::test]
    async fn manifest_returns_pwa_metadata() {
        let (status, body, h) = body_of(app(), "/site.webmanifest").await;
        assert_eq!(status, StatusCode::OK);
        assert!(
            h.get("content-type")
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("application/manifest+json"),
        );
        assert!(body.contains(r#""name":"tat.systems""#));
        assert!(body.contains("android-chrome-512x512.png"));
    }
}
