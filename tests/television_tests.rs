use ind_io::mock::{MockMedia, MockPower};
use ind_io::traits::{MediaControl, PowerControl};
use ind_io::{DeviceError, Television};

fn new_television() -> Television<MockPower, MockMedia> {
    Television::new(MockPower::new(), MockMedia::new())
}

fn power_on_any<P: PowerControl>(device: &mut P) -> Result<(), DeviceError> {
    device.turn_on()
}

fn set_volume_to<M: MediaControl>(device: &mut M, volume: u8) -> Result<(), DeviceError> {
    device.set_volume(volume)
}

#[test]
fn starts_off() {
    let tv = new_television();
    assert!(!tv.is_on());
}

#[test]
fn turn_on_and_turn_off_change_power_state() {
    let mut tv = new_television();
    tv.turn_on().unwrap();
    assert!(tv.is_on());
    tv.turn_off().unwrap();
    assert!(!tv.is_on());
}

#[test]
fn toggle_flips_power() {
    let mut tv = new_television();
    tv.toggle().unwrap();
    assert!(tv.is_on());
    tv.toggle().unwrap();
    assert!(!tv.is_on());
}

#[test]
fn reset_leaves_the_device_on_with_media_unchanged() {
    let mut tv = new_television();

    tv.set_channel(7).unwrap();
    tv.set_volume(40).unwrap();
    tv.mute().unwrap();
    tv.reset().unwrap();

    assert!(tv.is_on());
    assert_eq!(tv.channel(), 7);
    assert_eq!(tv.volume(), 40);
    assert!(tv.is_muted());
}

#[test]
fn channel_set_through_the_device_is_read_back() {
    let mut tv = new_television();
    tv.set_channel(47).unwrap();
    assert_eq!(tv.channel(), 47);
}

#[test]
fn volume_set_through_the_device_is_read_back() {
    let mut tv = new_television();
    tv.set_volume(30).unwrap();
    assert_eq!(tv.volume(), 30);
}

#[test]
fn channel_up_moves_to_a_higher_channel() {
    let mut tv = new_television();
    tv.set_channel(10).unwrap();
    tv.channel_up().unwrap();
    assert!(tv.channel() > 10);
}

#[test]
fn channel_down_moves_to_a_lower_channel() {
    let mut tv = new_television();
    tv.set_channel(10).unwrap();
    tv.channel_down().unwrap();
    assert!(tv.channel() < 10);
}

#[test]
fn volume_up_raises_the_volume() {
    let mut tv = new_television();
    tv.set_volume(20).unwrap();
    tv.volume_up().unwrap();
    assert!(tv.volume() > 20);
}

#[test]
fn volume_down_lowers_the_volume() {
    let mut tv = new_television();
    tv.set_volume(20).unwrap();
    tv.volume_down().unwrap();
    assert!(tv.volume() < 20);
}

#[test]
fn mute_and_unmute_change_the_mute_state() {
    let mut tv = new_television();

    tv.mute().unwrap();
    assert!(tv.is_muted());

    tv.unmute().unwrap();
    assert!(!tv.is_muted());
}

#[test]
fn toggle_mute_flips_the_mute_state() {
    let mut tv = new_television();

    tv.toggle_mute().unwrap();
    assert!(tv.is_muted());

    tv.toggle_mute().unwrap();
    assert!(!tv.is_muted());
}

#[test]
fn muting_keeps_the_volume_level() {
    let mut tv = new_television();

    tv.set_volume(30).unwrap();
    tv.mute().unwrap();

    assert!(tv.is_muted());
    assert_eq!(tv.volume(), 30);
}

#[test]
fn changing_the_volume_does_not_change_the_mute_state() {
    let mut tv = new_television();

    tv.mute().unwrap();
    tv.set_volume(50).unwrap();
    tv.volume_up().unwrap();

    assert!(tv.is_muted());
}

#[test]
fn setting_media_while_off_does_not_turn_it_on() {
    let mut tv = new_television();

    tv.set_channel(5).unwrap();
    tv.set_volume(25).unwrap();
    tv.mute().unwrap();

    assert!(!tv.is_on());
}

#[test]
fn setting_media_while_on_does_not_turn_it_off() {
    let mut tv = new_television();

    tv.turn_on().unwrap();
    tv.set_channel(5).unwrap();
    tv.set_volume(25).unwrap();
    tv.mute().unwrap();

    assert!(tv.is_on());
}

#[test]
fn turning_on_keeps_the_media_configured_while_off() {
    let mut tv = new_television();

    tv.set_channel(7).unwrap();
    tv.set_volume(40).unwrap();
    tv.mute().unwrap();
    tv.turn_on().unwrap();

    assert_eq!(tv.channel(), 7);
    assert_eq!(tv.volume(), 40);
    assert!(tv.is_muted());
}

#[test]
fn turning_off_keeps_the_configured_media() {
    let mut tv = new_television();

    tv.turn_on().unwrap();
    tv.set_channel(12).unwrap();
    tv.set_volume(55).unwrap();
    tv.mute().unwrap();
    tv.turn_off().unwrap();

    assert_eq!(tv.channel(), 12);
    assert_eq!(tv.volume(), 55);
    assert!(tv.is_muted());
}

#[test]
fn muting_does_not_turn_the_power_off() {
    let mut tv = new_television();

    tv.turn_on().unwrap();
    tv.mute().unwrap();

    assert!(tv.is_on());
    assert!(tv.is_muted());
}

#[test]
fn zero_volume_does_not_turn_the_power_off() {
    let mut tv = new_television();

    tv.turn_on().unwrap();
    tv.set_volume(40).unwrap();
    tv.set_volume(0).unwrap();

    assert!(tv.is_on());
    assert_eq!(tv.volume(), 0);
}

#[test]
fn invalid_channel_is_reported_and_changes_nothing() {
    let mut tv = new_television();
    tv.turn_on().unwrap();
    tv.set_channel(10).unwrap();

    let result = tv.set_channel(0);

    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(tv.channel(), 10);
    assert!(tv.is_on());
}

#[test]
fn invalid_volume_is_reported_and_changes_nothing() {
    let mut tv = new_television();
    tv.turn_on().unwrap();
    tv.set_volume(40).unwrap();

    let result = tv.set_volume(101);

    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(tv.volume(), 40);
    assert!(tv.is_on());
}

#[test]
fn power_still_works_after_invalid_media_input() {
    let mut tv = new_television();

    let result = tv.set_volume(101);
    assert_eq!(result, Err(DeviceError::InvalidInput));

    tv.turn_on().unwrap();
    assert!(tv.is_on());

    tv.turn_off().unwrap();
    assert!(!tv.is_on());
}

#[test]
fn can_be_used_through_generic_capability_bounds() {
    let mut tv = new_television();
    power_on_any(&mut tv).unwrap();
    set_volume_to(&mut tv, 30).unwrap();

    assert!(tv.is_on());
    assert_eq!(tv.volume(), 30);
}

#[test]
fn can_be_used_through_trait_objects() {
    let mut tv = new_television();

    let power: &mut dyn PowerControl = &mut tv;
    power.toggle().unwrap();
    assert!(power.is_on());

    let media: &mut dyn MediaControl = &mut tv;
    media.mute().unwrap();
    assert!(media.is_muted());
    media.toggle_mute().unwrap();
    assert!(!media.is_muted());
}
