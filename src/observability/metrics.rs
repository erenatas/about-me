use once_cell::sync::Lazy;
use opentelemetry::global;
use opentelemetry::metrics::Counter;

pub static PAGE_VIEWS: Lazy<Counter<u64>> = Lazy::new(|| {
    global::meter("about-me")
        .u64_counter("pageviews")
        .with_description("Number of page views")
        .build()
});

pub static API_REQUESTS: Lazy<Counter<u64>> = Lazy::new(|| {
    global::meter("about-me")
        .u64_counter("api_requests")
        .with_description("Number of API requests")
        .build()
});
