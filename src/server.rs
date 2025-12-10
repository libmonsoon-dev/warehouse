use crate::state::AppState;
use crate::web::app::{App, shell};
use crate::{dependency::AppContainer, rest};
use axum::{Router, http::StatusCode};
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, file_and_error_handler_with_context, generate_route_list};
use std::time::Duration;
use tokio::{net::TcpListener, signal};
use tower_http::timeout::TimeoutLayer;
use trace_id::TraceIdLayer;

pub async fn run(leptos_options: LeptosOptions, dependencies: AppContainer<'static>, listener: TcpListener) {
    let routes = generate_route_list(App);

    let app_state = AppState {
        dependencies,
        leptos_options,
    };

    let router = Router::new()
        .leptos_routes_with_context(
            &app_state,
            routes,
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            {
                let app_state = app_state.clone();
                move || shell(app_state.leptos_options.clone())
            },
        )
        .nest("/api/v1", rest::v1_handler())
        .fallback(file_and_error_handler_with_context::<AppState, _>(
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            shell,
        ))
        .with_state(app_state)
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
