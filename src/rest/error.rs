use crate::contract::error::ErrorCode;
use crate::dto::AppError;
use axum::Json;
use axum::response::{IntoResponse, Response};
use tracing_log::log;

pub struct ServerError(anyhow::Error);

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        log::error!("{:?}", self.0);

        AppError::from(ErrorCode::from(self.0.chain())).into_response()
    }
}

impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.code.status_code();
        let mut response = Json::from(self).into_response();
        *response.status_mut() = status;

        response
    }
}
