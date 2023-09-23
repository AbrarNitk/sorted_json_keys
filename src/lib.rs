pub mod filter;
pub mod sort;

#[cfg(test)]
pub mod test;

use thiserror::Error;

type MyResult<T> = Result<T, ErrorKind>;
type JsonValue = serde_json::Value;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("ValueError")]
    ValueError,

    #[error("SerdeJsonError : {0}")]
    SerdeError(#[from] serde_json::Error),
}
