//! Error types for FT-991 driver

use thiserror::Error;

/// Result type for FT-991 operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for FT-991 driver operations
#[derive(Error, Debug)]
pub enum Error {
    #[error("Serial port error: {0}")]
    SerialError(String),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Command timeout")]
    Timeout,

    #[error("Invalid response from rig")]
    InvalidResponse,

    #[error("Invalid frequency: {0}")]
    InvalidFrequency(u32),

    #[error("Invalid mode: {0}")]
    InvalidMode(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Rig not connected")]
    NotConnected,

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Unsupported operation")]
    Unsupported,
}
