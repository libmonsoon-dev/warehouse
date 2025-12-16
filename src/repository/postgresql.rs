use crate::domain::RepositoryError;
use anyhow::anyhow;
use diesel::result::{DatabaseErrorKind, Error};

pub mod models;
mod role;
mod rule;
pub mod schema;
mod user;

pub use role::*;
pub use rule::*;
pub use user::*;

pub fn map_diesel_error(err: Error) -> anyhow::Error {
    match err {
        Error::NotFound => RepositoryError::NotFound.into(),
        Error::DatabaseError(kind, info) => match &kind {
            DatabaseErrorKind::UniqueViolation => {
                RepositoryError::Exists(anyhow!(info.message().to_string())).into()
            }
            _ => anyhow!(info.message().to_string()),
        },
        _ => err.into(),
    }
}
