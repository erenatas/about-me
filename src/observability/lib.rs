use log::Level;
use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_sdk::logs::LoggerProvider;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::resource::SERVICE_NAME;

fn init_otel_logging() {
    let exporter = opentelemetry_stdout::LogExporter::default();
    let logger_provider = LoggerProvider::builder()
        .with_resource(Resource::new([KeyValue::new(
            SERVICE_NAME,
            "about-me",
        )]))
        .with_simple_exporter(exporter)
        .build();

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(Level::Info.to_level_filter());
}

pub fn init_opentelemetry() {
    init_otel_logging();
}