#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("Entity already exists")]
    Exists(#[source] anyhow::Error),

    #[error("Entity not found")]
    NotFound,

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
