//! A television combines two capabilities: power and media.
//!
//! Channel numbers start at 1, and channel 0 is rejected with
//! `DeviceError::InvalidInput` without changing the channel. Mute and volume
//! are independent: muting keeps the volume, changing the volume while muted
//! keeps the television muted, and unmuting keeps the volume. Turning the
//! television off keeps the channel and volume. Run it with
//! `cargo run --example basic_television`.

use ind_io::mock::{MockMedia, MockPower};
use ind_io::traits::{MediaControl, PowerControl};
use ind_io::{DeviceError, Television};

fn main() -> Result<(), DeviceError> {
    let mut tv = Television::new(MockPower::new(), MockMedia::new());

    tv.turn_on()?;

    tv.set_channel(7)?;
    tv.channel_up()?;
    println!("channel: {}", tv.channel());
    assert_eq!(tv.channel(), 8);

    tv.set_volume(30)?;
    tv.volume_up()?;
    println!("volume: {}", tv.volume());
    assert_eq!(tv.volume(), 31);
    assert!(!tv.is_muted());

    // Muting keeps the volume level.
    tv.mute()?;
    println!("muted: {}, volume: {}", tv.is_muted(), tv.volume());
    assert!(tv.is_muted());
    assert_eq!(tv.volume(), 31);

    // Changing the volume does not unmute the television.
    tv.volume_up()?;
    println!("muted: {}, volume: {}", tv.is_muted(), tv.volume());
    assert!(tv.is_muted());
    assert_eq!(tv.volume(), 32);

    // Unmuting keeps the volume.
    tv.unmute()?;
    println!("muted: {}, volume: {}", tv.is_muted(), tv.volume());
    assert!(!tv.is_muted());
    assert_eq!(tv.volume(), 32);

    // Channel numbers start at 1. Channel 0 is rejected, and the channel does
    // not change.
    let result = tv.set_channel(0);
    println!("setting channel 0 gave {result:?}");
    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(tv.channel(), 8);

    // Turning the television off keeps the channel and volume.
    tv.turn_off()?;
    println!(
        "on: {}, channel: {}, volume: {}",
        tv.is_on(),
        tv.channel(),
        tv.volume()
    );
    assert!(!tv.is_on());
    assert_eq!(tv.channel(), 8);
    assert_eq!(tv.volume(), 32);

    Ok(())
}
