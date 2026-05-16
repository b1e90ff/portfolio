use std::net::SocketAddr;

use anyhow::{Context, Result};
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

mod app;
mod config;
mod i18n;
mod keywords;
mod locale;
mod routes;
mod state;
mod view;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    init_tracing();

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
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("axum server error")?;

    info!("shutdown complete");
    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_env("PORTFOLIO_LOG")
        .or_else(|_| EnvFilter::try_new("info,portfolio=debug,tower_http=info"))
        .expect("static filter is valid");

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(false).with_level(true).compact())
        .init();
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
