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

    #[error("Unexpected: {0}")]
    Unexpected(#[from] eyre::Report),
}

impl<T> From<Error> for Result<T, Error> {
    fn from(value: Error) -> Self {
        Err(value)
    }
}

impl Error {
    pub(crate) fn failed_precondition(description: impl ToString) -> Self {
        Self::FailedPrecondition(description.to_string())
    }

    pub(crate) fn invalid_argument(description: impl ToString) -> Self {
        Self::InvalidArgument(description.to_string())
    }

    pub(crate) fn unexpected(description: impl ToString) -> Self {
        Self::Unexpected(eyre::Report::msg(description.to_string()))
    }
}
