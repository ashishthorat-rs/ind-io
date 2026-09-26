#![no_std]

// modules
pub mod error;
pub mod types;
pub mod traits;
pub mod devices;
pub mod mock;

// re-exports
pub use error::DeviceError;
pub use types::{Speed, ClimateMode, SwingMode};
pub use devices::{
    Switch, Fan, Light, Television,
    AirConditioner, Thermostat, AirPurifier, Heater, Curtain,
};
