#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String), // TODO: Remove for release

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
