use crate::domain::{AuthError, RepositoryError};
use anyhow::Chain;
use validator::ValidationError;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum ErrorCode {
    Ok = 0,
    UnexpectedError = 1,
    ValidationFailed = 2,
    AuthenticationFailed = 3,
    ObjectNotFound = 4,
    ObjectAlreadyExists = 5,
}

impl From<Chain<'_>> for ErrorCode {
    fn from(chain: Chain) -> Self {
        for cause in chain {
            if cause.downcast_ref::<ValidationError>().is_some() {
                return ErrorCode::ValidationFailed;
            }

            if let Some(auth_error) = cause.downcast_ref::<AuthError>() {
                match auth_error {
                    AuthError::InvalidCredentials(_) => {
                        return ErrorCode::AuthenticationFailed;
                    }
                    AuthError::UnexpectedError(_) => continue,
                }
            }

            if let Some(repo_error) = cause.downcast_ref::<RepositoryError>() {
                match repo_error {
                    RepositoryError::NotFound => return ErrorCode::ObjectNotFound,
                    RepositoryError::Exists(_) => return ErrorCode::ObjectAlreadyExists,
                    RepositoryError::UnexpectedError(_) => continue,
                }
            }
        }

        ErrorCode::UnexpectedError
    }
}
