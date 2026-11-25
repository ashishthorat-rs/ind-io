//! The [`LightControl`] capability: setting a brightness.
//!
//! The trait is re-exported as [`crate::traits::LightControl`].

use crate::error::DeviceError;
use crate::traits::Sealed;

/// The capability to set the brightness of a light and to read it back.
///
/// The brightness is a percentage from 0 to 100. A position, set through
/// [`PositionControl`], uses the same range.
///
/// A brightness is a level of this capability, not a power state. Under the
/// contract, a brightness of 0 does not turn the power off, setting a
/// brightness does not turn the power on, and turning the power off keeps the
/// brightness that was set. A device can be on at brightness 0; see
/// [independent capabilities].
///
/// The trait is sealed, so only types in this crate implement it.
/// [`MockLight`] and [`Light`] do, and any code can use the trait as a generic
/// bound or as a trait object for them. See [sealed traits].
///
/// # Examples
///
/// Code that is generic over the capability accepts any type that implements
/// the trait:
///
/// ```
/// use ind_io::mock::{MockLight, MockPower};
/// use ind_io::traits::LightControl;
/// use ind_io::{DeviceError, Light};
///
/// fn dim<D: LightControl>(device: &mut D) -> Result<(), DeviceError> {
///     device.set_brightness(20)
/// }
///
/// let mut light = Light::new(MockPower::new(), MockLight::new());
///
/// dim(&mut light)?;
/// assert_eq!(light.brightness(), 20);
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// [`PositionControl`]: crate::traits::PositionControl
/// [`MockLight`]: crate::mock::MockLight
/// [`Light`]: crate::Light
/// [independent capabilities]: crate::traits#independent-capabilities
/// [sealed traits]: crate::traits#sealed-traits
pub trait LightControl: Sealed {
    /// Sets the brightness to `percent`, a value from 0 to 100.
    ///
    /// Only the brightness changes. The power state does not. The mock backend
    /// accepts a brightness while the device is off, and the device stays off.
    /// A backend may reject a brightness that its hardware cannot accept in
    /// the current state.
    ///
    /// # Errors
    ///
    /// Returns [`DeviceError::InvalidInput`] if `percent` is above 100. The
    /// brightness is left unchanged. The mock backend enforces this limit. The
    /// limit has not been checked against a real backend.
    ///
    /// A backend that cannot apply a valid brightness in the current state
    /// returns a [`DeviceError`]. The crate does not specify which variant,
    /// and no type in this crate does so.
    ///
    /// # Examples
    ///
    /// The mock backend accepts a brightness while the light is off, and the
    /// light stays off. A brightness of 0 is a level, not a power state:
    ///
    /// ```
    /// use ind_io::mock::{MockLight, MockPower};
    /// use ind_io::traits::{LightControl, PowerControl};
    /// use ind_io::Light;
    ///
    /// let power = MockPower::with_state(false);
    /// let mut light = Light::new(power, MockLight::new());
    ///
    /// light.set_brightness(50)?;
    /// assert!(!light.is_on());
    ///
    /// light.turn_on()?;
    /// assert_eq!(light.brightness(), 50);
    ///
    /// light.set_brightness(0)?;
    /// assert!(light.is_on());
    /// assert_eq!(light.brightness(), 0);
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// The mock backend accepts every value from 0 to 100 and rejects every
    /// value above 100. A rejected value leaves the brightness and the power
    /// state as they were:
    ///
    /// ```
    /// use ind_io::mock::{MockLight, MockPower};
    /// use ind_io::traits::{LightControl, PowerControl};
    /// use ind_io::{DeviceError, Light};
    ///
    /// let mut light = Light::new(MockPower::new(), MockLight::new());
    /// light.turn_on()?;
    ///
    /// for percent in 0..=100 {
    ///     light.set_brightness(percent)?;
    ///     assert_eq!(light.brightness(), percent);
    /// }
    ///
    /// light.set_brightness(40)?;
    /// for percent in 101..=u8::MAX {
    ///     let result = light.set_brightness(percent);
    ///     assert_eq!(result, Err(DeviceError::InvalidInput));
    ///     assert_eq!(light.brightness(), 40);
    ///     assert!(light.is_on());
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`DeviceError::InvalidInput`]: crate::DeviceError::InvalidInput
    fn set_brightness(&mut self, percent: u8) -> Result<(), DeviceError>;

    /// Returns the brightness, a value from 0 to 100.
    ///
    /// The value is the brightness that the backend holds. The call cannot
    /// fail. It does not promise a fresh reading from hardware, so the value
    /// may not show a change made outside the library.
    fn brightness(&self) -> u8;
}
