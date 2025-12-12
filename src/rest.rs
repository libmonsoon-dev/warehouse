use crate::state::AppState;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

mod auth;
mod error;
mod health_check;

pub fn v1_handler() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(health_check::health_check))
        .nest("/auth", auth::router())
}
