use prometheus::{CounterVec, HistogramOpts, HistogramVec, Opts};
use std::fs::OpenOptions;
use std::sync::{Mutex, OnceLock};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_tracing() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();

    tracing_subscriber::registry()
        .with(EnvFilter::new("obsv=info,actix_server=off,actix_http=off"))
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_file(false)
                .with_line_number(false)
                .with_writer(Mutex::new(file)),
        )
        .init();
}

static HTTP_REQUESTS: OnceLock<CounterVec> = OnceLock::new();

pub fn http_requests() -> &'static CounterVec {
    HTTP_REQUESTS.get_or_init(|| {
        prometheus::register_counter_vec!(
            Opts::new("http_requests_total", "Total HTTP requests"),
            &["endpoint", "status_code"]
        )
        .unwrap()
    })
}

static HTTP_DURATION: OnceLock<HistogramVec> = OnceLock::new();

pub fn http_duration() -> &'static HistogramVec {
    HTTP_DURATION.get_or_init(|| {
        prometheus::register_histogram_vec!(
            HistogramOpts::new("http_request_duration_seconds", "Request latency in seconds")
                .buckets(vec![0.01, 0.05, 0.1, 0.5, 1.0]),
            &["endpoint"]
        )
        .unwrap()
    })
}
