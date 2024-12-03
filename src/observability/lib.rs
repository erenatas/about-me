
#[cfg(feature = "ssr")]
use axum_otel_metrics::HttpMetricsLayer;
use once_cell::sync::Lazy;
use opentelemetry_sdk::logs::{LogError, LoggerProvider};
use opentelemetry_sdk::{metrics::{MetricError, SdkMeterProvider}, Resource};
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use opentelemetry::KeyValue;


#[cfg(feature = "ssr")]
static RESOURCE: Lazy<Resource> = Lazy::new(|| {
    Resource::new(vec![KeyValue::new(SERVICE_NAME, "about-me")])
});

#[cfg(feature = "ssr")]
fn init_otel_logging()  -> Result<LoggerProvider, LogError> {
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_otlp::LogExporter;
    use opentelemetry_sdk::runtime;

    const OTLP_ENDPOINT: &str = "http://127.0.0.1:4317";

    let exporter = LogExporter::builder()
        .with_tonic()
        .with_endpoint(OTLP_ENDPOINT)
        .with_protocol(opentelemetry_otlp::Protocol::Grpc)
        .build()?;
    
    Ok(LoggerProvider::builder()
        .with_resource(RESOURCE.clone())
        .with_batch_exporter(exporter, runtime::Tokio)
        .build()
    )
}

#[cfg(feature = "ssr")]
fn init_otel_metrics() -> Result<SdkMeterProvider, MetricError> {
    use opentelemetry_otlp::MetricExporter;
    use opentelemetry_sdk::{metrics::PeriodicReader, runtime};

    let exporter = MetricExporter::builder().with_tonic().build()?;

    let reader = PeriodicReader::builder(exporter, runtime::Tokio).build();

    Ok(SdkMeterProvider::builder().with_reader(reader).with_resource(RESOURCE.clone()).build())
}

#[cfg(feature = "ssr")]
pub fn init_opentelemetry() {
    use opentelemetry::global;
    use tracing_subscriber::{EnvFilter, prelude::*, Registry};
    use log::LevelFilter;
    use tracing::subscriber::set_global_default;
    use opentelemetry_appender_log::OpenTelemetryLogBridge;
    use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
    

    match init_otel_logging() {
        Ok(logger_provider) => {
            // Create the OpenTelemetry bridges
            let otel_log_bridge = OpenTelemetryLogBridge::new(&logger_provider);
            let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider);


            // Create an EnvFilter that only allows INFO and above
            let filter = EnvFilter::new("info");

            // Build and set the subscriber first
            let subscriber = Registry::default()
                .with(filter)
                .with(tracing_subscriber::fmt::layer()) // Optional: adds console logging
                .with(otel_layer);

            // Set the global subscriber
            set_global_default(subscriber)
                .expect("Failed to set global subscriber");

            // Set up the log bridge after tracing is initialized
            log::set_boxed_logger(Box::new(otel_log_bridge))
                .expect("Failed to set logger");
            log::set_max_level(LevelFilter::Info);
        },
        Err(error) => {
            eprintln!("Failed to set the logger: {}", error);
        }
    }
    match init_otel_metrics() {
        Ok(meter_provider) => {
            global::set_meter_provider(meter_provider);
        },
        Err(error) => {
            eprintln!("Failed to set the meter: {}", error);
        }
    }
}

#[cfg(feature = "ssr")]
pub fn get_axum_metrics_layer() -> HttpMetricsLayer {
    use axum_otel_metrics::HttpMetricsLayerBuilder;

    HttpMetricsLayerBuilder::new()
    .with_service_name("about-me".to_string())
    .with_service_version(env!("CARGO_PKG_VERSION").to_string())
    .with_exporter("otlp".to_string())
    .build()
}