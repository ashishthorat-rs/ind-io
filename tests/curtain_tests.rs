use ind_io::mock::{MockPosition, MockPower};
use ind_io::traits::{PositionControl, PowerControl};
use ind_io::{Curtain, DeviceError};

fn new_curtain() -> Curtain<MockPower, MockPosition> {
    Curtain::new(MockPower::new(), MockPosition::new())
}

fn power_on_any<P: PowerControl>(device: &mut P) -> Result<(), DeviceError> {
    device.turn_on()
}

fn set_half_open<M: PositionControl>(device: &mut M) -> Result<(), DeviceError> {
    device.set_position(50)
}

#[test]
fn starts_off() {
    let curtain = new_curtain();
    assert!(!curtain.is_on());
}

#[test]
fn turn_on_and_turn_off_change_power_state() {
    let mut curtain = new_curtain();
    curtain.turn_on().unwrap();
    assert!(curtain.is_on());
    curtain.turn_off().unwrap();
    assert!(!curtain.is_on());
}

#[test]
fn toggle_flips_power() {
    let mut curtain = new_curtain();
    curtain.toggle().unwrap();
    assert!(curtain.is_on());
    curtain.toggle().unwrap();
    assert!(!curtain.is_on());
}

#[test]
fn reset_leaves_the_device_on_with_position_unchanged() {
    let mut curtain = new_curtain();
    curtain.set_position(60).unwrap();

    curtain.reset().unwrap();

    assert!(curtain.is_on());
    assert_eq!(curtain.position(), 60);
}

#[test]
fn position_set_through_the_device_is_read_back() {
    let mut curtain = new_curtain();
    curtain.set_position(50).unwrap();
    assert_eq!(curtain.position(), 50);
}

#[test]
fn open_and_close_move_to_the_limits() {
    let mut curtain = new_curtain();
    curtain.open().unwrap();
    assert_eq!(curtain.position(), 100);
    assert!(curtain.is_open());
    curtain.close().unwrap();
    assert_eq!(curtain.position(), 0);
    assert!(curtain.is_closed());
}

#[test]
fn setting_position_while_off_does_not_turn_it_on() {
    let mut curtain = new_curtain();
    curtain.set_position(50).unwrap();
    assert!(!curtain.is_on());
}

#[test]
fn setting_position_while_on_does_not_turn_it_off() {
    let mut curtain = new_curtain();
    curtain.turn_on().unwrap();
    curtain.set_position(75).unwrap();
    assert!(curtain.is_on());
}

#[test]
fn turning_on_keeps_the_position_set_while_off() {
    let mut curtain = new_curtain();
    curtain.set_position(30).unwrap();
    curtain.turn_on().unwrap();
    assert_eq!(curtain.position(), 30);
}

#[test]
fn turning_off_keeps_the_configured_position() {
    let mut curtain = new_curtain();
    curtain.turn_on().unwrap();
    curtain.set_position(80).unwrap();
    curtain.turn_off().unwrap();
    assert_eq!(curtain.position(), 80);
}

#[test]
fn closing_does_not_turn_the_power_off() {
    let mut curtain = new_curtain();
    curtain.turn_on().unwrap();
    curtain.open().unwrap();
    curtain.close().unwrap();
    assert!(curtain.is_on());
    assert!(curtain.is_closed());
}

#[test]
fn invalid_position_is_reported_and_changes_nothing() {
    let mut curtain = new_curtain();
    curtain.turn_on().unwrap();
    curtain.set_position(40).unwrap();

    let result = curtain.set_position(101);

    assert_eq!(result, Err(DeviceError::InvalidInput));
    assert_eq!(curtain.position(), 40);
    assert!(curtain.is_on());
}

#[test]
fn power_still_works_after_invalid_position() {
    let mut curtain = new_curtain();

    let result = curtain.set_position(101);
    assert_eq!(result, Err(DeviceError::InvalidInput));

    curtain.turn_on().unwrap();
    assert!(curtain.is_on());
    curtain.turn_off().unwrap();
    assert!(!curtain.is_on());
}

#[test]
fn can_be_used_through_generic_capability_bounds() {
    let mut curtain = new_curtain();
    power_on_any(&mut curtain).unwrap();
    set_half_open(&mut curtain).unwrap();
    assert!(curtain.is_on());
    assert_eq!(curtain.position(), 50);
}

#[test]
fn can_be_used_through_trait_objects() {
    let mut curtain = new_curtain();

    let power: &mut dyn PowerControl = &mut curtain;
    power.toggle().unwrap();
    assert!(power.is_on());

    let position: &mut dyn PositionControl = &mut curtain;
    position.open().unwrap();
    assert!(position.is_open());
    position.close().unwrap();
    assert!(position.is_closed());
}
