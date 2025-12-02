use crate::{dependency::AppContainer, rest};
use axum::{Router, http::StatusCode};
use std::time::Duration;
use tokio::{net::TcpListener, signal};
use tower_http::timeout::TimeoutLayer;
use trace_id::TraceIdLayer;

pub async fn run(state: AppContainer<'static>, listener: TcpListener) {
    let router = Router::new()
        .nest("/api/v1", rest::v1_handler())
        .with_state(state)
        .layer(TraceIdLayer::new())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ));

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to run server");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
