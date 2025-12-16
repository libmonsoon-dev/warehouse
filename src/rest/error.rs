use crate::dto::AppError;
use axum::Json;
use axum::response::{IntoResponse, Response};

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.code.status_code();
        let mut response = Json::from(self).into_response();
        *response.status_mut() = status;

        response
    }
}
