use crate::contract::error::ErrorCode;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct AppError {
    pub code: ErrorCode,
    pub message: String,
}

impl From<ErrorCode> for AppError {
    fn from(code: ErrorCode) -> Self {
        Self {
            message: match code {
                ErrorCode::Ok => "",
                ErrorCode::UnexpectedError => "Unknow error",
                ErrorCode::ValidationFailed => "Invalid arguments",
                ErrorCode::AuthenticationFailed => "Invalid login or password",
                ErrorCode::ObjectNotFound => "Requested object not found",
                ErrorCode::ObjectAlreadyExists => "Provided object already exist",
            }
            .to_string(),
            code,
        }
    }
}

impl ErrorCode {
    pub fn status_code(&self) -> http::StatusCode {
        match self {
            ErrorCode::Ok => http::StatusCode::OK,
            ErrorCode::UnexpectedError => http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::ValidationFailed => http::StatusCode::BAD_REQUEST,
            ErrorCode::AuthenticationFailed => http::StatusCode::UNAUTHORIZED,
            ErrorCode::ObjectNotFound => http::StatusCode::NOT_FOUND,
            ErrorCode::ObjectAlreadyExists => http::StatusCode::CONFLICT,
        }
    }
}
