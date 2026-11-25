//! The [`SpeedControl`] capability: setting a [`Speed`].
//!
//! The trait is re-exported as [`crate::traits::SpeedControl`].
//!
//! [`Speed`]: crate::Speed

use crate::error::DeviceError;
use crate::traits::Sealed;
use crate::types::Speed;

/// The capability to set a discrete [`Speed`] and to read it back.
///
/// A speed is a level of this capability, not a power state. Under the
/// contract, setting a speed does not change the power state, and turning the
/// power off keeps the speed that was set; see [independent capabilities].
/// [`Speed::Off`] is the lowest named level, and a device can be on while its
/// speed is `Off`.
///
/// The trait is sealed, so only types in this crate implement it.
/// [`MockSpeed`], [`Fan`], [`AirPurifier`] and [`AirConditioner`] do, and any
/// code can use the trait as a generic bound or as a trait object for them.
/// See [sealed traits].
///
/// # Examples
///
/// Code that is generic over the capability accepts any type that implements
/// the trait:
///
/// ```
/// use ind_io::mock::{MockPower, MockSpeed};
/// use ind_io::traits::SpeedControl;
/// use ind_io::{DeviceError, Fan, Speed};
///
/// fn set_high<D: SpeedControl>(device: &mut D) -> Result<(), DeviceError> {
///     device.set_speed(Speed::High)
/// }
///
/// let mut fan = Fan::new(MockPower::new(), MockSpeed::new());
///
/// set_high(&mut fan)?;
/// assert_eq!(fan.speed(), Speed::High);
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// [`Speed`]: crate::Speed
/// [`Speed::Off`]: crate::Speed::Off
/// [`MockSpeed`]: crate::mock::MockSpeed
/// [`Fan`]: crate::Fan
/// [`AirPurifier`]: crate::AirPurifier
/// [`AirConditioner`]: crate::AirConditioner
/// [independent capabilities]: crate::traits#independent-capabilities
/// [sealed traits]: crate::traits#sealed-traits
pub trait SpeedControl: Sealed {
    /// Sets the speed.
    ///
    /// Only the speed changes. The power state does not. The mock backend
    /// accepts a speed while the device is off, and the device stays off. A
    /// backend may reject a speed that its hardware cannot accept in the
    /// current state.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the backend cannot apply the speed. The
    /// crate does not specify which variant. The mock backend accepts every
    /// [`Speed`], and no type in this crate returns an error from this method.
    ///
    /// # Examples
    ///
    /// The mock backend accepts every speed with the power off and with the
    /// power on, and the power state does not change:
    ///
    /// ```
    /// use ind_io::mock::{MockPower, MockSpeed};
    /// use ind_io::traits::{PowerControl, SpeedControl};
    /// use ind_io::{Fan, Speed};
    ///
    /// let speeds = [
    ///     Speed::Off,
    ///     Speed::Low,
    ///     Speed::Medium,
    ///     Speed::High,
    ///     Speed::Turbo,
    /// ];
    ///
    /// for on in [false, true] {
    ///     let mut fan = Fan::new(MockPower::with_state(on), MockSpeed::new());
    ///
    ///     for speed in speeds {
    ///         fan.set_speed(speed)?;
    ///         assert_eq!(fan.speed(), speed);
    ///         assert_eq!(fan.is_on(), on);
    ///     }
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`Speed`]: crate::Speed
    fn set_speed(&mut self, speed: Speed) -> Result<(), DeviceError>;

    /// Returns the speed.
    ///
    /// The value is the speed that the backend holds. The call cannot fail. It
    /// does not promise a fresh reading from hardware, so the value may not
    /// show a change made outside the library.
    fn speed(&self) -> Speed;
}
