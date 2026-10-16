use ind_io::mock::{MockPower, MockSpeed};
use ind_io::traits::{PowerControl, SpeedControl};
use ind_io::{DeviceError, Fan, Speed};

fn new_fan() -> Fan<MockPower, MockSpeed> {
    Fan::new(MockPower::new(), MockSpeed::new())
}

fn power_on_any<P: PowerControl>(device: &mut P) -> Result<(), DeviceError> {
    device.turn_on()
}

fn set_high<S: SpeedControl>(device: &mut S) -> Result<(), DeviceError> {
    device.set_speed(Speed::High)
}

#[test]
fn starts_off() {
    let fan = new_fan();
    assert!(!fan.is_on());
}

#[test]
fn turn_on_and_turn_off_change_power_state() {
    let mut fan = new_fan();
    fan.turn_on().unwrap();
    assert!(fan.is_on());
    fan.turn_off().unwrap();
    assert!(!fan.is_on());
}

#[test]
fn toggle_flips_power() {
    let mut fan = new_fan();
    fan.toggle().unwrap();
    assert!(fan.is_on());
    fan.toggle().unwrap();
    assert!(!fan.is_on());
}

#[test]
fn reset_leaves_the_device_on_with_speed_unchanged() {
    let mut fan = new_fan();
    fan.set_speed(Speed::High).unwrap();
    fan.reset().unwrap();
    assert!(fan.is_on());
    assert_eq!(fan.speed(), Speed::High);
}

#[test]
fn speed_set_through_the_device_is_read_back() {
    let mut fan = new_fan();
    fan.set_speed(Speed::High).unwrap();
    assert_eq!(fan.speed(), Speed::High);
}

#[test]
fn setting_speed_while_off_does_not_turn_it_on() {
    let mut fan = new_fan();
    fan.set_speed(Speed::Medium).unwrap();
    assert!(!fan.is_on());
}

#[test]
fn setting_speed_while_on_does_not_turn_it_off() {
    let mut fan = new_fan();
    fan.turn_on().unwrap();
    fan.set_speed(Speed::Low).unwrap();
    assert!(fan.is_on());
}

#[test]
fn turning_on_keeps_the_speed_configured_while_off() {
    let mut fan = new_fan();
    fan.set_speed(Speed::Medium).unwrap();
    fan.turn_on().unwrap();
    assert_eq!(fan.speed(), Speed::Medium);
}

#[test]
fn turning_off_keeps_the_configured_speed() {
    let mut fan = new_fan();

    fan.turn_on().unwrap();
    fan.set_speed(Speed::Turbo).unwrap();
    fan.turn_off().unwrap();

    assert_eq!(fan.speed(), Speed::Turbo);
}

#[test]
fn speed_off_does_not_turn_the_power_off() {
    let mut fan = new_fan();

    fan.turn_on().unwrap();
    fan.set_speed(Speed::High).unwrap();
    fan.set_speed(Speed::Off).unwrap();

    assert!(fan.is_on());
    assert_eq!(fan.speed(), Speed::Off);
}

#[test]
fn can_be_used_through_generic_capability_bounds() {
    let mut fan = new_fan();
    power_on_any(&mut fan).unwrap();
    set_high(&mut fan).unwrap();
    assert!(fan.is_on());
    assert_eq!(fan.speed(), Speed::High);
}

#[test]
fn can_be_used_through_trait_objects() {
    let mut fan = new_fan();

    let power: &mut dyn PowerControl = &mut fan;
    power.toggle().unwrap();
    assert!(power.is_on());

    let speed: &mut dyn SpeedControl = &mut fan;
    speed.set_speed(Speed::Low).unwrap();
    assert_eq!(speed.speed(), Speed::Low);
}
