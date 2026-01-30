// - безопасная останова без потери запросов и операций:
//  - остановка принятия новых запросов
//  - плановые задачи и фонове процессы должны прекратить ставить в очередь новые задания
//  - закрытие сессий и соединений, завершение транзакций, потребителей сообщений и очередей
// - интеграционные тесты должны покрывать и завершение работы сервиса

use axum::{Router, extract::State, http::StatusCode, response::Html, routing::get};
use tokio::net::TcpListener;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[derive(Clone)]
struct AppState {
    shutdown_token: CancellationToken,
}

async fn health_check(State(state): State<AppState>) -> &'static str {
    info!("Health check");
    if state.shutdown_token.is_cancelled() {
        return "Shutting down";
    }
    "OK"
}

async fn slow_handler(State(state): State<AppState>) -> Result<Html<&'static str>, StatusCode> {
    info!("Slow handler started");
    tokio::select! {
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(10)) => {
            info!("Slow handler finished normally");
            Ok(Html("<h1>Done after 10s </h1>"))
        }
        _ = state.shutdown_token.cancelled() => {
            info!("Slow handler cancelled due to shutdown");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}

async fn fast_handler() -> &'static str {
    info!("Fast handler");
    "Fast response"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let shutdown_token = CancellationToken::new();

    let state = AppState {
        shutdown_token: shutdown_token.clone(),
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/slow", get(slow_handler))
        .route("/fast", get(fast_handler))
        .with_state(state);

    let addr = "0.0:3000";
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind to {}", addr));

    info!("Server listening on <{:#?}>", listener.local_addr());

    let server = axum::serve(listener, app.into_make_service());

    let shutdown_signal = async {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("Failed to listen for SIGTERM signal")
        };
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to listen for SIGTERM signal")
                .recv()
                .await;
        };

        tokio::select! {
            _ = ctrl_c => {}
            _ = terminate => {}
        }
    };

    let graceful_shutdown = async {
        let timeout_duration = tokio::time::Duration::from_secs(20);
        let timeout = tokio::time::timeout(timeout_duration, server);

        match timeout.await {
            Ok(Ok(())) => info!("Server stopped"),
            Ok(Err(e)) => error!("Server stopped with error: {}", e),
            Err(_) => error!("Server shutdown timed out, graceful shutdown failed"),
        }
    };

    tokio::select! {
        _ = shutdown_signal => info!("Shutdown signal received"),
        _ = graceful_shutdown => info!("Server shutdown complete"),

    }

    Ok(())
}
