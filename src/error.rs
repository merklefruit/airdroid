use std::convert::Infallible;
use teloxide::dispatching::dialogue::SqliteStorageError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String), // TODO: Remove for release

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Teloxide error: {0}")]
    Teloxide(#[from] teloxide::RequestError),

    #[error("Sqlite error: {0}")]
    Sqlite(#[from] SqliteStorageError<Infallible>),

    #[error("Teloxide handler error: {0}")]
    TeloxideHandler(#[from] Box<dyn std::error::Error + Send + Sync>),
}
