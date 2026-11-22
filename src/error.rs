//! The error type shared by the capability traits and the devices.
//!
//! [`DeviceError`] is the only error type in the crate. It is re-exported at
//! the crate root, so `ind_io::DeviceError` and `ind_io::error::DeviceError`
//! name the same type.

use core::fmt;

/// The error returned when a device operation fails.
///
/// Every fallible method of the capability traits, and of the devices that
/// implement them, returns `Result<(), DeviceError>`. A device returns the
/// error of its backend unchanged. The type is a plain enum: no variant
/// carries data, and the type is [`Copy`].
///
/// # Matching on an error
///
/// The enum is `#[non_exhaustive]`, so a `match` in another crate needs a
/// wildcard arm. Match on the variant, not on the text of the [`Display`]
/// output. That text is meant for people and logs, and it may be reworded.
/// The `Display` output honors the width and the alignment of the format
/// string, as in `{:>24}`.
///
/// # Provisional variants
///
/// `HardwareFailure`, `NotSupported` and `Timeout` are provisional. No type in
/// this crate returns them, and their meaning has not been validated against
/// a real backend.
///
/// # The `Error` trait
///
/// `DeviceError` implements [`Debug`] and [`Display`], but it does not
/// implement the `Error` trait. For that reason `?` cannot convert it into a
/// `Box<dyn Error>`. This crate is `no_std`, so the trait would be
/// `core::error::Error`, which needs Rust 1.81. That is above the minimum
/// supported Rust version of this crate, 1.75.0.
///
/// # Examples
///
/// Reject an out-of-range value, and react to the error by its variant:
///
/// ```
/// use ind_io::mock::{MockLight, MockPower};
/// use ind_io::traits::LightControl;
/// use ind_io::{DeviceError, Light};
///
/// let mut light = Light::new(MockPower::new(), MockLight::new());
///
/// let result = light.set_brightness(101);
/// assert_eq!(result, Err(DeviceError::InvalidInput));
///
/// let advice = match result {
///     Err(DeviceError::InvalidInput) => "use a brightness from 0 to 100",
///     // The enum is `#[non_exhaustive]`, so another crate needs this arm.
///     Err(_) => "the device reported an error",
///     Ok(()) => "brightness set",
/// };
/// assert_eq!(advice, "use a brightness from 0 to 100");
/// ```
///
/// Format an error with a width. The flag `>` puts the text at the right end
/// and the flag `<` puts it at the left end:
///
/// ```
/// use ind_io::DeviceError;
///
/// let plain = DeviceError::Timeout.to_string();
///
/// let right = format!("{:>24}", DeviceError::Timeout);
/// assert_eq!(right.len(), 24);
/// assert!(right.ends_with(&plain));
///
/// let left = format!("{:<24}", DeviceError::Timeout);
/// assert_eq!(left.len(), 24);
/// assert!(left.starts_with(&plain));
/// ```
///
/// [`Debug`]: core::fmt::Debug
/// [`Display`]: core::fmt::Display
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    /// An underlying hardware failure prevented an operation from completing.
    ///
    /// This variant is provisional. No type in this crate returns it, and its
    /// meaning has not been validated against a real backend.
    HardwareFailure,
    /// A value is outside the range that the operation accepts.
    ///
    /// The contract rejects:
    ///
    /// * a brightness above 100 ([`set_brightness`]);
    /// * a position above 100 ([`set_position`]);
    /// * a volume above 100 ([`set_volume`]);
    /// * channel 0, because channels start at 1 ([`set_channel`]);
    /// * a temperature that is not finite, which means NaN or infinity
    ///   ([`set_temperature`]). Any finite temperature is accepted.
    ///
    /// A rejected operation leaves the state unchanged. A step at a limit,
    /// such as `volume_down` at 0, is not an error.
    ///
    /// The mock backends in this crate enforce these rules. They have not been
    /// checked against a real backend.
    ///
    /// [`set_brightness`]: crate::traits::LightControl::set_brightness
    /// [`set_position`]: crate::traits::PositionControl::set_position
    /// [`set_volume`]: crate::traits::MediaControl::set_volume
    /// [`set_channel`]: crate::traits::MediaControl::set_channel
    /// [`set_temperature`]: crate::traits::ClimateControl::set_temperature
    InvalidInput,
    /// A backend does not offer the requested operation, such as swing on a
    /// climate backend that has none.
    ///
    /// This variant is provisional. No type in this crate returns it, and its
    /// meaning has not been validated against a real backend.
    NotSupported,
    /// An operation did not finish in time.
    ///
    /// This variant is provisional. No type in this crate returns it, and its
    /// meaning has not been validated against a real backend.
    Timeout,
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            DeviceError::HardwareFailure => "hardware failure",
            DeviceError::InvalidInput => "invalid input",
            DeviceError::NotSupported => "not supported",
            DeviceError::Timeout => "timed out",
        };
        f.pad(message)
    }
}
