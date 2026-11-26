//! The [`ClimateControl`] capability: setting a temperature, a mode and a
//! swing.
//!
//! The trait is re-exported as [`crate::traits::ClimateControl`].

use crate::error::DeviceError;
use crate::traits::Sealed;
use crate::types::{ClimateMode, SwingMode};

/// The capability to set a target temperature, a climate mode and a swing
/// mode, and to read each of them back.
///
/// The temperature is a target in degrees Celsius, given as a finite `f32`. It
/// is the configured target and not a measurement: [`temperature`] returns the
/// target that was set. This crate does not read sensors and has no control
/// strategy. Code that reacts to a room temperature, such as the controller in
/// the `advanced_thermostat` example, is application code, and the application
/// supplies that temperature. The example uses a simulated room temperature.
///
/// The crate attaches no behavior to a [`ClimateMode`] or a [`SwingMode`]. A
/// backend decides what a mode or a swing does, and whether it offers it.
///
/// The temperature, the mode and the swing are levels of this capability, not
/// power states. Under the contract, setting any of them does not turn the
/// power on or off, and turning the power off keeps them; see
/// [independent capabilities].
///
/// The trait is sealed, so only types in this crate implement it.
/// [`MockClimate`], [`Heater`], [`Thermostat`] and [`AirConditioner`] do, and
/// any code can use the trait as a generic bound or as a trait object for
/// them. See [sealed traits].
///
/// # Examples
///
/// Code that is generic over the capability accepts any type that implements
/// the trait:
///
/// ```
/// use ind_io::mock::{MockClimate, MockPower};
/// use ind_io::traits::ClimateControl;
/// use ind_io::{DeviceError, Thermostat};
///
/// fn warm<D: ClimateControl>(device: &mut D) -> Result<(), DeviceError> {
///     device.set_temperature(24.0)
/// }
///
/// let mut thermostat = Thermostat::new(MockPower::new(), MockClimate::new());
///
/// warm(&mut thermostat)?;
/// assert_eq!(thermostat.temperature(), 24.0);
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// With the mock backends, a power change keeps the temperature, the mode and
/// the swing in each of the three devices, whether the power starts off or on:
///
/// ```
/// use ind_io::mock::{MockClimate, MockPower, MockSpeed};
/// use ind_io::traits::{ClimateControl, PowerControl};
/// use ind_io::{
///     AirConditioner, ClimateMode, DeviceError, Heater, SwingMode,
///     Thermostat,
/// };
///
/// fn check<D>(device: &mut D) -> Result<(), DeviceError>
/// where
///     D: ClimateControl + PowerControl,
/// {
///     device.set_temperature(24.0)?;
///     device.set_mode(ClimateMode::Cool)?;
///     device.set_swing(SwingMode::Both)?;
///
///     device.turn_off()?;
///     assert_eq!(device.temperature(), 24.0);
///     assert_eq!(device.mode(), ClimateMode::Cool);
///     assert_eq!(device.swing(), SwingMode::Both);
///
///     device.turn_on()?;
///     assert_eq!(device.temperature(), 24.0);
///     assert_eq!(device.mode(), ClimateMode::Cool);
///     assert_eq!(device.swing(), SwingMode::Both);
///     Ok(())
/// }
///
/// for on in [false, true] {
///     let power = || MockPower::with_state(on);
///     let mut heater = Heater::new(power(), MockClimate::new());
///     let mut thermostat = Thermostat::new(power(), MockClimate::new());
///     let mut ac = AirConditioner::new(
///         power(),
///         MockSpeed::new(),
///         MockClimate::new(),
///     );
///
///     check(&mut heater)?;
///     check(&mut thermostat)?;
///     check(&mut ac)?;
/// }
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// [`temperature`]: ClimateControl::temperature
/// [`ClimateMode`]: crate::ClimateMode
/// [`SwingMode`]: crate::SwingMode
/// [`MockClimate`]: crate::mock::MockClimate
/// [`Heater`]: crate::Heater
/// [`Thermostat`]: crate::Thermostat
/// [`AirConditioner`]: crate::AirConditioner
/// [independent capabilities]: crate::traits#independent-capabilities
/// [sealed traits]: crate::traits#sealed-traits
pub trait ClimateControl: Sealed {
    /// Sets the target temperature to `celsius`, a finite number of degrees
    /// Celsius.
    ///
    /// Only the temperature changes. The mode, the swing and the power state
    /// do not. The mock backend accepts any finite value, such as 7.0 or -5.5,
    /// while the device is off, and the device stays off.
    ///
    /// The crate sets no comfort range, so any finite value is valid under the
    /// contract. An application that needs a limit applies it before it calls
    /// this method. The `advanced_heater` example does so with an upper limit.
    /// That limit is illustrative application policy, not a recommended safety
    /// standard or a hardware safety guarantee.
    ///
    /// # Errors
    ///
    /// Returns [`DeviceError::InvalidInput`] if `celsius` is NaN or infinite.
    /// The temperature is left unchanged. The mock backend enforces this
    /// limit. The limit has not been checked against a real backend.
    ///
    /// A backend that cannot apply a finite temperature in the current state
    /// returns a [`DeviceError`]. The crate does not specify which variant,
    /// and no type in this crate does so.
    ///
    /// # Examples
    ///
    /// With the mock backend, any finite value is accepted with the power off
    /// and with the power on, and each of the three devices passes it on. The
    /// mode, the swing and the power state stay as they were:
    ///
    /// ```
    /// use ind_io::mock::{MockClimate, MockPower, MockSpeed};
    /// use ind_io::traits::{ClimateControl, PowerControl};
    /// use ind_io::{
    ///     AirConditioner, ClimateMode, DeviceError, Heater, SwingMode,
    ///     Thermostat,
    /// };
    ///
    /// fn check<D>(device: &mut D, on: bool) -> Result<(), DeviceError>
    /// where
    ///     D: ClimateControl + PowerControl,
    /// {
    ///     device.set_mode(ClimateMode::Cool)?;
    ///     device.set_swing(SwingMode::Both)?;
    ///
    ///     for celsius in [7.0, -5.5, 0.0, 22.0, f32::MAX, f32::MIN] {
    ///         device.set_temperature(celsius)?;
    ///         assert_eq!(device.temperature(), celsius);
    ///         assert_eq!(device.mode(), ClimateMode::Cool);
    ///         assert_eq!(device.swing(), SwingMode::Both);
    ///         assert_eq!(device.is_on(), on);
    ///     }
    ///     Ok(())
    /// }
    ///
    /// for on in [false, true] {
    ///     let power = || MockPower::with_state(on);
    ///     let mut heater = Heater::new(power(), MockClimate::new());
    ///     let mut thermostat = Thermostat::new(power(), MockClimate::new());
    ///     let mut ac = AirConditioner::new(
    ///         power(),
    ///         MockSpeed::new(),
    ///         MockClimate::new(),
    ///     );
    ///
    ///     check(&mut heater, on)?;
    ///     check(&mut thermostat, on)?;
    ///     check(&mut ac, on)?;
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// The mock backend rejects NaN and infinity with the power off and with
    /// the power on, and each of the three devices passes the rejection on. A
    /// rejected value leaves the temperature and the power state as they were:
    ///
    /// ```
    /// use ind_io::mock::{MockClimate, MockPower, MockSpeed};
    /// use ind_io::traits::{ClimateControl, PowerControl};
    /// use ind_io::{AirConditioner, DeviceError, Heater, Thermostat};
    ///
    /// fn check<D>(device: &mut D, on: bool) -> Result<(), DeviceError>
    /// where
    ///     D: ClimateControl + PowerControl,
    /// {
    ///     device.set_temperature(21.0)?;
    ///
    ///     for celsius in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
    ///         let result = device.set_temperature(celsius);
    ///         assert_eq!(result, Err(DeviceError::InvalidInput));
    ///         assert_eq!(device.temperature(), 21.0);
    ///         assert_eq!(device.is_on(), on);
    ///     }
    ///     Ok(())
    /// }
    ///
    /// for on in [false, true] {
    ///     let power = || MockPower::with_state(on);
    ///     let mut heater = Heater::new(power(), MockClimate::new());
    ///     let mut thermostat = Thermostat::new(power(), MockClimate::new());
    ///     let mut ac = AirConditioner::new(
    ///         power(),
    ///         MockSpeed::new(),
    ///         MockClimate::new(),
    ///     );
    ///
    ///     check(&mut heater, on)?;
    ///     check(&mut thermostat, on)?;
    ///     check(&mut ac, on)?;
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`DeviceError::InvalidInput`]: crate::DeviceError::InvalidInput
    fn set_temperature(&mut self, celsius: f32) -> Result<(), DeviceError>;

    /// Sets the climate mode to `mode`.
    ///
    /// Only the mode changes. The temperature, the swing and the power state
    /// do not. The crate attaches no behavior to a mode: a backend decides
    /// what a mode does. The devices that implement this trait pass every mode
    /// on to their backend. The mock backend accepts every mode while the
    /// device is off, and the device stays off.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the backend cannot apply the mode. The
    /// crate does not specify which variant. The mock backend accepts every
    /// [`ClimateMode`], and no type in this crate returns an error from this
    /// method.
    ///
    /// # Examples
    ///
    /// With the mock backend, every mode is accepted with the power off and
    /// with the power on, and each of the three devices passes it on. The
    /// temperature, the swing and the power state stay as they were:
    ///
    /// ```
    /// use ind_io::mock::{MockClimate, MockPower, MockSpeed};
    /// use ind_io::traits::{ClimateControl, PowerControl};
    /// use ind_io::{
    ///     AirConditioner, ClimateMode, DeviceError, Heater, SwingMode,
    ///     Thermostat,
    /// };
    ///
    /// const MODES: [ClimateMode; 5] = [
    ///     ClimateMode::Cool,
    ///     ClimateMode::Heat,
    ///     ClimateMode::Dry,
    ///     ClimateMode::Fan,
    ///     ClimateMode::Auto,
    /// ];
    ///
    /// fn check<D>(device: &mut D, on: bool) -> Result<(), DeviceError>
    /// where
    ///     D: ClimateControl + PowerControl,
    /// {
    ///     device.set_temperature(19.0)?;
    ///     device.set_swing(SwingMode::Both)?;
    ///
    ///     for mode in MODES {
    ///         device.set_mode(mode)?;
    ///         assert_eq!(device.mode(), mode);
    ///         assert_eq!(device.temperature(), 19.0);
    ///         assert_eq!(device.swing(), SwingMode::Both);
    ///         assert_eq!(device.is_on(), on);
    ///     }
    ///     Ok(())
    /// }
    ///
    /// for on in [false, true] {
    ///     let power = || MockPower::with_state(on);
    ///     let mut heater = Heater::new(power(), MockClimate::new());
    ///     let mut thermostat = Thermostat::new(power(), MockClimate::new());
    ///     let mut ac = AirConditioner::new(
    ///         power(),
    ///         MockSpeed::new(),
    ///         MockClimate::new(),
    ///     );
    ///
    ///     check(&mut heater, on)?;
    ///     check(&mut thermostat, on)?;
    ///     check(&mut ac, on)?;
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`ClimateMode`]: crate::ClimateMode
    fn set_mode(&mut self, mode: ClimateMode) -> Result<(), DeviceError>;

    /// Sets the swing mode to `swing`.
    ///
    /// Only the swing changes. The temperature, the mode and the power state
    /// do not. The crate attaches no behavior to a swing mode: a backend
    /// decides what a swing mode does and whether it offers swing at all. The
    /// devices that implement this trait pass every swing mode on to their
    /// backend. The mock backend accepts every swing mode while the device is
    /// off, and the device stays off.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the backend cannot apply the swing mode.
    /// If the backend does not offer swing, the variant for that case is
    /// [`DeviceError::NotSupported`], but that variant is provisional and no
    /// type in this crate returns it. For any other cause the crate does not
    /// specify the variant. The mock backend accepts every [`SwingMode`], and
    /// no type in this crate returns an error from this method.
    ///
    /// # Examples
    ///
    /// With the mock backend, every swing mode is accepted with the power off
    /// and with the power on, and each of the three devices passes it on. The
    /// temperature, the mode and the power state stay as they were:
    ///
    /// ```
    /// use ind_io::mock::{MockClimate, MockPower, MockSpeed};
    /// use ind_io::traits::{ClimateControl, PowerControl};
    /// use ind_io::{
    ///     AirConditioner, ClimateMode, DeviceError, Heater, SwingMode,
    ///     Thermostat,
    /// };
    ///
    /// const SWINGS: [SwingMode; 4] = [
    ///     SwingMode::Off,
    ///     SwingMode::Vertical,
    ///     SwingMode::Horizontal,
    ///     SwingMode::Both,
    /// ];
    ///
    /// fn check<D>(device: &mut D, on: bool) -> Result<(), DeviceError>
    /// where
    ///     D: ClimateControl + PowerControl,
    /// {
    ///     device.set_temperature(19.0)?;
    ///     device.set_mode(ClimateMode::Heat)?;
    ///
    ///     for swing in SWINGS {
    ///         device.set_swing(swing)?;
    ///         assert_eq!(device.swing(), swing);
    ///         assert_eq!(device.temperature(), 19.0);
    ///         assert_eq!(device.mode(), ClimateMode::Heat);
    ///         assert_eq!(device.is_on(), on);
    ///     }
    ///     Ok(())
    /// }
    ///
    /// for on in [false, true] {
    ///     let power = || MockPower::with_state(on);
    ///     let mut heater = Heater::new(power(), MockClimate::new());
    ///     let mut thermostat = Thermostat::new(power(), MockClimate::new());
    ///     let mut ac = AirConditioner::new(
    ///         power(),
    ///         MockSpeed::new(),
    ///         MockClimate::new(),
    ///     );
    ///
    ///     check(&mut heater, on)?;
    ///     check(&mut thermostat, on)?;
    ///     check(&mut ac, on)?;
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`DeviceError::NotSupported`]: crate::DeviceError::NotSupported
    /// [`SwingMode`]: crate::SwingMode
    fn set_swing(&mut self, swing: SwingMode) -> Result<(), DeviceError>;

    /// Returns the target temperature in degrees Celsius.
    ///
    /// The value is the target temperature that the backend holds. It is not a
    /// measured temperature, because this crate does not read sensors. The
    /// call cannot fail. It does not promise a fresh reading from hardware, so
    /// the value may not show a change made outside the library.
    fn temperature(&self) -> f32;

    /// Returns the climate mode.
    ///
    /// The value is the mode that the backend holds. The call cannot fail. It
    /// does not promise a fresh reading from hardware, so the value may not
    /// show a change made outside the library.
    fn mode(&self) -> ClimateMode;

    /// Returns the swing mode.
    ///
    /// The value is the swing mode that the backend holds. The call cannot
    /// fail. It does not promise a fresh reading from hardware, so the value
    /// may not show a change made outside the library.
    fn swing(&self) -> SwingMode;
}
