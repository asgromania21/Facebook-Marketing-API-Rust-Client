use thiserror::Error;

#[derive(Error, Debug)]
pub enum FacebookError {
    #[error("HTTP request failed with status: {0}")]
    HttpError(String),

    #[error("Unauthorized access")]
    Unauthorized,

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}