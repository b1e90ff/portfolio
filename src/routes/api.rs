use std::net::IpAddr;
use std::sync::OnceLock;
use std::time::Instant;

use axum::Json;
use axum::extract::{ConnectInfo, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::warn;
use validator::Validate;

use crate::state::AppState;

static BOOT_INSTANT: OnceLock<Instant> = OnceLock::new();

pub fn boot_instant() -> Instant {
    *BOOT_INSTANT.get_or_init(Instant::now)
}

#[derive(Debug, Deserialize, Validate)]
pub struct ContactPayload {
    #[validate(length(min = 2, max = 100))]
    pub name: String,
    #[validate(email, length(max = 254))]
    pub email: String,
    #[validate(length(min = 3, max = 200))]
    pub subject: String,
    #[validate(length(min = 10, max = 5000))]
    pub message: String,
    #[serde(default, rename = "_honeypot")]
    pub honeypot: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiOk {
    pub ok: bool,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: &'static str,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub timestamp: String,
    pub uptime: u64,
    pub version: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        timestamp: Utc::now().to_rfc3339(),
        uptime: boot_instant().elapsed().as_secs(),
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub async fn contact(
    State(state): State<AppState>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    Json(payload): Json<ContactPayload>,
) -> Response {
    let ip = client_ip(&headers).unwrap_or(addr.ip());

    if !state.contact_rate_limit.check(ip) {
        warn!(%ip, "contact form rate-limited");
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(ApiError {
                error: "rate_limited",
            }),
        )
            .into_response();
    }

    if payload.honeypot.as_deref().is_some_and(|h| !h.is_empty()) {
        return (StatusCode::OK, Json(ApiOk { ok: true })).into_response();
    }

    if let Err(errors) = payload.validate() {
        warn!(?errors, "contact form validation failed");
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "invalid_input",
            }),
        )
            .into_response();
    }

    let Some(mailer) = state.mailer.as_ref() else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ApiError {
                error: "contact_unavailable",
            }),
        )
            .into_response();
    };

    match mailer
        .send_contact(
            payload.name.trim(),
            payload.email.trim(),
            payload.subject.trim(),
            payload.message.trim(),
        )
        .await
    {
        Ok(_) => (StatusCode::OK, Json(ApiOk { ok: true })).into_response(),
        Err(err) => {
            tracing::error!(?err, "failed to deliver contact email");
            (
                StatusCode::BAD_GATEWAY,
                Json(ApiError {
                    error: "delivery_failed",
                }),
            )
                .into_response()
        }
    }
}

fn client_ip(headers: &HeaderMap) -> Option<IpAddr> {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse().ok())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.trim().parse().ok())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_ip_prefers_forwarded_for() {
        let mut h = HeaderMap::new();
        h.insert("x-forwarded-for", "203.0.113.5, 10.0.0.1".parse().unwrap());
        let ip = client_ip(&h).unwrap();
        assert_eq!(ip.to_string(), "203.0.113.5");
    }

    #[test]
    fn client_ip_falls_back_to_real_ip() {
        let mut h = HeaderMap::new();
        h.insert("x-real-ip", "198.51.100.7".parse().unwrap());
        let ip = client_ip(&h).unwrap();
        assert_eq!(ip.to_string(), "198.51.100.7");
    }

    #[test]
    fn payload_validates_email_and_length() {
        let bad = ContactPayload {
            name: "x".into(),
            email: "no-at-sign".into(),
            subject: "ab".into(),
            message: "short".into(),
            honeypot: None,
        };
        assert!(bad.validate().is_err());

        let ok = ContactPayload {
            name: "Alice".into(),
            email: "a@example.com".into(),
            subject: "Hello".into(),
            message: "This is at least ten characters long.".into(),
            honeypot: None,
        };
        assert!(ok.validate().is_ok());
    }
}
