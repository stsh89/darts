#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed precondition: {0}")]
    FailedPrecondition(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Repo: {0}")]
    Repo(#[source] eyre::Report),

    #[error("Unexpected: {0}")]
    Unexpected(#[from] eyre::Report),
}
