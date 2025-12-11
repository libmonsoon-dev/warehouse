use crate::contract::error::ErrorCode;
use crate::dto::AppError;
use anyhow::{anyhow, Error};
use bytes::Bytes;
use leptos::prelude::*;
use leptos::server_fn::{ContentType, Decodes, Encodes, Format, FormatType};
use std::fmt::{Debug, Display, Formatter};
use std::string::FromUtf8Error;
use tracing_log::log;

pub struct ServerError(Error);

impl<T> From<T> for ServerError
where
    T: Into<Error>,
{
    fn from(err: T) -> Self {
        ServerError(err.into())
    }
}

impl Debug for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Clone for ServerError {
    fn clone(&self) -> Self {
        Self(anyhow!(self.0.to_string()))
    }
}

impl FromServerFnError for ServerError {
    type Encoder = ServerErrorEncoder;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        Self(value.into())
    }
}

pub struct ServerErrorEncoder;

impl ContentType for ServerErrorEncoder {
    const CONTENT_TYPE: &'static str = "text/plain";
}

impl FormatType for ServerErrorEncoder {
    const FORMAT_TYPE: Format = Format::Text;
}

impl Encodes<ServerError> for ServerErrorEncoder {
    type Error = std::fmt::Error;

    fn encode(output: &ServerError) -> Result<Bytes, Self::Error> {
        log::error!("{:?}", output.0);

        Ok(Bytes::from(
            AppError::from(ErrorCode::from(output.0.chain())).message,
        ))
    }
}

impl Decodes<ServerError> for ServerErrorEncoder {
    type Error = FromUtf8Error;

    fn decode(bytes: Bytes) -> Result<ServerError, Self::Error> {
        Ok(ServerError(anyhow!(String::from_utf8(bytes.to_vec())?)))
    }
}
