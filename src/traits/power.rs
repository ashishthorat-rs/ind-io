//! The [`PowerControl`] capability: switching power on and off.
//!
//! The trait is re-exported as [`crate::traits::PowerControl`].

use crate::error::DeviceError;
use crate::traits::Sealed;

/// The capability to switch power on and off and to read the power state.
///
/// The power state is either on or off. It changes only through this trait,
/// by [`turn_on`], [`turn_off`], [`toggle`] and [`reset`]. A setting of
/// another capability, such as a speed or a brightness, leaves it alone; see
/// [independent capabilities].
///
/// `Speed::Off`, a brightness of 0, a position of 0 and a volume of 0 are not
/// power states. A device can be on at any of them.
///
/// Under the contract, a power change keeps the values that were set for the
/// other capabilities. If it cannot, the result is a [`DeviceError`], not
/// success. The mock backends keep the values. Whether hardware can keep them
/// has not been validated.
///
/// The trait is sealed, so only types in this crate implement it. The devices
/// and [`MockPower`] do, and any code can use the trait as a generic bound or
/// as a trait object for them. See [sealed traits].
///
/// # Default methods
///
/// [`toggle`] and [`reset`] are default methods. [`toggle`] is built from
/// [`is_on`], [`turn_on`] and [`turn_off`], and [`reset`] from [`turn_off`]
/// and [`turn_on`]. No type in this crate replaces either method. A device
/// runs them on its own `is_on`, `turn_on` and `turn_off`, which pass each
/// call on to the backend. A device does not call a `toggle` or a `reset` of
/// its backend. Neither method has a `Self: Sized` clause, so both can be
/// called through a trait object such as `&mut dyn PowerControl`.
///
/// # Examples
///
/// Code that is generic over the capability accepts any type that implements
/// the trait:
///
/// ```
/// use ind_io::mock::MockPower;
/// use ind_io::traits::PowerControl;
/// use ind_io::{DeviceError, Switch};
///
/// fn switch_on<D: PowerControl>(device: &mut D) -> Result<(), DeviceError> {
///     device.turn_on()
/// }
///
/// let mut switch = Switch::new(MockPower::with_state(false));
/// assert!(!switch.is_on());
///
/// switch_on(&mut switch)?;
/// assert!(switch.is_on());
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// [`turn_on`]: PowerControl::turn_on
/// [`turn_off`]: PowerControl::turn_off
/// [`is_on`]: PowerControl::is_on
/// [`toggle`]: PowerControl::toggle
/// [`reset`]: PowerControl::reset
/// [`DeviceError`]: crate::DeviceError
/// [`MockPower`]: crate::mock::MockPower
/// [independent capabilities]: crate::traits#independent-capabilities
/// [sealed traits]: crate::traits#sealed-traits
pub trait PowerControl: Sealed {
    /// Turns the power on.
    ///
    /// With [`MockPower`], turning the power on when it is already on
    /// succeeds and leaves it on.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the power cannot be turned on. The crate
    /// does not specify which variant. No type in this crate returns an error
    /// from this method.
    ///
    /// # Examples
    ///
    /// Turning the power on twice with [`MockPower`]:
    ///
    /// ```
    /// use ind_io::mock::MockPower;
    /// use ind_io::traits::PowerControl;
    ///
    /// let mut power = MockPower::new();
    ///
    /// power.turn_on()?;
    /// power.turn_on()?;
    /// assert!(power.is_on());
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`MockPower`]: crate::mock::MockPower
    fn turn_on(&mut self) -> Result<(), DeviceError>;

    /// Turns the power off.
    ///
    /// With [`MockPower`], turning the power off when it is already off
    /// succeeds and leaves it off.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the power cannot be turned off. The crate
    /// does not specify which variant. No type in this crate returns an error
    /// from this method.
    ///
    /// # Examples
    ///
    /// Turning the power off twice with [`MockPower`]:
    ///
    /// ```
    /// use ind_io::mock::MockPower;
    /// use ind_io::traits::PowerControl;
    ///
    /// let mut power = MockPower::with_state(true);
    ///
    /// power.turn_off()?;
    /// power.turn_off()?;
    /// assert!(!power.is_on());
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`MockPower`]: crate::mock::MockPower
    fn turn_off(&mut self) -> Result<(), DeviceError>;

    /// Returns `true` if the power is on.
    ///
    /// The value is the power state that the backend holds. The call cannot
    /// fail. It does not promise a fresh reading from hardware, so the value
    /// may not show a change made outside the library.
    fn is_on(&self) -> bool;

    /// Toggles the power: turns it off if it is on, and on if it is off.
    ///
    /// # Errors
    ///
    /// Returns the error of [`turn_off`] or of [`turn_on`], whichever of the
    /// two the call makes.
    ///
    /// # Examples
    ///
    /// Toggling [`MockPower`] from each starting state:
    ///
    /// ```
    /// use ind_io::mock::MockPower;
    /// use ind_io::traits::PowerControl;
    ///
    /// for start in [false, true] {
    ///     let mut power = MockPower::with_state(start);
    ///
    ///     power.toggle()?;
    ///     assert_eq!(power.is_on(), !start);
    ///
    ///     power.toggle()?;
    ///     assert_eq!(power.is_on(), start);
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`turn_on`]: PowerControl::turn_on
    /// [`turn_off`]: PowerControl::turn_off
    /// [`MockPower`]: crate::mock::MockPower
    fn toggle(&mut self) -> Result<(), DeviceError> {
        if self.is_on() {
            self.turn_off()
        } else {
            self.turn_on()
        }
    }

    /// Resets the power by turning it off and then on again.
    ///
    /// This is a power cycle, not a factory reset: no setting is reset to a
    /// default. A successful `reset` leaves the power on, whatever the power
    /// state was before. As for any other power change, the values that were
    /// set for the other capabilities are kept; see [independent
    /// capabilities].
    ///
    /// # Errors
    ///
    /// Returns the error of [`turn_off`] if that call fails, and then does not
    /// call [`turn_on`]. Otherwise returns the result of [`turn_on`]. If
    /// [`turn_on`] fails, `reset` does not undo the [`turn_off`] that came
    /// before it. No type in this crate fails a power operation, so no example
    /// here shows either case.
    ///
    /// # Examples
    ///
    /// With the mock backends, resetting a light from each starting state
    /// leaves it on and keeps the brightness that was set:
    ///
    /// ```
    /// use ind_io::mock::{MockLight, MockPower};
    /// use ind_io::traits::{LightControl, PowerControl};
    /// use ind_io::Light;
    ///
    /// for start in [false, true] {
    ///     let power = MockPower::with_state(start);
    ///     let mut light = Light::new(power, MockLight::new());
    ///     light.set_brightness(70)?;
    ///
    ///     light.reset()?;
    ///     assert!(light.is_on());
    ///     assert_eq!(light.brightness(), 70);
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`turn_on`]: PowerControl::turn_on
    /// [`turn_off`]: PowerControl::turn_off
    /// [independent capabilities]: crate::traits#independent-capabilities
    fn reset(&mut self) -> Result<(), DeviceError> {
        self.turn_off()?;
        self.turn_on()
    }
}
