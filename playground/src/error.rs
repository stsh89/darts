#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Failed precondition: {0}")]
    FailedPrecondition(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Repo: {0}")]
    Repo(#[source] eyre::Report),

    #[error("Unexpected: {0}")]
    Unexpected(#[from] eyre::Report),
}
