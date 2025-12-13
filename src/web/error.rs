use crate::dto::AppError;
use leptos::prelude::*;
use leptos::server_fn::codec::JsonEncoding;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use crate::contract::error::ErrorCode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerError(pub AppError);

#[cfg(feature = "ssr")]
impl<T> From<T> for ServerError
where
    T: Into<anyhow::Error> + Debug,
{
    fn from(err: T) -> Self {
        tracing_log::log::error!("{:?}", err);

        let err_code = crate::contract::error::ErrorCode::from(err.into().chain());
        crate::web::utils::expect_response_options().set_status(err_code.status_code());

        Self(AppError::from(err_code))
    }
}

impl FromServerFnError for ServerError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self(AppError{ code: ErrorCode::UnexpectedError, message: value.to_string() })
    }
}
