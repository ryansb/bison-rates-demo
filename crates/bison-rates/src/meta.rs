use lambda_http::{http::HeaderValue, Response};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

pub fn version() -> String {
    format!("{}{}", env!("ENV"), env!("GIT_HASH"))
}

pub fn tower_tracer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    let format = tracing_subscriber::fmt::format()
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_level(true)
        .json();
    tracing_subscriber::fmt()
        .event_format(format)
        .with_max_level(Level::INFO)
        .init();

    tracing::info!({ version = version() }, "Container start");

    TraceLayer::new_for_http()
        .on_failure(DefaultOnFailure::new().level(Level::WARN))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
}

pub fn version_header<F>(mut resp: Response<F>) -> Response<F> {
    resp.headers_mut().insert(
        "X-Bison-Version",
        HeaderValue::from_str(&version()).unwrap(),
    );
    resp
}
