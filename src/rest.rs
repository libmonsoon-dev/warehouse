use crate::dependency::AppContainer;
use axum::Router;
use axum::routing::{get, post};

pub mod auth;
pub mod error;
pub mod health_check;

pub fn new_handler() -> Router<AppContainer<'static>> {
    Router::new()
        .route("/health-check", get(health_check::health_check))
        .route("/auth/sign-up", post(auth::sign_up))
        .route("/auth/sign-in", post(auth::sign_in))
}
