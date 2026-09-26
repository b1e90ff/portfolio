use std::env;

use anyhow::{Context, Result};
use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_subscriber::{EnvFilter, Layer, fmt, prelude::*};

const DEFAULT_LOG_FILTER: &str = "info,portfolio=debug,tower_http=info";
const ENDPOINT_VARS: [&str; 2] = [
    "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT",
    "OTEL_EXPORTER_OTLP_ENDPOINT",
];
// Exporter transport crates must not feed their own spans back into the exporter.
const EXPORT_FILTER: &str = "trace,reqwest=off,hyper=off,hyper_util=off,h2=off,opentelemetry=off";

pub struct Telemetry {
    tracer_provider: Option<SdkTracerProvider>,
}

impl Telemetry {
    /// Installs stdout logging and, when an OTLP endpoint is configured, span export.
    pub fn init() -> Result<Self> {
        let filter = EnvFilter::try_from_env("PORTFOLIO_LOG")
            .or_else(|_| EnvFilter::try_new(DEFAULT_LOG_FILTER))
            .context("build log filter")?;

        let tracer_provider = if otlp_endpoint_configured(|k| env::var(k).ok()) {
            Some(tracer_provider()?)
        } else {
            None
        };

        let otel_layer = tracer_provider.as_ref().map(|provider| {
            tracing_opentelemetry::layer()
                .with_tracer(provider.tracer(env!("CARGO_PKG_NAME")))
                .with_filter(EnvFilter::new(EXPORT_FILTER))
        });

        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().with_target(false).with_level(true).compact())
            .with(otel_layer)
            .try_init()
            .context("install tracing subscriber")?;

        if tracer_provider.is_some() {
            tracing::info!("otlp span export enabled");
        }
        Ok(Self { tracer_provider })
    }

    pub fn shutdown(self) {
        if let Some(provider) = self.tracer_provider
            && let Err(err) = provider.shutdown()
        {
            tracing::warn!(%err, "otlp tracer provider shutdown failed");
        }
    }
}

fn otlp_endpoint_configured(lookup: impl Fn(&str) -> Option<String>) -> bool {
    ENDPOINT_VARS
        .iter()
        .any(|k| lookup(k).is_some_and(|v| !v.trim().is_empty()))
}

fn tracer_provider() -> Result<SdkTracerProvider> {
    let exporter = SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()
        .context("build otlp span exporter")?;

    let mut resource = Resource::builder()
        .with_attribute(KeyValue::new("service.version", env!("CARGO_PKG_VERSION")));
    if env::var_os("OTEL_SERVICE_NAME").is_none() {
        resource = resource.with_service_name(env!("CARGO_PKG_NAME"));
    }

    Ok(SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource.build())
        .build())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_is_disabled_without_endpoint() {
        assert!(!otlp_endpoint_configured(|_| None));
        assert!(!otlp_endpoint_configured(|_| Some("  ".into())));
    }

    #[test]
    fn either_endpoint_variable_enables_export() {
        for var in ENDPOINT_VARS {
            let lookup = |k: &str| (k == var).then(|| "http://otel-collector:4318".to_string());
            assert!(
                otlp_endpoint_configured(lookup),
                "{var} should enable export"
            );
        }
    }

    #[test]
    fn tracer_provider_builds_without_collector() {
        let provider = tracer_provider().expect("exporter builds lazily");
        provider.shutdown().expect("clean shutdown");
    }
}
