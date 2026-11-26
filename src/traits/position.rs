//! The [`PositionControl`] capability: setting a position.
//!
//! The trait is re-exported as [`crate::traits::PositionControl`].

use crate::error::DeviceError;
use crate::traits::Sealed;
use crate::types::MAX_PERCENT;

/// The capability to set a position and to read it back.
///
/// A position is a percentage from 0 to 100, where 0 is closed and 100 is
/// open. The brightness of a light, set through [`LightControl`], uses the
/// same range.
///
/// A position is a level of this capability, not a power state. Under the
/// contract, a position of 0 does not turn the power off, setting a position
/// does not turn the power on, and turning the power off keeps the position
/// that was set. A device can be on at position 0; see
/// [independent capabilities].
///
/// The trait is sealed, so only types in this crate implement it.
/// [`MockPosition`] and [`Curtain`] do, and any code can use the trait as a
/// generic bound or as a trait object for them. See [sealed traits].
///
/// # Default methods
///
/// [`open`], [`close`], [`is_open`] and [`is_closed`] are default methods.
/// [`open`] and [`close`] are built from [`set_position`], and [`is_open`] and
/// [`is_closed`] from [`position`]. No type in this crate replaces any of
/// them. A device runs them on its own `set_position` and `position`, which
/// pass each call on to the backend. A device does not call an `open`, a
/// `close`, an `is_open` or an `is_closed` of its backend. None of the four
/// has a `Self: Sized` clause, so all four can be called through a trait
/// object such as `&mut dyn PositionControl`.
///
/// # Examples
///
/// Code that is generic over the capability accepts any type that implements
/// the trait:
///
/// ```
/// use ind_io::mock::{MockPosition, MockPower};
/// use ind_io::traits::PositionControl;
/// use ind_io::{Curtain, DeviceError};
///
/// fn halfway<D: PositionControl>(device: &mut D) -> Result<(), DeviceError> {
///     device.set_position(50)
/// }
///
/// let mut curtain = Curtain::new(MockPower::new(), MockPosition::new());
///
/// halfway(&mut curtain)?;
/// assert_eq!(curtain.position(), 50);
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// [`open`]: PositionControl::open
/// [`close`]: PositionControl::close
/// [`is_open`]: PositionControl::is_open
/// [`is_closed`]: PositionControl::is_closed
/// [`set_position`]: PositionControl::set_position
/// [`position`]: PositionControl::position
/// [`LightControl`]: crate::traits::LightControl
/// [`MockPosition`]: crate::mock::MockPosition
/// [`Curtain`]: crate::Curtain
/// [independent capabilities]: crate::traits#independent-capabilities
/// [sealed traits]: crate::traits#sealed-traits
pub trait PositionControl: Sealed {
    /// Sets the position to `percent`, a value from 0 to 100.
    ///
    /// Only the position changes. The power state does not. The mock backend
    /// accepts a position while the device is off, and the device stays off.
    /// A backend may reject a position that its hardware cannot accept in the
    /// current state.
    ///
    /// # Errors
    ///
    /// Returns [`DeviceError::InvalidInput`] if `percent` is above 100. The
    /// position is left unchanged. The mock backend enforces this limit. The
    /// limit has not been checked against a real backend.
    ///
    /// A backend that cannot apply a valid position in the current state
    /// returns a [`DeviceError`]. The crate does not specify which variant,
    /// and no type in this crate does so.
    ///
    /// # Examples
    ///
    /// The mock backend accepts a position while the device is off, and the
    /// device stays off. A position of 0 is a level, not a power state:
    ///
    /// ```
    /// use ind_io::mock::{MockPosition, MockPower};
    /// use ind_io::traits::{PositionControl, PowerControl};
    /// use ind_io::Curtain;
    ///
    /// let power = MockPower::with_state(false);
    /// let mut curtain = Curtain::new(power, MockPosition::new());
    ///
    /// curtain.set_position(40)?;
    /// assert!(!curtain.is_on());
    ///
    /// curtain.turn_on()?;
    /// assert_eq!(curtain.position(), 40);
    ///
    /// curtain.set_position(0)?;
    /// assert!(curtain.is_on());
    /// assert_eq!(curtain.position(), 0);
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// The mock backend accepts every value from 0 to 100 and rejects every
    /// value above 100. A rejected value leaves the position and the power
    /// state as they were:
    ///
    /// ```
    /// use ind_io::mock::{MockPosition, MockPower};
    /// use ind_io::traits::{PositionControl, PowerControl};
    /// use ind_io::{Curtain, DeviceError};
    ///
    /// let mut curtain = Curtain::new(MockPower::new(), MockPosition::new());
    /// curtain.turn_on()?;
    ///
    /// for percent in 0..=100 {
    ///     curtain.set_position(percent)?;
    ///     assert_eq!(curtain.position(), percent);
    /// }
    ///
    /// curtain.set_position(40)?;
    /// for percent in 101..=u8::MAX {
    ///     let result = curtain.set_position(percent);
    ///     assert_eq!(result, Err(DeviceError::InvalidInput));
    ///     assert_eq!(curtain.position(), 40);
    ///     assert!(curtain.is_on());
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`DeviceError::InvalidInput`]: crate::DeviceError::InvalidInput
    fn set_position(&mut self, percent: u8) -> Result<(), DeviceError>;

    /// Returns the position, a value from 0 to 100.
    ///
    /// The value is the position that the backend holds. It is a configured
    /// value, not a measurement. The call cannot fail. It does not promise a
    /// fresh reading from hardware, so the value may not show a change made
    /// outside the library.
    fn position(&self) -> u8;

    /// Opens fully by setting the position to 100.
    ///
    /// This is [`set_position`] with 100, so the same rules and errors apply.
    /// Only the position changes. The power state does not.
    ///
    /// # Errors
    ///
    /// Returns the error of [`set_position`].
    ///
    /// # Examples
    ///
    /// With the mock backend, `open` sets the position to 100 with the power
    /// off and with the power on, and the power state does not change:
    ///
    /// ```
    /// use ind_io::mock::{MockPosition, MockPower};
    /// use ind_io::traits::{PositionControl, PowerControl};
    /// use ind_io::Curtain;
    ///
    /// for on in [false, true] {
    ///     let power = MockPower::with_state(on);
    ///     let mut curtain = Curtain::new(power, MockPosition::new());
    ///     curtain.set_position(40)?;
    ///
    ///     curtain.open()?;
    ///     assert_eq!(curtain.position(), 100);
    ///     assert!(curtain.is_open());
    ///     assert_eq!(curtain.is_on(), on);
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`set_position`]: PositionControl::set_position
    fn open(&mut self) -> Result<(), DeviceError> {
        self.set_position(MAX_PERCENT)
    }

    /// Closes fully by setting the position to 0.
    ///
    /// This is [`set_position`] with 0, so the same rules and errors apply.
    /// Only the position changes. Closing does not turn the power off.
    ///
    /// # Errors
    ///
    /// Returns the error of [`set_position`].
    ///
    /// # Examples
    ///
    /// With the mock backend, `close` sets the position to 0 with the power
    /// off and with the power on, and the power state does not change:
    ///
    /// ```
    /// use ind_io::mock::{MockPosition, MockPower};
    /// use ind_io::traits::{PositionControl, PowerControl};
    /// use ind_io::Curtain;
    ///
    /// for on in [false, true] {
    ///     let power = MockPower::with_state(on);
    ///     let mut curtain = Curtain::new(power, MockPosition::new());
    ///     curtain.set_position(60)?;
    ///
    ///     curtain.close()?;
    ///     assert_eq!(curtain.position(), 0);
    ///     assert!(curtain.is_closed());
    ///     assert_eq!(curtain.is_on(), on);
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`set_position`]: PositionControl::set_position
    fn close(&mut self) -> Result<(), DeviceError> {
        self.set_position(0)
    }

    /// Returns `true` if the position is open, that is, 100.
    ///
    /// A position between 0 and 100 is neither open nor closed.
    ///
    /// # Examples
    ///
    /// With the mock backend, `is_open` is `true` at 100 and at no other
    /// position. A position between 0 and 100 is neither open nor closed:
    ///
    /// ```
    /// use ind_io::mock::{MockPosition, MockPower};
    /// use ind_io::traits::PositionControl;
    /// use ind_io::Curtain;
    ///
    /// let mut curtain = Curtain::new(MockPower::new(), MockPosition::new());
    ///
    /// for percent in 0..=100 {
    ///     curtain.set_position(percent)?;
    ///     let state = (curtain.is_open(), curtain.is_closed());
    ///     assert_eq!(state, (percent == 100, percent == 0));
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    fn is_open(&self) -> bool {
        self.position() == MAX_PERCENT
    }

    /// Returns `true` if the position is closed, that is, 0.
    ///
    /// A position between 0 and 100 is neither open nor closed.
    ///
    /// # Examples
    ///
    /// With the mock backend, `is_closed` is `true` at 0 and at no other
    /// position. A position between 0 and 100 is neither open nor closed:
    ///
    /// ```
    /// use ind_io::mock::{MockPosition, MockPower};
    /// use ind_io::traits::PositionControl;
    /// use ind_io::Curtain;
    ///
    /// let mut curtain = Curtain::new(MockPower::new(), MockPosition::new());
    ///
    /// for percent in 0..=100 {
    ///     curtain.set_position(percent)?;
    ///     let state = (curtain.is_closed(), curtain.is_open());
    ///     assert_eq!(state, (percent == 0, percent == 100));
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    fn is_closed(&self) -> bool {
        self.position() == 0
    }
}
