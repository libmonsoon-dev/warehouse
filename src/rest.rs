use crate::dependency::AppContainer;
use axum::Router;
use axum::routing::{get, post};

mod auth;
mod error;
mod health_check;

pub fn v1_handler() -> Router<AppContainer<'static>> {
    Router::new()
        .route("/health-check", get(health_check::health_check))
        .route("/auth/sign-up", post(auth::sign_up))
        .route("/auth/sign-in", post(auth::sign_in))
}
