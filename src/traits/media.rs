//! The [`MediaControl`] capability: changing the channel, the volume and the
//! mute state.
//!
//! The trait is re-exported as [`crate::traits::MediaControl`].

use crate::error::DeviceError;
use crate::traits::Sealed;

/// The capability to control media: to change the channel and the volume, to
/// mute the sound, and to read each of them back.
///
/// Channel numbers start at 1. Volume is a value from 0 to 100. Its limit is
/// separate from the limit that the brightness of [`LightControl`] and the
/// position of [`PositionControl`] share, because a volume is a different
/// quantity. Both limits are 100.
///
/// Mute and volume are independent in both directions. Under the contract,
/// muting keeps the volume, and changing the volume does not change the mute
/// state. A volume of 0 is a level and is not the same as muted. Neither is a
/// power state: a device can be on while it is muted and while its volume is
/// 0. Setting a channel or a volume does not turn the power on, and turning
/// the power off keeps the channel, the volume and the mute state; see
/// [independent capabilities].
///
/// The trait is sealed, so only types in this crate implement it.
/// [`MockMedia`] and [`Television`] do, and any code can use the trait as a
/// generic bound or as a trait object for them. See [sealed traits].
///
/// # Default methods
///
/// [`toggle_mute`] is a default method, built from [`is_muted`], [`mute`] and
/// [`unmute`]. No type in this crate replaces it. A device runs it on its own
/// `is_muted`, `mute` and `unmute`, which pass each call on to the backend. A
/// device does not call a `toggle_mute` of its backend. The method has no
/// `Self: Sized` clause, so it can be called through a trait object such as
/// `&mut dyn MediaControl`.
///
/// # Examples
///
/// Code that is generic over the capability accepts any type that implements
/// the trait:
///
/// ```
/// use ind_io::mock::{MockMedia, MockPower};
/// use ind_io::traits::MediaControl;
/// use ind_io::{DeviceError, Television};
///
/// fn quiet<D: MediaControl>(device: &mut D) -> Result<(), DeviceError> {
///     device.set_volume(10)
/// }
///
/// let mut tv = Television::new(MockPower::new(), MockMedia::new());
///
/// quiet(&mut tv)?;
/// assert_eq!(tv.volume(), 10);
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// With the mock backends, a power change keeps the channel, the volume and
/// the mute state, whether the sound is muted or not:
///
/// ```
/// use ind_io::mock::{MockMedia, MockPower};
/// use ind_io::traits::{MediaControl, PowerControl};
/// use ind_io::Television;
///
/// for muted in [false, true] {
///     let power = MockPower::with_state(true);
///     let mut tv = Television::new(power, MockMedia::new());
///     tv.set_channel(7)?;
///     tv.set_volume(30)?;
///     if muted {
///         tv.mute()?;
///     } else {
///         tv.unmute()?;
///     }
///
///     tv.turn_off()?;
///     assert_eq!(tv.channel(), 7);
///     assert_eq!(tv.volume(), 30);
///     assert_eq!(tv.is_muted(), muted);
///
///     tv.turn_on()?;
///     assert_eq!(tv.channel(), 7);
///     assert_eq!(tv.volume(), 30);
///     assert_eq!(tv.is_muted(), muted);
/// }
/// # Ok::<(), ind_io::DeviceError>(())
/// ```
///
/// [`toggle_mute`]: MediaControl::toggle_mute
/// [`is_muted`]: MediaControl::is_muted
/// [`mute`]: MediaControl::mute
/// [`unmute`]: MediaControl::unmute
/// [`LightControl`]: crate::traits::LightControl
/// [`PositionControl`]: crate::traits::PositionControl
/// [`MockMedia`]: crate::mock::MockMedia
/// [`Television`]: crate::Television
/// [independent capabilities]: crate::traits#independent-capabilities
/// [sealed traits]: crate::traits#sealed-traits
pub trait MediaControl: Sealed {
    /// Moves to the next higher channel.
    ///
    /// Only the channel changes. The volume, the mute state and the power
    /// state do not.
    ///
    /// Under the contract, at the highest channel that the backend supports,
    /// `channel_up` leaves the channel unchanged and returns `Ok`. The mock
    /// backend moves by 1, and its highest channel is `u32::MAX`.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the backend cannot change the channel. The
    /// crate does not specify which variant. No type in this crate returns an
    /// error from this method.
    ///
    /// # Examples
    ///
    /// With the mock backend, `channel_up` moves to the next higher channel,
    /// and stops at `u32::MAX` instead of overflowing:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::MediaControl;
    /// use ind_io::Television;
    ///
    /// let mut tv = Television::new(MockPower::new(), MockMedia::new());
    ///
    /// tv.set_channel(5)?;
    /// tv.channel_up()?;
    /// assert_eq!(tv.channel(), 6);
    ///
    /// tv.set_channel(u32::MAX)?;
    /// tv.channel_up()?;
    /// assert_eq!(tv.channel(), u32::MAX);
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// With the mock backend, `channel_up` keeps the volume, the mute state
    /// and the power state, with the sound muted and not muted and with the
    /// power off and on:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::{MediaControl, PowerControl};
    /// use ind_io::Television;
    ///
    /// for on in [false, true] {
    ///     for muted in [false, true] {
    ///         let power = MockPower::with_state(on);
    ///         let mut tv = Television::new(power, MockMedia::new());
    ///         tv.set_channel(5)?;
    ///         tv.set_volume(30)?;
    ///         if muted {
    ///             tv.mute()?;
    ///         } else {
    ///             tv.unmute()?;
    ///         }
    ///
    ///         tv.channel_up()?;
    ///         assert_eq!(tv.channel(), 6);
    ///         assert_eq!(tv.volume(), 30);
    ///         assert_eq!(tv.is_muted(), muted);
    ///         assert_eq!(tv.is_on(), on);
    ///     }
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    fn channel_up(&mut self) -> Result<(), DeviceError>;

    /// Moves to the next lower channel.
    ///
    /// Only the channel changes. The volume, the mute state and the power
    /// state do not.
    ///
    /// Under the contract, at channel 1, `channel_down` leaves the channel
    /// unchanged and returns `Ok`. The mock backend moves by 1.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the backend cannot change the channel. The
    /// crate does not specify which variant. No type in this crate returns an
    /// error from this method.
    ///
    /// # Examples
    ///
    /// With the mock backend, `channel_down` moves to the next lower channel,
    /// and stops at channel 1:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::MediaControl;
    /// use ind_io::Television;
    ///
    /// let mut tv = Television::new(MockPower::new(), MockMedia::new());
    ///
    /// tv.set_channel(3)?;
    /// tv.channel_down()?;
    /// assert_eq!(tv.channel(), 2);
    /// tv.channel_down()?;
    /// assert_eq!(tv.channel(), 1);
    ///
    /// tv.channel_down()?;
    /// assert_eq!(tv.channel(), 1);
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// With the mock backend, `channel_down` keeps the volume, the mute state
    /// and the power state, with the sound muted and not muted and with the
    /// power off and on:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::{MediaControl, PowerControl};
    /// use ind_io::Television;
    ///
    /// for on in [false, true] {
    ///     for muted in [false, true] {
    ///         let power = MockPower::with_state(on);
    ///         let mut tv = Television::new(power, MockMedia::new());
    ///         tv.set_channel(5)?;
    ///         tv.set_volume(30)?;
    ///         if muted {
    ///             tv.mute()?;
    ///         } else {
    ///             tv.unmute()?;
    ///         }
    ///
    ///         tv.channel_down()?;
    ///         assert_eq!(tv.channel(), 4);
    ///         assert_eq!(tv.volume(), 30);
    ///         assert_eq!(tv.is_muted(), muted);
    ///         assert_eq!(tv.is_on(), on);
    ///     }
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    fn channel_down(&mut self) -> Result<(), DeviceError>;

    /// Sets the channel to `channel`, a number from 1.
    ///
    /// Only the channel changes. The power state does not. The mock backend
    /// accepts a channel while the device is off, and the device stays off.
    ///
    /// # Errors
    ///
    /// Returns [`DeviceError::InvalidInput`] if `channel` is 0. The channel is
    /// left unchanged. The mock backend enforces this limit, and its highest
    /// channel is `u32::MAX`. The limit has not been checked against a real
    /// backend.
    ///
    /// A backend that cannot select a valid channel returns a
    /// [`DeviceError`]. The crate does not specify which variant, and no type
    /// in this crate does so.
    ///
    /// # Examples
    ///
    /// The mock backend accepts a channel from 1 up with the power off and
    /// with the power on. It rejects channel 0 without changing the channel.
    /// The volume, the mute state and the power state stay as they were:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::{MediaControl, PowerControl};
    /// use ind_io::{DeviceError, Television};
    ///
    /// for on in [false, true] {
    ///     let power = MockPower::with_state(on);
    ///     let mut tv = Television::new(power, MockMedia::new());
    ///     tv.set_volume(30)?;
    ///     tv.mute()?;
    ///
    ///     for channel in [1, 2, 47, u32::MAX] {
    ///         tv.set_channel(channel)?;
    ///         assert_eq!(tv.channel(), channel);
    ///     }
    ///
    ///     tv.set_channel(47)?;
    ///     assert_eq!(tv.set_channel(0), Err(DeviceError::InvalidInput));
    ///     assert_eq!(tv.channel(), 47);
    ///
    ///     assert_eq!(tv.volume(), 30);
    ///     assert!(tv.is_muted());
    ///     assert_eq!(tv.is_on(), on);
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`DeviceError::InvalidInput`]: crate::DeviceError::InvalidInput
    fn set_channel(&mut self, channel: u32) -> Result<(), DeviceError>;

    /// Sets the volume to `volume`, a value from 0 to 100.
    ///
    /// Only the volume changes. The mute state and the power state do not. The
    /// mock backend accepts a volume while the device is off, and the device
    /// stays off.
    ///
    /// # Errors
    ///
    /// Returns [`DeviceError::InvalidInput`] if `volume` is above 100. The
    /// volume is left unchanged. The mock backend enforces this limit. The
    /// limit has not been checked against a real backend.
    ///
    /// A backend that cannot apply a valid volume in the current state
    /// returns a [`DeviceError`]. The crate does not specify which variant,
    /// and no type in this crate does so.
    ///
    /// # Examples
    ///
    /// The mock backend accepts a volume while the sound is muted, and the
    /// sound stays muted. A volume of 0 is a level: it does not mute the sound
    /// and does not turn the power off. The channel does not change:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::{MediaControl, PowerControl};
    /// use ind_io::Television;
    ///
    /// let power = MockPower::with_state(true);
    /// let mut tv = Television::new(power, MockMedia::new());
    /// tv.set_channel(7)?;
    ///
    /// tv.mute()?;
    /// tv.set_volume(50)?;
    /// assert_eq!(tv.volume(), 50);
    /// assert!(tv.is_muted());
    ///
    /// tv.unmute()?;
    /// tv.set_volume(0)?;
    /// assert_eq!(tv.volume(), 0);
    /// assert!(!tv.is_muted());
    /// assert!(tv.is_on());
    /// assert_eq!(tv.channel(), 7);
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// The mock backend accepts every value from 0 to 100 and rejects every
    /// value above 100, while the device is off. A rejected value leaves the
    /// volume, the mute state and the power state as they were:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::{MediaControl, PowerControl};
    /// use ind_io::{DeviceError, Television};
    ///
    /// let power = MockPower::with_state(false);
    /// let mut tv = Television::new(power, MockMedia::new());
    /// tv.mute()?;
    ///
    /// for volume in 0..=100 {
    ///     tv.set_volume(volume)?;
    ///     assert_eq!(tv.volume(), volume);
    ///     assert!(tv.is_muted());
    ///     assert!(!tv.is_on());
    /// }
    ///
    /// tv.set_volume(40)?;
    /// for volume in 101..=u8::MAX {
    ///     let result = tv.set_volume(volume);
    ///     assert_eq!(result, Err(DeviceError::InvalidInput));
    ///     assert_eq!(tv.volume(), 40);
    ///     assert!(tv.is_muted());
    ///     assert!(!tv.is_on());
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    /// [`DeviceError::InvalidInput`]: crate::DeviceError::InvalidInput
    fn set_volume(&mut self, volume: u8) -> Result<(), DeviceError>;

    /// Raises the volume by one step.
    ///
    /// How far a step moves the volume is up to the backend, because the right
    /// increment depends on what the backend controls. The mock backend moves
    /// it by 1. Under the contract, at volume 100, `volume_up` leaves the
    /// volume unchanged and returns `Ok`. Raising the volume does not change
    /// the mute state.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the backend cannot change the volume. The
    /// crate does not specify which variant. No type in this crate returns an
    /// error from this method.
    ///
    /// # Examples
    ///
    /// With the mock backend, `volume_up` raises the volume by 1, and stops at
    /// 100. The mute state stays as it was, whether the sound is muted or not:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::MediaControl;
    /// use ind_io::Television;
    ///
    /// for muted in [false, true] {
    ///     let mut tv = Television::new(MockPower::new(), MockMedia::new());
    ///     if muted {
    ///         tv.mute()?;
    ///     } else {
    ///         tv.unmute()?;
    ///     }
    ///     tv.set_volume(98)?;
    ///
    ///     tv.volume_up()?;
    ///     assert_eq!(tv.volume(), 99);
    ///     tv.volume_up()?;
    ///     assert_eq!(tv.volume(), 100);
    ///
    ///     tv.volume_up()?;
    ///     assert_eq!(tv.volume(), 100);
    ///     assert_eq!(tv.is_muted(), muted);
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    fn volume_up(&mut self) -> Result<(), DeviceError>;

    /// Lowers the volume by one step.
    ///
    /// How far a step moves the volume is up to the backend, because the right
    /// increment depends on what the backend controls. The mock backend moves
    /// it by 1. Under the contract, at volume 0, `volume_down` leaves the
    /// volume unchanged and returns `Ok`. Lowering the volume does not change
    /// the mute state.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the backend cannot change the volume. The
    /// crate does not specify which variant. No type in this crate returns an
    /// error from this method.
    ///
    /// # Examples
    ///
    /// With the mock backend, `volume_down` lowers the volume by 1, and stops
    /// at 0. The mute state stays as it was, whether the sound is muted or not:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::MediaControl;
    /// use ind_io::Television;
    ///
    /// for muted in [false, true] {
    ///     let mut tv = Television::new(MockPower::new(), MockMedia::new());
    ///     if muted {
    ///         tv.mute()?;
    ///     } else {
    ///         tv.unmute()?;
    ///     }
    ///     tv.set_volume(2)?;
    ///
    ///     tv.volume_down()?;
    ///     assert_eq!(tv.volume(), 1);
    ///     tv.volume_down()?;
    ///     assert_eq!(tv.volume(), 0);
    ///
    ///     tv.volume_down()?;
    ///     assert_eq!(tv.volume(), 0);
    ///     assert_eq!(tv.is_muted(), muted);
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    fn volume_down(&mut self) -> Result<(), DeviceError>;

    /// Mutes the sound.
    ///
    /// Muting keeps the volume and does not change the power state. A device
    /// can be on while it is muted.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the backend cannot mute the sound. The
    /// crate does not specify which variant. No type in this crate returns an
    /// error from this method.
    ///
    /// # Examples
    ///
    /// With the mock backend, muting twice leaves the sound muted with the
    /// power off and with the power on. The volume and the power state stay as
    /// they were:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::{MediaControl, PowerControl};
    /// use ind_io::Television;
    ///
    /// for on in [false, true] {
    ///     let power = MockPower::with_state(on);
    ///     let mut tv = Television::new(power, MockMedia::new());
    ///     tv.set_volume(30)?;
    ///     tv.unmute()?;
    ///
    ///     tv.mute()?;
    ///     tv.mute()?;
    ///     assert!(tv.is_muted());
    ///     assert_eq!(tv.volume(), 30);
    ///     assert_eq!(tv.is_on(), on);
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    fn mute(&mut self) -> Result<(), DeviceError>;

    /// Unmutes the sound.
    ///
    /// Unmuting keeps the volume and does not change the power state.
    ///
    /// # Errors
    ///
    /// Returns a [`DeviceError`] if the backend cannot unmute the sound. The
    /// crate does not specify which variant. No type in this crate returns an
    /// error from this method.
    ///
    /// # Examples
    ///
    /// With the mock backend, unmuting twice leaves the sound unmuted with the
    /// power off and with the power on. The volume and the power state stay as
    /// they were:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::{MediaControl, PowerControl};
    /// use ind_io::Television;
    ///
    /// for on in [false, true] {
    ///     let power = MockPower::with_state(on);
    ///     let mut tv = Television::new(power, MockMedia::new());
    ///     tv.set_volume(30)?;
    ///     tv.mute()?;
    ///
    ///     tv.unmute()?;
    ///     tv.unmute()?;
    ///     assert!(!tv.is_muted());
    ///     assert_eq!(tv.volume(), 30);
    ///     assert_eq!(tv.is_on(), on);
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`DeviceError`]: crate::DeviceError
    fn unmute(&mut self) -> Result<(), DeviceError>;

    /// Returns `true` if the sound is muted.
    ///
    /// The value is the mute state that the backend holds. The call cannot
    /// fail. It does not promise a fresh reading from hardware, so the value
    /// may not show a change made outside the library.
    fn is_muted(&self) -> bool;

    /// Returns the channel, a number from 1.
    ///
    /// The value is the channel that the backend holds. The call cannot fail.
    /// It does not promise a fresh reading from hardware, so the value may not
    /// show a change made outside the library.
    fn channel(&self) -> u32;

    /// Returns the volume, a value from 0 to 100.
    ///
    /// The value is the volume that the backend holds. It is the configured
    /// level, not a measure of how loud the sound is, and muting does not
    /// change it. The call cannot fail. It does not promise a fresh reading
    /// from hardware, so the value may not show a change made outside the
    /// library.
    fn volume(&self) -> u8;

    /// Toggles the mute: unmutes the sound if it is muted, and mutes it if it
    /// is not.
    ///
    /// The volume and the power state do not change.
    ///
    /// # Errors
    ///
    /// Returns the error of [`unmute`] or of [`mute`], whichever of the two
    /// the call makes.
    ///
    /// # Examples
    ///
    /// With the mock backend, toggling the mute from each starting state, with
    /// the power off and with the power on, flips the mute state. The volume
    /// and the power state stay as they were:
    ///
    /// ```
    /// use ind_io::mock::{MockMedia, MockPower};
    /// use ind_io::traits::{MediaControl, PowerControl};
    /// use ind_io::Television;
    ///
    /// for on in [false, true] {
    ///     for muted in [false, true] {
    ///         let power = MockPower::with_state(on);
    ///         let mut tv = Television::new(power, MockMedia::new());
    ///         tv.set_volume(30)?;
    ///         if muted {
    ///             tv.mute()?;
    ///         } else {
    ///             tv.unmute()?;
    ///         }
    ///
    ///         tv.toggle_mute()?;
    ///         assert_eq!(tv.is_muted(), !muted);
    ///         assert_eq!(tv.volume(), 30);
    ///         assert_eq!(tv.is_on(), on);
    ///     }
    /// }
    /// # Ok::<(), ind_io::DeviceError>(())
    /// ```
    ///
    /// [`unmute`]: MediaControl::unmute
    /// [`mute`]: MediaControl::mute
    fn toggle_mute(&mut self) -> Result<(), DeviceError> {
        if self.is_muted() {
            self.unmute()
        } else {
            self.mute()
        }
    }
}
