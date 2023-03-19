#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String), // TODO: Remove for release

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Teloxide error: {0}")]
    Teloxide(#[from] teloxide::RequestError),

    #[error("RocksDB error: {0}")]
    RocksDB(#[from] rocksdb::Error),

    #[error("Teloxide handler error: {0}")]
    TeloxideHandler(#[from] Box<dyn std::error::Error + Send + Sync>),
}
