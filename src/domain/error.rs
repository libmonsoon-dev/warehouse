#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("Entity already exists")]
    Exists(#[source] anyhow::Error),

    #[error("Entity not found")]
    NotFound,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
