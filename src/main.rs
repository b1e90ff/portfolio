use std::net::SocketAddr;

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;

mod app;
mod config;
mod email;
mod i18n;
mod keywords;
mod locale;
mod og;
mod rate_limit;
mod routes;
mod state;
mod telemetry;
mod view;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let telemetry = telemetry::Telemetry::init()?;

    routes::api::boot_instant();
    let settings = config::Settings::from_env()?;
    let i18n = i18n::I18n::load(&settings.locales, &settings.default_locale)
        .context("loading translations")?;
    info!(locales = ?i18n.locales(), default = i18n.default_locale(), "translations loaded");

    let state = state::AppState::new(settings.clone(), i18n);
    let router = app::router(state);

    let addr: SocketAddr = settings.bind.parse().context("invalid PORTFOLIO_BIND")?;
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind {addr}"))?;

    info!(%addr, base_url = %settings.base_url, "portfolio listening");
    let served = axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .context("axum server error");

    info!("shutdown complete");
    telemetry.shutdown();
    served
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = signal::ctrl_c().await;
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut sig) = signal::unix::signal(signal::unix::SignalKind::terminate()) {
            sig.recv().await;
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
