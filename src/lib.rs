//! FT-991 Rust Driver
//!
//! A modern, type-safe Rust library for controlling the Yaesu FT-991 transceiver
//! via the CAT (Computer-Aided Transceiver) protocol over serial connection.
//!
//! # Examples
//!
//! ```no_run
//! use ft991_rs::{FT991, SerialConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = SerialConfig {
//!         port: "/dev/ttyUSB0".to_string(),
//!         baud_rate: 38400,
//!     };
//!
//!     let mut rig = FT991::new(config)?;
//!     let freq = rig.get_frequency().await?;
//!     println!("Current frequency: {} Hz", freq);
//!
//!     Ok(())
//! }
//! ```

pub mod cat;
pub mod commands;
pub mod error;
pub mod rig;
pub mod types;

pub use error::{Error, Result};
pub use rig::{FT991, SerialConfig};
pub use types::*;
