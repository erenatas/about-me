
use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::logs::{LogError, LoggerProvider};
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;


#[cfg(feature = "ssr")]
fn init_otel_logging()  -> Result<opentelemetry_sdk::logs::LoggerProvider, LogError> {
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
        .with_resource(Resource::new([KeyValue::new(
            SERVICE_NAME,
            "about-me",
        )]))
        .with_batch_exporter(exporter, runtime::Tokio)
        .build()
    )
}

#[cfg(feature = "ssr")]
pub fn init_opentelemetry() {
    use tracing::info;
    use tracing_subscriber::{prelude::*, Registry};
    use log::LevelFilter;
    use tracing::subscriber::set_global_default;


    // Initialize logs and save the logger_provider.
    let logger_provider = init_otel_logging().unwrap();

    // Create the OpenTelemetry bridges
    let otel_log_bridge = OpenTelemetryLogBridge::new(&logger_provider);
    let otel_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    // Build and set the subscriber first
    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer()) // Optional: adds console logging
        .with(otel_layer);

    // Set the global subscriber
    set_global_default(subscriber)
        .expect("Failed to set global subscriber");

    // Set up the log bridge after tracing is initialized
    log::set_boxed_logger(Box::new(otel_log_bridge))
        .expect("Failed to set logger");
    log::set_max_level(LevelFilter::Info);

    // Test log
    info!(name: "my-event", target: "my-target", "hello from {}. My price is {}", "apple", 1.99);
}