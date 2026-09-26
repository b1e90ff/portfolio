use std::env;

use anyhow::{Context, Result, bail};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{Key, KeyValue};
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

        let tracer_provider = if otlp_endpoint_configured(|k| env::var(k).ok())? {
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

/// The exporter has no TLS support, so only plain-HTTP collector endpoints are accepted.
fn otlp_endpoint_configured(lookup: impl Fn(&str) -> Option<String>) -> Result<bool> {
    let mut configured = false;
    for var in ENDPOINT_VARS {
        let Some(value) = lookup(var).filter(|v| !v.trim().is_empty()) else {
            continue;
        };
        if !value.trim_start().starts_with("http://") {
            bail!("{var} must use an http:// collector endpoint, got {value:?}");
        }
        configured = true;
    }
    Ok(configured)
}

fn tracer_provider() -> Result<SdkTracerProvider> {
    let exporter = SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()
        .context("build otlp span exporter")?;

    Ok(SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource())
        .build())
}

fn resource() -> Resource {
    let builder = Resource::builder()
        .with_attribute(KeyValue::new("service.version", env!("CARGO_PKG_VERSION")));
    if has_default_service_name(&Resource::builder().build()) {
        builder.with_service_name(env!("CARGO_PKG_NAME")).build()
    } else {
        builder.build()
    }
}

/// True when neither OTEL_SERVICE_NAME nor OTEL_RESOURCE_ATTRIBUTES provided a service name.
fn has_default_service_name(resource: &Resource) -> bool {
    resource
        .get(&Key::new("service.name"))
        .is_none_or(|name| name.as_str().starts_with("unknown_service"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_is_disabled_without_endpoint() {
        assert!(!otlp_endpoint_configured(|_| None).unwrap());
        assert!(!otlp_endpoint_configured(|_| Some("  ".into())).unwrap());
    }

    #[test]
    fn either_endpoint_variable_enables_export() {
        for var in ENDPOINT_VARS {
            let lookup = |k: &str| (k == var).then(|| "http://otel-collector:4318".to_string());
            assert!(
                otlp_endpoint_configured(lookup).unwrap(),
                "{var} should enable export"
            );
        }
    }

    #[test]
    fn https_endpoint_is_rejected_at_startup() {
        for var in ENDPOINT_VARS {
            let lookup = |k: &str| (k == var).then(|| "https://collector.example:4318".to_string());
            assert!(
                otlp_endpoint_configured(lookup).is_err(),
                "{var} should reject https"
            );
        }
    }

    #[test]
    fn sdk_fallback_service_name_counts_as_default() {
        let named = |name: &str| {
            Resource::builder_empty()
                .with_service_name(name.to_string())
                .build()
        };
        assert!(has_default_service_name(&Resource::builder_empty().build()));
        assert!(has_default_service_name(&named(
            "unknown_service:portfolio"
        )));
        assert!(!has_default_service_name(&named("portfolio-staging")));
    }

    #[test]
    fn tracer_provider_builds_without_collector() {
        let provider = tracer_provider().expect("exporter builds lazily");
        provider.shutdown().expect("clean shutdown");
    }
}
