use crate::{contract::repository::error::RepositoryError, service::auth::AuthError};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use validator::ValidationError;

pub struct Error(anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{:?}", self.0);

        for cause in self.0.chain() {
            if let Some(cause) = cause.downcast_ref::<ValidationError>() {
                return (StatusCode::BAD_REQUEST, cause.to_string()).into_response();
            }

            if let Some(auth_error) = cause.downcast_ref::<AuthError>() {
                match auth_error {
                    AuthError::InvalidCredentials(_) => {
                        return StatusCode::UNAUTHORIZED.into_response();
                    }
                    AuthError::UnexpectedError(_) => continue,
                }
            }

            if let Some(repo_error) = cause.downcast_ref::<RepositoryError>() {
                match repo_error {
                    RepositoryError::NotFound => return StatusCode::NOT_FOUND.into_response(),
                    RepositoryError::Exists(_) => return StatusCode::CONFLICT.into_response(),
                    RepositoryError::UnexpectedError(_) => continue,
                }
            }
        }

        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<E> From<E> for Error
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
