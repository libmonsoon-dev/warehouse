pub use crate::dependency::AppContainer;
use crate::routes::{auth, health_check};
use axum::Router;
use axum::routing::{get, post};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use trace_id::TraceIdLayer;

pub async fn run(state: AppContainer<'static>, listener: TcpListener) {
    let v1 = Router::new()
        .route("/health-check", get(health_check::health_check))
        .route("/auth/sign-up", post(auth::sign_up))
        .route("/auth/sign-in", post(auth::sign_in));

    let router = Router::new()
        .nest("/api/v1", v1)
        .with_state(state)
        .layer(TraceIdLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(10)));

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
