pub mod config;
pub mod deploy;
pub mod error;

pub type Result<T> = std::result::Result<T, crate::Error>;
pub use error::Error;
