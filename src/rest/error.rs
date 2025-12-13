use crate::contract::error::ErrorCode;
use crate::dto::AppError;
use axum::Json;
use axum::response::{IntoResponse, Response};
use std::fmt::Debug;
use tracing_log::log;

pub struct ServerError(AppError);

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status = self.0.code.status_code();
        let mut response = Json::from(self.0).into_response();
        *response.status_mut() = status;

        response
    }
}

impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error> + Debug,
{
    fn from(err: E) -> Self {
        log::error!("{:?}", err);

        Self(AppError::from(ErrorCode::from(err.into().chain())))
    }
}
