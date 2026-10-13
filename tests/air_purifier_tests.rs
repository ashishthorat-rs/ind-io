use ind_io::mock::{MockPower, MockSpeed};
use ind_io::traits::{PowerControl, SpeedControl};
use ind_io::{AirPurifier, DeviceError, Speed};

fn new_air_purifier() -> AirPurifier<MockPower, MockSpeed> {
    AirPurifier::new(MockPower::new(), MockSpeed::new())
}

fn power_on_any<P: PowerControl>(device: &mut P) -> Result<(), DeviceError> {
    device.turn_on()
}

fn set_high<S: SpeedControl>(device: &mut S) -> Result<(), DeviceError> {
    device.set_speed(Speed::High)
}

#[test]
fn starts_off() {
    let purifier = new_air_purifier();
    assert!(!purifier.is_on())
}

#[test]
fn turn_on_and_turn_off_change_power_state() {
    let mut purifier = new_air_purifier();
    purifier.turn_on().unwrap();
    assert!(purifier.is_on());
    purifier.turn_off().unwrap();
    assert!(!purifier.is_on());
}

#[test]
fn toggle_flips_power() {
    let mut purifier = new_air_purifier();
    purifier.toggle().unwrap();
    assert!(purifier.is_on());
    purifier.toggle().unwrap();
    assert!(!purifier.is_on());
}

#[test]
fn reset_leaves_the_device_on_with_speed_unchanged() {
    let mut purifier = new_air_purifier();
    purifier.set_speed(Speed::High).unwrap();

    purifier.reset().unwrap();

    assert!(purifier.is_on());
    assert_eq!(purifier.speed(), Speed::High);
}

#[test]
fn speed_set_through_the_device_is_read_back() {
    let mut purifier = new_air_purifier();
    purifier.set_speed(Speed::High).unwrap();
    assert_eq!(purifier.speed(), Speed::High);
}

#[test]
fn setting_speed_while_off_does_not_turn_it_on() {
    let mut purifier = new_air_purifier();
    purifier.set_speed(Speed::Medium).unwrap();
    assert!(!purifier.is_on());
}

#[test]
fn setting_speed_while_on_does_not_turn_it_off() {
    let mut purifier = new_air_purifier();
    purifier.turn_on().unwrap();
    purifier.set_speed(Speed::Low).unwrap();
    assert!(purifier.is_on());
}

#[test]
fn turning_on_keeps_the_speed_configured_while_off() {
    let mut purifier = new_air_purifier();
    purifier.set_speed(Speed::Medium).unwrap();
    purifier.turn_on().unwrap();
    assert_eq!(purifier.speed(), Speed::Medium);
}

#[test]
fn turning_off_keeps_the_configured_speed() {
    let mut purifier = new_air_purifier();
    purifier.turn_on().unwrap();
    purifier.set_speed(Speed::Turbo).unwrap();
    purifier.turn_off().unwrap();
    assert_eq!(purifier.speed(), Speed::Turbo);
}

#[test]
fn speed_off_does_not_turn_the_power_off() {
    let mut purifier = new_air_purifier();
    purifier.turn_on().unwrap();
    purifier.set_speed(Speed::High).unwrap();
    purifier.set_speed(Speed::Off).unwrap();
    assert!(purifier.is_on());
    assert_eq!(purifier.speed(), Speed::Off);
}

#[test]
fn can_be_used_through_generic_capability_bounds() {
    let mut purifier = new_air_purifier();
    power_on_any(&mut purifier).unwrap();
    set_high(&mut purifier).unwrap();
    assert!(purifier.is_on());
    assert_eq!(purifier.speed(), Speed::High);
}

#[test]
fn can_be_used_through_trait_objects() {
    let mut purifier = new_air_purifier();

    let power: &mut dyn PowerControl = &mut purifier;
    power.toggle().unwrap();
    assert!(power.is_on());

    let speed: &mut dyn SpeedControl = &mut purifier;
    speed.set_speed(Speed::Low).unwrap();
    assert_eq!(speed.speed(), Speed::Low);
}
