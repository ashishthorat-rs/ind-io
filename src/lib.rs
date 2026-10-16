#![no_std]
#![forbid(unsafe_code)]

// modules
pub mod devices;
pub mod error;
pub mod mock;
pub mod traits;
pub mod types;

pub use devices::{AirConditioner, AirPurifier, Curtain};
pub use error::DeviceError;
pub use types::{ClimateMode, Speed, SwingMode};
