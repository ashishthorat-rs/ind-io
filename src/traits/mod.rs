//! The capability traits.
//!
//! Each capability trait states the contract for one thing a device can do:
//!
//! * [`PowerControl`]: switching power on and off, and reading the power
//!   state;
//! * [`SpeedControl`]: setting and reading a [`Speed`];
//! * [`ClimateControl`]: setting and reading the target temperature, the
//!   climate mode and the swing mode;
//! * [`MediaControl`]: changing and reading the channel, the volume and mute;
//! * [`LightControl`]: setting and reading the brightness;
//! * [`PositionControl`]: setting and reading a position from 0 to 100
//!   percent, with `open` and `close`.
//!
//! Within this crate, a backend implements the trait of the capability it
//! provides. This crate has one mock backend for each capability, such as
//! [`MockPower`]. A device combines backends and implements the traits of
//! their capabilities. Its required methods pass each call on to the backend
//! of that capability. For example, a [`Fan`] built from a [`MockPower`] and
//! a [`MockSpeed`] implements both [`PowerControl`] and [`SpeedControl`].
//!
//! # Importing the traits
//!
//! The traits are not re-exported at the crate root, and the crate has no
//! prelude. Method syntax on a device or a mock backend works only when the
//! trait is in scope, so import each trait whose methods you call, as in
//! `use ind_io::traits::PowerControl;`. Each trait is defined in a submodule
//! of its own, such as `traits::power`, and re-exported here. Both paths name
//! the same trait.
//!
//! # Independent capabilities
//!
//! The capabilities are independent. Under the contract, a setting never
//! changes the power state, and turning the power on or off keeps every value
//! that was set. `Speed::Off`, a brightness of 0, a position of 0 and a volume
//! of 0 are levels of their own capability, not power states, and a device can
//! be on at any of them. No contract in this crate couples one capability to
//! another.
//!
//! The contract does not promise that hardware can carry out every operation
//! in every state. A backend may reject an operation that its hardware cannot
//! accept in the current state. Under the contract, a backend that rejects an
//! operation reports an error, and does not change another capability to
//! simulate success. The mock backends accept a call that sets or steps a
//! value while the device is off, as long as the value is valid.
//!
//! With the mock backends, the first example calls every method that sets or
//! steps a value of a capability other than power, with the power off and with
//! the power on, and checks that the power state is the same afterwards. It
//! includes the levels that look like off: the speed `Off`, a brightness of 0,
//! a volume of 0 and a position of 0:
//!
//! ```
//! use ind_io::mock::{
//!     MockClimate, MockLight, MockMedia, MockPosition, MockPower, MockSpeed,
//! };
//! use ind_io::traits::{
//!     ClimateControl, LightControl, MediaControl, PositionControl,
//!     PowerControl, SpeedControl,
//! };
//! use ind_io::{
//!     AirConditioner, ClimateMode, Curtain, DeviceError, Light, Speed,
//!     SwingMode, Television,
//! };
//!
//! // Runs `setting` with the power off and with the power on. The power
//! // state is the same after each run.
//! fn power_kept<D: PowerControl>(
//!     device: &mut D,
//!     setting: impl Fn(&mut D) -> Result<(), DeviceError>,
//! ) -> Result<(), DeviceError> {
//!     for on in [false, true] {
//!         if on {
//!             device.turn_on()?;
//!         } else {
//!             device.turn_off()?;
//!         }
//!         setting(device)?;
//!         assert_eq!(device.is_on(), on);
//!     }
//!     Ok(())
//! }
//!
//! let mut ac = AirConditioner::new(
//!     MockPower::new(),
//!     MockSpeed::new(),
//!     MockClimate::new(),
//! );
//! power_kept(&mut ac, |d| d.set_speed(Speed::High))?;
//! power_kept(&mut ac, |d| d.set_speed(Speed::Off))?;
//! power_kept(&mut ac, |d| d.set_temperature(18.5))?;
//! power_kept(&mut ac, |d| d.set_mode(ClimateMode::Cool))?;
//! power_kept(&mut ac, |d| d.set_swing(SwingMode::Both))?;
//!
//! let mut light = Light::new(MockPower::new(), MockLight::new());
//! power_kept(&mut light, |d| d.set_brightness(30))?;
//! power_kept(&mut light, |d| d.set_brightness(0))?;
//!
//! let mut tv = Television::new(MockPower::new(), MockMedia::new());
//! power_kept(&mut tv, |d| d.channel_up())?;
//! power_kept(&mut tv, |d| d.channel_down())?;
//! power_kept(&mut tv, |d| d.set_channel(7))?;
//! power_kept(&mut tv, |d| d.set_volume(20))?;
//! power_kept(&mut tv, |d| d.set_volume(0))?;
//! power_kept(&mut tv, |d| d.volume_up())?;
//! power_kept(&mut tv, |d| d.volume_down())?;
//! power_kept(&mut tv, |d| d.mute())?;
//! power_kept(&mut tv, |d| d.unmute())?;
//! power_kept(&mut tv, |d| d.toggle_mute())?;
//!
//! let mut curtain = Curtain::new(MockPower::new(), MockPosition::new());
//! power_kept(&mut curtain, |d| d.open())?;
//! power_kept(&mut curtain, |d| d.close())?;
//! power_kept(&mut curtain, |d| d.set_position(0))?;
//! power_kept(&mut curtain, |d| d.set_position(40))?;
//! # Ok::<(), ind_io::DeviceError>(())
//! ```
//!
//! With the mock backends, the second example sets a value for each capability
//! other than power. It then turns the power off, on and off again, and checks,
//! through every getter other than `is_on`, that the value is the same after
//! each change:
//!
//! ```
//! use ind_io::mock::{
//!     MockClimate, MockLight, MockMedia, MockPosition, MockPower, MockSpeed,
//! };
//! use ind_io::traits::{
//!     ClimateControl, LightControl, MediaControl, PositionControl,
//!     PowerControl, SpeedControl,
//! };
//! use ind_io::{
//!     AirConditioner, ClimateMode, Curtain, DeviceError, Light, Speed,
//!     SwingMode, Television,
//! };
//!
//! // Turns the power off, on and off again. `value` does not change.
//! fn value_kept<D, T>(
//!     device: &mut D,
//!     value: impl Fn(&D) -> T,
//! ) -> Result<(), DeviceError>
//! where
//!     D: PowerControl,
//!     T: PartialEq + std::fmt::Debug,
//! {
//!     let before = value(device);
//!     for on in [false, true, false] {
//!         if on {
//!             device.turn_on()?;
//!         } else {
//!             device.turn_off()?;
//!         }
//!         assert_eq!(value(device), before);
//!     }
//!     Ok(())
//! }
//!
//! let mut ac = AirConditioner::new(
//!     MockPower::new(),
//!     MockSpeed::new(),
//!     MockClimate::new(),
//! );
//! ac.set_speed(Speed::High)?;
//! ac.set_temperature(18.5)?;
//! ac.set_mode(ClimateMode::Cool)?;
//! ac.set_swing(SwingMode::Both)?;
//! value_kept(&mut ac, |d| d.speed())?;
//! value_kept(&mut ac, |d| d.temperature())?;
//! value_kept(&mut ac, |d| d.mode())?;
//! value_kept(&mut ac, |d| d.swing())?;
//!
//! let mut light = Light::new(MockPower::new(), MockLight::new());
//! light.set_brightness(30)?;
//! value_kept(&mut light, |d| d.brightness())?;
//!
//! let mut tv = Television::new(MockPower::new(), MockMedia::new());
//! tv.set_channel(7)?;
//! tv.set_volume(20)?;
//! tv.mute()?;
//! value_kept(&mut tv, |d| d.channel())?;
//! value_kept(&mut tv, |d| d.volume())?;
//! value_kept(&mut tv, |d| d.is_muted())?;
//!
//! let mut curtain = Curtain::new(MockPower::new(), MockPosition::new());
//! curtain.set_position(40)?;
//! value_kept(&mut curtain, |d| d.position())?;
//! curtain.set_position(100)?;
//! value_kept(&mut curtain, |d| d.is_open())?;
//! curtain.set_position(0)?;
//! value_kept(&mut curtain, |d| d.is_closed())?;
//! # Ok::<(), ind_io::DeviceError>(())
//! ```
//!
//! The mock backends keep the values that were set across a power change, as
//! the second example shows. Whether hardware can keep them has not been
//! validated. Under the contract, a power change that cannot keep the values
//! is reported as a [`DeviceError`], not as success. [`MockPower`] never fails
//! a power change, so no example here shows that error.
//!
//! An air conditioner combines two capabilities besides power: speed and
//! climate. With the mock backends, a speed leaves the climate settings alone,
//! and the climate settings leave the speed alone:
//!
//! ```
//! use ind_io::mock::{MockClimate, MockPower, MockSpeed};
//! use ind_io::traits::{ClimateControl, SpeedControl};
//! use ind_io::{AirConditioner, ClimateMode, Speed, SwingMode};
//!
//! let mut ac = AirConditioner::new(
//!     MockPower::new(),
//!     MockSpeed::new(),
//!     MockClimate::new(),
//! );
//!
//! ac.set_speed(Speed::High)?;
//! ac.set_temperature(18.5)?;
//! ac.set_mode(ClimateMode::Cool)?;
//! ac.set_swing(SwingMode::Both)?;
//!
//! // A speed leaves the climate settings alone.
//! ac.set_speed(Speed::Low)?;
//! assert_eq!(ac.temperature(), 18.5);
//! assert_eq!(ac.mode(), ClimateMode::Cool);
//! assert_eq!(ac.swing(), SwingMode::Both);
//!
//! // Climate settings leave the speed alone.
//! ac.set_temperature(24.0)?;
//! ac.set_mode(ClimateMode::Heat)?;
//! ac.set_swing(SwingMode::Off)?;
//! assert_eq!(ac.speed(), Speed::Low);
//! # Ok::<(), ind_io::DeviceError>(())
//! ```
//!
//! # Sealed traits
//!
//! The capability traits are sealed. The signature of a trait shows the
//! mechanism, as in `pub trait PowerControl: Sealed`. `Sealed` is a supertrait
//! that code outside this crate cannot name, so only types in this crate can
//! implement the trait. The other five capability traits have the same
//! supertrait. The page of each trait lists the types that implement it.
//!
//! The traits are sealed because their contracts have not been validated
//! against real backends. Sealing keeps every implementation inside this
//! crate, where a change to a contract can be reviewed together with the code
//! that implements it.
//!
//! Code outside this crate can:
//!
//! * call the trait methods on the devices and the mock backends;
//! * use a trait as a generic bound or as a trait object for those types;
//! * own a device in a field of a type of its own, and pass that field to
//!   code that has a trait bound.
//!
//! Code outside this crate cannot implement `PowerControl` for a type of its
//! own, and the same holds for the other five traits because they have the
//! same supertrait. The example tries `PowerControl`:
//!
//! ```compile_fail,E0277
//! use ind_io::traits::PowerControl;
//! use ind_io::DeviceError;
//!
//! struct Lamp {
//!     on: bool,
//! }
//!
//! // `PowerControl` is sealed, so this does not compile.
//! impl PowerControl for Lamp {
//!     fn turn_on(&mut self) -> Result<(), DeviceError> {
//!         self.on = true;
//!         Ok(())
//!     }
//!
//!     fn turn_off(&mut self) -> Result<(), DeviceError> {
//!         self.on = false;
//!         Ok(())
//!     }
//!
//!     fn is_on(&self) -> bool {
//!         self.on
//!     }
//! }
//! ```
//!
//! Because the traits are sealed, a backend can be added only inside this
//! crate, so a separate crate cannot provide one.
//!
//! Owning a device does not make a type implement a capability trait, but the
//! devices that the type owns still satisfy a trait bound:
//!
//! ```
//! use ind_io::mock::{MockLight, MockPower, MockSpeed};
//! use ind_io::traits::PowerControl;
//! use ind_io::{DeviceError, Fan, Light};
//!
//! struct CeilingFan {
//!     fan: Fan<MockPower, MockSpeed>,
//!     light: Light<MockPower, MockLight>,
//! }
//!
//! fn switch_on<D: PowerControl>(device: &mut D) -> Result<(), DeviceError> {
//!     device.turn_on()
//! }
//!
//! let mut ceiling_fan = CeilingFan {
//!     fan: Fan::new(MockPower::new(), MockSpeed::new()),
//!     light: Light::new(MockPower::new(), MockLight::new()),
//! };
//!
//! switch_on(&mut ceiling_fan.fan)?;
//! switch_on(&mut ceiling_fan.light)?;
//! assert!(ceiling_fan.fan.is_on());
//! assert!(ceiling_fan.light.is_on());
//! # Ok::<(), ind_io::DeviceError>(())
//! ```
//!
//! `CeilingFan` of the previous example does not implement `PowerControl`, and
//! code outside this crate cannot implement it for the type. So the type
//! itself does not satisfy the bound, and it cannot be used as a
//! `PowerControl` trait object either:
//!
//! ```compile_fail,E0277
//! # use ind_io::mock::{MockLight, MockPower, MockSpeed};
//! # use ind_io::traits::PowerControl;
//! # use ind_io::{DeviceError, Fan, Light};
//! #
//! # struct CeilingFan {
//! #     fan: Fan<MockPower, MockSpeed>,
//! #     light: Light<MockPower, MockLight>,
//! # }
//! #
//! # fn switch_on<D: PowerControl>(device: &mut D) -> Result<(), DeviceError> {
//! #     device.turn_on()
//! # }
//! #
//! # let mut ceiling_fan = CeilingFan {
//! #     fan: Fan::new(MockPower::new(), MockSpeed::new()),
//! #     light: Light::new(MockPower::new(), MockLight::new()),
//! # };
//! #
//! // This does not compile.
//! switch_on(&mut ceiling_fan)?;
//! # Ok::<(), ind_io::DeviceError>(())
//! ```
//!
//! ```compile_fail,E0277
//! # use ind_io::mock::{MockLight, MockPower, MockSpeed};
//! # use ind_io::traits::PowerControl;
//! # use ind_io::{Fan, Light};
//! #
//! # struct CeilingFan {
//! #     fan: Fan<MockPower, MockSpeed>,
//! #     light: Light<MockPower, MockLight>,
//! # }
//! #
//! # let mut ceiling_fan = CeilingFan {
//! #     fan: Fan::new(MockPower::new(), MockSpeed::new()),
//! #     light: Light::new(MockPower::new(), MockLight::new()),
//! # };
//! #
//! // This does not compile either.
//! let power: &mut dyn PowerControl = &mut ceiling_fan;
//! ```
//!
//! # Trait objects
//!
//! Each of the six traits can be used as a trait object, such as
//! `&mut dyn PowerControl`. No default method has a `Self: Sized` clause, so
//! the default methods `toggle`, `reset`, `toggle_mute`, `open`, `close`,
//! `is_open` and `is_closed` can be called on a trait object as well. Code
//! that takes a trait object can work with devices of different types. The
//! next example builds three devices from mock backends and uses them through
//! one trait object type:
//!
//! ```
//! use ind_io::mock::{MockLight, MockPower, MockSpeed};
//! use ind_io::traits::PowerControl;
//! use ind_io::{Fan, Light, Switch};
//!
//! let mut switch = Switch::new(MockPower::new());
//! let mut fan = Fan::new(MockPower::new(), MockSpeed::new());
//! let mut light = Light::new(MockPower::new(), MockLight::new());
//!
//! let devices: [&mut dyn PowerControl; 3] =
//!     [&mut switch, &mut fan, &mut light];
//! for device in devices {
//!     let was_on = device.is_on();
//!     device.toggle()?;
//!     assert_eq!(device.is_on(), !was_on);
//!
//!     device.reset()?;
//!     assert!(device.is_on());
//! }
//! # Ok::<(), ind_io::DeviceError>(())
//! ```
//!
//! The next example builds four devices from mock backends. It uses every
//! trait and every default method through a trait object:
//!
//! ```
//! use ind_io::mock::{
//!     MockClimate, MockLight, MockMedia, MockPosition, MockPower, MockSpeed,
//! };
//! use ind_io::traits::{
//!     ClimateControl, LightControl, MediaControl, PositionControl,
//!     PowerControl, SpeedControl,
//! };
//! use ind_io::{AirConditioner, Curtain, Light, Speed, Television};
//!
//! let mut ac = AirConditioner::new(
//!     MockPower::new(),
//!     MockSpeed::new(),
//!     MockClimate::new(),
//! );
//! let mut light = Light::new(MockPower::new(), MockLight::new());
//! let mut tv = Television::new(MockPower::new(), MockMedia::new());
//! let mut curtain = Curtain::new(MockPower::new(), MockPosition::new());
//!
//! let power: &mut dyn PowerControl = &mut ac;
//! let was_on = power.is_on();
//! power.toggle()?;
//! assert_eq!(power.is_on(), !was_on);
//! power.reset()?;
//! assert!(power.is_on());
//!
//! let speed: &mut dyn SpeedControl = &mut ac;
//! speed.set_speed(Speed::Low)?;
//! assert_eq!(speed.speed(), Speed::Low);
//!
//! let climate: &mut dyn ClimateControl = &mut ac;
//! climate.set_temperature(21.0)?;
//! assert_eq!(climate.temperature(), 21.0);
//!
//! let brightness: &mut dyn LightControl = &mut light;
//! brightness.set_brightness(30)?;
//! assert_eq!(brightness.brightness(), 30);
//!
//! let media: &mut dyn MediaControl = &mut tv;
//! let was_muted = media.is_muted();
//! media.toggle_mute()?;
//! assert_eq!(media.is_muted(), !was_muted);
//!
//! let position: &mut dyn PositionControl = &mut curtain;
//! position.open()?;
//! assert!(position.is_open());
//! position.close()?;
//! assert!(position.is_closed());
//! # Ok::<(), ind_io::DeviceError>(())
//! ```
//!
//! # Reading state
//!
//! A getter returns a plain value, not a `Result`, so it cannot fail. The
//! value is the state that the backend holds, such as the last speed that was
//! set, and not a measurement: [`ClimateControl::temperature`], for example,
//! is the target temperature, not the temperature of a room. The API does not
//! promise a fresh reading from hardware, so a getter may not show a change
//! made outside the library.
//!
//! [`Speed`]: crate::Speed
//! [`Fan`]: crate::Fan
//! [`MockPower`]: crate::mock::MockPower
//! [`MockSpeed`]: crate::mock::MockSpeed
//! [`DeviceError`]: crate::DeviceError

mod private {
    pub trait Sealed {}
}

pub(crate) use private::Sealed;

pub mod climate;
pub mod light;
pub mod media;
pub mod position;
pub mod power;
pub mod speed;

pub use climate::ClimateControl;
pub use light::LightControl;
pub use media::MediaControl;
pub use position::PositionControl;
pub use power::PowerControl;
pub use speed::SpeedControl;
