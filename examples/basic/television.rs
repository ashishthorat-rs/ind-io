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

    tv.mute()?;
    println!("muted: {}, volume: {}", tv.is_muted(), tv.volume());
    assert!(tv.is_muted());
    assert_eq!(tv.volume(), 31);

    tv.volume_up()?;
    println!("muted: {}, volume: {}", tv.is_muted(), tv.volume());
    assert!(tv.is_muted());
    assert_eq!(tv.volume(), 32);

    tv.unmute()?;
    println!("muted: {}, volume: {}", tv.is_muted(), tv.volume());
    assert!(!tv.is_muted());
    assert_eq!(tv.volume(), 32);

    let result = tv.set_channel(0);
    println!("setting channel 0 gave {result:?}");
    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(tv.channel(), 8);

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
