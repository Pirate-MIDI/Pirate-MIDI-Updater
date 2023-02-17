use serde::Serialize;

pub mod dfu;
pub mod github;
pub mod install;

// error object
#[derive(thiserror::Error, Debug, Clone, PartialEq, Serialize)]
pub enum CommandError {
    #[error("unable to retrieve file: {0:?}")]
    IO(String),
    #[error("unable to perform install: {0:?}")]
    Dfu(String),
    #[error("unable to send command to device: {0:?}")]
    Device(String),
    #[error("unable to fetch releases: {0:?}")]
    Retieval(String),
    #[error("Failed to make a request: {0:?}")]
    Http(String),
}
