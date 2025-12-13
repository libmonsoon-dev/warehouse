use crate::contract::error::ErrorCode;
use crate::dto::AppError;
use leptos::prelude::*;
use leptos::server_fn::codec::JsonEncoding;

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self {
            code: ErrorCode::UnexpectedError,
            message: value.to_string(),
        }
    }
}
