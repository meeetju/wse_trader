use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Generic(String),
    #[error("I/O error: {0}")]
    File(#[from] std::io::Error),
}
