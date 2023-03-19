pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Common aliases
pub use std::format as f;
