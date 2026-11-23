//! Value types shared by the capability traits and the devices.
//!
//! [`Speed`], [`ClimateMode`] and [`SwingMode`] are plain enums. Each one is
//! `Copy` and can be compared for equality. Each one is `#[non_exhaustive]`,
//! so a `match` in another crate needs a wildcard arm. The types are
//! re-exported at the crate root, so `ind_io::Speed` and `ind_io::types::Speed`
//! name the same type.

pub(crate) const MAX_PERCENT: u8 = 100;

/// A discrete speed setting, set through
/// [`SpeedControl`](crate::traits::SpeedControl).
///
/// The crate defines no number for any level, such as a count of revolutions
/// per minute. What a level means is left to the backend.
///
/// # Speed and power
///
/// [`Speed::Off`] is a level of the speed capability, not a power state. A
/// device can be on while its speed is `Off`, and setting a speed does not
/// change the power state. A backend may reject a speed that its hardware
/// cannot accept in the current state, for example while the device is off.
/// The mock backends do not.
///
/// # Comparing speeds
///
/// `Speed` implements neither `PartialOrd` nor `Ord`. To compare levels, rank
/// the variants in your own code. The enum is `#[non_exhaustive]`, so a
/// `match` over its variants needs a wildcard arm.
///
/// # Examples
///
/// The mock backends accept a speed while the device is off. Setting a speed
/// leaves the power state alone, and `Off` is only a speed:
///
/// ```
/// use ind_io::mock::{MockPower, MockSpeed};
/// use ind_io::traits::{PowerControl, SpeedControl};
/// use ind_io::{Fan, Speed};
///
/// let mut fan = Fan::new(MockPower::new(), MockSpeed::new());
///
/// fan.set_speed(Speed::High)?;
/// assert!(!fan.is_on());
///
/// fan.turn_on()?;
/// fan.set_speed(Speed::Off)?;
/// assert!(fan.is_on());
/// assert_eq!(fan.speed(), Speed::Off);
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// Rank the speeds in your own code:
///
/// ```
/// use ind_io::Speed;
///
/// fn rank(speed: Speed) -> u8 {
///     match speed {
///         Speed::Off => 0,
///         Speed::Low => 1,
///         Speed::Medium => 2,
///         Speed::High => 3,
///         Speed::Turbo => 4,
///         // A variant added later lands here. Choose its rank.
///         _ => 5,
///     }
/// }
///
/// assert!(rank(Speed::Turbo) > rank(Speed::Low));
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speed {
    /// The lowest named level.
    ///
    /// This is a level of the speed capability, not a power state. A device
    /// can be on at `Off`.
    Off,
    /// A low speed.
    Low,
    /// A medium speed.
    Medium,
    /// A high speed.
    High,
    /// The highest named level.
    Turbo,
}

/// The operating mode of a climate backend, set through
/// [`ClimateControl::set_mode`](crate::traits::ClimateControl::set_mode).
///
/// The variant names give the usual meaning of each mode. The crate attaches
/// no behavior to them: a backend decides what a mode does.
///
/// The devices do not restrict the mode. A [`Heater`](crate::Heater), a
/// [`Thermostat`](crate::Thermostat) and an
/// [`AirConditioner`](crate::AirConditioner) pass every mode on to their
/// backend, so a `Heater` built on
/// [`MockClimate`](crate::mock::MockClimate) accepts [`ClimateMode::Cool`].
/// The crate does not specify which error a backend returns for a mode that
/// it cannot support.
///
/// # Examples
///
/// ```
/// use ind_io::mock::{MockClimate, MockPower};
/// use ind_io::traits::ClimateControl;
/// use ind_io::{ClimateMode, Heater};
///
/// let mut heater = Heater::new(MockPower::new(), MockClimate::new());
///
/// heater.set_mode(ClimateMode::Cool)?;
/// assert_eq!(heater.mode(), ClimateMode::Cool);
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClimateMode {
    /// Cooling mode.
    Cool,
    /// Heating mode.
    Heat,
    /// Dehumidifying mode.
    Dry,
    /// Fan-only mode: air circulates without cooling or heating.
    Fan,
    /// Automatic mode: cooling or heating is chosen automatically.
    Auto,
}

/// The swing setting of a climate backend, set through
/// [`ClimateControl::set_swing`](crate::traits::ClimateControl::set_swing).
///
/// Swing is the movement of an air vane. The variant names give the usual
/// meaning of each setting. A backend decides what a setting does and whether
/// it offers swing at all. [`DeviceError::NotSupported`] is the variant for an
/// operation that a backend does not offer, but that variant is provisional
/// and no type in this crate returns it.
///
/// # Examples
///
/// The mock backend accepts every setting:
///
/// ```
/// use ind_io::mock::{MockClimate, MockPower, MockSpeed};
/// use ind_io::traits::ClimateControl;
/// use ind_io::{AirConditioner, SwingMode};
///
/// let mut ac = AirConditioner::new(
///     MockPower::new(),
///     MockSpeed::new(),
///     MockClimate::new(),
/// );
///
/// let settings = [
///     SwingMode::Off,
///     SwingMode::Vertical,
///     SwingMode::Horizontal,
///     SwingMode::Both,
/// ];
///
/// for swing in settings {
///     ac.set_swing(swing)?;
///     assert_eq!(ac.swing(), swing);
/// }
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// [`DeviceError::NotSupported`]: crate::DeviceError::NotSupported
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwingMode {
    /// The vane does not move.
    Off,
    /// The vane swings up and down.
    Vertical,
    /// The vane swings from side to side.
    Horizontal,
    /// The vane swings in both directions.
    Both,
}
