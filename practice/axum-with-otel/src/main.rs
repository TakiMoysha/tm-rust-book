use std::{
    backtrace::{Backtrace, BacktraceStatus},
    io::{self, stderr},
    panic::PanicHookInfo,
    thread,
};

use axum::{Json, Router, extract::State, http::StatusCode, response::Html, routing::get};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use tokio::net::TcpListener;
use tracing::{error, info};

use tracing_opentelemetry;

use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::logs::SdkLoggerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::{Resource, logs::LogExporter};
use tracing_subscriber::{EnvFilter, prelude::*};

#[derive(Debug, thiserror::Error)]
pub enum InitializeError {
    #[error("Can't initialize OTEL `{0}`")]
    TelemetryInitError(String),
    #[error("unkown error")]
    UnknownError,
}

fn init_otel() -> Result<(), Box<dyn std::error::Error>> {
    // let otlp_exporter = opentelemetry_otlp::SpanExporter::builder().build()?;

    let trace_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_http()
        .with_endpoint("http://127.0.0.1:4000/v1/otlp/v1/traces")
        .build()?;
    let tracer_provider = SdkTracerProvider::builder().with_batch_exporter(trace_exporter).build();

    // 2. Создаем экспортер логов с явным указанием HTTP-эндпоинта GreptimeDB
    let log_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_http()
        .with_endpoint("http://127.0.0.1:4000/v1/otlp/v1/logs")
        .build()?;
    let log_provider = SdkLoggerProvider::builder().with_batch_exporter(log_exporter).build();

    let filter_otel = EnvFilter::new("info")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());

    let otel_layer = layer::OpenTelemetryTracingBridge::new(&log_provider).with_filter(filter_otel);

    let filter_fmt = EnvFilter::new("info").add_directive("opentelemetry=warn".parse().unwrap());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_filter(filter_fmt);

    tracing_subscriber::registry().with(otel_layer).with(fmt_layer).init();

    error!(
        name: "DEV:Test-Error",
        target: "initi-system",
        event_id = 20,
        user_name = "otel",
        user_email = "otel@opentelemetry.io",
        message = "Test connection to OTEL"
    );

    let _ = ();
    Ok(())
}

#[derive(Clone)]
struct AppState;

#[deprecated]
async fn health_check(State(state): State<AppState>) -> Result<Json<&'static str>, StatusCode> {
    Ok(Json("{ 'hello': 'world' }"))
}

#[deprecated]
async fn slow_handler(State(state): State<AppState>) -> Result<Json<&'static str>, StatusCode> {
    info!("Slow handler started");
    Ok(Json("{ 'status': 'Ok'}"))
}

#[deprecated]
async fn fast_handler() -> &'static str {
    info!("Fast handler");
    "Fast response"
}

fn panik_hook(info: &PanicHookInfo<'_>) {
    let thread = thread::current();
    let name = thread.name().unwrap_or("<unnamed>");

    // let message = match info.payload().downcast_ref()::<&str>() {
    //     Some(message) => *message,
    //     None => match info.payload().downcast_ref::<String>() {
    //         Some(msg) => msg.as_str(),
    //         None => "Box<dyn Any>",
    //     }
    // };
    //
    // let mut stderr = io::stderr().lock();
    //
    // if let Some(location) = info.location() {
    //     let _ = writeln!( stderr, "\nthread '{name}' panicked at {location}:\n{message}");
    // } else {
    //     let _ = writeln!(stderr, "\nthread '{name}' panicked:\n{message}");
    // }
    //
    // let backtrace = Backtrace::capture();
    // let backtrace.status() == BacktraceStatus::Captured {
    //     let _ = writeln!(stderr, "stack backtrace:\n{backtrace}");
    // }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let otel = (
    //     env!("OTE_SERVICE_NAME"),
    //     env!("OTEL_EXPORTER_OTLP_ENDPOINT"),
    //     env!("OTEL_EXPORTER_OTLP_PROTO"),
    //     env!("OTEL_TRACES_EXPORTER"),
    //     env!("OTEL_METRICS_EXPORTER"),
    // );

    let otel_telemetry = init_otel();
    let state = AppState {};

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/slow", get(slow_handler))
        .route("/fast", get(fast_handler))
        .with_state(state);

    let addr = "0.0.0.0:3000";
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {}", addr));

    info!(
        "Server listening on <{:?}>",
        listener.local_addr().expect("can't unpack local_addr()")
    );

    let server = axum::serve(listener, app.into_make_service());

    server.await.map_err(|err| ());

    Ok(())
}
