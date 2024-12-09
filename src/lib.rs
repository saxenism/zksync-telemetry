// lib.rs
pub mod config;
pub mod error;
pub mod telemetry;
mod utils;

pub use config::TelemetryConfig;
pub use error::{TelemetryError, TelemetryResult};
pub use telemetry::Telemetry;